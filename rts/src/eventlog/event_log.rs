use crate::capability::getCapability;
use crate::event_log_constants::{
    EVENT_BLOCK_MARKER, EVENT_CONC_MARK_END, EVENT_CONC_UPD_REM_SET_FLUSH, EVENT_GC_STATS_GHC,
    EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN, EVENT_HEAP_INFO_GHC, EVENT_HEAP_PROF_BEGIN,
    EVENT_HEAP_PROF_SAMPLE_BEGIN, EVENT_HEAP_PROF_SAMPLE_END, EVENT_HEAP_PROF_SAMPLE_STRING,
    EVENT_IPE, EVENT_LOG_MSG, EVENT_MEM_RETURN, EVENT_NONMOVING_HEAP_CENSUS,
    EVENT_NONMOVING_PRUNED_SEGMENTS, EVENT_SPARK_COUNTERS, EVENT_TASK_CREATE, EVENT_TASK_DELETE,
    EVENT_TASK_MIGRATE, EVENT_THREAD_LABEL, EVENT_WALL_CLOCK_TIME,
};
use crate::event_types::eventTypes;
use crate::eventlog::event_log::{EventlogInitPost, eventlog_init_func, eventlog_init_func_t};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::event_log_format::{
    EVENT_DATA_BEGIN, EVENT_DATA_END, EVENT_ET_BEGIN, EVENT_ET_END, EVENT_HEADER_BEGIN,
    EVENT_HEADER_END, EVENT_HET_BEGIN, EVENT_HET_END, EVENT_PAYLOAD_SIZE_MAX, EventCapNo,
    EventCapsetID, EventCapsetType, EventKernelThreadId, EventPayloadSize, EventTaskId,
    EventThreadID, EventTimestamp, EventTypeNum, HEAP_PROF_BREAKDOWN_BIOGRAPHY,
    HEAP_PROF_BREAKDOWN_CLOSURE_DESCR, HEAP_PROF_BREAKDOWN_CLOSURE_TYPE,
    HEAP_PROF_BREAKDOWN_COST_CENTRE, HEAP_PROF_BREAKDOWN_ERA, HEAP_PROF_BREAKDOWN_INFO_TABLE,
    HEAP_PROF_BREAKDOWN_MODULE, HEAP_PROF_BREAKDOWN_RETAINER, HEAP_PROF_BREAKDOWN_TYPE_DESCR,
    HeapProfBreakdown, NUM_GHC_EVENT_TAGS,
};
use crate::ffi::rts::event_log_writer::{
    EVENTLOG_NOT_CONFIGURED, EVENTLOG_RUNNING, EventLogStatus, EventLogWriter,
};
use crate::ffi::rts::flags::{PROFILING_FLAGS, RtsFlags};
use crate::ffi::rts::ipe::{InfoProvEnt, formatClosureDescIpe};
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch};
use crate::ffi::rts::os_threads::Mutex;
use crate::ffi::rts::storage::closure_macros::INFO_PTR_TO_STRUCT;
use crate::ffi::rts::storage::tso::StgThreadID;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{
    StgBool, StgInt, StgInt8, StgInt32, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::get_time::getUnixEpochTime;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::schedule::{SCHED_SHUTTING_DOWN, getSchedState};
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::sparks::SparkCounters;
use crate::stats::stat_getElapsedTime;

#[cfg(test)]
mod tests;

pub(crate) type EventlogInitPost = Option<unsafe extern "C" fn() -> ()>;

pub(crate) type eventlog_init_func_t = eventlog_init_func;

/// cbindgen:no-export
pub(crate) struct eventlog_init_func {
    pub(crate) init_func: EventlogInitPost,
    pub(crate) next: *mut eventlog_init_func,
}

type EventsBuf = _EventsBuf;

/// cbindgen:no-export
struct _EventsBuf {
    begin: *mut StgInt8,
    pos: *mut StgInt8,
    marker: *mut StgInt8,
    size: StgWord64,
    capno: EventCapNo,
}

type EventType = _EventType;

/// cbindgen:no-export
struct _EventType {
    etNum: EventTypeNum,
    size: uint32_t,
    desc: *mut c_char,
}

static mut state_change_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut eventlog_enabled: bool = false;

static mut event_log_writer: *const EventLogWriter = null::<EventLogWriter>();

static mut eventlog_header_funcs: *mut eventlog_init_func_t =
    null::<eventlog_init_func_t>() as *mut eventlog_init_func_t;

const EVENT_LOG_SIZE: c_int = 2 as c_int * (1024 as c_int * 1024 as c_int);

static mut flushCount: c_int = 0 as c_int;

static mut capEventBuf: *mut EventsBuf = null::<EventsBuf>() as *mut EventsBuf;

static mut eventBuf: EventsBuf = _EventsBuf {
    begin: null::<StgInt8>() as *mut StgInt8,
    pos: null::<StgInt8>() as *mut StgInt8,
    marker: null::<StgInt8>() as *mut StgInt8,
    size: 0,
    capno: 0,
};

#[inline]
unsafe fn postWord8(mut eb: *mut EventsBuf, mut i: StgWord8) {
    let fresh0 = (*eb).pos;
    (*eb).pos = (*eb).pos.offset(1);
    *fresh0 = i as StgInt8;
}

#[inline]
unsafe fn postWord16(mut eb: *mut EventsBuf, mut i: StgWord16) {
    postWord8(eb, (i as c_int >> 8 as c_int) as StgWord8);
    postWord8(eb, i as StgWord8);
}

#[inline]
unsafe fn postWord32(mut eb: *mut EventsBuf, mut i: StgWord32) {
    postWord16(eb, (i >> 16 as c_int) as StgWord16);
    postWord16(eb, i as StgWord16);
}

#[inline]
unsafe fn postWord64(mut eb: *mut EventsBuf, mut i: StgWord64) {
    postWord32(eb, (i >> 32 as c_int) as StgWord32);
    postWord32(eb, i as StgWord32);
}

#[inline]
unsafe fn postBuf(mut eb: *mut EventsBuf, mut buf: *const StgWord8, mut size: uint32_t) {
    memcpy(
        (*eb).pos as *mut c_void,
        buf as *const c_void,
        size as size_t,
    );

    (*eb).pos = (*eb).pos.offset(size as isize);
}

#[inline]
unsafe fn postStringLen(mut eb: *mut EventsBuf, mut buf: *const c_char, mut len: StgWord) {
    if !buf.is_null() {
        memcpy(
            (*eb).pos as *mut c_void,
            buf as *const c_void,
            len as size_t,
        );

        (*eb).pos = (*eb).pos.offset(len as isize);
    }

    *(*eb).pos = 0 as StgInt8;
    (*eb).pos = (*eb).pos.offset(1);
}

#[inline]
unsafe fn time_ns() -> StgWord64 {
    return stat_getElapsedTime() as StgWord64;
}

#[inline]
unsafe fn postEventTypeNum(mut eb: *mut EventsBuf, mut etNum: EventTypeNum) {
    postWord16(eb, etNum as StgWord16);
}

#[inline]
unsafe fn postTimestamp(mut eb: *mut EventsBuf) {
    postWord64(eb, time_ns());
}

#[inline]
unsafe fn postThreadID(mut eb: *mut EventsBuf, mut id: EventThreadID) {
    postWord32(eb, id as StgWord32);
}

#[inline]
unsafe fn postCapNo(mut eb: *mut EventsBuf, mut no: EventCapNo) {
    postWord16(eb, no as StgWord16);
}

#[inline]
unsafe fn postCapsetID(mut eb: *mut EventsBuf, mut id: EventCapsetID) {
    postWord32(eb, id as StgWord32);
}

#[inline]
unsafe fn postCapsetType(mut eb: *mut EventsBuf, mut r#type: EventCapsetType) {
    postWord16(eb, r#type as StgWord16);
}

#[inline]
unsafe fn postOSProcessId(mut eb: *mut EventsBuf, mut pid: pid_t) {
    postWord32(eb, pid as StgWord32);
}

#[inline]
unsafe fn postKernelThreadId(mut eb: *mut EventsBuf, mut tid: EventKernelThreadId) {
    postWord64(eb, tid as StgWord64);
}

#[inline]
unsafe fn postTaskId(mut eb: *mut EventsBuf, mut tUniq: EventTaskId) {
    postWord64(eb, tUniq as StgWord64);
}

#[inline]
unsafe fn postPayloadSize(mut eb: *mut EventsBuf, mut size: EventPayloadSize) {
    postWord16(eb, size as StgWord16);
}

#[inline]
unsafe fn postEventHeader(mut eb: *mut EventsBuf, mut r#type: EventTypeNum) {
    postEventTypeNum(eb, r#type);
    postTimestamp(eb);
}

#[inline]
unsafe fn postInt8(mut eb: *mut EventsBuf, mut i: StgInt8) {
    postWord8(eb, i as StgWord8);
}

#[inline]
unsafe fn postInt32(mut eb: *mut EventsBuf, mut i: StgInt32) {
    postWord32(eb, i as StgWord32);
}

unsafe fn initEventLogWriter() {
    if !event_log_writer.is_null() && (*event_log_writer).initEventLogWriter.is_some() {
        (*event_log_writer)
            .initEventLogWriter
            .expect("non-null function pointer")();
    }
}

unsafe fn writeEventLog(mut eventlog: *mut c_void, mut eventlog_size: size_t) -> bool {
    if !event_log_writer.is_null() && (*event_log_writer).writeEventLog.is_some() {
        return (*event_log_writer)
            .writeEventLog
            .expect("non-null function pointer")(eventlog, eventlog_size);
    } else {
        return r#false != 0;
    };
}

unsafe fn stopEventLogWriter() {
    if !event_log_writer.is_null() && (*event_log_writer).stopEventLogWriter.is_some() {
        (*event_log_writer)
            .stopEventLogWriter
            .expect("non-null function pointer")();
    }
}

unsafe fn flushEventLogWriter() {
    if !event_log_writer.is_null() && (*event_log_writer).flushEventLog.is_some() {
        (*event_log_writer)
            .flushEventLog
            .expect("non-null function pointer")();
    }
}

unsafe fn postHeaderEvents() {
    resetEventsBuf(&raw mut eventBuf);
    postInt32(&raw mut eventBuf, EVENT_HEADER_BEGIN as StgInt32);
    postInt32(&raw mut eventBuf, EVENT_HET_BEGIN as StgInt32);

    let mut t = 0 as c_int;

    while t < NUM_GHC_EVENT_TAGS {
        if !eventTypes[t as usize].desc.is_null() {
            postEventType(
                &raw mut eventBuf,
                (&raw mut eventTypes as *mut EventType).offset(t as isize) as *mut EventType,
            );
        }

        t += 1;
    }

    postInt32(&raw mut eventBuf, EVENT_HET_END as StgInt32);
    postInt32(&raw mut eventBuf, EVENT_HEADER_END as StgInt32);
    postInt32(&raw mut eventBuf, EVENT_DATA_BEGIN as StgInt32);
}

unsafe fn postInitEvent(mut post_init: EventlogInitPost) {
    let mut new_func = null_mut::<eventlog_init_func_t>();

    new_func = stgMallocBytes(
        size_of::<eventlog_init_func_t>() as size_t,
        b"eventlog_init_func\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut eventlog_init_func_t;

    (*new_func).init_func = post_init;
    (*new_func).next = eventlog_header_funcs as *mut eventlog_init_func;
    eventlog_header_funcs = new_func;
    Some(post_init.expect("non-null function pointer")).expect("non-null function pointer")();
}

unsafe fn repostInitEvents() {
    let mut current_event = eventlog_header_funcs;

    while !current_event.is_null() {
        Some(
            (*current_event)
                .init_func
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")();
        current_event = (*current_event).next as *mut eventlog_init_func_t;
    }
}

unsafe fn resetInitEvents() {
    let mut tmp = null_mut::<eventlog_init_func_t>();
    let mut current_event = eventlog_header_funcs;

    while !current_event.is_null() {
        tmp = current_event;
        current_event = (*current_event).next as *mut eventlog_init_func_t;
        stgFree(tmp as *mut c_void);
    }

    eventlog_header_funcs = null_mut::<eventlog_init_func_t>();
}

unsafe fn get_n_capabilities() -> uint32_t {
    return 1 as uint32_t;
}

unsafe fn initEventLogging() {
    moreCapEventBufs(0 as uint32_t, get_n_capabilities());

    initEventsBuf(
        &raw mut eventBuf,
        EVENT_LOG_SIZE as StgWord64,
        -(1 as c_int) as EventCapNo,
    );
}

unsafe fn eventLogStatus() -> EventLogStatus {
    if eventlog_enabled {
        return EVENTLOG_RUNNING;
    } else {
        return EVENTLOG_NOT_CONFIGURED;
    };
}

unsafe fn startEventLogging_() -> bool {
    initEventLogWriter();
    postHeaderEvents();
    printAndClearEventBuf(&raw mut eventBuf);

    return r#true != 0;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startEventLogging(mut ev_writer: *const EventLogWriter) -> bool {
    if 0 as c_int != 0 as c_int {
        return r#false != 0;
    }

    if eventlog_enabled as c_int != 0 || !event_log_writer.is_null() {
        return r#false != 0;
    }

    event_log_writer = ev_writer;

    let mut ret = startEventLogging_();
    eventlog_enabled = r#true != 0;
    repostInitEvents();

    return ret;
}

unsafe fn restartEventLogging() {
    freeEventLoggingBuffer();
    stopEventLogWriter();
    initEventLogging();

    if !event_log_writer.is_null() {
        startEventLogging_();
        repostInitEvents();
    }
}

unsafe fn finishCapEventLogging() {
    if eventlog_enabled {
        let mut c: uint32_t = 0 as uint32_t;

        while c < getNumCapabilities() as uint32_t {
            if !(*capEventBuf.offset(c as isize)).begin.is_null() {
                printAndClearEventBuf(capEventBuf.offset(c as isize) as *mut EventsBuf);
                stgFree((*capEventBuf.offset(c as isize)).begin as *mut c_void);

                let ref mut fresh7 = (*capEventBuf.offset(c as isize)).begin;
                *fresh7 = null_mut::<StgInt8>();
            }

            c = c.wrapping_add(1);
        }
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn endEventLogging() {
    if !eventlog_enabled {
        return;
    }

    eventlog_enabled = r#false != 0;

    if getSchedState() as c_uint != SCHED_SHUTTING_DOWN as c_int as c_uint {
        flushEventLog(null_mut::<*mut Capability>());
    }

    postEventTypeNum(&raw mut eventBuf, EVENT_DATA_END as EventTypeNum);
    printAndClearEventBuf(&raw mut eventBuf);
    stopEventLogWriter();
    event_log_writer = null::<EventLogWriter>();
}

unsafe fn moreCapEventBufs(mut from: uint32_t, mut to: uint32_t) {
    if from > 0 as uint32_t {
        capEventBuf = stgReallocBytes(
            capEventBuf as *mut c_void,
            (to as size_t).wrapping_mul(size_of::<EventsBuf>() as size_t),
            b"moreCapEventBufs\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut EventsBuf;
    } else {
        capEventBuf = stgMallocBytes(
            (to as size_t).wrapping_mul(size_of::<EventsBuf>() as size_t),
            b"moreCapEventBufs\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut EventsBuf;
    }

    let mut c: uint32_t = from;

    while c < to {
        initEventsBuf(
            capEventBuf.offset(c as isize) as *mut EventsBuf,
            EVENT_LOG_SIZE as StgWord64,
            c as EventCapNo,
        );

        c = c.wrapping_add(1);
    }

    if from > 0 as uint32_t {
        let mut c_0: uint32_t = from;

        while c_0 < to {
            postBlockMarker(capEventBuf.offset(c_0 as isize) as *mut EventsBuf);
            c_0 = c_0.wrapping_add(1);
        }
    }
}

unsafe fn freeEventLoggingBuffer() {
    if !capEventBuf.is_null() {
        stgFree(capEventBuf as *mut c_void);
        capEventBuf = null_mut::<EventsBuf>();
    }
}

unsafe fn freeEventLogging() {
    freeEventLoggingBuffer();
    resetInitEvents();
}

unsafe fn postSchedEvent(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut thread: StgThreadID,
    mut info1: StgWord,
    mut info2: StgWord,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag as c_int {
        EVENT_CREATE_THREAD | EVENT_RUN_THREAD | EVENT_THREAD_RUNNABLE => {
            postThreadID(eb, thread as EventThreadID);
        }
        EVENT_CREATE_SPARK_THREAD => {
            postThreadID(eb, info1 as EventThreadID);
        }
        EVENT_MIGRATE_THREAD | EVENT_THREAD_WAKEUP => {
            postThreadID(eb, thread as EventThreadID);
            postCapNo(eb, info1 as EventCapNo);
        }
        EVENT_STOP_THREAD => {
            postThreadID(eb, thread as EventThreadID);
            postWord16(eb, info1 as StgWord16);
            postThreadID(eb, info2 as EventThreadID);
        }
        _ => {
            barf(
                b"postSchedEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
}

unsafe fn postSparkEvent(mut cap: *mut Capability, mut tag: EventTypeNum, mut info1: StgWord) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag as c_int {
        EVENT_CREATE_SPARK_THREAD => {
            postThreadID(eb, info1 as EventThreadID);
        }
        EVENT_SPARK_STEAL => {
            postCapNo(eb, info1 as EventCapNo);
        }

        EVENT_SPARK_CREATE | EVENT_SPARK_DUD | EVENT_SPARK_OVERFLOW | EVENT_SPARK_RUN
        | EVENT_SPARK_FIZZLE | EVENT_SPARK_GC => {}
        _ => {
            barf(
                b"postSparkEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
}

unsafe fn postSparkCountersEvent(
    mut cap: *mut Capability,
    mut counters: SparkCounters,
    mut remaining: StgWord,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EVENT_SPARK_COUNTERS as EventTypeNum);
    postEventHeader(eb, EVENT_SPARK_COUNTERS as EventTypeNum);
    postWord64(eb, counters.created as StgWord64);
    postWord64(eb, counters.dud as StgWord64);
    postWord64(eb, counters.overflowed as StgWord64);
    postWord64(eb, counters.converted as StgWord64);
    postWord64(eb, counters.gcd as StgWord64);
    postWord64(eb, counters.fizzled as StgWord64);
    postWord64(eb, remaining as StgWord64);
}

unsafe fn postCapEvent(mut tag: EventTypeNum, mut capno: EventCapNo) {
    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);

    match tag as c_int {
        EVENT_CAP_CREATE | EVENT_CAP_DELETE | EVENT_CAP_ENABLE | EVENT_CAP_DISABLE => {
            postCapNo(&raw mut eventBuf, capno);
        }
        _ => {
            barf(
                b"postCapEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
}

unsafe fn postCapsetEvent(mut tag: EventTypeNum, mut capset: EventCapsetID, mut info: StgWord) {
    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);
    postCapsetID(&raw mut eventBuf, capset);

    match tag as c_int {
        EVENT_CAPSET_CREATE => {
            postCapsetType(&raw mut eventBuf, info as EventCapsetType);
        }
        EVENT_CAPSET_DELETE => {}
        EVENT_CAPSET_ASSIGN_CAP | EVENT_CAPSET_REMOVE_CAP => {
            postCapNo(&raw mut eventBuf, info as EventCapNo);
        }
        EVENT_OSPROCESS_PID | EVENT_OSPROCESS_PPID => {
            postOSProcessId(&raw mut eventBuf, info as pid_t);
        }
        _ => {
            barf(
                b"postCapsetEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
}

unsafe fn postCapsetStrEvent(
    mut tag: EventTypeNum,
    mut capset: EventCapsetID,
    mut msg: *mut c_char,
) {
    let mut strsize = strlen(msg) as c_int;
    let mut size = (strsize as usize).wrapping_add(size_of::<EventCapsetID>() as usize) as c_int;

    if size > EVENT_PAYLOAD_SIZE_MAX {
        errorBelch(
            b"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out\0" as *const u8 as *const c_char,
        );

        return;
    }

    if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
        printAndClearEventBuf(&raw mut eventBuf);

        if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
            errorBelch(b"Event size exceeds buffer size, bail out\0" as *const u8 as *const c_char);
            return;
        }
    }

    postEventHeader(&raw mut eventBuf, tag);
    postPayloadSize(&raw mut eventBuf, size as EventPayloadSize);
    postCapsetID(&raw mut eventBuf, capset);
    postBuf(&raw mut eventBuf, msg as *mut StgWord8, strsize as uint32_t);
}

unsafe fn postCapsetVecEvent(
    mut tag: EventTypeNum,
    mut capset: EventCapsetID,
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) {
    let mut size = size_of::<EventCapsetID>() as c_int;
    let mut i = 0 as c_int;

    while i < argc {
        let mut increment = (1 as size_t).wrapping_add(strlen(*argv.offset(i as isize))) as c_int;

        if size + increment > EVENT_PAYLOAD_SIZE_MAX {
            errorBelch(
                b"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, record only %llu out of %llu args\0"
                    as *const u8 as *const c_char,
                i as StgWord,
                argc as StgWord,
            );

            argc = i;
            break;
        } else {
            size += increment;
            i += 1;
        }
    }

    if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
        printAndClearEventBuf(&raw mut eventBuf);

        if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
            errorBelch(b"Event size exceeds buffer size, bail out\0" as *const u8 as *const c_char);
            return;
        }
    }

    postEventHeader(&raw mut eventBuf, tag);
    postPayloadSize(&raw mut eventBuf, size as EventPayloadSize);
    postCapsetID(&raw mut eventBuf, capset);

    let mut i_0 = 0 as c_int;

    while i_0 < argc {
        postBuf(
            &raw mut eventBuf,
            *argv.offset(i_0 as isize) as *mut StgWord8,
            (1 as size_t).wrapping_add(strlen(*argv.offset(i_0 as isize))) as uint32_t,
        );

        i_0 += 1;
    }
}

unsafe fn postWallClockTime(mut capset: EventCapsetID) {
    let mut ts: StgWord64 = 0;
    let mut sec: StgWord64 = 0;
    let mut nsec: StgWord32 = 0;
    getUnixEpochTime(&raw mut sec, &raw mut nsec);
    ts = time_ns();
    ensureRoomForEvent(&raw mut eventBuf, EVENT_WALL_CLOCK_TIME as EventTypeNum);
    postEventTypeNum(&raw mut eventBuf, EVENT_WALL_CLOCK_TIME as EventTypeNum);
    postWord64(&raw mut eventBuf, ts);
    postCapsetID(&raw mut eventBuf, capset);
    postWord64(&raw mut eventBuf, sec);
    postWord32(&raw mut eventBuf, nsec);
}

unsafe fn postHeapEvent(
    mut cap: *mut Capability,
    mut tag: EventTypeNum,
    mut heap_capset: EventCapsetID,
    mut info1: W_,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag as c_int {
        EVENT_HEAP_ALLOCATED | EVENT_HEAP_SIZE | EVENT_BLOCKS_SIZE | EVENT_HEAP_LIVE => {
            postCapsetID(eb, heap_capset);
            postWord64(eb, info1 as StgWord64);
        }
        _ => {
            barf(
                b"postHeapEvent: unknown event tag %d\0" as *const u8 as *const c_char,
                tag as c_int,
            );
        }
    };
}

unsafe fn postEventHeapInfo(
    mut heap_capset: EventCapsetID,
    mut gens: uint32_t,
    mut maxHeapSize: W_,
    mut allocAreaSize: W_,
    mut mblockSize: W_,
    mut blockSize: W_,
) {
    ensureRoomForEvent(&raw mut eventBuf, EVENT_HEAP_INFO_GHC as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_HEAP_INFO_GHC as EventTypeNum);
    postCapsetID(&raw mut eventBuf, heap_capset);
    postWord16(&raw mut eventBuf, gens as StgWord16);
    postWord64(&raw mut eventBuf, maxHeapSize as StgWord64);
    postWord64(&raw mut eventBuf, allocAreaSize as StgWord64);
    postWord64(&raw mut eventBuf, mblockSize as StgWord64);
    postWord64(&raw mut eventBuf, blockSize as StgWord64);
}

unsafe fn postEventGcStats(
    mut cap: *mut Capability,
    mut heap_capset: EventCapsetID,
    mut r#gen: uint32_t,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: uint32_t,
    mut par_max_copied: W_,
    mut par_tot_copied: W_,
    mut par_balanced_copied: W_,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EVENT_GC_STATS_GHC as EventTypeNum);
    postEventHeader(eb, EVENT_GC_STATS_GHC as EventTypeNum);
    postCapsetID(eb, heap_capset);
    postWord16(eb, r#gen as StgWord16);
    postWord64(eb, copied as StgWord64);
    postWord64(eb, slop as StgWord64);
    postWord64(eb, fragmentation as StgWord64);
    postWord32(eb, par_n_threads as StgWord32);
    postWord64(eb, par_max_copied as StgWord64);
    postWord64(eb, par_tot_copied as StgWord64);
    postWord64(eb, par_balanced_copied as StgWord64);
}

unsafe fn postEventMemReturn(
    mut cap: *mut Capability,
    mut heap_capset: EventCapsetID,
    mut current_mblocks: uint32_t,
    mut needed_mblocks: uint32_t,
    mut returned_mblocks: uint32_t,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EVENT_MEM_RETURN as EventTypeNum);
    postEventHeader(eb, EVENT_MEM_RETURN as EventTypeNum);
    postCapsetID(eb, heap_capset);
    postWord32(eb, current_mblocks as StgWord32);
    postWord32(eb, needed_mblocks as StgWord32);
    postWord32(eb, returned_mblocks as StgWord32);
}

unsafe fn postTaskCreateEvent(
    mut taskId: EventTaskId,
    mut capno: EventCapNo,
    mut tid: EventKernelThreadId,
) {
    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_CREATE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_CREATE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);
    postCapNo(&raw mut eventBuf, capno);
    postKernelThreadId(&raw mut eventBuf, tid);
}

unsafe fn postTaskMigrateEvent(
    mut taskId: EventTaskId,
    mut capno: EventCapNo,
    mut new_capno: EventCapNo,
) {
    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_MIGRATE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_MIGRATE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);
    postCapNo(&raw mut eventBuf, capno);
    postCapNo(&raw mut eventBuf, new_capno);
}

unsafe fn postTaskDeleteEvent(mut taskId: EventTaskId) {
    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_DELETE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_DELETE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);
}

unsafe fn postEventNoCap(mut tag: EventTypeNum) {
    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);
}

unsafe fn postEvent(mut cap: *mut Capability, mut tag: EventTypeNum) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);
}

unsafe fn postEventAtTimestamp(
    mut cap: *mut Capability,
    mut ts: EventTimestamp,
    mut tag: EventTypeNum,
) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventTypeNum(eb, tag);
    postWord64(eb, ts as StgWord64);
}

const BUF: c_int = 512 as c_int;

unsafe fn postLogMsg(
    mut eb: *mut EventsBuf,
    mut r#type: EventTypeNum,
    mut msg: *mut c_char,
    mut ap: VaList,
) {
    let mut buf: [c_char; 512] = [0; 512];

    let mut size: uint32_t = vsnprintf(
        &raw mut buf as *mut c_char,
        BUF as size_t,
        msg,
        ap.as_va_list(),
    ) as uint32_t;

    if size > BUF as uint32_t {
        buf[(BUF - 1 as c_int) as usize] = '\0' as i32 as c_char;
        size = BUF as uint32_t;
    }

    ensureRoomForVariableEvent(eb, size as StgWord);
    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, &raw mut buf as *mut c_char as *mut StgWord8, size);
}

unsafe fn postMsg(mut msg: *mut c_char, mut ap: VaList) {
    postLogMsg(
        &raw mut eventBuf,
        EVENT_LOG_MSG as EventTypeNum,
        msg,
        ap.as_va_list(),
    );
}

unsafe fn postCapMsg(mut cap: *mut Capability, mut msg: *mut c_char, mut ap: VaList) {
    postLogMsg(
        capEventBuf.offset((*cap).no as isize) as *mut EventsBuf,
        EVENT_LOG_MSG as EventTypeNum,
        msg,
        ap.as_va_list(),
    );
}

unsafe fn postUserEvent(mut cap: *mut Capability, mut r#type: EventTypeNum, mut msg: *mut c_char) {
    let size = strlen(msg) as size_t;

    if size > EVENT_PAYLOAD_SIZE_MAX as size_t {
        errorBelch(
            b"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out\0" as *const u8 as *const c_char,
        );

        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(b"Event size exceeds buffer size, bail out\0" as *const u8 as *const c_char);
            return;
        }
    }

    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, msg as *mut StgWord8, size as uint32_t);
}

unsafe fn postUserBinaryEvent(
    mut cap: *mut Capability,
    mut r#type: EventTypeNum,
    mut msg: *mut uint8_t,
    mut size: size_t,
) {
    if size > EVENT_PAYLOAD_SIZE_MAX as size_t {
        errorBelch(
            b"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out\0" as *const u8 as *const c_char,
        );

        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(b"Event size exceeds buffer size, bail out\0" as *const u8 as *const c_char);
            return;
        }
    }

    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, msg as *mut StgWord8, size as uint32_t);
}

