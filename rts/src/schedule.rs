use crate::capability::{
    Capability_, PendingSync, PutMVar, SYNC_GC_PAR, SYNC_GC_SEQ, SYNC_OTHER, SyncType, anySparks,
    checkSparkCountInvariant, contextSwitchCapability, discardSparksCap, emptyInbox,
    freeCapabilities, getCapability, initCapabilities, interruptAllCapabilities, moreCapabilities,
    pending_sync, regTableToCapability, releaseAndWakeupCapability, releaseCapability,
    releaseCapability_, shutdownCapabilities, sparkPoolSizeCap, tryGrabCapability,
    waitForCapability, yieldCapability,
};
use crate::eventlog::event_log::flushAllCapsEventsBufs;
use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::rts::constants::{
    BlockedOnBlackHole, BlockedOnCCall, BlockedOnCCall_Interruptible, LDV_SHIFT, LDV_STATE_CREATE,
    NotBlocked, StackOverflow, TSO_ALLOC_LIMIT, TSO_BLOCKEX, TSO_INTERRUPTIBLE, TSO_LOCKED,
    ThreadBlocked, ThreadComplete, ThreadFinished, ThreadKilled,
};
use crate::ffi::rts::event_log_format::THREAD_SUSPENDED_FOREIGN_CALL;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::os_threads::{
    Condition, Mutex, OS_TRY_ACQUIRE_LOCK, closeMutex, getNumberOfProcessors, initCondition,
    initMutex, osThreadId, signalCondition, waitCondition,
};
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, CostCentreStack_, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::deRefStablePtr;
use crate::ffi::rts::storage::block::{
    BLOCK_MASK, BLOCK_SIZE, BLOCKS_PER_MBLOCK, Bdescr, MBLOCK_MASK, MBLOCK_SIZE,
    allocGroupOnNode_lock, bdescr, bdescr_, dbl_link_insert_after,
};
use crate::ffi::rts::storage::closure_macros::{
    doingErasProfiling, doingLDVProfiling, doingRetainerProfiling, get_ret_itbl, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closure_types::{
    ATOMICALLY_FRAME, CATCH_FRAME, CATCH_RETRY_FRAME, CATCH_STM_FRAME, STOP_FRAME,
};
use crate::ffi::rts::storage::closures::{
    Message, MessageThrowTo, StgClosure_, StgDeadThreadFrame, StgMVar, StgTRecHeader,
    StgTRecHeader_, StgThunk, StgUpdateFrame,
};
use crate::ffi::rts::storage::gc::{
    allocate, g0, generation, generations, initBdescr, memcount, nursery, oldest_gen,
};
use crate::ffi::rts::storage::m_block::mblocks_allocated;
use crate::ffi::rts::storage::tso::{
    StgStack, StgTSO_, StgThreadID, StgThreadReturnCode, dirty_STACK, dirty_TSO, setTSOLink,
    setTSOPrev,
};
use crate::ffi::rts::threads::{
    createIOThread, enabled_capabilities, getNumCapabilities, max_n_capabilities, n_capabilities,
};
use crate::ffi::rts::time::Time;
use crate::ffi::rts::timer::{startTimer, stopTimer};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::{_assertFail, stg_exit};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::{
    Capability, HaskellObj, HeapExhausted, Interrupted, Killed, NoStatus, Success, getAllocations,
    rts_apply, rts_checkSchedStatus, rts_evalStableIOMain, rts_lock, rts_unlock,
    shutdownHaskellAndExit,
};
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::misc_closures::{
    stg_END_TSO_QUEUE_closure, stg_NO_TREC_closure, stg_dead_thread_info,
    stg_maskAsyncExceptionszh_ret_info, stg_maskUninterruptiblezh_ret_info, stg_raise_info,
    stg_returnToStackTop, stg_unmaskAsyncExceptionszh_ret_info,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{
    StgFunPtr, StgHalfWord, StgInt64, StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord32,
};
use crate::ffi::stg::{ASSIGN_Int64, PK_Int64, W_};
use crate::interpreter::interpretBCO;
use crate::io_manager::{
    anyPendingTimeoutsOrIO, initIOManagerAfterFork, notifyIOManagerCapabilitiesChanged,
    wakeupIOManager,
};
use crate::messages::{blackHoleOwner, executeMessage};
use crate::prelude::*;
use crate::printer::what_next_strs;
use crate::proftimer::{pauseHeapProfTimer, performHeapProfile, resumeHeapProfTimer};
use crate::raise_async::{
    awakenBlockedExceptionQueue, maybePerformBlockedException, throwToSelf, throwToSingleThreaded,
    throwToSingleThreaded_,
};
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::schedule::{
    ACTIVITY_DONE_GC, ACTIVITY_INACTIVE, ACTIVITY_YES, RecentActivity, SCHED_INTERRUPTING,
    SCHED_RUNNING, SCHED_SHUTTING_DOWN, SchedState, emptyRunQueue, getRecentActivity,
    getSchedState, peekRunQueue, setRecentActivity, setSchedState, truncateRunQueue,
};
use crate::sm::gc::{GarbageCollect, GcConfig, doIdleGCWork, releaseGCThreads, waitForGcThreads};
use crate::sm::gc_thread::{gc_thread_, gc_threads};
use crate::sm::non_moving_mark::updateRemembSetPushClosure;
use crate::sm::sanity::{checkNurserySanity, checkRunQueue, checkTSO};
use crate::sm::storage::{
    calcNeeded, doYouWantToGC, finishedNurseryBlock, sm_mutex, storageAddCapabilities,
};
use crate::sparks::{createSparkThread, sparkPoolSize};
use crate::stable_name::stable_name_mutex;
use crate::stable_ptr::{freeStablePtr, stable_ptr_mutex};
use crate::stats::{resetChildProcessStats, stat_startGCSync};
use crate::stg_run::StgRun;
use crate::stm::{stmAbortTransaction, stmFreeAbortedTRec, stmValidateNestOfTransactions};
use crate::task::{
    InCall, InCall_, Task, all_tasks_mutex, discardTasksExcept, exitMyTask, freeTaskManager,
    initTaskManager, isBoundTask, myTask, newBoundTask, startWorkerTask, workerTaskStop,
};
use crate::thread_labels::setThreadLabel;
use crate::thread_paused::threadPaused;
use crate::threads::{
    migrateThread, performTryPutMVar, printAllThreads, threadStackOverflow, threadStackUnderflow,
    updateThunk,
};
use crate::timer::initTimer;
use crate::top_handler::getTopHandlerThread;
use crate::trace::{
    DEBUG_RTS, resetTracing, trace_, traceCapDisable, traceCapEnable, traceEventMigrateThread,
    traceEventRequestParGc, traceEventRequestSeqGc, traceEventRunThread, traceEventStopThread,
    traceSparkCounters, traceTaskCreate, traceTaskMigrate, traceThreadStatus_,
    tracingAddCapabilities,
};

#[cfg(test)]
mod tests;

pub(crate) type SchedState = u32;

pub(crate) const SCHED_SHUTTING_DOWN: SchedState = 2;

pub(crate) const SCHED_INTERRUPTING: SchedState = 1;

pub(crate) const SCHED_RUNNING: SchedState = 0;

pub(crate) const ACTIVITY_YES: RecentActivity = 0;

pub(crate) const ACTIVITY_DONE_GC: RecentActivity = 3;

pub(crate) type RecentActivity = u32;

pub(crate) const ACTIVITY_INACTIVE: RecentActivity = 2;

pub(crate) const ACTIVITY_MAYBE_NO: RecentActivity = 1;

#[inline]
pub(crate) unsafe fn setSchedState(mut ss: SchedState) {
    (&raw mut sched_state).store(ss as StgWord, Ordering::SeqCst);
}

#[inline]
pub(crate) unsafe fn getSchedState() -> SchedState {
    return (&raw mut sched_state).load(Ordering::SeqCst) as SchedState;
}

#[inline]
pub(crate) unsafe fn setRecentActivity(mut new_value: RecentActivity) -> RecentActivity {
    let mut old: StgWord = (&raw mut recent_activity).xchg(new_value as StgWord, Ordering::SeqCst);

    return old as RecentActivity;
}

#[inline]
pub(crate) unsafe fn getRecentActivity() -> RecentActivity {
    return (&raw mut recent_activity).load(Ordering::Relaxed) as RecentActivity;
}

#[inline]
pub(crate) unsafe fn peekRunQueue(mut cap: *mut Capability) -> *mut StgTSO {
    return (*cap).run_queue_hd;
}

#[inline]
pub(crate) unsafe fn emptyRunQueue(mut cap: *mut Capability) -> bool {
    return (*cap).n_run_queue == 0;
}

#[inline]
pub(crate) unsafe fn truncateRunQueue(mut cap: *mut Capability) {
    (*cap).run_queue_hd = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).run_queue_tl = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).n_run_queue = 0;
}

