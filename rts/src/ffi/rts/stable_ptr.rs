use crate::stg::types::StgPtr;

/// cbindgen:no-export
#[repr(C)]
pub struct spEntry {
    addr: StgPtr,
}
