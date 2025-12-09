use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_mblocks_allocated_layout() {
    assert_eq!(
        size_of_val(unsafe { &mblocks_allocated }),
        size_of_val(unsafe { &sys::mblocks_allocated })
    );
    assert_eq!(
        align_of_val(unsafe { &mblocks_allocated }),
        align_of_val(unsafe { &sys::mblocks_allocated })
    );
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getMBlocks(n: u32) -> bool {
    let expected = {
        let result: &c_void = unsafe { &*sys::getMBlocks(n) };
        todo!()
    };
    let actual = {
        let result: &c_void = unsafe { &*getMBlocks(n) };
        todo!()
    };
    actual == expected
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getMBlocks() {
    let g = &mut Gen::new(100);
    let actual = {
        let n: u32 = Arbitrary::arbitrary(g);
        let result: &c_void = unsafe { &*getMBlocks(n) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_freeMBlocks(n: u32) -> bool {
    let expected = {
        let mut addr: c_void = todo!();
        unsafe { sys::freeMBlocks(&raw mut addr, n) };
        todo!()
    };
    let actual = {
        let mut addr: c_void = todo!();
        unsafe { freeMBlocks(&raw mut addr, n) };
        todo!()
    };
    actual == expected
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_freeMBlocks() {
    let g = &mut Gen::new(100);
    let actual = {
        let addr: c_void = todo!();
        let n: u32 = Arbitrary::arbitrary(g);
        unsafe { freeMBlocks(&raw mut addr, n) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_releaseFreeMemory() {
    let expected = {
        unsafe { sys::releaseFreeMemory() };
        todo!()
    };
    let actual = {
        unsafe { releaseFreeMemory() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_releaseFreeMemory() {
    let actual = {
        unsafe { releaseFreeMemory() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