static mut allocated_bytes_at_heapoverflow: u64 = 0;

static mut heap_overflow: bool = false;

static mut recent_activity: StgWord = ACTIVITY_YES as i32 as StgWord;

static mut sched_state: StgWord = SCHED_RUNNING as i32 as StgWord;

static mut allocLimitKill: bool = true;

static mut allocLimitRunHook: bool = false;

static mut sched_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut sync_finished_cond: Condition = Condition {
    cond: _opaque_pthread_cond_t {
        __sig: 0,
        __opaque: [0; 40],
    },
};

static mut sync_finished_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn schedule(mut initialCapability: *mut Capability, mut task: *mut Task) -> *mut Capability {
    let mut t = null_mut::<StgTSO>();
    let mut cap = null_mut::<Capability>();
    let mut ret: StgThreadReturnCode = 0;
    let mut prev_what_next: u32 = 0;
    let mut ready_to_gc: bool = false;
    cap = initialCapability;
    t = null_mut::<StgTSO>();

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 207);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"cap %d: schedule()".as_ptr(), (*initialCapability).no);
    }

    loop {
        if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if ((*task).cap == cap) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if (if (*cap).run_queue_hd
            == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        {
            ((*cap).run_queue_tl
                == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                && (*cap).n_run_queue == 0) as i32
        } else {
            1
        } != 0) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if (if (*cap).suspended_ccalls.is_null() {
            ((*cap).n_suspended_ccalls == 0) as i32
        } else {
            1
        } != 0) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if (myTask() == task) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if ((*task).id == osThreadId()) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 215);
        }

        if (*cap).in_haskell {
            errorBelch(
                c"schedule: re-entered unsafely.\n   Perhaps a 'foreign import unsafe' should be 'safe'?"
                    .as_ptr(),
            );

            stg_exit(EXIT_FAILURE);
        }

        let mut current_block_59: u64;

        match getSchedState() as u32 {
            0 => {
                current_block_59 = 5658374378798827547;
            }
            1 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(c"SCHED_INTERRUPTING".as_ptr());
                }

                scheduleDoGC(&raw mut cap, task, true, false, false, false);

                if (getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32) as i32 as i64 != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 277);
                }

                current_block_59 = 675994927932133621;
            }
            2 => {
                current_block_59 = 675994927932133621;
            }
            _ => {
                barf(c"sched_state: %llu".as_ptr(), sched_state);
            }
        }

        match current_block_59 {
            675994927932133621 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(c"SCHED_SHUTTING_DOWN".as_ptr());
                }

                if !isBoundTask(task) && emptyRunQueue(cap) as i32 != 0 {
                    return cap;
                }
            }
            _ => {}
        }

        scheduleFindWork(&raw mut cap);
        schedulePushWork(cap, task);
        scheduleDetectDeadlock(&raw mut cap, task);
        scheduleYield(&raw mut cap, task);

        if emptyRunQueue(cap) {
            continue;
        }

        t = popRunQueue(cap);

        if RtsFlags.DebugFlags.sanity {
            checkTSO(t);
        }

        let mut bound = (*t).bound as *mut InCall;

        if !bound.is_null() {
            if !((*bound).task == task) {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(
                        c"thread %lu bound to another OS thread".as_ptr(),
                        (*t).id as u64,
                    );
                }

                pushOnRunQueue(cap, t);
                continue;
            }
        } else if !(*(*task).incall).tso.is_null() {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(
                    c"this OS thread cannot run thread %lu".as_ptr(),
                    (*t).id as u64,
                );
            }

            pushOnRunQueue(cap, t);
            continue;
        }

        if getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32
            && !((*t).what_next as i32 == ThreadComplete || (*t).what_next as i32 == ThreadKilled)
        {
            deleteThread(t);
        }

        if (*cap).disabled as i32 != 0 && (*t).bound.is_null() {
            let mut dest_cap = getCapability((*cap).no.wrapping_rem(enabled_capabilities));

            migrateThread(cap, t, dest_cap);
        } else {
            if RtsFlags.ConcFlags.ctxtSwitchTicks == 0
                && (!emptyRunQueue(cap) || anyPendingTimeoutsOrIO(cap) as i32 != 0)
            {
                (&raw mut (*cap).context_switch).store(1, Ordering::Relaxed);
            }

            loop {
                (*cap).r.rCurrentTSO = t as *mut StgTSO_;
                resumeHeapProfTimer();

                if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if ((*task).cap == cap) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if (if (*cap).run_queue_hd
                    == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                {
                    ((*cap).run_queue_tl
                        == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        && (*cap).n_run_queue == 0) as i32
                } else {
                    1
                } != 0) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if (if (*cap).suspended_ccalls.is_null() {
                    ((*cap).n_suspended_ccalls == 0) as i32
                } else {
                    1
                } != 0) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if (myTask() == task) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if ((*task).id == osThreadId()) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 429);
                }

                if ((*t).cap == cap) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 430);
                }

                if (if !(*t).bound.is_null() {
                    ((*(*(*t).bound).task).cap == cap) as i32
                } else {
                    1
                } != 0) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 431);
                }

                prev_what_next = (*t).what_next as u32;
                *__error() = (*t).saved_errno as i32;
                (&raw mut (*cap).interrupt).store(0, Ordering::Relaxed);
                (*cap).in_haskell = true;
                (&raw mut (*cap).idle).store(0, Ordering::Relaxed);
                dirty_TSO(cap, t);
                dirty_STACK(cap, (*t).stackobj as *mut StgStack);

                match getRecentActivity() as u32 {
                    3 => {
                        let mut prev: u32 = 0;
                        prev = setRecentActivity(ACTIVITY_YES) as u32;
                        prev == ACTIVITY_DONE_GC as i32 as u32;
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
                            transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(
                                Some(stg_returnToStackTop as unsafe extern "C" fn() -> StgFunPtr),
                            ),
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
                            c"schedule: invalid prev_what_next=%u field".as_ptr(),
                            prev_what_next,
                        );
                    }
                }

                (*cap).in_haskell = false;
                t = (*cap).r.rCurrentTSO as *mut StgTSO;
                (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
                (*t).saved_errno = *__error() as StgWord32;

                if ret == ThreadBlocked as StgThreadReturnCode {
                    let mut why_blocked: u16 =
                        (&raw mut (*t).why_blocked).load(Ordering::Acquire) as u16;

                    if why_blocked as i32 == BlockedOnBlackHole {
                        let mut owner = blackHoleOwner((*(*t).block_info.bh).bh);

                        traceEventStopThread(
                            cap,
                            t,
                            ((*t).why_blocked as StgThreadReturnCode)
                                .wrapping_add(6 as StgThreadReturnCode),
                            (if !owner.is_null() { (*owner).id } else { 0 }) as StgWord32,
                        );
                    } else {
                        traceEventStopThread(
                            cap,
                            t,
                            ((*t).why_blocked as StgThreadReturnCode)
                                .wrapping_add(6 as StgThreadReturnCode),
                            0,
                        );
                    }
                } else if ret == StackOverflow as StgThreadReturnCode {
                    traceEventStopThread(cap, t, ret, (*t).tot_stack_size);
                } else {
                    traceEventStopThread(cap, t, ret, 0);
                }

                if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if ((*task).cap == cap) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if (if (*cap).run_queue_hd
                    == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                {
                    ((*cap).run_queue_tl
                        == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        && (*cap).n_run_queue == 0) as i32
                } else {
                    1
                } != 0) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if (if (*cap).suspended_ccalls.is_null() {
                    ((*cap).n_suspended_ccalls == 0) as i32
                } else {
                    1
                } != 0) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if (myTask() == task) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if ((*task).id == osThreadId()) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 542);
                }

                if ((*t).cap == cap) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 543);
                }

                pauseHeapProfTimer();
                (*cap).r.rCCCS =
                    &raw mut CCS_SYSTEM as *mut CostCentreStack as *mut CostCentreStack_;
                schedulePostRunThread(cap, t);
                ready_to_gc = false;

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

                        if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32
                            as i64
                            != 0
                        {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        if ((*task).cap == cap) as i32 as i64 != 0 {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        if (if (*cap).run_queue_hd
                            == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        {
                            ((*cap).run_queue_tl
                                == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                                && (*cap).n_run_queue == 0) as i32
                        } else {
                            1
                        } != 0) as i32 as i64
                            != 0
                        {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        if (if (*cap).suspended_ccalls.is_null() {
                            ((*cap).n_suspended_ccalls == 0) as i32
                        } else {
                            1
                        } != 0) as i32 as i64
                            != 0
                        {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        if (myTask() == task) as i32 as i64 != 0 {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        if ((*task).id == osThreadId()) as i32 as i64 != 0 {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 582);
                        }

                        break;
                    }
                    _ => {
                        barf(
                            c"schedule: invalid thread return code %d".as_ptr(),
                            ret as i32,
                        );
                    }
                }
            }

            if ready_to_gc as i32 != 0 || scheduleNeedHeapProfile(ready_to_gc) as i32 != 0 {
                scheduleDoGC(&raw mut cap, task, false, ready_to_gc, false, false);
            }
        }
    }
}

