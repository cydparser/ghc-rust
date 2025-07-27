use super::*;
use crate::stg::types::{StgFunPtr, StgStablePtr};
use std::ffi::CString;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_createAdjustor(typeString: CString) -> bool {
    let hptr: StgStablePtr = Default::default();
    let wptr: StgFunPtr = Default::default();
    let expected = unsafe { sys::createAdjustor(hptr, wptr, typeString.as_ptr() as *mut c_char) };
    let actual = unsafe { createAdjustor(hptr, wptr, typeString.as_ptr() as *mut c_char) };
    actual == expected
}

#[test]
#[ignore]
fn test_createAdjustor() {
    let hptr = Default::default();
    let wptr = Default::default();
    #[expect(unused_mut)]
    let mut typeString = CString::from(c"TODO");
    unsafe { createAdjustor(hptr, wptr, typeString.as_ptr() as *mut c_char) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeHaskellFunctionPtr() {
    let ptr = null_mut(); // TODO
    unsafe { freeHaskellFunctionPtr(ptr) };
    todo!("assert")
}
