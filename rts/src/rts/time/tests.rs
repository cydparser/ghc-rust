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
#[test]
fn sys_eq_TIME_RESOLUTION() {
    assert_eq!(sys::TIME_RESOLUTION, TIME_RESOLUTION);
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getProcessElapsedTime() -> bool {
    let expected = unsafe { sys::getProcessElapsedTime() };
    let actual = unsafe { getProcessElapsedTime() };
    actual == expected
}

#[test]
#[ignore]
fn test_getProcessElapsedTime() {
    unsafe { getProcessElapsedTime() };
    todo!("assert")
}
