use std::mem::{size_of, transmute};

use quickcheck_macros::quickcheck;

use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq_UNIT() {
    assert_eq!(sys::UNIT, super::UNIT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BLOCK_SIZE() {
    assert_eq!(sys::BLOCK_SIZE, super::BLOCK_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BLOCK_MASK() {
    assert_eq!(sys::BLOCK_MASK, super::BLOCK_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MBLOCK_SIZE() {
    assert_eq!(sys::MBLOCK_SIZE, super::MBLOCK_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MBLOCK_MASK() {
    assert_eq!(sys::MBLOCK_MASK, super::MBLOCK_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BDESCR_SIZE() {
    assert_eq!(sys::BDESCR_SIZE, super::BDESCR_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BDESCR_MASK() {
    assert_eq!(sys::BDESCR_MASK, super::BDESCR_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BDESCR_SHIFT() {
    assert_eq!(sys::BDESCR_SHIFT, super::BDESCR_SHIFT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_EVACUATED() {
    assert_eq!(sys::BF_EVACUATED, super::BF_EVACUATED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_LARGE() {
    assert_eq!(sys::BF_LARGE, super::BF_LARGE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_PINNED() {
    assert_eq!(sys::BF_PINNED, super::BF_PINNED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_MARKED() {
    assert_eq!(sys::BF_MARKED, super::BF_MARKED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_EXEC() {
    assert_eq!(sys::BF_EXEC, super::BF_EXEC.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_FRAGMENTED() {
    assert_eq!(sys::BF_FRAGMENTED, super::BF_FRAGMENTED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_KNOWN() {
    assert_eq!(sys::BF_KNOWN, super::BF_KNOWN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_SWEPT() {
    assert_eq!(sys::BF_SWEPT, super::BF_SWEPT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_COMPACT() {
    assert_eq!(sys::BF_COMPACT, super::BF_COMPACT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_NONMOVING() {
    assert_eq!(sys::BF_NONMOVING, super::BF_NONMOVING.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_NONMOVING_SWEEPING() {
    assert_eq!(
        sys::BF_NONMOVING_SWEEPING,
        super::BF_NONMOVING_SWEEPING.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BF_FLAG_MAX() {
    assert_eq!(sys::BF_FLAG_MAX, super::BF_FLAG_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_NonmovingSegmentInfo() {
    assert_eq!(
        size_of::<sys::NonmovingSegmentInfo>(),
        size_of::<super::NonmovingSegmentInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NonmovingSegmentInfo"][::core::mem::size_of::<NonmovingSegmentInfo>() - 4usize];
    ["Alignment of NonmovingSegmentInfo"][::core::mem::align_of::<NonmovingSegmentInfo>() - 2usize];
    ["Offset of field: NonmovingSegmentInfo::allocator_idx"]
        [::core::mem::offset_of!(NonmovingSegmentInfo, allocator_idx) - 0usize];
    ["Offset of field: NonmovingSegmentInfo::next_free_snap"]
        [::core::mem::offset_of!(NonmovingSegmentInfo, next_free_snap) - 2usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_bdescr_() {
    assert_eq!(size_of::<sys::bdescr_>(), size_of::<super::bdescr_>())
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_bdescr___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_1>(),
        size_of::<super::bdescr___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr___bindgen_ty_1"][::core::mem::size_of::<bdescr___bindgen_ty_1>() - 8usize];
    ["Alignment of bdescr___bindgen_ty_1"]
        [::core::mem::align_of::<bdescr___bindgen_ty_1>() - 8usize];
    ["Offset of field: bdescr___bindgen_ty_1::free"]
        [::core::mem::offset_of!(bdescr___bindgen_ty_1, free) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_1::nonmoving_segment"]
        [::core::mem::offset_of!(bdescr___bindgen_ty_1, nonmoving_segment) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_bdescr___bindgen_ty_2() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_2>(),
        size_of::<super::bdescr___bindgen_ty_2>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr___bindgen_ty_2"][::core::mem::size_of::<bdescr___bindgen_ty_2>() - 8usize];
    ["Alignment of bdescr___bindgen_ty_2"]
        [::core::mem::align_of::<bdescr___bindgen_ty_2>() - 8usize];
    ["Offset of field: bdescr___bindgen_ty_2::back"]
        [::core::mem::offset_of!(bdescr___bindgen_ty_2, back) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_2::bitmap"]
        [::core::mem::offset_of!(bdescr___bindgen_ty_2, bitmap) - 0usize];
    ["Offset of field: bdescr___bindgen_ty_2::scan"]
        [::core::mem::offset_of!(bdescr___bindgen_ty_2, scan) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr_"][::core::mem::size_of::<bdescr_>() - 64usize];
    ["Alignment of bdescr_"][::core::mem::align_of::<bdescr_>() - 8usize];
    ["Offset of field: bdescr_::start"][::core::mem::offset_of!(bdescr_, start) - 0usize];
    ["Offset of field: bdescr_::link"][::core::mem::offset_of!(bdescr_, link) - 16usize];
    ["Offset of field: bdescr_::u"][::core::mem::offset_of!(bdescr_, u) - 24usize];
    ["Offset of field: bdescr_::gen_"][::core::mem::offset_of!(bdescr_, gen_) - 32usize];
    ["Offset of field: bdescr_::gen_no"][::core::mem::offset_of!(bdescr_, gen_no) - 40usize];
    ["Offset of field: bdescr_::dest_no"][::core::mem::offset_of!(bdescr_, dest_no) - 42usize];
    ["Offset of field: bdescr_::node"][::core::mem::offset_of!(bdescr_, node) - 44usize];
    ["Offset of field: bdescr_::flags"][::core::mem::offset_of!(bdescr_, flags) - 46usize];
    ["Offset of field: bdescr_::blocks"][::core::mem::offset_of!(bdescr_, blocks) - 48usize];
    ["Offset of field: bdescr_::_padding"][::core::mem::offset_of!(bdescr_, _padding) - 52usize];
};

#[test]
#[ignore]
fn test_initBlockAllocator() {
    unsafe { super::initBlockAllocator() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroup(n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroup(n.into())) };
    let actual = unsafe { super::allocGroup(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroup() {
    let n = Default::default();
    unsafe { super::allocGroup(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroupOnNode(node.into(), n.into())) };
    let actual = unsafe { super::allocGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { super::allocGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocAlignedGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocAlignedGroupOnNode(node.into(), n.into())) };
    let actual = unsafe { super::allocAlignedGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocAlignedGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { super::allocAlignedGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocMBlockAlignedGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocMBlockAlignedGroupOnNode(node.into(), n.into())) };
    let actual = unsafe { super::allocMBlockAlignedGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocMBlockAlignedGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { super::allocMBlockAlignedGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroup_lock(n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroup_lock(n.into())) };
    let actual = unsafe { super::allocGroup_lock(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroup_lock() {
    let n = Default::default();
    unsafe { super::allocGroup_lock(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocBlock_lock() -> bool {
    let expected = unsafe { transmute(sys::allocBlock_lock()) };
    let actual = unsafe { super::allocBlock_lock() };
    actual == expected
}

#[test]
#[ignore]
fn test_allocBlock_lock() {
    unsafe { super::allocBlock_lock() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroupOnNode_lock(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroupOnNode_lock(node.into(), n.into())) };
    let actual = unsafe { super::allocGroupOnNode_lock(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroupOnNode_lock() {
    let node = Default::default();
    let n = Default::default();
    unsafe { super::allocGroupOnNode_lock(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocBlockOnNode_lock(node: u32) -> bool {
    let expected = unsafe { transmute(sys::allocBlockOnNode_lock(node.into())) };
    let actual = unsafe { super::allocBlockOnNode_lock(node) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocBlockOnNode_lock() {
    let node = Default::default();
    unsafe { super::allocBlockOnNode_lock(node) };
    todo!("assert")
}
