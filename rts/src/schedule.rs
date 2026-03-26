use crate::capability::{
    Capability_, contextSwitchCapability, freeCapabilities, getCapability, initCapabilities,
    interruptAllCapabilities, regTableToCapability, releaseCapability, releaseCapability_,
    shutdownCapabilities, waitForCapability,
};
use crate::eventlog::event_log::flushAllCapsEventsBufs;
use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::rts::constants::{
    BlockedOnBlackHole, BlockedOnCCall, BlockedOnCCall_Interruptible, NotBlocked, StackOverflow,
    TSO_ALLOC_LIMIT, TSO_BLOCKEX, TSO_INTERRUPTIBLE, TSO_LOCKED, ThreadBlocked, ThreadComplete,
    ThreadFinished, ThreadKilled,
};
use crate::ffi::rts::event_log_format::THREAD_SUSPENDED_FOREIGN_CALL;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::storage::block::{
    BLOCK_MASK, BLOCK_SIZE, BLOCKS_PER_MBLOCK, Bdescr, MBLOCK_MASK, MBLOCK_SIZE,
    allocGroupOnNode_lock, bdescr, bdescr_, dbl_link_insert_after,
};
use crate::ffi::rts::storage::closure_macros::{get_ret_itbl, stack_frame_sizeW};
use crate::ffi::rts::storage::closure_types::{
    ATOMICALLY_FRAME, CATCH_FRAME, CATCH_RETRY_FRAME, CATCH_STM_FRAME, STOP_FRAME,
};
use crate::ffi::rts::storage::closures::{
    MessageThrowTo, StgClosure_, StgDeadThreadFrame, StgTRecHeader, StgTRecHeader_, StgThunk,
    StgUpdateFrame,
};
use crate::ffi::rts::storage::gc::{allocate, g0, generation, generations, initBdescr, memcount};
use crate::ffi::rts::storage::m_block::mblocks_allocated;
use crate::ffi::rts::storage::tso::{
    StgStack, StgTSO_, StgThreadID, StgThreadReturnCode, dirty_STACK, dirty_TSO, setTSOLink,
    setTSOPrev,
};
use crate::ffi::rts::threads::{createIOThread, n_capabilities};
use crate::ffi::rts::time::Time;
use crate::ffi::rts::timer::{startTimer, stopTimer};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::{
    Capability, HaskellObj, HeapExhausted, Interrupted, Killed, NoStatus, Success, getAllocations,
    rts_apply, rts_checkSchedStatus, rts_evalStableIOMain, rts_unlock, shutdownHaskellAndExit,
};
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::misc_closures::{
    stg_END_TSO_QUEUE_closure, stg_NO_TREC_closure, stg_maskAsyncExceptionszh_ret_info,
    stg_maskUninterruptiblezh_ret_info, stg_raise_info, stg_returnToStackTop,
    stg_unmaskAsyncExceptionszh_ret_info,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{StgFunPtr, StgInt64, StgPtr, StgWord, StgWord16, StgWord32};
use crate::ffi::stg::{ASSIGN_Int64, PK_Int64, W_};
use crate::interpreter::interpretBCO;
use crate::io_manager::{
    anyPendingTimeoutsOrIO, awaitCompletedTimeoutsOrIO, initIOManagerAfterFork,
    pollCompletedTimeoutsOrIO,
};
use crate::messages::blackHoleOwner;
use crate::posix::signals::{
    anyUserHandlers, next_pending_handler, pending_handler_buf, startSignalHandlers,
};
use crate::prelude::*;
use crate::proftimer::{pauseHeapProfTimer, performHeapProfile, resumeHeapProfTimer};
use crate::raise_async::{
    awakenBlockedExceptionQueue, maybePerformBlockedException, throwToSelf, throwToSingleThreaded,
    throwToSingleThreaded_,
};
use crate::rts_signals::awaitUserSignals;
use crate::schedule::{
    ACTIVITY_DONE_GC, ACTIVITY_YES, RecentActivity, SCHED_INTERRUPTING, SCHED_RUNNING,
    SCHED_SHUTTING_DOWN, SchedState, emptyRunQueue, getRecentActivity, getSchedState,
    setRecentActivity, setSchedState, truncateRunQueue,
};
use crate::sm::gc::{GarbageCollect, GcConfig, doIdleGCWork};
use crate::sm::storage::{calcNeeded, doYouWantToGC, finishedNurseryBlock};
use crate::stats::resetChildProcessStats;
use crate::stg_run::StgRun;
use crate::stm::{stmAbortTransaction, stmFreeAbortedTRec, stmValidateNestOfTransactions};
use crate::task::{
    InCall, InCall_, Task, discardTasksExcept, exitMyTask, freeTaskManager, initTaskManager,
    isBoundTask, newBoundTask,
};
use crate::thread_labels::setThreadLabel;
use crate::thread_paused::threadPaused;
use crate::threads::{threadStackOverflow, threadStackUnderflow, updateThunk};
use crate::timer::initTimer;
use crate::top_handler::getTopHandlerThread;
use crate::trace::{
    DEBUG_RTS, resetTracing, trace_, traceEventRunThread, traceEventStopThread, traceSparkCounters,
    traceTaskCreate,
};

#[cfg(test)]
mod tests;

pub(crate) type SchedState = c_uint;

pub(crate) const SCHED_SHUTTING_DOWN: SchedState = 2;

pub(crate) const SCHED_INTERRUPTING: SchedState = 1;

pub(crate) const SCHED_RUNNING: SchedState = 0;

pub(crate) const ACTIVITY_YES: RecentActivity = 0;

pub(crate) const ACTIVITY_DONE_GC: RecentActivity = 3;

pub(crate) type RecentActivity = c_uint;

pub(crate) const ACTIVITY_INACTIVE: RecentActivity = 2;

pub(crate) const ACTIVITY_MAYBE_NO: RecentActivity = 1;

#[inline]
pub(crate) unsafe fn setSchedState(mut ss: SchedState) {
    ::core::intrinsics::atomic_store_seqcst(&raw mut sched_state, ss as StgWord);
}

#[inline]
pub(crate) unsafe fn getSchedState() -> SchedState {
    return ::core::intrinsics::atomic_load_seqcst(&raw mut sched_state) as SchedState;
}

#[inline]
pub(crate) unsafe fn setRecentActivity(mut new_value: RecentActivity) -> RecentActivity {
    let mut old: StgWord =
        ::core::intrinsics::atomic_xchg_seqcst(&raw mut recent_activity, new_value as StgWord);

    return old as RecentActivity;
}

#[inline]
pub(crate) unsafe fn getRecentActivity() -> RecentActivity {
    return ::core::intrinsics::atomic_load_relaxed(&raw mut recent_activity) as RecentActivity;
}

#[inline]
pub(crate) unsafe fn peekRunQueue(mut cap: *mut Capability) -> *mut StgTSO {
    return (*cap).run_queue_hd;
}

#[inline]
pub(crate) unsafe fn emptyRunQueue(mut cap: *mut Capability) -> bool {
    return (*cap).n_run_queue == 0 as uint32_t;
}

#[inline]
pub(crate) unsafe fn truncateRunQueue(mut cap: *mut Capability) {
    (*cap).run_queue_hd = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).run_queue_tl = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).n_run_queue = 0 as uint32_t;
}

