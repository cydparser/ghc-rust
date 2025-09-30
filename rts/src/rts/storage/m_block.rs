#[cfg(feature = "sys")]
use crate::prelude::*;
#[cfg(feature = "ghc_testsuite")]
use crate::stg::W_;

#[cfg(test)]
mod tests;

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_mblocks_allocated"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut mblocks_allocated: W_ = 0;

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getMBlocks"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getMBlocks(n: u32) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getMBlocks(n)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getMBlocks")
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeMBlocks"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn freeMBlocks(addr: *mut c_void, n: u32) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::freeMBlocks(addr, n)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("freeMBlocks")
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_releaseFreeMemory"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn releaseFreeMemory() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::releaseFreeMemory()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("releaseFreeMemory")
}
