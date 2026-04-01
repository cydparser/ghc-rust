use crate::alloc_array::allocateMutArrPtrs;
use crate::capability::{Capability_, getCapability};
use crate::check_vector_support::vectorSupportGlobalVar;
use crate::ffi::hs_ffi::{HS_BOOL_TRUE, HsBool};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{
    BlockedOnMVarRead, LDV_SHIFT, LDV_STATE_CREATE, NotBlocked, RESERVED_STACK_WORDS,
    TSO_ALLOC_LIMIT, TSO_SQUEEZED, ThreadMigrating, ThreadRunGHC,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::prof::ccs::{CCS_MAIN, CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::block::{BLOCK_SIZE_W, round_to_mblocks};
use crate::ffi::rts::storage::closure_macros::{
    SET_INFO_RELAXED, UNTAG_CLOSURE, doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
    overwritingClosure, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{
    _StgMutArrPtrs, Message, MessageBlackHole, MessageThrowTo, MessageThrowTo_, MessageWakeup,
    StgBlockingQueue, StgBlockingQueue_, StgInd, StgMVar, StgMVarTSOQueue, StgMVarTSOQueue_,
    StgMutArrPtrs, StgTRecHeader, StgTRecHeader_, StgUnderflowFrame,
};
use crate::ffi::rts::storage::gc::{allocate, g0, generations};
use crate::ffi::rts::storage::tso::{
    STACK_DIRTY, StgStack, StgStack_, StgTSO_, StgThreadID, dirty_STACK, setTSOLink,
};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, stg_BLACKHOLE_info, stg_BLOCKING_QUEUE_CLEAN_info,
    stg_BLOCKING_QUEUE_DIRTY_info, stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure, stg_IND_info,
    stg_MSG_BLACKHOLE_info, stg_MSG_NULL_info, stg_MSG_TRY_WAKEUP_info, stg_MVAR_CLEAN_info,
    stg_MVAR_DIRTY_info, stg_NO_TREC_closure, stg_STACK_info, stg_STM_AWOKEN_closure, stg_TSO_info,
    stg_WHITEHOLE_info, stg_block_throwto_info, stg_ret_p_info, stg_stack_underflow_frame_d_info,
    stg_stack_underflow_frame_v16_info, stg_stack_underflow_frame_v32_info,
    stg_stack_underflow_frame_v64_info, stg_stop_thread_info,
};
use crate::ffi::stg::types::{StgBool, StgInt64, StgPtr, StgWord, StgWord8, StgWord16, StgWord32};
use crate::ffi::stg::{ASSIGN_Int64, P_, W_};
use crate::messages::sendMessage;
use crate::prelude::*;
use crate::printer::printStackChunk;
use crate::raise_async::throwToSelf;
use crate::schedule::{appendToRunQueue, sched_mutex};
use crate::sm::sanity::checkTSO;
use crate::sm::storage::dirty_MVAR;
use crate::smp_closure_ops::{lockClosure, unlockClosure};
use crate::task::InCall_;
use crate::trace::{
    DEBUG_RTS, trace_, traceCap_, traceEventCreateThread, traceEventMigrateThread,
    traceEventThreadWakeup,
};
use crate::updates::updateWithIndirection;

#[cfg(test)]
mod tests;

static mut next_thread_id: StgThreadID = 1;

const MIN_STACK_WORDS: usize = (RESERVED_STACK_WORDS as usize)
    .wrapping_add(
        (size_of::<StgStopFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize),
    )
    .wrapping_add(3 as usize);

unsafe fn createThread(mut cap: *mut Capability, mut size: W_) -> *mut StgTSO {
    let mut tso = null_mut::<StgTSO>();
    let mut stack = null_mut::<StgStack>();
    let mut stack_size: u32 = 0;

    if size
        < MIN_STACK_WORDS
            .wrapping_add(
                (size_of::<StgStack>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize),
            )
            .wrapping_add(
                (size_of::<StgTSO>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize),
            ) as W_
    {
        size = MIN_STACK_WORDS
            .wrapping_add(
                (size_of::<StgStack>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize),
            )
            .wrapping_add(
                (size_of::<StgTSO>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize),
            ) as W_;
    }

    stack_size = round_to_mblocks(
        (size as StgWord).wrapping_sub(
            (size_of::<StgTSO>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgWord,
        ),
    ) as u32;

    stack = allocate(cap, stack_size as W_) as *mut StgStack;

    let ref mut fresh8 = (*(stack as *mut StgClosure)).header.prof.ccs;
    *fresh8 = (*cap).r.rCCCS as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(stack as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(stack as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(stack as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*stack).header.info).store(&raw const stg_STACK_info, Ordering::Relaxed);
    (*stack).stack_size = (stack_size as usize).wrapping_sub(
        (size_of::<StgStack>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize),
    ) as StgWord32;

    (*stack).sp =
        (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize) as StgPtr;
    (*stack).dirty = STACK_DIRTY as StgWord8;
    (*stack).marking = 0;

    tso = allocate(
        cap,
        (size_of::<StgTSO>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut StgTSO;

    let ref mut fresh9 = (*(tso as *mut StgClosure)).header.prof.ccs;
    *fresh9 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(tso as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(tso as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(tso as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*tso).header.info).store(&raw const stg_TSO_info, Ordering::Relaxed);
    (*tso).what_next = ThreadRunGHC as StgWord16;
    (*tso).block_info.closure =
        &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgClosure;
    (*tso).why_blocked = NotBlocked as StgWord32;
    (*tso).blocked_exceptions = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        as *mut MessageThrowTo as *mut MessageThrowTo_;
    (*tso).bq = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        as *mut StgBlockingQueue as *mut StgBlockingQueue_;
    (*tso).flags = 0;
    (*tso).dirty = 1;
    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
    (*tso).saved_errno = 0;
    (*tso).bound = null_mut::<InCall_>();
    (*tso).cap = cap as *mut Capability_;
    (*tso).stackobj = stack as *mut StgStack_;
    (*tso).tot_stack_size = (*stack).stack_size;
    ASSIGN_Int64(&raw mut (*tso).alloc_limit as *mut W_, 0);
    (*tso).trec =
        &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader as *mut StgTRecHeader_;
    (*tso).label = null_mut::<StgArrBytes>();
    (*tso).prof.cccs = &raw mut CCS_MAIN as *mut CostCentreStack;
    (*stack).sp = (*stack).sp.offset(
        -((size_of::<StgStopFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize),
    );

    let ref mut fresh1 = (*((*stack).sp as *mut StgClosure)).header.prof.ccs;
    *fresh1 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*((*stack).sp as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*((*stack).sp as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*((*stack).sp as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*((*stack).sp as *mut StgClosure)).header.info).store(
        &raw const stg_stop_thread_info as *mut StgInfoTable,
        Ordering::Relaxed,
    );

    let mut __r = pthread_mutex_lock(&raw mut sched_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Threads.c".as_ptr(),
            131,
            __r,
        );
    }

    let fresh2 = next_thread_id;
    next_thread_id = next_thread_id.wrapping_add(1);
    (*tso).id = fresh2;
    (*tso).global_link = (*g0).threads as *mut StgTSO_;
    (*g0).threads = tso;

    if pthread_mutex_unlock(&raw mut sched_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Threads.c".as_ptr(),
            137,
        );
    }

    traceEventCreateThread(cap, tso);

    return tso;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn eq_thread(mut tso1: StgPtr, mut tso2: StgPtr) -> bool {
    return tso1 == tso2;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn cmp_thread(mut tso1: StgPtr, mut tso2: StgPtr) -> c_int {
    if tso1 == tso2 {
        return 0;
    }

    let mut id1: StgThreadID = (*(tso1 as *mut StgTSO)).id;
    let mut id2: StgThreadID = (*(tso2 as *mut StgTSO)).id;

    if (id1 != id2) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Threads.c".as_ptr(), 173);
    }

    return if id1 < id2 { -1 } else { 1 };
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getThreadId(mut tso: StgPtr) -> StgThreadID {
    return (*(tso as *mut StgTSO)).id;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(mut tso: StgPtr) {
    (*(tso as *mut StgTSO)).flags |= TSO_ALLOC_LIMIT as StgWord32;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(mut tso: StgPtr) {
    (*(tso as *mut StgTSO)).flags &= !TSO_ALLOC_LIMIT as StgWord32;
}

unsafe fn removeThreadFromQueue(
    mut cap: *mut Capability,
    mut queue: *mut *mut StgTSO,
    mut tso: *mut StgTSO,
) -> bool {
    let mut t = null_mut::<StgTSO>();
    let mut prev = null_mut::<StgTSO>();
    prev = null_mut::<StgTSO>();
    t = *queue;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if t == tso {
            if !prev.is_null() {
                setTSOLink(cap, prev, (*t)._link as *mut StgTSO);
                (*t)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut StgTSO_;

                return false;
            } else {
                *queue = (*t)._link as *mut StgTSO;
                (*t)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut StgTSO_;

                return true;
            }
        }

        prev = t;
        t = (*t)._link as *mut StgTSO;
    }

    barf(c"removeThreadFromQueue: not found".as_ptr());
}

unsafe fn removeThreadFromDeQueue(
    mut cap: *mut Capability,
    mut head: *mut *mut StgTSO,
    mut tail: *mut *mut StgTSO,
    mut tso: *mut StgTSO,
) -> bool {
    let mut t = null_mut::<StgTSO>();
    let mut prev = null_mut::<StgTSO>();
    let mut flag = false;
    prev = null_mut::<StgTSO>();
    t = *head;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if t == tso {
            if !prev.is_null() {
                setTSOLink(cap, prev, (*t)._link as *mut StgTSO);
                flag = false;
            } else {
                *head = (*t)._link as *mut StgTSO;
                flag = true;
            }

            (*t)._link =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;

            if *tail == tso {
                if !prev.is_null() {
                    *tail = prev;
                } else {
                    *tail = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
                }

                return true;
            } else {
                return flag;
            }
        }

        prev = t;
        t = (*t)._link as *mut StgTSO;
    }

    barf(c"removeThreadFromDeQueue: not found".as_ptr());
}

unsafe fn tryWakeupThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    traceEventThreadWakeup(cap, tso, (*(*tso).cap).no);

    let mut tso_owner = (&raw mut (*tso).cap).load(Ordering::Relaxed);

    if tso_owner != cap {
        let mut msg = null_mut::<MessageWakeup>();

        msg = allocate(
            cap,
            (size_of::<MessageWakeup>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as W_,
        ) as *mut MessageWakeup;

        (*msg).tso = tso;

        let ref mut fresh18 = (*(msg as *mut StgClosure)).header.prof.ccs;
        *fresh18 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(msg as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*(msg as *mut StgClosure)).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*(msg as *mut StgClosure)).header.prof.hp.era = user_era;
        }

        (&raw mut (*msg).header.info).store(&raw const stg_MSG_TRY_WAKEUP_info, Ordering::Relaxed);
        sendMessage(cap, tso_owner, msg as *mut Message);

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            traceCap_(
                cap,
                c"message: try wakeup thread %llu on cap %d".as_ptr(),
                (*tso).id,
                (*tso_owner).no,
            );
        }

        return;
    }

    match (&raw mut (*tso).why_blocked).load(Ordering::Acquire) {
        1 | 14 => {
            if (*tso)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
                (*tso).block_info.closure = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void
                    as *mut StgTSO as *mut StgClosure;
            } else {
                return;
            }
        }
        12 => {
            let mut i = null::<StgInfoTable>();
            i = lockClosure((*tso).block_info.closure);
            unlockClosure((*tso).block_info.closure, i);

            if i != &raw const stg_MSG_NULL_info {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    traceCap_(
                        cap,
                        c"thread %llu still blocked on throwto (%p)".as_ptr(),
                        (*tso).id,
                        (*(*tso).block_info.throwto).header.info,
                    );
                }

                return;
            }

            if (*(*(*tso).stackobj).sp.offset(0) == &raw const stg_block_throwto_info as StgWord)
                as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/Threads.c".as_ptr(), 319);
            }

            (*(*tso).stackobj).sp = (*(*tso).stackobj).sp.offset(3);
        }
        6 => {
            (*tso).block_info.closure = &raw mut stg_STM_AWOKEN_closure;
        }
        2 | 13 => {}
        _ => return,
    }

    (*tso).why_blocked = NotBlocked as StgWord32;
    appendToRunQueue(cap, tso);
}

unsafe fn migrateThread(mut from: *mut Capability, mut tso: *mut StgTSO, mut to: *mut Capability) {
    traceEventMigrateThread(from, tso, (*to).no);
    (*tso).why_blocked = ThreadMigrating as StgWord32;
    (*tso).cap = to as *mut Capability_;
    tryWakeupThread(from, tso);
}

unsafe fn wakeBlockingQueue(mut cap: *mut Capability, mut bq: *mut StgBlockingQueue) {
    let mut msg = null_mut::<MessageBlackHole>();
    let mut i = null::<StgInfoTable>();

    if ((*bq).header.info == &raw const stg_BLOCKING_QUEUE_DIRTY_info
        || (*bq).header.info == &raw const stg_BLOCKING_QUEUE_CLEAN_info) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Threads.c".as_ptr(), 390);
    }

    msg = (*bq).queue as *mut MessageBlackHole;

    while msg
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut MessageBlackHole
    {
        i = (&raw mut (*msg).header.info).load(Ordering::Acquire);

        if i != &raw const stg_IND_info {
            if (i == &raw const stg_MSG_BLACKHOLE_info) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Threads.c".as_ptr(), 396);
            }

            tryWakeupThread(cap, (*msg).tso);
        }

        msg = (*msg).link as *mut MessageBlackHole;
    }

    overwritingClosure(bq as *mut StgClosure);
    SET_INFO_RELAXED(bq as *mut StgClosure, &raw const stg_IND_info);

    if doingLDVProfiling() {
        (*(bq as *mut StgClosure)).header.prof.hp.ldvw =
            (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
    }
}

unsafe fn checkBlockingQueues(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut bq = null_mut::<StgBlockingQueue>();
    let mut next = null_mut::<StgBlockingQueue>();
    let mut p = null_mut::<StgClosure>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(
            cap,
            c"collision occurred; checking blocking queues for thread %llu".as_ptr(),
            (*tso).id,
        );
    }

    bq = (*tso).bq as *mut StgBlockingQueue;

    while bq
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgBlockingQueue
    {
        next = (*bq).link as *mut StgBlockingQueue;

        let mut bqinfo = (&raw mut (*bq).header.info).load(Ordering::Acquire);

        if !(bqinfo == &raw const stg_IND_info) {
            p = UNTAG_CLOSURE((*bq).bh);

            let mut pinfo = (&raw mut (*p).header.info).load(Ordering::Acquire);

            if pinfo != &raw const stg_BLACKHOLE_info
                || (&raw mut (*(p as *mut StgInd)).indirectee).load(Ordering::Relaxed)
                    != bq as *mut StgClosure
            {
                wakeBlockingQueue(cap, bq);
            }
        }

        bq = next;
    }
}

unsafe fn updateThunk(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut thunk: *mut StgClosure,
    mut val: *mut StgClosure,
) {
    let mut v = null_mut::<StgClosure>();
    let mut owner = null_mut::<StgTSO>();
    let mut i = null::<StgInfoTable>();
    i = (&raw mut (*thunk).header.info).load(Ordering::Acquire);

    if i != &raw const stg_BLACKHOLE_info
        && i != &raw const stg_CAF_BLACKHOLE_info
        && i != &raw const __stg_EAGER_BLACKHOLE_info
        && i != &raw const stg_WHITEHOLE_info
    {
        updateWithIndirection(cap, thunk, val);
        return;
    }

    v = UNTAG_CLOSURE((&raw mut (*(thunk as *mut StgInd)).indirectee).load(Ordering::Acquire));

    updateWithIndirection(cap, thunk, val);

    if v as *mut StgTSO == tso {
        return;
    }

    i = (&raw mut (*v).header.info).load(Ordering::Acquire);

    if i == &raw const stg_TSO_info {
        checkBlockingQueues(cap, tso);
        return;
    }

    if i != &raw const stg_BLOCKING_QUEUE_CLEAN_info
        && i != &raw const stg_BLOCKING_QUEUE_DIRTY_info
    {
        checkBlockingQueues(cap, tso);
        return;
    }

    owner = (*(v as *mut StgBlockingQueue)).owner;

    if owner != tso {
        checkBlockingQueues(cap, tso);
    } else {
        wakeBlockingQueue(cap, v as *mut StgBlockingQueue);
    };
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    return HS_BOOL_TRUE as HsBool;
}

unsafe fn isThreadBound(mut tso: *mut StgTSO) -> StgBool {
    return ((*tso).bound != NULL as *mut InCall_) as i32;
}

unsafe fn threadStackOverflow(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut new_stack = null_mut::<StgStack>();
    let mut old_stack = null_mut::<StgStack>();
    let mut frame = null_mut::<StgUnderflowFrame>();
    let mut chunk_size: W_ = 0;

    if RtsFlags.DebugFlags.sanity {
        checkTSO(tso);
    }

    if RtsFlags.GcFlags.maxStkSize > 0 && (*tso).tot_stack_size >= RtsFlags.GcFlags.maxStkSize {
        if (*tso).flags & TSO_SQUEEZED as StgWord32 != 0 {
            return;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"threadStackOverflow of TSO %llu (%p): stack too large (now %ld; max is %ld)"
                    .as_ptr(),
                (*tso).id,
                tso,
                (*(*tso).stackobj).stack_size as i64,
                RtsFlags.GcFlags.maxStkSize,
            );
        }

        if RtsFlags.DebugFlags.gc {
            printStackChunk(
                (*(*tso).stackobj).sp,
                ({
                    let mut _a = (&raw mut (*(*tso).stackobj).stack as *mut StgWord)
                        .offset((*(*tso).stackobj).stack_size as isize);

                    let mut _b = (*(*tso).stackobj).sp.offset(64);

                    if _a <= _b {
                        _a as *mut StgWord
                    } else {
                        _b as *mut StgWord
                    }
                }),
            );
        }

        throwToSelf(cap, tso, (*ghc_hs_iface).stackOverflow_closure);
        return;
    }

    if (*tso).flags & TSO_SQUEEZED as StgWord32 != 0
        && (*(*tso).stackobj)
            .sp
            .offset_from(&raw mut (*(*tso).stackobj).stack as *mut StgWord) as i64 as W_
            >= BLOCK_SIZE_W as W_
    {
        return;
    }

    old_stack = (*tso).stackobj as *mut StgStack;

    if (*old_stack).sp
        > (&raw mut (*old_stack).stack as *mut StgWord)
            .offset((*old_stack).stack_size.wrapping_div(2 as StgWord32) as isize)
    {
        chunk_size = ({
            let mut _a = (2 as u64).wrapping_mul(
                ((*old_stack).stack_size as u64).wrapping_add(
                    (size_of::<StgStack>() as u64)
                        .wrapping_add(size_of::<W_>() as u64)
                        .wrapping_sub(1 as u64)
                        .wrapping_div(size_of::<W_>() as u64),
                ),
            );

            let mut _b = RtsFlags.GcFlags.stkChunkSize as u64;

            if _a <= _b { _b as u64 } else { _a as u64 }
        }) as W_;
    } else {
        chunk_size = RtsFlags.GcFlags.stkChunkSize as W_;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(
            cap,
            c"allocating new stack chunk of size %d bytes".as_ptr(),
            chunk_size.wrapping_mul(size_of::<W_>() as W_),
        );
    }

    (*cap).r.rCurrentTSO = tso as *mut StgTSO_;
    new_stack = allocate(cap, chunk_size) as *mut StgStack;
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();

    let ref mut fresh19 = (*(new_stack as *mut StgClosure)).header.prof.ccs;
    *fresh19 = (*old_stack).header.prof.ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(new_stack as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(new_stack as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(new_stack as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*new_stack).header.info).store(&raw const stg_STACK_info, Ordering::Relaxed);
    (*new_stack).dirty = 0;
    (*new_stack).marking = 0;
    (*new_stack).stack_size = chunk_size.wrapping_sub(
        (size_of::<StgStack>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as StgWord32;

    (*new_stack).sp = (&raw mut (*new_stack).stack as *mut StgWord)
        .offset((*new_stack).stack_size as isize) as StgPtr;
    (*tso).tot_stack_size = (*tso).tot_stack_size.wrapping_add((*new_stack).stack_size);

    let mut sp = null_mut::<StgWord>();
    let mut chunk_words: W_ = 0;
    let mut size: W_ = 0;
    sp = (*old_stack).sp as *mut StgWord;

    while sp
        < ({
            let mut _a = (*old_stack)
                .sp
                .offset(RtsFlags.GcFlags.stkChunkBufferSize as isize);

            let mut _b = (&raw mut (*old_stack).stack as *mut StgWord)
                .offset((*old_stack).stack_size as isize);
            (if _a <= _b { _a as StgPtr } else { _b as StgPtr })
        })
    {
        size = stack_frame_sizeW(sp as *mut StgClosure) as W_;

        if sp.offset(size as isize)
            > (*old_stack).sp.offset(
                ((*new_stack).stack_size as usize).wrapping_sub(
                    (size_of::<StgUnderflowFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize),
                ) as isize,
            )
        {
            break;
        }

        sp = sp.offset(size as isize);
    }

    if !(sp
        == (&raw mut (*old_stack).stack as *mut StgWord).offset((*old_stack).stack_size as isize))
    {
        (*new_stack).sp = (*new_stack).sp.offset(
            -((size_of::<StgUnderflowFrame>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as isize),
        );

        frame = (*new_stack).sp as *mut StgUnderflowFrame;

        match vectorSupportGlobalVar {
            3 => {
                (*frame).info = &raw const stg_stack_underflow_frame_v64_info;
            }
            2 => {
                (*frame).info = &raw const stg_stack_underflow_frame_v32_info;
            }
            1 => {
                (*frame).info = &raw const stg_stack_underflow_frame_v16_info;
            }
            _ => {
                (*frame).info = &raw const stg_stack_underflow_frame_d_info;
            }
        }

        (*frame).next_chunk = old_stack as *mut StgStack_;
    }

    chunk_words = sp.offset_from((*old_stack).sp) as i64 as W_;

    memcpy(
        (*new_stack).sp.offset(-(chunk_words as isize)) as *mut c_void,
        (*old_stack).sp as *const c_void,
        chunk_words.wrapping_mul(size_of::<W_>() as W_) as usize,
    );

    (*old_stack).sp = (*old_stack).sp.offset(chunk_words as isize);
    (*new_stack).sp = (*new_stack).sp.offset(-(chunk_words as isize));
    (*tso).stackobj = new_stack as *mut StgStack_;
    dirty_STACK(cap, new_stack);

    if RtsFlags.DebugFlags.sanity {
        checkTSO(tso);
    }
}

unsafe fn threadStackUnderflow(mut cap: *mut Capability, mut tso: *mut StgTSO) -> W_ {
    let mut new_stack = null_mut::<StgStack>();
    let mut old_stack = null_mut::<StgStack>();
    let mut frame = null_mut::<StgUnderflowFrame>();
    let mut retvals: u32 = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceCap_(cap, c"stack underflow".as_ptr());
    }

    old_stack = (*tso).stackobj as *mut StgStack;
    frame = (&raw mut (*old_stack).stack as *mut StgWord)
        .offset((*old_stack).stack_size as isize)
        .offset(
            -((size_of::<StgUnderflowFrame>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as isize),
        ) as *mut StgUnderflowFrame;

    if ((*frame).info == &raw const stg_stack_underflow_frame_d_info
        || (*frame).info == &raw const stg_stack_underflow_frame_v16_info
        || (*frame).info == &raw const stg_stack_underflow_frame_v32_info
        || (*frame).info == &raw const stg_stack_underflow_frame_v64_info) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Threads.c".as_ptr(), 760);
    }

    new_stack = (*frame).next_chunk as *mut StgStack;
    (*tso).stackobj = new_stack as *mut StgStack_;
    retvals = (frame as P_).offset_from((*old_stack).sp) as i64 as u32;

    if retvals != 0 {
        if ((*new_stack)
            .sp
            .offset_from(&raw mut (*new_stack).stack as *mut StgWord) as i64 as W_)
            < retvals as W_
        {
            barf(c"threadStackUnderflow: not enough space for return values".as_ptr());
        }

        memcpy(
            (*new_stack).sp.offset(-(retvals as isize)) as *mut c_void,
            (*old_stack).sp as *const c_void,
            (retvals as usize).wrapping_mul(size_of::<W_>() as usize),
        );
    }

    (*old_stack).sp = (&raw mut (*old_stack).stack as *mut StgWord)
        .offset((*old_stack).stack_size as isize) as StgPtr;
    (*tso).tot_stack_size = (*tso).tot_stack_size.wrapping_sub((*old_stack).stack_size);
    dirty_STACK(cap, new_stack);
    (*new_stack).sp = (*new_stack).sp.offset(-(retvals as isize));

    return retvals as W_;
}

unsafe fn performTryPutMVar(
    mut cap: *mut Capability,
    mut mvar: *mut StgMVar,
    mut value: *mut StgClosure,
) -> bool {
    let mut why_blocked: StgWord = 0;
    let mut stack: *mut StgStack = null_mut::<StgStack>();
    let mut info = null::<StgInfoTable>();
    let mut qinfo = null::<StgInfoTable>();
    let mut q = null_mut::<StgMVarTSOQueue>();
    let mut tso = null_mut::<StgTSO>();
    info = lockClosure(mvar as *mut StgClosure);

    if (*mvar).value != &raw mut stg_END_TSO_QUEUE_closure {
        unlockClosure(mvar as *mut StgClosure, info);

        return false;
    }

    q = (*mvar).head as *mut StgMVarTSOQueue;

    loop {
        if q == &raw mut stg_END_TSO_QUEUE_closure as *mut StgMVarTSOQueue {
            if info == &raw const stg_MVAR_CLEAN_info {
                dirty_MVAR(&raw mut (*cap).r, mvar as *mut StgClosure, (*mvar).value);
            }

            (*mvar).value = value;
            unlockClosure(mvar as *mut StgClosure, &raw const stg_MVAR_DIRTY_info);

            return true;
        }

        qinfo = (&raw mut (*q).header.info).load(Ordering::Acquire);

        if qinfo == &raw const stg_IND_info || qinfo == &raw const stg_MSG_NULL_info {
            q = (&raw mut (*(q as *mut StgInd)).indirectee).load(Ordering::Acquire)
                as *mut StgMVarTSOQueue;
        } else {
            tso = (*q).tso as *mut StgTSO;
            q = (*q).link as *mut StgMVarTSOQueue;
            (*mvar).head = q as *mut StgMVarTSOQueue_;

            if q == &raw mut stg_END_TSO_QUEUE_closure as *mut StgMVarTSOQueue {
                (*mvar).tail = &raw mut stg_END_TSO_QUEUE_closure as *mut StgMVarTSOQueue
                    as *mut StgMVarTSOQueue_;
            } else if info == &raw const stg_MVAR_CLEAN_info {
                dirty_MVAR(&raw mut (*cap).r, mvar as *mut StgClosure, (*mvar).value);
                info = &raw const stg_MVAR_DIRTY_info;
            }

            why_blocked = (&raw mut (*tso).why_blocked).load(Ordering::Acquire) as StgWord;

            if (why_blocked == 14 || why_blocked == 1) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Threads.c".as_ptr(), 854);
            }

            if ((*tso).block_info.closure == mvar as *mut StgClosure) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Threads.c".as_ptr(), 855);
            }

            stack = (*tso).stackobj as *mut StgStack;
            ((*stack).sp.offset(1) as *mut StgWord).store(value as W_, Ordering::Relaxed);
            ((*stack).sp.offset(0) as *mut StgWord)
                .store(&raw const stg_ret_p_info as W_, Ordering::Relaxed);
            (&raw mut (*tso)._link).store(
                &raw mut stg_END_TSO_QUEUE_closure as *mut StgTSO,
                Ordering::Release,
            );

            if (*stack).dirty as i32 & STACK_DIRTY == 0 {
                dirty_STACK(cap, stack);
            }

            tryWakeupThread(cap, tso);

            if !(why_blocked == BlockedOnMVarRead as StgWord) {
                break;
            }
        }
    }

    if (why_blocked == 1) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Threads.c".as_ptr(), 876);
    }

    unlockClosure(mvar as *mut StgClosure, info);

    return true;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn listThreads(mut cap: *mut Capability) -> *mut _StgMutArrPtrs {
    let mut i: StgWord = 0;
    let mut __r = pthread_mutex_lock(&raw mut sched_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Threads.c".as_ptr(),
            886,
            __r,
        );
    }

    let mut n_threads: StgWord = 0;
    let mut g = 0;

    while (g as u32) < RtsFlags.GcFlags.generations {
        let mut t = (*generations.offset(g as isize)).threads;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            n_threads = n_threads.wrapping_add(1);
            t = (*t).global_link as *mut StgTSO;
        }

        g = g.wrapping_add(1);
    }

    let mut arr = allocateMutArrPtrs(cap, n_threads, (*cap).r.rCCCS as *mut CostCentreStack);

    if !((arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as i32 as i64 != 0) {
        i = 0;

        let mut g_0 = 0;

        while (g_0 as u32) < RtsFlags.GcFlags.generations {
            let mut t_0 = (*generations.offset(g_0 as isize)).threads;

            while t_0 != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
                if i == n_threads {
                    break;
                }

                let ref mut fresh3 =
                    *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(i as isize);
                *fresh3 = t_0 as *mut StgClosure;
                i = i.wrapping_add(1);
                t_0 = (*t_0).global_link as *mut StgTSO;
            }

            g_0 = g_0.wrapping_add(1);
        }

        if (i == n_threads) as i32 as i64 != 0 {
        } else {
            barf(c"listThreads: Found too few threads".as_ptr());
        }
    }

    if pthread_mutex_unlock(&raw mut sched_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Threads.c".as_ptr(),
            914,
        );
    }

    return arr as *mut _StgMutArrPtrs;
}

