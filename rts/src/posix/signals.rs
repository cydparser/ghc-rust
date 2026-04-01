use crate::capability::getCapability;
use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::tty::__hscore_get_saved_termios;
use crate::ffi::rts::{EXIT_INTERRUPTED, stg_exit};
use crate::ffi::rts_api::{Capability, HaskellObj, rts_evalIO, rts_lock, rts_unlock};
use crate::ffi::stg::types::{StgInt, StgWord8};
use crate::posix::signals::{STG_SIG_DFL, STG_SIG_ERR, STG_SIG_HAN, STG_SIG_RST};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::schedule::{SCHED_INTERRUPTING, getSchedState, interruptStgRts};
use crate::ticker::TickProc;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_DFL: c_int = -1;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_IGN: c_int = -2;

#[ffi(libraries)]
pub const STG_SIG_ERR: c_int = -3;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_HAN: c_int = -4;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_RST: c_int = -5;

pub(crate) static mut nocldstop: HsInt = 0;

static mut signal_handlers: *mut StgInt = null_mut::<StgInt>();

static mut nHandlers: StgInt = 0;

static mut n_haskell_handlers: u32 = 0;

static mut userSignals: sigset_t = 0;

static mut savedSignals: sigset_t = 0;

static mut sig_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn initUserSignals() {
    userSignals = 0;
    initMutex(&raw mut sig_mutex);
}

unsafe fn freeSignalHandlers() {
    if !signal_handlers.is_null() {
        stgFree(signal_handlers as *mut c_void);
        signal_handlers = null_mut::<StgInt>();
        nHandlers = 0;
        n_haskell_handlers = 0;
    }

    closeMutex(&raw mut sig_mutex);
}

unsafe fn more_handlers(mut sig: i32) {
    let mut i: StgInt = 0;

    if (sig as StgInt) < nHandlers {
        return;
    }

    if signal_handlers.is_null() {
        signal_handlers = stgMallocBytes(
            ((sig + 1 as i32) as usize).wrapping_mul(size_of::<StgInt>() as usize),
            c"more_handlers".as_ptr(),
        ) as *mut StgInt;
    } else {
        signal_handlers = stgReallocBytes(
            signal_handlers as *mut c_void,
            ((sig + 1 as i32) as usize).wrapping_mul(size_of::<StgInt>() as usize),
            c"more_handlers".as_ptr(),
        ) as *mut StgInt;
    }

    i = nHandlers;

    while i <= sig as StgInt {
        *signal_handlers.offset(i as isize) = STG_SIG_DFL as StgInt;
        i += 1;
    }

    nHandlers = (sig + 1) as StgInt;
}

static mut io_manager_wakeup_fd: i32 = -1;

static mut timer_manager_control_wr_fd: i32 = -1;

const IO_MANAGER_WAKEUP: i32 = 0xff;

