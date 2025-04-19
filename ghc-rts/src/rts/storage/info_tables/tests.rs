use std::mem::size_of;

use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq__HNF() {
    assert_eq!(sys::_HNF, _HNF);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__BTM() {
    assert_eq!(sys::_BTM, _BTM);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__NS() {
    assert_eq!(sys::_NS, _NS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__THU() {
    assert_eq!(sys::_THU, _THU);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__MUT() {
    assert_eq!(sys::_MUT, _MUT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__UPT() {
    assert_eq!(sys::_UPT, _UPT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__SRT() {
    assert_eq!(sys::_SRT, _SRT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__IND() {
    assert_eq!(sys::_IND, _IND);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq__FRM() {
    assert_eq!(sys::_FRM, _FRM);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgProfInfo() {
    assert_eq!(size_of::<sys::StgProfInfo>(), size_of::<StgProfInfo>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfInfo"][::core::mem::size_of::<StgProfInfo>() - 16usize];
    ["Alignment of StgProfInfo"][::core::mem::align_of::<StgProfInfo>() - 4usize];
    ["Offset of field: StgProfInfo::closure_type_off"]
        [::core::mem::offset_of!(StgProfInfo, closure_type_off) - 0usize];
    ["Offset of field: StgProfInfo::__pad_closure_type_off"]
        [::core::mem::offset_of!(StgProfInfo, __pad_closure_type_off) - 4usize];
    ["Offset of field: StgProfInfo::closure_desc_off"]
        [::core::mem::offset_of!(StgProfInfo, closure_desc_off) - 8usize];
    ["Offset of field: StgProfInfo::__pad_closure_desc_off"]
        [::core::mem::offset_of!(StgProfInfo, __pad_closure_desc_off) - 12usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgLargeBitmap_() {
    assert_eq!(
        size_of::<sys::StgLargeBitmap_>(),
        size_of::<StgLargeBitmap_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgLargeBitmap_"][::core::mem::size_of::<StgLargeBitmap_>() - 8usize];
    ["Alignment of StgLargeBitmap_"][::core::mem::align_of::<StgLargeBitmap_>() - 8usize];
    ["Offset of field: StgLargeBitmap_::size"]
        [::core::mem::offset_of!(StgLargeBitmap_, size) - 0usize];
    ["Offset of field: StgLargeBitmap_::bitmap"]
        [::core::mem::offset_of!(StgLargeBitmap_, bitmap) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgClosureInfo() {
    assert_eq!(
        size_of::<sys::StgClosureInfo>(),
        size_of::<StgClosureInfo>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgClosureInfo__bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgClosureInfo__bindgen_ty_1>(),
        size_of::<StgClosureInfo__bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosureInfo__bindgen_ty_1"]
        [::core::mem::size_of::<StgClosureInfo__bindgen_ty_1>() - 8usize];
    ["Alignment of StgClosureInfo__bindgen_ty_1"]
        [::core::mem::align_of::<StgClosureInfo__bindgen_ty_1>() - 4usize];
    ["Offset of field: StgClosureInfo__bindgen_ty_1::ptrs"]
        [::core::mem::offset_of!(StgClosureInfo__bindgen_ty_1, ptrs) - 0usize];
    ["Offset of field: StgClosureInfo__bindgen_ty_1::nptrs"]
        [::core::mem::offset_of!(StgClosureInfo__bindgen_ty_1, nptrs) - 4usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosureInfo"][::core::mem::size_of::<StgClosureInfo>() - 8usize];
    ["Alignment of StgClosureInfo"][::core::mem::align_of::<StgClosureInfo>() - 8usize];
    ["Offset of field: StgClosureInfo::payload"]
        [::core::mem::offset_of!(StgClosureInfo, payload) - 0usize];
    ["Offset of field: StgClosureInfo::bitmap"]
        [::core::mem::offset_of!(StgClosureInfo, bitmap) - 0usize];
    ["Offset of field: StgClosureInfo::large_bitmap_offset"]
        [::core::mem::offset_of!(StgClosureInfo, large_bitmap_offset) - 0usize];
    ["Offset of field: StgClosureInfo::__pad_large_bitmap_offset"]
        [::core::mem::offset_of!(StgClosureInfo, __pad_large_bitmap_offset) - 0usize];
    ["Offset of field: StgClosureInfo::selector_offset"]
        [::core::mem::offset_of!(StgClosureInfo, selector_offset) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgInfoTable_() {
    assert_eq!(size_of::<sys::StgInfoTable_>(), size_of::<StgInfoTable_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgInfoTable_"][::core::mem::size_of::<StgInfoTable_>() - 16usize];
    ["Alignment of StgInfoTable_"][::core::mem::align_of::<StgInfoTable_>() - 8usize];
    ["Offset of field: StgInfoTable_::layout"]
        [::core::mem::offset_of!(StgInfoTable_, layout) - 0usize];
    ["Offset of field: StgInfoTable_::type_"]
        [::core::mem::offset_of!(StgInfoTable_, type_) - 8usize];
    ["Offset of field: StgInfoTable_::srt"][::core::mem::offset_of!(StgInfoTable_, srt) - 12usize];
    ["Offset of field: StgInfoTable_::code"]
        [::core::mem::offset_of!(StgInfoTable_, code) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunInfoExtraRev_() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraRev_>(),
        size_of::<StgFunInfoExtraRev_>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunInfoExtraRev___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraRev___bindgen_ty_1>(),
        size_of::<StgFunInfoExtraRev___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraRev___bindgen_ty_1"]
        [::core::mem::size_of::<StgFunInfoExtraRev___bindgen_ty_1>() - 8usize];
    ["Alignment of StgFunInfoExtraRev___bindgen_ty_1"]
        [::core::mem::align_of::<StgFunInfoExtraRev___bindgen_ty_1>() - 8usize];
    ["Offset of field: StgFunInfoExtraRev___bindgen_ty_1::bitmap"]
        [::core::mem::offset_of!(StgFunInfoExtraRev___bindgen_ty_1, bitmap) - 0usize];
    ["Offset of field: StgFunInfoExtraRev___bindgen_ty_1::bitmap_offset"]
        [::core::mem::offset_of!(StgFunInfoExtraRev___bindgen_ty_1, bitmap_offset) - 0usize];
    ["Offset of field: StgFunInfoExtraRev___bindgen_ty_1::__pad_bitmap_offset"]
        [::core::mem::offset_of!(StgFunInfoExtraRev___bindgen_ty_1, __pad_bitmap_offset) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraRev_"][::core::mem::size_of::<StgFunInfoExtraRev_>() - 24usize];
    ["Alignment of StgFunInfoExtraRev_"][::core::mem::align_of::<StgFunInfoExtraRev_>() - 8usize];
    ["Offset of field: StgFunInfoExtraRev_::slow_apply_offset"]
        [::core::mem::offset_of!(StgFunInfoExtraRev_, slow_apply_offset) - 0usize];
    ["Offset of field: StgFunInfoExtraRev_::__pad_slow_apply_offset"]
        [::core::mem::offset_of!(StgFunInfoExtraRev_, __pad_slow_apply_offset) - 4usize];
    ["Offset of field: StgFunInfoExtraRev_::b"]
        [::core::mem::offset_of!(StgFunInfoExtraRev_, b) - 8usize];
    ["Offset of field: StgFunInfoExtraRev_::fun_type"]
        [::core::mem::offset_of!(StgFunInfoExtraRev_, fun_type) - 16usize];
    ["Offset of field: StgFunInfoExtraRev_::arity"]
        [::core::mem::offset_of!(StgFunInfoExtraRev_, arity) - 20usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunInfoExtraFwd_() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraFwd_>(),
        size_of::<StgFunInfoExtraFwd_>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunInfoExtraFwd___bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgFunInfoExtraFwd___bindgen_ty_1>(),
        size_of::<StgFunInfoExtraFwd___bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraFwd___bindgen_ty_1"]
        [::core::mem::size_of::<StgFunInfoExtraFwd___bindgen_ty_1>() - 8usize];
    ["Alignment of StgFunInfoExtraFwd___bindgen_ty_1"]
        [::core::mem::align_of::<StgFunInfoExtraFwd___bindgen_ty_1>() - 8usize];
    ["Offset of field: StgFunInfoExtraFwd___bindgen_ty_1::bitmap"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd___bindgen_ty_1, bitmap) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoExtraFwd_"][::core::mem::size_of::<StgFunInfoExtraFwd_>() - 32usize];
    ["Alignment of StgFunInfoExtraFwd_"][::core::mem::align_of::<StgFunInfoExtraFwd_>() - 8usize];
    ["Offset of field: StgFunInfoExtraFwd_::fun_type"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd_, fun_type) - 0usize];
    ["Offset of field: StgFunInfoExtraFwd_::arity"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd_, arity) - 4usize];
    ["Offset of field: StgFunInfoExtraFwd_::srt"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd_, srt) - 8usize];
    ["Offset of field: StgFunInfoExtraFwd_::b"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd_, b) - 16usize];
    ["Offset of field: StgFunInfoExtraFwd_::slow_apply"]
        [::core::mem::offset_of!(StgFunInfoExtraFwd_, slow_apply) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunInfoTable() {
    assert_eq!(
        size_of::<sys::StgFunInfoTable>(),
        size_of::<StgFunInfoTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunInfoTable"][::core::mem::size_of::<StgFunInfoTable>() - 40usize];
    ["Alignment of StgFunInfoTable"][::core::mem::align_of::<StgFunInfoTable>() - 8usize];
    ["Offset of field: StgFunInfoTable::f"][::core::mem::offset_of!(StgFunInfoTable, f) - 0usize];
    ["Offset of field: StgFunInfoTable::i"][::core::mem::offset_of!(StgFunInfoTable, i) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgRetInfoTable() {
    assert_eq!(
        size_of::<sys::StgRetInfoTable>(),
        size_of::<StgRetInfoTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRetInfoTable"][::core::mem::size_of::<StgRetInfoTable>() - 16usize];
    ["Alignment of StgRetInfoTable"][::core::mem::align_of::<StgRetInfoTable>() - 8usize];
    ["Offset of field: StgRetInfoTable::i"][::core::mem::offset_of!(StgRetInfoTable, i) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgThunkInfoTable_() {
    assert_eq!(
        size_of::<sys::StgThunkInfoTable_>(),
        size_of::<StgThunkInfoTable_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunkInfoTable_"][::core::mem::size_of::<StgThunkInfoTable_>() - 16usize];
    ["Alignment of StgThunkInfoTable_"][::core::mem::align_of::<StgThunkInfoTable_>() - 8usize];
    ["Offset of field: StgThunkInfoTable_::i"]
        [::core::mem::offset_of!(StgThunkInfoTable_, i) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgConInfoTable_() {
    assert_eq!(
        size_of::<sys::StgConInfoTable_>(),
        size_of::<StgConInfoTable_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgConInfoTable_"][::core::mem::size_of::<StgConInfoTable_>() - 24usize];
    ["Alignment of StgConInfoTable_"][::core::mem::align_of::<StgConInfoTable_>() - 8usize];
    ["Offset of field: StgConInfoTable_::con_desc"]
        [::core::mem::offset_of!(StgConInfoTable_, con_desc) - 0usize];
    ["Offset of field: StgConInfoTable_::__pad_con_desc"]
        [::core::mem::offset_of!(StgConInfoTable_, __pad_con_desc) - 4usize];
    ["Offset of field: StgConInfoTable_::i"][::core::mem::offset_of!(StgConInfoTable_, i) - 8usize];
};
