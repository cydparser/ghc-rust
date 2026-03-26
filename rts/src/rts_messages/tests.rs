use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_errorBelch(s: c_char) -> bool {
    let expected = {
        let mut s = s;
        unsafe { sys::errorBelch(&raw mut s) };
        todo!()
    };

    let actual = {
        let mut s = s;
        unsafe { errorBelch(&raw mut s) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_errorBelch() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut s: c_char = Arbitrary::arbitrary(g);
        unsafe { errorBelch(&raw mut s) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_debugBelch(s: c_char) -> bool {
    let expected = {
        let mut s = s;
        unsafe { sys::debugBelch(&raw mut s) };
        todo!()
    };

    let actual = {
        let mut s = s;
        unsafe { debugBelch(&raw mut s) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_debugBelch() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut s: c_char = Arbitrary::arbitrary(g);
        unsafe { debugBelch(&raw mut s) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
