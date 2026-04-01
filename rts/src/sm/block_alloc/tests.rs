use super::*;

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_allocAlignedGroupOnNode() {
    let g = &mut Gen::new(100);

    let actual = {
        let node: u32 = Arbitrary::arbitrary(g);
        let n: W_ = Arbitrary::arbitrary(g);
        let result: &bdescr = unsafe { &*allocAlignedGroupOnNode(node, n) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_allocAlignedGroupOnNode(node: u32, n: W_) -> bool {
    let expected = {
        let result: &bdescr = unsafe { transmute(&*sys::allocAlignedGroupOnNode(node, n)) };

        todo!()
    };

    let actual = {
        let result: &bdescr = unsafe { &*allocAlignedGroupOnNode(node, n) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_allocGroup_lock() {
    let g = &mut Gen::new(100);

    let actual = {
        let n: W_ = Arbitrary::arbitrary(g);
        let result: &bdescr = unsafe { &*allocGroup_lock(n) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_allocGroup_lock(n: W_) -> bool {
    let expected = {
        let result: &bdescr = unsafe { transmute(&*sys::allocGroup_lock(n)) };
        todo!()
    };

    let actual = {
        let result: &bdescr = unsafe { &*allocGroup_lock(n) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_freeGroup_lock() {
    let actual = {
        let p: bdescr = todo!();
        unsafe { freeGroup_lock(&raw mut p) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_freeGroup_lock() {
    let expected = {
        let mut p: sys::bdescr = todo!();
        unsafe { sys::freeGroup_lock(&raw mut p) };
        todo!()
    };

    let actual = {
        let mut p: bdescr = todo!();
        unsafe { freeGroup_lock(&raw mut p) };
        todo!()
    };
    assert_eq!(actual, expected);
}
