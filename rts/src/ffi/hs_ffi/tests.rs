use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_eq() {
    assert_eq!(HS_BOOL_TRUE, sys::HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_layout() {
    assert_eq!(size_of_val(&HS_BOOL_TRUE), size_of_val(&sys::HS_BOOL_TRUE));
    assert_eq!(
        align_of_val(&HS_BOOL_TRUE),
        align_of_val(&sys::HS_BOOL_TRUE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsChar_layout() {
    assert_eq!(size_of::<HsChar>(), size_of::<HsChar>());
    assert_eq!(align_of::<HsChar>(), align_of::<HsChar>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt_layout() {
    assert_eq!(size_of::<HsInt>(), size_of::<HsInt>());
    assert_eq!(align_of::<HsInt>(), align_of::<HsInt>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt8_layout() {
    assert_eq!(size_of::<HsInt8>(), size_of::<HsInt8>());
    assert_eq!(align_of::<HsInt8>(), align_of::<HsInt8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt16_layout() {
    assert_eq!(size_of::<HsInt16>(), size_of::<HsInt16>());
    assert_eq!(align_of::<HsInt16>(), align_of::<HsInt16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt32_layout() {
    assert_eq!(size_of::<HsInt32>(), size_of::<HsInt32>());
    assert_eq!(align_of::<HsInt32>(), align_of::<HsInt32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt64_layout() {
    assert_eq!(size_of::<HsInt64>(), size_of::<HsInt64>());
    assert_eq!(align_of::<HsInt64>(), align_of::<HsInt64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord_layout() {
    assert_eq!(size_of::<HsWord>(), size_of::<HsWord>());
    assert_eq!(align_of::<HsWord>(), align_of::<HsWord>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord8_layout() {
    assert_eq!(size_of::<HsWord8>(), size_of::<HsWord8>());
    assert_eq!(align_of::<HsWord8>(), align_of::<HsWord8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord16_layout() {
    assert_eq!(size_of::<HsWord16>(), size_of::<HsWord16>());
    assert_eq!(align_of::<HsWord16>(), align_of::<HsWord16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord32_layout() {
    assert_eq!(size_of::<HsWord32>(), size_of::<HsWord32>());
    assert_eq!(align_of::<HsWord32>(), align_of::<HsWord32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord64_layout() {
    assert_eq!(size_of::<HsWord64>(), size_of::<HsWord64>());
    assert_eq!(align_of::<HsWord64>(), align_of::<HsWord64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsFloat_layout() {
    assert_eq!(size_of::<HsFloat>(), size_of::<HsFloat>());
    assert_eq!(align_of::<HsFloat>(), align_of::<HsFloat>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsDouble_layout() {
    assert_eq!(size_of::<HsDouble>(), size_of::<HsDouble>());
    assert_eq!(align_of::<HsDouble>(), align_of::<HsDouble>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsBool_layout() {
    assert_eq!(size_of::<HsBool>(), size_of::<HsBool>());
    assert_eq!(align_of::<HsBool>(), align_of::<HsBool>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsStablePtr_layout() {
    assert_eq!(size_of::<HsStablePtr>(), size_of::<HsStablePtr>());
    assert_eq!(align_of::<HsStablePtr>(), align_of::<HsStablePtr>());
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
    actual == expected
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
    actual == expected
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
    actual == expected
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
    assert_eq!(actual, expected);
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
    actual == expected
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
