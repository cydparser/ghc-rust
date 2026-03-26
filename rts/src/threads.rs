use crate::alloc_array::allocateMutArrPtrs;
use crate::capability::Capability_;
use crate::check_vector_support::vectorSupportGlobalVar;
use crate::ffi::hs_ffi::{HS_BOOL_FALSE, HsBool};
use crate::ffi::rts::constants::{
    BlockedOnMVarRead, NotBlocked, RESERVED_STACK_WORDS, TSO_ALLOC_LIMIT, TSO_SQUEEZED,
    ThreadMigrating, ThreadRunGHC,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::block::{BLOCK_SIZE_W, round_to_mblocks};
use crate::ffi::rts::storage::closure_macros::{
    SET_INFO_RELAXED, UNTAG_CLOSURE, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{
    _StgMutArrPtrs, MessageBlackHole, MessageThrowTo, MessageThrowTo_, StgBlockingQueue,
    StgBlockingQueue_, StgInd, StgMVar, StgMVarTSOQueue, StgMVarTSOQueue_, StgMutArrPtrs,
    StgTRecHeader, StgTRecHeader_, StgUnderflowFrame,
};
use crate::ffi::rts::storage::gc::{allocate, g0, generations};
use crate::ffi::rts::storage::tso::{
    STACK_DIRTY, StgStack, StgStack_, StgTSO_, StgThreadID, dirty_STACK, setTSOLink,
};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, stg_BLACKHOLE_info, stg_BLOCKING_QUEUE_CLEAN_info,
    stg_BLOCKING_QUEUE_DIRTY_info, stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure, stg_IND_info,
    stg_MSG_NULL_info, stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info, stg_NO_TREC_closure,
    stg_STACK_info, stg_STM_AWOKEN_closure, stg_TSO_info, stg_WHITEHOLE_info, stg_ret_p_info,
    stg_stack_underflow_frame_d_info, stg_stack_underflow_frame_v16_info,
    stg_stack_underflow_frame_v32_info, stg_stack_underflow_frame_v64_info, stg_stop_thread_info,
};
use crate::ffi::stg::types::{StgBool, StgInt64, StgPtr, StgWord, StgWord8, StgWord16, StgWord32};
use crate::ffi::stg::{ASSIGN_Int64, P_, W_};
use crate::prelude::*;
use crate::raise_async::throwToSelf;
use crate::schedule::appendToRunQueue;
use crate::sm::storage::dirty_MVAR;
use crate::smp_closure_ops::{lockClosure, unlockClosure};
use crate::trace::{
    DEBUG_RTS, trace_, traceCap_, traceEventCreateThread, traceEventMigrateThread,
    traceEventThreadWakeup,
};
use crate::updates::updateWithIndirection;

#[cfg(test)]
mod tests;

static mut next_thread_id: StgThreadID = 1 as StgThreadID;

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
    let mut stack_size: uint32_t = 0;

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
    ) as uint32_t;

    stack = allocate(cap, stack_size as W_) as *mut StgStack;
    (*stack).header.info = &raw const stg_STACK_info;

    (*stack).stack_size = (stack_size as usize).wrapping_sub(
        (size_of::<StgStack>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize),
    ) as StgWord32;

    (*stack).sp =
        (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize) as StgPtr;
    (*stack).dirty = STACK_DIRTY as StgWord8;
    (*stack).marking = 0 as StgWord8;

    tso = allocate(
        cap,
        (size_of::<StgTSO>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut StgTSO;

    (*tso).header.info = &raw const stg_TSO_info;
    (*tso).what_next = ThreadRunGHC as StgWord16;
    (*tso).block_info.closure =
        &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgClosure;
    (*tso).why_blocked = NotBlocked as StgWord32;
    (*tso).blocked_exceptions = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        as *mut MessageThrowTo as *mut MessageThrowTo_;
    (*tso).bq = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        as *mut StgBlockingQueue as *mut StgBlockingQueue_;
    (*tso).flags = 0 as StgWord32;
    (*tso).dirty = 1 as StgWord32;
    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
    (*tso).saved_errno = 0 as StgWord32;
    (*tso).bound = null_mut::<InCall_>();
    (*tso).cap = cap as *mut Capability_;
    (*tso).stackobj = stack as *mut StgStack_;
    (*tso).tot_stack_size = (*stack).stack_size;
    ASSIGN_Int64(&raw mut (*tso).alloc_limit as *mut W_, 0 as StgInt64);
    (*tso).trec =
        &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader as *mut StgTRecHeader_;
    (*tso).label = null_mut::<StgArrBytes>();

    (*stack).sp = (*stack).sp.offset(
        -((size_of::<StgStopFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize),
    );

    let ref mut fresh1 = (*((*stack).sp as *mut StgClosure)).header.info;
    *fresh1 = &raw const stg_stop_thread_info as *mut StgInfoTable;

    let fresh2 = next_thread_id;
    next_thread_id = next_thread_id.wrapping_add(1);
    (*tso).id = fresh2;
    (*tso).global_link = (*g0).threads as *mut StgTSO_;
    (*g0).threads = tso;
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
        return 0 as c_int;
    }

    let mut id1: StgThreadID = (*(tso1 as *mut StgTSO)).id;
    let mut id2: StgThreadID = (*(tso2 as *mut StgTSO)).id;

    return if id1 < id2 { -(1 as c_int) } else { 1 as c_int };
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

                return r#false != 0;
            } else {
                *queue = (*t)._link as *mut StgTSO;
                (*t)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut StgTSO_;

                return r#true != 0;
            }
        }

        prev = t;
        t = (*t)._link as *mut StgTSO;
    }

    barf(b"removeThreadFromQueue: not found\0" as *const u8 as *const c_char);
}

