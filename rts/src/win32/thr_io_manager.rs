use crate::ffi::hs_ffi::HsWord32;
use crate::ffi::rts::io_interface::{IO_MANAGER_DIE, IO_MANAGER_WAKEUP};
use crate::ffi::rts::messages::{errorBelch, sysErrorBelch};
use crate::ffi::rts::os_threads::{Mutex, initMutex};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts_api::{HaskellObj, rts_evalIO, rts_lock, rts_unlock};
use crate::ffi::stg::types::StgWord32;
use crate::io_manager::is_io_mng_native_p;
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut io_manager_event: HANDLE = unsafe { INVALID_HANDLE_VALUE };

const EVENT_BUFSIZ: i32 = 256;

static mut event_buf_mutex: Mutex = _RTL_SRWLOCK {
    Ptr: null_mut::<c_void>(),
};

static mut event_buf: [StgWord32; 256] = [0; 256];

static mut next_event: u32 = 0;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getIOManagerEvent() -> *mut c_void {
    let mut hRes = null_mut::<c_void>();
    AcquireSRWLockExclusive(&raw mut event_buf_mutex);

    if io_manager_event == INVALID_HANDLE_VALUE {
        hRes = CreateEventA(
            null_mut::<_SECURITY_ATTRIBUTES>(),
            true,
            false,
            null::<CHAR>(),
        );

        if hRes.is_null() {
            sysErrorBelch(c"getIOManagerEvent".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        io_manager_event = hRes;
    } else {
        hRes = io_manager_event;
    }

    ReleaseSRWLockExclusive(&raw mut event_buf_mutex);

    return hRes as *mut c_void;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn readIOManagerEvent() -> HsWord32 {
    let mut res: HsWord32 = 0;
    AcquireSRWLockExclusive(&raw mut event_buf_mutex);

    if io_manager_event != INVALID_HANDLE_VALUE {
        if next_event == 0 {
            res = 0;
        } else {
            loop {
                next_event = next_event.wrapping_sub(1);
                res = event_buf[next_event as usize];

                if !(res == IO_MANAGER_WAKEUP as HsWord32 && next_event != 0) {
                    break;
                }
            }

            if next_event == 0 {
                if ResetEvent(io_manager_event) == 0 {
                    sysErrorBelch(c"readIOManagerEvent".as_ptr());
                    stg_exit(EXIT_FAILURE);
                }
            }
        }
    } else {
        res = IO_MANAGER_DIE as HsWord32;
    }

    ReleaseSRWLockExclusive(&raw mut event_buf_mutex);

    return res;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn sendIOManagerEvent(mut event: HsWord32) {
    AcquireSRWLockExclusive(&raw mut event_buf_mutex);

    if io_manager_event != INVALID_HANDLE_VALUE {
        if next_event == EVENT_BUFSIZ as u32 {
            errorBelch(c"event buffer overflowed; event dropped".as_ptr());
        } else {
            let fresh28 = next_event;
            next_event = next_event.wrapping_add(1);
            event_buf[fresh28 as usize] = event;

            if SetEvent(io_manager_event) == 0 {
                sysErrorBelch(c"sendIOManagerEvent: SetEvent".as_ptr());
                stg_exit(EXIT_FAILURE);
            }
        }
    }

    ReleaseSRWLockExclusive(&raw mut event_buf_mutex);
}

unsafe fn interruptIOManagerEvent() {
    if is_io_mng_native_p() {
        AcquireSRWLockExclusive(&raw mut event_buf_mutex);

        let mut cap = null_mut::<Capability>();
        cap = rts_lock();

        rts_evalIO(
            &raw mut cap,
            (*ghc_hs_iface).interruptIOManager_closure as HaskellObj,
            null_mut::<HaskellObj>(),
        );

        rts_unlock(cap);
        ReleaseSRWLockExclusive(&raw mut event_buf_mutex);
    }
}

unsafe fn ioManagerWakeup() {
    sendIOManagerEvent(IO_MANAGER_WAKEUP as HsWord32);
}

unsafe fn ioManagerDie() {
    sendIOManagerEvent(IO_MANAGER_DIE as HsWord32);
    AcquireSRWLockExclusive(&raw mut event_buf_mutex);
    io_manager_event = INVALID_HANDLE_VALUE;
    ReleaseSRWLockExclusive(&raw mut event_buf_mutex);
}

unsafe fn ioManagerStart() {
    initMutex(&raw mut event_buf_mutex);
    next_event = 0;

    let mut cap = null_mut::<Capability>();

    if io_manager_event == INVALID_HANDLE_VALUE {
        cap = rts_lock();

        rts_evalIO(
            &raw mut cap,
            (*ghc_hs_iface).ensureIOManagerIsRunning_closure as HaskellObj,
            null_mut::<HaskellObj>(),
        );

        rts_unlock(cap);
    }
}

unsafe fn ioManagerFinished() {
    CloseHandle(io_manager_event);
}