unsafe fn removeFromRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if (*tso).block_info.prev == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if ((*cap).run_queue_hd == tso) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 603);
        }

        (*cap).run_queue_hd = (*tso)._link as *mut StgTSO;
    } else {
        setTSOLink(cap, (*tso).block_info.prev, (*tso)._link as *mut StgTSO);
    }

    if (*tso)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if ((*cap).run_queue_tl == tso) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 609);
        }

        (*cap).run_queue_tl = (*tso).block_info.prev;
    } else {
        setTSOPrev(cap, (*tso)._link as *mut StgTSO, (*tso).block_info.prev);
    }

    (*tso).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*tso)._link = (*tso).block_info.prev as *mut StgTSO_;
    (*cap).n_run_queue = (*cap).n_run_queue.wrapping_sub(1);

    if RtsFlags.DebugFlags.sanity {
        checkRunQueue(cap);
    }
}

unsafe fn promoteInRunQueue(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    removeFromRunQueue(cap, tso);
    pushOnRunQueue(cap, tso);
}

unsafe fn scheduleFindWork(mut pcap: *mut *mut Capability) {
    scheduleStartSignalHandlers(*pcap);
    scheduleProcessInbox(pcap);
    scheduleCheckBlockedThreads(*pcap);

    if emptyRunQueue(*pcap) {
        scheduleActivateSpark(*pcap);
    }
}

unsafe fn shouldYieldCapability(
    mut cap: *mut Capability,
    mut task: *mut Task,
    mut didGcLast: bool,
) -> bool {
    return !(&raw mut pending_sync).load(Ordering::Relaxed).is_null() && !didGcLast
        || (&raw mut (*cap).n_returning_tasks).load(Ordering::Relaxed) != 0
        || !emptyRunQueue(cap)
            && (if (*(*task).incall).tso.is_null() {
                ((*peekRunQueue(cap)).bound != NULL as *mut InCall_) as i32
            } else {
                ((*peekRunQueue(cap)).bound != (*task).incall) as i32
            }) != 0;
}

unsafe fn scheduleYield(mut pcap: *mut *mut Capability, mut task: *mut Task) {
    let mut cap = *pcap;
    let mut didGcLast = false;

    if !shouldYieldCapability(cap, task, false)
        && (!emptyRunQueue(cap)
            || !emptyInbox(cap)
            || getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32)
    {
        return;
    }

    loop {
        if doIdleGCWork(cap, false) {
            didGcLast = false;
        } else {
            didGcLast = yieldCapability(&raw mut cap, task, !didGcLast);
        }

        if !shouldYieldCapability(cap, task, didGcLast) {
            break;
        }
    }

    *pcap = cap;
}

unsafe fn schedulePushWork(mut cap: *mut Capability, mut task: *mut Task) {
    let vla = getNumCapabilities() as usize;

    let mut free_caps: Vec<*mut Capability> = ::std::vec::from_elem(null_mut::<Capability>(), vla);

    let mut i: u32 = 0;
    let mut n_wanted_caps: u32 = 0;
    let mut n_free_caps: u32 = 0;

    let mut spare_threads: u32 = if (*cap).n_run_queue > 0 {
        (*cap).n_run_queue.wrapping_sub(1 as u32)
    } else {
        0
    };

    if !RtsFlags.ParFlags.migrate {
        spare_threads = 0;
    }

    n_wanted_caps = sparkPoolSizeCap(cap).wrapping_add(spare_threads);

    if n_wanted_caps == 0 {
        return;
    }

    i = (*cap)
        .no
        .wrapping_add(1 as u32)
        .wrapping_rem(getNumCapabilities() as u32);
    n_free_caps = 0;

    while n_free_caps < n_wanted_caps && i != (*cap).no {
        let mut cap0 = getCapability(i);

        if cap != cap0 && !(*cap0).disabled && tryGrabCapability(cap0, task) as i32 != 0 {
            if !emptyRunQueue(cap0)
                || (&raw mut (*cap0).n_returning_tasks).load(Ordering::Relaxed) != 0
                || !emptyInbox(cap0)
            {
                releaseCapability(cap0);
            } else {
                let fresh9 = n_free_caps;
                n_free_caps = n_free_caps.wrapping_add(1);

                let ref mut fresh10 = *free_caps.as_mut_ptr().offset(fresh9 as isize);
                *fresh10 = cap0;
            }
        }

        i = i
            .wrapping_add(1 as u32)
            .wrapping_rem(getNumCapabilities() as u32);
    }

    if n_free_caps > 0 {
        let mut prev = null_mut::<StgTSO>();
        let mut t = null_mut::<StgTSO>();
        let mut next = null_mut::<StgTSO>();

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"cap %d: %d threads, %d sparks, and %d free capabilities, sharing...".as_ptr(),
                (*cap).no,
                (*cap).n_run_queue,
                sparkPoolSizeCap(cap),
                n_free_caps,
            );
        }

        let mut keep_threads: u32 = (*cap)
            .n_run_queue
            .wrapping_add(n_free_caps)
            .wrapping_div(n_free_caps.wrapping_add(1 as u32));

        let mut n: u32 = (*cap).n_run_queue;
        prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
        t = (*cap).run_queue_hd;
        i = 0;

        while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && n > keep_threads
        {
            next = (*t)._link as *mut StgTSO;
            (*t)._link =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;

            if (*t).bound == (*task).incall || (*t).flags & TSO_LOCKED as StgWord32 != 0 {
                if prev == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
                    (*cap).run_queue_hd = t;
                } else {
                    setTSOLink(cap, prev, t);
                }

                setTSOPrev(cap, t, prev);
                prev = t;

                if keep_threads > 0 {
                    keep_threads = keep_threads.wrapping_sub(1);
                }
            } else {
                appendToRunQueue(*free_caps.as_mut_ptr().offset(i as isize), t);

                traceEventMigrateThread(cap, t, (**free_caps.as_mut_ptr().offset(i as isize)).no);

                if !(*t).bound.is_null() {
                    (*(*(*t).bound).task).cap =
                        *free_caps.as_mut_ptr().offset(i as isize) as *mut Capability_;
                }

                (&raw mut (*t).cap).store(
                    *free_caps.as_mut_ptr().offset(i as isize),
                    Ordering::Relaxed,
                );

                n = n.wrapping_sub(1);
                i = i.wrapping_add(1);

                if i == n_free_caps {
                    i = 0;
                }
            }

            t = next;
        }

        if t == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            (*cap).run_queue_tl = prev;
        } else {
            setTSOPrev(cap, t, prev);
        }

        if prev == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            (*cap).run_queue_hd = t;
        } else {
            setTSOLink(cap, prev, t);
        }

        (*cap).n_run_queue = n;

        if RtsFlags.DebugFlags.sanity {
            checkRunQueue(cap);
        }

        i = 0;

        while i < n_free_caps {
            (*task).cap = *free_caps.as_mut_ptr().offset(i as isize) as *mut Capability_;

            if sparkPoolSizeCap(cap) > 0 {
                releaseAndWakeupCapability(*free_caps.as_mut_ptr().offset(i as isize));
            } else {
                releaseCapability(*free_caps.as_mut_ptr().offset(i as isize));
            }

            i = i.wrapping_add(1);
        }
    }

    (*task).cap = cap as *mut Capability_;
}

