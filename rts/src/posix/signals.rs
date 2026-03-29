use crate::capability::interruptCapability;
use crate::ffi::hs_ffi::{HsInt, HsPtr};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::threads::{MainCapability, createIOThread};
use crate::ffi::rts::tty::__hscore_get_saved_termios;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::{EXIT_INTERRUPTED, stg_exit};
use crate::ffi::rts_api::{Capability, HaskellObj, rts_apply, rts_mkInt, rts_mkPtr};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgInt, StgWord8};
use crate::posix::signals::{STG_SIG_DFL, STG_SIG_ERR, STG_SIG_HAN, STG_SIG_RST};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::schedule::{
    SCHED_INTERRUPTING, SCHED_RUNNING, getSchedState, interruptStgRts, scheduleThread,
};
use crate::thread_labels::setThreadLabel;
use crate::ticker::TickProc;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_DFL: i32 = -1;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_IGN: i32 = -2;

#[ffi(libraries)]
pub const STG_SIG_ERR: i32 = -3;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_HAN: i32 = -4;

#[ffi(ghc_lib, libraries)]
pub const STG_SIG_RST: i32 = -5;

pub(crate) static mut nocldstop: HsInt = 0;

static mut signal_handlers: *mut StgInt = null_mut::<StgInt>();

static mut nHandlers: StgInt = 0;

static mut n_haskell_handlers: u32 = 0;

static mut userSignals: sigset_t = 0;

static mut savedSignals: sigset_t = 0;

unsafe fn initUserSignals() {
    userSignals = 0;
}

unsafe fn freeSignalHandlers() {
    if !signal_handlers.is_null() {
        stgFree(signal_handlers as *mut c_void);
        signal_handlers = null_mut::<StgInt>();
        nHandlers = 0;
        n_haskell_handlers = 0;
    }
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

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setTimerManagerControlFd(mut fd: i32) {
    timer_manager_control_wr_fd = fd;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerWakeupFd(mut fd: i32) {
    io_manager_wakeup_fd = fd;
}

unsafe fn ioManagerWakeup() {
    let mut r: i32 = 0;
    let wakeup_fd = io_manager_wakeup_fd;

    if wakeup_fd >= 0 {
        let mut byte: StgWord8 = IO_MANAGER_WAKEUP as StgWord8;
        r = write(wakeup_fd, &raw mut byte as *const c_void, 1) as i32;

        if r == -1 && io_manager_wakeup_fd >= 0 {
            sysErrorBelch(c"ioManagerWakeup: write".as_ptr());
        }
    }
}

const N_PENDING_HANDLERS: i32 = 16;

static mut pending_handler_buf: [siginfo_t; 16] = [__siginfo {
    si_signo: 0,
    si_errno: 0,
    si_code: 0,
    si_pid: 0,
    si_uid: 0,
    si_status: 0,
    si_addr: null_mut::<c_void>(),
    si_value: sigval { sival_int: 0 },
    si_band: 0,
    __pad: [0; 7],
}; 16];

static mut next_pending_handler: *mut siginfo_t =
    unsafe { &raw const pending_handler_buf as *mut siginfo_t };

unsafe fn generic_handler(mut sig: i32, mut info: *mut siginfo_t, mut p: *mut c_void) {
    memcpy(
        next_pending_handler as *mut c_void,
        info as *const c_void,
        size_of::<siginfo_t>() as usize,
    );

    next_pending_handler = next_pending_handler.offset(1);

    if next_pending_handler
        == (&raw mut pending_handler_buf as *mut siginfo_t).offset(N_PENDING_HANDLERS as isize)
            as *mut siginfo_t
    {
        errorBelch(c"too many pending signals".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    interruptCapability(&raw mut MainCapability);
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

unsafe fn awaitUserSignals() {
    while !(next_pending_handler != &raw mut pending_handler_buf as *mut siginfo_t)
        && getSchedState() as u32 == SCHED_RUNNING as i32 as u32
    {
        pause();
    }
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_sig_install(mut sig: i32, mut spi: i32, mut mask: *mut c_void) -> i32 {
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

            current_block_14 = 3512920355445576850;
        }
        STG_SIG_DFL => {
            action.__sigaction_u.__sa_handler = SIG_DFL;
            current_block_14 = 3512920355445576850;
        }
        STG_SIG_RST => {
            action.sa_flags |= SA_RESETHAND;
            current_block_14 = 15034202110365292022;
        }
        STG_SIG_HAN => {
            current_block_14 = 15034202110365292022;
        }
        _ => {
            barf(c"stg_sig_install: bad spi".as_ptr());
        }
    }

    match current_block_14 {
        15034202110365292022 => {
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

        return STG_SIG_ERR;
    }

    return previous_spi as i32;
}

unsafe fn startSignalHandlers(mut cap: *mut Capability) {
    let mut info = null_mut::<siginfo_t>();
    let mut sig: i32 = 0;
    blockUserSignals();

    while next_pending_handler != &raw mut pending_handler_buf as *mut siginfo_t {
        next_pending_handler = next_pending_handler.offset(-1);
        sig = (*next_pending_handler).si_signo;

        if *signal_handlers.offset(sig as isize) == STG_SIG_DFL as StgInt {
            continue;
        }

        info = stgMallocBytes(
            size_of::<siginfo_t>() as usize,
            c"startSignalHandlers".as_ptr(),
        ) as *mut siginfo_t;

        memcpy(
            info as *mut c_void,
            next_pending_handler as *const c_void,
            size_of::<siginfo_t>() as usize,
        );

        let mut t = createIOThread(
            cap,
            RtsFlags.GcFlags.initialStkSize as W_,
            rts_apply(
                cap,
                rts_apply(
                    cap,
                    (*ghc_hs_iface).runHandlersPtr_closure as HaskellObj,
                    rts_mkPtr(cap, info as HsPtr),
                ),
                rts_mkInt(cap, (*info).si_signo as HsInt),
            ) as *mut StgClosure,
        );

        scheduleThread(cap, t);
        setThreadLabel(cap, t, c"signal handler thread".as_ptr());
    }

    unblockUserSignals();
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
