use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_loadArchive() {
    let g = &mut Gen::new(100);

    let actual: HsInt = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { loadArchive(&raw mut path) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_loadArchive(path: pathchar) -> bool {
    let expected: HsInt = {
        let mut path = path;
        unsafe { sys::loadArchive(&raw mut path) }
    };

    let actual: HsInt = {
        let mut path = path;
        unsafe { loadArchive(&raw mut path) }
    };

    actual == expected
}
