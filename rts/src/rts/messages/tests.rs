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
fn equivalent_vbarf(s: c_char, ap: __va_list_tag) -> bool {
    let expected = unsafe { sys::vbarf(&s, &mut ap.into()) };
    let actual = unsafe { vbarf(&s, &mut ap) };
    actual == expected
}

#[test]
#[ignore]
fn test_vbarf() {
    let s = null();
    let mut ap = null_mut();
    unsafe { vbarf(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_verrorBelch() {
    let s = null();
    let mut ap = null_mut();
    unsafe { verrorBelch(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_vsysErrorBelch() {
    let s = null();
    let mut ap = null_mut();
    unsafe { vsysErrorBelch(&s, &mut ap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_vdebugBelch(s: c_char, ap: __va_list_tag) -> bool {
    let expected = unsafe { sys::vdebugBelch(&s, &mut ap.into()) };
    let actual = unsafe { vdebugBelch(&s, &mut ap) };
    actual == expected
}

#[test]
#[ignore]
fn test_vdebugBelch() {
    let s = null();
    let mut ap = null_mut();
    unsafe { vdebugBelch(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsFatalInternalErrorFn() {
    let arg1 = null();
    let mut arg2 = null_mut();
    unsafe { rtsFatalInternalErrorFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsDebugMsgFn(arg1: c_char, arg2: __va_list_tag) -> bool {
    let expected = unsafe { sys::rtsDebugMsgFn(&arg1, &mut arg2.into()) };
    let actual = unsafe { rtsDebugMsgFn(&arg1, &mut arg2) };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsDebugMsgFn() {
    let arg1 = null();
    let mut arg2 = null_mut();
    unsafe { rtsDebugMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsErrorMsgFn() {
    let arg1 = null();
    let mut arg2 = null_mut();
    unsafe { rtsErrorMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsSysErrorMsgFn() {
    let arg1 = null();
    let mut arg2 = null_mut();
    unsafe { rtsSysErrorMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsBadAlignmentBarf() -> bool {
    let expected = unsafe { sys::rtsBadAlignmentBarf() };
    let actual = unsafe { rtsBadAlignmentBarf() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsBadAlignmentBarf() {
    unsafe { rtsBadAlignmentBarf() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsOutOfBoundsAccess() -> bool {
    let expected = unsafe { sys::rtsOutOfBoundsAccess() };
    let actual = unsafe { rtsOutOfBoundsAccess() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsOutOfBoundsAccess() {
    unsafe { rtsOutOfBoundsAccess() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsMemcpyRangeOverlap() -> bool {
    let expected = unsafe { sys::rtsMemcpyRangeOverlap() };
    let actual = unsafe { rtsMemcpyRangeOverlap() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsMemcpyRangeOverlap() {
    unsafe { rtsMemcpyRangeOverlap() };
    todo!("assert")
}
