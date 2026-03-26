use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext64() {
    let g = &mut Gen::new(100);

    let actual: StgWord64 = {
        let src: StgWord64 = Arbitrary::arbitrary(g);
        let mask: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_pext64(src, mask) }
    };

    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext64(src: StgWord64, mask: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_pext64(src, mask) } };
    let actual: StgWord64 = { unsafe { hs_pext64(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext32() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext32(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext32(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext32(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext32(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext16() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext16(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext16(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext16(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext16(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext8() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext8(src, mask) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext8(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext8(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext8(src, mask) } };
    actual == expected
}
