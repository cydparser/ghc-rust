use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_closure_sizeW_(p: StgClosure, info: StgInfoTable) -> bool {
    let expected = unsafe { transmute(sys::closure_sizeW_(&p.into(), &info.into())) };
    let actual = unsafe { super::closure_sizeW_(&p, &info) };
    actual == expected
}

#[test]
#[ignore]
fn test_closure_sizeW_() {
    let p = Default::default();
    let info = Default::default();
    unsafe { super::closure_sizeW_(&p, &info) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingClosure() {
    let p = Default::default();
    unsafe { super::stg_overwritingClosure(&mut p) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingMutableClosureOfs() {
    let p = Default::default();
    let offset = Default::default();
    unsafe { super::stg_overwritingMutableClosureOfs(&mut p, offset) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_overwritingClosureSize() {
    let p = Default::default();
    let size = Default::default();
    unsafe { super::stg_overwritingClosureSize(&mut p, size) };
    todo!("assert")
}
