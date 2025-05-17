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
fn equivalent_closure_sizeW_(p: StgClosure, info: StgInfoTable) -> bool {
    let expected = unsafe { sys::closure_sizeW_(&p.into(), &info.into()) };
    let actual = unsafe { closure_sizeW_(&p, &info) };
    actual == expected
}

#[test]
#[ignore]
fn test_closure_sizeW_() {
    let p = null();
    let info = null();
    unsafe { closure_sizeW_(&p, &info) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingClosure() {
    let mut p = null_mut();
    unsafe { stg_overwritingClosure(&mut p) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingMutableClosureOfs() {
    let mut p = null_mut();
    let offset = Default::default();
    unsafe { stg_overwritingMutableClosureOfs(&mut p, offset) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingClosureSize() {
    let mut p = null_mut();
    let size = Default::default();
    unsafe { stg_overwritingClosureSize(&mut p, size) };
    todo!("assert")
}
