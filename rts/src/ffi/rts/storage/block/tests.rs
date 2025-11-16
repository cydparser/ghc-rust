#![allow(unused_imports)]
use super::*;
use crate::prelude::*;
use crate::stg::W_;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_UNIT() {
    assert_eq!(sys::UNIT, UNIT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLOCK_SIZE() {
    assert_eq!(sys::BLOCK_SIZE, BLOCK_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLOCK_MASK() {
    assert_eq!(sys::BLOCK_MASK, BLOCK_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MBLOCK_SIZE() {
    assert_eq!(sys::MBLOCK_SIZE, MBLOCK_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MBLOCK_MASK() {
    assert_eq!(sys::MBLOCK_MASK, MBLOCK_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BDESCR_SIZE() {
    assert_eq!(sys::BDESCR_SIZE, BDESCR_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BDESCR_MASK() {
    assert_eq!(sys::BDESCR_MASK, BDESCR_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BDESCR_SHIFT() {
    assert_eq!(sys::BDESCR_SHIFT, BDESCR_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_EVACUATED() {
    assert_eq!(sys::BF_EVACUATED, BF_EVACUATED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_LARGE() {
    assert_eq!(sys::BF_LARGE, BF_LARGE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_PINNED() {
    assert_eq!(sys::BF_PINNED, BF_PINNED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_MARKED() {
    assert_eq!(sys::BF_MARKED, BF_MARKED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_EXEC() {
    assert_eq!(sys::BF_EXEC, BF_EXEC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_FRAGMENTED() {
    assert_eq!(sys::BF_FRAGMENTED, BF_FRAGMENTED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_KNOWN() {
    assert_eq!(sys::BF_KNOWN, BF_KNOWN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_SWEPT() {
    assert_eq!(sys::BF_SWEPT, BF_SWEPT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_COMPACT() {
    assert_eq!(sys::BF_COMPACT, BF_COMPACT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_NONMOVING() {
    assert_eq!(sys::BF_NONMOVING, BF_NONMOVING);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_NONMOVING_SWEEPING() {
    assert_eq!(sys::BF_NONMOVING_SWEEPING, BF_NONMOVING_SWEEPING);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BF_FLAG_MAX() {
    assert_eq!(sys::BF_FLAG_MAX, BF_FLAG_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_NonmovingSegmentInfo() {
    assert_eq!(
        size_of::<sys::NonmovingSegmentInfo>(),
        size_of::<NonmovingSegmentInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NonmovingSegmentInfo"][size_of::<NonmovingSegmentInfo>() - 4usize];
    ["Alignment of NonmovingSegmentInfo"][align_of::<NonmovingSegmentInfo>() - 2usize];
    ["Offset of field: NonmovingSegmentInfo::allocator_idx"]
        [offset_of!(NonmovingSegmentInfo, allocator_idx) - 0usize];
    ["Offset of field: NonmovingSegmentInfo::next_free_snap"]
        [offset_of!(NonmovingSegmentInfo, next_free_snap) - 2usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_bdescr_() {
    assert_eq!(size_of::<sys::bdescr_>(), size_of::<bdescr_>())
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_bdescr___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_1>(),
        size_of::<bdescr___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr___bindgen_ty_1"][size_of::<bdescr___bindgen_ty_1>() - 8usize];
    ["Alignment of bdescr___bindgen_ty_1"][align_of::<bdescr___bindgen_ty_1>() - 8usize];
    ["Offset of field: bdescr___bindgen_ty_1::free"]
        [offset_of!(bdescr___bindgen_ty_1, free) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_1::nonmoving_segment"]
        [offset_of!(bdescr___bindgen_ty_1, nonmoving_segment) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_bdescr___bindgen_ty_2() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_2>(),
        size_of::<bdescr___bindgen_ty_2>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr___bindgen_ty_2"][size_of::<bdescr___bindgen_ty_2>() - 8usize];
    ["Alignment of bdescr___bindgen_ty_2"][align_of::<bdescr___bindgen_ty_2>() - 8usize];
    ["Offset of field: bdescr___bindgen_ty_2::back"]
        [offset_of!(bdescr___bindgen_ty_2, back) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_2::bitmap"]
        [offset_of!(bdescr___bindgen_ty_2, bitmap) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_2::scan"]
        [offset_of!(bdescr___bindgen_ty_2, scan) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr_"][size_of::<bdescr_>() - 64usize];
    ["Alignment of bdescr_"][align_of::<bdescr_>() - 8usize];
    ["Offset of field: bdescr_::start"][offset_of!(bdescr_, start) - 0usize];
    ["Offset of field: bdescr_::link"][offset_of!(bdescr_, link) - 16usize];
    ["Offset of field: bdescr_::u"][offset_of!(bdescr_, u) - 24usize];
    ["Offset of field: bdescr_::gen_"][offset_of!(bdescr_, gen_) - 32usize];
    ["Offset of field: bdescr_::gen_no"][offset_of!(bdescr_, gen_no) - 40usize];
    ["Offset of field: bdescr_::dest_no"][offset_of!(bdescr_, dest_no) - 42usize];
    ["Offset of field: bdescr_::node"][offset_of!(bdescr_, node) - 44usize];
    ["Offset of field: bdescr_::flags"][offset_of!(bdescr_, flags) - 46usize];
    ["Offset of field: bdescr_::blocks"][offset_of!(bdescr_, blocks) - 48usize];
    ["Offset of field: bdescr_::_padding"][offset_of!(bdescr_, _padding) - 52usize];
};

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
    expected == actual
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
    expected == actual
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
    assert_eq!(expected, actual);
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
