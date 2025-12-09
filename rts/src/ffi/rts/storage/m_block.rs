#[cfg(feature = "ghc_testsuite")]
use crate::ffi::stg::W_;
#[cfg(feature = "sys")]
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
pub static mut mblocks_allocated: W_ = 0;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getMBlocks(n: u32) -> *mut c_void {
    sys! {
        getMBlocks(n)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeMBlocks(addr: *mut c_void, n: u32) {
    sys! {
        freeMBlocks(addr, n)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn releaseFreeMemory() {
    sys! {
        releaseFreeMemory()
    }
}
