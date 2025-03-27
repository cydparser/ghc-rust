use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_libdwPoolTake() -> bool {
    let expected = unsafe { transmute(sys::libdwPoolTake()) };
    let actual = unsafe { super::libdwPoolTake() };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwPoolTake() {
    unsafe { super::libdwPoolTake() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPoolRelease() {
    let sess = Default::default();
    unsafe { super::libdwPoolRelease(&mut sess) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPoolClear() {
    unsafe { super::libdwPoolClear() };
    todo!("assert")
}
