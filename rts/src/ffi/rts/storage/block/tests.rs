#![allow(unused_imports)]
use super::*;
use crate::ffi::stg::W_;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_BLOCK_SIZE_eq() {
    assert_eq!(BLOCK_SIZE, sys::BLOCK_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BLOCK_SIZE_layout() {
    assert_eq!(size_of_val(&BLOCK_SIZE), size_of_val(&sys::BLOCK_SIZE));
    assert_eq!(align_of_val(&BLOCK_SIZE), align_of_val(&sys::BLOCK_SIZE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_MBLOCK_SIZE_eq() {
    assert_eq!(MBLOCK_SIZE, sys::MBLOCK_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MBLOCK_SIZE_layout() {
    assert_eq!(size_of_val(&MBLOCK_SIZE), size_of_val(&sys::MBLOCK_SIZE));
    assert_eq!(align_of_val(&MBLOCK_SIZE), align_of_val(&sys::MBLOCK_SIZE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr___bindgen_ty_1_layout() {
    assert_eq!(
        size_of::<bdescr___bindgen_ty_1>(),
        size_of::<bdescr___bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<bdescr___bindgen_ty_1>(),
        align_of::<bdescr___bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr___bindgen_ty_2_layout() {
    assert_eq!(
        size_of::<bdescr___bindgen_ty_2>(),
        size_of::<bdescr___bindgen_ty_2>()
    );
    assert_eq!(
        align_of::<bdescr___bindgen_ty_2>(),
        align_of::<bdescr___bindgen_ty_2>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr_layout() {
    assert_eq!(size_of::<bdescr>(), size_of::<sys::bdescr>());
    assert_eq!(align_of::<bdescr>(), align_of::<sys::bdescr>());
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
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

#[cfg(feature = "ghc_testsuite")]
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

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
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

#[cfg(feature = "ghc_testsuite")]
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

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
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

#[cfg(feature = "ghc_testsuite")]
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
fn sys_size_generation_() {
    assert_eq!(size_of::<sys::generation_>(), size_of::<generation_>())
}
