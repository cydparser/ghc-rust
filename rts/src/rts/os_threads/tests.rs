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

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_createOSThread(name: c_char) -> bool {
    let expected: c_int = {
        let mut tid: sys::OSThreadId = todo!();
        let mut name = name;
        let startProc: OSThreadProc = todo!();
        let mut param: c_void = todo!();
        unsafe { sys::createOSThread(&raw mut tid, &raw mut name, startProc, &raw mut param) }
    };
    let actual: c_int = {
        let mut tid: OSThreadId = todo!();
        let mut name = name;
        let startProc: OSThreadProc = todo!();
        let mut param: c_void = todo!();
        unsafe { createOSThread(&raw mut tid, &raw mut name, startProc, &raw mut param) }
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_createOSThread() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let tid: OSThreadId = todo!();
        let mut name: c_char = Arbitrary::arbitrary(g);
        let startProc: OSThreadProc = todo!();
        let param: c_void = todo!();
        unsafe { createOSThread(&raw mut tid, &raw mut name, startProc, &raw mut param) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_initCondition() {
    let expected = {
        let mut pCond: sys::Condition = todo!();
        unsafe { sys::initCondition(&raw mut pCond) };
        todo!()
    };
    let actual = {
        let mut pCond: Condition = todo!();
        unsafe { initCondition(&raw mut pCond) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_initCondition() {
    let actual = {
        let pCond: Condition = todo!();
        unsafe { initCondition(&raw mut pCond) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_broadcastCondition() {
    let expected = {
        let mut pCond: sys::Condition = todo!();
        unsafe { sys::broadcastCondition(&raw mut pCond) };
        todo!()
    };
    let actual = {
        let mut pCond: Condition = todo!();
        unsafe { broadcastCondition(&raw mut pCond) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_broadcastCondition() {
    let actual = {
        let pCond: Condition = todo!();
        unsafe { broadcastCondition(&raw mut pCond) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_waitCondition() {
    let expected = {
        let mut pCond: sys::Condition = todo!();
        let mut pMut: sys::Mutex = todo!();
        unsafe { sys::waitCondition(&raw mut pCond, &raw mut pMut) };
        todo!()
    };
    let actual = {
        let mut pCond: Condition = todo!();
        let mut pMut: Mutex = todo!();
        unsafe { waitCondition(&raw mut pCond, &raw mut pMut) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_waitCondition() {
    let actual = {
        let pCond: Condition = todo!();
        let pMut: Mutex = todo!();
        unsafe { waitCondition(&raw mut pCond, &raw mut pMut) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_initMutex() {
    let expected = {
        let mut pMut: sys::Mutex = todo!();
        unsafe { sys::initMutex(&raw mut pMut) };
        todo!()
    };
    let actual = {
        let mut pMut: Mutex = todo!();
        unsafe { initMutex(&raw mut pMut) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_initMutex() {
    let actual = {
        let pMut: Mutex = todo!();
        unsafe { initMutex(&raw mut pMut) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_forkOS_createThread() {
    let expected: c_int = {
        let entry: HsStablePtr = todo!();
        unsafe { sys::forkOS_createThread(entry) }
    };
    let actual: c_int = {
        let entry: HsStablePtr = todo!();
        unsafe { forkOS_createThread(entry) }
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_forkOS_createThread() {
    let actual: c_int = {
        let entry: HsStablePtr = todo!();
        unsafe { forkOS_createThread(entry) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getNumberOfProcessors() {
    let expected: u32 = { unsafe { sys::getNumberOfProcessors() } };
    let actual: u32 = { unsafe { getNumberOfProcessors() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getNumberOfProcessors() {
    let actual: u32 = { unsafe { getNumberOfProcessors() } };
    let expected: u32 = todo!();
    assert_eq!(expected, actual);
}
