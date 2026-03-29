use crate::ffi::rts::constants::{
    BlockedOnMVar, BlockedOnMVarRead, NotBlocked, TSO_BLOCKEX, TSO_INTERRUPTIBLE, ThreadComplete,
    ThreadFinished, ThreadKilled, ThreadRunGHC,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::closure_macros::{
    AP_STACK_sizeW, SET_INFO_RELAXED, get_itbl, get_ret_itbl, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{
    MessageThrowTo, MessageThrowTo_, StgAP_STACK, StgAtomicallyFrame, StgCatchFrame, StgClosure_,
    StgMVar, StgMVarTSOQueue, StgMVarTSOQueue_, StgTRecHeader, StgTRecHeader_, StgThunk,
    StgUpdateFrame,
};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::{StgStack, StgTSO_, dirty_STACK, dirty_TSO};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_AP_STACK_NOUPD_info, stg_AP_STACK_info, stg_END_TSO_QUEUE_closure, stg_IND_info,
    stg_MSG_NULL_info, stg_MSG_THROWTO_info, stg_NO_TREC_closure, stg_WHITEHOLE_info,
    stg_ap_pv_info, stg_atomically_info, stg_dummy_ret_closure, stg_enter_info,
    stg_maskAsyncExceptionszh_ret_info, stg_maskUninterruptiblezh_ret_info, stg_ret_p_info,
    stg_unmaskAsyncExceptionszh_ret_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
use crate::io_manager::{syncDelayCancel, syncIOCancel};
use crate::messages::doneWithMsgThrowTo;
use crate::prelude::*;
use crate::raise_async::{THROWTO_BLOCKED, THROWTO_SUCCESS, interruptible};
use crate::schedule::appendToRunQueue;
use crate::smp_closure_ops::{lockClosure, tryLockClosure, unlockClosure};
use crate::stm::{stmAbortTransaction, stmCondemnTransaction, stmFreeAbortedTRec};
use crate::threads::{threadStackUnderflow, tryWakeupThread, updateThunk};
use crate::trace::{DEBUG_RTS, trace_, traceCap_};

pub(crate) const THROWTO_SUCCESS: i32 = 0;

pub(crate) const THROWTO_BLOCKED: i32 = 1;

#[inline]
pub(crate) unsafe fn interruptible(mut t: *mut StgTSO) -> i32 {
    match (*t).why_blocked {
        1 | 6 | 14 | 12 | 3 | 4 | 5 => return 1,
        _ => return 0,
    };
}

unsafe fn throwToSingleThreaded__(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
    mut stop_at_atomically: bool,
    mut stop_here: *mut StgUpdateFrame,
) {
    if (*tso).what_next as i32 == ThreadComplete || (*tso).what_next as i32 == ThreadKilled {
        return;
    }

    removeFromQueues(cap, tso);
    raiseAsync(cap, tso, exception, stop_at_atomically, stop_here);
}

unsafe fn throwToSingleThreaded(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
) {
    throwToSingleThreaded__(cap, tso, exception, false, null_mut::<StgUpdateFrame>());
}

unsafe fn throwToSingleThreaded_(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
    mut stop_at_atomically: bool,
) {
    throwToSingleThreaded__(
        cap,
        tso,
        exception,
        stop_at_atomically,
        null_mut::<StgUpdateFrame>(),
    );
}

unsafe fn suspendComputation(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut stop_here: *mut StgUpdateFrame,
) {
    throwToSingleThreaded__(cap, tso, null_mut::<StgClosure>(), false, stop_here);
}

unsafe fn throwToSelf(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
) {
    let mut m = null_mut::<MessageThrowTo>();
    m = throwTo(cap, tso, tso, exception);

    if !m.is_null() {
        unlockClosure(m as *mut StgClosure, &raw const stg_MSG_THROWTO_info);
    }
}

unsafe fn throwTo(
    mut cap: *mut Capability,
    mut source: *mut StgTSO,
    mut target: *mut StgTSO,
    mut exception: *mut StgClosure,
) -> *mut MessageThrowTo {
    let mut msg = null_mut::<MessageThrowTo>();

    msg = allocate(
        cap,
        (size_of::<MessageThrowTo>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut MessageThrowTo;

    (*msg).header.info = &raw const stg_WHITEHOLE_info;
    (*msg).source = source;
    (*msg).target = target;
    (*msg).exception = exception;

    match throwToMsg(cap, msg) {
        0 => {
            (*msg).header.info = &raw const stg_MSG_THROWTO_info;

            return null_mut::<MessageThrowTo>();
        }
        1 | _ => return msg,
    };
}

unsafe fn throwToMsg(mut cap: *mut Capability, mut msg: *mut MessageThrowTo) -> u32 {
    let mut status: StgWord = 0;
    let mut target = (*msg).target;
    let mut target_cap = null_mut::<Capability>();

    loop {
        let mut what_next: StgWord16 = (*target).what_next;

        if what_next as i32 == ThreadComplete || what_next as i32 == ThreadKilled {
            return THROWTO_SUCCESS as u32;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            traceCap_(
                cap,
                c"throwTo: from thread %lu to thread %lu".as_ptr(),
                (*(*msg).source).id as u64,
                (*(*msg).target).id as u64,
            );
        }

        target_cap = (*target).cap as *mut Capability;

        if (*target).cap != cap {
            throwToSendMsg(cap, target_cap, msg);

            return THROWTO_BLOCKED as u32;
        }

        status = (*target).why_blocked as StgWord;

        match status {
            0 => {
                if (*target).flags & TSO_BLOCKEX as StgWord32 == 0 {
                    raiseAsync(
                        cap,
                        target,
                        (*msg).exception,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );

                    return THROWTO_SUCCESS as u32;
                } else {
                    blockedThrowTo(cap, target, msg);

                    return THROWTO_BLOCKED as u32;
                }
            }
            12 => {
                let mut i = null::<StgInfoTable>();
                let mut m = null_mut::<MessageThrowTo>();
                m = (*target).block_info.throwto as *mut MessageThrowTo;

                if m < msg {
                    i = lockClosure(m as *mut StgClosure);
                } else {
                    i = tryLockClosure(m as *mut StgClosure);

                    if i.is_null() {
                        throwToSendMsg(cap, (*target).cap as *mut Capability, msg);

                        return THROWTO_BLOCKED as u32;
                    }
                }

                if i == &raw const stg_MSG_NULL_info {
                    unlockClosure(m as *mut StgClosure, i);
                    tryWakeupThread(cap, target);
                } else if i != &raw const stg_MSG_THROWTO_info {
                    unlockClosure(m as *mut StgClosure, i);
                } else {
                    if (*target).flags & TSO_BLOCKEX as StgWord32 != 0
                        && (*target).flags & TSO_INTERRUPTIBLE as StgWord32 == 0
                    {
                        unlockClosure(m as *mut StgClosure, i);
                        blockedThrowTo(cap, target, msg);

                        return THROWTO_BLOCKED as u32;
                    }

                    doneWithMsgThrowTo(cap, m);

                    raiseAsync(
                        cap,
                        target,
                        (*msg).exception,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );

                    return THROWTO_SUCCESS as u32;
                }
            }
            1 | 14 => {
                let mut mvar = null_mut::<StgMVar>();
                let mut info = null_mut::<StgInfoTable>();
                mvar = (*target).block_info.closure as *mut StgMVar;

                match (*get_itbl(mvar as *mut StgClosure)).r#type {
                    39 | 40 => {
                        info = lockClosure(mvar as *mut StgClosure);

                        if (*target).why_blocked != BlockedOnMVar as StgWord32
                            && (*target).why_blocked != BlockedOnMVarRead as StgWord32
                            || (*target).block_info.closure as *mut StgMVar != mvar
                        {
                            unlockClosure(mvar as *mut StgClosure, info);
                        } else if (*target)._link
                            == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        {
                            unlockClosure(mvar as *mut StgClosure, info);
                            tryWakeupThread(cap, target);
                        } else if (*target).flags & TSO_BLOCKEX as StgWord32 != 0
                            && (*target).flags & TSO_INTERRUPTIBLE as StgWord32 == 0
                        {
                            blockedThrowTo(cap, target, msg);
                            unlockClosure(mvar as *mut StgClosure, info);

                            return THROWTO_BLOCKED as u32;
                        } else {
                            removeFromMVarBlockedQueue(target);

                            raiseAsync(
                                cap,
                                target,
                                (*msg).exception,
                                false,
                                null_mut::<StgUpdateFrame>(),
                            );

                            unlockClosure(mvar as *mut StgClosure, info);

                            return THROWTO_SUCCESS as u32;
                        }
                    }
                    _ => {}
                }
            }
            2 => {
                if (*target).flags & TSO_BLOCKEX as StgWord32 != 0 {
                    blockedThrowTo(cap, target, msg);

                    return THROWTO_BLOCKED as u32;
                } else {
                    SET_INFO_RELAXED(
                        (*target).block_info.bh as *mut StgClosure,
                        &raw const stg_IND_info,
                    );

                    raiseAsync(
                        cap,
                        target,
                        (*msg).exception,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );

                    return THROWTO_SUCCESS as u32;
                }
            }
            6 => {
                if (*target).flags & TSO_BLOCKEX as StgWord32 != 0
                    && (*target).flags & TSO_INTERRUPTIBLE as StgWord32 == 0
                {
                    blockedThrowTo(cap, target, msg);

                    return THROWTO_BLOCKED as u32;
                } else {
                    raiseAsync(
                        cap,
                        target,
                        (*msg).exception,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );

                    return THROWTO_SUCCESS as u32;
                }
            }
            11 | 10 => {
                blockedThrowTo(cap, target, msg);

                return THROWTO_BLOCKED as u32;
            }
            3 | 4 | 5 => {
                if (*target).flags & TSO_BLOCKEX as StgWord32 != 0
                    && (*target).flags & TSO_INTERRUPTIBLE as StgWord32 == 0
                {
                    blockedThrowTo(cap, target, msg);

                    return THROWTO_BLOCKED as u32;
                } else {
                    removeFromQueues(cap, target);

                    raiseAsync(
                        cap,
                        target,
                        (*msg).exception,
                        false,
                        null_mut::<StgUpdateFrame>(),
                    );

                    return THROWTO_SUCCESS as u32;
                }
            }
            13 => {
                tryWakeupThread(cap, target);
            }
            _ => {
                barf(
                    c"throwTo: unrecognised why_blocked (%d)".as_ptr(),
                    (*target).why_blocked,
                );
            }
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"throwTo: retrying...".as_ptr());
        }
    }
}

unsafe fn throwToSendMsg(
    mut cap: *mut Capability,
    mut target_cap: *mut Capability,
    mut msg: *mut MessageThrowTo,
) {
}

unsafe fn blockedThrowTo(
    mut cap: *mut Capability,
    mut target: *mut StgTSO,
    mut msg: *mut MessageThrowTo,
) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(
            cap,
            c"throwTo: blocking on thread %lu".as_ptr(),
            (*target).id as u64,
        );
    }

    dirty_TSO(cap, target);
    (*msg).link = (*target).blocked_exceptions;
    (*target).blocked_exceptions = msg as *mut MessageThrowTo_;
}

