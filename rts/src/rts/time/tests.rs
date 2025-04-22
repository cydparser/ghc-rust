use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_TIME_RESOLUTION() {
    assert_eq!(sys::TIME_RESOLUTION, super::TIME_RESOLUTION);
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getProcessElapsedTime() -> bool {
    let expected = unsafe { transmute(sys::getProcessElapsedTime()) };
    let actual = unsafe { super::getProcessElapsedTime() };
    actual == expected
}

#[test]
#[ignore]
fn test_getProcessElapsedTime() {
    unsafe { super::getProcessElapsedTime() };
    todo!("assert")
}
