use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___int_encodeDouble(j: I_, e: I_) -> bool {
    let expected: StgDouble = { unsafe { sys::__int_encodeDouble(j, e) } };
    let actual: StgDouble = { unsafe { __int_encodeDouble(j, e) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___int_encodeDouble() {
    let g = &mut Gen::new(100);
    let actual: StgDouble = {
        let j: I_ = Arbitrary::arbitrary(g);
        let e: I_ = Arbitrary::arbitrary(g);
        unsafe { __int_encodeDouble(j, e) }
    };
    let expected: StgDouble = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___word_encodeDouble(j: W_, e: I_) -> bool {
    let expected: StgDouble = { unsafe { sys::__word_encodeDouble(j, e) } };
    let actual: StgDouble = { unsafe { __word_encodeDouble(j, e) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___word_encodeDouble() {
    let g = &mut Gen::new(100);
    let actual: StgDouble = {
        let j: W_ = Arbitrary::arbitrary(g);
        let e: I_ = Arbitrary::arbitrary(g);
        unsafe { __word_encodeDouble(j, e) }
    };
    let expected: StgDouble = todo!();
    assert_eq!(expected, actual);
}
