use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_era_layout() {
    assert_eq!(
        size_of_val(unsafe { &era }),
        size_of_val(unsafe { &sys::era })
    );
    assert_eq!(
        align_of_val(unsafe { &era }),
        align_of_val(unsafe { &sys::era })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_user_era_layout() {
    assert_eq!(
        size_of_val(unsafe { &user_era }),
        size_of_val(unsafe { &sys::user_era })
    );
    assert_eq!(
        align_of_val(unsafe { &user_era }),
        align_of_val(unsafe { &sys::user_era })
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setUserEra() {
    let g = &mut Gen::new(100);

    let actual = {
        let w: StgWord = Arbitrary::arbitrary(g);
        unsafe { setUserEra(w) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setUserEra(w: StgWord) -> bool {
    let expected = {
        unsafe { sys::setUserEra(w) };
        todo!()
    };

    let actual = {
        unsafe { setUserEra(w) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getUserEra() {
    let actual: StgWord = { unsafe { getUserEra() } };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getUserEra() {
    let expected: StgWord = { unsafe { sys::getUserEra() } };
    let actual: StgWord = { unsafe { getUserEra() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_incrementUserEra() {
    let g = &mut Gen::new(100);

    let actual: StgWord = {
        let w: StgWord = Arbitrary::arbitrary(g);
        unsafe { incrementUserEra(w) }
    };

    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_incrementUserEra(w: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::incrementUserEra(w) } };
    let actual: StgWord = { unsafe { incrementUserEra(w) } };
    actual == expected
}
