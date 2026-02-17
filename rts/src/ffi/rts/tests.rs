use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_IN_STG_CODE_eq() {
    assert_eq!(IN_STG_CODE, sys::IN_STG_CODE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_IN_STG_CODE_layout() {
    assert_eq!(size_of_val(&IN_STG_CODE), size_of_val(&sys::IN_STG_CODE));
    assert_eq!(align_of_val(&IN_STG_CODE), align_of_val(&sys::IN_STG_CODE));
}

#[cfg(feature = "sys")]
#[test]
fn sys__REENTRANT_eq() {
    assert_eq!(_REENTRANT, sys::_REENTRANT);
}

#[cfg(feature = "sys")]
#[test]
fn sys__REENTRANT_layout() {
    assert_eq!(size_of_val(&_REENTRANT), size_of_val(&sys::_REENTRANT));
    assert_eq!(align_of_val(&_REENTRANT), align_of_val(&sys::_REENTRANT));
}

#[cfg(feature = "sys")]
#[test]
fn sys_EXIT_INTERNAL_ERROR_eq() {
    assert_eq!(EXIT_INTERNAL_ERROR, sys::EXIT_INTERNAL_ERROR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_EXIT_INTERNAL_ERROR_layout() {
    assert_eq!(
        size_of_val(&EXIT_INTERNAL_ERROR),
        size_of_val(&sys::EXIT_INTERNAL_ERROR)
    );
    assert_eq!(
        align_of_val(&EXIT_INTERNAL_ERROR),
        align_of_val(&sys::EXIT_INTERNAL_ERROR)
    );
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
fn test_reportHeapOverflow() {
    let actual = {
        unsafe { reportHeapOverflow() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_sig_install(arg1: c_int, arg2: c_int) -> bool {
    let expected: c_int = {
        let mut arg3: c_void = todo!();
        unsafe { sys::stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

    let actual: c_int = {
        let mut arg3: c_void = todo!();
        unsafe { stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_sig_install() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let arg1: c_int = Arbitrary::arbitrary(g);
        let arg2: c_int = Arbitrary::arbitrary(g);
        let arg3: c_void = todo!();
        unsafe { stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

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
fn test_rts_isProfiled() {
    let actual: c_int = { unsafe { rts_isProfiled() } };
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
fn test_rts_isDynamic() {
    let actual: c_int = { unsafe { rts_isDynamic() } };
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
fn test_rts_isThreaded() {
    let actual: c_int = { unsafe { rts_isThreaded() } };
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
fn test_rts_isDebugged() {
    let actual: c_int = { unsafe { rts_isDebugged() } };
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

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isTracing() {
    let actual: c_int = { unsafe { rts_isTracing() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}
