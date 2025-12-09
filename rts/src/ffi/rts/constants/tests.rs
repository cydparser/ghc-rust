#[allow(unused_imports)]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_RESERVED_STACK_WORDS_eq() {
    assert_eq!(RESERVED_STACK_WORDS, sys::RESERVED_STACK_WORDS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_RESERVED_STACK_WORDS_layout() {
    assert_eq!(
        size_of_val(&RESERVED_STACK_WORDS),
        size_of_val(&sys::RESERVED_STACK_WORDS)
    );
    assert_eq!(
        align_of_val(&RESERVED_STACK_WORDS),
        align_of_val(&sys::RESERVED_STACK_WORDS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BITMAP_BITS_SHIFT_eq() {
    assert_eq!(BITMAP_BITS_SHIFT, sys::BITMAP_BITS_SHIFT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BITMAP_BITS_SHIFT_layout() {
    assert_eq!(
        size_of_val(&BITMAP_BITS_SHIFT),
        size_of_val(&sys::BITMAP_BITS_SHIFT)
    );
    assert_eq!(
        align_of_val(&BITMAP_BITS_SHIFT),
        align_of_val(&sys::BITMAP_BITS_SHIFT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadRunGHC_eq() {
    assert_eq!(ThreadRunGHC, sys::ThreadRunGHC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadRunGHC_layout() {
    assert_eq!(size_of_val(&ThreadRunGHC), size_of_val(&sys::ThreadRunGHC));
    assert_eq!(
        align_of_val(&ThreadRunGHC),
        align_of_val(&sys::ThreadRunGHC)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadInterpret_eq() {
    assert_eq!(ThreadInterpret, sys::ThreadInterpret);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadInterpret_layout() {
    assert_eq!(
        size_of_val(&ThreadInterpret),
        size_of_val(&sys::ThreadInterpret)
    );
    assert_eq!(
        align_of_val(&ThreadInterpret),
        align_of_val(&sys::ThreadInterpret)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadKilled_eq() {
    assert_eq!(ThreadKilled, sys::ThreadKilled);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadKilled_layout() {
    assert_eq!(size_of_val(&ThreadKilled), size_of_val(&sys::ThreadKilled));
    assert_eq!(
        align_of_val(&ThreadKilled),
        align_of_val(&sys::ThreadKilled)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadComplete_eq() {
    assert_eq!(ThreadComplete, sys::ThreadComplete);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadComplete_layout() {
    assert_eq!(
        size_of_val(&ThreadComplete),
        size_of_val(&sys::ThreadComplete)
    );
    assert_eq!(
        align_of_val(&ThreadComplete),
        align_of_val(&sys::ThreadComplete)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_NotBlocked_eq() {
    assert_eq!(NotBlocked, sys::NotBlocked);
}

#[cfg(feature = "sys")]
#[test]
fn sys_NotBlocked_layout() {
    assert_eq!(size_of_val(&NotBlocked), size_of_val(&sys::NotBlocked));
    assert_eq!(align_of_val(&NotBlocked), align_of_val(&sys::NotBlocked));
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMVar_eq() {
    assert_eq!(BlockedOnMVar, sys::BlockedOnMVar);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMVar_layout() {
    assert_eq!(
        size_of_val(&BlockedOnMVar),
        size_of_val(&sys::BlockedOnMVar)
    );
    assert_eq!(
        align_of_val(&BlockedOnMVar),
        align_of_val(&sys::BlockedOnMVar)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMVarRead_eq() {
    assert_eq!(BlockedOnMVarRead, sys::BlockedOnMVarRead);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMVarRead_layout() {
    assert_eq!(
        size_of_val(&BlockedOnMVarRead),
        size_of_val(&sys::BlockedOnMVarRead)
    );
    assert_eq!(
        align_of_val(&BlockedOnMVarRead),
        align_of_val(&sys::BlockedOnMVarRead)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnBlackHole_eq() {
    assert_eq!(BlockedOnBlackHole, sys::BlockedOnBlackHole);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnBlackHole_layout() {
    assert_eq!(
        size_of_val(&BlockedOnBlackHole),
        size_of_val(&sys::BlockedOnBlackHole)
    );
    assert_eq!(
        align_of_val(&BlockedOnBlackHole),
        align_of_val(&sys::BlockedOnBlackHole)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnRead_eq() {
    assert_eq!(BlockedOnRead, sys::BlockedOnRead);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnRead_layout() {
    assert_eq!(
        size_of_val(&BlockedOnRead),
        size_of_val(&sys::BlockedOnRead)
    );
    assert_eq!(
        align_of_val(&BlockedOnRead),
        align_of_val(&sys::BlockedOnRead)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnWrite_eq() {
    assert_eq!(BlockedOnWrite, sys::BlockedOnWrite);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnWrite_layout() {
    assert_eq!(
        size_of_val(&BlockedOnWrite),
        size_of_val(&sys::BlockedOnWrite)
    );
    assert_eq!(
        align_of_val(&BlockedOnWrite),
        align_of_val(&sys::BlockedOnWrite)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnDelay_eq() {
    assert_eq!(BlockedOnDelay, sys::BlockedOnDelay);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnDelay_layout() {
    assert_eq!(
        size_of_val(&BlockedOnDelay),
        size_of_val(&sys::BlockedOnDelay)
    );
    assert_eq!(
        align_of_val(&BlockedOnDelay),
        align_of_val(&sys::BlockedOnDelay)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnSTM_eq() {
    assert_eq!(BlockedOnSTM, sys::BlockedOnSTM);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnSTM_layout() {
    assert_eq!(size_of_val(&BlockedOnSTM), size_of_val(&sys::BlockedOnSTM));
    assert_eq!(
        align_of_val(&BlockedOnSTM),
        align_of_val(&sys::BlockedOnSTM)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnDoProc_eq() {
    assert_eq!(BlockedOnDoProc, sys::BlockedOnDoProc);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnDoProc_layout() {
    assert_eq!(
        size_of_val(&BlockedOnDoProc),
        size_of_val(&sys::BlockedOnDoProc)
    );
    assert_eq!(
        align_of_val(&BlockedOnDoProc),
        align_of_val(&sys::BlockedOnDoProc)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnCCall_eq() {
    assert_eq!(BlockedOnCCall, sys::BlockedOnCCall);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnCCall_layout() {
    assert_eq!(
        size_of_val(&BlockedOnCCall),
        size_of_val(&sys::BlockedOnCCall)
    );
    assert_eq!(
        align_of_val(&BlockedOnCCall),
        align_of_val(&sys::BlockedOnCCall)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnCCall_Interruptible_eq() {
    assert_eq!(
        BlockedOnCCall_Interruptible,
        sys::BlockedOnCCall_Interruptible
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnCCall_Interruptible_layout() {
    assert_eq!(
        size_of_val(&BlockedOnCCall_Interruptible),
        size_of_val(&sys::BlockedOnCCall_Interruptible)
    );
    assert_eq!(
        align_of_val(&BlockedOnCCall_Interruptible),
        align_of_val(&sys::BlockedOnCCall_Interruptible)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMsgThrowTo_eq() {
    assert_eq!(BlockedOnMsgThrowTo, sys::BlockedOnMsgThrowTo);
}

#[cfg(feature = "sys")]
#[test]
fn sys_BlockedOnMsgThrowTo_layout() {
    assert_eq!(
        size_of_val(&BlockedOnMsgThrowTo),
        size_of_val(&sys::BlockedOnMsgThrowTo)
    );
    assert_eq!(
        align_of_val(&BlockedOnMsgThrowTo),
        align_of_val(&sys::BlockedOnMsgThrowTo)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadMigrating_eq() {
    assert_eq!(ThreadMigrating, sys::ThreadMigrating);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadMigrating_layout() {
    assert_eq!(
        size_of_val(&ThreadMigrating),
        size_of_val(&sys::ThreadMigrating)
    );
    assert_eq!(
        align_of_val(&ThreadMigrating),
        align_of_val(&sys::ThreadMigrating)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HeapOverflow_eq() {
    assert_eq!(HeapOverflow, sys::HeapOverflow);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HeapOverflow_layout() {
    assert_eq!(size_of_val(&HeapOverflow), size_of_val(&sys::HeapOverflow));
    assert_eq!(
        align_of_val(&HeapOverflow),
        align_of_val(&sys::HeapOverflow)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StackOverflow_eq() {
    assert_eq!(StackOverflow, sys::StackOverflow);
}

#[cfg(feature = "sys")]
#[test]
fn sys_StackOverflow_layout() {
    assert_eq!(
        size_of_val(&StackOverflow),
        size_of_val(&sys::StackOverflow)
    );
    assert_eq!(
        align_of_val(&StackOverflow),
        align_of_val(&sys::StackOverflow)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadBlocked_eq() {
    assert_eq!(ThreadBlocked, sys::ThreadBlocked);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadBlocked_layout() {
    assert_eq!(
        size_of_val(&ThreadBlocked),
        size_of_val(&sys::ThreadBlocked)
    );
    assert_eq!(
        align_of_val(&ThreadBlocked),
        align_of_val(&sys::ThreadBlocked)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadFinished_eq() {
    assert_eq!(ThreadFinished, sys::ThreadFinished);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ThreadFinished_layout() {
    assert_eq!(
        size_of_val(&ThreadFinished),
        size_of_val(&sys::ThreadFinished)
    );
    assert_eq!(
        align_of_val(&ThreadFinished),
        align_of_val(&sys::ThreadFinished)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_LOCKED_eq() {
    assert_eq!(TSO_LOCKED, sys::TSO_LOCKED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_LOCKED_layout() {
    assert_eq!(size_of_val(&TSO_LOCKED), size_of_val(&sys::TSO_LOCKED));
    assert_eq!(align_of_val(&TSO_LOCKED), align_of_val(&sys::TSO_LOCKED));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_BLOCKEX_eq() {
    assert_eq!(TSO_BLOCKEX, sys::TSO_BLOCKEX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_BLOCKEX_layout() {
    assert_eq!(size_of_val(&TSO_BLOCKEX), size_of_val(&sys::TSO_BLOCKEX));
    assert_eq!(align_of_val(&TSO_BLOCKEX), align_of_val(&sys::TSO_BLOCKEX));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_INTERRUPTIBLE_eq() {
    assert_eq!(TSO_INTERRUPTIBLE, sys::TSO_INTERRUPTIBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_INTERRUPTIBLE_layout() {
    assert_eq!(
        size_of_val(&TSO_INTERRUPTIBLE),
        size_of_val(&sys::TSO_INTERRUPTIBLE)
    );
    assert_eq!(
        align_of_val(&TSO_INTERRUPTIBLE),
        align_of_val(&sys::TSO_INTERRUPTIBLE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOPPED_ON_BREAKPOINT_eq() {
    assert_eq!(TSO_STOPPED_ON_BREAKPOINT, sys::TSO_STOPPED_ON_BREAKPOINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOPPED_ON_BREAKPOINT_layout() {
    assert_eq!(
        size_of_val(&TSO_STOPPED_ON_BREAKPOINT),
        size_of_val(&sys::TSO_STOPPED_ON_BREAKPOINT)
    );
    assert_eq!(
        align_of_val(&TSO_STOPPED_ON_BREAKPOINT),
        align_of_val(&sys::TSO_STOPPED_ON_BREAKPOINT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_MARKED_eq() {
    assert_eq!(TSO_MARKED, sys::TSO_MARKED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_MARKED_layout() {
    assert_eq!(size_of_val(&TSO_MARKED), size_of_val(&sys::TSO_MARKED));
    assert_eq!(align_of_val(&TSO_MARKED), align_of_val(&sys::TSO_MARKED));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_SQUEEZED_eq() {
    assert_eq!(TSO_SQUEEZED, sys::TSO_SQUEEZED);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_SQUEEZED_layout() {
    assert_eq!(size_of_val(&TSO_SQUEEZED), size_of_val(&sys::TSO_SQUEEZED));
    assert_eq!(
        align_of_val(&TSO_SQUEEZED),
        align_of_val(&sys::TSO_SQUEEZED)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_ALLOC_LIMIT_eq() {
    assert_eq!(TSO_ALLOC_LIMIT, sys::TSO_ALLOC_LIMIT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_ALLOC_LIMIT_layout() {
    assert_eq!(
        size_of_val(&TSO_ALLOC_LIMIT),
        size_of_val(&sys::TSO_ALLOC_LIMIT)
    );
    assert_eq!(
        align_of_val(&TSO_ALLOC_LIMIT),
        align_of_val(&sys::TSO_ALLOC_LIMIT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOP_NEXT_BREAKPOINT_eq() {
    assert_eq!(TSO_STOP_NEXT_BREAKPOINT, sys::TSO_STOP_NEXT_BREAKPOINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOP_NEXT_BREAKPOINT_layout() {
    assert_eq!(
        size_of_val(&TSO_STOP_NEXT_BREAKPOINT),
        size_of_val(&sys::TSO_STOP_NEXT_BREAKPOINT)
    );
    assert_eq!(
        align_of_val(&TSO_STOP_NEXT_BREAKPOINT),
        align_of_val(&sys::TSO_STOP_NEXT_BREAKPOINT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOP_AFTER_RETURN_eq() {
    assert_eq!(TSO_STOP_AFTER_RETURN, sys::TSO_STOP_AFTER_RETURN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TSO_STOP_AFTER_RETURN_layout() {
    assert_eq!(
        size_of_val(&TSO_STOP_AFTER_RETURN),
        size_of_val(&sys::TSO_STOP_AFTER_RETURN)
    );
    assert_eq!(
        align_of_val(&TSO_STOP_AFTER_RETURN),
        align_of_val(&sys::TSO_STOP_AFTER_RETURN)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_CLOSURE_DESC_BUFFER_SIZE_eq() {
    assert_eq!(CLOSURE_DESC_BUFFER_SIZE, sys::CLOSURE_DESC_BUFFER_SIZE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_CLOSURE_DESC_BUFFER_SIZE_layout() {
    assert_eq!(
        size_of_val(&CLOSURE_DESC_BUFFER_SIZE),
        size_of_val(&sys::CLOSURE_DESC_BUFFER_SIZE)
    );
    assert_eq!(
        align_of_val(&CLOSURE_DESC_BUFFER_SIZE),
        align_of_val(&sys::CLOSURE_DESC_BUFFER_SIZE)
    );
}
