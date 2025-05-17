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
fn equivalent___int_encodeDouble(j: I_, e: I_) -> bool {
    let expected = unsafe { sys::__int_encodeDouble(j, e) };
    let actual = unsafe { __int_encodeDouble(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___int_encodeDouble() {
    let j = Default::default();
    let e = Default::default();
    unsafe { __int_encodeDouble(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___int_encodeFloat(j: I_, e: I_) -> bool {
    let expected = unsafe { sys::__int_encodeFloat(j, e) };
    let actual = unsafe { __int_encodeFloat(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___int_encodeFloat() {
    let j = Default::default();
    let e = Default::default();
    unsafe { __int_encodeFloat(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___word_encodeDouble(j: W_, e: I_) -> bool {
    let expected = unsafe { sys::__word_encodeDouble(j, e) };
    let actual = unsafe { __word_encodeDouble(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___word_encodeDouble() {
    let j = Default::default();
    let e = Default::default();
    unsafe { __word_encodeDouble(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___word_encodeFloat(j: W_, e: I_) -> bool {
    let expected = unsafe { sys::__word_encodeFloat(j, e) };
    let actual = unsafe { __word_encodeFloat(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___word_encodeFloat() {
    let j = Default::default();
    let e = Default::default();
    unsafe { __word_encodeFloat(j, e) };
    todo!("assert")
}
