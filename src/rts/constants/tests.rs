use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_MIN_PAYLOAD_SIZE() {
    assert_eq!(sys::MIN_PAYLOAD_SIZE, super::MIN_PAYLOAD_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPEC_SELECTEE_SIZE() {
    assert_eq!(
        sys::MAX_SPEC_SELECTEE_SIZE,
        super::MAX_SPEC_SELECTEE_SIZE.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPEC_AP_SIZE() {
    assert_eq!(sys::MAX_SPEC_AP_SIZE, super::MAX_SPEC_AP_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPEC_THUNK_SIZE() {
    assert_eq!(sys::MAX_SPEC_THUNK_SIZE, super::MAX_SPEC_THUNK_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPEC_FUN_SIZE() {
    assert_eq!(sys::MAX_SPEC_FUN_SIZE, super::MAX_SPEC_FUN_SIZE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPEC_CONSTR_SIZE() {
    assert_eq!(
        sys::MAX_SPEC_CONSTR_SIZE,
        super::MAX_SPEC_CONSTR_SIZE.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_INTLIKE() {
    assert_eq!(sys::MAX_INTLIKE, super::MAX_INTLIKE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MIN_INTLIKE() {
    assert_eq!(sys::MIN_INTLIKE, super::MIN_INTLIKE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_CHARLIKE() {
    assert_eq!(sys::MAX_CHARLIKE, super::MAX_CHARLIKE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MIN_CHARLIKE() {
    assert_eq!(sys::MIN_CHARLIKE, super::MIN_CHARLIKE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MUT_ARR_PTRS_CARD_BITS() {
    assert_eq!(
        sys::MUT_ARR_PTRS_CARD_BITS,
        super::MUT_ARR_PTRS_CARD_BITS.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_VANILLA_REG() {
    assert_eq!(sys::MAX_VANILLA_REG, super::MAX_VANILLA_REG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_FLOAT_REG() {
    assert_eq!(sys::MAX_FLOAT_REG, super::MAX_FLOAT_REG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_DOUBLE_REG() {
    assert_eq!(sys::MAX_DOUBLE_REG, super::MAX_DOUBLE_REG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_LONG_REG() {
    assert_eq!(sys::MAX_LONG_REG, super::MAX_LONG_REG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_XMM_REG() {
    assert_eq!(sys::MAX_XMM_REG, super::MAX_XMM_REG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_INFO_OTHER_TAG() {
    assert_eq!(sys::INFO_OTHER_TAG, super::INFO_OTHER_TAG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_INFO_IND_TAG() {
    assert_eq!(sys::INFO_IND_TAG, super::INFO_IND_TAG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_INFO_FIRST_TAG() {
    assert_eq!(sys::INFO_FIRST_TAG, super::INFO_FIRST_TAG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RESERVED_C_STACK_BYTES() {
    assert_eq!(
        sys::RESERVED_C_STACK_BYTES,
        super::RESERVED_C_STACK_BYTES.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_RUN_STACK_FRAME_SIZE() {
    assert_eq!(
        sys::STG_RUN_STACK_FRAME_SIZE,
        super::STG_RUN_STACK_FRAME_SIZE.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_RUN() {
    assert_eq!(sys::STG_RUN, super::STG_RUN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_RETURN() {
    assert_eq!(sys::STG_RETURN, super::STG_RETURN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_RESERVED_STACK_WORDS() {
    assert_eq!(
        sys::RESERVED_STACK_WORDS,
        super::RESERVED_STACK_WORDS.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_AP_STACK_SPLIM() {
    assert_eq!(sys::AP_STACK_SPLIM, super::AP_STACK_SPLIM.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BLOCK_SHIFT() {
    assert_eq!(sys::BLOCK_SHIFT, super::BLOCK_SHIFT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MBLOCK_SHIFT() {
    assert_eq!(sys::MBLOCK_SHIFT, super::MBLOCK_SHIFT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BITMAP_SIZE_MASK() {
    assert_eq!(sys::BITMAP_SIZE_MASK, super::BITMAP_SIZE_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BITMAP_BITS_SHIFT() {
    assert_eq!(sys::BITMAP_BITS_SHIFT, super::BITMAP_BITS_SHIFT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_SHIFT() {
    assert_eq!(sys::LDV_SHIFT, super::LDV_SHIFT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_STATE_MASK() {
    assert_eq!(sys::LDV_STATE_MASK, super::LDV_STATE_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_CREATE_MASK() {
    assert_eq!(sys::LDV_CREATE_MASK, super::LDV_CREATE_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_LAST_MASK() {
    assert_eq!(sys::LDV_LAST_MASK, super::LDV_LAST_MASK.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_STATE_CREATE() {
    assert_eq!(sys::LDV_STATE_CREATE, super::LDV_STATE_CREATE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LDV_STATE_USE() {
    assert_eq!(sys::LDV_STATE_USE, super::LDV_STATE_USE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_INVALID_GHC_POINTER() {
    assert_eq!(sys::INVALID_GHC_POINTER, super::INVALID_GHC_POINTER.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadRunGHC() {
    assert_eq!(sys::ThreadRunGHC, super::ThreadRunGHC.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadInterpret() {
    assert_eq!(sys::ThreadInterpret, super::ThreadInterpret.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadKilled() {
    assert_eq!(sys::ThreadKilled, super::ThreadKilled.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadComplete() {
    assert_eq!(sys::ThreadComplete, super::ThreadComplete.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_NotBlocked() {
    assert_eq!(sys::NotBlocked, super::NotBlocked.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnMVar() {
    assert_eq!(sys::BlockedOnMVar, super::BlockedOnMVar.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnMVarRead() {
    assert_eq!(sys::BlockedOnMVarRead, super::BlockedOnMVarRead.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnBlackHole() {
    assert_eq!(sys::BlockedOnBlackHole, super::BlockedOnBlackHole.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnRead() {
    assert_eq!(sys::BlockedOnRead, super::BlockedOnRead.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnWrite() {
    assert_eq!(sys::BlockedOnWrite, super::BlockedOnWrite.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnDelay() {
    assert_eq!(sys::BlockedOnDelay, super::BlockedOnDelay.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnSTM() {
    assert_eq!(sys::BlockedOnSTM, super::BlockedOnSTM.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnDoProc() {
    assert_eq!(sys::BlockedOnDoProc, super::BlockedOnDoProc.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnCCall() {
    assert_eq!(sys::BlockedOnCCall, super::BlockedOnCCall.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnCCall_Interruptible() {
    assert_eq!(
        sys::BlockedOnCCall_Interruptible,
        super::BlockedOnCCall_Interruptible.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_BlockedOnMsgThrowTo() {
    assert_eq!(sys::BlockedOnMsgThrowTo, super::BlockedOnMsgThrowTo.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadMigrating() {
    assert_eq!(sys::ThreadMigrating, super::ThreadMigrating.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HeapOverflow() {
    assert_eq!(sys::HeapOverflow, super::HeapOverflow.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_StackOverflow() {
    assert_eq!(sys::StackOverflow, super::StackOverflow.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadYielding() {
    assert_eq!(sys::ThreadYielding, super::ThreadYielding.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadBlocked() {
    assert_eq!(sys::ThreadBlocked, super::ThreadBlocked.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ThreadFinished() {
    assert_eq!(sys::ThreadFinished, super::ThreadFinished.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_LOCKED() {
    assert_eq!(sys::TSO_LOCKED, super::TSO_LOCKED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_BLOCKEX() {
    assert_eq!(sys::TSO_BLOCKEX, super::TSO_BLOCKEX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_INTERRUPTIBLE() {
    assert_eq!(sys::TSO_INTERRUPTIBLE, super::TSO_INTERRUPTIBLE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_STOPPED_ON_BREAKPOINT() {
    assert_eq!(
        sys::TSO_STOPPED_ON_BREAKPOINT,
        super::TSO_STOPPED_ON_BREAKPOINT.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_MARKED() {
    assert_eq!(sys::TSO_MARKED, super::TSO_MARKED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_SQUEEZED() {
    assert_eq!(sys::TSO_SQUEEZED, super::TSO_SQUEEZED.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TSO_ALLOC_LIMIT() {
    assert_eq!(sys::TSO_ALLOC_LIMIT, super::TSO_ALLOC_LIMIT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SPIN_COUNT() {
    assert_eq!(sys::SPIN_COUNT, super::SPIN_COUNT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_SPARE_WORKERS() {
    assert_eq!(sys::MAX_SPARE_WORKERS, super::MAX_SPARE_WORKERS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_NUMA_NODES() {
    assert_eq!(sys::MAX_NUMA_NODES, super::MAX_NUMA_NODES.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_CLOSURE_DESC_BUFFER_SIZE() {
    assert_eq!(
        sys::CLOSURE_DESC_BUFFER_SIZE,
        super::CLOSURE_DESC_BUFFER_SIZE.into()
    );
}
