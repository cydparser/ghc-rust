use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MIN_PAYLOAD_SIZE() {
    assert_eq!(sys::MIN_PAYLOAD_SIZE, MIN_PAYLOAD_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPEC_SELECTEE_SIZE() {
    assert_eq!(sys::MAX_SPEC_SELECTEE_SIZE, MAX_SPEC_SELECTEE_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPEC_AP_SIZE() {
    assert_eq!(sys::MAX_SPEC_AP_SIZE, MAX_SPEC_AP_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPEC_THUNK_SIZE() {
    assert_eq!(sys::MAX_SPEC_THUNK_SIZE, MAX_SPEC_THUNK_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPEC_FUN_SIZE() {
    assert_eq!(sys::MAX_SPEC_FUN_SIZE, MAX_SPEC_FUN_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPEC_CONSTR_SIZE() {
    assert_eq!(sys::MAX_SPEC_CONSTR_SIZE, MAX_SPEC_CONSTR_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_INTLIKE() {
    assert_eq!(sys::MAX_INTLIKE, MAX_INTLIKE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MIN_INTLIKE() {
    assert_eq!(sys::MIN_INTLIKE, MIN_INTLIKE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_CHARLIKE() {
    assert_eq!(sys::MAX_CHARLIKE, MAX_CHARLIKE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MIN_CHARLIKE() {
    assert_eq!(sys::MIN_CHARLIKE, MIN_CHARLIKE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MUT_ARR_PTRS_CARD_BITS() {
    assert_eq!(sys::MUT_ARR_PTRS_CARD_BITS, MUT_ARR_PTRS_CARD_BITS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_VANILLA_REG() {
    assert_eq!(sys::MAX_VANILLA_REG, MAX_VANILLA_REG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_FLOAT_REG() {
    assert_eq!(sys::MAX_FLOAT_REG, MAX_FLOAT_REG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_DOUBLE_REG() {
    assert_eq!(sys::MAX_DOUBLE_REG, MAX_DOUBLE_REG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_LONG_REG() {
    assert_eq!(sys::MAX_LONG_REG, MAX_LONG_REG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_XMM_REG() {
    assert_eq!(sys::MAX_XMM_REG, MAX_XMM_REG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INFO_OTHER_TAG() {
    assert_eq!(sys::INFO_OTHER_TAG, INFO_OTHER_TAG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INFO_IND_TAG() {
    assert_eq!(sys::INFO_IND_TAG, INFO_IND_TAG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INFO_FIRST_TAG() {
    assert_eq!(sys::INFO_FIRST_TAG, INFO_FIRST_TAG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RESERVED_C_STACK_BYTES() {
    assert_eq!(sys::RESERVED_C_STACK_BYTES, RESERVED_C_STACK_BYTES);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_RUN() {
    assert_eq!(sys::STG_RUN, STG_RUN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_RETURN() {
    assert_eq!(sys::STG_RETURN, STG_RETURN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RESERVED_STACK_WORDS() {
    assert_eq!(sys::RESERVED_STACK_WORDS, RESERVED_STACK_WORDS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_AP_STACK_SPLIM() {
    assert_eq!(sys::AP_STACK_SPLIM, AP_STACK_SPLIM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BLOCK_SHIFT() {
    assert_eq!(sys::BLOCK_SHIFT, BLOCK_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MBLOCK_SHIFT() {
    assert_eq!(sys::MBLOCK_SHIFT, MBLOCK_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BITMAP_SIZE_MASK() {
    assert_eq!(sys::BITMAP_SIZE_MASK, BITMAP_SIZE_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BITMAP_BITS_SHIFT() {
    assert_eq!(sys::BITMAP_BITS_SHIFT, BITMAP_BITS_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_SHIFT() {
    assert_eq!(sys::LDV_SHIFT, LDV_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_STATE_MASK() {
    assert_eq!(sys::LDV_STATE_MASK, LDV_STATE_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_CREATE_MASK() {
    assert_eq!(sys::LDV_CREATE_MASK, LDV_CREATE_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_LAST_MASK() {
    assert_eq!(sys::LDV_LAST_MASK, LDV_LAST_MASK);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_STATE_CREATE() {
    assert_eq!(sys::LDV_STATE_CREATE, LDV_STATE_CREATE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_LDV_STATE_USE() {
    assert_eq!(sys::LDV_STATE_USE, LDV_STATE_USE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_INVALID_GHC_POINTER() {
    assert_eq!(sys::INVALID_GHC_POINTER, INVALID_GHC_POINTER);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadRunGHC() {
    assert_eq!(sys::ThreadRunGHC, ThreadRunGHC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadInterpret() {
    assert_eq!(sys::ThreadInterpret, ThreadInterpret);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadKilled() {
    assert_eq!(sys::ThreadKilled, ThreadKilled);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadComplete() {
    assert_eq!(sys::ThreadComplete, ThreadComplete);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_NotBlocked() {
    assert_eq!(sys::NotBlocked, NotBlocked);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnMVar() {
    assert_eq!(sys::BlockedOnMVar, BlockedOnMVar);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnMVarRead() {
    assert_eq!(sys::BlockedOnMVarRead, BlockedOnMVarRead);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnBlackHole() {
    assert_eq!(sys::BlockedOnBlackHole, BlockedOnBlackHole);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnRead() {
    assert_eq!(sys::BlockedOnRead, BlockedOnRead);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnWrite() {
    assert_eq!(sys::BlockedOnWrite, BlockedOnWrite);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnDelay() {
    assert_eq!(sys::BlockedOnDelay, BlockedOnDelay);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnSTM() {
    assert_eq!(sys::BlockedOnSTM, BlockedOnSTM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnDoProc() {
    assert_eq!(sys::BlockedOnDoProc, BlockedOnDoProc);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnCCall() {
    assert_eq!(sys::BlockedOnCCall, BlockedOnCCall);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnCCall_Interruptible() {
    assert_eq!(
        sys::BlockedOnCCall_Interruptible,
        BlockedOnCCall_Interruptible
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BlockedOnMsgThrowTo() {
    assert_eq!(sys::BlockedOnMsgThrowTo, BlockedOnMsgThrowTo);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadMigrating() {
    assert_eq!(sys::ThreadMigrating, ThreadMigrating);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HeapOverflow() {
    assert_eq!(sys::HeapOverflow, HeapOverflow);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_StackOverflow() {
    assert_eq!(sys::StackOverflow, StackOverflow);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadYielding() {
    assert_eq!(sys::ThreadYielding, ThreadYielding);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadBlocked() {
    assert_eq!(sys::ThreadBlocked, ThreadBlocked);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ThreadFinished() {
    assert_eq!(sys::ThreadFinished, ThreadFinished);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_LOCKED() {
    assert_eq!(sys::TSO_LOCKED, TSO_LOCKED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_BLOCKEX() {
    assert_eq!(sys::TSO_BLOCKEX, TSO_BLOCKEX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_INTERRUPTIBLE() {
    assert_eq!(sys::TSO_INTERRUPTIBLE, TSO_INTERRUPTIBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_STOPPED_ON_BREAKPOINT() {
    assert_eq!(sys::TSO_STOPPED_ON_BREAKPOINT, TSO_STOPPED_ON_BREAKPOINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_MARKED() {
    assert_eq!(sys::TSO_MARKED, TSO_MARKED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_SQUEEZED() {
    assert_eq!(sys::TSO_SQUEEZED, TSO_SQUEEZED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TSO_ALLOC_LIMIT() {
    assert_eq!(sys::TSO_ALLOC_LIMIT, TSO_ALLOC_LIMIT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SPIN_COUNT() {
    assert_eq!(sys::SPIN_COUNT, SPIN_COUNT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_SPARE_WORKERS() {
    assert_eq!(sys::MAX_SPARE_WORKERS, MAX_SPARE_WORKERS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_NUMA_NODES() {
    assert_eq!(sys::MAX_NUMA_NODES, MAX_NUMA_NODES);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CLOSURE_DESC_BUFFER_SIZE() {
    assert_eq!(sys::CLOSURE_DESC_BUFFER_SIZE, CLOSURE_DESC_BUFFER_SIZE);
}
