use crate::event_log_constants::{
    EVENT_CONC_MARK_BEGIN, EVENT_CONC_SWEEP_BEGIN, EVENT_CONC_SWEEP_END, EVENT_CONC_SYNC_BEGIN,
    EVENT_CONC_SYNC_END, EVENT_OSPROCESS_PID, EVENT_OSPROCESS_PPID, EVENT_PROGRAM_ARGS,
    EVENT_RTS_IDENTIFIER, EVENT_USER_BINARY_MSG, EVENT_USER_MARKER, EVENT_USER_MSG,
};
use crate::eventlog::event_log::{eventlog_enabled, flushLocalEventsBuf};
use crate::eventlog::event_log::{
    eventlog_enabled, flushLocalEventsBuf, freeEventLogging, initEventLogging, moreCapEventBufs,
    postCapEvent, postCapMsg, postCapsetEvent, postCapsetStrEvent, postCapsetVecEvent,
    postConcMarkEnd, postConcUpdRemSetFlush, postEvent, postEventAtTimestamp, postEventGcStats,
    postEventHeapInfo, postEventMemReturn, postEventNoCap, postHeapBioProfSampleBegin,
    postHeapEvent, postHeapProfBegin, postHeapProfCostCentre, postHeapProfSampleBegin,
    postHeapProfSampleCostCentre, postHeapProfSampleEnd, postHeapProfSampleString, postIPE,
    postMsg, postNonmovingHeapCensus, postNonmovingPrunedSegments, postProfBegin,
    postProfSampleCostCentre, postSchedEvent, postSparkCountersEvent, postSparkEvent,
    postTaskCreateEvent, postTaskDeleteEvent, postTaskMigrateEvent, postThreadLabel,
    postUserBinaryEvent, postUserEvent, postWallClockTime, restartEventLogging,
};
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{BlockedOnBlackHole, StackOverflow};
use crate::ffi::rts::event_log_format::EventTypeNum;
use crate::ffi::rts::event_log_format::{
    EventCapNo, EventCapsetID, EventKernelThreadId, EventTaskId, EventThreadID, EventTimestamp,
    EventTypeNum,
};
use crate::ffi::rts::event_log_writer::{
    NullEventLogWriter, endEventLogging, flushEventLog, startEventLogging,
};
use crate::ffi::rts::flags::{
    COLLECT_GC_STATS, NO_GC_STATS, RtsFlags, TRACE_EVENTLOG, TRACE_STDERR,
};
use crate::ffi::rts::ipe::{InfoProvEnt, formatClosureDescIpe};
use crate::ffi::rts::messages::{barf, debugBelch, vdebugBelch};
use crate::ffi::rts::os_threads::{Mutex, initMutex, kernelThreadId, osThreadId};
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::tso::{StgThreadID, StgThreadReturnCode};
use crate::ffi::rts::storage::tso::{StgThreadID, StgThreadReturnCode};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::{Capability, getFullProgArgv};
use crate::ffi::stg::W_;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::ffi::stg::types::{StgWord, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;
use crate::printer::what_next_strs;
use crate::rts_flags::rtsConfig;
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::sparks::sparkPoolSize;
use crate::sparks::{SparkCounters, sparkPoolSize};
use crate::stats::stat_getElapsedTime;
use crate::task::Task;
use crate::task::{Task, serialisableTaskId};
use crate::threads::printThreadStatus;
use crate::trace::{
    CAPSET_CLOCKDOMAIN_DEFAULT, CAPSET_HEAP_DEFAULT, CAPSET_OSPROCESS_DEFAULT, CapsetID, CapsetType,
};

pub(crate) type CapsetID = StgWord32;

pub(crate) type CapsetType = StgWord16;

pub(crate) const CAPSET_OSPROCESS_DEFAULT: CapsetID = 0;

pub(crate) const CAPSET_HEAP_DEFAULT: CapsetID = 0;

pub(crate) const CAPSET_CLOCKDOMAIN_DEFAULT: CapsetID = 1;

#[inline]
pub(crate) unsafe fn traceEventCreateThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if TRACE_sched as i64 != 0 {
        traceSchedEvent_(cap, 0, tso, (*(*tso).stackobj).stack_size as StgWord, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventRunThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if TRACE_sched as i64 != 0 {
        traceSchedEvent_(cap, 1, tso, (*tso).what_next as StgWord, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventStopThread(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut status: StgThreadReturnCode,
    mut info: StgWord32,
) {
    if TRACE_sched as i64 != 0 {
        traceSchedEvent_(cap, 2, tso, status as StgWord, info as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventMigrateThread(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut new_cap: u32,
) {
    if TRACE_sched as i64 != 0 {
        traceSchedEvent_(cap, 4, tso, new_cap as StgWord, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceCapCreate(mut cap: *mut Capability) {
    if TRACE_cap as i64 != 0 {
        traceCapEvent_(cap, 45);
    }
}

#[inline]
pub(crate) unsafe fn traceCapDelete(mut cap: *mut Capability) {
    if TRACE_cap as i64 != 0 {
        traceCapEvent_(cap, 46);
    }
}

#[inline]
pub(crate) unsafe fn traceCapEnable(mut cap: *mut Capability) {
    if TRACE_cap as i64 != 0 {
        traceCapEvent_(cap, 48);
    }
}

#[inline]
pub(crate) unsafe fn traceCapDisable(mut cap: *mut Capability) {
    if TRACE_cap as i64 != 0 {
        traceCapEvent_(cap, 47);
    }

    if eventlog_enabled {
        flushLocalEventsBuf(cap);
    }
}

#[inline]
pub(crate) unsafe fn traceEventThreadWakeup(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut other_cap: u32,
) {
    if TRACE_sched as i64 != 0 {
        traceSchedEvent_(cap, 8, tso, other_cap as StgWord, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceThreadLabel(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut label: *mut c_char,
    mut len: usize,
) {
    if TRACE_sched as i64 != 0 {
        traceThreadLabel_(cap, tso, label, len);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStart(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 9);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStartAtT(mut cap: *mut Capability, mut ts: StgWord64) {
    if TRACE_gc as i64 != 0 {
        traceGcEventAtT_(cap, ts, 9);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcEnd(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 10);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcEndAtT(mut cap: *mut Capability, mut ts: StgWord64) {
    if TRACE_gc as i64 != 0 {
        traceGcEventAtT_(cap, ts, 10);
    }
}

#[inline]
pub(crate) unsafe fn traceEventRequestSeqGc(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 11);
    }
}

#[inline]
pub(crate) unsafe fn traceEventRequestParGc(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 12);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcIdle(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 20);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcWork(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 21);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcDone(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 22);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcGlobalSync(mut cap: *mut Capability) {
    if TRACE_gc as i64 != 0 {
        traceGcEvent_(cap, 54);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStats(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut r#gen: u32,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: u32,
    mut par_max_copied: W_,
    mut par_tot_copied: W_,
    mut par_balanced_copied: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceEventGcStats_(
            cap,
            heap_capset,
            r#gen,
            copied,
            slop,
            fragmentation,
            par_n_threads,
            par_max_copied,
            par_tot_copied,
            par_balanced_copied,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventMemReturn(
    mut cap: *mut Capability,
    mut current_mblocks: u32,
    mut needed_mblocks: u32,
    mut returned_mblocks: u32,
) {
    if TRACE_gc as i64 != 0 {
        traceEventMemReturn_(cap, current_mblocks, needed_mblocks, returned_mblocks);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapInfo(
    mut heap_capset: CapsetID,
    mut gens: u32,
    mut maxHeapSize: W_,
    mut allocAreaSize: W_,
    mut mblockSize: W_,
    mut blockSize: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceEventHeapInfo_(
            heap_capset,
            gens,
            maxHeapSize,
            allocAreaSize,
            mblockSize,
            blockSize,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapAllocated(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut allocated: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceHeapEvent_(cap, 49, heap_capset, allocated);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapSize(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_size: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceHeapEvent_(cap, 50, heap_capset, heap_size);
    }
}

#[inline]
pub(crate) unsafe fn traceEventBlocksSize(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_size: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceHeapEvent_(cap, 91, heap_capset, heap_size);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapLive(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_live: W_,
) {
    if TRACE_gc as i64 != 0 {
        traceHeapEvent_(cap, 51, heap_capset, heap_live);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetCreate(mut capset: CapsetID, mut capset_type: CapsetType) {
    if TRACE_cap as i64 != 0 {
        traceCapsetEvent_(25, capset, capset_type as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetDelete(mut capset: CapsetID) {
    if TRACE_cap as i64 != 0 {
        traceCapsetEvent_(26, capset, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetAssignCap(mut capset: CapsetID, mut capno: u32) {
    if TRACE_cap as i64 != 0 {
        traceCapsetEvent_(27, capset, capno as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetRemoveCap(mut capset: CapsetID, mut capno: u32) {
    if TRACE_cap as i64 != 0 {
        traceCapsetEvent_(28, capset, capno as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceWallClockTime() {
    traceWallClockTime_();
}

#[inline]
pub(crate) unsafe fn traceOSProcessInfo() {
    traceOSProcessInfo_();
}

#[inline]
pub(crate) unsafe fn traceEventCreateSparkThread(
    mut cap: *mut Capability,
    mut spark_tid: StgThreadID,
) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 15, spark_tid as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceSparkCounters(mut cap: *mut Capability) {
    if TRACE_spark_sampled as i64 != 0 {
        traceSparkCounters_(
            cap,
            (*cap).spark_stats,
            sparkPoolSize((*cap).sparks) as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkCreate(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 35, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkDud(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 36, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkOverflow(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 37, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkRun(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 38, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkSteal(mut cap: *mut Capability, mut victim_cap: u32) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 39, victim_cap as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkFizzle(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 40, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkGC(mut cap: *mut Capability) {
    if TRACE_spark_full as i64 != 0 {
        traceSparkEvent_(cap, 41, 0);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskCreate(mut task: *mut Task, mut cap: *mut Capability) {
    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 933);
    }

    if !cap.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 937);
    }

    if TRACE_sched as i64 != 0 {
        traceTaskCreate_(task, cap);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskMigrate(
    mut task: *mut Task,
    mut cap: *mut Capability,
    mut new_cap: *mut Capability,
) {
    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 952);
    }

    if !cap.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 953);
    }

    if (cap != new_cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 954);
    }

    if !new_cap.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 955);
    }

    if TRACE_sched as i64 != 0 {
        traceTaskMigrate_(task, cap, new_cap);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskDelete(mut task: *mut Task) {
    if !(*task).cap.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Trace.h".as_ptr(), 966);
    }

    if TRACE_sched as i64 != 0 {
        traceTaskDelete_(task);
    }
}

static mut TRACE_sched: u8 = 0;

static mut TRACE_gc: u8 = 0;

static mut TRACE_nonmoving_gc: u8 = 0;

static mut TRACE_spark_sampled: u8 = 0;

static mut TRACE_spark_full: u8 = 0;

static mut TRACE_user: u8 = 0;

static mut TRACE_cap: u8 = 0;

static mut trace_utx: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn updateTraceFlagCache() {
    TRACE_sched = (RtsFlags.TraceFlags.scheduler as i32 != 0
        || RtsFlags.DebugFlags.scheduler as i32 != 0) as i32 as u8;
    TRACE_gc = (RtsFlags.TraceFlags.gc as i32 != 0
        || RtsFlags.DebugFlags.gc as i32 != 0
        || RtsFlags.DebugFlags.scheduler as i32 != 0) as i32 as u8;
    TRACE_nonmoving_gc = RtsFlags.TraceFlags.nonmoving_gc as u8;
    TRACE_spark_sampled = RtsFlags.TraceFlags.sparks_sampled as u8;
    TRACE_spark_full = (RtsFlags.TraceFlags.sparks_full as i32 != 0
        || RtsFlags.DebugFlags.sparks as i32 != 0) as i32 as u8;
    TRACE_user = RtsFlags.TraceFlags.user as u8;
    TRACE_cap = (TRACE_sched as i32 != 0
        || TRACE_gc as i32 != 0
        || TRACE_spark_sampled as i32 != 0
        || TRACE_spark_full as i32 != 0
        || TRACE_user as i32 != 0) as i32 as u8;
}

unsafe fn initTracing() {
    initMutex(&raw mut trace_utx);
    updateTraceFlagCache();

    if TRACE_gc as i32 != 0 && RtsFlags.GcFlags.giveStats == NO_GC_STATS as u32 {
        RtsFlags.GcFlags.giveStats = COLLECT_GC_STATS as u32;
    }

    initEventLogging();

    if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG && RtsFlags.TraceFlags.nullWriter as i32 != 0 {
        startEventLogging(&raw const NullEventLogWriter);
    } else if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG && !rtsConfig.eventlog_writer.is_null()
    {
        startEventLogging(rtsConfig.eventlog_writer);
    }
}

unsafe fn endTracing() {
    if eventlog_enabled {
        endEventLogging();
    }
}

unsafe fn freeTracing() {
    if eventlog_enabled {
        freeEventLogging();
    }
}

unsafe fn resetTracing() {
    restartEventLogging();
}

unsafe fn flushTrace() {
    if eventlog_enabled {
        flushEventLog(null_mut::<*mut Capability>());
    }
}

unsafe fn tracingAddCapabilities(mut from: u32, mut to: u32) {
    if eventlog_enabled {
        moreCapEventBufs(from, to);
    }
}

unsafe fn tracePreface() {
    debugBelch(c"%12lx: ".as_ptr(), osThreadId() as u64);

    if RtsFlags.TraceFlags.timestamp {
        debugBelch(c"%9llu: ".as_ptr(), stat_getElapsedTime());
    }
}

static mut thread_stop_reasons: [*mut c_char; 21] = [
    null_mut::<c_char>(),
    c"heap overflow".as_ptr(),
    c"stack overflow".as_ptr(),
    c"yielding".as_ptr(),
    c"blocked".as_ptr(),
    c"finished".as_ptr(),
    c"suspended while making a foreign call".as_ptr(),
    c"blocked on an MVar".as_ptr(),
    c"blocked on a black hole".as_ptr(),
    c"blocked on a read operation".as_ptr(),
    c"blocked on a write operation".as_ptr(),
    c"blocked on a delay operation".as_ptr(),
    c"blocked on STM".as_ptr(),
    c"blocked on asyncDoProc".as_ptr(),
    null_mut::<c_char>(),
    null_mut::<c_char>(),
    c"blocked on a foreign call".as_ptr(),
    c"blocked on a foreign call (interruptible)".as_ptr(),
    c"blocked on throwTo".as_ptr(),
    c"migrating".as_ptr(),
    c"blocked on an atomic MVar read".as_ptr(),
];

unsafe fn traceSchedEvent_stderr(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut tso: *mut StgTSO,
    mut info1: StgWord,
    mut info2: StgWord,
) {
    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            200,
            __r,
        );
    }

    tracePreface();

    let mut threadLabelLen = 0;
    let mut threadLabel = c"".as_ptr();

    if !(*tso).label.is_null() {
        threadLabelLen = (*(*tso).label).bytes as i32;
        threadLabel = &raw mut (*(*tso).label).payload as *mut StgWord as *mut c_char;
    }

    match tag as i32 {
        EVENT_CREATE_THREAD => {
            debugBelch(
                c"cap %d: created thread %llu[\"%.*s\"]\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
            );
        }
        EVENT_RUN_THREAD => {
            debugBelch(
                c"cap %d: running thread %llu[\"%.*s\"] (%s)\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                *(&raw mut what_next_strs as *mut *const c_char).offset((*tso).what_next as isize),
            );
        }
        EVENT_THREAD_RUNNABLE => {
            debugBelch(
                c"cap %d: thread %llu[\"%.*s\"] appended to run queue\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
            );
        }
        EVENT_MIGRATE_THREAD => {
            debugBelch(
                c"cap %d: thread %llu[\"%.*s\"] migrating to cap %d\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                info1 as i32,
            );
        }
        EVENT_THREAD_WAKEUP => {
            debugBelch(
                c"cap %d: waking up thread %llu[\"%.*s\"] on cap %d\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                info1 as i32,
            );
        }
        EVENT_STOP_THREAD => {
            if info1 == (6 + BlockedOnBlackHole) as StgWord {
                debugBelch(
                    c"cap %d: thread %llu[\"%.*s\"] stopped (blocked on black hole owned by thread %lu)\n"
                        .as_ptr(),
                    (*cap).no,
                    (*tso).id,
                    threadLabelLen,
                    threadLabel,
                    info2 as i64,
                );
            } else if info1 == StackOverflow as StgWord {
                debugBelch(
                    c"cap %d: thread %llu[\"%.*s\"] stopped (stack overflow, size %lu)\n".as_ptr(),
                    (*cap).no,
                    (*tso).id,
                    threadLabelLen,
                    threadLabel,
                    info2 as i64,
                );
            } else {
                debugBelch(
                    c"cap %d: thread %llu[\"%.*s\"] stopped (%s)\n".as_ptr(),
                    (*cap).no,
                    (*tso).id,
                    threadLabelLen,
                    threadLabel,
                    thread_stop_reasons[info1 as usize],
                );
            }
        }
        _ => {
            debugBelch(
                c"cap %d: thread %llu[\"%.*s\"]: event %d\n\n".as_ptr(),
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                tag as i32,
            );
        }
    }

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            252,
        );
    }
}

unsafe fn traceSchedEvent_(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut tso: *mut StgTSO,
    mut info1: StgWord,
    mut info2: StgWord,
) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceSchedEvent_stderr(cap, tag, tso, info1, info2);
    } else {
        postSchedEvent(
            cap,
            tag,
            if !tso.is_null() { (*tso).id } else { 0 },
            info1,
            info2,
        );
    };
}

unsafe fn traceGcEvent_stderr(mut cap: *mut Capability, mut tag: EventTypeNum) {
    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            272,
            __r,
        );
    }

    tracePreface();

    match tag as i32 {
        EVENT_REQUEST_SEQ_GC => {
            debugBelch(c"cap %d: requesting sequential GC\n".as_ptr(), (*cap).no);
        }
        EVENT_REQUEST_PAR_GC => {
            debugBelch(c"cap %d: requesting parallel GC\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_START => {
            debugBelch(c"cap %d: starting GC\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_END => {
            debugBelch(c"cap %d: finished GC\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_IDLE => {
            debugBelch(c"cap %d: GC idle\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_WORK => {
            debugBelch(c"cap %d: GC working\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_DONE => {
            debugBelch(c"cap %d: GC done\n".as_ptr(), (*cap).no);
        }
        EVENT_GC_GLOBAL_SYNC => {
            debugBelch(c"cap %d: all caps stopped for GC\n".as_ptr(), (*cap).no);
        }
        _ => {
            barf(c"traceGcEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    }

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            305,
        );
    }
}

unsafe fn traceGcEvent_(mut cap: *mut Capability, mut tag: EventTypeNum) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceGcEvent_stderr(cap, tag);
    } else {
        postEvent(cap, tag);
    };
}

unsafe fn traceGcEventAtT_(mut cap: *mut Capability, mut ts: StgWord64, mut tag: EventTypeNum) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceGcEvent_stderr(cap, tag);
    } else {
        postEventAtTimestamp(cap, ts as EventTimestamp, tag);
    };
}

unsafe fn traceHeapEvent_(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut heap_capset: CapsetID,
    mut info1: W_,
) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        postHeapEvent(cap, tag, heap_capset as EventCapsetID, info1);
    }
}

unsafe fn traceEventHeapInfo_(
    mut heap_capset: CapsetID,
    mut gens: u32,
    mut maxHeapSize: W_,
    mut allocAreaSize: W_,
    mut mblockSize: W_,
    mut blockSize: W_,
) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        postEventHeapInfo(
            heap_capset as EventCapsetID,
            gens,
            maxHeapSize,
            allocAreaSize,
            mblockSize,
            blockSize,
        );
    }
}

unsafe fn traceEventGcStats_(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut r#gen: u32,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: u32,
    mut par_max_copied: W_,
    mut par_tot_copied: W_,
    mut par_balanced_copied: W_,
) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        postEventGcStats(
            cap,
            heap_capset as EventCapsetID,
            r#gen,
            copied,
            slop,
            fragmentation,
            par_n_threads,
            par_max_copied,
            par_tot_copied,
            par_balanced_copied,
        );
    }
}

unsafe fn traceEventMemReturn_(
    mut cap: *mut Capability,
    mut current_mblocks: u32,
    mut needed_mblocks: u32,
    mut returned_mblocks: u32,
) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceCap_stderr(
            cap,
            c"Memory Return (Current: %u) (Needed: %u) (Returned: %u)".as_ptr(),
            current_mblocks,
            needed_mblocks,
            returned_mblocks,
        );
    } else {
        postEventMemReturn(
            cap,
            CAPSET_HEAP_DEFAULT,
            current_mblocks,
            needed_mblocks,
            returned_mblocks,
        );
    };
}

unsafe fn traceCapEvent_(mut cap: *mut Capability, mut tag: EventTypeNum) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        let mut __r = pthread_mutex_lock(&raw mut trace_utx);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                415,
                __r,
            );
        }

        tracePreface();

        match tag as i32 {
            EVENT_CAP_CREATE => {
                debugBelch(c"cap %d: initialised\n".as_ptr(), (*cap).no);
            }
            EVENT_CAP_DELETE => {
                debugBelch(c"cap %d: shutting down\n".as_ptr(), (*cap).no);
            }
            EVENT_CAP_ENABLE => {
                debugBelch(c"cap %d: enabling capability\n".as_ptr(), (*cap).no);
            }
            EVENT_CAP_DISABLE => {
                debugBelch(c"cap %d: disabling capability\n".as_ptr(), (*cap).no);
            }
            _ => {}
        }

        if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                432,
            );
        }
    } else if eventlog_enabled {
        postCapEvent(tag, (*cap).no as EventCapNo);
    }
}

unsafe fn traceCapsetEvent_(mut tag: EventTypeNum, mut capset: CapsetID, mut info: StgWord) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_sched as i32 != 0 {
        let mut __r = pthread_mutex_lock(&raw mut trace_utx);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                451,
                __r,
            );
        }

        tracePreface();

        match tag as i32 {
            EVENT_CAPSET_CREATE => {
                debugBelch(
                    c"created capset %u of type %d\n".as_ptr(),
                    capset,
                    info as i32,
                );
            }
            EVENT_CAPSET_DELETE => {
                debugBelch(c"deleted capset %u\n".as_ptr(), capset);
            }
            EVENT_CAPSET_ASSIGN_CAP => {
                debugBelch(c"assigned cap %llu to capset %u\n".as_ptr(), info, capset);
            }
            EVENT_CAPSET_REMOVE_CAP => {
                debugBelch(c"removed cap %llu from capset %u\n".as_ptr(), info, capset);
            }
            _ => {}
        }

        if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                471,
            );
        }
    } else if eventlog_enabled {
        postCapsetEvent(tag, capset as EventCapsetID, info);
    }
}

unsafe fn traceWallClockTime_() {
    if eventlog_enabled {
        postWallClockTime(CAPSET_CLOCKDOMAIN_DEFAULT);
    }
}

unsafe fn traceOSProcessInfo_() {
    if eventlog_enabled {
        postCapsetEvent(
            EVENT_OSPROCESS_PID as EventTypeNum,
            CAPSET_OSPROCESS_DEFAULT,
            getpid() as StgWord,
        );

        postCapsetEvent(
            EVENT_OSPROCESS_PPID as EventTypeNum,
            CAPSET_OSPROCESS_DEFAULT,
            getppid() as StgWord,
        );

        let mut buf: [c_char; 256] = [0; 256];

        snprintf(
            &raw mut buf as *mut c_char,
            size_of::<[c_char; 256]>() as usize,
            c"GHC-%s %s".as_ptr(),
            __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
            c"TODO RtsWay".as_ptr(),
        );

        postCapsetStrEvent(
            EVENT_RTS_IDENTIFIER as EventTypeNum,
            CAPSET_OSPROCESS_DEFAULT,
            &raw mut buf as *mut c_char,
        );

        let mut argc = 0;
        let mut argv = null_mut::<*mut c_char>();
        getFullProgArgv(&raw mut argc, &raw mut argv);

        if argc != 0 {
            postCapsetVecEvent(
                EVENT_PROGRAM_ARGS as EventTypeNum,
                CAPSET_OSPROCESS_DEFAULT,
                argc,
                argv as *mut *mut c_char,
            );
        }
    }
}

unsafe fn traceSparkEvent_stderr(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut info1: StgWord,
) {
    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            528,
            __r,
        );
    }

    tracePreface();

    match tag as i32 {
        EVENT_CREATE_SPARK_THREAD => {
            debugBelch(
                c"cap %d: creating spark thread %lu\n".as_ptr(),
                (*cap).no,
                info1 as i64,
            );
        }
        EVENT_SPARK_CREATE => {
            debugBelch(c"cap %d: added spark to pool\n".as_ptr(), (*cap).no);
        }
        EVENT_SPARK_DUD => {
            debugBelch(c"cap %d: discarded dud spark\n".as_ptr(), (*cap).no);
        }
        EVENT_SPARK_OVERFLOW => {
            debugBelch(c"cap %d: discarded overflowed spark\n".as_ptr(), (*cap).no);
        }
        EVENT_SPARK_RUN => {
            debugBelch(c"cap %d: running a spark\n".as_ptr(), (*cap).no);
        }
        EVENT_SPARK_STEAL => {
            debugBelch(
                c"cap %d: stealing a spark from cap %d\n".as_ptr(),
                (*cap).no,
                info1 as i32,
            );
        }
        EVENT_SPARK_FIZZLE => {
            debugBelch(
                c"cap %d: fizzled spark removed from pool\n".as_ptr(),
                (*cap).no,
            );
        }
        EVENT_SPARK_GC => {
            debugBelch(c"cap %d: GCd spark removed from pool\n".as_ptr(), (*cap).no);
        }
        _ => {
            barf(
                c"traceSparkEvent: unknown event tag %d".as_ptr(),
                tag as i32,
            );
        }
    }

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            570,
        );
    }
}

unsafe fn traceSparkEvent_(mut cap: *mut Capability, mut tag: EventTypeNum, mut info1: StgWord) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceSparkEvent_stderr(cap, tag, info1);
    } else {
        postSparkEvent(cap, tag, info1);
    };
}

unsafe fn traceSparkCounters_(
    mut cap: *mut Capability,
    mut counters: SparkCounters,
    mut remaining: StgWord,
) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        postSparkCountersEvent(cap, counters, remaining);
    }
}

unsafe fn traceTaskCreate_(mut task: *mut Task, mut cap: *mut Capability) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        let mut taskid = serialisableTaskId(task) as EventTaskId;
        let mut tid = kernelThreadId() as EventKernelThreadId;
        postTaskCreateEvent(taskid, (*cap).no as EventCapNo, tid);
    }
}

unsafe fn traceTaskMigrate_(
    mut task: *mut Task,
    mut cap: *mut Capability,
    mut new_cap: *mut Capability,
) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        let mut taskid = serialisableTaskId(task) as EventTaskId;

        postTaskMigrateEvent(taskid, (*cap).no as EventCapNo, (*new_cap).no as EventCapNo);
    }
}

unsafe fn traceTaskDelete_(mut task: *mut Task) {
    if !(RtsFlags.TraceFlags.tracing == TRACE_STDERR) {
        let mut taskid = serialisableTaskId(task) as EventTaskId;
        postTaskDeleteEvent(taskid);
    }
}

unsafe fn traceHeapProfBegin(mut profile_id: StgWord8) {
    if eventlog_enabled {
        postHeapProfBegin(profile_id);
    }
}

unsafe fn traceHeapBioProfSampleBegin(mut era: StgInt, mut time: StgWord64) {
    if eventlog_enabled {
        postHeapBioProfSampleBegin(era, time);
    }
}

unsafe fn traceHeapProfSampleBegin(mut era: StgInt) {
    if eventlog_enabled {
        postHeapProfSampleBegin(era);
    }
}

unsafe fn traceHeapProfSampleEnd(mut era: StgInt) {
    if eventlog_enabled {
        postHeapProfSampleEnd(era);
    }
}

unsafe fn traceHeapProfSampleString(
    mut profile_id: StgWord8,
    mut label: *const c_char,
    mut residency: StgWord,
) {
    if eventlog_enabled {
        postHeapProfSampleString(profile_id, label, residency as StgWord64);
    }
}

unsafe fn traceIPE(mut ipe: *const InfoProvEnt) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        let mut __r = pthread_mutex_lock(&raw mut trace_utx);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                690,
                __r,
            );
        }

        let mut closure_desc_buf: [c_char; 11] = [0; 11];
        formatClosureDescIpe(ipe, &raw mut closure_desc_buf as *mut c_char);
        tracePreface();

        debugBelch(
            c"IPE: table_name %s, closure_desc %s, ty_desc %s, label %s, unit %s, module %s, srcloc %s:%s\n"
                .as_ptr(),
            (*ipe).prov.table_name,
            &raw mut closure_desc_buf as *mut c_char,
            (*ipe).prov.ty_desc,
            (*ipe).prov.label,
            (*ipe).prov.unit_id,
            (*ipe).prov.module,
            (*ipe).prov.src_file,
            (*ipe).prov.src_span,
        );

        if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                701,
            );
        }
    } else if eventlog_enabled {
        postIPE(ipe);
    }
}

