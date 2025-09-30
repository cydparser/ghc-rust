use crate::prelude::*;
use crate::stg::types::{StgWord32, StgWord64};

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
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_hpc_rootModule"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_hpc_rootModule() -> *mut HpcModuleInfo {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_hpc_rootModule() as *mut HpcModuleInfo
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_hpc_rootModule")
}
