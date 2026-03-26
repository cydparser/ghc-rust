use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt8() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt8(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt8(x) } };
    let actual: StgWord = { unsafe { hs_popcnt8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt16() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt16(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt16(x) } };
    let actual: StgWord = { unsafe { hs_popcnt16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt32() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt32(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt32(x) } };
    let actual: StgWord = { unsafe { hs_popcnt32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt64() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt64(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt64(x: StgWord64) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt64(x) } };
    let actual: StgWord = { unsafe { hs_popcnt64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt(x) } };
    let actual: StgWord = { unsafe { hs_popcnt(x) } };
    actual == expected
}
