use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_CHAR_MIN() {
    assert_eq!(sys::HS_CHAR_MIN, HS_CHAR_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_CHAR_MAX() {
    assert_eq!(sys::HS_CHAR_MAX, HS_CHAR_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_BOOL_FALSE() {
    assert_eq!(sys::HS_BOOL_FALSE, HS_BOOL_FALSE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_BOOL_TRUE() {
    assert_eq!(sys::HS_BOOL_TRUE, HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_BOOL_MIN() {
    assert_eq!(sys::HS_BOOL_MIN, HS_BOOL_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_BOOL_MAX() {
    assert_eq!(sys::HS_BOOL_MAX, HS_BOOL_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT_MIN() {
    assert_eq!(sys::HS_INT_MIN, HS_INT_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT_MAX() {
    assert_eq!(sys::HS_INT_MAX, HS_INT_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_WORD_MAX() {
    assert_eq!(sys::HS_WORD_MAX, HS_WORD_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT8_MIN() {
    assert_eq!(sys::HS_INT8_MIN, HS_INT8_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT8_MAX() {
    assert_eq!(sys::HS_INT8_MAX, HS_INT8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT16_MIN() {
    assert_eq!(sys::HS_INT16_MIN, HS_INT16_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT16_MAX() {
    assert_eq!(sys::HS_INT16_MAX, HS_INT16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT32_MIN() {
    assert_eq!(sys::HS_INT32_MIN, HS_INT32_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT32_MAX() {
    assert_eq!(sys::HS_INT32_MAX, HS_INT32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT64_MIN() {
    assert_eq!(sys::HS_INT64_MIN, HS_INT64_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_INT64_MAX() {
    assert_eq!(sys::HS_INT64_MAX, HS_INT64_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_WORD8_MAX() {
    assert_eq!(sys::HS_WORD8_MAX, HS_WORD8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_WORD16_MAX() {
    assert_eq!(sys::HS_WORD16_MAX, HS_WORD16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_WORD32_MAX() {
    assert_eq!(sys::HS_WORD32_MAX, HS_WORD32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HS_WORD64_MAX() {
    assert_eq!(sys::HS_WORD64_MAX, HS_WORD64_MAX);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_exit() {
    let expected = {
        unsafe { sys::hs_exit() };
        todo!()
    };
    let actual = {
        unsafe { hs_exit() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_exit() {
    let actual = {
        unsafe { hs_exit() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_thread_done() {
    let expected = {
        unsafe { sys::hs_thread_done() };
        todo!()
    };
    let actual = {
        unsafe { hs_thread_done() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_thread_done() {
    let actual = {
        unsafe { hs_thread_done() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_perform_gc() {
    let expected = {
        unsafe { sys::hs_perform_gc() };
        todo!()
    };
    let actual = {
        unsafe { hs_perform_gc() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_perform_gc() {
    let actual = {
        unsafe { hs_perform_gc() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_free_stable_ptr() {
    let expected = {
        let sp: HsStablePtr = todo!();
        unsafe { sys::hs_free_stable_ptr(sp) };
        todo!()
    };
    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr(sp) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_free_stable_ptr() {
    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_free_stable_ptr(sp) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_lookup(key: StgWord64) -> bool {
    let expected = {
        let mut key = key;
        let result: StgPtr = unsafe { sys::hs_spt_lookup(&raw mut key) };
        todo!()
    };
    let actual = {
        let mut key = key;
        let result: StgPtr = unsafe { hs_spt_lookup(&raw mut key) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_lookup() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut key: StgWord64 = Arbitrary::arbitrary(g);
        let result: StgPtr = unsafe { hs_spt_lookup(&raw mut key) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_keys(szKeys: c_int) -> bool {
    let expected: c_int = {
        let mut keys: StgPtr = todo!();
        unsafe { sys::hs_spt_keys(&raw mut keys, szKeys) }
    };
    let actual: c_int = {
        let mut keys: StgPtr = todo!();
        unsafe { hs_spt_keys(&raw mut keys, szKeys) }
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_keys() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let keys: StgPtr = todo!();
        let szKeys: c_int = Arbitrary::arbitrary(g);
        unsafe { hs_spt_keys(&raw mut keys, szKeys) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_hs_spt_key_count() {
    let expected: c_int = { unsafe { sys::hs_spt_key_count() } };
    let actual: c_int = { unsafe { hs_spt_key_count() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_key_count() {
    let actual: c_int = { unsafe { hs_spt_key_count() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_try_putmvar(capability: c_int) -> bool {
    let expected = {
        let sp: HsStablePtr = todo!();
        unsafe { sys::hs_try_putmvar(capability, sp) };
        todo!()
    };
    let actual = {
        let sp: HsStablePtr = todo!();
        unsafe { hs_try_putmvar(capability, sp) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_try_putmvar() {
    let g = &mut Gen::new(100);
    let actual = {
        let capability: c_int = Arbitrary::arbitrary(g);
        let sp: HsStablePtr = todo!();
        unsafe { hs_try_putmvar(capability, sp) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
