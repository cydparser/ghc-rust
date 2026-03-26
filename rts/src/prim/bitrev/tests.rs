use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev8() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev8(x) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_bitrev8(x) } };
    let actual: StgWord = { unsafe { hs_bitrev8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev16() {
    let g = &mut Gen::new(100);

    let actual: StgWord16 = {
        let x: StgWord16 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev16(x) }
    };

    let expected: StgWord16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev16(x: StgWord16) -> bool {
    let expected: StgWord16 = { unsafe { sys::hs_bitrev16(x) } };
    let actual: StgWord16 = { unsafe { hs_bitrev16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev32() {
    let g = &mut Gen::new(100);

    let actual: StgWord32 = {
        let x: StgWord32 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev32(x) }
    };

    let expected: StgWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev32(x: StgWord32) -> bool {
    let expected: StgWord32 = { unsafe { sys::hs_bitrev32(x) } };
    let actual: StgWord32 = { unsafe { hs_bitrev32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev64() {
    let g = &mut Gen::new(100);

    let actual: StgWord64 = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev64(x) }
    };

    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev64(x: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_bitrev64(x) } };
    let actual: StgWord64 = { unsafe { hs_bitrev64(x) } };
    actual == expected
}