unsafe fn scheduleStartSignalHandlers(mut cap: *mut Capability) {}

unsafe fn scheduleCheckBlockedThreads(mut cap: *mut Capability) {}

unsafe fn scheduleDetectDeadlock(mut pcap: *mut *mut Capability, mut task: *mut Task) {
    let mut cap = *pcap;

    if emptyRunQueue(cap) as i32 != 0 && !anyPendingTimeoutsOrIO(cap) {
        if getRecentActivity() as u32 != ACTIVITY_INACTIVE as i32 as u32 {
            return;
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"deadlocked, forcing major GC...".as_ptr());
        }

        scheduleDoGC(pcap, task, true, false, true, false);
        cap = *pcap;

        if !emptyRunQueue(cap) {
            return;
        }
    }
}

unsafe fn scheduleProcessInbox(mut pcap: *mut *mut Capability) {
    let mut m = null_mut::<Message>();
    let mut next = null_mut::<Message>();
    let mut p = null_mut::<PutMVar>();
    let mut pnext = null_mut::<PutMVar>();
    let mut r: i32 = 0;
    let mut cap = *pcap;

    while !emptyInbox(cap) {
        if doYouWantToGC(cap) {
            scheduleDoGC(pcap, (*cap).running_task, false, false, false, false);
            cap = *pcap;
        }

        r = OS_TRY_ACQUIRE_LOCK(&raw mut (*cap).lock);

        if r != 0 {
            return;
        }

        m = (*cap).inbox;
        p = (*cap).putMVars as *mut PutMVar;
        (*cap).inbox =
            &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut Message;
        (*cap).putMVars = null_mut::<PutMVar_>();

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                1064,
            );
        }

        while m != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut Message
        {
            next = (*m).link as *mut Message;
            executeMessage(cap, m);
            m = next;
        }

        while !p.is_null() {
            pnext = (*p).link as *mut PutMVar;

            performTryPutMVar(
                cap,
                deRefStablePtr((*p).mvar) as *mut StgMVar,
                (*ghc_hs_iface).Z0T_closure,
            );

            freeStablePtr((*p).mvar);
            stgFree(p as *mut c_void);
            p = pnext;
        }
    }
}

unsafe fn scheduleActivateSpark(mut cap: *mut Capability) {
    if anySparks() as i32 != 0 && !(*cap).disabled {
        createSparkThread(cap);

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"creating a spark thread".as_ptr());
        }
    }
}

unsafe fn schedulePostRunThread(mut cap: *mut Capability, mut t: *mut StgTSO) {
    if (*t).trec != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader
        && (*t).why_blocked == NotBlocked as StgWord32
    {
        if stmValidateNestOfTransactions(cap, (*t).trec as *mut StgTRecHeader, true) == 0 {
            if DEBUG_RTS != 0
                && (RtsFlags.DebugFlags.scheduler as i32 | RtsFlags.DebugFlags.stm as i32) as i64
                    != 0
            {
                trace_(c"trec %p found wasting its time".as_ptr(), t);
            }

            throwToSingleThreaded_(cap, t, null_mut::<StgClosure>(), true);
        }
    }

    if PK_Int64(&raw mut (*t).alloc_limit as *mut W_) < 0
        && (*t).flags & TSO_ALLOC_LIMIT as StgWord32 != 0
    {
        if allocLimitKill {
            throwToSelf(cap, t, (*ghc_hs_iface).allocationLimitExceeded_closure);

            ASSIGN_Int64(
                &raw mut (*t).alloc_limit as *mut W_,
                (RtsFlags.GcFlags.allocLimitGrace as StgInt64 as u64)
                    .wrapping_mul(BLOCK_SIZE as u64) as StgInt64,
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

            setThreadLabel(cap, hookThread, c"allocation limit handler thread".as_ptr());
            pushOnRunQueue(cap, hookThread);
        }
    }
}

unsafe fn scheduleHandleHeapOverflow(mut cap: *mut Capability, mut t: *mut StgTSO) -> bool {
    if (&raw mut (*cap).r.rHpLim).load(Ordering::Relaxed).is_null()
        || (&raw mut (*cap).context_switch).load(Ordering::Relaxed) != 0
    {
        (&raw mut (*cap).context_switch).store(0, Ordering::Relaxed);
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
                c"allocation of %ld bytes too large (GHC should have complained at compile-time)"
                    .as_ptr(),
                (*cap).r.rHpAlloc as i64,
            );
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"--<< thread %ld (%s) stopped: requesting a large block (size %ld)\n".as_ptr(),
                (*t).id as i64,
                *(&raw mut what_next_strs as *mut *const c_char).offset((*t).what_next as isize),
                blocks,
            );
        }

        if !(*(*cap).r.rCurrentNursery).link.is_null() || (*(*cap).r.rNursery).n_blocks == 1 {
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
                (*x).flags = 0;
                x = x.offset(1);
            }

            if RtsFlags.DebugFlags.sanity {
                checkNurserySanity((*cap).r.rNursery as *mut nursery);
            }

            finishedNurseryBlock(cap, (*cap).r.rCurrentNursery as *mut bdescr);
            (*cap).r.rCurrentNursery = bd as *mut bdescr_;

            return false;
        }
    }

    return doYouWantToGC(cap);
}

unsafe fn scheduleHandleYield(
    mut cap: *mut Capability,
    mut t: *mut StgTSO,
    mut prev_what_next: u32,
) -> bool {
    if ((*t)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32
        as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 1267);
    }

    if (*t).what_next as u32 != prev_what_next {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"--<< thread %ld (%s) stopped to switch evaluators".as_ptr(),
                (*t).id as i64,
                *(&raw mut what_next_strs as *mut *const c_char).offset((*t).what_next as isize),
            );
        }

        return true;
    }

    if (&raw mut (*cap).context_switch).load(Ordering::Relaxed) != 0 {
        (&raw mut (*cap).context_switch).store(0, Ordering::Relaxed);
        appendToRunQueue(cap, t);
    } else {
        pushOnRunQueue(cap, t);
    }

    if RtsFlags.DebugFlags.sanity {
        checkTSO(t);
    }

    return false;
}

unsafe fn scheduleHandleThreadBlocked(mut t: *mut StgTSO) {
    if RtsFlags.DebugFlags.scheduler as i64 != 0 {
        traceThreadStatus_(t);
    }
}

unsafe fn scheduleHandleThreadFinished(
    mut cap: *mut Capability,
    mut task: *mut Task,
    mut t: *mut StgTSO,
) -> bool {
    awakenBlockedExceptionQueue(cap, t);

    if !(*t).bound.is_null() {
        if (*t).bound != (*task).incall {
            barf(c"finished bound thread that isn't mine".as_ptr());
        }

        if ((*(*task).incall).tso == t) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 1374);
        }

        if (*t).what_next as i32 == ThreadComplete {
            if !(*(*task).incall).ret.is_null() {
                let mut dead = (*(*(*(*task).incall).tso).stackobj).sp.offset(0) as *mut StgWord
                    as *mut StgDeadThreadFrame;

                if ((*dead).header.info == &raw const stg_dead_thread_info) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 1380);
                }

                *(*(*task).incall).ret = (*dead).result;
            }

            (*(*task).incall).rstat = Success;
        } else {
            if !(*(*task).incall).ret.is_null() {
                *(*(*task).incall).ret = null_mut::<StgClosure>();
            }

            if getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32 {
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

        return true;
    }

    return false;
}

unsafe fn scheduleNeedHeapProfile(mut ready_to_gc: bool) -> bool {
    if (&raw mut performHeapProfile).load(Ordering::Relaxed) as i32 != 0
        || RtsFlags.ProfFlags.heapProfileInterval == 0
            && RtsFlags.ProfFlags.doHeapProfile != 0
            && ready_to_gc as i32 != 0
    {
        return true;
    } else {
        return false;
    };
}

unsafe fn stopAllCapabilities(mut pCap: *mut *mut Capability, mut task: *mut Task) {
    stopAllCapabilitiesWith(pCap, task, SYNC_OTHER);
}