static mut allocated_bytes_at_heapoverflow: uint64_t = 0 as uint64_t;

static mut heap_overflow: bool = r#false != 0;

static mut recent_activity: StgWord = ACTIVITY_YES as c_int as StgWord;

static mut sched_state: StgWord = SCHED_RUNNING as c_int as StgWord;

static mut allocLimitKill: bool = r#true != 0;

static mut allocLimitRunHook: bool = r#false != 0;

unsafe fn schedule(mut initialCapability: *mut Capability, mut task: *mut Task) -> *mut Capability {
    let mut t = null_mut::<StgTSO>();
    let mut cap = null_mut::<Capability>();
    let mut ret: StgThreadReturnCode = 0;
    let mut prev_what_next: uint32_t = 0;
    let mut ready_to_gc: bool = false;
    cap = initialCapability;
    t = null_mut::<StgTSO>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"cap %d: schedule()\0" as *const u8 as *const c_char as *mut c_char,
            (*initialCapability).no,
        );
    }

    loop {
        if (*cap).in_haskell {
            errorBelch(
                b"schedule: re-entered unsafely.\n   Perhaps a 'foreign import unsafe' should be 'safe'?\0"
                    as *const u8 as *const c_char,
            );

            stg_exit(EXIT_FAILURE);
        }

        let mut current_block_59: u64;

        match getSchedState() as c_uint {
            0 => {
                current_block_59 = 2516253395664191498;
            }
            1 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
                    trace_(b"SCHED_INTERRUPTING\0" as *const u8 as *const c_char as *mut c_char);
                }

                scheduleDoGC(
                    &raw mut cap,
                    task,
                    r#true != 0,
                    r#false != 0,
                    r#false != 0,
                    r#false != 0,
                );

                current_block_59 = 1338095667010778376;
            }
            2 => {
                current_block_59 = 1338095667010778376;
            }
            _ => {
                barf(
                    b"sched_state: %llu\0" as *const u8 as *const c_char,
                    sched_state,
                );
            }
        }

        match current_block_59 {
            1338095667010778376 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
                    trace_(b"SCHED_SHUTTING_DOWN\0" as *const u8 as *const c_char as *mut c_char);
                }

                if !isBoundTask(task) && emptyRunQueue(cap) as c_int != 0 {
                    return cap;
                }
            }
            _ => {}
        }

        scheduleFindWork(&raw mut cap);
        schedulePushWork(cap, task);
        scheduleDetectDeadlock(&raw mut cap, task);
        emptyRunQueue(cap);
        t = popRunQueue(cap);

        if getSchedState() as c_uint >= SCHED_INTERRUPTING as c_int as c_uint
            && !((*t).what_next as c_int == ThreadComplete
                || (*t).what_next as c_int == ThreadKilled)
        {
            deleteThread(t);
        }

        if RtsFlags.ConcFlags.ctxtSwitchTicks == 0 as c_int
            && (!emptyRunQueue(cap) || anyPendingTimeoutsOrIO(cap) as c_int != 0)
        {
            (*cap).context_switch = 1 as c_int;
        }

        loop {
            (*cap).r.rCurrentTSO = t as *mut StgTSO_;
            resumeHeapProfTimer();
            prev_what_next = (*t).what_next as uint32_t;
            *__error() = (*t).saved_errno as c_int;
            (*cap).interrupt = 0 as c_int;
            (*cap).in_haskell = r#true != 0;
            (*cap).idle = 0 as uint32_t;
            dirty_TSO(cap, t);
            dirty_STACK(cap, (*t).stackobj as *mut StgStack);

            match getRecentActivity() as c_uint {
                3 => {
                    let mut prev: uint32_t = 0;
                    prev = setRecentActivity(ACTIVITY_YES) as uint32_t;

                    if prev == ACTIVITY_DONE_GC as c_int as uint32_t {
                        startTimer();
                    }
                }
                2 => {}
                _ => {
                    setRecentActivity(ACTIVITY_YES);
                }
            }

            traceEventRunThread(cap, t);

            match prev_what_next {
                3 | 4 => {
                    ret = ThreadFinished as StgThreadReturnCode;
                }
                1 => {
                    let mut r = null_mut::<StgRegTable>();

                    r = StgRun(
                        transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(Some(
                            stg_returnToStackTop as unsafe extern "C" fn() -> StgFunPtr,
                        )),
                        &raw mut (*cap).r,
                    );

                    cap = regTableToCapability(r);
                    ret = (*r).rRet as StgThreadReturnCode;
                }
                2 => {
                    cap = interpretBCO(cap);
                    ret = (*cap).r.rRet as StgThreadReturnCode;
                }
                _ => {
                    barf(
                        b"schedule: invalid prev_what_next=%u field\0" as *const u8
                            as *const c_char,
                        prev_what_next,
                    );
                }
            }

            (*cap).in_haskell = r#false != 0;
            t = (*cap).r.rCurrentTSO as *mut StgTSO;
            (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
            (*t).saved_errno = *__error() as StgWord32;

            if ret == ThreadBlocked as StgThreadReturnCode {
                let mut why_blocked: uint16_t = (*t).why_blocked as uint16_t;

                if why_blocked as c_int == BlockedOnBlackHole {
                    let mut owner = blackHoleOwner((*(*t).block_info.bh).bh);

                    traceEventStopThread(
                        cap,
                        t,
                        ((*t).why_blocked as StgThreadReturnCode)
                            .wrapping_add(6 as StgThreadReturnCode),
                        (if !owner.is_null() {
                            (*owner).id
                        } else {
                            0 as StgThreadID
                        }) as StgWord32,
                    );
                } else {
                    traceEventStopThread(
                        cap,
                        t,
                        ((*t).why_blocked as StgThreadReturnCode)
                            .wrapping_add(6 as StgThreadReturnCode),
                        0 as StgWord32,
                    );
                }
            } else if ret == StackOverflow as StgThreadReturnCode {
                traceEventStopThread(cap, t, ret, (*t).tot_stack_size);
            } else {
                traceEventStopThread(cap, t, ret, 0 as StgWord32);
            }

            pauseHeapProfTimer();
            schedulePostRunThread(cap, t);
            ready_to_gc = r#false != 0;

            match ret {
                1 => {
                    ready_to_gc = scheduleHandleHeapOverflow(cap, t);
                    break;
                }
                2 => {
                    threadStackOverflow(cap, t);
                    pushOnRunQueue(cap, t);
                    break;
                }
                3 => {
                    if !scheduleHandleYield(cap, t, prev_what_next) {
                        break;
                    }
                }
                4 => {
                    scheduleHandleThreadBlocked(t);
                    break;
                }
                5 => {
                    if scheduleHandleThreadFinished(cap, task, t) {
                        return cap;
                    }

                    break;
                }
                _ => {
                    barf(
                        b"schedule: invalid thread return code %d\0" as *const u8 as *const c_char,
                        ret as c_int,
                    );
                }
            }
        }

        if ready_to_gc as c_int != 0 || scheduleNeedHeapProfile(ready_to_gc) as c_int != 0 {
            scheduleDoGC(
                &raw mut cap,
                task,
                r#false != 0,
                ready_to_gc,
                r#false != 0,
                r#false != 0,
            );
        }
    }
}

