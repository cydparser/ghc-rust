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

#[cfg(feature = "sys")]
impl From<_HpcModuleInfo> for sys::_HpcModuleInfo {
    fn from(x: _HpcModuleInfo) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries}
pub type HpcModuleInfo = _HpcModuleInfo;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_hpc_rootModule() as *mut HpcModuleInfo
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_hpc_rootModule")
}
