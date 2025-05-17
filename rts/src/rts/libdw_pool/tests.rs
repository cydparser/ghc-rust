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
fn equivalent_libdwPoolTake() -> bool {
    let expected = unsafe { transmute(sys::libdwPoolTake()) };
    let actual = unsafe { libdwPoolTake() };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwPoolTake() {
    unsafe { libdwPoolTake() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPoolRelease() {
    let mut sess = null_mut();
    unsafe { libdwPoolRelease(&mut sess) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPoolClear() {
    unsafe { libdwPoolClear() };
    todo!("assert")
}