unsafe fn removeFromRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if (*tso).block_info.prev == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*cap).run_queue_hd = (*tso)._link as *mut StgTSO;
    } else {
        setTSOLink(cap, (*tso).block_info.prev, (*tso)._link as *mut StgTSO);
    }

    if (*tso)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*cap).run_queue_tl = (*tso).block_info.prev;
    } else {
        setTSOPrev(cap, (*tso)._link as *mut StgTSO, (*tso).block_info.prev);
    }

    (*tso).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*tso)._link = (*tso).block_info.prev as *mut StgTSO_;
    (*cap).n_run_queue = (*cap).n_run_queue.wrapping_sub(1);
}

unsafe fn promoteInRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    removeFromRunQueue(cap, tso);
    pushOnRunQueue(cap, tso);
}

unsafe fn scheduleFindWork(mut pcap: *mut *mut Capability) {
    scheduleStartSignalHandlers(*pcap);
    scheduleProcessInbox(pcap);
    scheduleCheckBlockedThreads(*pcap);
}

unsafe fn schedulePushWork(mut cap: *mut Capability, mut task: *mut Task) {}

unsafe fn scheduleStartSignalHandlers(mut cap: *mut Capability) {
    if RtsFlags.MiscFlags.install_signal_handlers as c_int != 0
        && next_pending_handler != &raw mut pending_handler_buf as *mut siginfo_t
    {
        startSignalHandlers(cap);
    }
}

unsafe fn scheduleCheckBlockedThreads(mut cap: *mut Capability) {
    if anyPendingTimeoutsOrIO(cap) {
        if emptyRunQueue(cap) {
            awaitCompletedTimeoutsOrIO(cap);
        } else {
            pollCompletedTimeoutsOrIO(cap);
        }
    }
}

unsafe fn scheduleDetectDeadlock(mut pcap: *mut *mut Capability, mut task: *mut Task) {
    let mut cap = *pcap;

    if emptyRunQueue(cap) as c_int != 0 && !anyPendingTimeoutsOrIO(cap) {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
            trace_(
                b"deadlocked, forcing major GC...\0" as *const u8 as *const c_char as *mut c_char,
            );
        }

        scheduleDoGC(
            pcap,
            task,
            r#true != 0,
            r#false != 0,
            r#true != 0,
            r#false != 0,
        );

        cap = *pcap;

        if !emptyRunQueue(cap) {
            return;
        }

        if RtsFlags.MiscFlags.install_signal_handlers as c_int != 0
            && anyUserHandlers() as c_int != 0
        {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
                trace_(
                    b"still deadlocked, waiting for signals...\0" as *const u8 as *const c_char
                        as *mut c_char,
                );
            }

            awaitUserSignals();

            if next_pending_handler != &raw mut pending_handler_buf as *mut siginfo_t {
                startSignalHandlers(cap);
            }

            return;
        }
    }
}

unsafe fn scheduleProcessInbox(mut pcap: *mut *mut Capability) {}

