use std::mem::transmute;

use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::rts::types::StgInfoTable;
use crate::stg::types::{StgFunPtr, StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

#[unsafe(no_mangle)]
pub static stg_upd_frame_info: StgInfoTable = unsafe { transmute(sys::stg_upd_frame_info) };

#[unsafe(no_mangle)]
pub static stg_bh_upd_frame_info: StgInfoTable = unsafe { sys::stg_bh_upd_frame_info };

static stg_marked_upd_frame_info: StgInfoTable = unsafe { sys::stg_marked_upd_frame_info };

static stg_noupd_frame_info: StgInfoTable = unsafe { sys::stg_noupd_frame_info };

static stg_orig_thunk_info_frame_info: StgInfoTable =
    unsafe { sys::stg_orig_thunk_info_frame_info };

#[unsafe(no_mangle)]
pub static stg_catch_frame_info: StgInfoTable = unsafe { sys::stg_catch_frame_info };

#[unsafe(no_mangle)]
pub static stg_catch_retry_frame_info: StgInfoTable = unsafe { sys::stg_catch_retry_frame_info };

#[unsafe(no_mangle)]
pub static stg_atomically_frame_info: StgInfoTable = unsafe { sys::stg_atomically_frame_info };

static stg_atomically_waiting_frame_info: StgInfoTable =
    unsafe { sys::stg_atomically_waiting_frame_info };

#[unsafe(no_mangle)]
pub static stg_catch_stm_frame_info: StgInfoTable = unsafe { sys::stg_catch_stm_frame_info };

static stg_unmaskAsyncExceptionszh_ret_info: StgInfoTable =
    unsafe { sys::stg_unmaskAsyncExceptionszh_ret_info };

static stg_maskUninterruptiblezh_ret_info: StgInfoTable =
    unsafe { sys::stg_maskUninterruptiblezh_ret_info };

static stg_maskAsyncExceptionszh_ret_info: StgInfoTable =
    unsafe { sys::stg_maskAsyncExceptionszh_ret_info };

#[unsafe(no_mangle)]
pub static stg_stack_underflow_frame_d_info: StgInfoTable =
    unsafe { sys::stg_stack_underflow_frame_d_info };

static stg_stack_underflow_frame_v16_info: StgInfoTable =
    unsafe { sys::stg_stack_underflow_frame_v16_info };

static stg_stack_underflow_frame_v32_info: StgInfoTable =
    unsafe { sys::stg_stack_underflow_frame_v32_info };

static stg_stack_underflow_frame_v64_info: StgInfoTable =
    unsafe { sys::stg_stack_underflow_frame_v64_info };

static stg_keepAlive_frame_info: StgInfoTable = unsafe { sys::stg_keepAlive_frame_info };

#[unsafe(no_mangle)]
pub static stg_restore_cccs_d_info: StgInfoTable = unsafe { sys::stg_restore_cccs_d_info };

#[unsafe(no_mangle)]
pub static stg_restore_cccs_v16_info: StgInfoTable = unsafe { sys::stg_restore_cccs_v16_info };

#[unsafe(no_mangle)]
pub static stg_restore_cccs_v32_info: StgInfoTable = unsafe { sys::stg_restore_cccs_v32_info };

#[unsafe(no_mangle)]
pub static stg_restore_cccs_v64_info: StgInfoTable = unsafe { sys::stg_restore_cccs_v64_info };

static stg_restore_cccs_eval_info: StgInfoTable = unsafe { sys::stg_restore_cccs_eval_info };

static stg_prompt_frame_info: StgInfoTable = unsafe { sys::stg_prompt_frame_info };

static stg_ctoi_R1p_info: StgInfoTable = unsafe { sys::stg_ctoi_R1p_info };

static stg_ctoi_R1n_info: StgInfoTable = unsafe { sys::stg_ctoi_R1n_info };

static stg_ctoi_F1_info: StgInfoTable = unsafe { sys::stg_ctoi_F1_info };

static stg_ctoi_D1_info: StgInfoTable = unsafe { sys::stg_ctoi_D1_info };

static stg_ctoi_L1_info: StgInfoTable = unsafe { sys::stg_ctoi_L1_info };

static stg_ctoi_V_info: StgInfoTable = unsafe { sys::stg_ctoi_V_info };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ctoi_t() -> StgFunPtr {
    unsafe { transmute(sys::stg_ctoi_t()) }
}

static stg_ctoi_t0_info: StgInfoTable = unsafe { sys::stg_ctoi_t0_info };

static stg_ctoi_t1_info: StgInfoTable = unsafe { sys::stg_ctoi_t1_info };

static stg_ctoi_t2_info: StgInfoTable = unsafe { sys::stg_ctoi_t2_info };

#[unsafe(no_mangle)]
pub static stg_ctoi_t3_info: StgInfoTable = unsafe { sys::stg_ctoi_t3_info };

static stg_ctoi_t4_info: StgInfoTable = unsafe { sys::stg_ctoi_t4_info };

static stg_ctoi_t5_info: StgInfoTable = unsafe { sys::stg_ctoi_t5_info };

static stg_ctoi_t6_info: StgInfoTable = unsafe { sys::stg_ctoi_t6_info };

static stg_ctoi_t7_info: StgInfoTable = unsafe { sys::stg_ctoi_t7_info };

static stg_ctoi_t8_info: StgInfoTable = unsafe { sys::stg_ctoi_t8_info };

static stg_ctoi_t9_info: StgInfoTable = unsafe { sys::stg_ctoi_t9_info };

static stg_ctoi_t10_info: StgInfoTable = unsafe { sys::stg_ctoi_t10_info };

static stg_ctoi_t11_info: StgInfoTable = unsafe { sys::stg_ctoi_t11_info };

static stg_ctoi_t12_info: StgInfoTable = unsafe { sys::stg_ctoi_t12_info };

static stg_ctoi_t13_info: StgInfoTable = unsafe { sys::stg_ctoi_t13_info };

static stg_ctoi_t14_info: StgInfoTable = unsafe { sys::stg_ctoi_t14_info };

static stg_ctoi_t15_info: StgInfoTable = unsafe { sys::stg_ctoi_t15_info };

static stg_ctoi_t16_info: StgInfoTable = unsafe { sys::stg_ctoi_t16_info };

static stg_ctoi_t17_info: StgInfoTable = unsafe { sys::stg_ctoi_t17_info };

static stg_ctoi_t18_info: StgInfoTable = unsafe { sys::stg_ctoi_t18_info };

static stg_ctoi_t19_info: StgInfoTable = unsafe { sys::stg_ctoi_t19_info };

static stg_ctoi_t20_info: StgInfoTable = unsafe { sys::stg_ctoi_t20_info };

static stg_ctoi_t21_info: StgInfoTable = unsafe { sys::stg_ctoi_t21_info };

static stg_ctoi_t22_info: StgInfoTable = unsafe { sys::stg_ctoi_t22_info };

static stg_ctoi_t23_info: StgInfoTable = unsafe { sys::stg_ctoi_t23_info };

static stg_ctoi_t24_info: StgInfoTable = unsafe { sys::stg_ctoi_t24_info };

static stg_ctoi_t25_info: StgInfoTable = unsafe { sys::stg_ctoi_t25_info };

static stg_ctoi_t26_info: StgInfoTable = unsafe { sys::stg_ctoi_t26_info };

static stg_ctoi_t27_info: StgInfoTable = unsafe { sys::stg_ctoi_t27_info };

static stg_ctoi_t28_info: StgInfoTable = unsafe { sys::stg_ctoi_t28_info };

static stg_ctoi_t29_info: StgInfoTable = unsafe { sys::stg_ctoi_t29_info };

static stg_ctoi_t30_info: StgInfoTable = unsafe { sys::stg_ctoi_t30_info };

static stg_ctoi_t31_info: StgInfoTable = unsafe { sys::stg_ctoi_t31_info };

static stg_ctoi_t32_info: StgInfoTable = unsafe { sys::stg_ctoi_t32_info };

static stg_ctoi_t33_info: StgInfoTable = unsafe { sys::stg_ctoi_t33_info };

static stg_ctoi_t34_info: StgInfoTable = unsafe { sys::stg_ctoi_t34_info };

static stg_ctoi_t35_info: StgInfoTable = unsafe { sys::stg_ctoi_t35_info };

static stg_ctoi_t36_info: StgInfoTable = unsafe { sys::stg_ctoi_t36_info };

static stg_ctoi_t37_info: StgInfoTable = unsafe { sys::stg_ctoi_t37_info };

static stg_ctoi_t38_info: StgInfoTable = unsafe { sys::stg_ctoi_t38_info };

static stg_ctoi_t39_info: StgInfoTable = unsafe { sys::stg_ctoi_t39_info };

static stg_ctoi_t40_info: StgInfoTable = unsafe { sys::stg_ctoi_t40_info };

static stg_ctoi_t41_info: StgInfoTable = unsafe { sys::stg_ctoi_t41_info };

static stg_ctoi_t42_info: StgInfoTable = unsafe { sys::stg_ctoi_t42_info };

static stg_ctoi_t43_info: StgInfoTable = unsafe { sys::stg_ctoi_t43_info };

static stg_ctoi_t44_info: StgInfoTable = unsafe { sys::stg_ctoi_t44_info };

static stg_ctoi_t45_info: StgInfoTable = unsafe { sys::stg_ctoi_t45_info };

static stg_ctoi_t46_info: StgInfoTable = unsafe { sys::stg_ctoi_t46_info };

static stg_ctoi_t47_info: StgInfoTable = unsafe { sys::stg_ctoi_t47_info };

static stg_ctoi_t48_info: StgInfoTable = unsafe { sys::stg_ctoi_t48_info };

static stg_ctoi_t49_info: StgInfoTable = unsafe { sys::stg_ctoi_t49_info };

static stg_ctoi_t50_info: StgInfoTable = unsafe { sys::stg_ctoi_t50_info };

static stg_ctoi_t51_info: StgInfoTable = unsafe { sys::stg_ctoi_t51_info };

static stg_ctoi_t52_info: StgInfoTable = unsafe { sys::stg_ctoi_t52_info };

static stg_ctoi_t53_info: StgInfoTable = unsafe { sys::stg_ctoi_t53_info };

static stg_ctoi_t54_info: StgInfoTable = unsafe { sys::stg_ctoi_t54_info };

static stg_ctoi_t55_info: StgInfoTable = unsafe { sys::stg_ctoi_t55_info };

static stg_ctoi_t56_info: StgInfoTable = unsafe { sys::stg_ctoi_t56_info };

static stg_ctoi_t57_info: StgInfoTable = unsafe { sys::stg_ctoi_t57_info };

static stg_ctoi_t58_info: StgInfoTable = unsafe { sys::stg_ctoi_t58_info };

static stg_ctoi_t59_info: StgInfoTable = unsafe { sys::stg_ctoi_t59_info };

static stg_ctoi_t60_info: StgInfoTable = unsafe { sys::stg_ctoi_t60_info };

static stg_ctoi_t61_info: StgInfoTable = unsafe { sys::stg_ctoi_t61_info };

static stg_ctoi_t62_info: StgInfoTable = unsafe { sys::stg_ctoi_t62_info };

#[unsafe(no_mangle)]
pub static stg_primcall_info: StgInfoTable = unsafe { sys::stg_primcall_info };

static stg_apply_interp_info: StgInfoTable = unsafe { sys::stg_apply_interp_info };

static stg_dead_thread_info: StgInfoTable = unsafe { sys::stg_dead_thread_info };

static stg_IND_info: StgInfoTable = unsafe { sys::stg_IND_info };

#[unsafe(no_mangle)]
pub static stg_IND_STATIC_info: StgInfoTable = unsafe { sys::stg_IND_STATIC_info };

static stg_BLACKHOLE_info: StgInfoTable = unsafe { sys::stg_BLACKHOLE_info };

static stg_CAF_BLACKHOLE_info: StgInfoTable = unsafe { sys::stg_CAF_BLACKHOLE_info };

#[unsafe(no_mangle)]
pub static __stg_EAGER_BLACKHOLE_info: StgInfoTable = unsafe { sys::__stg_EAGER_BLACKHOLE_info };

static stg_WHITEHOLE_info: StgInfoTable = unsafe { sys::stg_WHITEHOLE_info };

static stg_BLOCKING_QUEUE_CLEAN_info: StgInfoTable = unsafe { sys::stg_BLOCKING_QUEUE_CLEAN_info };

static stg_BLOCKING_QUEUE_DIRTY_info: StgInfoTable = unsafe { sys::stg_BLOCKING_QUEUE_DIRTY_info };

#[unsafe(no_mangle)]
pub static stg_BCO_info: StgFunInfoTable = unsafe { sys::stg_BCO_info };

static stg_EVACUATED_info: StgInfoTable = unsafe { sys::stg_EVACUATED_info };

static stg_WEAK_info: StgInfoTable = unsafe { sys::stg_WEAK_info };

static stg_DEAD_WEAK_info: StgInfoTable = unsafe { sys::stg_DEAD_WEAK_info };

static stg_C_FINALIZER_LIST_info: StgInfoTable = unsafe { sys::stg_C_FINALIZER_LIST_info };

static stg_STABLE_NAME_info: StgInfoTable = unsafe { sys::stg_STABLE_NAME_info };

static stg_MVAR_CLEAN_info: StgInfoTable = unsafe { sys::stg_MVAR_CLEAN_info };

static stg_MVAR_DIRTY_info: StgInfoTable = unsafe { sys::stg_MVAR_DIRTY_info };

static stg_TVAR_CLEAN_info: StgInfoTable = unsafe { sys::stg_TVAR_CLEAN_info };

static stg_TVAR_DIRTY_info: StgInfoTable = unsafe { sys::stg_TVAR_DIRTY_info };

static stg_TSO_info: StgInfoTable = unsafe { sys::stg_TSO_info };

#[unsafe(no_mangle)]
pub static stg_STACK_info: StgInfoTable = unsafe { sys::stg_STACK_info };

static stg_RUBBISH_ENTRY_info: StgInfoTable = unsafe { sys::stg_RUBBISH_ENTRY_info };

#[unsafe(no_mangle)]
pub static stg_ARR_WORDS_info: StgInfoTable = unsafe { sys::stg_ARR_WORDS_info };

static stg_MUT_ARR_WORDS_info: StgInfoTable = unsafe { sys::stg_MUT_ARR_WORDS_info };

static stg_MUT_ARR_PTRS_CLEAN_info: StgInfoTable = unsafe { sys::stg_MUT_ARR_PTRS_CLEAN_info };

static stg_MUT_ARR_PTRS_DIRTY_info: StgInfoTable = unsafe { sys::stg_MUT_ARR_PTRS_DIRTY_info };

#[unsafe(no_mangle)]
pub static stg_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable =
    unsafe { sys::stg_MUT_ARR_PTRS_FROZEN_CLEAN_info };

#[unsafe(no_mangle)]
pub static stg_MUT_ARR_PTRS_FROZEN_DIRTY_info: StgInfoTable =
    unsafe { sys::stg_MUT_ARR_PTRS_FROZEN_DIRTY_info };

static stg_SMALL_MUT_ARR_PTRS_CLEAN_info: StgInfoTable =
    unsafe { sys::stg_SMALL_MUT_ARR_PTRS_CLEAN_info };

static stg_SMALL_MUT_ARR_PTRS_DIRTY_info: StgInfoTable =
    unsafe { sys::stg_SMALL_MUT_ARR_PTRS_DIRTY_info };

static stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable =
    unsafe { sys::stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info };

static stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info: StgInfoTable =
    unsafe { sys::stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info };

static stg_MUT_VAR_CLEAN_info: StgInfoTable = unsafe { sys::stg_MUT_VAR_CLEAN_info };

static stg_MUT_VAR_DIRTY_info: StgInfoTable = unsafe { sys::stg_MUT_VAR_DIRTY_info };

static stg_END_TSO_QUEUE_info: StgInfoTable = unsafe { sys::stg_END_TSO_QUEUE_info };

static stg_GCD_CAF_info: StgInfoTable = unsafe { sys::stg_GCD_CAF_info };

static stg_STM_AWOKEN_info: StgInfoTable = unsafe { sys::stg_STM_AWOKEN_info };

static stg_MSG_TRY_WAKEUP_info: StgInfoTable = unsafe { sys::stg_MSG_TRY_WAKEUP_info };

static stg_MSG_THROWTO_info: StgInfoTable = unsafe { sys::stg_MSG_THROWTO_info };

static stg_MSG_BLACKHOLE_info: StgInfoTable = unsafe { sys::stg_MSG_BLACKHOLE_info };

static stg_MSG_CLONE_STACK_info: StgInfoTable = unsafe { sys::stg_MSG_CLONE_STACK_info };

static stg_MSG_NULL_info: StgInfoTable = unsafe { sys::stg_MSG_NULL_info };

static stg_MVAR_TSO_QUEUE_info: StgInfoTable = unsafe { sys::stg_MVAR_TSO_QUEUE_info };

static stg_catch_info: StgInfoTable = unsafe { sys::stg_catch_info };

#[unsafe(no_mangle)]
pub static stg_PAP_info: StgInfoTable = unsafe { sys::stg_PAP_info };

static stg_AP_info: StgInfoTable = unsafe { sys::stg_AP_info };

static stg_AP_NOUPD_info: StgInfoTable = unsafe { sys::stg_AP_NOUPD_info };

static stg_AP_STACK_info: StgInfoTable = unsafe { sys::stg_AP_STACK_info };

static stg_AP_STACK_NOUPD_info: StgInfoTable = unsafe { sys::stg_AP_STACK_NOUPD_info };

static stg_CONTINUATION_info: StgInfoTable = unsafe { sys::stg_CONTINUATION_info };

static stg_PROMPT_TAG_info: StgInfoTable = unsafe { sys::stg_PROMPT_TAG_info };

static stg_dummy_ret_info: StgInfoTable = unsafe { sys::stg_dummy_ret_info };

static stg_raise_info: StgInfoTable = unsafe { sys::stg_raise_info };

static stg_raise_ret_info: StgInfoTable = unsafe { sys::stg_raise_ret_info };

static stg_atomically_info: StgInfoTable = unsafe { sys::stg_atomically_info };

static stg_TVAR_WATCH_QUEUE_info: StgInfoTable = unsafe { sys::stg_TVAR_WATCH_QUEUE_info };

static stg_TREC_CHUNK_info: StgInfoTable = unsafe { sys::stg_TREC_CHUNK_info };

static stg_TREC_HEADER_info: StgInfoTable = unsafe { sys::stg_TREC_HEADER_info };

static stg_END_STM_WATCH_QUEUE_info: StgInfoTable = unsafe { sys::stg_END_STM_WATCH_QUEUE_info };

static stg_END_STM_CHUNK_LIST_info: StgInfoTable = unsafe { sys::stg_END_STM_CHUNK_LIST_info };

static stg_NO_TREC_info: StgInfoTable = unsafe { sys::stg_NO_TREC_info };

static stg_COMPACT_NFDATA_CLEAN_info: StgInfoTable = unsafe { sys::stg_COMPACT_NFDATA_CLEAN_info };

static stg_COMPACT_NFDATA_DIRTY_info: StgInfoTable = unsafe { sys::stg_COMPACT_NFDATA_DIRTY_info };

#[unsafe(no_mangle)]
pub static stg_SRT_1_info: StgInfoTable = unsafe { sys::stg_SRT_1_info };

static stg_SRT_2_info: StgInfoTable = unsafe { sys::stg_SRT_2_info };

static stg_SRT_3_info: StgInfoTable = unsafe { sys::stg_SRT_3_info };

static stg_SRT_4_info: StgInfoTable = unsafe { sys::stg_SRT_4_info };

static stg_SRT_5_info: StgInfoTable = unsafe { sys::stg_SRT_5_info };

static stg_SRT_6_info: StgInfoTable = unsafe { sys::stg_SRT_6_info };

static stg_SRT_7_info: StgInfoTable = unsafe { sys::stg_SRT_7_info };

static stg_SRT_8_info: StgInfoTable = unsafe { sys::stg_SRT_8_info };

static stg_SRT_9_info: StgInfoTable = unsafe { sys::stg_SRT_9_info };

static stg_SRT_10_info: StgInfoTable = unsafe { sys::stg_SRT_10_info };

static stg_SRT_11_info: StgInfoTable = unsafe { sys::stg_SRT_11_info };

static stg_SRT_12_info: StgInfoTable = unsafe { sys::stg_SRT_12_info };

static stg_SRT_13_info: StgInfoTable = unsafe { sys::stg_SRT_13_info };

static stg_SRT_14_info: StgInfoTable = unsafe { sys::stg_SRT_14_info };

static stg_SRT_15_info: StgInfoTable = unsafe { sys::stg_SRT_15_info };

#[unsafe(no_mangle)]
pub static stg_SRT_16_info: StgInfoTable = unsafe { sys::stg_SRT_16_info };

#[unsafe(no_mangle)]
pub static mut stg_END_TSO_QUEUE_closure: StgClosure = unsafe { sys::stg_END_TSO_QUEUE_closure };

static mut stg_STM_AWOKEN_closure: StgClosure = unsafe { sys::stg_STM_AWOKEN_closure };

static mut stg_NO_FINALIZER_closure: StgClosure = unsafe { sys::stg_NO_FINALIZER_closure };

static mut stg_dummy_ret_closure: StgClosure = unsafe { sys::stg_dummy_ret_closure };

static mut stg_forceIO_closure: StgClosure = unsafe { sys::stg_forceIO_closure };

static mut stg_END_STM_WATCH_QUEUE_closure: StgClosure =
    unsafe { sys::stg_END_STM_WATCH_QUEUE_closure };

static mut stg_END_STM_CHUNK_LIST_closure: StgClosure =
    unsafe { sys::stg_END_STM_CHUNK_LIST_closure };

static mut stg_NO_TREC_closure: StgClosure = unsafe { sys::stg_NO_TREC_closure };

static stg_NO_FINALIZER_info: StgInfoTable = unsafe { sys::stg_NO_FINALIZER_info };

static mut stg_CHARLIKE_closure: [StgIntCharlikeClosure; 0usize] =
    unsafe { sys::stg_CHARLIKE_closure };

#[unsafe(no_mangle)]
pub static mut stg_INTLIKE_closure: [StgIntCharlikeClosure; 0usize] =
    unsafe { sys::stg_INTLIKE_closure };

static stg_forceIO_info: StgInfoTable = unsafe { sys::stg_forceIO_info };

static stg_noforceIO_info: StgInfoTable = unsafe { sys::stg_noforceIO_info };

static stg_sel_0_upd_info: StgInfoTable = unsafe { sys::stg_sel_0_upd_info };

static stg_sel_1_upd_info: StgInfoTable = unsafe { sys::stg_sel_1_upd_info };

static stg_sel_2_upd_info: StgInfoTable = unsafe { sys::stg_sel_2_upd_info };

static stg_sel_3_upd_info: StgInfoTable = unsafe { sys::stg_sel_3_upd_info };

static stg_sel_4_upd_info: StgInfoTable = unsafe { sys::stg_sel_4_upd_info };

static stg_sel_5_upd_info: StgInfoTable = unsafe { sys::stg_sel_5_upd_info };

static stg_sel_6_upd_info: StgInfoTable = unsafe { sys::stg_sel_6_upd_info };

static stg_sel_7_upd_info: StgInfoTable = unsafe { sys::stg_sel_7_upd_info };

static stg_sel_8_upd_info: StgInfoTable = unsafe { sys::stg_sel_8_upd_info };

static stg_sel_9_upd_info: StgInfoTable = unsafe { sys::stg_sel_9_upd_info };

static stg_sel_10_upd_info: StgInfoTable = unsafe { sys::stg_sel_10_upd_info };

static stg_sel_11_upd_info: StgInfoTable = unsafe { sys::stg_sel_11_upd_info };

static stg_sel_12_upd_info: StgInfoTable = unsafe { sys::stg_sel_12_upd_info };

static stg_sel_13_upd_info: StgInfoTable = unsafe { sys::stg_sel_13_upd_info };

static stg_sel_14_upd_info: StgInfoTable = unsafe { sys::stg_sel_14_upd_info };

static stg_sel_15_upd_info: StgInfoTable = unsafe { sys::stg_sel_15_upd_info };

static stg_sel_0_noupd_info: StgInfoTable = unsafe { sys::stg_sel_0_noupd_info };

static stg_sel_1_noupd_info: StgInfoTable = unsafe { sys::stg_sel_1_noupd_info };

static stg_sel_2_noupd_info: StgInfoTable = unsafe { sys::stg_sel_2_noupd_info };

static stg_sel_3_noupd_info: StgInfoTable = unsafe { sys::stg_sel_3_noupd_info };

static stg_sel_4_noupd_info: StgInfoTable = unsafe { sys::stg_sel_4_noupd_info };

static stg_sel_5_noupd_info: StgInfoTable = unsafe { sys::stg_sel_5_noupd_info };

static stg_sel_6_noupd_info: StgInfoTable = unsafe { sys::stg_sel_6_noupd_info };

static stg_sel_7_noupd_info: StgInfoTable = unsafe { sys::stg_sel_7_noupd_info };

static stg_sel_8_noupd_info: StgInfoTable = unsafe { sys::stg_sel_8_noupd_info };

static stg_sel_9_noupd_info: StgInfoTable = unsafe { sys::stg_sel_9_noupd_info };

static stg_sel_10_noupd_info: StgInfoTable = unsafe { sys::stg_sel_10_noupd_info };

static stg_sel_11_noupd_info: StgInfoTable = unsafe { sys::stg_sel_11_noupd_info };

static stg_sel_12_noupd_info: StgInfoTable = unsafe { sys::stg_sel_12_noupd_info };

static stg_sel_13_noupd_info: StgInfoTable = unsafe { sys::stg_sel_13_noupd_info };

static stg_sel_14_noupd_info: StgInfoTable = unsafe { sys::stg_sel_14_noupd_info };

static stg_sel_15_noupd_info: StgInfoTable = unsafe { sys::stg_sel_15_noupd_info };

static stg_ap_1_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_1_upd_info };

