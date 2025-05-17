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
#[test]
fn sys_size_Condition() {
    assert_eq!(size_of::<sys::Condition>(), size_of::<Condition>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Condition"][size_of::<Condition>() - 56usize];
    ["Alignment of Condition"][align_of::<Condition>() - 8usize];
    ["Offset of field: Condition::cond"][offset_of!(Condition, cond) - 0usize];
    ["Offset of field: Condition::timeout_clk"][offset_of!(Condition, timeout_clk) - 48usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_osThreadId() -> bool {
    let expected = unsafe { sys::osThreadId() };
    let actual = unsafe { osThreadId() };
    actual == expected
}

#[test]
#[ignore]
fn test_osThreadId() {
    unsafe { osThreadId() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownThread() -> bool {
    let expected = unsafe { sys::shutdownThread() };
    let actual = unsafe { shutdownThread() };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownThread() {
    unsafe { shutdownThread() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_yieldThread() {
    unsafe { yieldThread() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createOSThread(
    tid: OSThreadId,
    name: c_char,
    startProc: OSThreadProc,
    param: c_void,
) -> bool {
    let expected = unsafe { sys::createOSThread(&mut tid, &name, startProc, &mut param) };
    let actual = unsafe { createOSThread(&mut tid, &name, startProc, &mut param) };
    actual == expected
}

#[test]
#[ignore]
fn test_createOSThread() {
    let mut tid = null_mut();
    let name = null();
    let startProc = Default::default();
    let mut param = null_mut();
    unsafe { createOSThread(&mut tid, &name, startProc, &mut param) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createAttachedOSThread(
    tid: OSThreadId,
    name: c_char,
    startProc: OSThreadProc,
    param: c_void,
) -> bool {
    let expected = unsafe { sys::createAttachedOSThread(&mut tid, &name, startProc, &mut param) };
    let actual = unsafe { createAttachedOSThread(&mut tid, &name, startProc, &mut param) };
    actual == expected
}

#[test]
#[ignore]
fn test_createAttachedOSThread() {
    let mut tid = null_mut();
    let name = null();
    let startProc = Default::default();
    let mut param = null_mut();
    unsafe { createAttachedOSThread(&mut tid, &name, startProc, &mut param) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_osThreadIsAlive(id: OSThreadId) -> bool {
    let expected = unsafe { transmute(sys::osThreadIsAlive(id)) };
    let actual = unsafe { osThreadIsAlive(id) };
    actual == expected
}

#[test]
#[ignore]
fn test_osThreadIsAlive() {
    let id = Default::default();
    unsafe { osThreadIsAlive(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_interruptOSThread() {
    let id = Default::default();
    unsafe { interruptOSThread(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_joinOSThread() {
    let id = Default::default();
    unsafe { joinOSThread(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initCondition() {
    let mut pCond = null_mut();
    unsafe { initCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeCondition() {
    let mut pCond = null_mut();
    unsafe { closeCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_broadcastCondition() {
    let mut pCond = null_mut();
    unsafe { broadcastCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_signalCondition() {
    let mut pCond = null_mut();
    unsafe { signalCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_waitCondition() {
    let mut pCond = null_mut();
    let mut pMut = null_mut();
    unsafe { waitCondition(&mut pCond, &mut pMut) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_timedWaitCondition(pCond: Condition, pMut: Mutex, timeout: Time) -> bool {
    let expected = unsafe {
        transmute(sys::timedWaitCondition(
            &mut pCond.into(),
            &mut pMut,
            timeout,
        ))
    };
    let actual = unsafe { timedWaitCondition(&mut pCond, &mut pMut, timeout) };
    actual == expected
}

#[test]
#[ignore]
fn test_timedWaitCondition() {
    let mut pCond = null_mut();
    let mut pMut = null_mut();
    let timeout = Default::default();
    unsafe { timedWaitCondition(&mut pCond, &mut pMut, timeout) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initMutex() {
    let mut pMut = null_mut();
    unsafe { initMutex(&mut pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeMutex() {
    let mut pMut = null_mut();
    unsafe { closeMutex(&mut pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_newThreadLocalKey() {
    let mut key = null_mut();
    unsafe { newThreadLocalKey(&mut key) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getThreadLocalVar(key: ThreadLocalKey) -> bool {
    let expected = unsafe { sys::getThreadLocalVar(&mut key) };
    let actual = unsafe { getThreadLocalVar(&mut key) };
    actual == expected
}

#[test]
#[ignore]
fn test_getThreadLocalVar() {
    let mut key = null_mut();
    unsafe { getThreadLocalVar(&mut key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadLocalVar() {
    let mut key = null_mut();
    let mut value = null_mut();
    unsafe { setThreadLocalVar(&mut key, &mut value) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeThreadLocalKey() {
    let mut key = null_mut();
    unsafe { freeThreadLocalKey(&mut key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadAffinity() {
    let n = Default::default();
    let m = Default::default();
    unsafe { setThreadAffinity(n, m) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadNode() {
    let node = Default::default();
    unsafe { setThreadNode(node) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_releaseThreadNode() {
    unsafe { releaseThreadNode() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_forkOS_createThread(entry: HsStablePtr) -> bool {
    let expected = unsafe { sys::forkOS_createThread(entry) };
    let actual = unsafe { forkOS_createThread(entry) };
    actual == expected
}

#[test]
#[ignore]
fn test_forkOS_createThread() {
    let entry = Default::default();
    unsafe { forkOS_createThread(entry) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeThreadingResources() {
    unsafe { freeThreadingResources() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getNumberOfProcessors() -> bool {
    let expected = unsafe { sys::getNumberOfProcessors() };
    let actual = unsafe { getNumberOfProcessors() };
    actual == expected
}

#[test]
#[ignore]
fn test_getNumberOfProcessors() {
    unsafe { getNumberOfProcessors() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_kernelThreadId() -> bool {
    let expected = unsafe { sys::kernelThreadId() };
    let actual = unsafe { kernelThreadId() };
    actual == expected
}

#[test]
#[ignore]
fn test_kernelThreadId() {
    unsafe { kernelThreadId() };
    todo!("assert")
}