unsafe fn maybePerformBlockedException(mut cap: *mut Capability, mut tso: *mut StgTSO) -> i32 {
    let mut msg = null_mut::<MessageThrowTo>();
    let mut i = null::<StgInfoTable>();
    let mut source = null_mut::<StgTSO>();

    if (*tso).what_next as i32 == ThreadComplete || (*tso).what_next as i32 == ThreadFinished {
        if (*tso).blocked_exceptions
            != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                as *mut MessageThrowTo
        {
            awakenBlockedExceptionQueue(cap, tso);

            return 1;
        } else {
            return 0;
        }
    }

    if (*tso).blocked_exceptions
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut MessageThrowTo
        && (*tso).flags & TSO_BLOCKEX as StgWord32 != 0
    {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            traceCap_(
                cap,
                c"throwTo: thread %llu has blocked exceptions but is inside block".as_ptr(),
                (*tso).id,
            );
        }
    }

    if (*tso).blocked_exceptions
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut MessageThrowTo
        && ((*tso).flags & TSO_BLOCKEX as StgWord32 == 0
            || (*tso).flags & TSO_INTERRUPTIBLE as StgWord32 != 0 && interruptible(tso) != 0)
    {
        loop {
            msg = (*tso).blocked_exceptions as *mut MessageThrowTo;

            if msg
                == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut MessageThrowTo
            {
                return 0;
            }

            i = lockClosure(msg as *mut StgClosure);
            (*tso).blocked_exceptions = (*msg).link as *mut MessageThrowTo as *mut MessageThrowTo_;

            if !(i == &raw const stg_MSG_NULL_info) {
                break;
            }

            unlockClosure(msg as *mut StgClosure, i);
        }

        throwToSingleThreaded(cap, (*msg).target, (*msg).exception);
        source = (*msg).source;
        doneWithMsgThrowTo(cap, msg);
        tryWakeupThread(cap, source);

        return 1;
    }

    return 0;
}

