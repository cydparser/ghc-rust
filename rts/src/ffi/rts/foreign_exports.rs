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

#[ffi(compiler)]
#[repr(C)]
pub struct ForeignExportsList {
    pub next: *mut ForeignExportsList,
    pub n_entries: c_int,
    pub oc: *mut _ObjectCode,
    pub stable_ptrs: *mut *mut StgStablePtr,
    pub exports: __IncompleteArrayField<StgPtr>,
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn registerForeignExports(exports: *mut ForeignExportsList) {
    sys! {
        registerForeignExports(exports as * mut sys::ForeignExportsList)
    }
}
