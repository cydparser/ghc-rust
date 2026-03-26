use crate::ffi::rts::messages::barf;
use crate::ffi::rts::stable_ptr::{deRefStablePtr, getStablePtr};
use crate::ffi::rts::storage::closures::StgWeak;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::stg::misc_closures::{stg_DEAD_WEAK_info, stg_WEAK_info};
use crate::ffi::stg::types::{StgPtr, StgStablePtr};
use crate::prelude::*;
use crate::stable_ptr::freeStablePtr;

static mut topHandlerPtr: StgStablePtr = null::<c_void>() as *mut c_void;

unsafe fn rts_setMainThread(mut weak: *mut StgWeak) {
    if !topHandlerPtr.is_null() {
        freeStablePtr(topHandlerPtr);
    }

    topHandlerPtr = getStablePtr(weak as StgPtr);
}

unsafe fn getTopHandlerThread() -> *mut StgTSO {
    let mut weak = deRefStablePtr(topHandlerPtr) as *mut StgWeak;

    if weak.is_null() {
        return null_mut::<StgTSO>();
    }

    let mut info = (*weak).header.info;

    if info == &raw const stg_WEAK_info {
        let mut key = (*weak).key;

        return key as *mut StgTSO;
    } else if info == &raw const stg_DEAD_WEAK_info {
        return null_mut::<StgTSO>();
    } else {
        barf(
            b"getTopHandlerThread: neither a WEAK nor a DEAD_WEAK: %p %p %d\0" as *const u8
                as *const c_char,
            weak,
            info,
            (*info).r#type,
        );
    };
}

unsafe fn initTopHandler() {
    topHandlerPtr = NULL as StgStablePtr;
}

unsafe fn exitTopHandler() {
    freeStablePtr(topHandlerPtr);
    topHandlerPtr = NULL as StgStablePtr;
}
