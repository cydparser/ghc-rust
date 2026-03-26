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
    postHeapEvent, postHeapProfBegin, postHeapProfSampleBegin, postHeapProfSampleEnd,
    postHeapProfSampleString, postIPE, postMsg, postNonmovingHeapCensus,
    postNonmovingPrunedSegments, postSchedEvent, postSparkCountersEvent, postSparkEvent,
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
use crate::ffi::rts::os_threads::kernelThreadId;
use crate::ffi::rts::storage::tso::{StgThreadID, StgThreadReturnCode};
use crate::ffi::rts::storage::tso::{StgThreadID, StgThreadReturnCode};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::{Capability, getFullProgArgv};
use crate::ffi::stg::W_;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgInt, StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::ffi::stg::types::{StgWord, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;
use crate::printer::what_next_strs;
use crate::rts_flags::rtsConfig;
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::sparks::SparkCounters;
use crate::stats::stat_getElapsedTime;
use crate::task::Task;
use crate::task::{Task, serialisableTaskId};
use crate::threads::printThreadStatus;
use crate::trace::{
    CAPSET_CLOCKDOMAIN_DEFAULT, CAPSET_HEAP_DEFAULT, CAPSET_OSPROCESS_DEFAULT, CapsetID, CapsetType,
};

pub(crate) type CapsetID = StgWord32;

pub(crate) type CapsetType = StgWord16;

pub(crate) const CAPSET_OSPROCESS_DEFAULT: CapsetID = 0 as c_int as CapsetID;

pub(crate) const CAPSET_HEAP_DEFAULT: CapsetID = 0 as c_int as CapsetID;

pub(crate) const CAPSET_CLOCKDOMAIN_DEFAULT: CapsetID = 1 as c_int as CapsetID;

#[inline]
pub(crate) unsafe fn traceEventCreateThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if TRACE_sched as c_long != 0 {
        traceSchedEvent_(
            cap,
            0 as EventTypeNum,
            tso,
            (*(*tso).stackobj).stack_size as StgWord,
            0 as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventRunThread(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if TRACE_sched as c_long != 0 {
        traceSchedEvent_(
            cap,
            1 as EventTypeNum,
            tso,
            (*tso).what_next as StgWord,
            0 as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventStopThread(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut status: StgThreadReturnCode,
    mut info: StgWord32,
) {
    if TRACE_sched as c_long != 0 {
        traceSchedEvent_(
            cap,
            2 as EventTypeNum,
            tso,
            status as StgWord,
            info as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceEventMigrateThread(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut new_cap: uint32_t,
) {
    if TRACE_sched as c_long != 0 {
        traceSchedEvent_(
            cap,
            4 as EventTypeNum,
            tso,
            new_cap as StgWord,
            0 as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceCapCreate(mut cap: *mut Capability) {
    if TRACE_cap as c_long != 0 {
        traceCapEvent_(cap, 45 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceCapDelete(mut cap: *mut Capability) {
    if TRACE_cap as c_long != 0 {
        traceCapEvent_(cap, 46 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceCapEnable(mut cap: *mut Capability) {
    if TRACE_cap as c_long != 0 {
        traceCapEvent_(cap, 48 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceCapDisable(mut cap: *mut Capability) {
    if TRACE_cap as c_long != 0 {
        traceCapEvent_(cap, 47 as EventTypeNum);
    }

    if eventlog_enabled {
        flushLocalEventsBuf(cap);
    }
}

#[inline]
pub(crate) unsafe fn traceEventThreadWakeup(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut other_cap: uint32_t,
) {
    if TRACE_sched as c_long != 0 {
        traceSchedEvent_(
            cap,
            8 as EventTypeNum,
            tso,
            other_cap as StgWord,
            0 as StgWord,
        );
    }
}

#[inline]
pub(crate) unsafe fn traceThreadLabel(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut label: *mut c_char,
    mut len: size_t,
) {
    if TRACE_sched as c_long != 0 {
        traceThreadLabel_(cap, tso, label, len);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStart(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 9 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStartAtT(mut cap: *mut Capability, mut ts: StgWord64) {
    if TRACE_gc as c_long != 0 {
        traceGcEventAtT_(cap, ts, 9 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcEnd(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 10 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcEndAtT(mut cap: *mut Capability, mut ts: StgWord64) {
    if TRACE_gc as c_long != 0 {
        traceGcEventAtT_(cap, ts, 10 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventRequestSeqGc(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 11 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventRequestParGc(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 12 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcIdle(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 20 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcWork(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 21 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcDone(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 22 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcGlobalSync(mut cap: *mut Capability) {
    if TRACE_gc as c_long != 0 {
        traceGcEvent_(cap, 54 as EventTypeNum);
    }
}

#[inline]
pub(crate) unsafe fn traceEventGcStats(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut r#gen: uint32_t,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: uint32_t,
    mut par_max_copied: W_,
    mut par_tot_copied: W_,
    mut par_balanced_copied: W_,
) {
    if TRACE_gc as c_long != 0 {
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
    mut current_mblocks: uint32_t,
    mut needed_mblocks: uint32_t,
    mut returned_mblocks: uint32_t,
) {
    if TRACE_gc as c_long != 0 {
        traceEventMemReturn_(cap, current_mblocks, needed_mblocks, returned_mblocks);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapInfo(
    mut heap_capset: CapsetID,
    mut gens: uint32_t,
    mut maxHeapSize: W_,
    mut allocAreaSize: W_,
    mut mblockSize: W_,
    mut blockSize: W_,
) {
    if TRACE_gc as c_long != 0 {
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
    if TRACE_gc as c_long != 0 {
        traceHeapEvent_(cap, 49 as EventTypeNum, heap_capset, allocated);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapSize(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_size: W_,
) {
    if TRACE_gc as c_long != 0 {
        traceHeapEvent_(cap, 50 as EventTypeNum, heap_capset, heap_size);
    }
}

#[inline]
pub(crate) unsafe fn traceEventBlocksSize(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_size: W_,
) {
    if TRACE_gc as c_long != 0 {
        traceHeapEvent_(cap, 91 as EventTypeNum, heap_capset, heap_size);
    }
}

#[inline]
pub(crate) unsafe fn traceEventHeapLive(
    mut cap: *mut Capability,
    mut heap_capset: CapsetID,
    mut heap_live: W_,
) {
    if TRACE_gc as c_long != 0 {
        traceHeapEvent_(cap, 51 as EventTypeNum, heap_capset, heap_live);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetCreate(mut capset: CapsetID, mut capset_type: CapsetType) {
    if TRACE_cap as c_long != 0 {
        traceCapsetEvent_(25 as EventTypeNum, capset, capset_type as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetDelete(mut capset: CapsetID) {
    if TRACE_cap as c_long != 0 {
        traceCapsetEvent_(26 as EventTypeNum, capset, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetAssignCap(mut capset: CapsetID, mut capno: uint32_t) {
    if TRACE_cap as c_long != 0 {
        traceCapsetEvent_(27 as EventTypeNum, capset, capno as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceCapsetRemoveCap(mut capset: CapsetID, mut capno: uint32_t) {
    if TRACE_cap as c_long != 0 {
        traceCapsetEvent_(28 as EventTypeNum, capset, capno as StgWord);
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
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 15 as EventTypeNum, spark_tid as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceSparkCounters(mut cap: *mut Capability) {}

#[inline]
pub(crate) unsafe fn traceEventSparkCreate(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 35 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkDud(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 36 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkOverflow(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 37 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkRun(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 38 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkSteal(mut cap: *mut Capability, mut victim_cap: uint32_t) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 39 as EventTypeNum, victim_cap as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkFizzle(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 40 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceEventSparkGC(mut cap: *mut Capability) {
    if TRACE_spark_full as c_long != 0 {
        traceSparkEvent_(cap, 41 as EventTypeNum, 0 as StgWord);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskCreate(mut task: *mut Task, mut cap: *mut Capability) {
    if ((*task).cap == cap) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            933 as c_uint,
        );
    }

    if !cap.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            937 as c_uint,
        );
    }

    if TRACE_sched as c_long != 0 {
        traceTaskCreate_(task, cap);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskMigrate(
    mut task: *mut Task,
    mut cap: *mut Capability,
    mut new_cap: *mut Capability,
) {
    if ((*task).cap == cap) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            952 as c_uint,
        );
    }

    if !cap.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            953 as c_uint,
        );
    }

    if (cap != new_cap) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            954 as c_uint,
        );
    }

    if !new_cap.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            955 as c_uint,
        );
    }

    if TRACE_sched as c_long != 0 {
        traceTaskMigrate_(task, cap, new_cap);
    }
}

#[inline]
pub(crate) unsafe fn traceTaskDelete(mut task: *mut Task) {
    if !(*task).cap.is_null() as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Trace.h\0" as *const u8 as *const c_char,
            966 as c_uint,
        );
    }

    if TRACE_sched as c_long != 0 {
        traceTaskDelete_(task);
    }
}

static mut TRACE_sched: uint8_t = 0;

static mut TRACE_gc: uint8_t = 0;

static mut TRACE_nonmoving_gc: uint8_t = 0;

static mut TRACE_spark_sampled: uint8_t = 0;

static mut TRACE_spark_full: uint8_t = 0;

static mut TRACE_user: uint8_t = 0;

static mut TRACE_cap: uint8_t = 0;

unsafe fn updateTraceFlagCache() {
    TRACE_sched = (RtsFlags.TraceFlags.scheduler as c_int != 0
        || RtsFlags.DebugFlags.scheduler as c_int != 0) as c_int as uint8_t;
    TRACE_gc = (RtsFlags.TraceFlags.gc as c_int != 0
        || RtsFlags.DebugFlags.gc as c_int != 0
        || RtsFlags.DebugFlags.scheduler as c_int != 0) as c_int as uint8_t;
    TRACE_nonmoving_gc = RtsFlags.TraceFlags.nonmoving_gc as uint8_t;
    TRACE_spark_sampled = RtsFlags.TraceFlags.sparks_sampled as uint8_t;
    TRACE_spark_full = (RtsFlags.TraceFlags.sparks_full as c_int != 0
        || RtsFlags.DebugFlags.sparks as c_int != 0) as c_int as uint8_t;
    TRACE_user = RtsFlags.TraceFlags.user as uint8_t;
    TRACE_cap = (TRACE_sched as c_int != 0
        || TRACE_gc as c_int != 0
        || TRACE_spark_sampled as c_int != 0
        || TRACE_spark_full as c_int != 0
        || TRACE_user as c_int != 0) as c_int as uint8_t;
}

unsafe fn initTracing() {
    updateTraceFlagCache();

    if TRACE_gc as c_int != 0 && RtsFlags.GcFlags.giveStats == NO_GC_STATS as uint32_t {
        RtsFlags.GcFlags.giveStats = COLLECT_GC_STATS as uint32_t;
    }

    initEventLogging();

    if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG && RtsFlags.TraceFlags.nullWriter as c_int != 0
    {
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

unsafe fn tracingAddCapabilities(mut from: uint32_t, mut to: uint32_t) {
    if eventlog_enabled {
        moreCapEventBufs(from, to);
    }
}

unsafe fn tracePreface() {
    if RtsFlags.TraceFlags.timestamp {
        debugBelch(
            b"%9llu: \0" as *const u8 as *const c_char,
            stat_getElapsedTime(),
        );
    }
}

static mut thread_stop_reasons: [*mut c_char; 21] = [
    null::<c_char>() as *mut c_char,
    b"heap overflow\0" as *const u8 as *const c_char as *mut c_char,
    b"stack overflow\0" as *const u8 as *const c_char as *mut c_char,
    b"yielding\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked\0" as *const u8 as *const c_char as *mut c_char,
    b"finished\0" as *const u8 as *const c_char as *mut c_char,
    b"suspended while making a foreign call\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on an MVar\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on a black hole\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on a read operation\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on a write operation\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on a delay operation\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on STM\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on asyncDoProc\0" as *const u8 as *const c_char as *mut c_char,
    null::<c_char>() as *mut c_char,
    null::<c_char>() as *mut c_char,
    b"blocked on a foreign call\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on a foreign call (interruptible)\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on throwTo\0" as *const u8 as *const c_char as *mut c_char,
    b"migrating\0" as *const u8 as *const c_char as *mut c_char,
    b"blocked on an atomic MVar read\0" as *const u8 as *const c_char as *mut c_char,
];

unsafe fn traceSchedEvent_stderr(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut tso: *mut StgTSO,
    mut info1: StgWord,
    mut info2: StgWord,
) {
    tracePreface();

    let mut threadLabelLen = 0 as c_int;
    let mut threadLabel = b"\0" as *const u8 as *const c_char as *mut c_char;

    if !(*tso).label.is_null() {
        threadLabelLen = (*(*tso).label).bytes as c_int;
        threadLabel = &raw mut (*(*tso).label).payload as *mut StgWord as *mut c_char;
    }

    match tag as c_int {
        EVENT_CREATE_THREAD => {
            debugBelch(
                b"cap %d: created thread %llu[\"%.*s\"]\n\0" as *const u8 as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
            );
        }
        EVENT_RUN_THREAD => {
            debugBelch(
                b"cap %d: running thread %llu[\"%.*s\"] (%s)\n\0" as *const u8 as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                *(&raw mut what_next_strs as *mut *const c_char).offset((*tso).what_next as isize),
            );
        }
        EVENT_THREAD_RUNNABLE => {
            debugBelch(
                b"cap %d: thread %llu[\"%.*s\"] appended to run queue\n\0" as *const u8
                    as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
            );
        }
        EVENT_MIGRATE_THREAD => {
            debugBelch(
                b"cap %d: thread %llu[\"%.*s\"] migrating to cap %d\n\0" as *const u8
                    as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                info1 as c_int,
            );
        }
        EVENT_THREAD_WAKEUP => {
            debugBelch(
                b"cap %d: waking up thread %llu[\"%.*s\"] on cap %d\n\0" as *const u8
                    as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                info1 as c_int,
            );
        }
        EVENT_STOP_THREAD => {
            if info1 == (6 as c_int + BlockedOnBlackHole) as StgWord {
                debugBelch(
                    b"cap %d: thread %llu[\"%.*s\"] stopped (blocked on black hole owned by thread %lu)\n\0"
                        as *const u8 as *const c_char,
                    (*cap).no,
                    (*tso).id,
                    threadLabelLen,
                    threadLabel,
                    info2 as c_long,
                );
            } else if info1 == StackOverflow as StgWord {
                debugBelch(
                    b"cap %d: thread %llu[\"%.*s\"] stopped (stack overflow, size %lu)\n\0"
                        as *const u8 as *const c_char,
                    (*cap).no,
                    (*tso).id,
                    threadLabelLen,
                    threadLabel,
                    info2 as c_long,
                );
            } else {
                debugBelch(
                    b"cap %d: thread %llu[\"%.*s\"] stopped (%s)\n\0" as *const u8 as *const c_char,
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
                b"cap %d: thread %llu[\"%.*s\"]: event %d\n\n\0" as *const u8 as *const c_char,
                (*cap).no,
                (*tso).id,
                threadLabelLen,
                threadLabel,
                tag as c_int,
            );
        }
    };
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
            if !tso.is_null() {
                (*tso).id
            } else {
                0 as StgThreadID
            },
            info1,
            info2,
        );
    };
}

unsafe fn traceGcEvent_stderr(mut cap: *mut Capability, mut tag: EventTypeNum) {
    tracePreface();

    match tag as c_int {
        EVENT_REQUEST_SEQ_GC => {
            debugBelch(
                b"cap %d: requesting sequential GC\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_REQUEST_PAR_GC => {
            debugBelch(
                b"cap %d: requesting parallel GC\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_START => {
            debugBelch(
                b"cap %d: starting GC\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_END => {
            debugBelch(
                b"cap %d: finished GC\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_IDLE => {
            debugBelch(
                b"cap %d: GC idle\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_WORK => {
            debugBelch(
                b"cap %d: GC working\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_DONE => {
            debugBelch(
                b"cap %d: GC done\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_GC_GLOBAL_SYNC => {
            debugBelch(
                b"cap %d: all caps stopped for GC\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        _ => {
            barf(
                b"traceGcEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
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
    mut gens: uint32_t,
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
    mut r#gen: uint32_t,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: uint32_t,
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
    mut current_mblocks: uint32_t,
    mut needed_mblocks: uint32_t,
    mut returned_mblocks: uint32_t,
) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        traceCap_stderr(
            cap,
            b"Memory Return (Current: %u) (Needed: %u) (Returned: %u)\0" as *const u8
                as *const c_char as *mut c_char,
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
        tracePreface();

        match tag as c_int {
            EVENT_CAP_CREATE => {
                debugBelch(
                    b"cap %d: initialised\n\0" as *const u8 as *const c_char,
                    (*cap).no,
                );
            }
            EVENT_CAP_DELETE => {
                debugBelch(
                    b"cap %d: shutting down\n\0" as *const u8 as *const c_char,
                    (*cap).no,
                );
            }
            EVENT_CAP_ENABLE => {
                debugBelch(
                    b"cap %d: enabling capability\n\0" as *const u8 as *const c_char,
                    (*cap).no,
                );
            }
            EVENT_CAP_DISABLE => {
                debugBelch(
                    b"cap %d: disabling capability\n\0" as *const u8 as *const c_char,
                    (*cap).no,
                );
            }
            _ => {}
        }
    } else if eventlog_enabled {
        postCapEvent(tag, (*cap).no as EventCapNo);
    }
}

unsafe fn traceCapsetEvent_(mut tag: EventTypeNum, mut capset: CapsetID, mut info: StgWord) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_sched as c_int != 0 {
        tracePreface();

        match tag as c_int {
            EVENT_CAPSET_CREATE => {
                debugBelch(
                    b"created capset %u of type %d\n\0" as *const u8 as *const c_char,
                    capset,
                    info as c_int,
                );
            }
            EVENT_CAPSET_DELETE => {
                debugBelch(
                    b"deleted capset %u\n\0" as *const u8 as *const c_char,
                    capset,
                );
            }
            EVENT_CAPSET_ASSIGN_CAP => {
                debugBelch(
                    b"assigned cap %llu to capset %u\n\0" as *const u8 as *const c_char,
                    info,
                    capset,
                );
            }
            EVENT_CAPSET_REMOVE_CAP => {
                debugBelch(
                    b"removed cap %llu from capset %u\n\0" as *const u8 as *const c_char,
                    info,
                    capset,
                );
            }
            _ => {}
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
            size_of::<[c_char; 256]>() as size_t,
            b"GHC-%s %s\0" as *const u8 as *const c_char,
            __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
            b"TODO RtsWay\0" as *const u8 as *const c_char,
        );

        postCapsetStrEvent(
            EVENT_RTS_IDENTIFIER as EventTypeNum,
            CAPSET_OSPROCESS_DEFAULT,
            &raw mut buf as *mut c_char,
        );

        let mut argc = 0 as c_int;
        let mut argv = null_mut::<*mut c_char>();
        getFullProgArgv(&raw mut argc, &raw mut argv);

        if argc != 0 as c_int {
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
    tracePreface();

    match tag as c_int {
        EVENT_CREATE_SPARK_THREAD => {
            debugBelch(
                b"cap %d: creating spark thread %lu\n\0" as *const u8 as *const c_char,
                (*cap).no,
                info1 as c_long,
            );
        }
        EVENT_SPARK_CREATE => {
            debugBelch(
                b"cap %d: added spark to pool\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_SPARK_DUD => {
            debugBelch(
                b"cap %d: discarded dud spark\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_SPARK_OVERFLOW => {
            debugBelch(
                b"cap %d: discarded overflowed spark\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_SPARK_RUN => {
            debugBelch(
                b"cap %d: running a spark\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_SPARK_STEAL => {
            debugBelch(
                b"cap %d: stealing a spark from cap %d\n\0" as *const u8 as *const c_char,
                (*cap).no,
                info1 as c_int,
            );
        }
        EVENT_SPARK_FIZZLE => {
            debugBelch(
                b"cap %d: fizzled spark removed from pool\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        EVENT_SPARK_GC => {
            debugBelch(
                b"cap %d: GCd spark removed from pool\n\0" as *const u8 as *const c_char,
                (*cap).no,
            );
        }
        _ => {
            barf(
                b"traceSparkEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
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
        let mut closure_desc_buf: [c_char; 11] = [0; 11];
        formatClosureDescIpe(ipe, &raw mut closure_desc_buf as *mut c_char);
        tracePreface();

        debugBelch(
            b"IPE: table_name %s, closure_desc %s, ty_desc %s, label %s, unit %s, module %s, srcloc %s:%s\n\0"
                as *const u8 as *const c_char,
            (*ipe).prov.table_name,
            &raw mut closure_desc_buf as *mut c_char,
            (*ipe).prov.ty_desc,
            (*ipe).prov.label,
            (*ipe).prov.unit_id,
            (*ipe).prov.module,
            (*ipe).prov.src_file,
            (*ipe).prov.src_span,
        );
    } else if eventlog_enabled {
        postIPE(ipe);
    }
}

unsafe fn vtraceCap_stderr(mut cap: *mut Capability, mut msg: *mut c_char, mut ap: VaList) {
    tracePreface();
    debugBelch(b"cap %d: \0" as *const u8 as *const c_char, (*cap).no);
    vdebugBelch(msg, ap.as_va_list());
    debugBelch(b"\n\0" as *const u8 as *const c_char);
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
    tracePreface();
    vdebugBelch(msg, ap.as_va_list());
    debugBelch(b"\n\0" as *const u8 as *const c_char);
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
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_user as c_int != 0 {
        traceCap_stderr(
            cap,
            b"%s\0" as *const u8 as *const c_char as *mut c_char,
            msg,
        );
    } else if eventlog_enabled as c_int != 0 && TRACE_user as c_int != 0 {
        postUserEvent(cap, EVENT_USER_MSG as EventTypeNum, msg);
    }
}

unsafe fn traceUserBinaryMsg(mut cap: *mut Capability, mut msg: *mut uint8_t, mut size: size_t) {
    if eventlog_enabled as c_int != 0 && TRACE_user as c_int != 0 {
        postUserBinaryEvent(cap, EVENT_USER_BINARY_MSG as EventTypeNum, msg, size);
    }
}

unsafe fn traceUserMarker(mut cap: *mut Capability, mut markername: *mut c_char) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR && TRACE_user as c_int != 0 {
        traceCap_stderr(
            cap,
            b"User marker: %s\0" as *const u8 as *const c_char as *mut c_char,
            markername,
        );
    } else if eventlog_enabled as c_int != 0 && TRACE_user as c_int != 0 {
        postUserEvent(cap, EVENT_USER_MARKER as EventTypeNum, markername);
    }
}

unsafe fn traceThreadLabel_(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut label: *mut c_char,
    mut len: size_t,
) {
    if RtsFlags.TraceFlags.tracing == TRACE_STDERR {
        tracePreface();

        debugBelch(
            b"cap %d: thread %llu has label %.*s\n\0" as *const u8 as *const c_char,
            (*cap).no,
            (*tso).id,
            len as c_int,
            label,
        );
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

unsafe fn traceNonmovingHeapCensus(
    mut blk_size: uint16_t,
    mut census: *const NonmovingAllocCensus,
) {
    if eventlog_enabled as c_int != 0 && TRACE_nonmoving_gc as c_int != 0 {
        postNonmovingHeapCensus(blk_size, census);
    }
}

unsafe fn traceNonmovingPrunedSegments(mut pruned_segments: uint32_t, mut free_segments: uint32_t) {
    if eventlog_enabled as c_int != 0 && TRACE_nonmoving_gc as c_int != 0 {
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
    tracePreface();
    vdebugBelch(str, ap.as_va_list());
}

unsafe fn traceEnd() {
    debugBelch(b"\n\0" as *const u8 as *const c_char);
}