unsafe fn traceHeapProfCostCentre(
    mut ccID: StgWord32,
    mut label: *const c_char,
    mut module: *const c_char,
    mut srcloc: *const c_char,
    mut is_caf: StgBool,
) {
    if eventlog_enabled {
        postHeapProfCostCentre(ccID, label, module, srcloc, is_caf);
    }
}

unsafe fn traceHeapProfSampleCostCentre(
    mut profile_id: StgWord8,
    mut stack: *mut CostCentreStack,
    mut residency: StgWord,
) {
    if eventlog_enabled {
        postHeapProfSampleCostCentre(profile_id, stack, residency as StgWord64);
    }
}

unsafe fn traceProfSampleCostCentre(
    mut cap: *mut Capability,
    mut stack: *mut CostCentreStack,
    mut tick: StgWord,
) {
    if eventlog_enabled {
        postProfSampleCostCentre(cap, stack, tick as StgWord64);
    }
}

unsafe fn traceProfBegin() {
    if eventlog_enabled {
        postProfBegin();
    }
}

unsafe fn vtraceCap_stderr(mut cap: *mut Capability, mut msg: *mut c_char, mut ap: VaList) {
    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            749,
            __r,
        );
    }

    tracePreface();
    debugBelch(c"cap %d: ".as_ptr(), (*cap).no);
    vdebugBelch(msg, ap.as_va_list());
    debugBelch(c"\n".as_ptr());

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            756,
        );
    }
}

