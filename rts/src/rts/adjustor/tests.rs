use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createAdjustor(hptr: StgStablePtr, wptr: StgFunPtr, typeString: c_char) -> bool {
    let expected = unsafe { sys::createAdjustor(hptr, wptr, &mut typeString) };
    let actual = unsafe { createAdjustor(hptr, wptr, &mut typeString) };
    actual == expected
}

#[test]
#[ignore]
fn test_createAdjustor() {
    let hptr = Default::default();
    let wptr = Default::default();
    let mut typeString = null_mut();
    unsafe { createAdjustor(hptr, wptr, &mut typeString) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeHaskellFunctionPtr() {
    let mut ptr = null_mut();
    unsafe { freeHaskellFunctionPtr(&mut ptr) };
    todo!("assert")
}