unsafe fn postThreadLabel(
    mut cap: *mut Capability,
    mut id: EventThreadID,
    mut label: *mut c_char,
    mut len: size_t,
) {
    let strsize = len as c_int;
    let size = (strsize as usize).wrapping_add(size_of::<EventThreadID>() as usize) as c_int;

    if size > EVENT_PAYLOAD_SIZE_MAX {
        errorBelch(
            b"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out\0" as *const u8 as *const c_char,
        );

        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(b"Event size exceeds buffer size, bail out\0" as *const u8 as *const c_char);
            return;
        }
    }

    postEventHeader(eb, EVENT_THREAD_LABEL as EventTypeNum);
    postPayloadSize(eb, size as EventPayloadSize);
    postThreadID(eb, id);
    postBuf(eb, label as *mut StgWord8, strsize as uint32_t);
}

unsafe fn postConcUpdRemSetFlush(mut cap: *mut Capability) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EVENT_CONC_UPD_REM_SET_FLUSH as EventTypeNum);
    postEventHeader(eb, EVENT_CONC_UPD_REM_SET_FLUSH as EventTypeNum);
    postCapNo(eb, (*cap).no as EventCapNo);
}

unsafe fn postConcMarkEnd(mut marked_obj_count: StgWord32) {
    ensureRoomForEvent(&raw mut eventBuf, EVENT_CONC_MARK_END as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_CONC_MARK_END as EventTypeNum);
    postWord32(&raw mut eventBuf, marked_obj_count);
}