unsafe fn traceCap_stderr(mut cap: *mut Capability, mut msg: *mut c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();
    vtraceCap_stderr(cap, msg, ap.as_va_list());
}

unsafe fn traceCap_(mut cap: *mut Capability, mut msg: *mut c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        vtraceCap_stderr(cap, msg, ap.as_va_list());
    } else {
        postCapMsg(cap, msg, ap.as_va_list());
    };
}

unsafe fn vtrace_stderr(mut msg: *mut c_char, mut ap: VaList) {
    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            788,
            __r,
        );
    }

    tracePreface();
    vdebugBelch(msg, ap.as_va_list());
    debugBelch(c"\n".as_ptr());

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            794,
        );
    }
}

unsafe fn trace_(mut msg: *mut c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        vtrace_stderr(msg, ap.as_va_list());
    } else {
        postMsg(msg, ap.as_va_list());
    };
}

unsafe fn traceUserMsg(mut cap: *mut Capability, mut msg: *mut c_char) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_user as i32 != 0 {
        traceCap_stderr(cap, c"%s".as_ptr(), msg);
    } else if eventlog_enabled as i32 != 0 && TRACE_user as i32 != 0 {
        postUserEvent(cap, EVENT_USER_MSG as EventTypeNum, msg);
    }
}

unsafe fn traceUserBinaryMsg(mut cap: *mut Capability, mut msg: *mut u8, mut size: usize) {
    if eventlog_enabled as i32 != 0 && TRACE_user as i32 != 0 {
        postUserBinaryEvent(cap, EVENT_USER_BINARY_MSG as EventTypeNum, msg, size);
    }
}

