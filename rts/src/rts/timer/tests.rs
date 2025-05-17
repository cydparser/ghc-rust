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
fn test_startTimer() {
    unsafe { startTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stopTimer() {
    unsafe { stopTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsTimerSignal() -> bool {
    let expected = unsafe { sys::rtsTimerSignal() };
    let actual = unsafe { rtsTimerSignal() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsTimerSignal() {
    unsafe { rtsTimerSignal() };
    todo!("assert")
}
