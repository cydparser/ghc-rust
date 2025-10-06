#![allow(unused_imports)]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INVALID_OBJECT() {
    assert_eq!(sys::INVALID_OBJECT, ClosureType::INVALID_OBJECT as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR() {
    assert_eq!(sys::CONSTR, ClosureType::CONSTR as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_1_0() {
    assert_eq!(sys::CONSTR_1_0, ClosureType::CONSTR_1_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_0_1() {
    assert_eq!(sys::CONSTR_0_1, ClosureType::CONSTR_0_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_2_0() {
    assert_eq!(sys::CONSTR_2_0, ClosureType::CONSTR_2_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_1_1() {
    assert_eq!(sys::CONSTR_1_1, ClosureType::CONSTR_1_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_0_2() {
    assert_eq!(sys::CONSTR_0_2, ClosureType::CONSTR_0_2 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONSTR_NOCAF() {
    assert_eq!(sys::CONSTR_NOCAF, ClosureType::CONSTR_NOCAF as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN() {
    assert_eq!(sys::FUN, ClosureType::FUN as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_1_0() {
    assert_eq!(sys::FUN_1_0, ClosureType::FUN_1_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_0_1() {
    assert_eq!(sys::FUN_0_1, ClosureType::FUN_0_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_2_0() {
    assert_eq!(sys::FUN_2_0, ClosureType::FUN_2_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_1_1() {
    assert_eq!(sys::FUN_1_1, ClosureType::FUN_1_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_0_2() {
    assert_eq!(sys::FUN_0_2, ClosureType::FUN_0_2 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_FUN_STATIC() {
    assert_eq!(sys::FUN_STATIC, ClosureType::FUN_STATIC as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK() {
    assert_eq!(sys::THUNK, ClosureType::THUNK as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_1_0() {
    assert_eq!(sys::THUNK_1_0, ClosureType::THUNK_1_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_0_1() {
    assert_eq!(sys::THUNK_0_1, ClosureType::THUNK_0_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_2_0() {
    assert_eq!(sys::THUNK_2_0, ClosureType::THUNK_2_0 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_1_1() {
    assert_eq!(sys::THUNK_1_1, ClosureType::THUNK_1_1 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_0_2() {
    assert_eq!(sys::THUNK_0_2, ClosureType::THUNK_0_2 as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_STATIC() {
    assert_eq!(sys::THUNK_STATIC, ClosureType::THUNK_STATIC as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_THUNK_SELECTOR() {
    assert_eq!(sys::THUNK_SELECTOR, ClosureType::THUNK_SELECTOR as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BCO() {
    assert_eq!(sys::BCO, ClosureType::BCO as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_AP() {
    assert_eq!(sys::AP, ClosureType::AP as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_PAP() {
    assert_eq!(sys::PAP, ClosureType::PAP as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_AP_STACK() {
    assert_eq!(sys::AP_STACK, ClosureType::AP_STACK as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_IND() {
    assert_eq!(sys::IND, ClosureType::IND as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_IND_STATIC() {
    assert_eq!(sys::IND_STATIC, ClosureType::IND_STATIC as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_BCO() {
    assert_eq!(sys::RET_BCO, ClosureType::RET_BCO as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_SMALL() {
    assert_eq!(sys::RET_SMALL, ClosureType::RET_SMALL as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_BIG() {
    assert_eq!(sys::RET_BIG, ClosureType::RET_BIG as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RET_FUN() {
    assert_eq!(sys::RET_FUN, ClosureType::RET_FUN as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_UPDATE_FRAME() {
    assert_eq!(sys::UPDATE_FRAME, ClosureType::UPDATE_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_FRAME() {
    assert_eq!(sys::CATCH_FRAME, ClosureType::CATCH_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_UNDERFLOW_FRAME() {
    assert_eq!(sys::UNDERFLOW_FRAME, ClosureType::UNDERFLOW_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STOP_FRAME() {
    assert_eq!(sys::STOP_FRAME, ClosureType::STOP_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLOCKING_QUEUE() {
    assert_eq!(sys::BLOCKING_QUEUE, ClosureType::BLOCKING_QUEUE as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLACKHOLE() {
    assert_eq!(sys::BLACKHOLE, ClosureType::BLACKHOLE as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MVAR_CLEAN() {
    assert_eq!(sys::MVAR_CLEAN, ClosureType::MVAR_CLEAN as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MVAR_DIRTY() {
    assert_eq!(sys::MVAR_DIRTY, ClosureType::MVAR_DIRTY as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TVAR() {
    assert_eq!(sys::TVAR, ClosureType::TVAR as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ARR_WORDS() {
    assert_eq!(sys::ARR_WORDS, ClosureType::ARR_WORDS as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(
        sys::MUT_ARR_PTRS_CLEAN,
        ClosureType::MUT_ARR_PTRS_CLEAN as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(
        sys::MUT_ARR_PTRS_DIRTY,
        ClosureType::MUT_ARR_PTRS_DIRTY as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(
        sys::MUT_ARR_PTRS_FROZEN_DIRTY,
        ClosureType::MUT_ARR_PTRS_FROZEN_DIRTY as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(
        sys::MUT_ARR_PTRS_FROZEN_CLEAN,
        ClosureType::MUT_ARR_PTRS_FROZEN_CLEAN as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_VAR_CLEAN() {
    assert_eq!(sys::MUT_VAR_CLEAN, ClosureType::MUT_VAR_CLEAN as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_VAR_DIRTY() {
    assert_eq!(sys::MUT_VAR_DIRTY, ClosureType::MUT_VAR_DIRTY as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WEAK() {
    assert_eq!(sys::WEAK, ClosureType::WEAK as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_PRIM() {
    assert_eq!(sys::PRIM, ClosureType::PRIM as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_PRIM() {
    assert_eq!(sys::MUT_PRIM, ClosureType::MUT_PRIM as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO() {
    assert_eq!(sys::TSO, ClosureType::TSO as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STACK() {
    assert_eq!(sys::STACK, ClosureType::STACK as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TREC_CHUNK() {
    assert_eq!(sys::TREC_CHUNK, ClosureType::TREC_CHUNK as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ATOMICALLY_FRAME() {
    assert_eq!(sys::ATOMICALLY_FRAME, ClosureType::ATOMICALLY_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_RETRY_FRAME() {
    assert_eq!(
        sys::CATCH_RETRY_FRAME,
        ClosureType::CATCH_RETRY_FRAME as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CATCH_STM_FRAME() {
    assert_eq!(sys::CATCH_STM_FRAME, ClosureType::CATCH_STM_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WHITEHOLE() {
    assert_eq!(sys::WHITEHOLE, ClosureType::WHITEHOLE as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_CLEAN() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_CLEAN,
        ClosureType::SMALL_MUT_ARR_PTRS_CLEAN as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_DIRTY() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_DIRTY,
        ClosureType::SMALL_MUT_ARR_PTRS_DIRTY as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY,
        ClosureType::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN() {
    assert_eq!(
        sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
        ClosureType::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN as u32
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COMPACT_NFDATA() {
    assert_eq!(sys::COMPACT_NFDATA, ClosureType::COMPACT_NFDATA as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CONTINUATION() {
    assert_eq!(sys::CONTINUATION, ClosureType::CONTINUATION as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ANN_FRAME() {
    assert_eq!(sys::ANN_FRAME, ClosureType::ANN_FRAME as u32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_N_CLOSURE_TYPES() {
    assert_eq!(sys::N_CLOSURE_TYPES, ClosureType::N_CLOSURE_TYPES as u32);
}
