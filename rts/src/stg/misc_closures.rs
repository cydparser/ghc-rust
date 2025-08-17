use crate::prelude::*;
use crate::rts::storage::closures::{StgClosure, StgClosure_, StgHeader, StgIntCharlikeClosure};
use crate::rts::storage::info_tables::{
    StgClosureInfo, StgFunInfoExtraRev_, StgFunInfoExtraRev__anon_union_1, StgFunInfoTable,
    StgInfoTable, StgInfoTable_, StgThunkInfoTable, StgThunkInfoTable_,
};
use crate::stg::types::StgFunPtr;

#[cfg(test)]
mod tests;

const TODO_StgInfoTable: StgInfoTable = StgInfoTable_ {
    layout: StgClosureInfo { bitmap: 0 },
    type_: 0,
    srt: 0,
    code: __IncompleteArrayField::new(),
};

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_upd_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_bh_upd_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_bh_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_marked_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_noupd_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_orig_thunk_info_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_catch_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_catch_retry_frame_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_retry_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_atomically_frame_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_atomically_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_atomically_waiting_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_catch_stm_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_stm_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_unmaskAsyncExceptionszh_ret_info: StgInfoTable = TODO_StgInfoTable;

static stg_maskUninterruptiblezh_ret_info: StgInfoTable = TODO_StgInfoTable;

static stg_maskAsyncExceptionszh_ret_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_stack_underflow_frame_d_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_stack_underflow_frame_d_info: StgInfoTable = TODO_StgInfoTable;

static stg_stack_underflow_frame_v16_info: StgInfoTable = TODO_StgInfoTable;

static stg_stack_underflow_frame_v32_info: StgInfoTable = TODO_StgInfoTable;

static stg_stack_underflow_frame_v64_info: StgInfoTable = TODO_StgInfoTable;

static stg_keepAlive_frame_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_restore_cccs_d_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_restore_cccs_d_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_restore_cccs_v16_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_restore_cccs_v16_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_restore_cccs_v32_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_restore_cccs_v32_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_restore_cccs_v64_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_restore_cccs_v64_info: StgInfoTable = TODO_StgInfoTable;

static stg_restore_cccs_eval_info: StgInfoTable = TODO_StgInfoTable;

static stg_prompt_frame_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_R1p_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_R1n_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_F1_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_D1_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_L1_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_V_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ctoi_t"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ctoi_t() -> StgFunPtr {
    unsafe { sys::stg_ctoi_t() }
}

static stg_ctoi_t0_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t1_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t2_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ctoi_t3_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ctoi_t3_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t4_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t5_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t6_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t7_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t8_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t9_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t10_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t11_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t12_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t13_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t14_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t15_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t16_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t17_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t18_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t19_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t20_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t21_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t22_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t23_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t24_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t25_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t26_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t27_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t28_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t29_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t30_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t31_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t32_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t33_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t34_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t35_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t36_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t37_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t38_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t39_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t40_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t41_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t42_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t43_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t44_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t45_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t46_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t47_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t48_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t49_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t50_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t51_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t52_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t53_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t54_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t55_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t56_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t57_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t58_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t59_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t60_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t61_info: StgInfoTable = TODO_StgInfoTable;

static stg_ctoi_t62_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_primcall_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_primcall_info: StgInfoTable = TODO_StgInfoTable;

static stg_apply_interp_info: StgInfoTable = TODO_StgInfoTable;

static stg_dead_thread_info: StgInfoTable = TODO_StgInfoTable;

static stg_IND_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_IND_STATIC_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_IND_STATIC_info: StgInfoTable = TODO_StgInfoTable;

static stg_BLACKHOLE_info: StgInfoTable = TODO_StgInfoTable;

static stg_CAF_BLACKHOLE_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust___stg_EAGER_BLACKHOLE_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static __stg_EAGER_BLACKHOLE_info: StgInfoTable = TODO_StgInfoTable;

static stg_WHITEHOLE_info: StgInfoTable = TODO_StgInfoTable;

static stg_BLOCKING_QUEUE_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_BLOCKING_QUEUE_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

const TODO_StgFunInfoTable: StgFunInfoTable = StgFunInfoTable {
    f: StgFunInfoExtraRev_ {
        slow_apply_offset: 0,
        __pad_slow_apply_offset: 0,
        b: StgFunInfoExtraRev__anon_union_1 { bitmap: 0 },
        fun_type: 0,
        arity: 0,
    },
    i: TODO_StgInfoTable,
};

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_BCO_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_BCO_info: StgFunInfoTable = TODO_StgFunInfoTable;

static stg_EVACUATED_info: StgInfoTable = TODO_StgInfoTable;

static stg_WEAK_info: StgInfoTable = TODO_StgInfoTable;