unsafe fn stopAllCapabilitiesWith(
    mut pCap: *mut *mut Capability,
    mut task: *mut Task,
    mut sync_type: SyncType,
) {
    let mut was_syncing: bool = false;
    let mut prev_sync_type = SYNC_OTHER;

    let mut sync = PendingSync {
        r#type: sync_type,
        idle: null_mut::<bool>(),
        task: task,
    };

    loop {
        was_syncing = requestSync(pCap, task, &raw mut sync, &raw mut prev_sync_type);

        if !was_syncing {
            break;
        }
    }

    acquireAllCapabilities(
        if !pCap.is_null() {
            *pCap
        } else {
            null_mut::<Capability>()
        },
        task,
    );

    (&raw mut pending_sync).store(null_mut::<PendingSync>(), Ordering::Relaxed);
    signalCondition(&raw mut sync_finished_cond);
}

unsafe fn requestSync(
    mut pcap: *mut *mut Capability,
    mut task: *mut Task,
    mut new_sync: *mut PendingSync,
    mut prev_sync_type: *mut SyncType,
) -> bool {
    let mut sync = null_mut::<PendingSync>();

    sync = cas(
        &raw mut pending_sync as StgVolatilePtr,
        NULL as StgWord,
        new_sync as StgWord,
    ) as *mut PendingSync;

    if !sync.is_null() {
        *prev_sync_type = (*sync).r#type;

        if pcap.is_null() {
            let mut __r = pthread_mutex_lock(&raw mut sync_finished_mutex);

            if __r != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/Schedule.c".as_ptr(),
                    1534,
                    __r,
                );
            }

            while !pending_sync.is_null() {
                waitCondition(&raw mut sync_finished_cond, &raw mut sync_finished_mutex);
            }

            if pthread_mutex_unlock(&raw mut sync_finished_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Schedule.c".as_ptr(),
                    1538,
                );
            }
        } else {
            loop {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(
                        c"someone else is trying to sync (%d)...".as_ptr(),
                        (*sync).r#type as u32,
                    );
                }

                if !(*pcap).is_null() as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 1543);
                }

                yieldCapability(pcap, task, true);
                sync = (&raw mut pending_sync).load(Ordering::SeqCst);

                if sync.is_null() {
                    break;
                }
            }
        }

        return true;
    } else {
        return false;
    };
}

unsafe fn acquireAllCapabilities(mut cap: *mut Capability, mut task: *mut Task) {
    let mut tmpcap = null_mut::<Capability>();
    let mut i: u32 = 0;

    if !(&raw mut pending_sync).load(Ordering::SeqCst).is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 1576);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"grabbing all the capabilities (%d/%d)".as_ptr(),
                i,
                getNumCapabilities(),
            );
        }

        tmpcap = getCapability(i);

        if tmpcap != cap {
            (*task).cap = tmpcap as *mut Capability_;
            waitForCapability(&raw mut tmpcap, task);

            if (*tmpcap).no != i {
                barf(c"acquireAllCapabilities: got the wrong capability".as_ptr());
            }
        }

        i = i.wrapping_add(1);
    }

    (*task).cap = (if cap.is_null() { tmpcap } else { cap }) as *mut Capability_;
}

