use crate::stg::types::StgInt;

#[cfg(test)]
mod tests;

pub const TICKY_BIN_COUNT: u32 = 9;

static mut ENT_VIA_NODE_ctr: StgInt = 0;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_ENT_STATIC_THK_SINGLE_ctr")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_STATIC_THK_SINGLE_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ENT_DYN_THK_SINGLE_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_DYN_THK_SINGLE_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ENT_STATIC_THK_MANY_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_STATIC_THK_MANY_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ENT_DYN_THK_MANY_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_DYN_THK_MANY_ctr: StgInt = 0;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_ENT_STATIC_FUN_DIRECT_ctr")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_STATIC_FUN_DIRECT_ctr: StgInt = 0;

static mut ENT_DYN_FUN_DIRECT_ctr: StgInt = 0;

static mut ENT_STATIC_CON_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ENT_DYN_CON_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_DYN_CON_ctr: StgInt = 0;

static mut ENT_STATIC_IND_ctr: StgInt = 0;

static mut ENT_DYN_IND_ctr: StgInt = 0;

static mut ENT_PERM_IND_ctr: StgInt = 0;

static mut ENT_PAP_ctr: StgInt = 0;

static mut ENT_CONTINUATION_ctr: StgInt = 0;

static mut ENT_AP_ctr: StgInt = 0;

static mut ENT_AP_STACK_ctr: StgInt = 0;

static mut ENT_BH_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ENT_LNE_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ENT_LNE_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_UNKNOWN_CALL_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut UNKNOWN_CALL_ctr: StgInt = 0;

static mut SLOW_CALL_fast_v16_ctr: StgInt = 0;

static mut SLOW_CALL_fast_v_ctr: StgInt = 0;

static mut SLOW_CALL_fast_f_ctr: StgInt = 0;

static mut SLOW_CALL_fast_d_ctr: StgInt = 0;

static mut SLOW_CALL_fast_l_ctr: StgInt = 0;

static mut SLOW_CALL_fast_n_ctr: StgInt = 0;

static mut SLOW_CALL_fast_p_ctr: StgInt = 0;

static mut SLOW_CALL_fast_pv_ctr: StgInt = 0;

static mut SLOW_CALL_fast_pp_ctr: StgInt = 0;

static mut SLOW_CALL_fast_ppv_ctr: StgInt = 0;

static mut SLOW_CALL_fast_ppp_ctr: StgInt = 0;

static mut SLOW_CALL_fast_pppv_ctr: StgInt = 0;

static mut SLOW_CALL_fast_pppp_ctr: StgInt = 0;

static mut SLOW_CALL_fast_ppppp_ctr: StgInt = 0;

static mut SLOW_CALL_fast_pppppp_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_VERY_SLOW_CALL_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut VERY_SLOW_CALL_ctr: StgInt = 0;

static mut ticky_slow_call_unevald: StgInt = 0;

static mut SLOW_CALL_ctr: StgInt = 0;

static mut MULTI_CHUNK_SLOW_CALL_ctr: StgInt = 0;

static mut MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_KNOWN_CALL_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut KNOWN_CALL_ctr: StgInt = 0;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_KNOWN_CALL_TOO_FEW_ARGS_ctr")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut KNOWN_CALL_TOO_FEW_ARGS_ctr: StgInt = 0;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_KNOWN_CALL_EXTRA_ARGS_ctr")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut KNOWN_CALL_EXTRA_ARGS_ctr: StgInt = 0;

static mut SLOW_CALL_FUN_TOO_FEW_ctr: StgInt = 0;

static mut SLOW_CALL_FUN_CORRECT_ctr: StgInt = 0;

static mut SLOW_CALL_FUN_TOO_MANY_ctr: StgInt = 0;

static mut SLOW_CALL_PAP_TOO_FEW_ctr: StgInt = 0;

static mut SLOW_CALL_PAP_CORRECT_ctr: StgInt = 0;

static mut SLOW_CALL_PAP_TOO_MANY_ctr: StgInt = 0;

static mut SLOW_CALL_UNEVALD_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_UPDF_OMITTED_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut UPDF_OMITTED_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_UPDF_PUSHED_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut UPDF_PUSHED_ctr: StgInt = 0;

static mut CATCHF_PUSHED_ctr: StgInt = 0;