static stg_DEAD_WEAK_info: StgInfoTable = TODO_StgInfoTable;

static stg_C_FINALIZER_LIST_info: StgInfoTable = TODO_StgInfoTable;

static stg_STABLE_NAME_info: StgInfoTable = TODO_StgInfoTable;

static stg_MVAR_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_MVAR_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_TVAR_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_TVAR_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_TSO_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_STACK_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_STACK_info: StgInfoTable = TODO_StgInfoTable;

static stg_RUBBISH_ENTRY_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ARR_WORDS_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ARR_WORDS_info: StgInfoTable = TODO_StgInfoTable;

static stg_MUT_ARR_WORDS_info: StgInfoTable = TODO_StgInfoTable;

static stg_MUT_ARR_PTRS_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_MUT_ARR_PTRS_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_MUT_ARR_PTRS_FROZEN_CLEAN_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_MUT_ARR_PTRS_FROZEN_DIRTY_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_MUT_ARR_PTRS_FROZEN_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_SMALL_MUT_ARR_PTRS_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_SMALL_MUT_ARR_PTRS_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_MUT_VAR_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_MUT_VAR_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

static stg_END_TSO_QUEUE_info: StgInfoTable = TODO_StgInfoTable;

static stg_GCD_CAF_info: StgInfoTable = TODO_StgInfoTable;

static stg_STM_AWOKEN_info: StgInfoTable = TODO_StgInfoTable;

static stg_MSG_TRY_WAKEUP_info: StgInfoTable = TODO_StgInfoTable;

static stg_MSG_THROWTO_info: StgInfoTable = TODO_StgInfoTable;

static stg_MSG_BLACKHOLE_info: StgInfoTable = TODO_StgInfoTable;

static stg_MSG_CLONE_STACK_info: StgInfoTable = TODO_StgInfoTable;

static stg_MSG_NULL_info: StgInfoTable = TODO_StgInfoTable;

static stg_MVAR_TSO_QUEUE_info: StgInfoTable = TODO_StgInfoTable;

static stg_catch_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_PAP_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_PAP_info: StgInfoTable = TODO_StgInfoTable;

static stg_AP_info: StgInfoTable = TODO_StgInfoTable;

static stg_AP_NOUPD_info: StgInfoTable = TODO_StgInfoTable;

static stg_AP_STACK_info: StgInfoTable = TODO_StgInfoTable;

static stg_AP_STACK_NOUPD_info: StgInfoTable = TODO_StgInfoTable;

static stg_CONTINUATION_info: StgInfoTable = TODO_StgInfoTable;

static stg_PROMPT_TAG_info: StgInfoTable = TODO_StgInfoTable;

static stg_dummy_ret_info: StgInfoTable = TODO_StgInfoTable;

static stg_raise_info: StgInfoTable = TODO_StgInfoTable;

static stg_raise_ret_info: StgInfoTable = TODO_StgInfoTable;

static stg_atomically_info: StgInfoTable = TODO_StgInfoTable;

static stg_TVAR_WATCH_QUEUE_info: StgInfoTable = TODO_StgInfoTable;

static stg_TREC_CHUNK_info: StgInfoTable = TODO_StgInfoTable;

static stg_TREC_HEADER_info: StgInfoTable = TODO_StgInfoTable;

static stg_END_STM_WATCH_QUEUE_info: StgInfoTable = TODO_StgInfoTable;

static stg_END_STM_CHUNK_LIST_info: StgInfoTable = TODO_StgInfoTable;

static stg_NO_TREC_info: StgInfoTable = TODO_StgInfoTable;

static stg_COMPACT_NFDATA_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

static stg_COMPACT_NFDATA_DIRTY_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_SRT_1_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_SRT_1_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_2_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_3_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_4_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_5_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_6_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_7_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_8_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_9_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_10_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_11_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_12_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_13_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_14_info: StgInfoTable = TODO_StgInfoTable;

static stg_SRT_15_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_SRT_16_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_SRT_16_info: StgInfoTable = TODO_StgInfoTable;

const TODO_StgClosure: StgClosure = StgClosure_ {
    header: StgHeader { info: null() },
    payload: __IncompleteArrayField::new(),
};

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_END_TSO_QUEUE_closure")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut stg_END_TSO_QUEUE_closure: StgClosure = TODO_StgClosure;

static mut stg_STM_AWOKEN_closure: StgClosure = TODO_StgClosure;

static mut stg_NO_FINALIZER_closure: StgClosure = TODO_StgClosure;

static mut stg_dummy_ret_closure: StgClosure = TODO_StgClosure;

static mut stg_forceIO_closure: StgClosure = TODO_StgClosure;

