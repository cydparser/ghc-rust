use super::*;

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
fn sys_NonmovingSegmentInfo_layout() {
    assert_eq!(
        offset_of!(NonmovingSegmentInfo, allocator_idx),
        offset_of!(sys::NonmovingSegmentInfo, allocator_idx)
    );
    assert_eq!(
        offset_of!(NonmovingSegmentInfo, next_free_snap),
        offset_of!(sys::NonmovingSegmentInfo, next_free_snap)
    );
    assert_eq!(
        size_of::<NonmovingSegmentInfo>(),
        size_of::<sys::NonmovingSegmentInfo>()
    );
    assert_eq!(
        align_of::<NonmovingSegmentInfo>(),
        align_of::<sys::NonmovingSegmentInfo>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr__layout() {
    assert_eq!(offset_of!(bdescr_, start), offset_of!(sys::bdescr_, start));
    assert_eq!(
        offset_of!(bdescr_, union_free_or_nonmoving),
        offset_of!(sys::bdescr_, __bindgen_anon_1)
    );
    assert_eq!(size_of::<*mut bdescr_>(), size_of::<*mut sys::bdescr_>());
    assert_eq!(offset_of!(bdescr_, link), offset_of!(sys::bdescr_, link));
    assert_eq!(offset_of!(bdescr_, u), offset_of!(sys::bdescr_, u));
    assert_eq!(
        size_of::<*mut generation_>(),
        size_of::<*mut sys::generation_>()
    );
    assert_eq!(offset_of!(bdescr_, gen_), offset_of!(sys::bdescr_, gen_));
    assert_eq!(
        offset_of!(bdescr_, gen_no),
        offset_of!(sys::bdescr_, gen_no)
    );
    assert_eq!(
        offset_of!(bdescr_, dest_no),
        offset_of!(sys::bdescr_, dest_no)
    );
    assert_eq!(offset_of!(bdescr_, node), offset_of!(sys::bdescr_, node));
    assert_eq!(offset_of!(bdescr_, flags), offset_of!(sys::bdescr_, flags));
    assert_eq!(
        offset_of!(bdescr_, blocks),
        offset_of!(sys::bdescr_, blocks)
    );
    assert_eq!(
        offset_of!(bdescr_, _padding),
        offset_of!(sys::bdescr_, _padding)
    );
    assert_eq!(size_of::<bdescr_>(), size_of::<sys::bdescr_>());
    assert_eq!(align_of::<bdescr_>(), align_of::<sys::bdescr_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr_free_or_nonmoving_layout() {
    assert_eq!(
        size_of::<bdescr_free_or_nonmoving>(),
        size_of::<sys::bdescr___bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<bdescr_free_or_nonmoving>(),
        align_of::<sys::bdescr___bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr___bindgen_ty_2_layout() {
    assert_eq!(
        size_of::<bdescr___bindgen_ty_2>(),
        size_of::<sys::bdescr___bindgen_ty_2>()
    );
    assert_eq!(
        align_of::<bdescr___bindgen_ty_2>(),
        align_of::<sys::bdescr___bindgen_ty_2>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_bdescr_layout() {
    assert_eq!(size_of::<bdescr>(), size_of::<sys::bdescr>());
    assert_eq!(align_of::<bdescr>(), align_of::<sys::bdescr>());
}
