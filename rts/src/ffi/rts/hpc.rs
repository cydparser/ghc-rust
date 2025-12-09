use crate::ffi::stg::types::{StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _HpcModuleInfo {
    modName: *mut c_char,
    tickCount: StgWord32,
    hashNo: StgWord32,
    tixArr: *mut StgWord64,
    from_file: bool,
    next: *mut _HpcModuleInfo,
}

#[ffi(libraries)]
pub type HpcModuleInfo = _HpcModuleInfo;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_hpc_module(
    modName: *mut c_char,
    modCount: StgWord32,
    modHashNo: StgWord32,
    tixArr: *mut StgWord64,
) {
    sys! {
        hs_hpc_module(modName, modCount, modHashNo, tixArr)
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    sys! {
        hs_hpc_rootModule().cast()
    }
}
