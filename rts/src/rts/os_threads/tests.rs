use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_Condition() {
    assert_eq!(size_of::<sys::Condition>(), size_of::<Condition>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Condition"][size_of::<Condition>() - 48usize];
    ["Alignment of Condition"][align_of::<Condition>() - 8usize];
    ["Offset of field: Condition::cond"][offset_of!(Condition, cond) - 0usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
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

#[test]
#[ignore]
fn test_yieldThread() {
    unsafe { yieldThread() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_createOSThread() {
    let tid = null_mut();
    let name = null();
    let startProc = Default::default();
    let param = null_mut();
    unsafe { createOSThread(tid, name, startProc, param) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_createAttachedOSThread() {
    let tid = null_mut();
    let name = null();
    let startProc = Default::default();
    let param = null_mut();
    unsafe { createAttachedOSThread(tid, name, startProc, param) };
    todo!("assert")
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
    let pCond = null_mut();
    unsafe { initCondition(pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeCondition() {
    let pCond = null_mut();
    unsafe { closeCondition(pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_broadcastCondition() {
    let pCond = null_mut();
    unsafe { broadcastCondition(pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_signalCondition() {
    let pCond = null_mut();
    unsafe { signalCondition(pCond) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_waitCondition() {
    let pCond = null_mut();
    let pMut = null_mut();
    unsafe { waitCondition(pCond, pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_timedWaitCondition() {
    let pCond = null_mut();
    let pMut = null_mut();
    let timeout = Default::default();
    unsafe { timedWaitCondition(pCond, pMut, timeout) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initMutex() {
    let pMut = null_mut();
    unsafe { initMutex(pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_closeMutex() {
    let pMut = null_mut();
    unsafe { closeMutex(pMut) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_newThreadLocalKey() {
    let key = null_mut();
    unsafe { newThreadLocalKey(key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getThreadLocalVar() {
    let key = null_mut();
    unsafe { getThreadLocalVar(key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setThreadLocalVar() {
    let key = null_mut();
    let value = null_mut();
    unsafe { setThreadLocalVar(key, value) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeThreadLocalKey() {
    let key = null_mut();
    unsafe { freeThreadLocalKey(key) };
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
#[ignore]
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
#[ignore]
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
