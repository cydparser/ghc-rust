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
fn equivalent_getMonotonicNSec() -> bool {
    let expected = unsafe { sys::getMonotonicNSec() };
    let actual = unsafe { getMonotonicNSec() };
    actual == expected
}

#[test]
#[ignore]
fn test_getMonotonicNSec() {
    unsafe { getMonotonicNSec() };
    todo!("assert")
}
