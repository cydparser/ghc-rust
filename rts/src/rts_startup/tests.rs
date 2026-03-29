use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init(argc: i32, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init(&raw mut argc, &raw mut argv) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init_with_rtsopts() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init_with_rtsopts(argc: i32, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };

    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init_ghc() {
    let g = &mut Gen::new(100);

    let actual = {
        let argc: i32 = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let argv = &raw mut argv;
        let rts_config: RtsConfig = todo!();
        unsafe { hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init_ghc(argc: i32, argv: c_char) -> bool {
    let expected = {
        let argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let argv = &raw mut argv;
        let rts_config: sys::RtsConfig = todo!();
        unsafe { sys::hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };

    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        let rts_config: RtsConfig = todo!();
        unsafe { hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_exit() {
    let actual = {
        unsafe { hs_exit() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_exit() {
    let expected = {
        unsafe { sys::hs_exit() };
        todo!()
    };

    let actual = {
        unsafe { hs_exit() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_exit_nowait() {
    let actual = {
        unsafe { hs_exit_nowait() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_exit_nowait() {
    let expected = {
        unsafe { sys::hs_exit_nowait() };
        todo!()
    };

    let actual = {
        unsafe { hs_exit_nowait() };
        todo!()
    };
    assert_eq!(actual, expected);
}