unsafe fn removeThreadFromDeQueue(
    mut cap: *mut Capability,
    mut head: *mut *mut StgTSO,
    mut tail: *mut *mut StgTSO,
    mut tso: *mut StgTSO,
) -> bool {
    let mut t = null_mut::<StgTSO>();
    let mut prev = null_mut::<StgTSO>();
    let mut flag = r#false != 0;
    prev = null_mut::<StgTSO>();
    t = *head;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if t == tso {
            if !prev.is_null() {
                setTSOLink(cap, prev, (*t)._link as *mut StgTSO);
                flag = r#false != 0;
            } else {
                *head = (*t)._link as *mut StgTSO;
                flag = r#true != 0;
            }

            (*t)._link =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;

            if *tail == tso {
                if !prev.is_null() {
                    *tail = prev;
                } else {
                    *tail = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
                }

                return r#true != 0;
            } else {
                return flag;
            }
        }

        prev = t;
        t = (*t)._link as *mut StgTSO;
    }

    barf(b"removeThreadFromDeQueue: not found\0" as *const u8 as *const c_char);
}

unsafe fn tryWakeupThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    traceEventThreadWakeup(cap, tso, (*(*tso).cap).no);

    match (*tso).why_blocked {
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
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
                    traceCap_(
                        cap,
                        b"thread %llu still blocked on throwto (%p)\0" as *const u8 as *const c_char
                            as *mut c_char,
                        (*tso).id,
                        (*(*tso).block_info.throwto).header.info,
                    );
                }

                return;
            }

            (*(*tso).stackobj).sp = (*(*tso).stackobj).sp.offset(3 as c_int as isize);
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
    msg = (*bq).queue as *mut MessageBlackHole;

    while msg
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut MessageBlackHole
    {
        i = (*msg).header.info;

        if i != &raw const stg_IND_info {
            tryWakeupThread(cap, (*msg).tso);
        }

        msg = (*msg).link as *mut MessageBlackHole;
    }

    SET_INFO_RELAXED(bq as *mut StgClosure, &raw const stg_IND_info);
}