unsafe fn schedulePostRunThread(mut cap: *mut Capability, mut t: *mut StgTSO) {
    if (*t).trec != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader
        && (*t).why_blocked == NotBlocked as StgWord32
    {
        if stmValidateNestOfTransactions(cap, (*t).trec as *mut StgTRecHeader, r#true) == 0 {
            if DEBUG_RTS != 0
                && (RtsFlags.DebugFlags.scheduler as c_int | RtsFlags.DebugFlags.stm as c_int)
                    as c_long
                    != 0
            {
                trace_(
                    b"trec %p found wasting its time\0" as *const u8 as *const c_char
                        as *mut c_char,
                    t,
                );
            }

            throwToSingleThreaded_(cap, t, null_mut::<StgClosure>(), r#true != 0);
        }
    }

    if PK_Int64(&raw mut (*t).alloc_limit as *mut W_) < 0 as StgInt64
        && (*t).flags & TSO_ALLOC_LIMIT as StgWord32 != 0
    {
        if allocLimitKill {
            throwToSelf(cap, t, (*ghc_hs_iface).allocationLimitExceeded_closure);

            ASSIGN_Int64(
                &raw mut (*t).alloc_limit as *mut W_,
                (RtsFlags.GcFlags.allocLimitGrace as StgInt64 as c_ulonglong)
                    .wrapping_mul(BLOCK_SIZE as c_ulonglong) as StgInt64,
            );
        } else {
            (*t).flags = (*t).flags & !TSO_ALLOC_LIMIT as StgWord32;
        }

        if allocLimitRunHook {
            let mut c = rts_apply(
                cap,
                (*ghc_hs_iface).runAllocationLimitHandler_closure as HaskellObj,
                t as HaskellObj,
            ) as *mut StgClosure;

            let mut hookThread = createIOThread(cap, RtsFlags.GcFlags.initialStkSize as W_, c);

            setThreadLabel(
                cap,
                hookThread,
                b"allocation limit handler thread\0" as *const u8 as *const c_char as *mut c_char,
            );

            pushOnRunQueue(cap, hookThread);
        }
    }
}

unsafe fn scheduleHandleHeapOverflow(mut cap: *mut Capability, mut t: *mut StgTSO) -> bool {
    if ::core::intrinsics::atomic_load_relaxed(&raw mut (*cap).r.rHpLim).is_null()
        || ::core::intrinsics::atomic_load_relaxed(&raw mut (*cap).context_switch) != 0
    {
        ::core::intrinsics::atomic_store_relaxed(&raw mut (*cap).context_switch, 0 as c_int);
        appendToRunQueue(cap, t);
    } else {
        pushOnRunQueue(cap, t);
    }

    if (*cap).r.rHpAlloc > BLOCK_SIZE as StgWord {
        let mut bd = null_mut::<bdescr>();
        let mut blocks: W_ = 0;
        blocks = ((*cap)
            .r
            .rHpAlloc
            .wrapping_add(BLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !BLOCK_MASK as W_)
            .wrapping_div(BLOCK_SIZE as W_);

        if blocks > BLOCKS_PER_MBLOCK {
            barf(
                b"allocation of %ld bytes too large (GHC should have complained at compile-time)\0"
                    as *const u8 as *const c_char,
                (*cap).r.rHpAlloc as c_long,
            );
        }

        if !(*(*cap).r.rCurrentNursery).link.is_null()
            || (*(*cap).r.rNursery).n_blocks == 1 as memcount
        {
            bd = allocGroupOnNode_lock((*cap).node, blocks);
            (*(*cap).r.rNursery).n_blocks = ((*(*cap).r.rNursery).n_blocks as StgWord)
                .wrapping_add(blocks as StgWord)
                as memcount as memcount;
            dbl_link_insert_after(bd, (*cap).r.rCurrentNursery as *mut bdescr);

            let mut x = null_mut::<bdescr>();
            x = bd;

            while x < bd.offset(blocks as isize) {
                initBdescr(x, g0, g0);
                (*x).c2rust_unnamed.free = (*x).start;
                (*x).flags = 0 as StgWord16;
                x = x.offset(1);
            }

            finishedNurseryBlock(cap, (*cap).r.rCurrentNursery as *mut bdescr);
            (*cap).r.rCurrentNursery = bd as *mut bdescr_;

            return r#false != 0;
        }
    }

    return doYouWantToGC(cap);
}

unsafe fn scheduleHandleYield(
    mut cap: *mut Capability,
    mut t: *mut StgTSO,
    mut prev_what_next: uint32_t,
) -> bool {
    if (*t).what_next as uint32_t != prev_what_next {
        return r#true != 0;
    }

    if ::core::intrinsics::atomic_load_relaxed(&raw mut (*cap).context_switch) != 0 as c_int {
        ::core::intrinsics::atomic_store_relaxed(&raw mut (*cap).context_switch, 0 as c_int);
        appendToRunQueue(cap, t);
    } else {
        pushOnRunQueue(cap, t);
    }

    return r#false != 0;
}

unsafe fn scheduleHandleThreadBlocked(mut t: *mut StgTSO) {}

unsafe fn scheduleHandleThreadFinished(
    mut cap: *mut Capability,
    mut task: *mut Task,
    mut t: *mut StgTSO,
) -> bool {
    awakenBlockedExceptionQueue(cap, t);

    if !(*t).bound.is_null() {
        if (*t).bound != (*task).incall {
            appendToRunQueue(cap, t);

            return r#false != 0;
        }

        if (*t).what_next as c_int == ThreadComplete {
            if !(*(*task).incall).ret.is_null() {
                let mut dead = (*(*(*(*task).incall).tso).stackobj)
                    .sp
                    .offset(0 as c_int as isize) as *mut StgWord
                    as *mut StgDeadThreadFrame;
                *(*(*task).incall).ret = (*dead).result;
            }

            (*(*task).incall).rstat = Success;
        } else {
            if !(*(*task).incall).ret.is_null() {
                *(*(*task).incall).ret = null_mut::<StgClosure>();
            }

            if getSchedState() as c_uint >= SCHED_INTERRUPTING as c_int as c_uint {
                if heap_overflow {
                    (*(*task).incall).rstat = HeapExhausted;
                } else {
                    (*(*task).incall).rstat = Interrupted;
                }
            } else {
                (*(*task).incall).rstat = Killed;
            }
        }

        (*t).bound = null_mut::<InCall_>();
        (*(*task).incall).tso = null_mut::<StgTSO>();

        return r#true != 0;
    }

    return r#false != 0;
}

unsafe fn scheduleNeedHeapProfile(mut ready_to_gc: bool) -> bool {
    if performHeapProfile as c_int != 0
        || RtsFlags.ProfFlags.heapProfileInterval == 0 as Time
            && RtsFlags.ProfFlags.doHeapProfile != 0
            && ready_to_gc as c_int != 0
    {
        return r#true != 0;
    } else {
        return r#false != 0;
    };
}

unsafe fn scheduleDoGC(
    mut pcap: *mut *mut Capability,
    mut task: *mut Task,
    mut force_major: bool,
    mut is_overflow_gc: bool,
    mut deadlock_detect: bool,
    mut nonconcurrent: bool,
) {
    let mut config: GcConfig = GcConfig {
        collect_gen: 0,
        do_heap_census: false,
        overflow_gc: false,
        deadlock_detect: false,
        nonconcurrent: false,
        parallel: false,
    };

    let mut cap = *pcap;
    let mut heap_census: bool = false;
    let mut collect_gen: uint32_t = 0;
    let mut major_gc: bool = false;

    if getSchedState() as c_uint == SCHED_SHUTTING_DOWN as c_int as c_uint {
        return;
    }

    heap_census = scheduleNeedHeapProfile(r#true != 0);

    let mut mblock_overflow = RtsFlags.GcFlags.maxHeapSize != 0 as uint32_t
        && mblocks_allocated
            > (1 as W_).wrapping_add(
                (((RtsFlags.GcFlags.maxHeapSize as W_)
                    .wrapping_sub(
                        (((1 as c_ulong) << 20 as c_int) as W_)
                            .wrapping_sub(
                                ((0x40 as c_ulong).wrapping_mul(
                                    ((1 as c_ulong) << 20 as c_int)
                                        .wrapping_div((1 as c_ulong) << 12 as c_int),
                                ) as W_)
                                    .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                    .wrapping_sub(1 as W_)
                                    & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong)
                                        as W_,
                            )
                            .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                    )
                    .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
                    .wrapping_add(MBLOCK_SIZE as W_)
                    .wrapping_sub(1 as W_)
                    & !MBLOCK_MASK as W_) as *mut c_void as W_)
                    .wrapping_div(MBLOCK_SIZE as W_),
            );

    collect_gen = calcNeeded(
        force_major as c_int != 0 || heap_census as c_int != 0 || mblock_overflow as c_int != 0,
        null_mut::<StgWord>(),
    ) as uint32_t;

    major_gc = collect_gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t);

    loop {
        if getSchedState() as c_uint == SCHED_INTERRUPTING as c_int as c_uint
            && major_gc as c_int != 0
        {
            deleteAllThreads();
            setSchedState(SCHED_SHUTTING_DOWN);
        }

        doIdleGCWork(cap, r#true != 0);

        config = GcConfig {
            collect_gen: collect_gen,
            do_heap_census: heap_census,
            overflow_gc: is_overflow_gc,
            deadlock_detect: deadlock_detect,
            nonconcurrent: nonconcurrent,
            parallel: false,
        };

        GarbageCollect(config, cap, null_mut::<bool>());

        if getSchedState() as c_uint == SCHED_SHUTTING_DOWN as c_int as c_uint {
            doIdleGCWork(cap, r#true != 0);
        }

        traceSparkCounters(cap);

        let mut current_block_20: u64;

        match getRecentActivity() as c_uint {
            2 => {
                if force_major {
                    setRecentActivity(ACTIVITY_DONE_GC);
                    stopTimer();
                    current_block_20 = 2668756484064249700;
                } else {
                    current_block_20 = 5199047908754221960;
                }
            }
            1 => {
                current_block_20 = 5199047908754221960;
            }
            3 | 0 | _ => {
                current_block_20 = 2668756484064249700;
            }
        }

        match current_block_20 {
            5199047908754221960 => {
                setRecentActivity(ACTIVITY_YES);
            }
            _ => {}
        }

        if heap_census {
            performHeapProfile = 0 as c_int != 0;
        }

        if !(heap_overflow as c_int != 0
            && getSchedState() as c_uint == SCHED_RUNNING as c_int as c_uint)
        {
            break;
        }

        let mut main_thread = getTopHandlerThread();

        if main_thread.is_null() {
            setSchedState(SCHED_INTERRUPTING);
        } else {
            heap_overflow = r#false != 0;

            let allocation_count = getAllocations() as uint64_t;

            if RtsFlags.GcFlags.heapLimitGrace
                < allocation_count.wrapping_sub(allocated_bytes_at_heapoverflow)
                || allocated_bytes_at_heapoverflow == 0 as uint64_t
            {
                allocated_bytes_at_heapoverflow = allocation_count;
                throwToSelf(cap, main_thread, (*ghc_hs_iface).heapOverflow_closure);
            }

            break;
        }
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn forkProcess(mut entry: *mut HsStablePtr) -> pid_t {
    let mut pid: pid_t = 0;
    let mut t = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut cap = null_mut::<Capability>();
    let mut g: uint32_t = 0;
    let mut task = null_mut::<Task>();
    let mut i: uint32_t = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(b"forking!\0" as *const u8 as *const c_char as *mut c_char);
    }

    task = newBoundTask();
    cap = null_mut::<Capability>();
    waitForCapability(&raw mut cap, task);
    i = 0 as uint32_t;

    while i < n_capabilities {
        i = i.wrapping_add(1);
    }

    stopTimer();
    flushAllCapsEventsBufs();
    pid = fork();

    if pid != 0 {
        startTimer();
        i = 0 as uint32_t;

        while i < n_capabilities {
            releaseCapability_(getCapability(i), r#false != 0);
            i = i.wrapping_add(1);
        }

        exitMyTask();

        return pid;
    } else {
        resetChildProcessStats();
        resetTracing();
        g = 0 as uint32_t;

        while g < RtsFlags.GcFlags.generations {
            t = (*generations.offset(g as isize)).threads;

            while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
                next = (*t).global_link as *mut StgTSO;
                deleteThread_(t);
                (*t).bound = null_mut::<InCall_>();
                t = next;
            }

            g = g.wrapping_add(1);
        }

        discardTasksExcept(task);
        i = 0 as uint32_t;

        while i < n_capabilities {
            cap = getCapability(i);
            truncateRunQueue(cap);
            (*cap).n_run_queue = 0 as uint32_t;
            (*cap).suspended_ccalls = null_mut::<InCall>();
            (*cap).n_suspended_ccalls = 0 as uint32_t;

            if (*cap).no != 0 as uint32_t {
                (*task).cap = cap as *mut Capability_;
                releaseCapability(cap);
            }

            i = i.wrapping_add(1);
        }

        cap = getCapability(0 as uint32_t);
        (*task).cap = cap as *mut Capability_;
        g = 0 as uint32_t;

        while g < RtsFlags.GcFlags.generations {
            let ref mut fresh1 = (*generations.offset(g as isize)).threads;
            *fresh1 = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            g = g.wrapping_add(1);
        }

        initTimer();
        traceTaskCreate(task, cap);
        initIOManagerAfterFork(&raw mut cap);
        startTimer();

        rts_evalStableIOMain(
            &raw mut cap,
            entry as HsStablePtr,
            null_mut::<HsStablePtr>(),
        );

        rts_checkSchedStatus(
            b"forkProcess\0" as *const u8 as *const c_char as *mut c_char,
            cap,
        );

        rts_unlock(cap);
        shutdownHaskellAndExit(EXIT_SUCCESS, 0 as c_int);
    };
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setNumCapabilities(mut new_n_capabilities: uint32_t) {
    if new_n_capabilities != 1 as uint32_t {
        errorBelch(
            b"setNumCapabilities: not supported in the non-threaded RTS\0" as *const u8
                as *const c_char,
        );
    }
}

unsafe fn deleteAllThreads() {
    let mut t = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut g: uint32_t = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(b"deleting all threads\0" as *const u8 as *const c_char as *mut c_char);
    }

    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        t = (*generations.offset(g as isize)).threads;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            next = (*t).global_link as *mut StgTSO;
            deleteThread(t);
            t = next;
        }

        g = g.wrapping_add(1);
    }
}

#[inline]
unsafe fn suspendTask(mut cap: *mut Capability, mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    incall = (*task).incall as *mut InCall;
    (*incall).next = (*cap).suspended_ccalls as *mut InCall_;
    (*incall).prev = null_mut::<InCall_>();

    if !(*cap).suspended_ccalls.is_null() {
        (*(*cap).suspended_ccalls).prev = incall as *mut InCall_;
    }

    (*cap).suspended_ccalls = incall;
    (*cap).n_suspended_ccalls = (*cap).n_suspended_ccalls.wrapping_add(1);
}

#[inline]
unsafe fn recoverSuspendedTask(mut cap: *mut Capability, mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    incall = (*task).incall as *mut InCall;

    if !(*incall).prev.is_null() {
        (*(*incall).prev).next = (*incall).next;
    } else {
        (*cap).suspended_ccalls = (*incall).next as *mut InCall;
    }

    if !(*incall).next.is_null() {
        (*(*incall).next).prev = (*incall).prev;
    }

    (*incall).prev = null_mut::<InCall_>();
    (*incall).next = (*incall).prev;
    (*cap).n_suspended_ccalls = (*cap).n_suspended_ccalls.wrapping_sub(1);
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn suspendThread(
    mut reg: *mut StgRegTable,
    mut interruptible_0: bool,
) -> *mut c_void {
    let mut cap = null_mut::<Capability>();
    let mut saved_errno: c_int = 0;
    let mut tso = null_mut::<StgTSO>();
    let mut task = null_mut::<Task>();
    saved_errno = *__error();
    cap = regTableToCapability(reg);
    task = (*cap).running_task;
    tso = (*cap).r.rCurrentTSO as *mut StgTSO;

    traceEventStopThread(
        cap,
        tso,
        THREAD_SUSPENDED_FOREIGN_CALL as StgThreadReturnCode,
        0 as StgWord32,
    );

    (*tso).what_next = 1 as StgWord16;
    threadPaused(cap, tso);

    if interruptible_0 {
        (*tso).why_blocked = BlockedOnCCall_Interruptible as StgWord32;
    } else {
        (*tso).why_blocked = BlockedOnCCall as StgWord32;
    }

    (*(*task).incall).suspended_tso = tso;
    (*(*task).incall).suspended_cap = cap;
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
    suspendTask(cap, task);
    (*cap).in_haskell = r#false != 0;
    releaseCapability_(cap, r#false != 0);
    *__error() = saved_errno;

    return task as *mut c_void;
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn resumeThread(mut task_: *mut c_void) -> *mut StgRegTable {
    let mut tso = null_mut::<StgTSO>();
    let mut incall = null_mut::<InCall>();
    let mut cap = null_mut::<Capability>();
    let mut task = task_ as *mut Task;
    let mut saved_errno: c_int = 0;
    saved_errno = *__error();
    incall = (*task).incall as *mut InCall;
    cap = (*incall).suspended_cap;
    (*task).cap = cap as *mut Capability_;
    waitForCapability(&raw mut cap, task);
    recoverSuspendedTask(cap, task);
    tso = (*incall).suspended_tso;
    (*incall).suspended_tso = null_mut::<StgTSO>();
    (*incall).suspended_cap = null_mut::<Capability>();
    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
    traceEventRunThread(cap, tso);
    (*tso).why_blocked = NotBlocked as StgWord32;

    if (*tso).flags & TSO_BLOCKEX as StgWord32 == 0 as StgWord32 {
        if (*tso).blocked_exceptions
            != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                as *mut MessageThrowTo
        {
            maybePerformBlockedException(cap, tso);
        }
    }

    (*cap).r.rCurrentTSO = tso as *mut StgTSO_;
    (*cap).in_haskell = r#true != 0;
    *__error() = saved_errno;
    dirty_TSO(cap, tso);
    dirty_STACK(cap, (*tso).stackobj as *mut StgStack);

    return &raw mut (*cap).r;
}

unsafe fn scheduleThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    appendToRunQueue(cap, tso);
}

unsafe fn scheduleThreadNow(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    pushOnRunQueue(cap, tso);
}

unsafe fn scheduleThreadOn(mut cap: *mut Capability, mut cpu: StgWord, mut tso: *mut StgTSO) {
    (*tso).flags |= TSO_LOCKED as StgWord32;
    appendToRunQueue(cap, tso);
    contextSwitchCapability(cap, r#false != 0);
}

unsafe fn scheduleWaitThread(
    mut tso: *mut StgTSO,
    mut ret: *mut HaskellObj,
    mut pcap: *mut *mut Capability,
) {
    let mut task = null_mut::<Task>();
    let mut cap = null_mut::<Capability>();
    cap = *pcap;
    task = (*cap).running_task;
    (*tso).bound = (*task).incall as *mut InCall_;
    (*tso).cap = cap as *mut Capability_;
    (*(*task).incall).tso = tso;
    (*(*task).incall).ret = ret as *mut *mut StgClosure;
    (*(*task).incall).rstat = NoStatus;
    appendToRunQueue(cap, tso);
    cap = schedule(cap, task);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"bound thread (%llu) finished\0" as *const u8 as *const c_char as *mut c_char,
            (*tso).id,
        );
    }

    *pcap = cap;
}

unsafe fn startWorkerTasks(mut from: uint32_t, mut to: uint32_t) {}

unsafe fn initScheduler() {
    setSchedState(SCHED_RUNNING);
    setRecentActivity(ACTIVITY_YES);
    allocated_bytes_at_heapoverflow = 0 as uint64_t;
    initCapabilities();
    initTaskManager();
    startWorkerTasks(1 as uint32_t, n_capabilities);
}

unsafe fn exitScheduler(mut wait_foreign: bool) {
    let mut task = newBoundTask();

    if (getSchedState() as c_uint) < SCHED_SHUTTING_DOWN as c_int as c_uint {
        setSchedState(SCHED_INTERRUPTING);

        let mut cap = (*task).cap as *mut Capability;
        waitForCapability(&raw mut cap, task);

        scheduleDoGC(
            &raw mut cap,
            task,
            r#true != 0,
            r#false != 0,
            r#false != 0,
            r#true != 0,
        );

        releaseCapability(cap);
    }

    shutdownCapabilities(task, wait_foreign);
    exitMyTask();
}

unsafe fn freeScheduler() {
    let mut still_running: uint32_t = 0;
    still_running = freeTaskManager();

    if still_running == 0 as uint32_t {
        freeCapabilities();
    }
}

unsafe fn performGC_(mut force_major: bool, mut nonconcurrent: bool) {
    let mut task = null_mut::<Task>();
    let mut cap = null_mut::<Capability>();
    task = newBoundTask();
    waitForCapability(&raw mut cap, task);

    scheduleDoGC(
        &raw mut cap,
        task,
        force_major,
        r#false != 0,
        r#false != 0,
        nonconcurrent,
    );

    releaseCapability(cap);
    exitMyTask();
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performGC() {
    performGC_(r#false != 0, r#false != 0);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    performGC_(r#true != 0, r#false != 0);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    performGC_(r#true != 0, r#true != 0);
}

unsafe fn interruptStgRts() {
    setSchedState(SCHED_INTERRUPTING);
    interruptAllCapabilities();
}

unsafe fn deleteThread(mut tso: *mut StgTSO) {
    if (*tso).why_blocked != BlockedOnCCall as StgWord32
        && (*tso).why_blocked != BlockedOnCCall_Interruptible as StgWord32
    {
        throwToSingleThreaded((*tso).cap as *mut Capability, tso, null_mut::<StgClosure>());
    }
}

unsafe fn deleteThread_(mut tso: *mut StgTSO) {
    if (*tso).why_blocked == BlockedOnCCall as StgWord32
        || (*tso).why_blocked == BlockedOnCCall_Interruptible as StgWord32
    {
        (*tso).what_next = ThreadKilled as StgWord16;
        appendToRunQueue((*tso).cap as *mut Capability, tso);
    } else {
        deleteThread(tso);
    };
}

unsafe fn appendToRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*cap).run_queue_hd = tso;
        (*tso).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    } else {
        setTSOLink(cap, (*cap).run_queue_tl, tso);
        setTSOPrev(cap, tso, (*cap).run_queue_tl);
    }

    (*cap).run_queue_tl = tso;
    (*cap).n_run_queue = (*cap).n_run_queue.wrapping_add(1);
}

unsafe fn pushOnRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    setTSOLink(cap, tso, (*cap).run_queue_hd);
    (*tso).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;

    if (*cap).run_queue_hd != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        setTSOPrev(cap, (*cap).run_queue_hd, tso);
    }

    (*cap).run_queue_hd = tso;

    if (*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*cap).run_queue_tl = tso;
    }

    (*cap).n_run_queue = (*cap).n_run_queue.wrapping_add(1);
}

