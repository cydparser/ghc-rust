use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getMonotonicNSec() -> bool {
    let expected = unsafe { transmute(sys::getMonotonicNSec()) };
    let actual = unsafe { super::getMonotonicNSec() };
    actual == expected
}

#[test]
#[ignore]
fn test_getMonotonicNSec() {
    unsafe { super::getMonotonicNSec() };
    todo!("assert")
}
