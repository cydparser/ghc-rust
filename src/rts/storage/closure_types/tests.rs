use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_INVALID_OBJECT() {
    assert_eq!(sys::INVALID_OBJECT, super::INVALID_OBJECT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR() {
    assert_eq!(sys::CONSTR, super::CONSTR);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_1_0() {
    assert_eq!(sys::CONSTR_1_0, super::CONSTR_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_0_1() {
    assert_eq!(sys::CONSTR_0_1, super::CONSTR_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_2_0() {
    assert_eq!(sys::CONSTR_2_0, super::CONSTR_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_1_1() {
    assert_eq!(sys::CONSTR_1_1, super::CONSTR_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_0_2() {
    assert_eq!(sys::CONSTR_0_2, super::CONSTR_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONSTR_NOCAF() {
    assert_eq!(sys::CONSTR_NOCAF, super::CONSTR_NOCAF);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN() {
    assert_eq!(sys::FUN, super::FUN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_1_0() {
    assert_eq!(sys::FUN_1_0, super::FUN_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_0_1() {
    assert_eq!(sys::FUN_0_1, super::FUN_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_2_0() {
    assert_eq!(sys::FUN_2_0, super::FUN_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_1_1() {
    assert_eq!(sys::FUN_1_1, super::FUN_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_0_2() {
    assert_eq!(sys::FUN_0_2, super::FUN_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FUN_STATIC() {
    assert_eq!(sys::FUN_STATIC, super::FUN_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK() {
    assert_eq!(sys::THUNK, super::THUNK);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_1_0() {
    assert_eq!(sys::THUNK_1_0, super::THUNK_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_0_1() {
    assert_eq!(sys::THUNK_0_1, super::THUNK_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_2_0() {
    assert_eq!(sys::THUNK_2_0, super::THUNK_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_1_1() {
    assert_eq!(sys::THUNK_1_1, super::THUNK_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_0_2() {
    assert_eq!(sys::THUNK_0_2, super::THUNK_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_STATIC() {
    assert_eq!(sys::THUNK_STATIC, super::THUNK_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_THUNK_SELECTOR() {
    assert_eq!(sys::THUNK_SELECTOR, super::THUNK_SELECTOR);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BCO() {
    assert_eq!(sys::BCO, super::BCO);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_AP() {
    assert_eq!(sys::AP, super::AP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_PAP() {
    assert_eq!(sys::PAP, super::PAP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_AP_STACK() {
    assert_eq!(sys::AP_STACK, super::AP_STACK);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_IND() {
    assert_eq!(sys::IND, super::IND);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_IND_STATIC() {
    assert_eq!(sys::IND_STATIC, super::IND_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RET_BCO() {
    assert_eq!(sys::RET_BCO, super::RET_BCO);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RET_SMALL() {
    assert_eq!(sys::RET_SMALL, super::RET_SMALL);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RET_BIG() {
    assert_eq!(sys::RET_BIG, super::RET_BIG);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RET_FUN() {
    assert_eq!(sys::RET_FUN, super::RET_FUN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_UPDATE_FRAME() {
    assert_eq!(sys::UPDATE_FRAME, super::UPDATE_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CATCH_FRAME() {
    assert_eq!(sys::CATCH_FRAME, super::CATCH_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_UNDERFLOW_FRAME() {
    assert_eq!(sys::UNDERFLOW_FRAME, super::UNDERFLOW_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STOP_FRAME() {
    assert_eq!(sys::STOP_FRAME, super::STOP_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BLOCKING_QUEUE() {
    assert_eq!(sys::BLOCKING_QUEUE, super::BLOCKING_QUEUE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BLACKHOLE() {
    assert_eq!(sys::BLACKHOLE, super::BLACKHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MVAR_CLEAN() {
    assert_eq!(sys::MVAR_CLEAN, super::MVAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MVAR_DIRTY() {
    assert_eq!(sys::MVAR_DIRTY, super::MVAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TVAR() {
    assert_eq!(sys::TVAR, super::TVAR);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARR_WORDS() {
    assert_eq!(sys::ARR_WORDS, super::ARR_WORDS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(sys::MUT_ARR_PTRS_CLEAN, super::MUT_ARR_PTRS_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(sys::MUT_ARR_PTRS_DIRTY, super::MUT_ARR_PTRS_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(
        sys::MUT_ARR_PTRS_FROZEN_DIRTY,
        super::MUT_ARR_PTRS_FROZEN_DIRTY
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(
        sys::MUT_ARR_PTRS_FROZEN_CLEAN,
        super::MUT_ARR_PTRS_FROZEN_CLEAN
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_VAR_CLEAN() {
    assert_eq!(sys::MUT_VAR_CLEAN, super::MUT_VAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_VAR_DIRTY() {
    assert_eq!(sys::MUT_VAR_DIRTY, super::MUT_VAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_WEAK() {
    assert_eq!(sys::WEAK, super::WEAK);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_PRIM() {
    assert_eq!(sys::PRIM, super::PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_PRIM() {
    assert_eq!(sys::MUT_PRIM, super::MUT_PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO() {
    assert_eq!(sys::TSO, super::TSO);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STACK() {
    assert_eq!(sys::STACK, super::STACK);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TREC_CHUNK() {
    assert_eq!(sys::TREC_CHUNK, super::TREC_CHUNK);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ATOMICALLY_FRAME() {
    assert_eq!(sys::ATOMICALLY_FRAME, super::ATOMICALLY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CATCH_RETRY_FRAME() {
    assert_eq!(sys::CATCH_RETRY_FRAME, super::CATCH_RETRY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CATCH_STM_FRAME() {
    assert_eq!(sys::CATCH_STM_FRAME, super::CATCH_STM_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_WHITEHOLE() {
    assert_eq!(sys::WHITEHOLE, super::WHITEHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SMALL_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_CLEAN,
        super::SMALL_MUT_ARR_PTRS_CLEAN
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SMALL_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_DIRTY,
        super::SMALL_MUT_ARR_PTRS_DIRTY
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY,
        super::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
        super::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COMPACT_NFDATA() {
    assert_eq!(sys::COMPACT_NFDATA, super::COMPACT_NFDATA);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CONTINUATION() {
    assert_eq!(sys::CONTINUATION, super::CONTINUATION);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_N_CLOSURE_TYPES() {
    assert_eq!(sys::N_CLOSURE_TYPES, super::N_CLOSURE_TYPES);
}
