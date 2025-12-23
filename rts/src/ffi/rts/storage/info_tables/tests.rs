#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_StgLargeBitmap_layout() {
    assert_eq!(
        size_of::<StgLargeBitmap>(),
        size_of::<sys::StgLargeBitmap>()
    );
    assert_eq!(
        align_of::<StgLargeBitmap>(),
        align_of::<sys::StgLargeBitmap>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgClosureInfo_layout() {
    assert_eq!(
        size_of::<StgClosureInfo>(),
        size_of::<sys::StgClosureInfo>()
    );
    assert_eq!(
        align_of::<StgClosureInfo>(),
        align_of::<sys::StgClosureInfo>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgClosureInfo__bindgen_ty_1_layout() {
    assert_eq!(
        offset_of!(StgClosureInfo__bindgen_ty_1, ptrs),
        offset_of!(sys::StgClosureInfo__bindgen_ty_1, ptrs)
    );
    assert_eq!(
        offset_of!(StgClosureInfo__bindgen_ty_1, nptrs),
        offset_of!(sys::StgClosureInfo__bindgen_ty_1, nptrs)
    );
    assert_eq!(
        size_of::<StgClosureInfo__bindgen_ty_1>(),
        size_of::<sys::StgClosureInfo__bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<StgClosureInfo__bindgen_ty_1>(),
        align_of::<sys::StgClosureInfo__bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgSRTField_layout() {
    assert_eq!(size_of::<StgSRTField>(), size_of::<StgSRTField>());
    assert_eq!(align_of::<StgSRTField>(), align_of::<StgSRTField>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInfoTable__layout() {
    assert_eq!(
        size_of::<StgClosureInfo>(),
        size_of::<sys::StgClosureInfo>()
    );
    assert_eq!(
        offset_of!(StgInfoTable_, layout),
        offset_of!(sys::StgInfoTable_, layout)
    );
    assert_eq!(
        offset_of!(StgInfoTable_, type_),
        offset_of!(sys::StgInfoTable_, type_)
    );
    assert_eq!(
        offset_of!(StgInfoTable_, srt),
        offset_of!(sys::StgInfoTable_, srt)
    );
    assert_eq!(
        offset_of!(StgInfoTable_, code),
        offset_of!(sys::StgInfoTable_, code)
    );
    assert_eq!(size_of::<StgInfoTable_>(), size_of::<sys::StgInfoTable_>());
    assert_eq!(
        align_of::<StgInfoTable_>(),
        align_of::<sys::StgInfoTable_>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoExtraRev__layout() {
    assert_eq!(
        offset_of!(StgFunInfoExtraRev_, slow_apply_offset),
        offset_of!(sys::StgFunInfoExtraRev_, slow_apply_offset)
    );
    assert_eq!(
        size_of::<StgFunInfoExtraRev___bindgen_ty_1>(),
        size_of::<sys::StgFunInfoExtraRev___bindgen_ty_1>()
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraRev_, b),
        offset_of!(sys::StgFunInfoExtraRev_, b)
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraRev_, fun_type),
        offset_of!(sys::StgFunInfoExtraRev_, fun_type)
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraRev_, arity),
        offset_of!(sys::StgFunInfoExtraRev_, arity)
    );
    assert_eq!(
        size_of::<StgFunInfoExtraRev_>(),
        size_of::<sys::StgFunInfoExtraRev_>()
    );
    assert_eq!(
        align_of::<StgFunInfoExtraRev_>(),
        align_of::<sys::StgFunInfoExtraRev_>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoExtraRev___bindgen_ty_1_layout() {
    assert_eq!(
        size_of::<StgFunInfoExtraRev___bindgen_ty_1>(),
        size_of::<sys::StgFunInfoExtraRev___bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<StgFunInfoExtraRev___bindgen_ty_1>(),
        align_of::<sys::StgFunInfoExtraRev___bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoExtraRev_layout() {
    assert_eq!(
        size_of::<StgFunInfoExtraRev>(),
        size_of::<sys::StgFunInfoExtraRev>()
    );
    assert_eq!(
        align_of::<StgFunInfoExtraRev>(),
        align_of::<sys::StgFunInfoExtraRev>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoExtraFwd__layout() {
    assert_eq!(
        offset_of!(StgFunInfoExtraFwd_, fun_type),
        offset_of!(sys::StgFunInfoExtraFwd_, fun_type)
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraFwd_, arity),
        offset_of!(sys::StgFunInfoExtraFwd_, arity)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraFwd_, srt),
        offset_of!(sys::StgFunInfoExtraFwd_, srt)
    );
    assert_eq!(
        size_of::<StgFunInfoExtraFwd___bindgen_ty_1>(),
        size_of::<sys::StgFunInfoExtraFwd___bindgen_ty_1>()
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraFwd_, b),
        offset_of!(sys::StgFunInfoExtraFwd_, b)
    );
    assert_eq!(
        offset_of!(StgFunInfoExtraFwd_, slow_apply),
        offset_of!(sys::StgFunInfoExtraFwd_, slow_apply)
    );
    assert_eq!(
        size_of::<StgFunInfoExtraFwd_>(),
        size_of::<sys::StgFunInfoExtraFwd_>()
    );
    assert_eq!(
        align_of::<StgFunInfoExtraFwd_>(),
        align_of::<sys::StgFunInfoExtraFwd_>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoExtraFwd___bindgen_ty_1_layout() {
    assert_eq!(
        size_of::<StgFunInfoExtraFwd___bindgen_ty_1>(),
        size_of::<sys::StgFunInfoExtraFwd___bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<StgFunInfoExtraFwd___bindgen_ty_1>(),
        align_of::<sys::StgFunInfoExtraFwd___bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunInfoTable_layout() {
    assert_eq!(
        size_of::<StgFunInfoExtraRev>(),
        size_of::<sys::StgFunInfoExtraRev>()
    );
    assert_eq!(
        offset_of!(StgFunInfoTable, f),
        offset_of!(sys::StgFunInfoTable, f)
    );
    assert_eq!(size_of::<StgInfoTable>(), size_of::<sys::StgInfoTable>());
    assert_eq!(
        offset_of!(StgFunInfoTable, i),
        offset_of!(sys::StgFunInfoTable, i)
    );
    assert_eq!(
        size_of::<StgFunInfoTable>(),
        size_of::<sys::StgFunInfoTable>()
    );
    assert_eq!(
        align_of::<StgFunInfoTable>(),
        align_of::<sys::StgFunInfoTable>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_arg_bitmaps_layout() {
    assert_eq!(
        size_of_val(&stg_arg_bitmaps),
        size_of_val(unsafe { &sys::stg_arg_bitmaps })
    );
    assert_eq!(
        align_of_val(&stg_arg_bitmaps),
        align_of_val(unsafe { &sys::stg_arg_bitmaps })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgRetInfoTable_layout() {
    assert_eq!(size_of::<StgInfoTable>(), size_of::<sys::StgInfoTable>());
    assert_eq!(
        offset_of!(StgRetInfoTable, i),
        offset_of!(sys::StgRetInfoTable, i)
    );
    assert_eq!(
        size_of::<StgRetInfoTable>(),
        size_of::<sys::StgRetInfoTable>()
    );
    assert_eq!(
        align_of::<StgRetInfoTable>(),
        align_of::<sys::StgRetInfoTable>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgConInfoTable_layout() {
    assert_eq!(
        size_of::<StgConInfoTable>(),
        size_of::<sys::StgConInfoTable>()
    );
    assert_eq!(
        align_of::<StgConInfoTable>(),
        align_of::<sys::StgConInfoTable>()
    );
}