static stg_ap_2_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_2_upd_info };

static stg_ap_3_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_3_upd_info };

static stg_ap_4_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_4_upd_info };

static stg_ap_5_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_5_upd_info };

static stg_ap_6_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_6_upd_info };

static stg_ap_7_upd_info: StgThunkInfoTable = unsafe { sys::stg_ap_7_upd_info };

#[unsafe(no_mangle)]
pub static stg_unpack_cstring_info: StgInfoTable = unsafe { sys::stg_unpack_cstring_info };

#[unsafe(no_mangle)]
pub static stg_unpack_cstring_utf8_info: StgInfoTable =
    unsafe { sys::stg_unpack_cstring_utf8_info };

#[unsafe(no_mangle)]
pub static stg_ap_v_info: StgInfoTable = unsafe { sys::stg_ap_v_info };

static stg_ap_f_info: StgInfoTable = unsafe { sys::stg_ap_f_info };

static stg_ap_d_info: StgInfoTable = unsafe { sys::stg_ap_d_info };

static stg_ap_l_info: StgInfoTable = unsafe { sys::stg_ap_l_info };

static stg_ap_v16_info: StgInfoTable = unsafe { sys::stg_ap_v16_info };

static stg_ap_v32_info: StgInfoTable = unsafe { sys::stg_ap_v32_info };