unsafe fn releaseAllCapabilities(mut n: u32, mut keep_cap: *mut Capability, mut task: *mut Task) {
    let mut i: u32 = 0;

    if !task.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 1609);
    }

    i = 0;

    while i < n {
        let mut tmpcap = getCapability(i);

        if keep_cap != tmpcap {
            (*task).cap = tmpcap as *mut Capability_;
            releaseCapability(tmpcap);
        }

        i = i.wrapping_add(1);
    }

    (*task).cap = keep_cap as *mut Capability_;
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
    let mut collect_gen: u32 = 0;
    let mut major_gc: bool = false;
    let mut gc_type: u32 = 0;
    let mut i: u32 = 0;
    let mut need_idle: u32 = 0;
    let mut n_gc_threads: u32 = 0;
    let mut n_idle_caps: u32 = 0;
    let mut n_failed_trygrab_idles: u32 = 0;
    let mut tso = null_mut::<StgTSO>();
    let mut idle_cap = null_mut::<bool>();

    if getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32 {
        return;
    }

    heap_census = scheduleNeedHeapProfile(true);

    let mut mblock_overflow = RtsFlags.GcFlags.maxHeapSize != 0
        && mblocks_allocated
            > (1 as W_).wrapping_add(
                (((RtsFlags.GcFlags.maxHeapSize as W_)
                    .wrapping_sub(
                        (((1 as u64) << 20 as i32) as W_)
                            .wrapping_sub(
                                ((0x40 as u64).wrapping_mul(
                                    ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                                ) as W_)
                                    .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                    .wrapping_sub(1 as W_)
                                    & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                            )
                            .wrapping_div(((1 as u64) << 12 as i32) as W_),
                    )
                    .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                    .wrapping_add(MBLOCK_SIZE as W_)
                    .wrapping_sub(1 as W_)
                    & !MBLOCK_MASK as W_) as *mut c_void as W_)
                    .wrapping_div(MBLOCK_SIZE as W_),
            );

    collect_gen = calcNeeded(
        force_major as i32 != 0 || heap_census as i32 != 0 || mblock_overflow as i32 != 0,
        null_mut::<StgWord>(),
    ) as u32;

    major_gc = collect_gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as u32);

    if (getSchedState() as u32) < SCHED_INTERRUPTING as i32 as u32
        && RtsFlags.ParFlags.parGcEnabled as i32 != 0
        && collect_gen >= RtsFlags.ParFlags.parGcGen
        && (*oldest_gen).mark == 0
    {
        gc_type = SYNC_GC_PAR as i32 as u32;
    } else {
        gc_type = SYNC_GC_SEQ as i32 as u32;
    }

    let mut sync = PendingSync {
        r#type: gc_type as SyncType,
        idle: null_mut::<bool>(),
        task: task,
    };

    let mut prev_sync = SYNC_OTHER;
    let mut was_syncing: bool = false;

    loop {
        n_gc_threads = RtsFlags.ParFlags.parGcThreads;

        if n_gc_threads == 0 && enabled_capabilities > getNumberOfProcessors() {
            n_gc_threads = getNumberOfProcessors();
        }

        if gc_type == SYNC_GC_PAR as i32 as u32 && n_gc_threads > 0 {
            if n_gc_threads >= enabled_capabilities {
                need_idle = 0;
            } else {
                need_idle = enabled_capabilities.wrapping_sub(n_gc_threads);
            }
        } else {
            need_idle = 0;
        }

        idle_cap = stgMallocBytes(
            (getNumCapabilities() as usize).wrapping_mul(size_of::<bool>() as usize),
            c"scheduleDoGC".as_ptr(),
        ) as *mut bool;

        sync.idle = idle_cap;

        let mut n_idle: u32 = need_idle;
        i = 0;

        while i < getNumCapabilities() as u32 {
            if (*getCapability(i)).disabled {
                *idle_cap.offset(i as isize) = true;
            } else if n_idle > 0 && (*getCapability(i)).running_task.is_null() {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(c"asking for cap %d to be idle".as_ptr(), i);
                }

                n_idle = n_idle.wrapping_sub(1);
                *idle_cap.offset(i as isize) = true;
            } else {
                *idle_cap.offset(i as isize) = false;
            }

            i = i.wrapping_add(1);
        }

        i = 0;

        while n_idle > 0 && i < getNumCapabilities() as u32 {
            if !*idle_cap.offset(i as isize) && i != (*cap).no {
                *idle_cap.offset(i as isize) = true;
                n_idle = n_idle.wrapping_sub(1);
            }

            i = i.wrapping_add(1);
        }

        if (n_idle == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 1754);
        }

        was_syncing = requestSync(pcap, task, &raw mut sync, &raw mut prev_sync);
        cap = *pcap;

        if was_syncing {
            stgFree(idle_cap as *mut c_void);
        }

        if was_syncing as i32 != 0
            && (prev_sync as u32 == SYNC_GC_SEQ as i32 as u32
                || prev_sync as u32 == SYNC_GC_PAR as i32 as u32)
            && !(getSchedState() as u32 == SCHED_INTERRUPTING as i32 as u32
                && force_major as i32 != 0)
        {
            return;
        }

        if getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32 {
            return;
        }

        if !was_syncing {
            break;
        }
    }

    stat_startGCSync(*gc_threads.offset((*cap).no as isize) as *mut gc_thread_);

    let mut old_n_capabilities = getNumCapabilities();
    interruptAllCapabilities();

    if gc_type == SYNC_GC_SEQ as i32 as u32 {
        traceEventRequestSeqGc(cap);
    } else {
        traceEventRequestParGc(cap);
    }

    if gc_type == SYNC_GC_SEQ as i32 as u32 {
        acquireAllCapabilities(cap, task);
    } else {
        if RtsFlags.ParFlags.parGcNoSyncWithIdle == 0
            || RtsFlags.ParFlags.parGcLoadBalancingEnabled as i32 != 0
                && collect_gen >= RtsFlags.ParFlags.parGcLoadBalancingGen
        {
            i = 0;

            while i < n_capabilities {
                if (*getCapability(i)).disabled {
                    *idle_cap.offset(i as isize) = tryGrabCapability(getCapability(i), task);

                    if *idle_cap.offset(i as isize) {
                        n_idle_caps = n_idle_caps.wrapping_add(1);
                    }
                } else if i != (*cap).no && *idle_cap.offset(i as isize) as i32 != 0 {
                    let mut tmpcap = getCapability(i);
                    (*task).cap = tmpcap as *mut Capability_;
                    waitForCapability(&raw mut tmpcap, task);
                    n_idle_caps = n_idle_caps.wrapping_add(1);
                }

                i = i.wrapping_add(1);
            }
        } else {
            i = 0;

            while i < n_capabilities {
                if (*getCapability(i)).disabled {
                    *idle_cap.offset(i as isize) = tryGrabCapability(getCapability(i), task);

                    if *idle_cap.offset(i as isize) {
                        n_idle_caps = n_idle_caps.wrapping_add(1);
                    }
                } else if i != (*cap).no
                    && (*getCapability(i)).idle >= RtsFlags.ParFlags.parGcNoSyncWithIdle
                {
                    *idle_cap.offset(i as isize) = tryGrabCapability(getCapability(i), task);

                    if *idle_cap.offset(i as isize) {
                        n_idle_caps = n_idle_caps.wrapping_add(1);
                    } else {
                        n_failed_trygrab_idles = n_failed_trygrab_idles.wrapping_add(1);
                    }
                }

                i = i.wrapping_add(1);
            }
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"%d idle caps, %d failed grabs".as_ptr(),
                n_idle_caps,
                n_failed_trygrab_idles,
            );
        }

        i = 0;

        while i < n_capabilities {
            (&raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i))
                .idle)
                .store(
                    (&raw mut (*(getCapability
                        as unsafe extern "C" fn(c_uint) -> *mut Capability)(
                        i
                    ))
                    .idle)
                        .load(Ordering::Relaxed)
                        .wrapping_add(1 as u32),
                    Ordering::Relaxed,
                );

            i = i.wrapping_add(1);
        }

        waitForGcThreads(cap, idle_cap as *mut bool);

        if checkSparkCountInvariant() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 1860);
        }
    }

    if RtsFlags.DebugFlags.scheduler {
        printAllThreads();
    }

    loop {
        if getSchedState() as u32 == SCHED_INTERRUPTING as i32 as u32 && major_gc as i32 != 0 {
            deleteAllThreads();
            i = 0;

            while i < n_capabilities {
                let ref mut fresh8 = (*getCapability(i)).spark_stats.gcd;
                *fresh8 =
                    (*fresh8).wrapping_add(sparkPoolSize((*getCapability(i)).sparks) as StgWord);
                discardSparksCap(getCapability(i));
                i = i.wrapping_add(1);
            }

            setSchedState(SCHED_SHUTTING_DOWN);
        }

        i = enabled_capabilities;

        while i < n_capabilities {
            let mut tmp_cap = null_mut::<Capability>();
            let mut dest_cap = null_mut::<Capability>();
            tmp_cap = getCapability(i);

            if (*tmp_cap).disabled as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Schedule.c".as_ptr(), 1902);
            }

            if i != (*cap).no {
                dest_cap = getCapability(i.wrapping_rem(enabled_capabilities));

                while !emptyRunQueue(tmp_cap) {
                    tso = popRunQueue(tmp_cap);
                    migrateThread(tmp_cap, tso, dest_cap);

                    if !(*tso).bound.is_null() {
                        traceTaskMigrate(
                            (*(*tso).bound).task as *mut Task,
                            (*(*(*tso).bound).task).cap as *mut Capability,
                            dest_cap,
                        );

                        (*(*(*tso).bound).task).cap = dest_cap as *mut Capability_;
                    }
                }
            }

            i = i.wrapping_add(1);
        }

        doIdleGCWork(cap, true);

        config = GcConfig {
            collect_gen: collect_gen,
            do_heap_census: heap_census,
            overflow_gc: is_overflow_gc,
            deadlock_detect: deadlock_detect,
            nonconcurrent: nonconcurrent,
            parallel: false,
        };

        (&raw mut pending_sync).store(null_mut::<PendingSync>(), Ordering::Relaxed);
        signalCondition(&raw mut sync_finished_cond);
        config.parallel = gc_type == SYNC_GC_PAR as i32 as u32;
        GarbageCollect(config, cap, idle_cap as *mut bool);

        if getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32 {
            doIdleGCWork(cap, true);
        }

        traceSparkCounters(cap);

        let mut current_block_172: u64;

        match getRecentActivity() as u32 {
            2 => {
                if force_major {
                    setRecentActivity(ACTIVITY_DONE_GC);
                    current_block_172 = 14184516523743666873;
                } else {
                    current_block_172 = 8925939473551295275;
                }
            }
            1 => {
                current_block_172 = 8925939473551295275;
            }
            3 | 0 | _ => {
                current_block_172 = 14184516523743666873;
            }
        }

        match current_block_172 {
            8925939473551295275 => {
                setRecentActivity(ACTIVITY_YES);
            }
            _ => {}
        }

        if checkSparkCountInvariant() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 1980);
        }

        if heap_census {
            (&raw mut performHeapProfile).store(0 != 0, Ordering::Relaxed);
        }

        if (n_capabilities == old_n_capabilities as u32) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 1991);
        }

        if gc_type == SYNC_GC_PAR as i32 as u32 {
            i = 0;

            while i < n_capabilities {
                if i != (*cap).no {
                    if *idle_cap.offset(i as isize) {
                        if ((*getCapability(i)).running_task == task) as i32 as i64 != 0 {
                        } else {
                            _assertFail(c"rts/Schedule.c".as_ptr(), 1998);
                        }

                        (*task).cap = getCapability(i) as *mut Capability_;
                        releaseCapability(getCapability(i));
                    } else if ((*getCapability(i)).running_task != task) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/Schedule.c".as_ptr(), 2002);
                    }
                }

                i = i.wrapping_add(1);
            }

            (*task).cap = cap as *mut Capability_;
            releaseGCThreads(cap, idle_cap as *mut bool);
        }

        if !(heap_overflow as i32 != 0 && getSchedState() as u32 == SCHED_RUNNING as i32 as u32) {
            break;
        }

        let mut main_thread = getTopHandlerThread();

        if main_thread.is_null() {
            setSchedState(SCHED_INTERRUPTING);
        } else {
            heap_overflow = false;

            let allocation_count = getAllocations() as u64;

            if RtsFlags.GcFlags.heapLimitGrace
                < allocation_count.wrapping_sub(allocated_bytes_at_heapoverflow)
                || allocated_bytes_at_heapoverflow == 0
            {
                allocated_bytes_at_heapoverflow = allocation_count;
                throwToSelf(cap, main_thread, (*ghc_hs_iface).heapOverflow_closure);
            }

            break;
        }
    }

    stgFree(idle_cap as *mut c_void);

    if gc_type == SYNC_GC_SEQ as i32 as u32 {
        releaseAllCapabilities(n_capabilities, cap, task);
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
    let mut g: u32 = 0;
    let mut task = null_mut::<Task>();
    let mut i: u32 = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"forking!".as_ptr());
    }

    task = newBoundTask();
    cap = null_mut::<Capability>();
    waitForCapability(&raw mut cap, task);
    stopAllCapabilities(&raw mut cap, task);

    let mut __r = pthread_mutex_lock(&raw mut sched_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2106,
            __r,
        );
    }

    let mut __r_0 = pthread_mutex_lock(&raw mut sm_mutex);

    if __r_0 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2107,
            __r_0,
        );
    }

    let mut __r_1 = pthread_mutex_lock(&raw mut stable_ptr_mutex);

    if __r_1 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2108,
            __r_1,
        );
    }

    let mut __r_2 = pthread_mutex_lock(&raw mut stable_name_mutex);

    if __r_2 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2109,
            __r_2,
        );
    }

    i = 0;

    while i < n_capabilities {
        let mut __r_3 = pthread_mutex_lock(
            &raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i)).lock,
        );

        if __r_3 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2112,
                __r_3,
            );
        }

        i = i.wrapping_add(1);
    }

    let mut __r_4 = pthread_mutex_lock(&raw mut (*task).lock);

    if __r_4 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2116,
            __r_4,
        );
    }

    let mut __r_5 = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r_5 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2119,
            __r_5,
        );
    }

    stopTimer();
    flushAllCapsEventsBufs();
    pid = fork();

    if pid != 0 {
        startTimer();

        if pthread_mutex_unlock(&raw mut sched_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2134,
            );
        }

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2135,
            );
        }

        if pthread_mutex_unlock(&raw mut stable_ptr_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2136,
            );
        }

        if pthread_mutex_unlock(&raw mut stable_name_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2137,
            );
        }

        if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2138,
            );
        }

        if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2142,
            );
        }

        i = 0;

        while i < n_capabilities {
            releaseCapability_(getCapability(i), false);

            if pthread_mutex_unlock(
                &raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i))
                    .lock,
            ) != 0
            {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Schedule.c".as_ptr(),
                    2147,
                );
            }

            i = i.wrapping_add(1);
        }

        exitMyTask();

        return pid;
    } else {
        resetChildProcessStats();
        initMutex(&raw mut sched_mutex);
        initMutex(&raw mut sm_mutex);
        initMutex(&raw mut stable_ptr_mutex);
        initMutex(&raw mut stable_name_mutex);
        initMutex(&raw mut (*task).lock);
        i = 0;

        while i < n_capabilities {
            initMutex(
                &raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(i))
                    .lock,
            );

            i = i.wrapping_add(1);
        }

        initMutex(&raw mut all_tasks_mutex);
        resetTracing();
        g = 0;

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
        i = 0;

        while i < n_capabilities {
            cap = getCapability(i);
            truncateRunQueue(cap);
            (*cap).n_run_queue = 0;
            (*cap).suspended_ccalls = null_mut::<InCall>();
            (*cap).n_suspended_ccalls = 0;
            (*cap).spare_workers = null_mut::<Task>();
            (*cap).n_spare_workers = 0;
            (*cap).returning_tasks_hd = null_mut::<Task>();
            (*cap).returning_tasks_tl = null_mut::<Task>();
            (*cap).n_returning_tasks = 0;

            if (*cap).no != 0 {
                (*task).cap = cap as *mut Capability_;
                releaseCapability(cap);
            }

            i = i.wrapping_add(1);
        }

        cap = getCapability(0);
        (*task).cap = cap as *mut Capability_;
        g = 0;

        while g < RtsFlags.GcFlags.generations {
            let ref mut fresh11 = (*generations.offset(g as isize)).threads;
            *fresh11 = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
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

        rts_checkSchedStatus(c"forkProcess".as_ptr(), cap);
        rts_unlock(cap);
        shutdownHaskellAndExit(EXIT_SUCCESS, 0);
    };
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setNumCapabilities(mut new_n_capabilities: c_uint) {
    let mut task = null_mut::<Task>();
    let mut cap = null_mut::<Capability>();
    let mut n: u32 = 0;
    let mut old_capabilities = null_mut::<Capability>();
    let mut old_n_capabilities: u32 = n_capabilities;

    if new_n_capabilities == enabled_capabilities {
        return;
    } else if new_n_capabilities <= 0 {
        errorBelch(c"setNumCapabilities: Capability count must be positive".as_ptr());
        return;
    } else if new_n_capabilities > max_n_capabilities {
        errorBelch(
            c"setNumCapabilities: Attempt to increase capability count beyond maximum capability count %u; clamping...\n"
                .as_ptr(),
            max_n_capabilities,
        );

        new_n_capabilities = max_n_capabilities;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"changing the number of Capabilities from %d to %d".as_ptr(),
            enabled_capabilities,
            new_n_capabilities,
        );
    }

    cap = rts_lock();
    task = (*cap).running_task;
    stopTimer();
    stopAllCapabilities(&raw mut cap, task);

    if new_n_capabilities < enabled_capabilities {
        n = new_n_capabilities;

        while n < enabled_capabilities {
            (*getCapability(n)).disabled = true;
            traceCapDisable(getCapability(n));
            n = n.wrapping_add(1);
        }

        enabled_capabilities = new_n_capabilities;
    } else {
        n = enabled_capabilities;

        while n < new_n_capabilities && n < n_capabilities {
            (*getCapability(n)).disabled = false;
            traceCapEnable(getCapability(n));
            n = n.wrapping_add(1);
        }

        enabled_capabilities = n;

        if new_n_capabilities > n_capabilities {
            tracingAddCapabilities(n_capabilities, new_n_capabilities);
            moreCapabilities(n_capabilities, new_n_capabilities);

            let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

            if __r != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/Schedule.c".as_ptr(),
                    2392,
                    __r,
                );
            }

            storageAddCapabilities(n_capabilities, new_n_capabilities);

            if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Schedule.c".as_ptr(),
                    2394,
                );
            }
        }
    }

    if new_n_capabilities > n_capabilities {
        (&raw mut n_capabilities).store(new_n_capabilities, Ordering::Relaxed);
        (&raw mut enabled_capabilities).store(new_n_capabilities, Ordering::Relaxed);
    }

    releaseAllCapabilities(old_n_capabilities, cap, task);

    if !old_capabilities.is_null() {
        stgFree(old_capabilities as *mut c_void);
    }

    notifyIOManagerCapabilitiesChanged(&raw mut cap);
    startTimer();
    rts_unlock(cap);
}

