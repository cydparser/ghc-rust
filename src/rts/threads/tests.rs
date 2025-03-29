use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createThread(cap: Capability, stack_size: W_) -> bool {
    let expected = unsafe { transmute(sys::createThread(&mut cap.into(), stack_size.into())) };
    let actual = unsafe { super::createThread(&mut cap, stack_size) };
    actual == expected
}

#[test]
#[ignore]
fn test_createThread() {
    let mut cap = Default::default();
    let stack_size = Default::default();
    unsafe { super::createThread(&mut cap, stack_size) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_scheduleWaitThread() {
    let mut tso = Default::default();
    let mut ret = Default::default();
    let mut cap = Default::default();
    unsafe { super::scheduleWaitThread(&mut tso, &mut ret, &mut &mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createGenThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createGenThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    };
    let actual = unsafe { super::createGenThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createGenThread() {
    let mut cap = Default::default();
    let stack_size = Default::default();
    let mut closure = Default::default();
    unsafe { super::createGenThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createIOThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createIOThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    };
    let actual = unsafe { super::createIOThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createIOThread() {
    let mut cap = Default::default();
    let stack_size = Default::default();
    let mut closure = Default::default();
    unsafe { super::createIOThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createStrictIOThread(cap: Capability, stack_size: W_, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::createStrictIOThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    };
    let actual = unsafe { super::createStrictIOThread(&mut cap, stack_size, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_createStrictIOThread() {
    let mut cap = Default::default();
    let stack_size = Default::default();
    let mut closure = Default::default();
    unsafe { super::createStrictIOThread(&mut cap, stack_size, &mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_suspendThread(arg1: StgRegTable, interruptible: bool) -> bool {
    let expected = unsafe { transmute(sys::suspendThread(&mut arg1.into(), interruptible.into())) };
    let actual = unsafe { super::suspendThread(&mut arg1, interruptible) };
    actual == expected
}

#[test]
#[ignore]
fn test_suspendThread() {
    let mut arg1 = Default::default();
    let interruptible = Default::default();
    unsafe { super::suspendThread(&mut arg1, interruptible) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_resumeThread(arg1: ::core::ffi::c_void) -> bool {
    let expected = unsafe { transmute(sys::resumeThread(&mut arg1.into())) };
    let actual = unsafe { super::resumeThread(&mut arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_resumeThread() {
    let mut arg1 = Default::default();
    unsafe { super::resumeThread(&mut arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::eq_thread(tso1.into(), tso2.into())) };
    let actual = unsafe { super::eq_thread(tso1, tso2) };
    actual == expected
}

#[test]
#[ignore]
fn test_eq_thread() {
    let tso1 = Default::default();
    let tso2 = Default::default();
    unsafe { super::eq_thread(tso1, tso2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_cmp_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::cmp_thread(tso1.into(), tso2.into())) };
    let actual = unsafe { super::cmp_thread(tso1, tso2) };
    actual == expected
}

#[test]
#[ignore]
fn test_cmp_thread() {
    let tso1 = Default::default();
    let tso2 = Default::default();
    unsafe { super::cmp_thread(tso1, tso2) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getThreadId(tso: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::rts_getThreadId(tso.into())) };
    let actual = unsafe { super::rts_getThreadId(tso) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getThreadId() {
    let tso = Default::default();
    unsafe { super::rts_getThreadId(tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_enableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { super::rts_enableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_disableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { super::rts_disableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_listThreads(cap: Capability) -> bool {
    let expected = unsafe { transmute(sys::listThreads(&mut cap.into())) };
    let actual = unsafe { super::listThreads(&mut cap) };
    actual == expected
}

#[test]
#[ignore]
fn test_listThreads() {
    let mut cap = Default::default();
    unsafe { super::listThreads(&mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_forkProcess(entry: HsStablePtr) -> bool {
    let expected = unsafe { transmute(sys::forkProcess(&mut entry.into())) };
    let actual = unsafe { super::forkProcess(&mut entry) };
    actual == expected
}

#[test]
#[ignore]
fn test_forkProcess() {
    let mut entry = Default::default();
    unsafe { super::forkProcess(&mut entry) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rtsSupportsBoundThreads() -> bool {
    let expected = unsafe { transmute(sys::rtsSupportsBoundThreads()) };
    let actual = unsafe { super::rtsSupportsBoundThreads() };
    actual == expected
}

#[test]
#[ignore]
fn test_rtsSupportsBoundThreads() {
    unsafe { super::rtsSupportsBoundThreads() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setNumCapabilities() {
    let new_ = Default::default();
    unsafe { super::setNumCapabilities(new_) };
    todo!("assert")
}
