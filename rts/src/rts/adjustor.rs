use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeHaskellFunctionPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn freeHaskellFunctionPtr(ptr: *mut c_void) {
    unsafe { sys::freeHaskellFunctionPtr(ptr) }
}
