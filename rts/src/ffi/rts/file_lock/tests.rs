use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_lockFile(id: StgWord64, dev: StgWord64, ino: StgWord64, for_writing: c_int) -> bool {
    let expected: c_int = { unsafe { sys::lockFile(id, dev, ino, for_writing) } };
    let actual: c_int = { unsafe { lockFile(id, dev, ino, for_writing) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_lockFile() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let id: StgWord64 = Arbitrary::arbitrary(g);
        let dev: StgWord64 = Arbitrary::arbitrary(g);
        let ino: StgWord64 = Arbitrary::arbitrary(g);
        let for_writing: c_int = Arbitrary::arbitrary(g);
        unsafe { lockFile(id, dev, ino, for_writing) }
    };

    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_unlockFile(id: StgWord64) -> bool {
    let expected: c_int = { unsafe { sys::unlockFile(id) } };
    let actual: c_int = { unsafe { unlockFile(id) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unlockFile() {
    let g = &mut Gen::new(100);
    let actual: c_int = {
        let id: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { unlockFile(id) }
    };

    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}
