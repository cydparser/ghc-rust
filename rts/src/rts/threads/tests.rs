use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_createThread() {
    todo!()
}

#[test]
#[ignore]
fn test_createThread() {
    let cap = null_mut();
    let stack_size = Default::default();
    unsafe { createThread(cap, stack_size) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_scheduleWaitThread() {
    todo!()
}

#[test]
#[ignore]
fn test_scheduleWaitThread() {
    let tso = null_mut();
    let ret = null_mut();
    let cap = null_mut();
    unsafe { scheduleWaitThread(tso, ret, cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_createGenThread() {
    todo!()
}

#[test]
#[ignore]
fn test_createGenThread() {
    let cap = null_mut();
    let stack_size = Default::default();
    let closure = null_mut();
    unsafe { createGenThread(cap, stack_size, closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_createIOThread() {
    todo!()
}

#[test]
#[ignore]
fn test_createIOThread() {
    let cap = null_mut();
    let stack_size = Default::default();
    let closure = null_mut();
    unsafe { createIOThread(cap, stack_size, closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_createStrictIOThread() {
    todo!()
}

#[test]
#[ignore]
fn test_createStrictIOThread() {
    let cap = null_mut();
    let stack_size = Default::default();
    let closure = null_mut();
    unsafe { createStrictIOThread(cap, stack_size, closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_suspendThread() {
    todo!()
}

#[test]
#[ignore]
fn test_suspendThread() {
    let arg1 = null_mut();
    let interruptible = Default::default();
    unsafe { suspendThread(arg1, interruptible) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_resumeThread() {
    todo!()
}

#[test]
#[ignore]
fn test_resumeThread() {
    let arg1 = null_mut();
    unsafe { resumeThread(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_eq_thread() {
    todo!()
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
#[test]
#[ignore]
fn equivalent_cmp_thread() {
    todo!()
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
#[test]
#[ignore]
fn equivalent_rts_getThreadId() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getThreadId() {
    let tso = Default::default();
    unsafe { rts_getThreadId(tso) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_enableThreadAllocationLimit() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_enableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { rts_enableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_disableThreadAllocationLimit() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_disableThreadAllocationLimit() {
    let tso = Default::default();
    unsafe { rts_disableThreadAllocationLimit(tso) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_listThreads() {
    todo!()
}

#[test]
#[ignore]
fn test_listThreads() {
    let cap = null_mut();
    unsafe { listThreads(cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_forkProcess() {
    todo!()
}

#[test]
#[ignore]
fn test_forkProcess() {
    let entry = null_mut();
    unsafe { forkProcess(entry) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
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

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_setNumCapabilities() {
    todo!()
}

#[test]
#[ignore]
fn test_setNumCapabilities() {
    let new_ = Default::default();
    unsafe { setNumCapabilities(new_) };
    todo!("assert")
}
