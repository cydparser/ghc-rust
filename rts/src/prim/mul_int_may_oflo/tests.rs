use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_mulIntMayOflo() {
    let g = &mut Gen::new(100);

    let actual: W_ = {
        let a: W_ = Arbitrary::arbitrary(g);
        let b: W_ = Arbitrary::arbitrary(g);
        unsafe { hs_mulIntMayOflo(a, b) }
    };

    let expected: W_ = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_mulIntMayOflo(a: W_, b: W_) -> bool {
    let expected: W_ = { unsafe { sys::hs_mulIntMayOflo(a, b) } };
    let actual: W_ = { unsafe { hs_mulIntMayOflo(a, b) } };
    actual == expected
}
