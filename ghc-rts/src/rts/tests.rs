use core::ffi;
use std::mem::transmute;

use quickcheck_macros::quickcheck;

#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq_IN_STG_CODE() {
    assert_eq!(sys::IN_STG_CODE, super::IN_STG_CODE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__REENTRANT() {
    assert_eq!(sys::_REENTRANT, super::_REENTRANT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_SizeT() {
    assert_eq!(sys::FMT_SizeT, super::FMT_SizeT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexSizeT() {
    assert_eq!(sys::FMT_HexSizeT, super::FMT_HexSizeT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_EXIT_INTERNAL_ERROR() {
    assert_eq!(sys::EXIT_INTERNAL_ERROR, super::EXIT_INTERNAL_ERROR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_EXIT_DEADLOCK() {
    assert_eq!(sys::EXIT_DEADLOCK, super::EXIT_DEADLOCK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_EXIT_INTERRUPTED() {
    assert_eq!(sys::EXIT_INTERRUPTED, super::EXIT_INTERRUPTED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_EXIT_HEAPOVERFLOW() {
    assert_eq!(sys::EXIT_HEAPOVERFLOW, super::EXIT_HEAPOVERFLOW.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_EXIT_KILLED() {
    assert_eq!(sys::EXIT_KILLED, super::EXIT_KILLED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_DEBUG_IS_ON() {
    assert_eq!(sys::DEBUG_IS_ON, super::DEBUG_IS_ON.into());
}

#[test]
#[ignore]
fn test__assertFail() {
    let filename = Default::default();
    let linenum = Default::default();
    unsafe { super::_assertFail(&filename, linenum) };
    todo!("assert")
}

#[test]
#[ignore]
fn test__warnFail() {
    let filename = Default::default();
    let linenum = Default::default();
    unsafe { super::_warnFail(&filename, linenum) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_reportStackOverflow() {
    let tso = Default::default();
    unsafe { super::reportStackOverflow(&mut tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_reportHeapOverflow() {
    unsafe { super::reportHeapOverflow() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_stg_exit() {
    let n = Default::default();
    unsafe { super::stg_exit(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_sig_install(arg1: ffi::c_int, arg2: ffi::c_int) -> bool {
    let arg3 = todo!("*void");
    let expected = unsafe { sys::stg_sig_install(arg1.into(), arg2.into(), &mut arg3) };
    let arg3 = todo!("*void");
    let actual = unsafe { super::stg_sig_install(arg1, arg2, &mut arg3) };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_sig_install() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    let arg3 = todo!("*void");
    unsafe { super::stg_sig_install(arg1, arg2, &mut arg3) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isProfiled() -> bool {
    let expected = unsafe { transmute(sys::rts_isProfiled()) };
    let actual = unsafe { super::rts_isProfiled() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isProfiled() {
    unsafe { super::rts_isProfiled() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isDynamic() -> bool {
    let expected = unsafe { transmute(sys::rts_isDynamic()) };
    let actual = unsafe { super::rts_isDynamic() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isDynamic() {
    unsafe { super::rts_isDynamic() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isThreaded() -> bool {
    let expected = unsafe { transmute(sys::rts_isThreaded()) };
    let actual = unsafe { super::rts_isThreaded() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isThreaded() {
    unsafe { super::rts_isThreaded() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isDebugged() -> bool {
    let expected = unsafe { transmute(sys::rts_isDebugged()) };
    let actual = unsafe { super::rts_isDebugged() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isDebugged() {
    unsafe { super::rts_isDebugged() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isTracing() -> bool {
    let expected = unsafe { transmute(sys::rts_isTracing()) };
    let actual = unsafe { super::rts_isTracing() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isTracing() {
    unsafe { super::rts_isTracing() };
    todo!("assert")
}
