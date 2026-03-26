use crate::ffi::stg::types::StgStablePtr;
pub use crate::foreign_exports::registerForeignExports;
pub use crate::linker::_ObjectCode;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[repr(C)]
pub struct ForeignExportsList {
    pub next: *mut ForeignExportsList,
    pub n_entries: c_int,
    pub oc: *mut _ObjectCode,
    pub stable_ptrs: *mut *mut StgStablePtr,
    pub exports: __IncompleteArrayField<StgPtr>,
}
