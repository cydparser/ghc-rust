use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz8() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz8(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz8(x) } };
    let actual: StgWord = { unsafe { hs_clz8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz16() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz16(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz16(x) } };
    let actual: StgWord = { unsafe { hs_clz16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz32() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz32(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz32(x) } };
    let actual: StgWord = { unsafe { hs_clz32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz64() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_clz64(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz64(x: StgWord64) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz64(x) } };
    let actual: StgWord = { unsafe { hs_clz64(x) } };
    actual == expected
}
