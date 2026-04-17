use crate::capability::Capability;
use crate::ffi::rts::constants::NotBlocked;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::storage::tso::{StgTSO_, setTSOLink};
use crate::ffi::rts::time::{TIME_RESOLUTION, Time, getProcessElapsedTime};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{StgWord, StgWord32};
use crate::hs_ffi::HsInt;
use crate::posix::select::LowResTime;
use crate::posix::signals::{next_pending_handler, pending_handler_buf, startSignalHandlers};
use crate::prelude::*;
use crate::raise_async::raiseAsync;
use crate::rts_flags::RtsFlags;
use crate::schedule::{
    SCHED_INTERRUPTING, SCHED_RUNNING, emptyRunQueue, getSchedState, pushOnRunQueue,
};

pub(crate) type LowResTime = StgWord;

const RTS_FD_IS_BLOCKING: FdState = 1;

const RTS_FD_IS_READY: FdState = 0;

const RTS_FD_IS_INVALID: FdState = 2;

type FdState = u32;

unsafe fn getLowResTimeOfDay() -> LowResTime {
    return getProcessElapsedTime() as LowResTime;
}

unsafe fn getDelayTarget(mut us: HsInt) -> LowResTime {
    let mut elapsed: Time = 0;
    elapsed = getProcessElapsedTime();

    if us > (9223372036854775807 - elapsed) / 1000 {
        return 9223372036854775807;
    } else {
        return (elapsed + us * 1000) as LowResTime;
    };
}

unsafe fn wakeUpSleepingThreads(mut cap: *mut Capability, mut now: LowResTime) -> bool {
    let mut iomgr = (*cap).iomgr;
    let mut tso = null_mut::<StgTSO>();
    let mut flag = false;

    while (*iomgr).sleeping_queue
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
    {
        tso = (*iomgr).sleeping_queue;

        if (now as i64 - (*tso).block_info.target as i64) < 0 {
            break;
        }

        (*iomgr).sleeping_queue = (*tso)._link as *mut StgTSO;
        (*tso).why_blocked = 0;
        (*tso)._link =
            &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
        pushOnRunQueue(cap, tso);
        flag = true;
    }

    return flag;
}

unsafe fn fdOutOfRange(mut fd: i32) -> ! {
    errorBelch(
        c"file descriptor %d out of range for select (0--%d).\nRecompile with -threaded to work around this."
            .as_ptr(),
        fd,
        FD_SETSIZE,
    );

    stg_exit(EXIT_FAILURE);
}

unsafe fn fdPollReadState(mut fd: i32) -> FdState {
    let mut r: i32 = 0;
    let mut rfd = fd_set { fds_bits: [0; 32] };
    let mut now = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    __darwin_fd_set(fd, &raw mut rfd);
    now.tv_sec = 0;
    now.tv_usec = 0;

    loop {
        r = select(
            fd + 1,
            &raw mut rfd,
            null_mut::<fd_set>(),
            null_mut::<fd_set>(),
            &raw mut now,
        );

        if r != -1 {
            break;
        }

        match *__error() {
            EBADF => return RTS_FD_IS_INVALID,
            EINTR => {}
            _ => {
                sysErrorBelch(c"select".as_ptr());
                stg_exit(EXIT_FAILURE);
            }
        }
    }

    if r == 0 {
        return RTS_FD_IS_BLOCKING;
    } else {
        return RTS_FD_IS_READY;
    };
}

unsafe fn fdPollWriteState(mut fd: i32) -> FdState {
    let mut r: i32 = 0;
    let mut wfd = fd_set { fds_bits: [0; 32] };
    let mut now = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    __darwin_fd_set(fd, &raw mut wfd);
    now.tv_sec = 0;
    now.tv_usec = 0;

    loop {
        r = select(
            fd + 1,
            null_mut::<fd_set>(),
            &raw mut wfd,
            null_mut::<fd_set>(),
            &raw mut now,
        );

        if r != -1 {
            break;
        }

        match *__error() {
            EBADF => return RTS_FD_IS_INVALID,
            EINTR => {}
            _ => {
                sysErrorBelch(c"select".as_ptr());
                stg_exit(EXIT_FAILURE);
            }
        }
    }

    if r == 0 {
        return RTS_FD_IS_BLOCKING;
    } else {
        return RTS_FD_IS_READY;
    };
}

