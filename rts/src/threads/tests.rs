use super::*;

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
fn test_cmp_thread() {
    let actual: i32 = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { cmp_thread(tso1, tso2) }
    };

    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_cmp_thread() {
    let expected: i32 = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { sys::cmp_thread(tso1, tso2) }
    };

    let actual: i32 = {
        let tso1: StgPtr = todo!();
        let tso2: StgPtr = todo!();
        unsafe { cmp_thread(tso1, tso2) }
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
fn test_rtsSupportsBoundThreads() {
    let actual: HsBool = { unsafe { rtsSupportsBoundThreads() } };
    let expected: HsBool = todo!();
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
