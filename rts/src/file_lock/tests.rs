use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_lockFile() {
    let g = &mut Gen::new(100);

    let actual: i32 = {
        let id: StgWord64 = Arbitrary::arbitrary(g);
        let dev: StgWord64 = Arbitrary::arbitrary(g);
        let ino: StgWord64 = Arbitrary::arbitrary(g);
        let for_writing: i32 = Arbitrary::arbitrary(g);
        unsafe { lockFile(id, dev, ino, for_writing) }
    };

    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_lockFile(id: StgWord64, dev: StgWord64, ino: StgWord64, for_writing: i32) -> bool {
    let expected: i32 = { unsafe { sys::lockFile(id, dev, ino, for_writing) } };
    let actual: i32 = { unsafe { lockFile(id, dev, ino, for_writing) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unlockFile() {
    let g = &mut Gen::new(100);

    let actual: i32 = {
        let id: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { unlockFile(id) }
    };

    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_unlockFile(id: StgWord64) -> bool {
    let expected: i32 = { unsafe { sys::unlockFile(id) } };
    let actual: i32 = { unsafe { unlockFile(id) } };
    actual == expected
}
