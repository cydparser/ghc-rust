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
fn equivalent_createThread(cap: Capability, stack_size: W_) -> bool {
    let expected = unsafe { transmute(sys::createThread(&mut cap.into(), stack_size)) };
    let actual = unsafe { createThread(&mut cap, stack_size) };
    actual == expected
}

#[test]
#[ignore]
fn test_createThread() {
    let mut cap = null_mut();
    let stack_size = Default::default();
    unsafe { createThread(&mut cap, stack_size) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_scheduleWaitThread() {
    let mut tso = null_mut();
    let mut ret = null_mut();
    let mut cap = null_mut();
    unsafe { scheduleWaitThread(&mut tso, &mut ret, &mut &mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createGenThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createGenThread(
            &mut cap.into(),
            stack_size,
            &mut closure.into(),
        ))
    };
    let actual = unsafe { createGenThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createGenThread() {
    let mut cap = null_mut();
    let stack_size = Default::default();
    let mut closure = null_mut();
    unsafe { createGenThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createIOThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createIOThread(
            &mut cap.into(),
            stack_size,
            &mut closure.into(),
        ))
    };
    let actual = unsafe { createIOThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createIOThread() {
    let mut cap = null_mut();
    let stack_size = Default::default();
    let mut closure = null_mut();
    unsafe { createIOThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createStrictIOThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createStrictIOThread(
            &mut cap.into(),
            stack_size,
            &mut closure.into(),
        ))
    };
    let actual = unsafe { createStrictIOThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createStrictIOThread() {
    let mut cap = null_mut();
    let stack_size = Default::default();
    let mut closure = null_mut();
    unsafe { createStrictIOThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_suspendThread(arg1: StgRegTable, interruptible: bool) -> bool {
    let expected = unsafe { sys::suspendThread(&mut arg1.into(), interruptible.into()) };
    let actual = unsafe { suspendThread(&mut arg1, interruptible) };
    actual == expected
}

#[test]
#[ignore]
fn test_suspendThread() {
    let mut arg1 = null_mut();
    let interruptible = todo!();
    unsafe { suspendThread(&mut arg1, interruptible) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_resumeThread(arg1: c_void) -> bool {
    let expected = unsafe { transmute(sys::resumeThread(&mut arg1)) };
    let actual = unsafe { resumeThread(&mut arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_resumeThread() {
    let mut arg1 = null_mut();
    unsafe { resumeThread(&mut arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::eq_thread(tso1, tso2)) };
    let actual = unsafe { eq_thread(tso1, tso2) };
    actual == expected
}

#[test]
#[ignore]
fn test_eq_thread() {
    let tso1 = Default::default();
    let tso2 = Default::default();
    unsafe { eq_thread(tso1, tso2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_cmp_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    let expected = unsafe { sys::cmp_thread(tso1, tso2) };
    let actual = unsafe { cmp_thread(tso1, tso2) };
    actual == expected
}

#[test]
#[ignore]
fn test_cmp_thread() {
    let tso1 = Default::default();
    let tso2 = Default::default();
    unsafe { cmp_thread(tso1, tso2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getThreadId(tso: StgPtr) -> bool {
    let expected = unsafe { sys::rts_getThreadId(tso) };
    let actual = unsafe { rts_getThreadId(tso) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getThreadId() {
    let tso = Default::default();
    unsafe { rts_getThreadId(tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_enableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { rts_enableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_disableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { rts_disableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_listThreads(cap: Capability) -> bool {
    let expected = unsafe { transmute(sys::listThreads(&mut cap.into())) };
    let actual = unsafe { listThreads(&mut cap) };
    actual == expected
}

#[test]
#[ignore]
fn test_listThreads() {
    let mut cap = null_mut();
    unsafe { listThreads(&mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_forkProcess(entry: HsStablePtr) -> bool {
    let expected = unsafe { transmute(sys::forkProcess(&mut entry)) };
    let actual = unsafe { forkProcess(&mut entry) };
    actual == expected
}

#[test]
#[ignore]
fn test_forkProcess() {
    let mut entry = null_mut();
    unsafe { forkProcess(&mut entry) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsSupportsBoundThreads() -> bool {
    let expected = unsafe { sys::rtsSupportsBoundThreads() };
    let actual = unsafe { rtsSupportsBoundThreads() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsSupportsBoundThreads() {
    unsafe { rtsSupportsBoundThreads() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setNumCapabilities() {
    let new_ = Default::default();
    unsafe { setNumCapabilities(new_) };
    todo!("assert")
}
