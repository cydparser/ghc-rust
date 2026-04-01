use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::stable_ptr::{deRefStablePtr, getStablePtr};
use crate::ffi::rts::storage::closures::StgWeak;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::stg::misc_closures::{stg_DEAD_WEAK_info, stg_TSO_info, stg_WEAK_info};
use crate::ffi::stg::types::{StgPtr, StgStablePtr};
use crate::prelude::*;
use crate::stable_ptr::freeStablePtr;

static mut m: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut topHandlerPtr: StgStablePtr = null_mut::<c_void>();

unsafe fn rts_setMainThread(mut weak: *mut StgWeak) {
    let mut __r = pthread_mutex_lock(&raw mut m);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/TopHandler.c".as_ptr(),
            12,
            __r,
        );
    }

    if !topHandlerPtr.is_null() {
        freeStablePtr(topHandlerPtr);
    }

    topHandlerPtr = getStablePtr(weak as StgPtr);

    if ((*weak).header.info == &raw const stg_WEAK_info) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TopHandler.c".as_ptr(), 18);
    }

    if ((*(*weak).key).header.info == &raw const stg_TSO_info) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TopHandler.c".as_ptr(), 22);
    }

    if pthread_mutex_unlock(&raw mut m) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/TopHandler.c".as_ptr(),
            24,
        );
    }
}

unsafe fn getTopHandlerThread() -> *mut StgTSO {
    let mut __r = pthread_mutex_lock(&raw mut m);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/TopHandler.c".as_ptr(),
            28,
            __r,
        );
    }

    let mut weak = deRefStablePtr(topHandlerPtr) as *mut StgWeak;

    if pthread_mutex_unlock(&raw mut m) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/TopHandler.c".as_ptr(),
            30,
        );
    }

    if weak.is_null() {
        return null_mut::<StgTSO>();
    }

    let mut info = (&raw mut (*weak).header.info).load(Ordering::Acquire);

    if info == &raw const stg_WEAK_info {
        let mut key = (*weak).key;

        if ((*key).header.info == &raw const stg_TSO_info) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/TopHandler.c".as_ptr(), 41);
        }

        return key as *mut StgTSO;
    } else if info == &raw const stg_DEAD_WEAK_info {
        return null_mut::<StgTSO>();
    } else {
        barf(
            c"getTopHandlerThread: neither a WEAK nor a DEAD_WEAK: %p %p %d".as_ptr(),
            weak,
            info,
            (*info).r#type,
        );
    };
}

unsafe fn initTopHandler() {
    initMutex(&raw mut m);
    topHandlerPtr = NULL as StgStablePtr;
}

unsafe fn exitTopHandler() {
    freeStablePtr(topHandlerPtr);
    topHandlerPtr = NULL as StgStablePtr;
    closeMutex(&raw mut m);
}
