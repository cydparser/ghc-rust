use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createAdjustor(
    hptr: StgStablePtr,
    wptr: StgFunPtr,
    typeString: *mut c_char,
) -> *mut c_void {
    sys! {
        createAdjustor(hptr, wptr, typeString)
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeHaskellFunctionPtr(ptr: *mut c_void) {
    sys! {
        freeHaskellFunctionPtr(ptr)
    }
}
