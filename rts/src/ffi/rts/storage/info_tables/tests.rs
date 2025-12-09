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
