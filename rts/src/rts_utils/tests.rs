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

    let actual: c_int = {
        let sig: c_int = Arbitrary::arbitrary(g);
        unsafe { genericRaise(sig) }
    };

    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_genericRaise(sig: c_int) -> bool {
    let expected: c_int = { unsafe { sys::genericRaise(sig) } };
    let actual: c_int = { unsafe { genericRaise(sig) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isProfiled() {
    let actual: c_int = { unsafe { rts_isProfiled() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isProfiled() {
    let expected: c_int = { unsafe { sys::rts_isProfiled() } };
    let actual: c_int = { unsafe { rts_isProfiled() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isDynamic() {
    let actual: c_int = { unsafe { rts_isDynamic() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isDynamic() {
    let expected: c_int = { unsafe { sys::rts_isDynamic() } };
    let actual: c_int = { unsafe { rts_isDynamic() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isThreaded() {
    let actual: c_int = { unsafe { rts_isThreaded() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isThreaded() {
    let expected: c_int = { unsafe { sys::rts_isThreaded() } };
    let actual: c_int = { unsafe { rts_isThreaded() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isDebugged() {
    let actual: c_int = { unsafe { rts_isDebugged() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isDebugged() {
    let expected: c_int = { unsafe { sys::rts_isDebugged() } };
    let actual: c_int = { unsafe { rts_isDebugged() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isTracing() {
    let actual: c_int = { unsafe { rts_isTracing() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_isTracing() {
    let expected: c_int = { unsafe { sys::rts_isTracing() } };
    let actual: c_int = { unsafe { rts_isTracing() } };
    assert_eq!(actual, expected);
}
