#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_STATIC_THK_SINGLE_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_STATIC_THK_SINGLE_ctr }),
        size_of_val(unsafe { &sys::ENT_STATIC_THK_SINGLE_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_STATIC_THK_SINGLE_ctr }),
        align_of_val(unsafe { &sys::ENT_STATIC_THK_SINGLE_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_DYN_THK_SINGLE_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_DYN_THK_SINGLE_ctr }),
        size_of_val(unsafe { &sys::ENT_DYN_THK_SINGLE_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_DYN_THK_SINGLE_ctr }),
        align_of_val(unsafe { &sys::ENT_DYN_THK_SINGLE_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_STATIC_THK_MANY_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_STATIC_THK_MANY_ctr }),
        size_of_val(unsafe { &sys::ENT_STATIC_THK_MANY_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_STATIC_THK_MANY_ctr }),
        align_of_val(unsafe { &sys::ENT_STATIC_THK_MANY_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_DYN_THK_MANY_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_DYN_THK_MANY_ctr }),
        size_of_val(unsafe { &sys::ENT_DYN_THK_MANY_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_DYN_THK_MANY_ctr }),
        align_of_val(unsafe { &sys::ENT_DYN_THK_MANY_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_STATIC_FUN_DIRECT_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_STATIC_FUN_DIRECT_ctr }),
        size_of_val(unsafe { &sys::ENT_STATIC_FUN_DIRECT_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_STATIC_FUN_DIRECT_ctr }),
        align_of_val(unsafe { &sys::ENT_STATIC_FUN_DIRECT_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_DYN_FUN_DIRECT_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_DYN_FUN_DIRECT_ctr }),
        size_of_val(unsafe { &sys::ENT_DYN_FUN_DIRECT_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_DYN_FUN_DIRECT_ctr }),
        align_of_val(unsafe { &sys::ENT_DYN_FUN_DIRECT_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_DYN_CON_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_DYN_CON_ctr }),
        size_of_val(unsafe { &sys::ENT_DYN_CON_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_DYN_CON_ctr }),
        align_of_val(unsafe { &sys::ENT_DYN_CON_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ENT_LNE_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ENT_LNE_ctr }),
        size_of_val(unsafe { &sys::ENT_LNE_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ENT_LNE_ctr }),
        align_of_val(unsafe { &sys::ENT_LNE_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_UNKNOWN_CALL_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &UNKNOWN_CALL_ctr }),
        size_of_val(unsafe { &sys::UNKNOWN_CALL_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &UNKNOWN_CALL_ctr }),
        align_of_val(unsafe { &sys::UNKNOWN_CALL_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_VERY_SLOW_CALL_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &VERY_SLOW_CALL_ctr }),
        size_of_val(unsafe { &sys::VERY_SLOW_CALL_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &VERY_SLOW_CALL_ctr }),
        align_of_val(unsafe { &sys::VERY_SLOW_CALL_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_KNOWN_CALL_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &KNOWN_CALL_ctr }),
        size_of_val(unsafe { &sys::KNOWN_CALL_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &KNOWN_CALL_ctr }),
        align_of_val(unsafe { &sys::KNOWN_CALL_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_KNOWN_CALL_TOO_FEW_ARGS_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &KNOWN_CALL_TOO_FEW_ARGS_ctr }),
        size_of_val(unsafe { &sys::KNOWN_CALL_TOO_FEW_ARGS_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &KNOWN_CALL_TOO_FEW_ARGS_ctr }),
        align_of_val(unsafe { &sys::KNOWN_CALL_TOO_FEW_ARGS_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_KNOWN_CALL_EXTRA_ARGS_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &KNOWN_CALL_EXTRA_ARGS_ctr }),
        size_of_val(unsafe { &sys::KNOWN_CALL_EXTRA_ARGS_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &KNOWN_CALL_EXTRA_ARGS_ctr }),
        align_of_val(unsafe { &sys::KNOWN_CALL_EXTRA_ARGS_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_UPDF_OMITTED_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &UPDF_OMITTED_ctr }),
        size_of_val(unsafe { &sys::UPDF_OMITTED_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &UPDF_OMITTED_ctr }),
        align_of_val(unsafe { &sys::UPDF_OMITTED_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_UPDF_PUSHED_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &UPDF_PUSHED_ctr }),
        size_of_val(unsafe { &sys::UPDF_PUSHED_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &UPDF_PUSHED_ctr }),
        align_of_val(unsafe { &sys::UPDF_PUSHED_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_HEAP_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_HEAP_ctr }),
        size_of_val(unsafe { &sys::ALLOC_HEAP_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_HEAP_ctr }),
        align_of_val(unsafe { &sys::ALLOC_HEAP_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_HEAP_tot_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_HEAP_tot }),
        size_of_val(unsafe { &sys::ALLOC_HEAP_tot })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_HEAP_tot }),
        align_of_val(unsafe { &sys::ALLOC_HEAP_tot })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_HEAP_CHK_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &HEAP_CHK_ctr }),
        size_of_val(unsafe { &sys::HEAP_CHK_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &HEAP_CHK_ctr }),
        align_of_val(unsafe { &sys::HEAP_CHK_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_STK_CHK_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &STK_CHK_ctr }),
        size_of_val(unsafe { &sys::STK_CHK_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &STK_CHK_ctr }),
        align_of_val(unsafe { &sys::STK_CHK_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_FUN_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_FUN_ctr }),
        size_of_val(unsafe { &sys::ALLOC_FUN_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_FUN_ctr }),
        align_of_val(unsafe { &sys::ALLOC_FUN_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_FUN_gds_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_FUN_gds }),
        size_of_val(unsafe { &sys::ALLOC_FUN_gds })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_FUN_gds }),
        align_of_val(unsafe { &sys::ALLOC_FUN_gds })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_UPD_CAF_BH_UPDATABLE_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &UPD_CAF_BH_UPDATABLE_ctr }),
        size_of_val(unsafe { &sys::UPD_CAF_BH_UPDATABLE_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &UPD_CAF_BH_UPDATABLE_ctr }),
        align_of_val(unsafe { &sys::UPD_CAF_BH_UPDATABLE_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_UPD_CAF_BH_SINGLE_ENTRY_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &UPD_CAF_BH_SINGLE_ENTRY_ctr }),
        size_of_val(unsafe { &sys::UPD_CAF_BH_SINGLE_ENTRY_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &UPD_CAF_BH_SINGLE_ENTRY_ctr }),
        align_of_val(unsafe { &sys::UPD_CAF_BH_SINGLE_ENTRY_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_UP_THK_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_UP_THK_ctr }),
        size_of_val(unsafe { &sys::ALLOC_UP_THK_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_UP_THK_ctr }),
        align_of_val(unsafe { &sys::ALLOC_UP_THK_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_SE_THK_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_SE_THK_ctr }),
        size_of_val(unsafe { &sys::ALLOC_SE_THK_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_SE_THK_ctr }),
        align_of_val(unsafe { &sys::ALLOC_SE_THK_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_THK_gds_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_THK_gds }),
        size_of_val(unsafe { &sys::ALLOC_THK_gds })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_THK_gds }),
        align_of_val(unsafe { &sys::ALLOC_THK_gds })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_THK_slp_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_THK_slp }),
        size_of_val(unsafe { &sys::ALLOC_THK_slp })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_THK_slp }),
        align_of_val(unsafe { &sys::ALLOC_THK_slp })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_CON_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_CON_ctr }),
        size_of_val(unsafe { &sys::ALLOC_CON_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_CON_ctr }),
        align_of_val(unsafe { &sys::ALLOC_CON_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_CON_gds_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_CON_gds }),
        size_of_val(unsafe { &sys::ALLOC_CON_gds })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_CON_gds }),
        align_of_val(unsafe { &sys::ALLOC_CON_gds })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PRIM_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PRIM_ctr }),
        size_of_val(unsafe { &sys::ALLOC_PRIM_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PRIM_ctr }),
        align_of_val(unsafe { &sys::ALLOC_PRIM_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PRIM_adm_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PRIM_adm }),
        size_of_val(unsafe { &sys::ALLOC_PRIM_adm })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PRIM_adm }),
        align_of_val(unsafe { &sys::ALLOC_PRIM_adm })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PRIM_gds_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PRIM_gds }),
        size_of_val(unsafe { &sys::ALLOC_PRIM_gds })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PRIM_gds }),
        align_of_val(unsafe { &sys::ALLOC_PRIM_gds })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PRIM_slp_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PRIM_slp }),
        size_of_val(unsafe { &sys::ALLOC_PRIM_slp })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PRIM_slp }),
        align_of_val(unsafe { &sys::ALLOC_PRIM_slp })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PAP_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PAP_ctr }),
        size_of_val(unsafe { &sys::ALLOC_PAP_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PAP_ctr }),
        align_of_val(unsafe { &sys::ALLOC_PAP_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PAP_gds_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PAP_gds }),
        size_of_val(unsafe { &sys::ALLOC_PAP_gds })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PAP_gds }),
        align_of_val(unsafe { &sys::ALLOC_PAP_gds })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ALLOC_PAP_slp_layout() {
    assert_eq!(
        size_of_val(unsafe { &ALLOC_PAP_slp }),
        size_of_val(unsafe { &sys::ALLOC_PAP_slp })
    );
    assert_eq!(
        align_of_val(unsafe { &ALLOC_PAP_slp }),
        align_of_val(unsafe { &sys::ALLOC_PAP_slp })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_NEW_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_NEW_ctr }),
        size_of_val(unsafe { &sys::RET_NEW_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_NEW_ctr }),
        align_of_val(unsafe { &sys::RET_NEW_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_OLD_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_OLD_ctr }),
        size_of_val(unsafe { &sys::RET_OLD_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_OLD_ctr }),
        align_of_val(unsafe { &sys::RET_OLD_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_UNBOXED_TUP_ctr_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_UNBOXED_TUP_ctr }),
        size_of_val(unsafe { &sys::RET_UNBOXED_TUP_ctr })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_UNBOXED_TUP_ctr }),
        align_of_val(unsafe { &sys::RET_UNBOXED_TUP_ctr })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_TAG_UNTAGGED_pred_layout() {
    assert_eq!(
        size_of_val(unsafe { &TAG_UNTAGGED_pred }),
        size_of_val(unsafe { &sys::TAG_UNTAGGED_pred })
    );
    assert_eq!(
        align_of_val(unsafe { &TAG_UNTAGGED_pred }),
        align_of_val(unsafe { &sys::TAG_UNTAGGED_pred })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_TAG_UNTAGGED_miss_layout() {
    assert_eq!(
        size_of_val(unsafe { &TAG_UNTAGGED_miss }),
        size_of_val(unsafe { &sys::TAG_UNTAGGED_miss })
    );
    assert_eq!(
        align_of_val(unsafe { &TAG_UNTAGGED_miss }),
        align_of_val(unsafe { &sys::TAG_UNTAGGED_miss })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_TAG_TAGGED_pred_layout() {
    assert_eq!(
        size_of_val(unsafe { &TAG_TAGGED_pred }),
        size_of_val(unsafe { &sys::TAG_TAGGED_pred })
    );
    assert_eq!(
        align_of_val(unsafe { &TAG_TAGGED_pred }),
        align_of_val(unsafe { &sys::TAG_TAGGED_pred })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_NEW_hst_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_NEW_hst }),
        size_of_val(unsafe { &sys::RET_NEW_hst })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_NEW_hst }),
        align_of_val(unsafe { &sys::RET_NEW_hst })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_OLD_hst_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_OLD_hst }),
        size_of_val(unsafe { &sys::RET_OLD_hst })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_OLD_hst }),
        align_of_val(unsafe { &sys::RET_OLD_hst })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_RET_UNBOXED_TUP_hst_layout() {
    assert_eq!(
        size_of_val(unsafe { &RET_UNBOXED_TUP_hst }),
        size_of_val(unsafe { &sys::RET_UNBOXED_TUP_hst })
    );
    assert_eq!(
        align_of_val(unsafe { &RET_UNBOXED_TUP_hst }),
        align_of_val(unsafe { &sys::RET_UNBOXED_TUP_hst })
    );
}