unsafe fn traceUserMarker(mut cap: *mut Capability, mut markername: *mut c_char) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_user as i32 != 0 {
        traceCap_stderr(cap, c"User marker: %s".as_ptr(), markername);
    } else if eventlog_enabled as i32 != 0 && TRACE_user as i32 != 0 {
        postUserEvent(cap, EVENT_USER_MARKER as EventTypeNum, markername);
    }
}

unsafe fn traceThreadLabel_(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut label: *mut c_char,
    mut len: usize,
) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        let mut __r = pthread_mutex_lock(&raw mut trace_utx);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                872,
                __r,
            );
        }

        tracePreface();

        debugBelch(
            c"cap %d: thread %llu has label %.*s\n".as_ptr(),
            (*cap).no,
            (*tso).id,
            len as i32,
            label,
        );

        if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Trace.c".as_ptr(),
                876,
            );
        }
    } else {
        postThreadLabel(cap, (*tso).id as EventThreadID, label, len);
    };
}

unsafe fn traceConcMarkBegin() {
    if eventlog_enabled {
        postEventNoCap(EVENT_CONC_MARK_BEGIN as EventTypeNum);
    }
}

unsafe fn traceConcMarkEnd(mut marked_obj_count: StgWord32) {
    if eventlog_enabled {
        postConcMarkEnd(marked_obj_count);
    }
}

