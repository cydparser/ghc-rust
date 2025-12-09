use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::stg::types::StgPtr;

/// cbindgen:no-export
#[repr(C)]
pub struct snEntry {
    addr: StgPtr,
    old: StgPtr,
    sn_obj: *mut StgClosure,
}
