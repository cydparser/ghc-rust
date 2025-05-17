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
fn equivalent_getOrSetGHCConcSignalSignalHandlerStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetGHCConcSignalSignalHandlerStore(ptr) };
    let actual = unsafe { getOrSetGHCConcSignalSignalHandlerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcSignalSignalHandlerStore() {
    let ptr = Default::default();
    unsafe { getOrSetGHCConcSignalSignalHandlerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsPendingDelaysStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr) };
    let actual = unsafe { getOrSetGHCConcWindowsPendingDelaysStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsPendingDelaysStore() {
    let ptr = Default::default();
    unsafe { getOrSetGHCConcWindowsPendingDelaysStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr) };
    let actual = unsafe { getOrSetGHCConcWindowsIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { getOrSetGHCConcWindowsIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetGHCConcWindowsProddingStore(ptr) };
    let actual = unsafe { getOrSetGHCConcWindowsProddingStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsProddingStore() {
    let ptr = Default::default();
    unsafe { getOrSetGHCConcWindowsProddingStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemEventThreadEventManagerStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetSystemEventThreadEventManagerStore(ptr) };
    let actual = unsafe { getOrSetSystemEventThreadEventManagerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemEventThreadEventManagerStore() {
    let ptr = Default::default();
    unsafe { getOrSetSystemEventThreadEventManagerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemEventThreadIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetSystemEventThreadIOManagerThreadStore(ptr) };
    let actual = unsafe { getOrSetSystemEventThreadIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemEventThreadIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { getOrSetSystemEventThreadIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemTimerThreadEventManagerStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetSystemTimerThreadEventManagerStore(ptr) };
    let actual = unsafe { getOrSetSystemTimerThreadEventManagerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemTimerThreadEventManagerStore() {
    let ptr = Default::default();
    unsafe { getOrSetSystemTimerThreadEventManagerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemTimerThreadIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetSystemTimerThreadIOManagerThreadStore(ptr) };
    let actual = unsafe { getOrSetSystemTimerThreadIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemTimerThreadIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { getOrSetSystemTimerThreadIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetLibHSghcFastStringTable(ptr) };
    let actual = unsafe { getOrSetLibHSghcFastStringTable(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcFastStringTable() {
    let ptr = Default::default();
    unsafe { getOrSetLibHSghcFastStringTable(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasPprDebug(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetLibHSghcGlobalHasPprDebug(ptr) };
    let actual = unsafe { getOrSetLibHSghcGlobalHasPprDebug(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasPprDebug() {
    let ptr = Default::default();
    unsafe { getOrSetLibHSghcGlobalHasPprDebug(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasNoDebugOutput(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) };
    let actual = unsafe { getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasNoDebugOutput() {
    let ptr = Default::default();
    unsafe { getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasNoStateHack(ptr: StgStablePtr) -> bool {
    let expected = unsafe { sys::getOrSetLibHSghcGlobalHasNoStateHack(ptr) };
    let actual = unsafe { getOrSetLibHSghcGlobalHasNoStateHack(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasNoStateHack() {
    let ptr = Default::default();
    unsafe { getOrSetLibHSghcGlobalHasNoStateHack(ptr) };
    todo!("assert")
}