static stg_ap_v64_info: StgInfoTable = unsafe { sys::stg_ap_v64_info };

static stg_ap_n_info: StgInfoTable = unsafe { sys::stg_ap_n_info };

static stg_ap_p_info: StgInfoTable = unsafe { sys::stg_ap_p_info };

static stg_ap_pv_info: StgInfoTable = unsafe { sys::stg_ap_pv_info };

#[unsafe(no_mangle)]
pub static stg_ap_pp_info: StgInfoTable = unsafe { sys::stg_ap_pp_info };

static stg_ap_ppv_info: StgInfoTable = unsafe { sys::stg_ap_ppv_info };

static stg_ap_ppp_info: StgInfoTable = unsafe { sys::stg_ap_ppp_info };

static stg_ap_pppv_info: StgInfoTable = unsafe { sys::stg_ap_pppv_info };

static stg_ap_pppp_info: StgInfoTable = unsafe { sys::stg_ap_pppp_info };

static stg_ap_ppppp_info: StgInfoTable = unsafe { sys::stg_ap_ppppp_info };

static stg_ap_pppppp_info: StgInfoTable = unsafe { sys::stg_ap_pppppp_info };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_0_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_0_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_v_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_v_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_f_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_f_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_d_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_d_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_l_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_l_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_v16_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_v16_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_v32_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_v32_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_v64_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_v64_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_n_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_n_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_p_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_p_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_pv_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_pv_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_pp_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_pp_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_ppv_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_ppv_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_ap_ppp_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_ppp_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_pppv_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_pppv_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_pppp_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_pppp_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_ppppp_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_ppppp_fast()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_pppppp_fast() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_pppppp_fast()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_PAP_apply() -> StgFunPtr {
    unsafe { transmute(sys::stg_PAP_apply()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_CONTINUATION_apply() -> StgFunPtr {
    unsafe { transmute(sys::stg_CONTINUATION_apply()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_stk_v16() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_stk_v16()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_stk_v32() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_stk_v32()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_ap_stk_v64() -> StgFunPtr {
    unsafe { transmute(sys::stg_ap_stk_v64()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_stk_save_v16() -> StgFunPtr {
    unsafe { transmute(sys::stg_stk_save_v16()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_stk_save_v32() -> StgFunPtr {
    unsafe { transmute(sys::stg_stk_save_v32()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_stk_save_v64() -> StgFunPtr {
    unsafe { transmute(sys::stg_stk_save_v64()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_noregs() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_noregs()) }
}

static stg_ret_v_info: StgInfoTable = unsafe { sys::stg_ret_v_info };

#[unsafe(no_mangle)]
pub static stg_ret_p_info: StgInfoTable = unsafe { sys::stg_ret_p_info };

#[unsafe(no_mangle)]
pub static stg_ret_n_info: StgInfoTable = unsafe { sys::stg_ret_n_info };

static stg_ret_f_info: StgInfoTable = unsafe { sys::stg_ret_f_info };

static stg_ret_d_info: StgInfoTable = unsafe { sys::stg_ret_d_info };

static stg_ret_l_info: StgInfoTable = unsafe { sys::stg_ret_l_info };

#[unsafe(no_mangle)]
pub static stg_ret_t_info: StgInfoTable = unsafe { sys::stg_ret_t_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim_p() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim_p()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim_pp() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim_pp()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim_n() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim_n()) }
}

static stg_gc_prim_p_ll_ret_info: StgInfoTable = unsafe { sys::stg_gc_prim_p_ll_ret_info };

static stg_gc_prim_pp_ll_ret_info: StgInfoTable = unsafe { sys::stg_gc_prim_pp_ll_ret_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim_p_ll() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim_p_ll()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_gc_prim_pp_ll() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_prim_pp_ll()) }
}

static stg_enter_info: StgInfoTable = unsafe { sys::stg_enter_info };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __stg_gc_enter_1() -> StgFunPtr {
    unsafe { transmute(sys::__stg_gc_enter_1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_unpt_r1() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_unpt_r1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_unbx_r1() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_unbx_r1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_f1() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_f1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_d1() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_d1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_l1() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_l1()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_pp() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_pp()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_ppp() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_ppp()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_gc_pppp() -> StgFunPtr {
    unsafe { transmute(sys::stg_gc_pppp()) }
}

#[unsafe(no_mangle)]
pub static stg_gc_fun_info: StgInfoTable = unsafe { sys::stg_gc_fun_info };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __stg_gc_fun() -> StgFunPtr {
    unsafe { transmute(sys::__stg_gc_fun()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_yield_noregs() -> StgFunPtr {
    unsafe { transmute(sys::stg_yield_noregs()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_yield_to_interpreter() -> StgFunPtr {
    unsafe { transmute(sys::stg_yield_to_interpreter()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_noregs() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_noregs()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_blackhole() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_blackhole()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_blackhole_finally() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_blackhole_finally()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_takemvar() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_takemvar()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_readmvar() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_readmvar()) }
}

static stg_block_takemvar_info: StgInfoTable = unsafe { sys::stg_block_takemvar_info };

static stg_block_readmvar_info: StgInfoTable = unsafe { sys::stg_block_readmvar_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_putmvar() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_putmvar()) }
}

static stg_block_putmvar_info: StgInfoTable = unsafe { sys::stg_block_putmvar_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_stmwait() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_stmwait()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_block_throwto() -> StgFunPtr {
    unsafe { transmute(sys::stg_block_throwto()) }
}

static stg_block_throwto_info: StgInfoTable = unsafe { sys::stg_block_throwto_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_readIOPortzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_readIOPortzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_writeIOPortzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_writeIOPortzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newIOPortzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newIOPortzh()) }
}

#[unsafe(no_mangle)]
pub static stg_stop_thread_info: StgInfoTable = unsafe { sys::stg_stop_thread_info };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_returnToStackTop() -> StgFunPtr {
    unsafe { transmute(sys::stg_returnToStackTop()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_returnToSched() -> StgFunPtr {
    unsafe { transmute(sys::stg_returnToSched()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_returnToSchedNotPaused() -> StgFunPtr {
    unsafe { transmute(sys::stg_returnToSchedNotPaused()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_returnToSchedButFirst() -> StgFunPtr {
    unsafe { transmute(sys::stg_returnToSchedButFirst()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_threadFinished() -> StgFunPtr {
    unsafe { transmute(sys::stg_threadFinished()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn StgReturn() -> StgFunPtr {
    unsafe { transmute(sys::StgReturn()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_decodeFloatzuIntzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_decodeFloatzuIntzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_decodeDoublezu2Intzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_decodeDoublezu2Intzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_decodeDoublezuInt64zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_decodeDoublezuInt64zh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_unsafeThawArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_unsafeThawArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newByteArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newByteArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newPinnedByteArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newPinnedByteArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newAlignedPinnedByteArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newAlignedPinnedByteArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isByteArrayPinnedzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isByteArrayPinnedzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isMutableByteArrayPinnedzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isMutableByteArrayPinnedzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isByteArrayWeaklyPinnedzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isByteArrayWeaklyPinnedzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isMutableByteArrayWeaklyPinnedzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isMutableByteArrayWeaklyPinnedzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_shrinkMutableByteArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_shrinkMutableByteArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_resizzeMutableByteArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_resizzeMutableByteArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_shrinkSmallMutableArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_shrinkSmallMutableArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casIntArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casIntArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casInt8Arrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casInt8Arrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casInt16Arrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casInt16Arrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casInt32Arrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casInt32Arrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casInt64Arrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casInt64Arrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_copyArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_copyArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_copyMutableArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_copyMutableArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_cloneArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_cloneArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_cloneMutableArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_cloneMutableArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_freezzeArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_freezzeArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_thawArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_thawArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_unsafeThawSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_unsafeThawSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_cloneSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_cloneSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_cloneSmallMutableArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_cloneSmallMutableArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_freezzeSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_freezzeSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_thawSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_thawSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_copySmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_copySmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_copySmallMutableArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_copySmallMutableArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casSmallArrayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casSmallArrayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newMutVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newMutVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_atomicModifyMutVar2zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_atomicModifyMutVar2zh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_atomicModifyMutVarzuzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_atomicModifyMutVarzuzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_casMutVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_casMutVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isEmptyMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isEmptyMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_takeMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_takeMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_putMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_putMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_readMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_readMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_tryTakeMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_tryTakeMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_tryPutMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_tryPutMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_tryReadMVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_tryReadMVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_waitReadzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_waitReadzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_waitWritezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_waitWritezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_delayzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_delayzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_catchzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_catchzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_raisezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_raisezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_raiseDivZZerozh() -> StgFunPtr {
    unsafe { transmute(sys::stg_raiseDivZZerozh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_raiseUnderflowzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_raiseUnderflowzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_raiseOverflowzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_raiseOverflowzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_raiseIOzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_raiseIOzh()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_paniczh() -> StgFunPtr {
    unsafe { transmute(sys::stg_paniczh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_keepAlivezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_keepAlivezh()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_absentErrorzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_absentErrorzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newPromptTagzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newPromptTagzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_promptzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_promptzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_control0zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_control0zh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_control0zh_ll() -> StgFunPtr {
    unsafe { transmute(sys::stg_control0zh_ll()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_makeStableNamezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_makeStableNamezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_makeStablePtrzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_makeStablePtrzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_deRefStablePtrzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_deRefStablePtrzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactAddzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactAddzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactAddWithSharingzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactAddWithSharingzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactNewzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactNewzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactAppendzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactAppendzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactResizzezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactResizzezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactGetRootzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactGetRootzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactContainszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactContainszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactContainsAnyzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactContainsAnyzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactGetFirstBlockzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactGetFirstBlockzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactGetNextBlockzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactGetNextBlockzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactAllocateBlockzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactAllocateBlockzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactFixupPointerszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactFixupPointerszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_compactSizzezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_compactSizzezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_forkzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_forkzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_forkOnzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_forkOnzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_yieldzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_yieldzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_killMyself() -> StgFunPtr {
    unsafe { transmute(sys::stg_killMyself()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_killThreadzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_killThreadzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_getMaskingStatezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_getMaskingStatezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_maskAsyncExceptionszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_maskAsyncExceptionszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_maskUninterruptiblezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_maskUninterruptiblezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_unmaskAsyncExceptionszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_unmaskAsyncExceptionszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_myThreadIdzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_myThreadIdzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_labelThreadzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_labelThreadzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_isCurrentThreadBoundzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_isCurrentThreadBoundzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_threadLabelzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_threadLabelzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_threadStatuszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_threadStatuszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_listThreadszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_listThreadszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_mkWeakzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_mkWeakzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_mkWeakNoFinalizzerzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_mkWeakNoFinalizzerzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_mkWeakForeignzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_mkWeakForeignzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_addCFinalizzerToWeakzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_addCFinalizzerToWeakzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_finalizzeWeakzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_finalizzeWeakzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_deRefWeakzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_deRefWeakzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_runRWzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_runRWzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newBCOzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newBCOzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_mkApUpd0zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_mkApUpd0zh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_retryzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_retryzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_catchRetryzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_catchRetryzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_catchSTMzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_catchSTMzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_atomicallyzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_atomicallyzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_newTVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_newTVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_readTVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_readTVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_readTVarIOzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_readTVarIOzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_writeTVarzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_writeTVarzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_unpackClosurezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_unpackClosurezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_closureSizzezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_closureSizzezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_whereFromzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_whereFromzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_getApStackValzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_getApStackValzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_getSparkzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_getSparkzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_numSparkszh() -> StgFunPtr {
    unsafe { transmute(sys::stg_numSparkszh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_noDuplicatezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_noDuplicatezh()) }
}

static stg_noDuplicate_info: StgFunInfoTable = unsafe { sys::stg_noDuplicate_info };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_clearCCSzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_clearCCSzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_traceEventzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_traceEventzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_traceBinaryEventzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_traceBinaryEventzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_traceMarkerzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_traceMarkerzh()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_getThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_getThreadAllocationCounterzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_setThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_setThreadAllocationCounterzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_castWord64ToDoublezh() -> StgFunPtr {
    unsafe { transmute(sys::stg_castWord64ToDoublezh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_castDoubleToWord64zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_castDoubleToWord64zh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_castWord32ToFloatzh() -> StgFunPtr {
    unsafe { transmute(sys::stg_castWord32ToFloatzh()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_castFloatToWord32zh() -> StgFunPtr {
    unsafe { transmute(sys::stg_castFloatToWord32zh()) }
}
