use crate::capability::Capability;
use crate::capability::{CapIOManager, getCapability};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::stg::types::StgWord32;
use crate::hs_ffi::HsInt;
use crate::io_manager::{
    IO_MANAGER_MIO_POSIX, IOManagerAvailability, IOManagerAvailable, IOManagerType,
    IOManagerUnavailable, IOManagerUnrecognised, IORead, IOReadOrWrite,
};
use crate::posix::signals::{ioManagerDie, ioManagerStart, ioManagerStartCap, ioManagerWakeup};
use crate::prelude::*;
use crate::rts_api::{HaskellObj, rts_evalIO};
use crate::rts_flags::{IO_MANAGER_FLAG, IO_MNGR_FLAG_AUTO, IO_MNGR_FLAG_MIO, RtsFlags};
use crate::rts_utils::stgMallocBytes;
use crate::sm::gc::evac_fn;
use crate::trace::{DEBUG_RTS, trace_};

#[cfg(test)]
mod tests;

pub(crate) type CapIOManager = _CapIOManager;

/// cbindgen:no-export
pub(crate) struct _CapIOManager {
    pub(crate) control_fd: i32,
}

pub(crate) type IOManagerType = u32;

pub(crate) const IO_MANAGER_MIO_POSIX: IOManagerType = 0;

pub(crate) type IOManagerAvailability = u32;

pub(crate) const IOManagerUnrecognised: IOManagerAvailability = 2;

pub(crate) const IOManagerUnavailable: IOManagerAvailability = 1;

pub(crate) const IOManagerAvailable: IOManagerAvailability = 0;

pub(crate) type IOReadOrWrite = u32;

pub(crate) const IOWrite: IOReadOrWrite = 1;

pub(crate) const IORead: IOReadOrWrite = 0;

static mut iomgr_type: IOManagerType = IO_MANAGER_MIO_POSIX;

unsafe fn parseIOManagerFlag(
    mut iomgrstr: *const c_char,
    mut flag: *mut IO_MANAGER_FLAG,
) -> IOManagerAvailability {
    if strcmp(c"select".as_ptr(), iomgrstr) == 0 {
        return IOManagerUnavailable;
    } else if strcmp(c"mio".as_ptr(), iomgrstr) == 0 {
        *flag = IO_MNGR_FLAG_MIO;

        return IOManagerAvailable;
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
            iomgr_type = IO_MANAGER_MIO_POSIX;
        }
        2 => {
            iomgr_type = IO_MANAGER_MIO_POSIX;
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
            return c"mio".as_ptr();
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
            (*iomgr).control_fd = -1;
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
            ioManagerStart();
        }
        _ => {}
    };
}

unsafe fn initIOManagerAfterFork(mut pcap: *mut *mut Capability) {
    match iomgr_type as u32 {
        0 => {
            ioManagerStartCap(pcap);
        }
        _ => {}
    };
}

unsafe fn notifyIOManagerCapabilitiesChanged(mut pcap: *mut *mut Capability) {
    match iomgr_type as u32 {
        0 => {
            rts_evalIO(
                pcap,
                (*ghc_hs_iface).ioManagerCapabilitiesChanged_closure as HaskellObj,
                null_mut::<HaskellObj>(),
            );
        }
        _ => {}
    };
}

unsafe fn stopIOManager() {
    match iomgr_type as u32 {
        0 => {
            ioManagerDie();
        }
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
        0 => {
            ioManagerWakeup();
        }
        _ => {}
    };
}

unsafe fn markCapabilityIOManager(
    mut evac: evac_fn,
    mut user: *mut c_void,
    mut cap: *mut Capability,
) {
    match iomgr_type as u32 {
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
pub unsafe extern "C" fn setIOManagerControlFd(mut cap_no: c_uint, mut fd: c_int) {
    if cap_no < getNumCapabilities() as u32 {
        (&raw mut (*(*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(cap_no))
            .iomgr)
            .control_fd)
            .store(fd, Ordering::Relaxed);
    } else {
        errorBelch(
            c"warning: setIOManagerControlFd called with illegal capability number.".as_ptr(),
        );
    };
}

unsafe fn anyPendingTimeoutsOrIO(mut cap: *mut Capability) -> bool {
    match iomgr_type as u32 {
        0 => return false,
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
        _ => {}
    }

    barf(c"pollCompletedTimeoutsOrIO not implemented".as_ptr());
}

unsafe fn awaitCompletedTimeoutsOrIO(mut cap: *mut Capability) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"waiting for completed IO or timeouts".as_ptr());
    }

    match iomgr_type as u32 {
        _ => {}
    }

    barf(c"pollCompletedTimeoutsOrIO not implemented".as_ptr());
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

    if ((*tso).why_blocked == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/IOManager.c".as_ptr(), 720);
    }

    match iomgr_type as u32 {
        _ => {}
    }

    barf(c"waitRead# / waitWrite# not available for current I/O manager".as_ptr());
}

unsafe fn syncIOCancel(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(c"cancelling I/O for thread %ld".as_ptr(), (*tso).id as i64);
    }

    match iomgr_type as u32 {
        _ => {}
    }

    barf(
        c"syncIOCancel not supported for I/O manager %d".as_ptr(),
        iomgr_type as u32,
    );
}

unsafe fn syncDelay(mut cap: *mut Capability, mut tso: *mut StgTSO, mut us_delay: HsInt) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"thread %ld waiting for %lld us".as_ptr(),
            (*tso).id,
            us_delay,
        );
    }

    if ((*tso).why_blocked == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/IOManager.c".as_ptr(), 769);
    }

    match iomgr_type as u32 {
        _ => {}
    }

    barf(
        c"syncDelay not supported for I/O manager %d".as_ptr(),
        iomgr_type as u32,
    );
}

unsafe fn syncDelayCancel(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.iomanager as i64 != 0 {
        trace_(
            c"cancelling delay for thread %ld".as_ptr(),
            (*tso).id as i64,
        );
    }

    match iomgr_type as u32 {
        _ => {}
    }

    barf(
        c"syncDelayCancel not supported for I/O manager %d".as_ptr(),
        iomgr_type as u32,
    );
}

unsafe fn is_io_mng_native_p() -> bool {
    match iomgr_type as u32 {
        _ => {}
    }

    return false;
}
