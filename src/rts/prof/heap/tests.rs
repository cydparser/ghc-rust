use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_requestHeapCensus() {
    unsafe { super::requestHeapCensus() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startHeapProfTimer() {
    unsafe { super::startHeapProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stopHeapProfTimer() {
    unsafe { super::stopHeapProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setUserEra() {
    let w = Default::default();
    unsafe { super::setUserEra(w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getUserEra() -> bool {
    let expected = unsafe { transmute(sys::getUserEra()) };
    let actual = unsafe { super::getUserEra() };
    actual == expected
}

#[test]
#[ignore]
fn test_getUserEra() {
    unsafe { super::getUserEra() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_incrementUserEra(w: StgWord) -> bool {
    let expected = unsafe { transmute(sys::incrementUserEra(w.into())) };
    let actual = unsafe { super::incrementUserEra(w) };
    actual == expected
}

#[test]
#[ignore]
fn test_incrementUserEra() {
    let w = Default::default();
    unsafe { super::incrementUserEra(w) };
    todo!("assert")
}