unsafe fn popRunQueue(mut cap: *mut Capability) -> *mut StgTSO {
    let mut t = (*cap).run_queue_hd;
    (*cap).run_queue_hd = (*t)._link as *mut StgTSO;

    let mut link = (*t)._link;

    if link != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*link).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    }

    (*t)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;

    if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*cap).run_queue_tl = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    }

    (*cap).n_run_queue = (*cap).n_run_queue.wrapping_sub(1);

    return t;
}

unsafe fn raiseExceptionHelper(
    mut reg: *mut StgRegTable,
    mut tso: *mut StgTSO,
    mut exception: *mut StgClosure,
) -> StgWord {
    let mut cap = regTableToCapability(reg);
    let mut raise_closure = null_mut::<StgThunk>();
    let mut p = null_mut::<StgWord>();
    let mut next = null_mut::<StgWord>();
    let mut info = null::<StgRetInfoTable>();
    p = (*(*tso).stackobj).sp;

    loop {
        info = get_ret_itbl(p as *mut StgClosure);
        next = p.offset(stack_frame_sizeW(p as *mut StgClosure) as isize);

        match (*info).i.r#type {
            33 => {
                if raise_closure.is_null() {
                    raise_closure = allocate(
                        cap,
                        (size_of::<StgThunk>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            .wrapping_add(1 as usize) as W_,
                    ) as *mut StgThunk;

                    (*raise_closure).header.info = &raw const stg_raise_info;

                    let ref mut fresh7 = *(&raw mut (*raise_closure).payload
                        as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize);
                    *fresh7 = exception as *mut StgClosure_;
                }

                updateThunk(
                    cap,
                    tso,
                    (*(p as *mut StgUpdateFrame)).updatee,
                    raise_closure as *mut StgClosure,
                );

                p = next;
            }
            55 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found ATOMICALLY_FRAME at %p\0" as *const u8 as *const c_char
                            as *mut c_char,
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            34 => {
                (*(*tso).stackobj).sp = p;

                return CATCH_FRAME as StgWord;
            }
            57 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_STM_FRAME at %p\0" as *const u8 as *const c_char
                            as *mut c_char,
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return CATCH_STM_FRAME as StgWord;
            }
            35 => {
                (*(*tso).stackobj).sp = p;
                threadStackUnderflow(cap, tso);
                p = (*(*tso).stackobj).sp;
            }
            36 => {
                (*(*tso).stackobj).sp = p;

                return STOP_FRAME as StgWord;
            }
            56 => {
                let mut trec = (*tso).trec as *mut StgTRecHeader;
                let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_RETRY_FRAME at %p during raise\0" as *const u8
                            as *const c_char as *mut c_char,
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"trec=%p outer=%p\0" as *const u8 as *const c_char as *mut c_char,
                        trec,
                        outer,
                    );
                }

                stmAbortTransaction(cap, trec);
                stmFreeAbortedTRec(cap, trec);
                (*tso).trec = outer as *mut StgTRecHeader_;
                p = next;
            }
            _ => {
                if *p == &raw const stg_unmaskAsyncExceptionszh_ret_info as StgWord {
                    (*tso).flags &= !(TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
                } else if *p == &raw const stg_maskAsyncExceptionszh_ret_info as StgWord {
                    (*tso).flags |= (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
                } else if *p == &raw const stg_maskUninterruptiblezh_ret_info as StgWord {
                    (*tso).flags |= TSO_BLOCKEX as StgWord32;
                    (*tso).flags &= !TSO_INTERRUPTIBLE as StgWord32;
                }

                p = next;
            }
        }
    }
}