unsafe fn postNonmovingHeapCensus(mut blk_size: uint16_t, mut census: *const NonmovingAllocCensus) {
    postEventHeader(
        &raw mut eventBuf,
        EVENT_NONMOVING_HEAP_CENSUS as EventTypeNum,
    );

    postWord16(&raw mut eventBuf, blk_size as StgWord16);
    postWord32(&raw mut eventBuf, (*census).n_active_segs as StgWord32);
    postWord32(&raw mut eventBuf, (*census).n_filled_segs as StgWord32);
    postWord32(&raw mut eventBuf, (*census).n_live_blocks as StgWord32);
}

unsafe fn postNonmovingPrunedSegments(mut pruned_segments: uint32_t, mut free_segments: uint32_t) {
    postEventHeader(
        &raw mut eventBuf,
        EVENT_NONMOVING_PRUNED_SEGMENTS as EventTypeNum,
    );

    postWord32(&raw mut eventBuf, pruned_segments as StgWord32);
    postWord32(&raw mut eventBuf, free_segments as StgWord32);
}

unsafe fn closeBlockMarker(mut ebuf: *mut EventsBuf) {
    if !(*ebuf).marker.is_null() {
        let mut save_pos = (*ebuf).pos;
        (*ebuf).pos = (*ebuf)
            .marker
            .offset(size_of::<EventTypeNum>() as usize as isize)
            .offset(size_of::<EventTimestamp>() as usize as isize);

        postWord32(
            ebuf,
            save_pos.offset_from((*ebuf).marker) as c_long as StgWord32,
        );

        postTimestamp(ebuf);
        (*ebuf).pos = save_pos;
        (*ebuf).marker = null_mut::<StgInt8>();
    }
}

