use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INVALID_OBJECT() {
    assert_eq!(sys::INVALID_OBJECT, INVALID_OBJECT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR() {
    assert_eq!(sys::CONSTR, CONSTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_1_0() {
    assert_eq!(sys::CONSTR_1_0, CONSTR_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_0_1() {
    assert_eq!(sys::CONSTR_0_1, CONSTR_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_2_0() {
    assert_eq!(sys::CONSTR_2_0, CONSTR_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_1_1() {
    assert_eq!(sys::CONSTR_1_1, CONSTR_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_0_2() {
    assert_eq!(sys::CONSTR_0_2, CONSTR_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_NOCAF() {
    assert_eq!(sys::CONSTR_NOCAF, CONSTR_NOCAF);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN() {
    assert_eq!(sys::FUN, FUN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_1_0() {
    assert_eq!(sys::FUN_1_0, FUN_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_0_1() {
    assert_eq!(sys::FUN_0_1, FUN_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_2_0() {
    assert_eq!(sys::FUN_2_0, FUN_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_1_1() {
    assert_eq!(sys::FUN_1_1, FUN_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_0_2() {
    assert_eq!(sys::FUN_0_2, FUN_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_STATIC() {
    assert_eq!(sys::FUN_STATIC, FUN_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK() {
    assert_eq!(sys::THUNK, THUNK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_1_0() {
    assert_eq!(sys::THUNK_1_0, THUNK_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_0_1() {
    assert_eq!(sys::THUNK_0_1, THUNK_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_2_0() {
    assert_eq!(sys::THUNK_2_0, THUNK_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_1_1() {
    assert_eq!(sys::THUNK_1_1, THUNK_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_0_2() {
    assert_eq!(sys::THUNK_0_2, THUNK_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_STATIC() {
    assert_eq!(sys::THUNK_STATIC, THUNK_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_SELECTOR() {
    assert_eq!(sys::THUNK_SELECTOR, THUNK_SELECTOR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BCO() {
    assert_eq!(sys::BCO, BCO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_AP() {
    assert_eq!(sys::AP, AP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_PAP() {
    assert_eq!(sys::PAP, PAP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_AP_STACK() {
    assert_eq!(sys::AP_STACK, AP_STACK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_IND() {
    assert_eq!(sys::IND, IND);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_IND_STATIC() {
    assert_eq!(sys::IND_STATIC, IND_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_BCO() {
    assert_eq!(sys::RET_BCO, RET_BCO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_SMALL() {
    assert_eq!(sys::RET_SMALL, RET_SMALL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_BIG() {
    assert_eq!(sys::RET_BIG, RET_BIG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_FUN() {
    assert_eq!(sys::RET_FUN, RET_FUN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_UPDATE_FRAME() {
    assert_eq!(sys::UPDATE_FRAME, UPDATE_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_FRAME() {
    assert_eq!(sys::CATCH_FRAME, CATCH_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_UNDERFLOW_FRAME() {
    assert_eq!(sys::UNDERFLOW_FRAME, UNDERFLOW_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STOP_FRAME() {
    assert_eq!(sys::STOP_FRAME, STOP_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLOCKING_QUEUE() {
    assert_eq!(sys::BLOCKING_QUEUE, BLOCKING_QUEUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLACKHOLE() {
    assert_eq!(sys::BLACKHOLE, BLACKHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MVAR_CLEAN() {
    assert_eq!(sys::MVAR_CLEAN, MVAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MVAR_DIRTY() {
    assert_eq!(sys::MVAR_DIRTY, MVAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TVAR() {
    assert_eq!(sys::TVAR, TVAR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ARR_WORDS() {
    assert_eq!(sys::ARR_WORDS, ARR_WORDS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(sys::MUT_ARR_PTRS_CLEAN, MUT_ARR_PTRS_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(sys::MUT_ARR_PTRS_DIRTY, MUT_ARR_PTRS_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(sys::MUT_ARR_PTRS_FROZEN_DIRTY, MUT_ARR_PTRS_FROZEN_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(sys::MUT_ARR_PTRS_FROZEN_CLEAN, MUT_ARR_PTRS_FROZEN_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_VAR_CLEAN() {
    assert_eq!(sys::MUT_VAR_CLEAN, MUT_VAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_VAR_DIRTY() {
    assert_eq!(sys::MUT_VAR_DIRTY, MUT_VAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WEAK() {
    assert_eq!(sys::WEAK, WEAK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_PRIM() {
    assert_eq!(sys::PRIM, PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_PRIM() {
    assert_eq!(sys::MUT_PRIM, MUT_PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO() {
    assert_eq!(sys::TSO, TSO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STACK() {
    assert_eq!(sys::STACK, STACK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TREC_CHUNK() {
    assert_eq!(sys::TREC_CHUNK, TREC_CHUNK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ATOMICALLY_FRAME() {
    assert_eq!(sys::ATOMICALLY_FRAME, ATOMICALLY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_RETRY_FRAME() {
    assert_eq!(sys::CATCH_RETRY_FRAME, CATCH_RETRY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_STM_FRAME() {
    assert_eq!(sys::CATCH_STM_FRAME, CATCH_STM_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WHITEHOLE() {
    assert_eq!(sys::WHITEHOLE, WHITEHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(sys::SMALL_MUT_ARR_PTRS_CLEAN, SMALL_MUT_ARR_PTRS_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(sys::SMALL_MUT_ARR_PTRS_DIRTY, SMALL_MUT_ARR_PTRS_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY,
        SMALL_MUT_ARR_PTRS_FROZEN_DIRTY
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
        SMALL_MUT_ARR_PTRS_FROZEN_CLEAN
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COMPACT_NFDATA() {
    assert_eq!(sys::COMPACT_NFDATA, COMPACT_NFDATA);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONTINUATION() {
    assert_eq!(sys::CONTINUATION, CONTINUATION);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_N_CLOSURE_TYPES() {
    assert_eq!(sys::N_CLOSURE_TYPES, N_CLOSURE_TYPES);
}