unsafe fn findRetryFrameHelper(mut cap: *mut Capability, mut tso: *mut StgTSO) -> StgWord {
    let mut info = null::<StgRetInfoTable>();
    let mut p = null_mut::<StgWord>();
    let mut next = null_mut::<StgWord>();
    p = (*(*tso).stackobj).sp;

    loop {
        info = get_ret_itbl(p as *const StgClosure);
        next = p.offset(stack_frame_sizeW(p as *mut StgClosure) as isize);

        match (*info).i.r#type {
            55 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found ATOMICALLY_FRAME at %p during retry\0" as *const u8 as *const c_char
                            as *mut c_char,
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            56 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_RETRY_FRAME at %p during retry\0" as *const u8
                            as *const c_char as *mut c_char,
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return CATCH_RETRY_FRAME as StgWord;
            }
            57 => {
                let mut trec = (*tso).trec as *mut StgTRecHeader;
                let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_STM_FRAME at %p during retry\0" as *const u8 as *const c_char
                            as *mut c_char,
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"trec=%p outer=%p\0" as *const u8 as *const c_char as *mut c_char,
                        trec,
                        outer,
                    );
                }

                stmAbortTransaction(cap, trec);
                stmFreeAbortedTRec(cap, trec);
                (*tso).trec = outer as *mut StgTRecHeader_;
                p = next;
            }
            35 => {
                (*(*tso).stackobj).sp = p;
                threadStackUnderflow(cap, tso);
                p = (*(*tso).stackobj).sp;
            }
            _ => {
                p = next;
            }
        }
    }
}