unsafe fn postBlockMarker(mut eb: *mut EventsBuf) {
    ensureRoomForEvent(eb, EVENT_BLOCK_MARKER as EventTypeNum);
    closeBlockMarker(eb);
    (*eb).marker = (*eb).pos;
    postEventHeader(eb, EVENT_BLOCK_MARKER as EventTypeNum);
    postWord32(eb, 0 as StgWord32);
    postWord64(eb, 0 as StgWord64);
    postCapNo(eb, (*eb).capno);
}

unsafe fn getHeapProfBreakdown() -> HeapProfBreakdown {
    match RtsFlags.ProfFlags.doHeapProfile {
        1 => return HEAP_PROF_BREAKDOWN_COST_CENTRE,
        2 => return HEAP_PROF_BREAKDOWN_MODULE,
        4 => return HEAP_PROF_BREAKDOWN_CLOSURE_DESCR,
        5 => return HEAP_PROF_BREAKDOWN_TYPE_DESCR,
        6 => return HEAP_PROF_BREAKDOWN_RETAINER,
        7 => return HEAP_PROF_BREAKDOWN_BIOGRAPHY,
        8 => return HEAP_PROF_BREAKDOWN_CLOSURE_TYPE,
        9 => return HEAP_PROF_BREAKDOWN_INFO_TABLE,
        10 => return HEAP_PROF_BREAKDOWN_ERA,
        _ => {
            barf(
                b"getHeapProfBreakdown: unknown heap profiling mode\0" as *const u8
                    as *const c_char,
            );
        }
    };
}

