use crate::alloc_array::allocateArrBytes;
use crate::capability::recordClosureMutated;
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::closures::StgArrBytes;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::sm::non_moving_mark::updateRemembSetPushClosure;
use crate::trace::traceThreadLabel;

unsafe fn setThreadLabel(mut cap: *mut Capability, mut tso: *mut StgTSO, mut label: *mut c_char) {
    let mut len = strlen(label) as i32;

    let mut arr = allocateArrBytes(cap, len as StgWord, (*cap).r.rCCCS as *mut CostCentreStack);

    if (arr == null_mut::<c_void>() as *mut StgArrBytes) as i32 as i64 != 0 {
        return;
    }

    memcpy(
        &raw mut (*arr).payload as *mut c_void,
        label as *const c_void,
        len as usize,
    );
    labelThread(cap, tso, arr);
}

unsafe fn labelThread(mut cap: *mut Capability, mut tso: *mut StgTSO, mut label: *mut StgArrBytes) {
    if !(*tso).label.is_null() {
        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, (*tso).label as *mut StgClosure);
        }
    }

    recordClosureMutated(cap, tso as *mut StgClosure);
    (&raw mut (*tso).label).store(label, Ordering::Release);

    traceThreadLabel(
        cap,
        tso,
        &raw mut (*label).payload as *mut StgWord as *mut c_char,
        (*label).bytes as usize,
    );
}
