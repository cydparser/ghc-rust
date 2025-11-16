use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeHaskellFunctionPtr(ptr: *mut c_void) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::freeHaskellFunctionPtr(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("freeHaskellFunctionPtr")
}