static mut stg_END_STM_WATCH_QUEUE_closure: StgClosure = TODO_StgClosure;

static mut stg_END_STM_CHUNK_LIST_closure: StgClosure = TODO_StgClosure;

static mut stg_NO_TREC_closure: StgClosure = TODO_StgClosure;

static stg_NO_FINALIZER_info: StgInfoTable = TODO_StgInfoTable;

static mut stg_CHARLIKE_closure: [StgIntCharlikeClosure; 0usize] = [];

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_INTLIKE_closure"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut stg_INTLIKE_closure: [StgIntCharlikeClosure; 0usize] = [];

static stg_forceIO_info: StgInfoTable = TODO_StgInfoTable;

static stg_noforceIO_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_0_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_1_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_2_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_3_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_4_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_5_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_6_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_7_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_8_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_9_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_10_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_11_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_12_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_13_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_14_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_15_upd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_0_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_1_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_2_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_3_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_4_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_5_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_6_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_7_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_8_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_9_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_10_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_11_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_12_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_13_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_14_noupd_info: StgInfoTable = TODO_StgInfoTable;

static stg_sel_15_noupd_info: StgInfoTable = TODO_StgInfoTable;

const TODO_StgThunkInfoTable: StgThunkInfoTable = StgThunkInfoTable_ {
    i: TODO_StgInfoTable,
};

static stg_ap_1_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_2_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_3_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_4_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_5_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_6_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

static stg_ap_7_upd_info: StgThunkInfoTable = TODO_StgThunkInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_unpack_cstring_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_unpack_cstring_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_unpack_cstring_utf8_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_unpack_cstring_utf8_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_v_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ap_v_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_f_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_d_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_l_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_v16_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_v32_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_v64_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_n_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_p_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_pv_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_pp_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ap_pp_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_ppv_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_ppp_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_pppv_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_pppp_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_ppppp_info: StgInfoTable = TODO_StgInfoTable;

static stg_ap_pppppp_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_0_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_0_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_0_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_v_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_v_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_f_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_f_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_d_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_d_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_l_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_l_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_v16_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_v16_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_v16_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_v32_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_v32_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_v32_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_v64_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_v64_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_v64_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_n_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_n_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_n_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_p_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_p_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_p_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_pv_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_pv_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_pp_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_pp_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_pp_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_ppv_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_ppv_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ap_ppp_fast"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_ap_ppp_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_ppp_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_pppv_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_pppv_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_pppp_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_pppp_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_ppppp_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_ppppp_fast() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_pppppp_fast() -> StgFunPtr {
    unsafe { sys::stg_ap_pppppp_fast() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_PAP_apply"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_PAP_apply() -> StgFunPtr {
    unsafe { sys::stg_PAP_apply() }
}

