use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[test]
#[ignore]
fn test_updateRemembSetPushClosure_() {
    let mut reg = null_mut();
    let mut p = null_mut();
    unsafe { updateRemembSetPushClosure_(&mut reg, &mut p) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_updateRemembSetPushThunk_() {
    let mut reg = null_mut();
    let mut p = null_mut();
    unsafe { updateRemembSetPushThunk_(&mut reg, &mut p) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyArray_barrier() -> bool {
    let expected = unsafe { sys::stg_copyArray_barrier() };
    let actual = unsafe { stg_copyArray_barrier() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyArray_barrier() {
    unsafe { stg_copyArray_barrier() };
    todo!("assert")
}