unsafe fn awaitCompletedTimeoutsOrIOSelect(mut cap: *mut Capability, mut wait: bool) {
    let mut iomgr = (*cap).iomgr;
    let mut tso = null_mut::<StgTSO>();
    let mut prev = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut rfd = fd_set { fds_bits: [0; 32] };
    let mut wfd = fd_set { fds_bits: [0; 32] };
    let mut numFound: i32 = 0;
    let mut maxfd = -1;
    let mut seen_bad_fd = false;
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ptv = null_mut::<timeval>();
    let mut now: LowResTime = 0;

    loop {
        now = getLowResTimeOfDay();

        if wakeUpSleepingThreads(cap, now) {
            return;
        }

        tso = (*iomgr).blocked_queue_hd;

        while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            next = (*tso)._link as *mut StgTSO;

            match (*tso).why_blocked {
                3 => {
                    let mut fd = (*tso).block_info.fd as i32;

                    if fd >= FD_SETSIZE || fd < 0 {
                        fdOutOfRange(fd);
                    }

                    maxfd = if fd > maxfd { fd } else { maxfd };
                    __darwin_fd_set(fd, &raw mut rfd);
                }
                4 => {
                    let mut fd_0 = (*tso).block_info.fd as i32;

                    if fd_0 >= FD_SETSIZE || fd_0 < 0 {
                        fdOutOfRange(fd_0);
                    }

                    maxfd = if fd_0 > maxfd { fd_0 } else { maxfd };
                    __darwin_fd_set(fd_0, &raw mut wfd);
                }
                _ => {
                    barf(c"AwaitEvent".as_ptr());
                }
            }

            tso = next;
        }

        if !wait {
            tv.tv_sec = 0;
            tv.tv_usec = 0;
            ptv = &raw mut tv;
        } else if (*iomgr).sleeping_queue
            != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        {
            let max_seconds = 2678400;
            let mut min: Time = (*(*iomgr).sleeping_queue)
                .block_info
                .target
                .wrapping_sub(now as StgWord) as Time;
            tv.tv_sec = (min / TIME_RESOLUTION as Time) as i64;

            if tv.tv_sec < max_seconds {
                tv.tv_usec = (min / 1000 % 1000000) as i32;
            } else {
                tv.tv_sec = max_seconds as i64;
                tv.tv_usec = 0;
            }

            ptv = &raw mut tv;
        } else {
            ptv = null_mut::<timeval>();
        }

        loop {
            numFound = select(
                maxfd + 1,
                &raw mut rfd,
                &raw mut wfd,
                null_mut::<fd_set>(),
                ptv,
            );

            if !(numFound < 0) {
                break;
            }

            if *__error() != EINTR {
                if *__error() == EBADF {
                    seen_bad_fd = true;
                    break;
                } else {
                    sysErrorBelch(c"select".as_ptr());
                    stg_exit(EXIT_FAILURE);
                }
            } else {
                if RtsFlags.MiscFlags.install_signal_handlers as i32 != 0
                    && next_pending_handler != &raw mut pending_handler_buf as *mut siginfo_t
                {
                    startSignalHandlers(cap);
                    return;
                }

                if getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32 {
                    return;
                }

                wakeUpSleepingThreads(cap, getLowResTimeOfDay());

                if !emptyRunQueue(cap) {
                    return;
                }
            }
        }

        prev = null_mut::<StgTSO>();
        tso = (*iomgr).blocked_queue_hd;

        while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            next = (*tso)._link as *mut StgTSO;

            let mut fd_1: i32 = 0;
            let mut fd_state = RTS_FD_IS_BLOCKING;

            match (*tso).why_blocked {
                3 => {
                    fd_1 = (*tso).block_info.fd as i32;

                    if seen_bad_fd {
                        fd_state = fdPollReadState(fd_1);
                    } else if __darwin_fd_isset(fd_1, &raw mut rfd) != 0 {
                        fd_state = RTS_FD_IS_READY;
                    }
                }
                4 => {
                    fd_1 = (*tso).block_info.fd as i32;

                    if seen_bad_fd {
                        fd_state = fdPollWriteState(fd_1);
                    } else if __darwin_fd_isset(fd_1, &raw mut wfd) != 0 {
                        fd_state = RTS_FD_IS_READY;
                    }
                }
                _ => {
                    barf(c"awaitCompletedTimeoutsOrIOSelect".as_ptr());
                }
            }

            match fd_state as u32 {
                2 => {
                    raiseAsync(
                        cap,
                        tso,
                        (*ghc_hs_iface).blockedOnBadFD_closure,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );
                }
                0 => {
                    (*tso).why_blocked = NotBlocked as StgWord32;
                    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        as *mut StgTSO_;
                    pushOnRunQueue(cap, tso);
                }
                1 => {
                    if prev.is_null() {
                        (*iomgr).blocked_queue_hd = tso;
                    } else {
                        setTSOLink(cap, prev, tso);
                    }

                    prev = tso;
                }
                _ => {}
            }

            tso = next;
        }

        if prev.is_null() {
            (*iomgr).blocked_queue_tl =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            (*iomgr).blocked_queue_hd = (*iomgr).blocked_queue_tl;
        } else {
            (*prev)._link =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
            (*iomgr).blocked_queue_tl = prev;
        }

        if !(wait as i32 != 0
            && getSchedState() as u32 == SCHED_RUNNING as i32 as u32
            && emptyRunQueue(cap) as i32 != 0)
        {
            break;
        }
    }
}
