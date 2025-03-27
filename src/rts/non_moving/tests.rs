use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_updateRemembSetPushClosure_() {
    let reg = Default::default();
    let p = Default::default();
    unsafe { super::updateRemembSetPushClosure_(&mut reg, &mut p) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_updateRemembSetPushThunk_() {
    let reg = Default::default();
    let p = Default::default();
    unsafe { super::updateRemembSetPushThunk_(&mut reg, &mut p) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyArray_barrier() -> bool {
    let expected = unsafe { transmute(sys::stg_copyArray_barrier()) };
    let actual = unsafe { super::stg_copyArray_barrier() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyArray_barrier() {
    unsafe { super::stg_copyArray_barrier() };
    todo!("assert")
}