unsafe fn findAtomicallyFrameHelper(mut cap: *mut Capability, mut tso: *mut StgTSO) -> StgWord {
    let mut info = null::<StgRetInfoTable>();
    let mut p = null_mut::<StgWord>();
    let mut next = null_mut::<StgWord>();
    p = (*(*tso).stackobj).sp;

    loop {
        info = get_ret_itbl(p as *const StgClosure);
        next = p.offset(stack_frame_sizeW(p as *mut StgClosure) as isize);

        match (*info).i.r#type {
            55 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found ATOMICALLY_FRAME at %p while aborting after orElse\0" as *const u8
                            as *const c_char as *mut c_char,
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            56 => {
                let mut trec = (*tso).trec as *mut StgTRecHeader;
                let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_RETRY_FRAME at %p while aborting after orElse\0" as *const u8
                            as *const c_char as *mut c_char,
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"trec=%p outer=%p\0" as *const u8 as *const c_char as *mut c_char,
                        trec,
                        outer,
                    );
                }

                stmAbortTransaction(cap, trec);
                stmFreeAbortedTRec(cap, trec);
                (*tso).trec = outer as *mut StgTRecHeader_;
                p = next;
            }
            57 => {
                let mut trec_0 = (*tso).trec as *mut StgTRecHeader;
                let mut outer_0 = (*trec_0).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"found CATCH_STM_FRAME at %p while aborting after orElse\0" as *const u8
                            as *const c_char as *mut c_char,
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as c_long != 0 {
                    trace_(
                        b"trec=%p outer=%p\0" as *const u8 as *const c_char as *mut c_char,
                        trec_0,
                        outer_0,
                    );
                }

                stmAbortTransaction(cap, trec_0);
                stmFreeAbortedTRec(cap, trec_0);
                (*tso).trec = outer_0 as *mut StgTRecHeader_;
                p = next;
            }
            35 => {
                (*(*tso).stackobj).sp = p;
                threadStackUnderflow(cap, tso);
                p = (*(*tso).stackobj).sp;
            }
            _ => {
                p = next;
            }
        }
    }
}

