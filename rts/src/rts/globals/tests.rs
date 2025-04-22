use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcSignalSignalHandlerStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetGHCConcSignalSignalHandlerStore(ptr.into())) };
    let actual = unsafe { super::getOrSetGHCConcSignalSignalHandlerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcSignalSignalHandlerStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetGHCConcSignalSignalHandlerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsPendingDelaysStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr.into())) };
    let actual = unsafe { super::getOrSetGHCConcWindowsPendingDelaysStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsPendingDelaysStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetGHCConcWindowsPendingDelaysStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected =
        unsafe { transmute(sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr.into())) };
    let actual = unsafe { super::getOrSetGHCConcWindowsIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetGHCConcWindowsIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetGHCConcWindowsProddingStore(ptr.into())) };
    let actual = unsafe { super::getOrSetGHCConcWindowsProddingStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetGHCConcWindowsProddingStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetGHCConcWindowsProddingStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemEventThreadEventManagerStore(ptr: StgStablePtr) -> bool {
    let expected =
        unsafe { transmute(sys::getOrSetSystemEventThreadEventManagerStore(ptr.into())) };
    let actual = unsafe { super::getOrSetSystemEventThreadEventManagerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemEventThreadEventManagerStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetSystemEventThreadEventManagerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemEventThreadIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe {
        transmute(sys::getOrSetSystemEventThreadIOManagerThreadStore(
            ptr.into(),
        ))
    };
    let actual = unsafe { super::getOrSetSystemEventThreadIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemEventThreadIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetSystemEventThreadIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemTimerThreadEventManagerStore(ptr: StgStablePtr) -> bool {
    let expected =
        unsafe { transmute(sys::getOrSetSystemTimerThreadEventManagerStore(ptr.into())) };
    let actual = unsafe { super::getOrSetSystemTimerThreadEventManagerStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemTimerThreadEventManagerStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetSystemTimerThreadEventManagerStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetSystemTimerThreadIOManagerThreadStore(ptr: StgStablePtr) -> bool {
    let expected = unsafe {
        transmute(sys::getOrSetSystemTimerThreadIOManagerThreadStore(
            ptr.into(),
        ))
    };
    let actual = unsafe { super::getOrSetSystemTimerThreadIOManagerThreadStore(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetSystemTimerThreadIOManagerThreadStore() {
    let ptr = Default::default();
    unsafe { super::getOrSetSystemTimerThreadIOManagerThreadStore(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetLibHSghcFastStringTable(ptr.into())) };
    let actual = unsafe { super::getOrSetLibHSghcFastStringTable(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcFastStringTable() {
    let ptr = Default::default();
    unsafe { super::getOrSetLibHSghcFastStringTable(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasPprDebug(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetLibHSghcGlobalHasPprDebug(ptr.into())) };
    let actual = unsafe { super::getOrSetLibHSghcGlobalHasPprDebug(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasPprDebug() {
    let ptr = Default::default();
    unsafe { super::getOrSetLibHSghcGlobalHasPprDebug(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasNoDebugOutput(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr.into())) };
    let actual = unsafe { super::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasNoDebugOutput() {
    let ptr = Default::default();
    unsafe { super::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getOrSetLibHSghcGlobalHasNoStateHack(ptr: StgStablePtr) -> bool {
    let expected = unsafe { transmute(sys::getOrSetLibHSghcGlobalHasNoStateHack(ptr.into())) };
    let actual = unsafe { super::getOrSetLibHSghcGlobalHasNoStateHack(ptr) };
    actual == expected
}

#[test]
#[ignore]
fn test_getOrSetLibHSghcGlobalHasNoStateHack() {
    let ptr = Default::default();
    unsafe { super::getOrSetLibHSghcGlobalHasNoStateHack(ptr) };
    todo!("assert")
}
