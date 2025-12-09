use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_genericRaise(sig: c_int) -> bool {
    let expected: c_int = { unsafe { sys::genericRaise(sig) } };
    let actual: c_int = { unsafe { genericRaise(sig) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_genericRaise() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let sig: c_int = Arbitrary::arbitrary(g);
        unsafe { genericRaise(sig) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}
