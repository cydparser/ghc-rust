use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep64() {
    let g = &mut Gen::new(100);

    let actual: StgWord64 = {
        let src: StgWord64 = Arbitrary::arbitrary(g);
        let mask: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_pdep64(src, mask) }
    };

    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep64(src: StgWord64, mask: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_pdep64(src, mask) } };
    let actual: StgWord64 = { unsafe { hs_pdep64(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep32() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep32(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep32(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep32(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep32(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep16() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep16(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep16(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep16(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep16(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep8() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep8(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep8(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep8(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep8(src, mask) } };
    actual == expected
}