const IO_MANAGER_DIE: i32 = 0xfe;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setTimerManagerControlFd(mut fd: c_int) {
    (&raw mut timer_manager_control_wr_fd).store(fd, Ordering::Relaxed);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerWakeupFd(mut fd: c_int) {
    (&raw mut io_manager_wakeup_fd).store(fd, Ordering::SeqCst);
}

unsafe fn ioManagerWakeup() {
    let mut r: i32 = 0;
    let wakeup_fd = (&raw mut io_manager_wakeup_fd).load(Ordering::SeqCst);

    if wakeup_fd >= 0 {
        let mut byte: StgWord8 = IO_MANAGER_WAKEUP as StgWord8;
        r = write(wakeup_fd, &raw mut byte as *const c_void, 1) as i32;

        if r == -1 && (&raw mut io_manager_wakeup_fd).load(Ordering::SeqCst) >= 0 {
            sysErrorBelch(c"ioManagerWakeup: write".as_ptr());
        }
    }
}

unsafe fn ioManagerDie() {
    let mut byte: StgWord8 = IO_MANAGER_DIE as StgWord8;
    let mut i: u32 = 0;
    let mut r: i32 = 0;
    let fd = (&raw mut timer_manager_control_wr_fd).load(Ordering::Relaxed);

    if 0 <= fd {
        r = write(fd, &raw mut byte as *const c_void, 1) as i32;

        if r == -1 {
            sysErrorBelch(c"ioManagerDie: write".as_ptr());
        }

        (&raw mut timer_manager_control_wr_fd).store(-1, Ordering::Relaxed);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        let fd_0 =
            (&raw mut (*(*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i))
                .iomgr)
                .control_fd)
                .load(Ordering::Relaxed);

        if 0 <= fd_0 {
            r = write(fd_0, &raw mut byte as *const c_void, 1) as i32;

            if r == -1 {
                sysErrorBelch(c"ioManagerDie: write".as_ptr());
            }

            (&raw mut (*(*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i))
                .iomgr)
                .control_fd)
                .store(-1, Ordering::Relaxed);
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn ioManagerStartCap(mut cap: *mut *mut Capability) {
    rts_evalIO(
        cap,
        (*ghc_hs_iface).ensureIOManagerIsRunning_closure as HaskellObj,
        null_mut::<HaskellObj>(),
    );
}

unsafe fn ioManagerStart() {
    let mut cap = null_mut::<Capability>();

    if (&raw mut timer_manager_control_wr_fd).load(Ordering::SeqCst) < 0
        || (&raw mut io_manager_wakeup_fd).load(Ordering::SeqCst) < 0
    {
        cap = rts_lock();
        ioManagerStartCap(&raw mut cap);
        rts_unlock(cap);
    }
}

unsafe fn generic_handler(mut sig: i32, mut info: *mut siginfo_t, mut p: *mut c_void) {
    let mut buf: [StgWord8; 105] = [0; 105];
    let mut r: i32 = 0;
    buf[0] = sig as StgWord8;

    if info.is_null() {
        memset(
            (&raw mut buf as *mut StgWord8).offset(1) as *mut c_void,
            0,
            size_of::<siginfo_t>() as usize,
        );
    } else {
        memcpy(
            (&raw mut buf as *mut StgWord8).offset(1) as *mut c_void,
            info as *const c_void,
            size_of::<siginfo_t>() as usize,
        );
    }

    let mut timer_control_fd = (&raw mut timer_manager_control_wr_fd).load(Ordering::Relaxed);

    if 0 <= timer_control_fd {
        r = write(
            timer_control_fd,
            &raw mut buf as *mut StgWord8 as *const c_void,
            (size_of::<siginfo_t>() as usize).wrapping_add(1 as usize),
        ) as i32;

        if r == -1 && *__error() == EAGAIN {
            errorBelch(c"lost signal due to full pipe: %d\n".as_ptr(), sig);
        }
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn blockUserSignals() {
    sigprocmask(SIG_BLOCK, &raw mut userSignals, &raw mut savedSignals);
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    sigprocmask(SIG_SETMASK, &raw mut savedSignals, null_mut::<sigset_t>());
}

unsafe fn anyUserHandlers() -> bool {
    return n_haskell_handlers != 0;
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_sig_install(
    mut sig: c_int,
    mut spi: c_int,
    mut mask: *mut c_void,
) -> c_int {
    let mut signals: sigset_t = 0;
    let mut osignals: sigset_t = 0;
    let mut previous_spi: StgInt = 0;

    let mut action = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    memset(
        &raw mut action as *mut c_void,
        0,
        size_of::<sigaction>() as usize,
    );

    let mut __r = pthread_mutex_lock(&raw mut sig_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/posix/Signals.c".as_ptr(),
            382,
            __r,
        );
    }

    if sig < 0
        || {
            signals = 0;
            0 != 0
        }
        || {
            signals |= __sigbits(sig) as sigset_t;
            0 != 0
        }
        || sigprocmask(SIG_BLOCK, &raw mut signals, &raw mut osignals) != 0
    {
        if pthread_mutex_unlock(&raw mut sig_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/posix/Signals.c".as_ptr(),
                390,
            );
        }

        return STG_SIG_ERR;
    }

    more_handlers(sig);
    previous_spi = *signal_handlers.offset(sig as isize);
    action.sa_flags = 0;

    let mut current_block_14: u64;

    match spi {
        STG_SIG_IGN => {
            action.__sigaction_u.__sa_handler =
                transmute::<isize, Option<unsafe extern "C" fn(c_int) -> ()>>(1);

            current_block_14 = 26972500619410423;
        }
        STG_SIG_DFL => {
            action.__sigaction_u.__sa_handler = SIG_DFL;
            current_block_14 = 26972500619410423;
        }
        STG_SIG_RST => {
            action.sa_flags |= SA_RESETHAND;
            current_block_14 = 14629483928897605736;
        }
        STG_SIG_HAN => {
            current_block_14 = 14629483928897605736;
        }
        _ => {
            barf(c"stg_sig_install: bad spi".as_ptr());
        }
    }

    match current_block_14 {
        14629483928897605736 => {
            action.__sigaction_u.__sa_sigaction = Some(
                generic_handler as unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void) -> (),
            )
                as Option<unsafe extern "C" fn(c_int, *mut __siginfo, *mut c_void) -> ()>;

            action.sa_flags |= SA_SIGINFO;
        }
        _ => {}
    }

    if !mask.is_null() {
        action.sa_mask = *(mask as *mut sigset_t);
    } else {
        action.sa_mask = 0;
    }

    action.sa_flags |= if sig == SIGCHLD && nocldstop != 0 {
        SA_NOCLDSTOP
    } else {
        0
    };

    if sigaction(sig, &raw mut action, null_mut::<sigaction>()) != 0 {
        errorBelch(c"sigaction".as_ptr());

        if pthread_mutex_unlock(&raw mut sig_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/posix/Signals.c".as_ptr(),
                431,
            );
        }

        return STG_SIG_ERR;
    }

    *signal_handlers.offset(sig as isize) = spi as StgInt;

    match spi {
        STG_SIG_RST | STG_SIG_HAN => {
            userSignals |= __sigbits(sig) as sigset_t;

            if previous_spi != STG_SIG_HAN as StgInt && previous_spi != STG_SIG_RST as StgInt {
                n_haskell_handlers = n_haskell_handlers.wrapping_add(1);
            }
        }
        _ => {
            userSignals &= !__sigbits(sig) as sigset_t;

            if previous_spi == STG_SIG_HAN as StgInt || previous_spi == STG_SIG_RST as StgInt {
                n_haskell_handlers = n_haskell_handlers.wrapping_sub(1);
            }
        }
    }

    if sigprocmask(SIG_SETMASK, &raw mut osignals, null_mut::<sigset_t>()) != 0 {
        errorBelch(c"sigprocmask".as_ptr());

        if pthread_mutex_unlock(&raw mut sig_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/posix/Signals.c".as_ptr(),
                457,
            );
        }

        return STG_SIG_ERR;
    }

    if pthread_mutex_unlock(&raw mut sig_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/posix/Signals.c".as_ptr(),
            461,
        );
    }

    return previous_spi as i32;
}

