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

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_upd_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_upd_frame_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_catch_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_frame_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_catch_retry_frame_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_retry_frame_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_atomically_frame_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_atomically_frame_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_catch_stm_frame_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_catch_stm_frame_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_stack_underflow_frame_d_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_stack_underflow_frame_d_info: StgInfoTable = TODO_StgInfoTable;

const TODO_StgFunInfoTable: StgFunInfoTable = StgFunInfoTable {
    f: StgFunInfoExtraRev_ {
        slow_apply_offset: 0,
        b: StgFunInfoExtraRev__anon_union_1 { bitmap: 0 },
        fun_type: 0,
        arity: 0,
    },
    i: TODO_StgInfoTable,
};

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_BCO_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_BCO_info: StgFunInfoTable = TODO_StgFunInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_STACK_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_STACK_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ARR_WORDS_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ARR_WORDS_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_MUT_ARR_PTRS_FROZEN_CLEAN_info")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_MUT_ARR_PTRS_FROZEN_CLEAN_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ret_p_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ret_p_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_ret_n_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_ret_n_info: StgInfoTable = TODO_StgInfoTable;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_stop_thread_info"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static stg_SRT_1_info: StgInfoTable = TODO_StgInfoTable;
pub static stg_stop_thread_info: StgInfoTable = 0;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_paniczh"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_paniczh() -> StgFunPtr {
    unsafe { sys::stg_paniczh() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_absentErrorzh"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_absentErrorzh() -> StgFunPtr {
    unsafe { sys::stg_absentErrorzh() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_getThreadAllocationCounterzh")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_getThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { sys::stg_getThreadAllocationCounterzh() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_stg_getOtherThreadAllocationCounterzh")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn stg_getOtherThreadAllocationCounterzh() -> StgFunPtr {
    unsafe { sys::stg_getOtherThreadAllocationCounterzh() }
}