unsafe fn deleteAllThreads() {
    let mut t = null_mut::<StgTSO>();
    let mut next = null_mut::<StgTSO>();
    let mut g: u32 = 0;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"deleting all threads".as_ptr());
    }

    g = 0;

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

unsafe fn suspendTask(mut cap: *mut Capability, mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    incall = (*task).incall as *mut InCall;

    if ((*incall).next.is_null() && (*incall).prev.is_null()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2463);
    }

    (*incall).next = (*cap).suspended_ccalls as *mut InCall_;
    (*incall).prev = null_mut::<InCall_>();

    if !(*cap).suspended_ccalls.is_null() {
        (*(*cap).suspended_ccalls).prev = incall as *mut InCall_;
    }

    (*cap).suspended_ccalls = incall;
    (*cap).n_suspended_ccalls = (*cap).n_suspended_ccalls.wrapping_add(1);
}

unsafe fn recoverSuspendedTask(mut cap: *mut Capability, mut task: *mut Task) {
    let mut incall = null_mut::<InCall>();
    incall = (*task).incall as *mut InCall;

    if !(*incall).prev.is_null() {
        (*(*incall).prev).next = (*incall).next;
    } else {
        if ((*cap).suspended_ccalls == incall) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 2482);
        }

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
    let mut saved_errno: i32 = 0;
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
        0,
    );

    (&raw mut (*tso).what_next).store(1, Ordering::Relaxed);
    threadPaused(cap, tso);

    if interruptible_0 {
        (*tso).why_blocked = BlockedOnCCall_Interruptible as StgWord32;
    } else {
        (*tso).why_blocked = BlockedOnCCall as StgWord32;
    }

    (*(*task).incall).suspended_tso = tso;
    (*(*task).incall).suspended_cap = cap;
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2554,
            __r,
        );
    }

    suspendTask(cap, task);
    (*cap).in_haskell = false;
    releaseCapability_(cap, false);

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2560,
        );
    }

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
    let mut saved_errno: i32 = 0;
    saved_errno = *__error();
    incall = (*task).incall as *mut InCall;
    cap = (*incall).suspended_cap;
    (*task).cap = cap as *mut Capability_;
    waitForCapability(&raw mut cap, task);
    recoverSuspendedTask(cap, task);
    tso = (*incall).suspended_tso;
    (*incall).suspended_tso = null_mut::<StgTSO>();
    (*incall).suspended_cap = null_mut::<Capability>();

    if nonmoving_write_barrier_enabled as i64 != 0 {
        updateRemembSetPushClosure(cap, (*tso)._link as *mut StgClosure);
    }

    (*tso)._link = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgTSO_;
    traceEventRunThread(cap, tso);
    (*tso).why_blocked = NotBlocked as StgWord32;

    if (*tso).flags & TSO_BLOCKEX as StgWord32 == 0 {
        if (*tso).blocked_exceptions
            != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                as *mut MessageThrowTo
        {
            maybePerformBlockedException(cap, tso);
        }
    }

    (*cap).r.rCurrentTSO = tso as *mut StgTSO_;
    (*cap).in_haskell = true;
    *__error() = saved_errno;
    dirty_TSO(cap, tso);
    dirty_STACK(cap, (*tso).stackobj as *mut StgStack);

    if RtsFlags.DebugFlags.sanity {
        checkTSO(tso);
    }

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
    cpu = cpu.wrapping_rem(enabled_capabilities as StgWord);

    if cpu == (*cap).no as StgWord {
        appendToRunQueue(cap, tso);
    } else {
        migrateThread(cap, tso, getCapability(cpu as u32));
    }

    contextSwitchCapability(getCapability(cpu as u32), false);
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

    if 1 != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"new bound thread (%llu)".as_ptr(), (*tso).id);
    }

    cap = schedule(cap, task);

    if ((*(*task).incall).rstat as u32 != NoStatus as i32 as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2714);
    }

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2715);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"bound thread (%llu) finished".as_ptr(), (*tso).id);
    }

    *pcap = cap;
}