unsafe fn printThreadBlockage(mut tso: *mut StgTSO) {
    match (&raw mut (*tso).why_blocked).load(Ordering::Acquire) {
        1 => {
            debugBelch(
                c"is blocked on an MVar @ %p".as_ptr(),
                (*tso).block_info.closure,
            );
        }
        14 => {
            debugBelch(
                c"is blocked on atomic MVar read @ %p".as_ptr(),
                (*tso).block_info.closure,
            );
        }
        2 => {
            debugBelch(
                c"is blocked on a black hole %p".as_ptr(),
                (*(*tso).block_info.bh).bh as *mut StgBlockingQueue,
            );
        }
        12 => {
            debugBelch(c"is blocked on a throwto message".as_ptr());
        }
        0 => {
            debugBelch(c"is not blocked".as_ptr());
        }
        13 => {
            debugBelch(c"is runnable, but not on the run queue".as_ptr());
        }
        10 => {
            debugBelch(c"is blocked on an external call".as_ptr());
        }
        11 => {
            debugBelch(c"is blocked on an external call (but may be interrupted)".as_ptr());
        }
        6 => {
            debugBelch(c"is blocked on an STM operation".as_ptr());
        }
        _ => {
            barf(
                c"printThreadBlockage: strange tso->why_blocked: %d for TSO %llu (%p)".as_ptr(),
                (*tso).why_blocked,
                (*tso).id,
                tso,
            );
        }
    };
}