unsafe fn shutdown_handler(mut sig: i32) {
    if getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32 {
        _exit(EXIT_INTERRUPTED);
    } else {
        interruptStgRts();
    };
}

unsafe fn backtrace_handler(mut sig: i32) {
    fprintf(
        __stderrp,
        c"This build does not support backtraces.\n".as_ptr(),
    );
}

unsafe fn empty_handler(mut sig: i32) {}

unsafe fn sigtstp_handler(mut sig: i32) {
    let mut fd: i32 = 0;

    let mut ts: [termios; 3] = [termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    }; 3];

    fd = 0;

    while fd <= 2 {
        if !__hscore_get_saved_termios(fd).is_null() {
            tcgetattr(
                fd,
                (&raw mut ts as *mut termios).offset(fd as isize) as *mut termios,
            );
        }

        fd += 1;
    }

    kill(getpid(), SIGSTOP);
    fd = 0;

    while fd <= 2 {
        if !__hscore_get_saved_termios(fd).is_null() {
            tcsetattr(
                0,
                TCSANOW,
                (&raw mut ts as *mut termios).offset(fd as isize) as *mut termios,
            );
        }

        fd += 1;
    }
}

unsafe fn set_sigtstp_action(mut handle: bool) {
    let mut sa = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    memset(
        &raw mut sa as *mut c_void,
        0,
        size_of::<sigaction>() as usize,
    );

    if handle {
        sa.__sigaction_u.__sa_handler = Some(sigtstp_handler as unsafe extern "C" fn(c_int) -> ())
            as Option<unsafe extern "C" fn(c_int) -> ()>;
    } else {
        sa.__sigaction_u.__sa_handler = SIG_DFL;
    }

    sa.sa_flags = 0;
    sa.sa_mask = 0;

    if sigaction(SIGTSTP, &raw mut sa, null_mut::<sigaction>()) != 0 {
        sysErrorBelch(c"warning: failed to install SIGTSTP handler".as_ptr());
    }
}

