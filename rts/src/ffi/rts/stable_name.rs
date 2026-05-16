use crate::ffi::rts::storage::closures::StgClosure;
use crate::stg::types::StgPtr;

/// cbindgen:no-export
#[repr(C)]
pub struct snEntry {
    pub(crate) addr: StgPtr,
    pub(crate) old: StgPtr,
    pub(crate) sn_obj: *mut StgClosure,
}