unsafe fn printThreadStatus(mut t: *mut StgTSO) {
    debugBelch(
        c"\tthread %4lu @ %p ".as_ptr(),
        (*t).id as u64,
        t as *mut c_void,
    );

    if !(*t).label.is_null() {
        debugBelch(
            c"[\"%.*s\"] ".as_ptr(),
            (*(*t).label).bytes as i32,
            &raw mut (*(*t).label).payload as *mut StgWord as *mut c_char,
        );
    }

    match (*t).what_next as i32 {
        ThreadKilled => {
            debugBelch(c"has been killed".as_ptr());
        }
        ThreadComplete => {
            debugBelch(c"has completed".as_ptr());
        }
        _ => {
            printThreadBlockage(t);
        }
    }

    if (*t).dirty != 0 {
        debugBelch(c" (TSO_DIRTY)".as_ptr());
    }

    debugBelch(c"\n".as_ptr());
}

unsafe fn printAllThreads() {
    let mut t = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut i: u32 = 0;
    let mut g: u32 = 0;
    let mut cap = null_mut::<Capability>();
    debugBelch(c"all threads:\n".as_ptr());
    i = 0;

    while i < getNumCapabilities() as u32 {
        cap = getCapability(i);
        debugBelch(c"threads on capability %d:\n".as_ptr(), (*cap).no);
        t = (*cap).run_queue_hd;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            printThreadStatus(t);
            t = (*t)._link as *mut StgTSO;
        }

        i = i.wrapping_add(1);
    }

    debugBelch(c"other threads:\n".as_ptr());
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        t = (*generations.offset(g as isize)).threads;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            if (*t).why_blocked != NotBlocked as StgWord32 {
                printThreadStatus(t);
            }

            next = (*t).global_link as *mut StgTSO;
            t = next;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn printGlobalThreads() {
    let mut g: u32 = 0;

    while g < RtsFlags.GcFlags.generations {
        debugBelch(c"\ngen %d\n".as_ptr(), g);

        let mut t = (*generations.offset(g as isize)).threads;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            debugBelch(c"thread %p (id=%lu)\n".as_ptr(), t, (*t).id as u64);
            t = (*t).global_link as *mut StgTSO;
        }

        let mut t_0 = (*generations.offset(g as isize)).old_threads;

        while t_0 != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            debugBelch(
                c"thread %p (id=%lu) (old)\n".as_ptr(),
                t_0,
                (*t_0).id as u64,
            );
            t_0 = (*t_0).global_link as *mut StgTSO;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn printThreadQueue(mut t: *mut StgTSO) {
    let mut i: u32 = 0;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        printThreadStatus(t);
        i = i.wrapping_add(1);
        t = (*t)._link as *mut StgTSO;
    }

    debugBelch(c"%d threads on queue\n".as_ptr(), i);
}
