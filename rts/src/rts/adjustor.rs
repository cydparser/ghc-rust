use crate::prelude::*;
use crate::stg::types::{StgFunPtr, StgStablePtr};

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createAdjustor"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createAdjustor(
    hptr: StgStablePtr,
    wptr: StgFunPtr,
    typeString: *mut c_char,
) -> *mut c_void {
    unsafe { sys::createAdjustor(hptr, wptr, typeString) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeHaskellFunctionPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeHaskellFunctionPtr(ptr: *mut c_void) {
    unsafe { sys::freeHaskellFunctionPtr(ptr) }
}
