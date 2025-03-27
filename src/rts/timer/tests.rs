use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_startTimer() {
    unsafe { super::startTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stopTimer() {
    unsafe { super::stopTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsTimerSignal() -> bool {
    let expected = unsafe { transmute(sys::rtsTimerSignal()) };
    let actual = unsafe { super::rtsTimerSignal() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsTimerSignal() {
    unsafe { super::rtsTimerSignal() };
    todo!("assert")
}
