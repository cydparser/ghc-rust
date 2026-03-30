use crate::capability::CapIOManager;
use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::constants::{BlockedOnRead, BlockedOnWrite};
use crate::ffi::rts::flags::{IO_MANAGER_FLAG, IO_MNGR_FLAG_AUTO, IO_MNGR_FLAG_SELECT, RtsFlags};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::getStablePtr;
use crate::ffi::rts::storage::tso::{StgTSO_, setTSOLink};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::types::{StgInt, StgPtr, StgWord, StgWord32};
use crate::io_manager::{
    IO_MANAGER_SELECT, IOManagerAvailability, IOManagerAvailable, IOManagerType,
    IOManagerUnavailable, IOManagerUnrecognised, IORead, IOReadOrWrite,
};
use crate::posix::select::{LowResTime, awaitCompletedTimeoutsOrIOSelect, getDelayTarget};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::sm::gc::evac_fn;
use crate::threads::{removeThreadFromDeQueue, removeThreadFromQueue};
use crate::trace::{DEBUG_RTS, trace_};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
pub(crate) struct _CapIOManager {
    pub(crate) blocked_queue_hd: *mut StgTSO,
    pub(crate) blocked_queue_tl: *mut StgTSO,
    pub(crate) sleeping_queue: *mut StgTSO,
}

pub(crate) type IOManagerType = u32;

pub(crate) const IO_MANAGER_SELECT: IOManagerType = 0;

pub(crate) type IOManagerAvailability = u32;

pub(crate) const IOManagerUnrecognised: IOManagerAvailability = 2;

pub(crate) const IOManagerUnavailable: IOManagerAvailability = 1;

pub(crate) const IOManagerAvailable: IOManagerAvailability = 0;

pub(crate) type IOReadOrWrite = u32;

pub(crate) const IOWrite: IOReadOrWrite = 1;

pub(crate) const IORead: IOReadOrWrite = 0;

static mut iomgr_type: IOManagerType = IO_MANAGER_SELECT;

unsafe fn parseIOManagerFlag(
    mut iomgrstr: *const c_char,
    mut flag: *mut IO_MANAGER_FLAG,
) -> IOManagerAvailability {
    if strcmp(c"select".as_ptr(), iomgrstr) == 0 {
        *flag = IO_MNGR_FLAG_SELECT;

        return IOManagerAvailable;
    } else if strcmp(c"mio".as_ptr(), iomgrstr) == 0 {
        return IOManagerUnavailable;
    } else if strcmp(c"winio".as_ptr(), iomgrstr) == 0 {
        return IOManagerUnavailable;
    } else if strcmp(c"win32-legacy".as_ptr(), iomgrstr) == 0 {
        return IOManagerUnavailable;
    } else if strcmp(c"auto".as_ptr(), iomgrstr) == 0 {
        *flag = IO_MNGR_FLAG_AUTO;

        return IOManagerAvailable;
    } else if strcmp(c"native".as_ptr(), iomgrstr) == 0 {
        *flag = IO_MNGR_FLAG_AUTO;

        return IOManagerAvailable;
    } else if strcmp(c"posix".as_ptr(), iomgrstr) == 0 {
        *flag = IO_MNGR_FLAG_AUTO;

        return IOManagerAvailable;
    } else {
        return IOManagerUnrecognised;
    };
}

unsafe fn selectIOManager() {
    match RtsFlags.MiscFlags.ioManager as u32 {
        0 => {
            iomgr_type = IO_MANAGER_SELECT;
        }
        1 => {
            iomgr_type = IO_MANAGER_SELECT;
        }
        _ => {
            barf(
                c"selectIOManager: %d".as_ptr(),
                RtsFlags.MiscFlags.ioManager as u32,
            );
        }
    };
}

unsafe fn showIOManager() -> *mut c_char {
    match iomgr_type as u32 {
        0 => {
            return c"select".as_ptr();
        }
        _ => {
            barf(c"showIOManager: %d".as_ptr(), iomgr_type as u32);
        }
    };
}

pub(crate) unsafe fn initCapabilityIOManager(mut cap: *mut Capability) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"initialising I/O manager %s for cap %d".as_ptr(),
            showIOManager(),
            (*cap).no,
        );
    }

    let mut iomgr = stgMallocBytes(
        size_of::<CapIOManager>() as usize,
        c"initCapabilityIOManager".as_ptr(),
    ) as *mut CapIOManager;

    match iomgr_type as u32 {
        0 => {
            (*iomgr).blocked_queue_hd =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            (*iomgr).blocked_queue_tl =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            (*iomgr).sleeping_queue =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
        }
        _ => {}
    }

    (*cap).iomgr = iomgr;
}

unsafe fn initIOManager() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"initialising %s I/O manager".as_ptr(), showIOManager());
    }

    match iomgr_type as u32 {
        0 => {
            getStablePtr((*ghc_hs_iface).blockedOnBadFD_closure as StgPtr);
        }
        _ => {}
    };
}

unsafe fn initIOManagerAfterFork(mut pcap: *mut *mut Capability) {
    match iomgr_type as u32 {
        _ => {}
    };
}

unsafe fn notifyIOManagerCapabilitiesChanged(mut pcap: *mut *mut Capability) {
    match iomgr_type as u32 {
        _ => {}
    };
}

unsafe fn stopIOManager() {
    match iomgr_type as u32 {
        _ => {}
    };
}

unsafe fn exitIOManager(mut wait_threads: bool) {
    match iomgr_type as u32 {
        _ => {}
    };
}

unsafe fn wakeupIOManager() {
    match iomgr_type as u32 {
        _ => {}
    };
}