unsafe fn checkBlockingQueues(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut bq = null_mut::<StgBlockingQueue>();
    let mut next = null_mut::<StgBlockingQueue>();
    let mut p = null_mut::<StgClosure>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        traceCap_(
            cap,
            b"collision occurred; checking blocking queues for thread %llu\0" as *const u8
                as *const c_char as *mut c_char,
            (*tso).id,
        );
    }

    bq = (*tso).bq as *mut StgBlockingQueue;

    while bq
        != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgBlockingQueue
    {
        next = (*bq).link as *mut StgBlockingQueue;

        let mut bqinfo = (*bq).header.info;

        if !(bqinfo == &raw const stg_IND_info) {
            p = UNTAG_CLOSURE((*bq).bh);

            let mut pinfo = (*p).header.info;

            if pinfo != &raw const stg_BLACKHOLE_info
                || (*(p as *mut StgInd)).indirectee != bq as *mut StgClosure
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
    i = (*thunk).header.info;

    if i != &raw const stg_BLACKHOLE_info
        && i != &raw const stg_CAF_BLACKHOLE_info
        && i != &raw const __stg_EAGER_BLACKHOLE_info
        && i != &raw const stg_WHITEHOLE_info
    {
        updateWithIndirection(cap, thunk, val);
        return;
    }

    v = UNTAG_CLOSURE((*(thunk as *mut StgInd)).indirectee);
    updateWithIndirection(cap, thunk, val);

    if v as *mut StgTSO == tso {
        return;
    }

    i = (*v).header.info;

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
    return HS_BOOL_FALSE as HsBool;
}

unsafe fn isThreadBound(mut tso: *mut StgTSO) -> StgBool {
    return r#false;
}

unsafe fn threadStackOverflow(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut new_stack = null_mut::<StgStack>();
    let mut old_stack = null_mut::<StgStack>();
    let mut frame = null_mut::<StgUnderflowFrame>();
    let mut chunk_size: W_ = 0;

    if RtsFlags.GcFlags.maxStkSize > 0 as uint32_t
        && (*tso).tot_stack_size >= RtsFlags.GcFlags.maxStkSize
    {
        if (*tso).flags & TSO_SQUEEZED as StgWord32 != 0 {
            return;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
            trace_(
                b"threadStackOverflow of TSO %llu (%p): stack too large (now %ld; max is %ld)\0"
                    as *const u8 as *const c_char as *mut c_char,
                (*tso).id,
                tso,
                (*(*tso).stackobj).stack_size as c_long,
                RtsFlags.GcFlags.maxStkSize,
            );
        }

        throwToSelf(cap, tso, (*ghc_hs_iface).stackOverflow_closure);
        return;
    }

    if (*tso).flags & TSO_SQUEEZED as StgWord32 != 0
        && (*(*tso).stackobj)
            .sp
            .offset_from(&raw mut (*(*tso).stackobj).stack as *mut StgWord) as c_long
            as W_
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
            let mut _a = (2 as c_ulong).wrapping_mul(
                ((*old_stack).stack_size as c_ulong).wrapping_add(
                    (size_of::<StgStack>() as c_ulong)
                        .wrapping_add(size_of::<W_>() as c_ulong)
                        .wrapping_sub(1 as c_ulong)
                        .wrapping_div(size_of::<W_>() as c_ulong),
                ),
            );

            let mut _b = RtsFlags.GcFlags.stkChunkSize as c_ulong;

            if _a <= _b {
                _b as c_ulong
            } else {
                _a as c_ulong
            }
        }) as W_;
    } else {
        chunk_size = RtsFlags.GcFlags.stkChunkSize as W_;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        traceCap_(
            cap,
            b"allocating new stack chunk of size %d bytes\0" as *const u8 as *const c_char
                as *mut c_char,
            chunk_size.wrapping_mul(size_of::<W_>() as W_),
        );
    }

    (*cap).r.rCurrentTSO = tso as *mut StgTSO_;
    new_stack = allocate(cap, chunk_size) as *mut StgStack;
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
    (*new_stack).header.info = &raw const stg_STACK_info;
    (*new_stack).dirty = 0 as StgWord8;
    (*new_stack).marking = 0 as StgWord8;

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

    chunk_words = sp.offset_from((*old_stack).sp) as c_long as W_;

    memcpy(
        (*new_stack).sp.offset(-(chunk_words as isize)) as *mut c_void,
        (*old_stack).sp as *const c_void,
        chunk_words.wrapping_mul(size_of::<W_>() as W_) as size_t,
    );

    (*old_stack).sp = (*old_stack).sp.offset(chunk_words as isize);
    (*new_stack).sp = (*new_stack).sp.offset(-(chunk_words as isize));
    (*tso).stackobj = new_stack as *mut StgStack_;
    dirty_STACK(cap, new_stack);
}

