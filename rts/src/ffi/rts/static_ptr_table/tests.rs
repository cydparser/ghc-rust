use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_insert(key: StgWord64) -> bool {
    let expected = {
        let key = key;
        let mut spe_closure: c_void = todo!();
        unsafe { sys::hs_spt_insert(&raw mut key, &raw mut spe_closure) };
        todo!()
    };

    let actual = {
        let mut key = key;
        let mut spe_closure: c_void = todo!();
        unsafe { hs_spt_insert(&raw mut key, &raw mut spe_closure) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_insert() {
    let g = &mut Gen::new(100);
    let actual = {
        let key: StgWord64 = Arbitrary::arbitrary(g);
        let spe_closure: c_void = todo!();
        unsafe { hs_spt_insert(&raw mut key, &raw mut spe_closure) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_insert_stableptr(key: StgWord64) -> bool {
    let expected = {
        let key = key;
        let mut entry: StgStablePtr = todo!();
        unsafe { sys::hs_spt_insert_stableptr(&raw mut key, &raw mut entry) };
        todo!()
    };

    let actual = {
        let mut key = key;
        let mut entry: StgStablePtr = todo!();
        unsafe { hs_spt_insert_stableptr(&raw mut key, &raw mut entry) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_insert_stableptr() {
    let g = &mut Gen::new(100);
    let actual = {
        let key: StgWord64 = Arbitrary::arbitrary(g);
        let entry: StgStablePtr = todo!();
        unsafe { hs_spt_insert_stableptr(&raw mut key, &raw mut entry) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_remove(key: StgWord64) -> bool {
    let expected = {
        let mut key = key;
        unsafe { sys::hs_spt_remove(&raw mut key) };
        todo!()
    };

    let actual = {
        let mut key = key;
        unsafe { hs_spt_remove(&raw mut key) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_spt_remove() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut key: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_spt_remove(&raw mut key) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