unsafe fn awakenBlockedExceptionQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut msg = null_mut::<MessageThrowTo>();
    let mut i = null::<StgInfoTable>();
    let mut source = null_mut::<StgTSO>();
    msg = (*tso).blocked_exceptions as *mut MessageThrowTo;

    while msg
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut MessageThrowTo
    {
        i = lockClosure(msg as *mut StgClosure);

        if i != &raw const stg_MSG_NULL_info {
            source = (*msg).source;
            doneWithMsgThrowTo(cap, msg);
            tryWakeupThread(cap, source);
        } else {
            unlockClosure(msg as *mut StgClosure, i);
        }

        msg = (*msg).link as *mut MessageThrowTo;
    }

    (*tso).blocked_exceptions = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        as *mut MessageThrowTo as *mut MessageThrowTo_;
}

unsafe fn removeFromMVarBlockedQueue(mut tso: *mut StgTSO) {
    let mut mvar = (*tso).block_info.closure as *mut StgMVar;
    let mut q = (*tso)._link as *mut StgMVarTSOQueue;

    if q == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgMVarTSOQueue
    {
        return;
    }

    if (*mvar).head == q {
        (*mvar).head = (*q).link;
        SET_INFO_RELAXED(q as *mut StgClosure, &raw const stg_IND_info);

        if (*mvar).tail == q {
            (*mvar).tail = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                as *mut StgMVarTSOQueue as *mut StgMVarTSOQueue_;
        }
    } else if (*mvar).tail == q {
        SET_INFO_RELAXED(q as *mut StgClosure, &raw const stg_MSG_NULL_info);
    } else {
        SET_INFO_RELAXED(q as *mut StgClosure, &raw const stg_IND_info);
    }

    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
}