unsafe fn install_vtalrm_handler(mut sig: i32, mut handle_tick: TickProc) {
    let mut action = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    memset(
        &raw mut action as *mut c_void,
        0,
        size_of::<sigaction>() as usize,
    );
    action.__sigaction_u.__sa_handler = handle_tick as Option<unsafe extern "C" fn(c_int) -> ()>;
    action.sa_mask = 0;
    action.sa_flags = SA_RESTART;

    if sigaction(sig, &raw mut action, null_mut::<sigaction>()) == -1 {
        sysErrorBelch(c"sigaction".as_ptr());
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn initDefaultHandlers() {
    let mut action = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    let mut oact = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    memset(
        &raw mut oact as *mut c_void,
        0,
        size_of::<sigaction>() as usize,
    );
    memset(
        &raw mut action as *mut c_void,
        0,
        size_of::<sigaction>() as usize,
    );

    action.__sigaction_u.__sa_handler = Some(shutdown_handler as unsafe extern "C" fn(c_int) -> ())
        as Option<unsafe extern "C" fn(c_int) -> ()>;

    action.sa_mask = 0;
    action.sa_flags = 0;

    if sigaction(SIGINT, &raw mut action, &raw mut oact) != 0 {
        sysErrorBelch(c"warning: failed to install SIGINT handler".as_ptr());
    }

    action.__sigaction_u.__sa_handler = Some(empty_handler as unsafe extern "C" fn(c_int) -> ())
        as Option<unsafe extern "C" fn(c_int) -> ()>;

    action.sa_mask = 0;
    action.sa_flags = 0;

    if sigaction(SIGPIPE, &raw mut action, &raw mut oact) != 0 {
        sysErrorBelch(c"warning: failed to install SIGPIPE handler".as_ptr());
    }

    action.__sigaction_u.__sa_handler = Some(backtrace_handler as unsafe extern "C" fn(c_int) -> ())
        as Option<unsafe extern "C" fn(c_int) -> ()>;

    action.sa_mask = 0;
    action.sa_flags = 0;

    if sigaction(SIGQUIT, &raw mut action, &raw mut oact) != 0 {
        sysErrorBelch(c"warning: failed to install SIGQUIT handler".as_ptr());
    }

    set_sigtstp_action(true);
}

unsafe fn resetDefaultHandlers() {
    let mut action = sigaction {
        __sigaction_u: __sigaction_u { __sa_handler: None },
        sa_mask: 0,
        sa_flags: 0,
    };

    action.__sigaction_u.__sa_handler = SIG_DFL;
    action.sa_mask = 0;
    action.sa_flags = 0;

    if sigaction(SIGINT, &raw mut action, null_mut::<sigaction>()) != 0 {
        sysErrorBelch(c"warning: failed to uninstall SIGINT handler".as_ptr());
    }

    if sigaction(SIGPIPE, &raw mut action, null_mut::<sigaction>()) != 0 {
        sysErrorBelch(c"warning: failed to uninstall SIGPIPE handler".as_ptr());
    }

    set_sigtstp_action(false);
}
