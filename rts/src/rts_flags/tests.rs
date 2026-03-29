use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RtsFlags_layout() {
    assert_eq!(
        size_of_val(unsafe { &RtsFlags }),
        size_of_val(unsafe { &sys::RtsFlags })
    );
    assert_eq!(
        align_of_val(unsafe { &RtsFlags }),
        align_of_val(unsafe { &sys::RtsFlags })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_defaultRtsConfig_layout() {
    assert_eq!(
        size_of_val(unsafe { &defaultRtsConfig }),
        size_of_val(unsafe { &sys::defaultRtsConfig })
    );
    assert_eq!(
        align_of_val(unsafe { &defaultRtsConfig }),
        align_of_val(unsafe { &sys::defaultRtsConfig })
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getProgArgv() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getProgArgv(argc: i32, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setProgArgv() {
    let g = &mut Gen::new(100);

    let actual = {
        let argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        unsafe { setProgArgv(argc, &raw mut argv) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setProgArgv(argc: i32, argv: c_char) -> bool {
    let expected = {
        let mut argv = argv;
        let mut argv = &raw mut argv;
        unsafe { sys::setProgArgv(argc, &raw mut argv) };
        todo!()
    };

    let actual = {
        let mut argv = argv;
        let mut argv = &raw mut argv;
        unsafe { setProgArgv(argc, &raw mut argv) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getFullProgArgv() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getFullProgArgv(argc: i32, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };

    actual == expected
}