unsafe fn removeFromQueues(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    match (*tso).why_blocked {
        0 | 13 => return,
        1 | 14 => {
            removeFromMVarBlockedQueue(tso);
        }
        6 | 2 => {}
        12 => {
            let mut m = (*tso).block_info.throwto as *mut MessageThrowTo;
            doneWithMsgThrowTo(cap, m);
        }
        3 | 4 | 7 => {
            syncIOCancel(cap, tso);
        }
        5 => {
            syncDelayCancel(cap, tso);
        }
        _ => {
            barf(c"removeFromQueues: %d".as_ptr(), (*tso).why_blocked);
        }
    }

    (*tso).why_blocked = 0;
    appendToRunQueue(cap, tso);
}

unsafe fn raiseAsync(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
    mut stop_at_atomically: bool,
    mut stop_here: *mut StgUpdateFrame,
) -> *mut StgTSO {
    let mut info = null::<StgRetInfoTable>();
    let mut sp = null_mut::<StgWord>();
    let mut frame = null_mut::<StgWord>();
    let mut updatee = null_mut::<StgClosure>();
    let mut i: u32 = 0;
    let mut stack = null_mut::<StgStack>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(
            cap,
            c"raising exception in thread %llu.".as_ptr(),
            (*tso).id,
        );
    }

    stack = (*tso).stackobj as *mut StgStack;
    dirty_TSO(cap, tso);
    dirty_STACK(cap, stack);
    sp = (*stack).sp;

    if !stop_here.is_null() {
        updatee = (*stop_here).updatee;
    } else {
        updatee = null_mut::<StgClosure>();
    }

    if *sp.offset(0) == &raw const stg_enter_info as W_ {
        sp = sp.offset(1);
    } else {
        sp = sp.offset(-1);
        *sp.offset(0) = &raw mut stg_dummy_ret_closure as W_ as StgWord;
    }

    frame = sp.offset(1);

    while stop_here.is_null() || frame < stop_here as StgPtr {
        info = get_ret_itbl(frame as *mut StgClosure);

        match (*info).i.r#type {
            33 => {
                let mut ap = null_mut::<StgAP_STACK>();
                let mut words: u32 = 0;
                words = (frame.offset_from(sp) as i64 - 1) as u32;
                ap = allocate(cap, AP_STACK_sizeW(words) as W_) as *mut StgAP_STACK;
                (*ap).size = words as StgWord;
                (*ap).fun = *sp.offset(0) as *mut StgClosure;
                sp = sp.offset(1);
                i = 0;

                while i < words {
                    let fresh6 = sp;
                    sp = sp.offset(1);

                    let ref mut fresh7 =
                        *(&raw mut (*ap).payload as *mut *mut StgClosure).offset(i as isize);
                    *fresh7 = *fresh6 as *mut StgClosure;
                    i = i.wrapping_add(1);
                }

                (*ap).header.info = &raw const stg_AP_STACK_info;

                if (*(frame as *mut StgUpdateFrame)).updatee == updatee {
                    ap = updatee as *mut StgAP_STACK;
                } else {
                    updateThunk(
                        cap,
                        tso,
                        (*(frame as *mut StgUpdateFrame)).updatee,
                        ap as *mut StgClosure,
                    );
                }

                sp = sp.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize) as isize,
                );

                *sp.offset(0) = ap as W_ as StgWord;
                frame = sp.offset(1);
                continue;
            }
            35 => {
                let mut ap_0 = null_mut::<StgAP_STACK>();
                let mut words_0: u32 = 0;
                words_0 = (frame.offset_from(sp) as i64 - 1) as u32;
                ap_0 = allocate(cap, AP_STACK_sizeW(words_0) as W_) as *mut StgAP_STACK;
                (*ap_0).size = words_0 as StgWord;
                (*ap_0).fun = *sp.offset(0) as *mut StgClosure;
                sp = sp.offset(1);
                i = 0;

                while i < words_0 {
                    let fresh8 = sp;
                    sp = sp.offset(1);

                    let ref mut fresh9 =
                        *(&raw mut (*ap_0).payload as *mut *mut StgClosure).offset(i as isize);
                    *fresh9 = *fresh8 as *mut StgClosure;
                    i = i.wrapping_add(1);
                }

                (*ap_0).header.info = &raw const stg_AP_STACK_NOUPD_info;
                (*stack).sp = sp;
                threadStackUnderflow(cap, tso);
                stack = (*tso).stackobj as *mut StgStack;
                sp = (*stack).sp;
                sp = sp.offset(-1);
                *sp.offset(0) = ap_0 as W_ as StgWord;
                frame = sp.offset(1);
                continue;
            }
            36 => {
                (*tso).what_next = ThreadKilled as StgWord16;
                (*stack).sp = frame.offset(
                    (size_of::<StgStopFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                break;
            }
            34 => {
                if !exception.is_null() {
                    let mut handler = (*(frame as *mut StgCatchFrame)).handler;
                    sp = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);

                    if (*tso).flags & TSO_BLOCKEX as StgWord32 == 0 {
                        sp = sp.offset(-1);
                        *sp.offset(0) =
                            &raw const stg_unmaskAsyncExceptionszh_ret_info as W_ as StgWord;
                    }

                    if (*tso).flags & (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32
                        != TSO_BLOCKEX as StgWord32
                    {
                        (*tso).flags |= (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
                    }

                    sp = sp.offset(-4);
                    *sp.offset(0) = &raw const stg_enter_info as W_ as StgWord;
                    *sp.offset(1) = handler as W_ as StgWord;
                    *sp.offset(2) = &raw const stg_ap_pv_info as W_ as StgWord;
                    *sp.offset(3) = exception as W_ as StgWord;
                    (*stack).sp = sp;
                    (*tso).what_next = 1;
                    break;
                }
            }
            55 => {
                if stop_at_atomically {
                    stmCondemnTransaction(cap, (*tso).trec as *mut StgTRecHeader);
                    (*stack).sp = frame.offset(-2);
                    *(*stack).sp.offset(1) = &raw mut stg_NO_TREC_closure as W_ as StgWord;
                    *(*stack).sp.offset(0) = &raw const stg_ret_p_info as W_ as StgWord;
                    (*tso).what_next = ThreadRunGHC as StgWord16;
                    break;
                } else {
                    let mut trec = (*tso).trec as *mut StgTRecHeader;
                    let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;
                    let mut atomically = null_mut::<StgThunk>();
                    let mut af = frame as *mut StgAtomicallyFrame;

                    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                        traceCap_(cap, c"raiseAsync: freezing atomically frame".as_ptr());
                    }

                    stmAbortTransaction(cap, trec);
                    stmFreeAbortedTRec(cap, trec);
                    (*tso).trec = outer as *mut StgTRecHeader_;

                    atomically = allocate(
                        cap,
                        (size_of::<StgThunk>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            .wrapping_add(1 as usize) as W_,
                    ) as *mut StgThunk;

                    (*atomically).header.info = &raw const stg_atomically_info;

                    let ref mut fresh10 =
                        *(&raw mut (*atomically).payload as *mut *mut StgClosure_).offset(0);
                    *fresh10 = (*af).code as *mut StgClosure_;
                    frame = frame.offset(
                        (size_of::<StgAtomicallyFrame>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            as isize,
                    );

                    sp = frame.offset(-1);
                    *sp.offset(0) = atomically as W_ as StgWord;
                    continue;
                }
            }
            57 | 56 => {
                let mut trec_0 = (*tso).trec as *mut StgTRecHeader;
                let mut outer_0 = (*trec_0).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    traceCap_(
                        cap,
                        c"found atomically block delivering async exception".as_ptr(),
                    );
                }

                stmAbortTransaction(cap, trec_0);
                stmFreeAbortedTRec(cap, trec_0);
                (*tso).trec = outer_0 as *mut StgTRecHeader_;
            }
            _ => {
                if *frame == &raw const stg_unmaskAsyncExceptionszh_ret_info as W_ {
                    (*tso).flags &= !(TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
                } else if *frame == &raw const stg_maskAsyncExceptionszh_ret_info as W_ {
                    (*tso).flags |= (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
                } else if *frame == &raw const stg_maskUninterruptiblezh_ret_info as W_ {
                    (*tso).flags |= TSO_BLOCKEX as StgWord32;
                    (*tso).flags &= !TSO_INTERRUPTIBLE as StgWord32;
                }
            }
        }

        frame = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);
    }

    if (*tso).why_blocked != NotBlocked as StgWord32 {
        (*tso).why_blocked = NotBlocked as StgWord32;
        appendToRunQueue(cap, tso);
    }

    return tso;
}