unsafe fn scheduleWorker(mut cap: *mut Capability, mut task: *mut Task) {
    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2728);
    }

    cap = schedule(cap, task);

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2730);
    }

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2744,
            __r,
        );
    }

    releaseCapability_(cap, false);
    workerTaskStop(task);

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2747,
        );
    }
}

unsafe fn startWorkerTasks(mut from: u32, mut to: u32) {
    let mut i: u32 = 0;
    let mut cap = null_mut::<Capability>();
    i = from;

    while i < to {
        cap = getCapability(i);

        let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2764,
                __r,
            );
        }

        startWorkerTask(cap);

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Schedule.c".as_ptr(),
                2766,
            );
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn initScheduler() {
    setSchedState(SCHED_RUNNING);
    setRecentActivity(ACTIVITY_YES);
    initMutex(&raw mut sched_mutex);
    initMutex(&raw mut sync_finished_mutex);
    initCondition(&raw mut sync_finished_cond);

    let mut __r = pthread_mutex_lock(&raw mut sched_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2795,
            __r,
        );
    }

    allocated_bytes_at_heapoverflow = 0;
    initCapabilities();
    initTaskManager();
    startWorkerTasks(1, n_capabilities);

    if pthread_mutex_unlock(&raw mut sched_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2815,
        );
    }
}

unsafe fn exitScheduler(mut wait_foreign: bool) {
    let mut task = newBoundTask();

    if (getSchedState() as u32) < SCHED_SHUTTING_DOWN as i32 as u32 {
        setSchedState(SCHED_INTERRUPTING);

        let mut cap = (*task).cap as *mut Capability;
        waitForCapability(&raw mut cap, task);
        scheduleDoGC(&raw mut cap, task, true, false, false, true);

        if (*(*task).incall).tso.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Schedule.c".as_ptr(), 2831);
        }

        releaseCapability(cap);
    }

    if (getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2834);
    }

    shutdownCapabilities(task, wait_foreign);
    exitMyTask();
}

unsafe fn freeScheduler() {
    let mut still_running: u32 = 0;
    let mut __r = pthread_mutex_lock(&raw mut sched_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2846,
            __r,
        );
    }

    still_running = freeTaskManager();

    if still_running == 0 {
        freeCapabilities();
    }

    if pthread_mutex_unlock(&raw mut sched_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Schedule.c".as_ptr(),
            2857,
        );
    }

    closeMutex(&raw mut sched_mutex);
}

unsafe fn performGC_(mut force_major: bool, mut nonconcurrent: bool) {
    let mut task = null_mut::<Task>();
    let mut cap = null_mut::<Capability>();
    task = newBoundTask();
    waitForCapability(&raw mut cap, task);
    scheduleDoGC(&raw mut cap, task, force_major, false, false, nonconcurrent);
    releaseCapability(cap);
    exitMyTask();
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performGC() {
    performGC_(false, false);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    performGC_(true, false);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    performGC_(true, true);
}

unsafe fn interruptStgRts() {
    if (getSchedState() as u32 != SCHED_SHUTTING_DOWN as i32 as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2916);
    }

    setSchedState(SCHED_INTERRUPTING);
    interruptAllCapabilities();
    wakeUpRts();
}

unsafe fn wakeUpRts() {
    wakeupIOManager();
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
    if ((*tso)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32
        as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 2992);
    }

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
    if ((*cap).n_run_queue > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 3021);
    }

    let mut t = (*cap).run_queue_hd;

    if (t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Schedule.c".as_ptr(), 3023);
    }

    (*cap).run_queue_hd = (*t)._link as *mut StgTSO;

    let mut link = (&raw mut (*t)._link).load(Ordering::Relaxed);

    if link != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        (*link).block_info.prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    }

    (&raw mut (*t)._link).store(
        &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO,
        Ordering::Relaxed,
    );

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

                    let ref mut fresh17 = (*(raise_closure as *mut StgClosure)).header.prof.ccs;
                    *fresh17 = (*cap).r.rCCCS as *mut CostCentreStack;

                    if doingLDVProfiling() {
                        if doingLDVProfiling() {
                            (*(raise_closure as *mut StgClosure)).header.prof.hp.ldvw =
                                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
                        }
                    } else if doingRetainerProfiling() {
                        (*(raise_closure as *mut StgClosure)).header.prof.hp.trav = 0;
                    } else if doingErasProfiling() {
                        (*(raise_closure as *mut StgClosure)).header.prof.hp.era = user_era;
                    }

                    (&raw mut (*raise_closure).header.info)
                        .store(&raw const stg_raise_info, Ordering::Relaxed);

                    let ref mut fresh18 =
                        *(&raw mut (*raise_closure).payload as *mut *mut StgClosure_).offset(0);
                    *fresh18 = exception as *mut StgClosure_;
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
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found ATOMICALLY_FRAME at %p".as_ptr(), p);
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            34 => {
                (*(*tso).stackobj).sp = p;

                return CATCH_FRAME as StgWord;
            }
            57 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found CATCH_STM_FRAME at %p".as_ptr(), p);
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

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found CATCH_RETRY_FRAME at %p during raise".as_ptr(), p);
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"trec=%p outer=%p".as_ptr(), trec, outer);
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
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found ATOMICALLY_FRAME at %p during retry".as_ptr(), p);
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            56 => {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found CATCH_RETRY_FRAME at %p during retry".as_ptr(), p);
                }

                (*(*tso).stackobj).sp = p;

                return CATCH_RETRY_FRAME as StgWord;
            }
            57 => {
                let mut trec = (*tso).trec as *mut StgTRecHeader;
                let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"found CATCH_STM_FRAME at %p during retry".as_ptr(), p);
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"trec=%p outer=%p".as_ptr(), trec, outer);
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
                if ((*info).i.r#type != 34) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 3231);
                }

                if ((*info).i.r#type != 36) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 3232);
                }

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
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(
                        c"found ATOMICALLY_FRAME at %p while aborting after orElse".as_ptr(),
                        p,
                    );
                }

                (*(*tso).stackobj).sp = p;

                return ATOMICALLY_FRAME as StgWord;
            }
            56 => {
                let mut trec = (*tso).trec as *mut StgTRecHeader;
                let mut outer = (*trec).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(
                        c"found CATCH_RETRY_FRAME at %p while aborting after orElse".as_ptr(),
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"trec=%p outer=%p".as_ptr(), trec, outer);
                }

                stmAbortTransaction(cap, trec);
                stmFreeAbortedTRec(cap, trec);
                (*tso).trec = outer as *mut StgTRecHeader_;
                p = next;
            }
            57 => {
                let mut trec_0 = (*tso).trec as *mut StgTRecHeader;
                let mut outer_0 = (*trec_0).enclosing_trec as *mut StgTRecHeader;

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(
                        c"found CATCH_STM_FRAME at %p while aborting after orElse".as_ptr(),
                        p,
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stm as i64 != 0 {
                    trace_(c"trec=%p outer=%p".as_ptr(), trec_0, outer_0);
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
                if ((*info).i.r#type != 34) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 3297);
                }

                if ((*info).i.r#type != 36) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Schedule.c".as_ptr(), 3298);
                }

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

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"resurrecting thread %llu".as_ptr(), (*tso).id);
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
                    c"resurrectThreads: thread blocked in a strange way: %d".as_ptr(),
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