pub(crate) unsafe fn markCapabilityIOManager(
    mut evac: evac_fn,
    mut user: *mut c_void,
    mut cap: *mut Capability,
) {
    match iomgr_type as u32 {
        0 => {
            let mut iomgr = (*cap).iomgr;
            evac.expect("non-null function pointer")(
                user,
                &raw mut (*iomgr).blocked_queue_hd as *mut c_void as *mut *mut StgClosure,
            );

            evac.expect("non-null function pointer")(
                user,
                &raw mut (*iomgr).blocked_queue_tl as *mut c_void as *mut *mut StgClosure,
            );

            evac.expect("non-null function pointer")(
                user,
                &raw mut (*iomgr).sleeping_queue as *mut c_void as *mut *mut StgClosure,
            );
        }
        _ => {}
    };
}

unsafe fn scavengeTSOIOManager(mut tso: *mut StgTSO) {
    match iomgr_type as u32 {
        _ => {}
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setIOManagerControlFd(mut cap_no: u32, mut fd: i32) {}

unsafe fn anyPendingTimeoutsOrIO(mut cap: *mut Capability) -> bool {
    match iomgr_type as u32 {
        0 => {
            let mut iomgr = (*cap).iomgr;

            return (*iomgr).blocked_queue_hd
                != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                || (*iomgr).sleeping_queue
                    != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
        }
        _ => {
            barf(c"anyPendingTimeoutsOrIO not implemented".as_ptr());
        }
    };
}

unsafe fn pollCompletedTimeoutsOrIO(mut cap: *mut Capability) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"polling for completed IO or timeouts".as_ptr());
    }

    match iomgr_type as u32 {
        0 => {
            awaitCompletedTimeoutsOrIOSelect(cap, false);
        }
        _ => {
            barf(c"pollCompletedTimeoutsOrIO not implemented".as_ptr());
        }
    };
}

unsafe fn awaitCompletedTimeoutsOrIO(mut cap: *mut Capability) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"waiting for completed IO or timeouts".as_ptr());
    }

    match iomgr_type as u32 {
        0 => {
            awaitCompletedTimeoutsOrIOSelect(cap, true);
        }
        _ => {
            barf(c"pollCompletedTimeoutsOrIO not implemented".as_ptr());
        }
    };
}

unsafe fn syncIOWaitReady(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut rw: IOReadOrWrite,
    mut fd: HsInt,
) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"thread %ld waiting for %s I/O readiness on fd %d".as_ptr(),
            (*tso).id as i64,
            if rw as u32 == IORead as i32 as u32 {
                c"read".as_ptr()
            } else {
                c"write".as_ptr()
            },
            fd as i32,
        );
    }

    match iomgr_type as u32 {
        0 => {
            let mut why_blocked: StgWord = (if rw as u32 == IORead as i32 as u32 {
                BlockedOnRead
            } else {
                BlockedOnWrite
            }) as StgWord;

            (*tso).block_info.fd = fd as StgInt;
            (*tso).why_blocked = why_blocked as StgWord32;
            appendToIOBlockedQueue(cap, tso);
        }
        _ => {
            barf(c"waitRead# / waitWrite# not available for current I/O manager".as_ptr());
        }
    };
}

unsafe fn syncIOCancel(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"cancelling I/O for thread %ld".as_ptr(), (*tso).id as i64);
    }

    match iomgr_type as u32 {
        0 => {
            removeThreadFromDeQueue(
                cap,
                &raw mut (*(*cap).iomgr).blocked_queue_hd,
                &raw mut (*(*cap).iomgr).blocked_queue_tl,
                tso,
            );
        }
        _ => {
            barf(
                c"syncIOCancel not supported for I/O manager %d".as_ptr(),
                iomgr_type as u32,
            );
        }
    };
}

unsafe fn syncDelay(mut cap: *mut Capability, mut tso: *mut StgTSO, mut us_delay: HsInt) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"thread %ld waiting for %lld us".as_ptr(),
            (*tso).id,
            us_delay,
        );
    }

    match iomgr_type as u32 {
        0 => {
            let mut target = getDelayTarget(us_delay);
            (*tso).block_info.target = target as StgWord;
            (*tso).why_blocked = 5;
            insertIntoSleepingQueue(cap, tso, target);
        }
        _ => {
            barf(
                c"syncDelay not supported for I/O manager %d".as_ptr(),
                iomgr_type as u32,
            );
        }
    };
}

unsafe fn syncDelayCancel(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"cancelling delay for thread %ld".as_ptr(),
            (*tso).id as i64,
        );
    }

    match iomgr_type as u32 {
        0 => {
            removeThreadFromQueue(cap, &raw mut (*(*cap).iomgr).sleeping_queue, tso);
        }
        _ => {
            barf(
                c"syncDelayCancel not supported for I/O manager %d".as_ptr(),
                iomgr_type as u32,
            );
        }
    };
}

unsafe fn appendToIOBlockedQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut iomgr = (*cap).iomgr;

    if (*iomgr).blocked_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
    {
        (*iomgr).blocked_queue_hd = tso;
    } else {
        setTSOLink(cap, (*iomgr).blocked_queue_tl, tso);
    }

    (*iomgr).blocked_queue_tl = tso;
}

unsafe fn insertIntoSleepingQueue(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut target: LowResTime,
) {
    let mut iomgr = (*cap).iomgr;
    let mut prev = null_mut::<StgTSO>();
    let mut t = (*iomgr).sleeping_queue;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        && (*t).block_info.target < target
    {
        prev = t;
        t = (*t)._link as *mut StgTSO;
    }

    (*tso)._link = t as *mut StgTSO_;

    if prev.is_null() {
        (*iomgr).sleeping_queue = tso;
    } else {
        setTSOLink(cap, prev, tso);
    };
}

unsafe fn is_io_mng_native_p() -> bool {
    match iomgr_type as u32 {
        _ => {}
    }

    return false;
}
