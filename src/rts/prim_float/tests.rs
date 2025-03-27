use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___int_encodeDouble(j: I_, e: I_) -> bool {
    let expected = unsafe { transmute(sys::__int_encodeDouble(j.into(), e.into())) };
    let actual = unsafe { super::__int_encodeDouble(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___int_encodeDouble() {
    let j = Default::default();
    let e = Default::default();
    unsafe { super::__int_encodeDouble(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___int_encodeFloat(j: I_, e: I_) -> bool {
    let expected = unsafe { transmute(sys::__int_encodeFloat(j.into(), e.into())) };
    let actual = unsafe { super::__int_encodeFloat(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___int_encodeFloat() {
    let j = Default::default();
    let e = Default::default();
    unsafe { super::__int_encodeFloat(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___word_encodeDouble(j: W_, e: I_) -> bool {
    let expected = unsafe { transmute(sys::__word_encodeDouble(j.into(), e.into())) };
    let actual = unsafe { super::__word_encodeDouble(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___word_encodeDouble() {
    let j = Default::default();
    let e = Default::default();
    unsafe { super::__word_encodeDouble(j, e) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___word_encodeFloat(j: W_, e: I_) -> bool {
    let expected = unsafe { transmute(sys::__word_encodeFloat(j.into(), e.into())) };
    let actual = unsafe { super::__word_encodeFloat(j, e) };
    actual == expected
}

#[test]
#[ignore]
fn test___word_encodeFloat() {
    let j = Default::default();
    let e = Default::default();
    unsafe { super::__word_encodeFloat(j, e) };
    todo!("assert")
}