unsafe fn traceConcSyncBegin() {
    if eventlog_enabled {
        postEventNoCap(EVENT_CONC_SYNC_BEGIN as EventTypeNum);
    }
}

unsafe fn traceConcSyncEnd() {
    if eventlog_enabled {
        postEventNoCap(EVENT_CONC_SYNC_END as EventTypeNum);
    }
}

unsafe fn traceConcSweepBegin() {
    if eventlog_enabled {
        postEventNoCap(EVENT_CONC_SWEEP_BEGIN as EventTypeNum);
    }
}

unsafe fn traceConcSweepEnd() {
    if eventlog_enabled {
        postEventNoCap(EVENT_CONC_SWEEP_END as EventTypeNum);
    }
}

unsafe fn traceConcUpdRemSetFlush(mut cap: *mut Capability) {
    if eventlog_enabled {
        postConcUpdRemSetFlush(cap);
    }
}

unsafe fn traceNonmovingHeapCensus(mut blk_size: u16, mut census: *const NonmovingAllocCensus) {
    if eventlog_enabled as i32 != 0 && TRACE_nonmoving_gc as i32 != 0 {
        postNonmovingHeapCensus(blk_size, census);
    }
}

unsafe fn traceNonmovingPrunedSegments(mut pruned_segments: u32, mut free_segments: u32) {
    if eventlog_enabled as i32 != 0 && TRACE_nonmoving_gc as i32 != 0 {
        postNonmovingPrunedSegments(pruned_segments, free_segments);
    }
}

unsafe fn traceThreadStatus_(mut tso: *mut StgTSO) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        printThreadStatus(tso);
    }
}

unsafe fn traceBegin(mut str: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    let mut __r = pthread_mutex_lock(&raw mut trace_utx);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            957,
            __r,
        );
    }

    tracePreface();
    vdebugBelch(str, ap.as_va_list());
}

unsafe fn traceEnd() {
    debugBelch(c"\n".as_ptr());

    if pthread_mutex_unlock(&raw mut trace_utx) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Trace.c".as_ptr(),
            967,
        );
    }
}