unsafe fn postHeapProfBegin(mut profile_id: StgWord8) {
    let mut flags: *mut PROFILING_FLAGS = &raw mut RtsFlags.ProfFlags;

    let mut modSelector_len: StgWord = (if !(*flags).modSelector.is_null() {
        strlen((*flags).modSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut descrSelector_len: StgWord = (if !(*flags).descrSelector.is_null() {
        strlen((*flags).descrSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut typeSelector_len: StgWord = (if !(*flags).typeSelector.is_null() {
        strlen((*flags).typeSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut ccSelector_len: StgWord = (if !(*flags).ccSelector.is_null() {
        strlen((*flags).ccSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut ccsSelector_len: StgWord = (if !(*flags).ccsSelector.is_null() {
        strlen((*flags).ccsSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut retainerSelector_len: StgWord = (if !(*flags).retainerSelector.is_null() {
        strlen((*flags).retainerSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut bioSelector_len: StgWord = (if !(*flags).bioSelector.is_null() {
        strlen((*flags).bioSelector)
    } else {
        0 as size_t
    }) as StgWord;

    let mut len: StgWord = ((1 as c_int + 8 as c_int + 4 as c_int) as StgWord)
        .wrapping_add(modSelector_len)
        .wrapping_add(descrSelector_len)
        .wrapping_add(typeSelector_len)
        .wrapping_add(ccSelector_len)
        .wrapping_add(ccsSelector_len)
        .wrapping_add(retainerSelector_len)
        .wrapping_add(bioSelector_len)
        .wrapping_add(7 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/eventlog/EventLog.c\0" as *const u8 as *const c_char,
            1244 as c_uint,
        );
    }

    postEventHeader(&raw mut eventBuf, EVENT_HEAP_PROF_BEGIN as EventTypeNum);
    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord8(&raw mut eventBuf, profile_id);
    postWord64(&raw mut eventBuf, (*flags).heapProfileInterval as StgWord64);
    postWord32(&raw mut eventBuf, getHeapProfBreakdown() as StgWord32);
    postStringLen(&raw mut eventBuf, (*flags).modSelector, modSelector_len);
    postStringLen(&raw mut eventBuf, (*flags).descrSelector, descrSelector_len);
    postStringLen(&raw mut eventBuf, (*flags).typeSelector, typeSelector_len);
    postStringLen(&raw mut eventBuf, (*flags).ccSelector, ccSelector_len);
    postStringLen(&raw mut eventBuf, (*flags).ccsSelector, ccsSelector_len);

    postStringLen(
        &raw mut eventBuf,
        (*flags).retainerSelector,
        retainerSelector_len,
    );

    postStringLen(&raw mut eventBuf, (*flags).bioSelector, bioSelector_len);
}

unsafe fn postHeapProfSampleBegin(mut era: StgInt) {
    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_BEGIN as EventTypeNum,
    );

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_BEGIN as EventTypeNum,
    );

    postWord64(&raw mut eventBuf, era as StgWord64);
}

unsafe fn postHeapBioProfSampleBegin(mut era: StgInt, mut time: StgWord64) {
    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN as EventTypeNum,
    );

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN as EventTypeNum,
    );

    postWord64(&raw mut eventBuf, era as StgWord64);
    postWord64(&raw mut eventBuf, time);
}

unsafe fn postHeapProfSampleEnd(mut era: StgInt) {
    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_END as EventTypeNum,
    );

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_END as EventTypeNum,
    );

    postWord64(&raw mut eventBuf, era as StgWord64);
}

unsafe fn postHeapProfSampleString(
    mut profile_id: StgWord8,
    mut label: *const c_char,
    mut residency: StgWord64,
) {
    let mut label_len: StgWord = strlen(label) as StgWord;
    let mut len: StgWord = ((1 as c_int + 8 as c_int) as StgWord)
        .wrapping_add(label_len)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/eventlog/EventLog.c\0" as *const u8 as *const c_char,
            1296 as c_uint,
        );
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_STRING as EventTypeNum,
    );

    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord8(&raw mut eventBuf, profile_id);
    postWord64(&raw mut eventBuf, residency);
    postStringLen(&raw mut eventBuf, label, label_len);
}

unsafe fn postIPE(mut ipe: *const InfoProvEnt) {
    let mut closure_desc_buf: [c_char; 11] = [0; 11];
    formatClosureDescIpe(ipe, &raw mut closure_desc_buf as *mut c_char);

    let MAX_IPE_STRING_LEN: StgWord = 65535 as StgWord;
    let mut table_name_len: StgWord =
        if (strlen((*ipe).prov.table_name) as StgWord) < MAX_IPE_STRING_LEN {
            strlen((*ipe).prov.table_name) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut closure_desc_len: StgWord =
        if (strlen(&raw mut closure_desc_buf as *mut c_char) as StgWord) < MAX_IPE_STRING_LEN {
            strlen(&raw mut closure_desc_buf as *mut c_char) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut ty_desc_len: StgWord = if (strlen((*ipe).prov.ty_desc) as StgWord) < MAX_IPE_STRING_LEN
    {
        strlen((*ipe).prov.ty_desc) as StgWord
    } else {
        MAX_IPE_STRING_LEN
    };

    let mut label_len: StgWord = if (strlen((*ipe).prov.label) as StgWord) < MAX_IPE_STRING_LEN {
        strlen((*ipe).prov.label) as StgWord
    } else {
        MAX_IPE_STRING_LEN
    };

    let mut module_len: StgWord = if (strlen((*ipe).prov.module) as StgWord) < MAX_IPE_STRING_LEN {
        strlen((*ipe).prov.module) as StgWord
    } else {
        MAX_IPE_STRING_LEN
    };

    let mut src_file_len: StgWord =
        if (strlen((*ipe).prov.src_file) as StgWord) < MAX_IPE_STRING_LEN {
            strlen((*ipe).prov.src_file) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut src_span_len: StgWord =
        if (strlen((*ipe).prov.src_span) as StgWord) < MAX_IPE_STRING_LEN {
            strlen((*ipe).prov.src_span) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut extra_comma: StgWord = 1 as StgWord;
    let mut len: StgWord = (8 as StgWord)
        .wrapping_add(table_name_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(closure_desc_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(ty_desc_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(label_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(module_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(src_file_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(extra_comma)
        .wrapping_add(src_span_len)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/eventlog/EventLog.c\0" as *const u8 as *const c_char,
            1472 as c_uint,
        );
    }

    postEventHeader(&raw mut eventBuf, EVENT_IPE as EventTypeNum);
    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);

    postWord64(
        &raw mut eventBuf,
        INFO_PTR_TO_STRUCT((*ipe).info) as StgWord64,
    );

    postStringLen(&raw mut eventBuf, (*ipe).prov.table_name, table_name_len);

    postStringLen(
        &raw mut eventBuf,
        &raw mut closure_desc_buf as *mut c_char,
        closure_desc_len,
    );

    postStringLen(&raw mut eventBuf, (*ipe).prov.ty_desc, ty_desc_len);
    postStringLen(&raw mut eventBuf, (*ipe).prov.label, label_len);
    postStringLen(&raw mut eventBuf, (*ipe).prov.module, module_len);

    postBuf(
        &raw mut eventBuf,
        (*ipe).prov.src_file as *const StgWord8,
        src_file_len as uint32_t,
    );

    let mut colon: StgWord8 = ':' as i32 as StgWord8;
    postBuf(&raw mut eventBuf, &raw mut colon, 1 as uint32_t);
    postStringLen(&raw mut eventBuf, (*ipe).prov.src_span, src_span_len);
}

unsafe fn printAndClearEventBuf(mut ebuf: *mut EventsBuf) {
    closeBlockMarker(ebuf);

    if !(*ebuf).begin.is_null() && (*ebuf).pos != (*ebuf).begin {
        let mut elog_size: size_t = (*ebuf).pos.offset_from((*ebuf).begin) as c_long as size_t;

        if !writeEventLog((*ebuf).begin as *mut c_void, elog_size) {
            debugBelch(
                b"printAndClearEventLog: could not flush event log\n\0" as *const u8
                    as *const c_char,
            );

            resetEventsBuf(ebuf);
            flushEventLogWriter();
            return;
        }

        resetEventsBuf(ebuf);
        flushCount += 1;
        postBlockMarker(ebuf);
    }
}

unsafe fn initEventsBuf(mut eb: *mut EventsBuf, mut size: StgWord64, mut capno: EventCapNo) {
    (*eb).pos = stgMallocBytes(
        size as size_t,
        b"initEventsBuf\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut StgInt8;

    (*eb).begin = (*eb).pos;
    (*eb).size = size;
    (*eb).marker = null_mut::<StgInt8>();
    (*eb).capno = capno;
    postBlockMarker(eb);
}

unsafe fn resetEventsBuf(mut eb: *mut EventsBuf) {
    (*eb).pos = (*eb).begin;
    (*eb).marker = null_mut::<StgInt8>();
}

unsafe fn hasRoomForEvent(mut eb: *mut EventsBuf, mut eNum: EventTypeNum) -> StgBool {
    let mut size: uint32_t = (size_of::<EventTypeNum>() as usize)
        .wrapping_add(size_of::<EventTimestamp>() as usize)
        .wrapping_add(eventTypes[eNum as usize].size as usize)
        as uint32_t;

    if (*eb).pos.offset(size as isize) > (*eb).begin.offset((*eb).size as isize) {
        return 0 as StgBool;
    } else {
        return 1 as StgBool;
    };
}

unsafe fn hasRoomForVariableEvent(mut eb: *mut EventsBuf, mut payload_bytes: StgWord) -> StgBool {
    let mut size: StgWord = ((size_of::<EventTypeNum>() as usize)
        .wrapping_add(size_of::<EventTimestamp>() as usize)
        .wrapping_add(size_of::<EventPayloadSize>() as usize)
        as StgWord)
        .wrapping_add(payload_bytes);

    if (*eb).pos.offset(size as isize) > (*eb).begin.offset((*eb).size as isize) {
        return 0 as StgBool;
    } else {
        return 1 as StgBool;
    };
}

unsafe fn ensureRoomForEvent(mut eb: *mut EventsBuf, mut tag: EventTypeNum) {
    if hasRoomForEvent(eb, tag) == 0 {
        printAndClearEventBuf(eb);
    }
}

unsafe fn ensureRoomForVariableEvent(mut eb: *mut EventsBuf, mut size: StgWord) -> c_int {
    if hasRoomForVariableEvent(eb, size) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size) == 0 {
            return 1 as c_int;
        }
    }

    return 0 as c_int;
}

unsafe fn postEventType(mut eb: *mut EventsBuf, mut et: *mut EventType) {
    postInt32(eb, EVENT_ET_BEGIN as StgInt32);
    postEventTypeNum(eb, (*et).etNum);
    postWord16(eb, (*et).size as StgWord16);

    let desclen = strlen((*et).desc) as c_int;
    postWord32(eb, desclen as StgWord32);

    let mut d = 0 as c_int;

    while d < desclen {
        postInt8(eb, *(*et).desc.offset(d as isize) as StgInt8);
        d += 1;
    }

    postWord32(eb, 0 as StgWord32);
    postInt32(eb, EVENT_ET_END as StgInt32);
}

unsafe fn flushLocalEventsBuf(mut cap: *mut Capability) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    printAndClearEventBuf(eb);
}

unsafe fn flushAllCapsEventsBufs() {
    if event_log_writer.is_null() {
        return;
    }

    printAndClearEventBuf(&raw mut eventBuf);

    let mut i = 0 as c_uint;

    while i < getNumCapabilities() {
        flushLocalEventsBuf(getCapability(i as uint32_t));
        i = i.wrapping_add(1);
    }

    flushEventLogWriter();
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn flushEventLog(mut cap: *mut *mut Capability) {
    if event_log_writer.is_null() {
        return;
    }

    printAndClearEventBuf(&raw mut eventBuf);
    flushLocalEventsBuf(getCapability(0 as uint32_t));
    flushEventLogWriter();
}

unsafe fn run_static_initializers() {
    eventTypes = [
        _EventType {
            etNum: 0 as EventTypeNum,
            size: size_of::<EventThreadID>() as uint32_t,
            desc: b"Create thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 1 as EventTypeNum,
            size: size_of::<EventThreadID>() as uint32_t,
            desc: b"Run thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 2 as EventTypeNum,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<EventThreadID>() as usize) as uint32_t,
            desc: b"Stop thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 3 as EventTypeNum,
            size: size_of::<EventThreadID>() as uint32_t,
            desc: b"Thread runnable\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 4 as EventTypeNum,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Migrate thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 8 as EventTypeNum,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Wakeup thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 9 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Starting GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 10 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Finished GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 11 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Request sequential GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 12 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Request parallel GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 15 as EventTypeNum,
            size: size_of::<EventThreadID>() as uint32_t,
            desc: b"Create spark thread\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 16 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Log message\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 18 as EventTypeNum,
            size: (size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<EventTimestamp>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Block marker\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 19 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"User message\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 20 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"GC idle\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 21 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"GC working\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 22 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"GC done\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 25 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapsetType>() as usize) as uint32_t,
            desc: b"Create capability set\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 26 as EventTypeNum,
            size: size_of::<EventCapsetID>() as uint32_t,
            desc: b"Delete capability set\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 27 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Add capability to capability set\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 28 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Remove capability from capability set\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 29 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"RTS name and version\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 30 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Program arguments\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 31 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Program environment variables\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 32 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as uint32_t,
            desc: b"Process ID\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 33 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as uint32_t,
            desc: b"Parent process ID\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 34 as EventTypeNum,
            size: (size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Spark counters\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 35 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark create\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 36 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark dud\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 37 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark overflow\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 38 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark run\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 39 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Spark steal\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 40 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark fizzle\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 41 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Spark GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 42 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Intern string\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 43 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as uint32_t,
            desc: b"Wall clock time\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 44 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Thread label\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 45 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Create capability\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 46 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Delete capability\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 47 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Disable capability\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 48 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Enable capability\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 49 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Total heap memory ever allocated\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 50 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Current heap size (number of allocated mblocks)\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 51 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Current heap live data\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 52 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Heap static parameters\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 53 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"GC statistics\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 54 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Synchronise stop-the-world GC\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 55 as EventTypeNum,
            size: (size_of::<EventTaskId>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize)
                .wrapping_add(size_of::<EventKernelThreadId>() as usize)
                as uint32_t,
            desc: b"Task create\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 56 as EventTypeNum,
            size: (size_of::<EventTaskId>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as uint32_t,
            desc: b"Task migrate\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 57 as EventTypeNum,
            size: size_of::<EventTaskId>() as uint32_t,
            desc: b"Task delete\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 58 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"User marker\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 59 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Empty event for bug #9003\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 90 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as uint32_t,
            desc: b"The RTS attempted to return heap memory to the OS\0" as *const u8
                as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 91 as EventTypeNum,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Report the size of the heap in blocks\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 160 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Start of heap profile\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 161 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Cost-centre definition\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 162 as EventTypeNum,
            size: size_of::<StgWord64>() as uint32_t,
            desc: b"Start of heap profile sample\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 163 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Heap profile cost-centre sample\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 164 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Heap profile string sample\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 165 as EventTypeNum,
            size: size_of::<StgWord64>() as uint32_t,
            desc: b"End of heap profile sample\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 166 as EventTypeNum,
            size: (size_of::<StgWord64>() as usize).wrapping_add(size_of::<StgWord64>() as usize)
                as uint32_t,
            desc: b"Start of heap profile (biographical) sample\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 167 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Time profile cost-centre stack\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 168 as EventTypeNum,
            size: size_of::<StgWord64>() as uint32_t,
            desc: b"Start of a time profile\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 169 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"An IPE entry\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 181 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"User binary message\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 200 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Begin concurrent mark phase\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 201 as EventTypeNum,
            size: size_of::<StgWord32>() as uint32_t,
            desc: b"End concurrent mark phase\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 202 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Begin concurrent GC synchronisation\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 203 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"End concurrent mark synchronisation\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 204 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Begin concurrent sweep phase\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 205 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"End concurrent sweep phase\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 206 as EventTypeNum,
            size: size_of::<EventCapNo>() as uint32_t,
            desc: b"Update remembered set flushed\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 207 as EventTypeNum,
            size: (size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as uint32_t,
            desc: b"Nonmoving heap census\0" as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 208 as EventTypeNum,
            size: (size_of::<StgWord32>() as usize).wrapping_add(size_of::<StgWord32>() as usize)
                as uint32_t,
            desc: b"Report the amount of segments pruned and remaining on the free list.\0"
                as *const u8 as *const c_char as *mut c_char,
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 210 as EventTypeNum,
            size: 0xffff as uint32_t,
            desc: b"Ticky-ticky entry counter definition\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 211 as EventTypeNum,
            size: (size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as uint32_t,
            desc: b"Ticky-ticky entry counter sample\0" as *const u8 as *const c_char
                as *mut c_char,
        },
        _EventType {
            etNum: 212 as EventTypeNum,
            size: 0 as uint32_t,
            desc: b"Ticky-ticky entry counter begin sample\0" as *const u8 as *const c_char
                as *mut c_char,
        },
    ];
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
