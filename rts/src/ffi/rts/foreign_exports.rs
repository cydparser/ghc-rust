use crate::ffi::stg::types::{StgPtr, StgStablePtr};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _ObjectCode {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<_ObjectCode> for sys::_ObjectCode {
    fn from(x: _ObjectCode) -> Self {
        unsafe { transmute(x) }
    }
}

/// cbindgen:no-export
#[repr(C)]
pub struct ForeignExportsList {
    next: *mut ForeignExportsList,
    n_entries: c_int,
    oc: *mut _ObjectCode,
    stable_ptrs: *mut *mut StgStablePtr,
    exports: __IncompleteArrayField<StgPtr>,
}

#[cfg(feature = "sys")]
impl From<ForeignExportsList> for sys::ForeignExportsList {
    fn from(x: ForeignExportsList) -> Self {
        unsafe { transmute(x) }
    }
}
