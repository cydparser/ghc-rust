use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_word2float32() {
    let g = &mut Gen::new(100);

    let actual: StgFloat = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_word2float32(x) }
    };

    let expected: StgFloat = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_word2float32(x: StgWord) -> bool {
    let expected: StgFloat = { unsafe { sys::hs_word2float32(x) } };
    let actual: StgFloat = { unsafe { hs_word2float32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_word2float64() {
    let g = &mut Gen::new(100);

    let actual: StgDouble = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_word2float64(x) }
    };

    let expected: StgDouble = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_word2float64(x: StgWord) -> bool {
    let expected: StgDouble = { unsafe { sys::hs_word2float64(x) } };
    let actual: StgDouble = { unsafe { hs_word2float64(x) } };
    actual == expected
}
