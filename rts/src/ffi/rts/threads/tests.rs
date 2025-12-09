use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_createThread(stack_size: W_) -> bool {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let result: &StgTSO = unsafe { transmute(&*sys::createThread(&raw mut cap, stack_size)) };
        todo!()
    };
    let actual = {
        let mut cap: Capability = todo!();
        let result: &StgTSO = unsafe { &*createThread(&raw mut cap, stack_size) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_createThread() {
    let g = &mut Gen::new(100);
    let actual = {
        let cap: Capability = todo!();
        let stack_size: W_ = Arbitrary::arbitrary(g);
        let result: &StgTSO = unsafe { &*createThread(&raw mut cap, stack_size) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_createGenThread(stack_size: W_) -> bool {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let mut closure: sys::StgClosure = todo!();
        let result: &StgTSO = unsafe {
            transmute(&*sys::createGenThread(
                &raw mut cap,
                stack_size,
                &raw mut closure,
            ))
        };
        todo!()
    };
    let actual = {
        let mut cap: Capability = todo!();
        let mut closure: StgClosure = todo!();
        let result: &StgTSO =
            unsafe { &*createGenThread(&raw mut cap, stack_size, &raw mut closure) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_createGenThread() {
    let g = &mut Gen::new(100);
    let actual = {
        let cap: Capability = todo!();
        let stack_size: W_ = Arbitrary::arbitrary(g);
        let closure: StgClosure = todo!();
        let result: &StgTSO =
            unsafe { &*createGenThread(&raw mut cap, stack_size, &raw mut closure) };
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
fn equivalent_eq_thread() {
    let expected: bool = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { sys::eq_thread(tso1, tso2) }
    };
    let actual: bool = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { eq_thread(tso1, tso2) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_eq_thread() {
    let actual: bool = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { eq_thread(tso1, tso2) }
    };
    let expected: bool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_cmp_thread() {
    let expected: c_int = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { sys::cmp_thread(tso1, tso2) }
    };
    let actual: c_int = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { cmp_thread(tso1, tso2) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_cmp_thread() {
    let actual: c_int = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { cmp_thread(tso1, tso2) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getThreadId() {
    let expected: StgThreadID = {
        let tso: StgPtr = todo!();
        unsafe { sys::rts_getThreadId(tso) }
    };
    let actual: StgThreadID = {
        let tso: StgPtr = todo!();
        unsafe { rts_getThreadId(tso) }
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getThreadId() {
    let actual: StgThreadID = {
        let tso: StgPtr = todo!();
        unsafe { rts_getThreadId(tso) }
    };
    let expected: StgThreadID = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_enableThreadAllocationLimit() {
    let expected = {
        let tso: StgPtr = todo!();
        unsafe { sys::rts_enableThreadAllocationLimit(tso) };
        todo!()
    };
    let actual = {
        let tso: StgPtr = todo!();
        unsafe { rts_enableThreadAllocationLimit(tso) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_enableThreadAllocationLimit() {
    let actual = {
        let tso: StgPtr = todo!();
        unsafe { rts_enableThreadAllocationLimit(tso) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_disableThreadAllocationLimit() {
    let expected = {
        let tso: StgPtr = todo!();
        unsafe { sys::rts_disableThreadAllocationLimit(tso) };
        todo!()
    };
    let actual = {
        let tso: StgPtr = todo!();
        unsafe { rts_disableThreadAllocationLimit(tso) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_disableThreadAllocationLimit() {
    let actual = {
        let tso: StgPtr = todo!();
        unsafe { rts_disableThreadAllocationLimit(tso) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_listThreads() {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let result: &_StgMutArrPtrs = unsafe { transmute(&*sys::listThreads(&raw mut cap)) };
        todo!()
    };
    let actual = {
        let mut cap: Capability = todo!();
        let result: &_StgMutArrPtrs = unsafe { &*listThreads(&raw mut cap) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_listThreads() {
    let actual = {
        let cap: Capability = todo!();
        let result: &_StgMutArrPtrs = unsafe { &*listThreads(&raw mut cap) };
        todo!()
    };
    let expected = todo!();
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
fn equivalent_rtsSupportsBoundThreads() {
    let expected: HsBool = { unsafe { sys::rtsSupportsBoundThreads() } };
    let actual: HsBool = { unsafe { rtsSupportsBoundThreads() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rtsSupportsBoundThreads() {
    let actual: HsBool = { unsafe { rtsSupportsBoundThreads() } };
    let expected: HsBool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_n_capabilities_layout() {
    assert_eq!(
        size_of_val(unsafe { &n_capabilities }),
        size_of_val(unsafe { &sys::n_capabilities })
    );
    assert_eq!(
        align_of_val(unsafe { &n_capabilities }),
        align_of_val(unsafe { &sys::n_capabilities })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_enabled_capabilities_layout() {
    assert_eq!(
        size_of_val(unsafe { &enabled_capabilities }),
        size_of_val(unsafe { &sys::enabled_capabilities })
    );
    assert_eq!(
        align_of_val(unsafe { &enabled_capabilities }),
        align_of_val(unsafe { &sys::enabled_capabilities })
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn sys_MainCapability_layout() {
    // TODO(rust): MainCapability
    // assert_eq!(
    //     size_of_val(unsafe { &MainCapability }),
    //     size_of_val(unsafe { &sys::MainCapability })
    // );
    // assert_eq!(
    //     align_of_val(unsafe { &MainCapability }),
    //     align_of_val(unsafe { &sys::MainCapability })
    // );
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