static mut UPDF_RCC_PUSHED_ctr: StgInt = 0;

static mut UPDF_RCC_OMITTED_ctr: StgInt = 0;

static mut UPD_SQUEEZED_ctr: StgInt = 0;

static mut UPD_CON_IN_NEW_ctr: StgInt = 0;

static mut UPD_CON_IN_PLACE_ctr: StgInt = 0;

static mut UPD_PAP_IN_NEW_ctr: StgInt = 0;

static mut UPD_PAP_IN_PLACE_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_HEAP_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_HEAP_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_HEAP_tot"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_HEAP_tot: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_HEAP_CHK_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut HEAP_CHK_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_STK_CHK_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut STK_CHK_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_RTS_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_RTS_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_RTS_tot"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_RTS_tot: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_FUN_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_FUN_ctr: StgInt = 0;

static mut ALLOC_FUN_adm: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_FUN_gds"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_FUN_gds: StgInt = 0;

static mut ALLOC_FUN_slp: StgInt = 0;

static mut UPD_NEW_IND_ctr: StgInt = 0;

static mut UPD_NEW_PERM_IND_ctr: StgInt = 0;

static mut UPD_OLD_IND_ctr: StgInt = 0;

static mut UPD_OLD_PERM_IND_ctr: StgInt = 0;

static mut UPD_BH_UPDATABLE_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_UPD_CAF_BH_UPDATABLE_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut UPD_CAF_BH_UPDATABLE_ctr: StgInt = 0;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_UPD_CAF_BH_SINGLE_ENTRY_ctr")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut UPD_CAF_BH_SINGLE_ENTRY_ctr: StgInt = 0;

static mut GC_SEL_ABANDONED_ctr: StgInt = 0;

static mut GC_SEL_MINOR_ctr: StgInt = 0;

static mut GC_SEL_MAJOR_ctr: StgInt = 0;

static mut GC_FAILED_PROMOTION_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_UP_THK_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_UP_THK_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_SE_THK_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_SE_THK_ctr: StgInt = 0;

static mut ALLOC_THK_adm: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_THK_gds"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_THK_gds: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_THK_slp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_THK_slp: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_CON_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_CON_ctr: StgInt = 0;

static mut ALLOC_CON_adm: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_CON_gds"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_CON_gds: StgInt = 0;

static mut ALLOC_CON_slp: StgInt = 0;

static mut ALLOC_TUP_ctr: StgInt = 0;

static mut ALLOC_TUP_adm: StgInt = 0;

static mut ALLOC_TUP_gds: StgInt = 0;

static mut ALLOC_TUP_slp: StgInt = 0;

static mut ALLOC_BH_ctr: StgInt = 0;

static mut ALLOC_BH_adm: StgInt = 0;

static mut ALLOC_BH_gds: StgInt = 0;

static mut ALLOC_BH_slp: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PRIM_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PRIM_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PRIM_adm"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PRIM_adm: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PRIM_gds"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PRIM_gds: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PRIM_slp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PRIM_slp: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PAP_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PAP_ctr: StgInt = 0;

static mut ALLOC_PAP_adm: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PAP_gds"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PAP_gds: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ALLOC_PAP_slp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ALLOC_PAP_slp: StgInt = 0;

static mut ALLOC_TSO_ctr: StgInt = 0;

static mut ALLOC_TSO_tot: StgInt = 0;

static mut ALLOC_STACK_ctr: StgInt = 0;

static mut ALLOC_STACK_tot: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_NEW_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_NEW_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_OLD_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_OLD_ctr: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_UNBOXED_TUP_ctr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_UNBOXED_TUP_ctr: StgInt = 0;

static mut RET_SEMI_loads_avoided: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_TAG_UNTAGGED_pred"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut TAG_UNTAGGED_pred: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_TAG_UNTAGGED_miss"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut TAG_UNTAGGED_miss: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_TAG_TAGGED_pred"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut TAG_TAGGED_pred: StgInt = 0;

static mut TAG_TAGGED_miss: StgInt = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_NEW_hst"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_NEW_hst: [StgInt; 9] = [0; 9];

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_OLD_hst"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_OLD_hst: [StgInt; 9usize] = [0; 9];

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RET_UNBOXED_TUP_hst"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RET_UNBOXED_TUP_hst: [StgInt; 9] = [0; 9];