unsafe fn resurrectThreads(mut threads: *mut StgTSO) {
    let mut tso = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut cap = null_mut::<Capability>();
    let mut r#gen = null_mut::<generation>();
    tso = threads;

    while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        next = (*tso).global_link as *mut StgTSO;
        r#gen = (*Bdescr(tso as StgPtr)).r#gen as *mut generation;
        (*tso).global_link = (*r#gen).threads as *mut StgTSO_;
        (*r#gen).threads = tso;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
            trace_(
                b"resurrecting thread %llu\0" as *const u8 as *const c_char as *mut c_char,
                (*tso).id,
            );
        }

        cap = (*tso).cap as *mut Capability;

        match (*tso).why_blocked {
            1 | 14 => {
                throwToSingleThreaded(cap, tso, (*ghc_hs_iface).blockedIndefinitelyOnMVar_closure);
            }
            2 => {
                throwToSingleThreaded(cap, tso, (*ghc_hs_iface).nonTermination_closure);
            }
            6 => {
                throwToSingleThreaded(cap, tso, (*ghc_hs_iface).blockedIndefinitelyOnSTM_closure);
            }
            0 | 12 => {}
            _ => {
                barf(
                    b"resurrectThreads: thread blocked in a strange way: %d\0" as *const u8
                        as *const c_char,
                    (*tso).why_blocked,
                );
            }
        }

        tso = next;
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setAllocLimitKill(mut shouldKill: bool, mut shouldHook: bool) {
    allocLimitKill = shouldKill;
    allocLimitRunHook = shouldHook;
}
