use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_vbarf(s: ::core::ffi::c_char, ap: __va_list_tag) -> bool {
    let expected = unsafe { transmute(sys::vbarf(&s.into(), &mut ap.into())) };
    let actual = unsafe { super::vbarf(&s, &mut ap) };
    actual == expected
}

#[test]
#[ignore]
fn test_vbarf() {
    let s = Default::default();
    let ap = Default::default();
    unsafe { super::vbarf(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_verrorBelch() {
    let s = Default::default();
    let ap = Default::default();
    unsafe { super::verrorBelch(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_vsysErrorBelch() {
    let s = Default::default();
    let ap = Default::default();
    unsafe { super::vsysErrorBelch(&s, &mut ap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_vdebugBelch(s: ::core::ffi::c_char, ap: __va_list_tag) -> bool {
    let expected = unsafe { transmute(sys::vdebugBelch(&s.into(), &mut ap.into())) };
    let actual = unsafe { super::vdebugBelch(&s, &mut ap) };
    actual == expected
}

#[test]
#[ignore]
fn test_vdebugBelch() {
    let s = Default::default();
    let ap = Default::default();
    unsafe { super::vdebugBelch(&s, &mut ap) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsFatalInternalErrorFn() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    unsafe { super::rtsFatalInternalErrorFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsDebugMsgFn(arg1: ::core::ffi::c_char, arg2: __va_list_tag) -> bool {
    let expected = unsafe { transmute(sys::rtsDebugMsgFn(&arg1.into(), &mut arg2.into())) };
    let actual = unsafe { super::rtsDebugMsgFn(&arg1, &mut arg2) };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsDebugMsgFn() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    unsafe { super::rtsDebugMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsErrorMsgFn() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    unsafe { super::rtsErrorMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rtsSysErrorMsgFn() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    unsafe { super::rtsSysErrorMsgFn(&arg1, &mut arg2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsBadAlignmentBarf() -> bool {
    let expected = unsafe { transmute(sys::rtsBadAlignmentBarf()) };
    let actual = unsafe { super::rtsBadAlignmentBarf() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsBadAlignmentBarf() {
    unsafe { super::rtsBadAlignmentBarf() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsOutOfBoundsAccess() -> bool {
    let expected = unsafe { transmute(sys::rtsOutOfBoundsAccess()) };
    let actual = unsafe { super::rtsOutOfBoundsAccess() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsOutOfBoundsAccess() {
    unsafe { super::rtsOutOfBoundsAccess() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsMemcpyRangeOverlap() -> bool {
    let expected = unsafe { transmute(sys::rtsMemcpyRangeOverlap()) };
    let actual = unsafe { super::rtsMemcpyRangeOverlap() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsMemcpyRangeOverlap() {
    unsafe { super::rtsMemcpyRangeOverlap() };
    todo!("assert")
}
