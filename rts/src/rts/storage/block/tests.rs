use super::*;

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
fn sys_size_bdescr__anon_union_ty_1() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_1>(),
        size_of::<bdescr__anon_union_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr__anon_union_1"][size_of::<bdescr__anon_union_1>() - 8usize];
    ["Alignment of bdescr__anon_union_1"][align_of::<bdescr__anon_union_1>() - 8usize];
    ["Offset of field: bdescr__anon_union_1::free"]
        [offset_of!(bdescr__anon_union_1, free) - 0usize];
    ["Offset of field: bdescr__anon_union_1::nonmoving_segment"]
        [offset_of!(bdescr__anon_union_1, nonmoving_segment) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_bdescr___bindgen_ty_2() {
    assert_eq!(
        size_of::<sys::bdescr___bindgen_ty_2>(),
        size_of::<bdescr__anon_union_2>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bdescr___bindgen_ty_2"][size_of::<bdescr__anon_union_2>() - 8usize];
    ["Alignment of bdescr__anon_union_2"][align_of::<bdescr__anon_union_2>() - 8usize];
    ["Offset of field: bdescr__anon_union_2::back"]
        [offset_of!(bdescr__anon_union_2, back) - 0usize];
    ["Offset of field: bdescr__anon_union_2::bitmap"]
        [offset_of!(bdescr__anon_union_2, bitmap) - 0usize];
    ["Offset of field: bdescr__anon_union_2::scan"]
        [offset_of!(bdescr__anon_union_2, scan) - 0usize];
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

#[test]
#[ignore]
fn test_initBlockAllocator() {
    unsafe { initBlockAllocator() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroup(n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroup(n)) };
    let actual = unsafe { allocGroup(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroup() {
    let n = Default::default();
    unsafe { allocGroup(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroupOnNode(node, n)) };
    let actual = unsafe { allocGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { allocGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocAlignedGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocAlignedGroupOnNode(node, n)) };
    let actual = unsafe { allocAlignedGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocAlignedGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { allocAlignedGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocMBlockAlignedGroupOnNode(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocMBlockAlignedGroupOnNode(node, n)) };
    let actual = unsafe { allocMBlockAlignedGroupOnNode(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocMBlockAlignedGroupOnNode() {
    let node = Default::default();
    let n = Default::default();
    unsafe { allocMBlockAlignedGroupOnNode(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroup_lock(n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroup_lock(n)) };
    let actual = unsafe { allocGroup_lock(n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroup_lock() {
    let n = Default::default();
    unsafe { allocGroup_lock(n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocBlock_lock() -> bool {
    let expected = unsafe { transmute(sys::allocBlock_lock()) };
    let actual = unsafe { allocBlock_lock() };
    actual == expected
}

#[test]
#[ignore]
fn test_allocBlock_lock() {
    unsafe { allocBlock_lock() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocGroupOnNode_lock(node: u32, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocGroupOnNode_lock(node, n)) };
    let actual = unsafe { allocGroupOnNode_lock(node, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocGroupOnNode_lock() {
    let node = Default::default();
    let n = Default::default();
    unsafe { allocGroupOnNode_lock(node, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocBlockOnNode_lock(node: u32) -> bool {
    let expected = unsafe { transmute(sys::allocBlockOnNode_lock(node)) };
    let actual = unsafe { allocBlockOnNode_lock(node) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocBlockOnNode_lock() {
    let node = Default::default();
    unsafe { allocBlockOnNode_lock(node) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeGroup() {
    let p = null_mut();
    unsafe { freeGroup(p) };
}

#[test]
#[ignore]
fn test_freeChain() {
    let p = null_mut();
    unsafe { freeChain(p) };
}

#[test]
#[ignore]
fn test_freeGroup_lock() {
    let p = null_mut();
    unsafe { freeGroup_lock(p) };
}

#[test]
#[ignore]
fn test_freeChain_lock() {
    let p = null_mut();
    unsafe { freeChain_lock(p) };
}
