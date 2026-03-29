use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_reportStackOverflow() {
    let actual = {
        let tso: StgTSO = todo!();
        unsafe { reportStackOverflow(&raw mut tso) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_reportStackOverflow() {
    let expected = {
        let mut tso: sys::StgTSO = todo!();
        unsafe { sys::reportStackOverflow(&raw mut tso) };
        todo!()
    };

    let actual = {
        let mut tso: StgTSO = todo!();
        unsafe { reportStackOverflow(&raw mut tso) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_reportHeapOverflow() {
    let actual = {
        unsafe { reportHeapOverflow() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_reportHeapOverflow() {
    let expected = {
        unsafe { sys::reportHeapOverflow() };
        todo!()
    };

    let actual = {
        unsafe { reportHeapOverflow() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_genericRaise() {
    let g = &mut Gen::new(100);

    let actual: i32 = {
        let sig: i32 = Arbitrary::arbitrary(g);
        unsafe { genericRaise(sig) }
    };

    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_genericRaise(sig: i32) -> bool {
    let expected: i32 = { unsafe { sys::genericRaise(sig) } };
    let actual: i32 = { unsafe { genericRaise(sig) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isProfiled() {
    let actual: i32 = { unsafe { rts_isProfiled() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isProfiled() {
    let expected: i32 = { unsafe { sys::rts_isProfiled() } };
    let actual: i32 = { unsafe { rts_isProfiled() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isDynamic() {
    let actual: i32 = { unsafe { rts_isDynamic() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isDynamic() {
    let expected: i32 = { unsafe { sys::rts_isDynamic() } };
    let actual: i32 = { unsafe { rts_isDynamic() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isThreaded() {
    let actual: i32 = { unsafe { rts_isThreaded() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isThreaded() {
    let expected: i32 = { unsafe { sys::rts_isThreaded() } };
    let actual: i32 = { unsafe { rts_isThreaded() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isDebugged() {
    let actual: i32 = { unsafe { rts_isDebugged() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isDebugged() {
    let expected: i32 = { unsafe { sys::rts_isDebugged() } };
    let actual: i32 = { unsafe { rts_isDebugged() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isTracing() {
    let actual: i32 = { unsafe { rts_isTracing() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isTracing() {
    let expected: i32 = { unsafe { sys::rts_isTracing() } };
    let actual: i32 = { unsafe { rts_isTracing() } };
    assert_eq!(actual, expected);
}
