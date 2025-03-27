use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_Condition() {
    assert_eq!(size_of::<sys::Condition>(), size_of::<super::Condition>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Condition"][::core::mem::size_of::<Condition>() - 56usize];
    ["Alignment of Condition"][::core::mem::align_of::<Condition>() - 8usize];
    ["Offset of field: Condition::cond"][::core::mem::offset_of!(Condition, cond) - 0usize];
    ["Offset of field: Condition::timeout_clk"]
        [::core::mem::offset_of!(Condition, timeout_clk) - 48usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_osThreadId() -> bool {
    let expected = unsafe { transmute(sys::osThreadId()) };
    let actual = unsafe { super::osThreadId() };
    actual == expected
}

#[test]
#[ignore]
fn test_osThreadId() {
    unsafe { super::osThreadId() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownThread() -> bool {
    let expected = unsafe { transmute(sys::shutdownThread()) };
    let actual = unsafe { super::shutdownThread() };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownThread() {
    unsafe { super::shutdownThread() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_yieldThread() {
    unsafe { super::yieldThread() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createOSThread(
    tid: OSThreadId,
    name: ::core::ffi::c_char,
    startProc: OSThreadProc,
    param: ::core::ffi::c_void,
) -> bool {
    let expected = unsafe {
        transmute(sys::createOSThread(
            &mut tid.into(),
            &name.into(),
            startProc.into(),
            &mut param.into(),
        ))
    };
    let actual = unsafe { super::createOSThread(&mut tid, &name, startProc, &mut param) };
    actual == expected
}

#[test]
#[ignore]
fn test_createOSThread() {
    let tid = Default::default();
    let name = Default::default();
    let startProc = Default::default();
    let param = Default::default();
    unsafe { super::createOSThread(&mut tid, &name, startProc, &mut param) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createAttachedOSThread(
    tid: OSThreadId,
    name: ::core::ffi::c_char,
    startProc: OSThreadProc,
    param: ::core::ffi::c_void,
) -> bool {
    let expected = unsafe {
        transmute(sys::createAttachedOSThread(
            &mut tid.into(),
            &name.into(),
            startProc.into(),
            &mut param.into(),
        ))
    };
    let actual = unsafe { super::createAttachedOSThread(&mut tid, &name, startProc, &mut param) };
    actual == expected
}

#[test]
#[ignore]
fn test_createAttachedOSThread() {
    let tid = Default::default();
    let name = Default::default();
    let startProc = Default::default();
    let param = Default::default();
    unsafe { super::createAttachedOSThread(&mut tid, &name, startProc, &mut param) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_osThreadIsAlive(id: OSThreadId) -> bool {
    let expected = unsafe { transmute(sys::osThreadIsAlive(id.into())) };
    let actual = unsafe { super::osThreadIsAlive(id) };
    actual == expected
}

#[test]
#[ignore]
fn test_osThreadIsAlive() {
    let id = Default::default();
    unsafe { super::osThreadIsAlive(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_interruptOSThread() {
    let id = Default::default();
    unsafe { super::interruptOSThread(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_joinOSThread() {
    let id = Default::default();
    unsafe { super::joinOSThread(id) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initCondition() {
    let pCond = Default::default();
    unsafe { super::initCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeCondition() {
    let pCond = Default::default();
    unsafe { super::closeCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_broadcastCondition() {
    let pCond = Default::default();
    unsafe { super::broadcastCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_signalCondition() {
    let pCond = Default::default();
    unsafe { super::signalCondition(&mut pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_waitCondition() {
    let pCond = Default::default();
    let pMut = Default::default();
    unsafe { super::waitCondition(&mut pCond, &mut pMut) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_timedWaitCondition(pCond: Condition, pMut: Mutex, timeout: Time) -> bool {
    let expected = unsafe {
        transmute(sys::timedWaitCondition(
            &mut pCond.into(),
            &mut pMut.into(),
            timeout.into(),
        ))
    };
    let actual = unsafe { super::timedWaitCondition(&mut pCond, &mut pMut, timeout) };
    actual == expected
}

#[test]
#[ignore]
fn test_timedWaitCondition() {
    let pCond = Default::default();
    let pMut = Default::default();
    let timeout = Default::default();
    unsafe { super::timedWaitCondition(&mut pCond, &mut pMut, timeout) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initMutex() {
    let pMut = Default::default();
    unsafe { super::initMutex(&mut pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeMutex() {
    let pMut = Default::default();
    unsafe { super::closeMutex(&mut pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_newThreadLocalKey() {
    let key = Default::default();
    unsafe { super::newThreadLocalKey(&mut key) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getThreadLocalVar(key: ThreadLocalKey) -> bool {
    let expected = unsafe { transmute(sys::getThreadLocalVar(&mut key.into())) };
    let actual = unsafe { super::getThreadLocalVar(&mut key) };
    actual == expected
}

#[test]
#[ignore]
fn test_getThreadLocalVar() {
    let key = Default::default();
    unsafe { super::getThreadLocalVar(&mut key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadLocalVar() {
    let key = Default::default();
    let value = Default::default();
    unsafe { super::setThreadLocalVar(&mut key, &mut value) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeThreadLocalKey() {
    let key = Default::default();
    unsafe { super::freeThreadLocalKey(&mut key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadAffinity() {
    let n = Default::default();
    let m = Default::default();
    unsafe { super::setThreadAffinity(n, m) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadNode() {
    let node = Default::default();
    unsafe { super::setThreadNode(node) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_releaseThreadNode() {
    unsafe { super::releaseThreadNode() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_forkOS_createThread(entry: HsStablePtr) -> bool {
    let expected = unsafe { transmute(sys::forkOS_createThread(entry.into())) };
    let actual = unsafe { super::forkOS_createThread(entry) };
    actual == expected
}

#[test]
#[ignore]
fn test_forkOS_createThread() {
    let entry = Default::default();
    unsafe { super::forkOS_createThread(entry) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeThreadingResources() {
    unsafe { super::freeThreadingResources() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getNumberOfProcessors() -> bool {
    let expected = unsafe { transmute(sys::getNumberOfProcessors()) };
    let actual = unsafe { super::getNumberOfProcessors() };
    actual == expected
}

#[test]
#[ignore]
fn test_getNumberOfProcessors() {
    unsafe { super::getNumberOfProcessors() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_kernelThreadId() -> bool {
    let expected = unsafe { transmute(sys::kernelThreadId()) };
    let actual = unsafe { super::kernelThreadId() };
    actual == expected
}

#[test]
#[ignore]
fn test_kernelThreadId() {
    unsafe { super::kernelThreadId() };
    todo!("assert")
}
