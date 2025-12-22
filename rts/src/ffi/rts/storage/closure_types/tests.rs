#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_INVALID_OBJECT_eq() {
    assert_eq!(INVALID_OBJECT, sys::INVALID_OBJECT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_INVALID_OBJECT_layout() {
    assert_eq!(
        size_of_val(&INVALID_OBJECT),
        size_of_val(&sys::INVALID_OBJECT)
    );
    assert_eq!(
        align_of_val(&INVALID_OBJECT),
        align_of_val(&sys::INVALID_OBJECT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_eq() {
    assert_eq!(CONSTR, sys::CONSTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_layout() {
    assert_eq!(size_of_val(&CONSTR), size_of_val(&sys::CONSTR));
    assert_eq!(align_of_val(&CONSTR), align_of_val(&sys::CONSTR));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_1_0_eq() {
    assert_eq!(CONSTR_1_0, sys::CONSTR_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_1_0_layout() {
    assert_eq!(size_of_val(&CONSTR_1_0), size_of_val(&sys::CONSTR_1_0));
    assert_eq!(align_of_val(&CONSTR_1_0), align_of_val(&sys::CONSTR_1_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_0_1_eq() {
    assert_eq!(CONSTR_0_1, sys::CONSTR_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_0_1_layout() {
    assert_eq!(size_of_val(&CONSTR_0_1), size_of_val(&sys::CONSTR_0_1));
    assert_eq!(align_of_val(&CONSTR_0_1), align_of_val(&sys::CONSTR_0_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_2_0_eq() {
    assert_eq!(CONSTR_2_0, sys::CONSTR_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_2_0_layout() {
    assert_eq!(size_of_val(&CONSTR_2_0), size_of_val(&sys::CONSTR_2_0));
    assert_eq!(align_of_val(&CONSTR_2_0), align_of_val(&sys::CONSTR_2_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_1_1_eq() {
    assert_eq!(CONSTR_1_1, sys::CONSTR_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_1_1_layout() {
    assert_eq!(size_of_val(&CONSTR_1_1), size_of_val(&sys::CONSTR_1_1));
    assert_eq!(align_of_val(&CONSTR_1_1), align_of_val(&sys::CONSTR_1_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_0_2_eq() {
    assert_eq!(CONSTR_0_2, sys::CONSTR_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_0_2_layout() {
    assert_eq!(size_of_val(&CONSTR_0_2), size_of_val(&sys::CONSTR_0_2));
    assert_eq!(align_of_val(&CONSTR_0_2), align_of_val(&sys::CONSTR_0_2));
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_NOCAF_eq() {
    assert_eq!(CONSTR_NOCAF, sys::CONSTR_NOCAF);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONSTR_NOCAF_layout() {
    assert_eq!(size_of_val(&CONSTR_NOCAF), size_of_val(&sys::CONSTR_NOCAF));
    assert_eq!(
        align_of_val(&CONSTR_NOCAF),
        align_of_val(&sys::CONSTR_NOCAF)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_eq() {
    assert_eq!(FUN, sys::FUN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_layout() {
    assert_eq!(size_of_val(&FUN), size_of_val(&sys::FUN));
    assert_eq!(align_of_val(&FUN), align_of_val(&sys::FUN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_1_0_eq() {
    assert_eq!(FUN_1_0, sys::FUN_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_1_0_layout() {
    assert_eq!(size_of_val(&FUN_1_0), size_of_val(&sys::FUN_1_0));
    assert_eq!(align_of_val(&FUN_1_0), align_of_val(&sys::FUN_1_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_0_1_eq() {
    assert_eq!(FUN_0_1, sys::FUN_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_0_1_layout() {
    assert_eq!(size_of_val(&FUN_0_1), size_of_val(&sys::FUN_0_1));
    assert_eq!(align_of_val(&FUN_0_1), align_of_val(&sys::FUN_0_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_2_0_eq() {
    assert_eq!(FUN_2_0, sys::FUN_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_2_0_layout() {
    assert_eq!(size_of_val(&FUN_2_0), size_of_val(&sys::FUN_2_0));
    assert_eq!(align_of_val(&FUN_2_0), align_of_val(&sys::FUN_2_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_1_1_eq() {
    assert_eq!(FUN_1_1, sys::FUN_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_1_1_layout() {
    assert_eq!(size_of_val(&FUN_1_1), size_of_val(&sys::FUN_1_1));
    assert_eq!(align_of_val(&FUN_1_1), align_of_val(&sys::FUN_1_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_0_2_eq() {
    assert_eq!(FUN_0_2, sys::FUN_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_0_2_layout() {
    assert_eq!(size_of_val(&FUN_0_2), size_of_val(&sys::FUN_0_2));
    assert_eq!(align_of_val(&FUN_0_2), align_of_val(&sys::FUN_0_2));
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_STATIC_eq() {
    assert_eq!(FUN_STATIC, sys::FUN_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_FUN_STATIC_layout() {
    assert_eq!(size_of_val(&FUN_STATIC), size_of_val(&sys::FUN_STATIC));
    assert_eq!(align_of_val(&FUN_STATIC), align_of_val(&sys::FUN_STATIC));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_eq() {
    assert_eq!(THUNK, sys::THUNK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_layout() {
    assert_eq!(size_of_val(&THUNK), size_of_val(&sys::THUNK));
    assert_eq!(align_of_val(&THUNK), align_of_val(&sys::THUNK));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_1_0_eq() {
    assert_eq!(THUNK_1_0, sys::THUNK_1_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_1_0_layout() {
    assert_eq!(size_of_val(&THUNK_1_0), size_of_val(&sys::THUNK_1_0));
    assert_eq!(align_of_val(&THUNK_1_0), align_of_val(&sys::THUNK_1_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_0_1_eq() {
    assert_eq!(THUNK_0_1, sys::THUNK_0_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_0_1_layout() {
    assert_eq!(size_of_val(&THUNK_0_1), size_of_val(&sys::THUNK_0_1));
    assert_eq!(align_of_val(&THUNK_0_1), align_of_val(&sys::THUNK_0_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_2_0_eq() {
    assert_eq!(THUNK_2_0, sys::THUNK_2_0);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_2_0_layout() {
    assert_eq!(size_of_val(&THUNK_2_0), size_of_val(&sys::THUNK_2_0));
    assert_eq!(align_of_val(&THUNK_2_0), align_of_val(&sys::THUNK_2_0));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_1_1_eq() {
    assert_eq!(THUNK_1_1, sys::THUNK_1_1);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_1_1_layout() {
    assert_eq!(size_of_val(&THUNK_1_1), size_of_val(&sys::THUNK_1_1));
    assert_eq!(align_of_val(&THUNK_1_1), align_of_val(&sys::THUNK_1_1));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_0_2_eq() {
    assert_eq!(THUNK_0_2, sys::THUNK_0_2);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_0_2_layout() {
    assert_eq!(size_of_val(&THUNK_0_2), size_of_val(&sys::THUNK_0_2));
    assert_eq!(align_of_val(&THUNK_0_2), align_of_val(&sys::THUNK_0_2));
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_STATIC_eq() {
    assert_eq!(THUNK_STATIC, sys::THUNK_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_STATIC_layout() {
    assert_eq!(size_of_val(&THUNK_STATIC), size_of_val(&sys::THUNK_STATIC));
    assert_eq!(
        align_of_val(&THUNK_STATIC),
        align_of_val(&sys::THUNK_STATIC)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_SELECTOR_eq() {
    assert_eq!(THUNK_SELECTOR, sys::THUNK_SELECTOR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_THUNK_SELECTOR_layout() {
    assert_eq!(
        size_of_val(&THUNK_SELECTOR),
        size_of_val(&sys::THUNK_SELECTOR)
    );
    assert_eq!(
        align_of_val(&THUNK_SELECTOR),
        align_of_val(&sys::THUNK_SELECTOR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BCO_eq() {
    assert_eq!(BCO, sys::BCO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BCO_layout() {
    assert_eq!(size_of_val(&BCO), size_of_val(&sys::BCO));
    assert_eq!(align_of_val(&BCO), align_of_val(&sys::BCO));
}

#[cfg(feature = "sys")]
#[test]
fn sys_AP_eq() {
    assert_eq!(AP, sys::AP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_AP_layout() {
    assert_eq!(size_of_val(&AP), size_of_val(&sys::AP));
    assert_eq!(align_of_val(&AP), align_of_val(&sys::AP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_PAP_eq() {
    assert_eq!(PAP, sys::PAP);
}

#[cfg(feature = "sys")]
#[test]
fn sys_PAP_layout() {
    assert_eq!(size_of_val(&PAP), size_of_val(&sys::PAP));
    assert_eq!(align_of_val(&PAP), align_of_val(&sys::PAP));
}

#[cfg(feature = "sys")]
#[test]
fn sys_AP_STACK_eq() {
    assert_eq!(AP_STACK, sys::AP_STACK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_AP_STACK_layout() {
    assert_eq!(size_of_val(&AP_STACK), size_of_val(&sys::AP_STACK));
    assert_eq!(align_of_val(&AP_STACK), align_of_val(&sys::AP_STACK));
}

#[cfg(feature = "sys")]
#[test]
fn sys_IND_eq() {
    assert_eq!(IND, sys::IND);
}

#[cfg(feature = "sys")]
#[test]
fn sys_IND_layout() {
    assert_eq!(size_of_val(&IND), size_of_val(&sys::IND));
    assert_eq!(align_of_val(&IND), align_of_val(&sys::IND));
}

#[cfg(feature = "sys")]
#[test]
fn sys_IND_STATIC_eq() {
    assert_eq!(IND_STATIC, sys::IND_STATIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_IND_STATIC_layout() {
    assert_eq!(size_of_val(&IND_STATIC), size_of_val(&sys::IND_STATIC));
    assert_eq!(align_of_val(&IND_STATIC), align_of_val(&sys::IND_STATIC));
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_BCO_eq() {
    assert_eq!(RET_BCO, sys::RET_BCO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_BCO_layout() {
    assert_eq!(size_of_val(&RET_BCO), size_of_val(&sys::RET_BCO));
    assert_eq!(align_of_val(&RET_BCO), align_of_val(&sys::RET_BCO));
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_SMALL_eq() {
    assert_eq!(RET_SMALL, sys::RET_SMALL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_SMALL_layout() {
    assert_eq!(size_of_val(&RET_SMALL), size_of_val(&sys::RET_SMALL));
    assert_eq!(align_of_val(&RET_SMALL), align_of_val(&sys::RET_SMALL));
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_BIG_eq() {
    assert_eq!(RET_BIG, sys::RET_BIG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_BIG_layout() {
    assert_eq!(size_of_val(&RET_BIG), size_of_val(&sys::RET_BIG));
    assert_eq!(align_of_val(&RET_BIG), align_of_val(&sys::RET_BIG));
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_FUN_eq() {
    assert_eq!(RET_FUN, sys::RET_FUN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_RET_FUN_layout() {
    assert_eq!(size_of_val(&RET_FUN), size_of_val(&sys::RET_FUN));
    assert_eq!(align_of_val(&RET_FUN), align_of_val(&sys::RET_FUN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_UPDATE_FRAME_eq() {
    assert_eq!(UPDATE_FRAME, sys::UPDATE_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_UPDATE_FRAME_layout() {
    assert_eq!(size_of_val(&UPDATE_FRAME), size_of_val(&sys::UPDATE_FRAME));
    assert_eq!(
        align_of_val(&UPDATE_FRAME),
        align_of_val(&sys::UPDATE_FRAME)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_FRAME_eq() {
    assert_eq!(CATCH_FRAME, sys::CATCH_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_FRAME_layout() {
    assert_eq!(size_of_val(&CATCH_FRAME), size_of_val(&sys::CATCH_FRAME));
    assert_eq!(align_of_val(&CATCH_FRAME), align_of_val(&sys::CATCH_FRAME));
}

#[cfg(feature = "sys")]
#[test]
fn sys_UNDERFLOW_FRAME_eq() {
    assert_eq!(UNDERFLOW_FRAME, sys::UNDERFLOW_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_UNDERFLOW_FRAME_layout() {
    assert_eq!(
        size_of_val(&UNDERFLOW_FRAME),
        size_of_val(&sys::UNDERFLOW_FRAME)
    );
    assert_eq!(
        align_of_val(&UNDERFLOW_FRAME),
        align_of_val(&sys::UNDERFLOW_FRAME)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_STOP_FRAME_eq() {
    assert_eq!(STOP_FRAME, sys::STOP_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STOP_FRAME_layout() {
    assert_eq!(size_of_val(&STOP_FRAME), size_of_val(&sys::STOP_FRAME));
    assert_eq!(align_of_val(&STOP_FRAME), align_of_val(&sys::STOP_FRAME));
}

#[cfg(feature = "sys")]
#[test]
fn sys_BLOCKING_QUEUE_eq() {
    assert_eq!(BLOCKING_QUEUE, sys::BLOCKING_QUEUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BLOCKING_QUEUE_layout() {
    assert_eq!(
        size_of_val(&BLOCKING_QUEUE),
        size_of_val(&sys::BLOCKING_QUEUE)
    );
    assert_eq!(
        align_of_val(&BLOCKING_QUEUE),
        align_of_val(&sys::BLOCKING_QUEUE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BLACKHOLE_eq() {
    assert_eq!(BLACKHOLE, sys::BLACKHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BLACKHOLE_layout() {
    assert_eq!(size_of_val(&BLACKHOLE), size_of_val(&sys::BLACKHOLE));
    assert_eq!(align_of_val(&BLACKHOLE), align_of_val(&sys::BLACKHOLE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_MVAR_CLEAN_eq() {
    assert_eq!(MVAR_CLEAN, sys::MVAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MVAR_CLEAN_layout() {
    assert_eq!(size_of_val(&MVAR_CLEAN), size_of_val(&sys::MVAR_CLEAN));
    assert_eq!(align_of_val(&MVAR_CLEAN), align_of_val(&sys::MVAR_CLEAN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_MVAR_DIRTY_eq() {
    assert_eq!(MVAR_DIRTY, sys::MVAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MVAR_DIRTY_layout() {
    assert_eq!(size_of_val(&MVAR_DIRTY), size_of_val(&sys::MVAR_DIRTY));
    assert_eq!(align_of_val(&MVAR_DIRTY), align_of_val(&sys::MVAR_DIRTY));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TVAR_eq() {
    assert_eq!(TVAR, sys::TVAR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TVAR_layout() {
    assert_eq!(size_of_val(&TVAR), size_of_val(&sys::TVAR));
    assert_eq!(align_of_val(&TVAR), align_of_val(&sys::TVAR));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARR_WORDS_eq() {
    assert_eq!(ARR_WORDS, sys::ARR_WORDS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ARR_WORDS_layout() {
    assert_eq!(size_of_val(&ARR_WORDS), size_of_val(&sys::ARR_WORDS));
    assert_eq!(align_of_val(&ARR_WORDS), align_of_val(&sys::ARR_WORDS));
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_CLEAN_eq() {
    assert_eq!(MUT_ARR_PTRS_CLEAN, sys::MUT_ARR_PTRS_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_CLEAN_layout() {
    assert_eq!(
        size_of_val(&MUT_ARR_PTRS_CLEAN),
        size_of_val(&sys::MUT_ARR_PTRS_CLEAN)
    );
    assert_eq!(
        align_of_val(&MUT_ARR_PTRS_CLEAN),
        align_of_val(&sys::MUT_ARR_PTRS_CLEAN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_DIRTY_eq() {
    assert_eq!(MUT_ARR_PTRS_DIRTY, sys::MUT_ARR_PTRS_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_DIRTY_layout() {
    assert_eq!(
        size_of_val(&MUT_ARR_PTRS_DIRTY),
        size_of_val(&sys::MUT_ARR_PTRS_DIRTY)
    );
    assert_eq!(
        align_of_val(&MUT_ARR_PTRS_DIRTY),
        align_of_val(&sys::MUT_ARR_PTRS_DIRTY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_FROZEN_DIRTY_eq() {
    assert_eq!(MUT_ARR_PTRS_FROZEN_DIRTY, sys::MUT_ARR_PTRS_FROZEN_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_FROZEN_DIRTY_layout() {
    assert_eq!(
        size_of_val(&MUT_ARR_PTRS_FROZEN_DIRTY),
        size_of_val(&sys::MUT_ARR_PTRS_FROZEN_DIRTY)
    );
    assert_eq!(
        align_of_val(&MUT_ARR_PTRS_FROZEN_DIRTY),
        align_of_val(&sys::MUT_ARR_PTRS_FROZEN_DIRTY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_FROZEN_CLEAN_eq() {
    assert_eq!(MUT_ARR_PTRS_FROZEN_CLEAN, sys::MUT_ARR_PTRS_FROZEN_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_ARR_PTRS_FROZEN_CLEAN_layout() {
    assert_eq!(
        size_of_val(&MUT_ARR_PTRS_FROZEN_CLEAN),
        size_of_val(&sys::MUT_ARR_PTRS_FROZEN_CLEAN)
    );
    assert_eq!(
        align_of_val(&MUT_ARR_PTRS_FROZEN_CLEAN),
        align_of_val(&sys::MUT_ARR_PTRS_FROZEN_CLEAN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_VAR_CLEAN_eq() {
    assert_eq!(MUT_VAR_CLEAN, sys::MUT_VAR_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_VAR_CLEAN_layout() {
    assert_eq!(
        size_of_val(&MUT_VAR_CLEAN),
        size_of_val(&sys::MUT_VAR_CLEAN)
    );
    assert_eq!(
        align_of_val(&MUT_VAR_CLEAN),
        align_of_val(&sys::MUT_VAR_CLEAN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_VAR_DIRTY_eq() {
    assert_eq!(MUT_VAR_DIRTY, sys::MUT_VAR_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_VAR_DIRTY_layout() {
    assert_eq!(
        size_of_val(&MUT_VAR_DIRTY),
        size_of_val(&sys::MUT_VAR_DIRTY)
    );
    assert_eq!(
        align_of_val(&MUT_VAR_DIRTY),
        align_of_val(&sys::MUT_VAR_DIRTY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_WEAK_eq() {
    assert_eq!(WEAK, sys::WEAK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_WEAK_layout() {
    assert_eq!(size_of_val(&WEAK), size_of_val(&sys::WEAK));
    assert_eq!(align_of_val(&WEAK), align_of_val(&sys::WEAK));
}

#[cfg(feature = "sys")]
#[test]
fn sys_PRIM_eq() {
    assert_eq!(PRIM, sys::PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_PRIM_layout() {
    assert_eq!(size_of_val(&PRIM), size_of_val(&sys::PRIM));
    assert_eq!(align_of_val(&PRIM), align_of_val(&sys::PRIM));
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_PRIM_eq() {
    assert_eq!(MUT_PRIM, sys::MUT_PRIM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_MUT_PRIM_layout() {
    assert_eq!(size_of_val(&MUT_PRIM), size_of_val(&sys::MUT_PRIM));
    assert_eq!(align_of_val(&MUT_PRIM), align_of_val(&sys::MUT_PRIM));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_eq() {
    assert_eq!(TSO, sys::TSO);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_layout() {
    assert_eq!(size_of_val(&TSO), size_of_val(&sys::TSO));
    assert_eq!(align_of_val(&TSO), align_of_val(&sys::TSO));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STACK_eq() {
    assert_eq!(STACK, sys::STACK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STACK_layout() {
    assert_eq!(size_of_val(&STACK), size_of_val(&sys::STACK));
    assert_eq!(align_of_val(&STACK), align_of_val(&sys::STACK));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TREC_CHUNK_eq() {
    assert_eq!(TREC_CHUNK, sys::TREC_CHUNK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TREC_CHUNK_layout() {
    assert_eq!(size_of_val(&TREC_CHUNK), size_of_val(&sys::TREC_CHUNK));
    assert_eq!(align_of_val(&TREC_CHUNK), align_of_val(&sys::TREC_CHUNK));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ATOMICALLY_FRAME_eq() {
    assert_eq!(ATOMICALLY_FRAME, sys::ATOMICALLY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ATOMICALLY_FRAME_layout() {
    assert_eq!(
        size_of_val(&ATOMICALLY_FRAME),
        size_of_val(&sys::ATOMICALLY_FRAME)
    );
    assert_eq!(
        align_of_val(&ATOMICALLY_FRAME),
        align_of_val(&sys::ATOMICALLY_FRAME)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_RETRY_FRAME_eq() {
    assert_eq!(CATCH_RETRY_FRAME, sys::CATCH_RETRY_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_RETRY_FRAME_layout() {
    assert_eq!(
        size_of_val(&CATCH_RETRY_FRAME),
        size_of_val(&sys::CATCH_RETRY_FRAME)
    );
    assert_eq!(
        align_of_val(&CATCH_RETRY_FRAME),
        align_of_val(&sys::CATCH_RETRY_FRAME)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_STM_FRAME_eq() {
    assert_eq!(CATCH_STM_FRAME, sys::CATCH_STM_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CATCH_STM_FRAME_layout() {
    assert_eq!(
        size_of_val(&CATCH_STM_FRAME),
        size_of_val(&sys::CATCH_STM_FRAME)
    );
    assert_eq!(
        align_of_val(&CATCH_STM_FRAME),
        align_of_val(&sys::CATCH_STM_FRAME)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_WHITEHOLE_eq() {
    assert_eq!(WHITEHOLE, sys::WHITEHOLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_WHITEHOLE_layout() {
    assert_eq!(size_of_val(&WHITEHOLE), size_of_val(&sys::WHITEHOLE));
    assert_eq!(align_of_val(&WHITEHOLE), align_of_val(&sys::WHITEHOLE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_CLEAN_eq() {
    assert_eq!(SMALL_MUT_ARR_PTRS_CLEAN, sys::SMALL_MUT_ARR_PTRS_CLEAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_CLEAN_layout() {
    assert_eq!(
        size_of_val(&SMALL_MUT_ARR_PTRS_CLEAN),
        size_of_val(&sys::SMALL_MUT_ARR_PTRS_CLEAN)
    );
    assert_eq!(
        align_of_val(&SMALL_MUT_ARR_PTRS_CLEAN),
        align_of_val(&sys::SMALL_MUT_ARR_PTRS_CLEAN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_DIRTY_eq() {
    assert_eq!(SMALL_MUT_ARR_PTRS_DIRTY, sys::SMALL_MUT_ARR_PTRS_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_DIRTY_layout() {
    assert_eq!(
        size_of_val(&SMALL_MUT_ARR_PTRS_DIRTY),
        size_of_val(&sys::SMALL_MUT_ARR_PTRS_DIRTY)
    );
    assert_eq!(
        align_of_val(&SMALL_MUT_ARR_PTRS_DIRTY),
        align_of_val(&sys::SMALL_MUT_ARR_PTRS_DIRTY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_eq() {
    assert_eq!(
        SMALL_MUT_ARR_PTRS_FROZEN_DIRTY,
        sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_layout() {
    assert_eq!(
        size_of_val(&SMALL_MUT_ARR_PTRS_FROZEN_DIRTY),
        size_of_val(&sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY)
    );
    assert_eq!(
        align_of_val(&SMALL_MUT_ARR_PTRS_FROZEN_DIRTY),
        align_of_val(&sys::SMALL_MUT_ARR_PTRS_FROZEN_DIRTY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_eq() {
    assert_eq!(
        SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
        sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_layout() {
    assert_eq!(
        size_of_val(&SMALL_MUT_ARR_PTRS_FROZEN_CLEAN),
        size_of_val(&sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN)
    );
    assert_eq!(
        align_of_val(&SMALL_MUT_ARR_PTRS_FROZEN_CLEAN),
        align_of_val(&sys::SMALL_MUT_ARR_PTRS_FROZEN_CLEAN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COMPACT_NFDATA_eq() {
    assert_eq!(COMPACT_NFDATA, sys::COMPACT_NFDATA);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COMPACT_NFDATA_layout() {
    assert_eq!(
        size_of_val(&COMPACT_NFDATA),
        size_of_val(&sys::COMPACT_NFDATA)
    );
    assert_eq!(
        align_of_val(&COMPACT_NFDATA),
        align_of_val(&sys::COMPACT_NFDATA)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONTINUATION_eq() {
    assert_eq!(CONTINUATION, sys::CONTINUATION);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONTINUATION_layout() {
    assert_eq!(size_of_val(&CONTINUATION), size_of_val(&sys::CONTINUATION));
    assert_eq!(
        align_of_val(&CONTINUATION),
        align_of_val(&sys::CONTINUATION)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ANN_FRAME_eq() {
    assert_eq!(ANN_FRAME, sys::ANN_FRAME);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ANN_FRAME_layout() {
    assert_eq!(size_of_val(&ANN_FRAME), size_of_val(&sys::ANN_FRAME));
    assert_eq!(align_of_val(&ANN_FRAME), align_of_val(&sys::ANN_FRAME));
}

#[cfg(feature = "sys")]
#[test]
fn sys_N_CLOSURE_TYPES_eq() {
    assert_eq!(N_CLOSURE_TYPES, sys::N_CLOSURE_TYPES);
}

#[cfg(feature = "sys")]
#[test]
fn sys_N_CLOSURE_TYPES_layout() {
    assert_eq!(
        size_of_val(&N_CLOSURE_TYPES),
        size_of_val(&sys::N_CLOSURE_TYPES)
    );
    assert_eq!(
        align_of_val(&N_CLOSURE_TYPES),
        align_of_val(&sys::N_CLOSURE_TYPES)
    );
}
