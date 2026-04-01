use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_forkProcess() {
    let actual: pid_t = {
        let entry: HsStablePtr = todo!();
        unsafe { forkProcess(&raw mut entry) }
    };

    let expected: pid_t = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_forkProcess() {
    let expected: pid_t = {
        let mut entry: HsStablePtr = todo!();
        unsafe { sys::forkProcess(&raw mut entry) }
    };

    let actual: pid_t = {
        let mut entry: HsStablePtr = todo!();
        unsafe { forkProcess(&raw mut entry) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setNumCapabilities() {
    let g = &mut Gen::new(100);

    let actual = {
        let new_: u32 = Arbitrary::arbitrary(g);
        unsafe { setNumCapabilities(new_) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setNumCapabilities(new_: u32) -> bool {
    let expected = {
        unsafe { sys::setNumCapabilities(new_) };
        todo!()
    };

    let actual = {
        unsafe { setNumCapabilities(new_) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_suspendThread() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: StgRegTable = todo!();
        let interruptible: bool = Arbitrary::arbitrary(g);
        let result: &c_void = unsafe { &*suspendThread(&raw mut arg1, interruptible) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_suspendThread(interruptible: bool) -> bool {
    let expected = {
        let mut arg1: sys::StgRegTable = todo!();

        let result: &c_void = unsafe { &*sys::suspendThread(&raw mut arg1, interruptible) };

        todo!()
    };

    let actual = {
        let mut arg1: StgRegTable = todo!();
        let result: &c_void = unsafe { &*suspendThread(&raw mut arg1, interruptible) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_resumeThread() {
    let actual = {
        let arg1: c_void = todo!();
        let result: &StgRegTable = unsafe { &*resumeThread(&raw mut arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_resumeThread() {
    let expected = {
        let mut arg1: c_void = todo!();

        let result: &StgRegTable = unsafe { transmute(&*sys::resumeThread(&raw mut arg1)) };

        todo!()
    };

    let actual = {
        let mut arg1: c_void = todo!();
        let result: &StgRegTable = unsafe { &*resumeThread(&raw mut arg1) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performGC() {
    let actual = {
        unsafe { performGC() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performGC() {
    let expected = {
        unsafe { sys::performGC() };
        todo!()
    };

    let actual = {
        unsafe { performGC() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performMajorGC() {
    let actual = {
        unsafe { performMajorGC() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performMajorGC() {
    let expected = {
        unsafe { sys::performMajorGC() };
        todo!()
    };

    let actual = {
        unsafe { performMajorGC() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performBlockingMajorGC() {
    let actual = {
        unsafe { performBlockingMajorGC() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performBlockingMajorGC() {
    let expected = {
        unsafe { sys::performBlockingMajorGC() };
        todo!()
    };

    let actual = {
        unsafe { performBlockingMajorGC() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setAllocLimitKill() {
    let g = &mut Gen::new(100);

    let actual = {
        let arg1: bool = Arbitrary::arbitrary(g);
        let arg2: bool = Arbitrary::arbitrary(g);
        unsafe { setAllocLimitKill(arg1, arg2) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setAllocLimitKill(arg1: bool, arg2: bool) -> bool {
    let expected = {
        unsafe { sys::setAllocLimitKill(arg1, arg2) };
        todo!()
    };

    let actual = {
        unsafe { setAllocLimitKill(arg1, arg2) };
        todo!()
    };

    actual == expected
}