unsafe fn threadStackUnderflow(mut cap: *mut Capability, mut tso: *mut StgTSO) -> W_ {
    let mut new_stack = null_mut::<StgStack>();
    let mut old_stack = null_mut::<StgStack>();
    let mut frame = null_mut::<StgUnderflowFrame>();
    let mut retvals: uint32_t = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        traceCap_(
            cap,
            b"stack underflow\0" as *const u8 as *const c_char as *mut c_char,
        );
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

    new_stack = (*frame).next_chunk as *mut StgStack;
    (*tso).stackobj = new_stack as *mut StgStack_;
    retvals = (frame as P_).offset_from((*old_stack).sp) as c_long as uint32_t;

    if retvals != 0 as uint32_t {
        if ((*new_stack)
            .sp
            .offset_from(&raw mut (*new_stack).stack as *mut StgWord) as c_long as W_)
            < retvals as W_
        {
            barf(
                b"threadStackUnderflow: not enough space for return values\0" as *const u8
                    as *const c_char,
            );
        }

        memcpy(
            (*new_stack).sp.offset(-(retvals as isize)) as *mut c_void,
            (*old_stack).sp as *const c_void,
            (retvals as size_t).wrapping_mul(size_of::<W_>() as size_t),
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
        return r#false != 0;
    }

    q = (*mvar).head as *mut StgMVarTSOQueue;

    loop {
        if q == &raw mut stg_END_TSO_QUEUE_closure as *mut StgMVarTSOQueue {
            if info == &raw const stg_MVAR_CLEAN_info {
                dirty_MVAR(&raw mut (*cap).r, mvar as *mut StgClosure, (*mvar).value);
            }

            (*mvar).value = value;
            unlockClosure(mvar as *mut StgClosure, &raw const stg_MVAR_DIRTY_info);

            return r#true != 0;
        }

        qinfo = (*q).header.info;

        if qinfo == &raw const stg_IND_info || qinfo == &raw const stg_MSG_NULL_info {
            q = (*(q as *mut StgInd)).indirectee as *mut StgMVarTSOQueue;
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

            why_blocked = (*tso).why_blocked as StgWord;
            stack = (*tso).stackobj as *mut StgStack;
            *(*stack).sp.offset(1 as c_int as isize) = value as W_ as StgWord;
            *(*stack).sp.offset(0 as c_int as isize) = &raw const stg_ret_p_info as W_ as StgWord;
            (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut StgTSO as *mut StgTSO_;

            if (*stack).dirty as c_int & STACK_DIRTY == 0 as c_int {
                dirty_STACK(cap, stack);
            }

            tryWakeupThread(cap, tso);

            if !(why_blocked == BlockedOnMVarRead as StgWord) {
                break;
            }
        }
    }

    unlockClosure(mvar as *mut StgClosure, info);

    return r#true != 0;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn listThreads(mut cap: *mut Capability) -> *mut _StgMutArrPtrs {
    let mut i: StgWord = 0;
    let mut n_threads: StgWord = 0 as StgWord;
    let mut g = 0 as c_uint;

    while (g as uint32_t) < RtsFlags.GcFlags.generations {
        let mut t = (*generations.offset(g as isize)).threads;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            n_threads = n_threads.wrapping_add(1);
            t = (*t).global_link as *mut StgTSO;
        }

        g = g.wrapping_add(1);
    }

    let mut arr = allocateMutArrPtrs(cap, n_threads, (*cap).r.rCCCS as *mut CostCentreStack);

    if !((arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as c_int as c_long != 0) {
        i = 0 as StgWord;

        let mut g_0 = 0 as c_uint;

        while (g_0 as uint32_t) < RtsFlags.GcFlags.generations {
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

        if (i == n_threads) as c_int as c_long != 0 {
        } else {
            barf(b"listThreads: Found too few threads\0" as *const u8 as *const c_char);
        }
    }

    return arr as *mut _StgMutArrPtrs;
}
