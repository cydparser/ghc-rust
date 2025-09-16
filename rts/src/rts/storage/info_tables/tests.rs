use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq__HNF() {
    assert_eq!(sys::_HNF, _HNF);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__BTM() {
    assert_eq!(sys::_BTM, _BTM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__NS() {
    assert_eq!(sys::_NS, _NS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__THU() {
    assert_eq!(sys::_THU, _THU);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__MUT() {
    assert_eq!(sys::_MUT, _MUT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__UPT() {
    assert_eq!(sys::_UPT, _UPT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__SRT() {
    assert_eq!(sys::_SRT, _SRT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__IND() {
    assert_eq!(sys::_IND, _IND);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq__FRM() {
    assert_eq!(sys::_FRM, _FRM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgProfInfo() {
    assert_eq!(size_of::<sys::StgProfInfo>(), size_of::<StgProfInfo>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfInfo"][size_of::<StgProfInfo>() - 16usize];
    ["Alignment of StgProfInfo"][align_of::<StgProfInfo>() - 8usize];
    ["Offset of field: StgProfInfo::closure_type_off"]
        [offset_of!(StgProfInfo, closure_type_off) - 0usize];
    ["Offset of field: StgProfInfo::closure_desc_off"]
        [offset_of!(StgProfInfo, closure_desc_off) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgLargeBitmap_() {
    assert_eq!(
        size_of::<sys::StgLargeBitmap_>(),
        size_of::<StgLargeBitmap_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgLargeBitmap_"][size_of::<StgLargeBitmap_>() - 8usize];
    ["Alignment of StgLargeBitmap_"][align_of::<StgLargeBitmap_>() - 8usize];
    ["Offset of field: StgLargeBitmap_::size"][offset_of!(StgLargeBitmap_, size) - 0usize];
    ["Offset of field: StgLargeBitmap_::bitmap"][offset_of!(StgLargeBitmap_, bitmap) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgClosureInfo() {
    assert_eq!(
        size_of::<sys::StgClosureInfo>(),
        size_of::<StgClosureInfo>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgClosureInfo__bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgClosureInfo__bindgen_ty_1>(),
        size_of::<StgClosureInfo__bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosureInfo__bindgen_ty_1"][size_of::<StgClosureInfo__bindgen_ty_1>() - 8usize];
    ["Alignment of StgClosureInfo__bindgen_ty_1"]
        [align_of::<StgClosureInfo__bindgen_ty_1>() - 4usize];
    ["Offset of field: StgClosureInfo__bindgen_ty_1::ptrs"]
        [offset_of!(StgClosureInfo__bindgen_ty_1, ptrs) - 0usize];
    ["Offset of field: StgClosureInfo__bindgen_ty_1::nptrs"]
        [offset_of!(StgClosureInfo__bindgen_ty_1, nptrs) - 4usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosureInfo"][size_of::<StgClosureInfo>() - 8usize];
    ["Alignment of StgClosureInfo"][align_of::<StgClosureInfo>() - 8usize];
    ["Offset of field: StgClosureInfo::payload"][offset_of!(StgClosureInfo, payload) - 0usize];
    ["Offset of field: StgClosureInfo::bitmap"][offset_of!(StgClosureInfo, bitmap) - 0usize];
    ["Offset of field: StgClosureInfo::large_bitmap_offset"]
        [offset_of!(StgClosureInfo, large_bitmap_offset) - 0usize];
    ["Offset of field: StgClosureInfo::selector_offset"]
        [offset_of!(StgClosureInfo, selector_offset) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgInfoTable_() {
    assert_eq!(size_of::<sys::StgInfoTable_>(), size_of::<StgInfoTable_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgInfoTable_"][size_of::<StgInfoTable_>() - 16usize];
    ["Alignment of StgInfoTable_"][align_of::<StgInfoTable_>() - 8usize];
    ["Offset of field: StgInfoTable_::layout"][offset_of!(StgInfoTable_, layout) - 0usize];
    ["Offset of field: StgInfoTable_::type_"][offset_of!(StgInfoTable_, type_) - 8usize];
    ["Offset of field: StgInfoTable_::srt"][offset_of!(StgInfoTable_, srt) - 12usize];
    ["Offset of field: StgInfoTable_::code"][offset_of!(StgInfoTable_, code) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunInfoExtraRev_() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraRev_>(),
        size_of::<StgFunInfoExtraRev_>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunInfoExtraRev___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraRev___bindgen_ty_1>(),
        size_of::<StgFunInfoExtraRev___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraRev___bindgen_ty_1"]
        [size_of::<StgFunInfoExtraRev___bindgen_ty_1>() - 8usize];
    ["Alignment of StgFunInfoExtraRev___bindgen_ty_1"]
        [align_of::<StgFunInfoExtraRev___bindgen_ty_1>() - 8usize];
    ["Offset of field: StgFunInfoExtraRev___bindgen_ty_1::bitmap"]
        [offset_of!(StgFunInfoExtraRev___bindgen_ty_1, bitmap) - 0usize];
    ["Offset of field: StgFunInfoExtraRev___bindgen_ty_1::bitmap_offset"]
        [offset_of!(StgFunInfoExtraRev___bindgen_ty_1, bitmap_offset) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraRev_"][size_of::<StgFunInfoExtraRev_>() - 24usize];
    ["Alignment of StgFunInfoExtraRev_"][align_of::<StgFunInfoExtraRev_>() - 8usize];
    ["Offset of field: StgFunInfoExtraRev_::slow_apply_offset"]
        [offset_of!(StgFunInfoExtraRev_, slow_apply_offset) - 0usize];
    ["Offset of field: StgFunInfoExtraRev_::b"][offset_of!(StgFunInfoExtraRev_, b) - 8usize];
    ["Offset of field: StgFunInfoExtraRev_::fun_type"]
        [offset_of!(StgFunInfoExtraRev_, fun_type) - 16usize];
    ["Offset of field: StgFunInfoExtraRev_::arity"]
        [offset_of!(StgFunInfoExtraRev_, arity) - 20usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunInfoExtraFwd_() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraFwd_>(),
        size_of::<StgFunInfoExtraFwd_>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunInfoExtraFwd___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraFwd___bindgen_ty_1>(),
        size_of::<StgFunInfoExtraFwd___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraFwd___bindgen_ty_1"]
        [size_of::<StgFunInfoExtraFwd___bindgen_ty_1>() - 8usize];
    ["Alignment of StgFunInfoExtraFwd___bindgen_ty_1"]
        [align_of::<StgFunInfoExtraFwd___bindgen_ty_1>() - 8usize];
    ["Offset of field: StgFunInfoExtraFwd___bindgen_ty_1::bitmap"]
        [offset_of!(StgFunInfoExtraFwd___bindgen_ty_1, bitmap) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraFwd_"][size_of::<StgFunInfoExtraFwd_>() - 32usize];
    ["Alignment of StgFunInfoExtraFwd_"][align_of::<StgFunInfoExtraFwd_>() - 8usize];
    ["Offset of field: StgFunInfoExtraFwd_::fun_type"]
        [offset_of!(StgFunInfoExtraFwd_, fun_type) - 0usize];
    ["Offset of field: StgFunInfoExtraFwd_::arity"]
        [offset_of!(StgFunInfoExtraFwd_, arity) - 4usize];
    ["Offset of field: StgFunInfoExtraFwd_::srt"][offset_of!(StgFunInfoExtraFwd_, srt) - 8usize];
    ["Offset of field: StgFunInfoExtraFwd_::b"][offset_of!(StgFunInfoExtraFwd_, b) - 16usize];
    ["Offset of field: StgFunInfoExtraFwd_::slow_apply"]
        [offset_of!(StgFunInfoExtraFwd_, slow_apply) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunInfoTable() {
    assert_eq!(
        size_of::<sys::StgFunInfoTable>(),
        size_of::<StgFunInfoTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoTable"][size_of::<StgFunInfoTable>() - 40usize];
    ["Alignment of StgFunInfoTable"][align_of::<StgFunInfoTable>() - 8usize];
    ["Offset of field: StgFunInfoTable::f"][offset_of!(StgFunInfoTable, f) - 0usize];
    ["Offset of field: StgFunInfoTable::i"][offset_of!(StgFunInfoTable, i) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgRetInfoTable() {
    assert_eq!(
        size_of::<sys::StgRetInfoTable>(),
        size_of::<StgRetInfoTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRetInfoTable"][size_of::<StgRetInfoTable>() - 16usize];
    ["Alignment of StgRetInfoTable"][align_of::<StgRetInfoTable>() - 8usize];
    ["Offset of field: StgRetInfoTable::i"][offset_of!(StgRetInfoTable, i) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgThunkInfoTable_() {
    assert_eq!(
        size_of::<sys::StgThunkInfoTable_>(),
        size_of::<StgThunkInfoTable_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunkInfoTable_"][size_of::<StgThunkInfoTable_>() - 16usize];
    ["Alignment of StgThunkInfoTable_"][align_of::<StgThunkInfoTable_>() - 8usize];
    ["Offset of field: StgThunkInfoTable_::i"][offset_of!(StgThunkInfoTable_, i) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgConInfoTable_() {
    assert_eq!(
        size_of::<sys::StgConInfoTable_>(),
        size_of::<StgConInfoTable_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgConInfoTable_"][size_of::<StgConInfoTable_>() - 24usize];
    ["Alignment of StgConInfoTable_"][align_of::<StgConInfoTable_>() - 8usize];
    ["Offset of field: StgConInfoTable_::con_desc"]
        [offset_of!(StgConInfoTable_, con_desc) - 0usize];
    ["Offset of field: StgConInfoTable_::i"][offset_of!(StgConInfoTable_, i) - 8usize];
};