#[instrument]
pub(crate) unsafe fn stg_CONTINUATION_apply() -> StgFunPtr {
    unsafe { sys::stg_CONTINUATION_apply() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_stk_v16() -> StgFunPtr {
    unsafe { sys::stg_ap_stk_v16() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_stk_v32() -> StgFunPtr {
    unsafe { sys::stg_ap_stk_v32() }
}

#[instrument]
pub(crate) unsafe fn stg_ap_stk_v64() -> StgFunPtr {
    unsafe { sys::stg_ap_stk_v64() }
}

#[instrument]
pub(crate) unsafe fn stg_stk_save_v16() -> StgFunPtr {
    unsafe { sys::stg_stk_save_v16() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_stk_save_v32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_stk_save_v32() -> StgFunPtr {
    unsafe { sys::stg_stk_save_v32() }
}

#[instrument]
pub(crate) unsafe fn stg_stk_save_v64() -> StgFunPtr {
    unsafe { sys::stg_stk_save_v64() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_noregs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_noregs() -> StgFunPtr {
    unsafe { sys::stg_gc_noregs() }
}

static stg_ret_v_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ret_p_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ret_p_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ret_n_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ret_n_info: StgInfoTable = TODO_StgInfoTable;

static stg_ret_f_info: StgInfoTable = TODO_StgInfoTable;

static stg_ret_d_info: StgInfoTable = TODO_StgInfoTable;

static stg_ret_l_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ret_t_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ret_t_info: StgInfoTable = TODO_StgInfoTable;

#[instrument]
pub(crate) unsafe fn stg_gc_prim() -> StgFunPtr {
    unsafe { sys::stg_gc_prim() }
}

#[instrument]
pub(crate) unsafe fn stg_gc_prim_p() -> StgFunPtr {
    unsafe { sys::stg_gc_prim_p() }
}

#[instrument]
pub(crate) unsafe fn stg_gc_prim_pp() -> StgFunPtr {
    unsafe { sys::stg_gc_prim_pp() }
}

#[instrument]
pub(crate) unsafe fn stg_gc_prim_n() -> StgFunPtr {
    unsafe { sys::stg_gc_prim_n() }
}

static stg_gc_prim_p_ll_ret_info: StgInfoTable = TODO_StgInfoTable;

static stg_gc_prim_pp_ll_ret_info: StgInfoTable = TODO_StgInfoTable;

#[instrument]
pub(crate) unsafe fn stg_gc_prim_p_ll() -> StgFunPtr {
    unsafe { sys::stg_gc_prim_p_ll() }
}

#[instrument]
pub(crate) unsafe fn stg_gc_prim_pp_ll() -> StgFunPtr {
    unsafe { sys::stg_gc_prim_pp_ll() }
}

static stg_enter_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust___stg_gc_enter_1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __stg_gc_enter_1() -> StgFunPtr {
    unsafe { sys::__stg_gc_enter_1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_unpt_r1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_unpt_r1() -> StgFunPtr {
    unsafe { sys::stg_gc_unpt_r1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_unbx_r1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_unbx_r1() -> StgFunPtr {
    unsafe { sys::stg_gc_unbx_r1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_f1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_f1() -> StgFunPtr {
    unsafe { sys::stg_gc_f1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_d1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_d1() -> StgFunPtr {
    unsafe { sys::stg_gc_d1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_l1"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_l1() -> StgFunPtr {
    unsafe { sys::stg_gc_l1() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_pp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_pp() -> StgFunPtr {
    unsafe { sys::stg_gc_pp() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_ppp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_ppp() -> StgFunPtr {
    unsafe { sys::stg_gc_ppp() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_pppp"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_gc_pppp() -> StgFunPtr {
    unsafe { sys::stg_gc_pppp() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_gc_fun_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_gc_fun_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust___stg_gc_fun"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __stg_gc_fun() -> StgFunPtr {
    unsafe { sys::__stg_gc_fun() }
}

#[instrument]
pub(crate) unsafe fn stg_yield_noregs() -> StgFunPtr {
    unsafe { sys::stg_yield_noregs() }
}

#[instrument]
pub(crate) unsafe fn stg_yield_to_interpreter() -> StgFunPtr {
    unsafe { sys::stg_yield_to_interpreter() }
}

#[instrument]
pub(crate) unsafe fn stg_block_noregs() -> StgFunPtr {
    unsafe { sys::stg_block_noregs() }
}

#[instrument]
pub(crate) unsafe fn stg_block_blackhole() -> StgFunPtr {
    unsafe { sys::stg_block_blackhole() }
}

// TODO: stg_block_blackhole_finally
// #[instrument]
// pub(crate) unsafe fn stg_block_blackhole_finally() -> StgFunPtr {
//     unsafe { sys::stg_block_blackhole_finally() }
// }

#[instrument]
pub(crate) unsafe fn stg_block_takemvar() -> StgFunPtr {
    unsafe { sys::stg_block_takemvar() }
}

#[instrument]
pub(crate) unsafe fn stg_block_readmvar() -> StgFunPtr {
    unsafe { sys::stg_block_readmvar() }
}

static stg_block_takemvar_info: StgInfoTable = TODO_StgInfoTable;

static stg_block_readmvar_info: StgInfoTable = TODO_StgInfoTable;

#[instrument]
pub(crate) unsafe fn stg_block_putmvar() -> StgFunPtr {
    unsafe { sys::stg_block_putmvar() }
}

static stg_block_putmvar_info: StgInfoTable = TODO_StgInfoTable;

#[instrument]
pub(crate) unsafe fn stg_block_stmwait() -> StgFunPtr {
    unsafe { sys::stg_block_stmwait() }
}

#[instrument]
pub(crate) unsafe fn stg_block_throwto() -> StgFunPtr {
    unsafe { sys::stg_block_throwto() }
}

static stg_block_throwto_info: StgInfoTable = TODO_StgInfoTable;

#[instrument]
pub(crate) unsafe fn stg_readIOPortzh() -> StgFunPtr {
    unsafe { sys::stg_readIOPortzh() }
}

#[instrument]
pub(crate) unsafe fn stg_writeIOPortzh() -> StgFunPtr {
    unsafe { sys::stg_writeIOPortzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newIOPortzh() -> StgFunPtr {
    unsafe { sys::stg_newIOPortzh() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_stop_thread_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_stop_thread_info: StgInfoTable = TODO_StgInfoTable;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_returnToStackTop"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_returnToStackTop() -> StgFunPtr {
    unsafe { sys::stg_returnToStackTop() }
}

#[instrument]
pub(crate) unsafe fn stg_returnToSched() -> StgFunPtr {
    unsafe { sys::stg_returnToSched() }
}

#[instrument]
pub(crate) unsafe fn stg_returnToSchedNotPaused() -> StgFunPtr {
    unsafe { sys::stg_returnToSchedNotPaused() }
}

#[instrument]
pub(crate) unsafe fn stg_returnToSchedButFirst() -> StgFunPtr {
    unsafe { sys::stg_returnToSchedButFirst() }
}

#[instrument]
pub(crate) unsafe fn stg_threadFinished() -> StgFunPtr {
    unsafe { sys::stg_threadFinished() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_StgReturn"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn StgReturn() -> StgFunPtr {
    unsafe { sys::StgReturn() }
}

#[instrument]
pub(crate) unsafe fn stg_decodeFloatzuIntzh() -> StgFunPtr {
    unsafe { sys::stg_decodeFloatzuIntzh() }
}

#[instrument]
pub(crate) unsafe fn stg_decodeDoublezu2Intzh() -> StgFunPtr {
    unsafe { sys::stg_decodeDoublezu2Intzh() }
}

#[instrument]
pub(crate) unsafe fn stg_decodeDoublezuInt64zh() -> StgFunPtr {
    unsafe { sys::stg_decodeDoublezuInt64zh() }
}

#[instrument]
pub(crate) unsafe fn stg_unsafeThawArrayzh() -> StgFunPtr {
    unsafe { sys::stg_unsafeThawArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casArrayzh() -> StgFunPtr {
    unsafe { sys::stg_casArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newByteArrayzh() -> StgFunPtr {
    unsafe { sys::stg_newByteArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newPinnedByteArrayzh() -> StgFunPtr {
    unsafe { sys::stg_newPinnedByteArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newAlignedPinnedByteArrayzh() -> StgFunPtr {
    unsafe { sys::stg_newAlignedPinnedByteArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isByteArrayPinnedzh() -> StgFunPtr {
    unsafe { sys::stg_isByteArrayPinnedzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isMutableByteArrayPinnedzh() -> StgFunPtr {
    unsafe { sys::stg_isMutableByteArrayPinnedzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isByteArrayWeaklyPinnedzh() -> StgFunPtr {
    unsafe { sys::stg_isByteArrayWeaklyPinnedzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isMutableByteArrayWeaklyPinnedzh() -> StgFunPtr {
    unsafe { sys::stg_isMutableByteArrayWeaklyPinnedzh() }
}

#[instrument]
pub(crate) unsafe fn stg_shrinkMutableByteArrayzh() -> StgFunPtr {
    unsafe { sys::stg_shrinkMutableByteArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_resizzeMutableByteArrayzh() -> StgFunPtr {
    unsafe { sys::stg_resizzeMutableByteArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_shrinkSmallMutableArrayzh() -> StgFunPtr {
    unsafe { sys::stg_shrinkSmallMutableArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casIntArrayzh() -> StgFunPtr {
    unsafe { sys::stg_casIntArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casInt8Arrayzh() -> StgFunPtr {
    unsafe { sys::stg_casInt8Arrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casInt16Arrayzh() -> StgFunPtr {
    unsafe { sys::stg_casInt16Arrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casInt32Arrayzh() -> StgFunPtr {
    unsafe { sys::stg_casInt32Arrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casInt64Arrayzh() -> StgFunPtr {
    unsafe { sys::stg_casInt64Arrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newArrayzh() -> StgFunPtr {
    unsafe { sys::stg_newArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_copyArrayzh() -> StgFunPtr {
    unsafe { sys::stg_copyArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_copyMutableArrayzh() -> StgFunPtr {
    unsafe { sys::stg_copyMutableArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_cloneArrayzh() -> StgFunPtr {
    unsafe { sys::stg_cloneArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_cloneMutableArrayzh() -> StgFunPtr {
    unsafe { sys::stg_cloneMutableArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_freezzeArrayzh() -> StgFunPtr {
    unsafe { sys::stg_freezzeArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_thawArrayzh() -> StgFunPtr {
    unsafe { sys::stg_thawArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_newSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_unsafeThawSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_unsafeThawSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_cloneSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_cloneSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_cloneSmallMutableArrayzh() -> StgFunPtr {
    unsafe { sys::stg_cloneSmallMutableArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_freezzeSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_freezzeSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_thawSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_thawSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_copySmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_copySmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_copySmallMutableArrayzh() -> StgFunPtr {
    unsafe { sys::stg_copySmallMutableArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casSmallArrayzh() -> StgFunPtr {
    unsafe { sys::stg_casSmallArrayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newMutVarzh() -> StgFunPtr {
    unsafe { sys::stg_newMutVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_atomicModifyMutVar2zh() -> StgFunPtr {
    unsafe { sys::stg_atomicModifyMutVar2zh() }
}

#[instrument]
pub(crate) unsafe fn stg_atomicModifyMutVarzuzh() -> StgFunPtr {
    unsafe { sys::stg_atomicModifyMutVarzuzh() }
}

#[instrument]
pub(crate) unsafe fn stg_casMutVarzh() -> StgFunPtr {
    unsafe { sys::stg_casMutVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isEmptyMVarzh() -> StgFunPtr {
    unsafe { sys::stg_isEmptyMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newMVarzh() -> StgFunPtr {
    unsafe { sys::stg_newMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_takeMVarzh() -> StgFunPtr {
    unsafe { sys::stg_takeMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_putMVarzh() -> StgFunPtr {
    unsafe { sys::stg_putMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_readMVarzh() -> StgFunPtr {
    unsafe { sys::stg_readMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_tryTakeMVarzh() -> StgFunPtr {
    unsafe { sys::stg_tryTakeMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_tryPutMVarzh() -> StgFunPtr {
    unsafe { sys::stg_tryPutMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_tryReadMVarzh() -> StgFunPtr {
    unsafe { sys::stg_tryReadMVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_waitReadzh() -> StgFunPtr {
    unsafe { sys::stg_waitReadzh() }
}

#[instrument]
pub(crate) unsafe fn stg_waitWritezh() -> StgFunPtr {
    unsafe { sys::stg_waitWritezh() }
}

#[instrument]
pub(crate) unsafe fn stg_delayzh() -> StgFunPtr {
    unsafe { sys::stg_delayzh() }
}

#[instrument]
pub(crate) unsafe fn stg_catchzh() -> StgFunPtr {
    unsafe { sys::stg_catchzh() }
}

#[instrument]
pub(crate) unsafe fn stg_raisezh() -> StgFunPtr {
    unsafe { sys::stg_raisezh() }
}

#[instrument]
pub(crate) unsafe fn stg_raiseDivZZerozh() -> StgFunPtr {
    unsafe { sys::stg_raiseDivZZerozh() }
}

#[instrument]
pub(crate) unsafe fn stg_raiseUnderflowzh() -> StgFunPtr {
    unsafe { sys::stg_raiseUnderflowzh() }
}

#[instrument]
pub(crate) unsafe fn stg_raiseOverflowzh() -> StgFunPtr {
    unsafe { sys::stg_raiseOverflowzh() }
}

#[instrument]
pub(crate) unsafe fn stg_raiseIOzh() -> StgFunPtr {
    unsafe { sys::stg_raiseIOzh() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_paniczh"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_paniczh() -> StgFunPtr {
    unsafe { sys::stg_paniczh() }
}

#[instrument]
pub(crate) unsafe fn stg_keepAlivezh() -> StgFunPtr {
    unsafe { sys::stg_keepAlivezh() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_absentErrorzh"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_absentErrorzh() -> StgFunPtr {
    unsafe { sys::stg_absentErrorzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newPromptTagzh() -> StgFunPtr {
    unsafe { sys::stg_newPromptTagzh() }
}

#[instrument]
pub(crate) unsafe fn stg_promptzh() -> StgFunPtr {
    unsafe { sys::stg_promptzh() }
}

#[instrument]
pub(crate) unsafe fn stg_control0zh() -> StgFunPtr {
    unsafe { sys::stg_control0zh() }
}

#[instrument]
pub(crate) unsafe fn stg_control0zh_ll() -> StgFunPtr {
    unsafe { sys::stg_control0zh_ll() }
}

#[instrument]
pub(crate) unsafe fn stg_makeStableNamezh() -> StgFunPtr {
    unsafe { sys::stg_makeStableNamezh() }
}

#[instrument]
pub(crate) unsafe fn stg_makeStablePtrzh() -> StgFunPtr {
    unsafe { sys::stg_makeStablePtrzh() }
}

#[instrument]
pub(crate) unsafe fn stg_deRefStablePtrzh() -> StgFunPtr {
    unsafe { sys::stg_deRefStablePtrzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactAddzh() -> StgFunPtr {
    unsafe { sys::stg_compactAddzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactAddWithSharingzh() -> StgFunPtr {
    unsafe { sys::stg_compactAddWithSharingzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactNewzh() -> StgFunPtr {
    unsafe { sys::stg_compactNewzh() }
}

// TODO: stg_compactAppendzh
// #[instrument]
// pub(crate) unsafe fn stg_compactAppendzh() -> StgFunPtr {
//     unsafe { sys::stg_compactAppendzh() }
// }

#[instrument]
pub(crate) unsafe fn stg_compactResizzezh() -> StgFunPtr {
    unsafe { sys::stg_compactResizzezh() }
}

// TODO: stg_compactGetRootzh
// #[instrument]
// pub(crate) unsafe fn stg_compactGetRootzh() -> StgFunPtr {
//     unsafe { sys::stg_compactGetRootzh() }
// }

#[instrument]
pub(crate) unsafe fn stg_compactContainszh() -> StgFunPtr {
    unsafe { sys::stg_compactContainszh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactContainsAnyzh() -> StgFunPtr {
    unsafe { sys::stg_compactContainsAnyzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactGetFirstBlockzh() -> StgFunPtr {
    unsafe { sys::stg_compactGetFirstBlockzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactGetNextBlockzh() -> StgFunPtr {
    unsafe { sys::stg_compactGetNextBlockzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactAllocateBlockzh() -> StgFunPtr {
    unsafe { sys::stg_compactAllocateBlockzh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactFixupPointerszh() -> StgFunPtr {
    unsafe { sys::stg_compactFixupPointerszh() }
}

#[instrument]
pub(crate) unsafe fn stg_compactSizzezh() -> StgFunPtr {
    unsafe { sys::stg_compactSizzezh() }
}

#[instrument]
pub(crate) unsafe fn stg_forkzh() -> StgFunPtr {
    unsafe { sys::stg_forkzh() }
}

#[instrument]
pub(crate) unsafe fn stg_forkOnzh() -> StgFunPtr {
    unsafe { sys::stg_forkOnzh() }
}

#[instrument]
pub(crate) unsafe fn stg_yieldzh() -> StgFunPtr {
    unsafe { sys::stg_yieldzh() }
}

#[instrument]
pub(crate) unsafe fn stg_killMyself() -> StgFunPtr {
    unsafe { sys::stg_killMyself() }
}

#[instrument]
pub(crate) unsafe fn stg_killThreadzh() -> StgFunPtr {
    unsafe { sys::stg_killThreadzh() }
}

#[instrument]
pub(crate) unsafe fn stg_getMaskingStatezh() -> StgFunPtr {
    unsafe { sys::stg_getMaskingStatezh() }
}

#[instrument]
pub(crate) unsafe fn stg_maskAsyncExceptionszh() -> StgFunPtr {
    unsafe { sys::stg_maskAsyncExceptionszh() }
}

#[instrument]
pub(crate) unsafe fn stg_maskUninterruptiblezh() -> StgFunPtr {
    unsafe { sys::stg_maskUninterruptiblezh() }
}

#[instrument]
pub(crate) unsafe fn stg_unmaskAsyncExceptionszh() -> StgFunPtr {
    unsafe { sys::stg_unmaskAsyncExceptionszh() }
}

// TODO: stg_myThreadIdzh
// #[instrument]
// pub(crate) unsafe fn stg_myThreadIdzh() -> StgFunPtr {
//     unsafe { sys::stg_myThreadIdzh() }
// }

#[instrument]
pub(crate) unsafe fn stg_labelThreadzh() -> StgFunPtr {
    unsafe { sys::stg_labelThreadzh() }
}

#[instrument]
pub(crate) unsafe fn stg_isCurrentThreadBoundzh() -> StgFunPtr {
    unsafe { sys::stg_isCurrentThreadBoundzh() }
}

#[instrument]
pub(crate) unsafe fn stg_threadLabelzh() -> StgFunPtr {
    unsafe { sys::stg_threadLabelzh() }
}

#[instrument]
pub(crate) unsafe fn stg_threadStatuszh() -> StgFunPtr {
    unsafe { sys::stg_threadStatuszh() }
}

#[instrument]
pub(crate) unsafe fn stg_listThreadszh() -> StgFunPtr {
    unsafe { sys::stg_listThreadszh() }
}

#[instrument]
pub(crate) unsafe fn stg_mkWeakzh() -> StgFunPtr {
    unsafe { sys::stg_mkWeakzh() }
}

#[instrument]
pub(crate) unsafe fn stg_mkWeakNoFinalizzerzh() -> StgFunPtr {
    unsafe { sys::stg_mkWeakNoFinalizzerzh() }
}

// TODO: stg_mkWeakForeignzh
// #[instrument]
// pub(crate) unsafe fn stg_mkWeakForeignzh() -> StgFunPtr {
//     unsafe { sys::stg_mkWeakForeignzh() }
// }

#[instrument]
pub(crate) unsafe fn stg_addCFinalizzerToWeakzh() -> StgFunPtr {
    unsafe { sys::stg_addCFinalizzerToWeakzh() }
}

#[instrument]
pub(crate) unsafe fn stg_finalizzeWeakzh() -> StgFunPtr {
    unsafe { sys::stg_finalizzeWeakzh() }
}

#[instrument]
pub(crate) unsafe fn stg_deRefWeakzh() -> StgFunPtr {
    unsafe { sys::stg_deRefWeakzh() }
}

// TODO: stg_runRWzh
// #[instrument]
// pub(crate) unsafe fn stg_runRWzh() -> StgFunPtr {
//     unsafe { sys::stg_runRWzh() }
// }

#[instrument]
pub(crate) unsafe fn stg_newBCOzh() -> StgFunPtr {
    unsafe { sys::stg_newBCOzh() }
}

#[instrument]
pub(crate) unsafe fn stg_mkApUpd0zh() -> StgFunPtr {
    unsafe { sys::stg_mkApUpd0zh() }
}

#[instrument]
pub(crate) unsafe fn stg_retryzh() -> StgFunPtr {
    unsafe { sys::stg_retryzh() }
}

#[instrument]
pub(crate) unsafe fn stg_catchRetryzh() -> StgFunPtr {
    unsafe { sys::stg_catchRetryzh() }
}

#[instrument]
pub(crate) unsafe fn stg_catchSTMzh() -> StgFunPtr {
    unsafe { sys::stg_catchSTMzh() }
}

#[instrument]
pub(crate) unsafe fn stg_atomicallyzh() -> StgFunPtr {
    unsafe { sys::stg_atomicallyzh() }
}

#[instrument]
pub(crate) unsafe fn stg_newTVarzh() -> StgFunPtr {
    unsafe { sys::stg_newTVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_readTVarzh() -> StgFunPtr {
    unsafe { sys::stg_readTVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_readTVarIOzh() -> StgFunPtr {
    unsafe { sys::stg_readTVarIOzh() }
}

#[instrument]
pub(crate) unsafe fn stg_writeTVarzh() -> StgFunPtr {
    unsafe { sys::stg_writeTVarzh() }
}

#[instrument]
pub(crate) unsafe fn stg_unpackClosurezh() -> StgFunPtr {
    unsafe { sys::stg_unpackClosurezh() }
}

#[instrument]
pub(crate) unsafe fn stg_closureSizzezh() -> StgFunPtr {
    unsafe { sys::stg_closureSizzezh() }
}

#[instrument]
pub(crate) unsafe fn stg_whereFromzh() -> StgFunPtr {
    unsafe { sys::stg_whereFromzh() }
}

#[instrument]
pub(crate) unsafe fn stg_getApStackValzh() -> StgFunPtr {
    unsafe { sys::stg_getApStackValzh() }
}

#[instrument]
pub(crate) unsafe fn stg_getSparkzh() -> StgFunPtr {
    unsafe { sys::stg_getSparkzh() }
}

#[instrument]
pub(crate) unsafe fn stg_numSparkszh() -> StgFunPtr {
    unsafe { sys::stg_numSparkszh() }
}

#[instrument]
pub(crate) unsafe fn stg_noDuplicatezh() -> StgFunPtr {
    unsafe { sys::stg_noDuplicatezh() }
}

static stg_noDuplicate_info: StgFunInfoTable = TODO_StgFunInfoTable;

#[instrument]
pub(crate) unsafe fn stg_clearCCSzh() -> StgFunPtr {
    unsafe { sys::stg_clearCCSzh() }
}

#[instrument]
pub(crate) unsafe fn stg_traceEventzh() -> StgFunPtr {
    unsafe { sys::stg_traceEventzh() }
}

#[instrument]
pub(crate) unsafe fn stg_traceBinaryEventzh() -> StgFunPtr {
    unsafe { sys::stg_traceBinaryEventzh() }
}

#[instrument]
pub(crate) unsafe fn stg_traceMarkerzh() -> StgFunPtr {
    unsafe { sys::stg_traceMarkerzh() }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_getThreadAllocationCounterzh")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_getThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { sys::stg_getThreadAllocationCounterzh() }
}

#[instrument]
pub(crate) unsafe fn stg_setThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { sys::stg_setThreadAllocationCounterzh() }
}

#[instrument]
pub(crate) unsafe fn stg_castWord64ToDoublezh() -> StgFunPtr {
    unsafe { sys::stg_castWord64ToDoublezh() }
}

#[instrument]
pub(crate) unsafe fn stg_castDoubleToWord64zh() -> StgFunPtr {
    unsafe { sys::stg_castDoubleToWord64zh() }
}

#[instrument]
pub(crate) unsafe fn stg_castWord32ToFloatzh() -> StgFunPtr {
    unsafe { sys::stg_castWord32ToFloatzh() }
}

#[instrument]
pub(crate) unsafe fn stg_castFloatToWord32zh() -> StgFunPtr {
    unsafe { sys::stg_castFloatToWord32zh() }
}
