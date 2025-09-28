use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_spt_insert_stableptr(key: StgWord64) -> bool {
    let expected = {
        #[expect(unused_mut)]
        let mut key = key;
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
    expected == actual
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
