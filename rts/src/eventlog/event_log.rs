use crate::capability::Capability;
use crate::capability::{SYNC_FLUSH_EVENT_LOG, getCapability};
use crate::event_log_constants::{
    EVENT_BLOCK_MARKER, EVENT_CONC_MARK_END, EVENT_CONC_UPD_REM_SET_FLUSH, EVENT_GC_STATS_GHC,
    EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN, EVENT_HEAP_INFO_GHC, EVENT_HEAP_PROF_BEGIN,
    EVENT_HEAP_PROF_COST_CENTRE, EVENT_HEAP_PROF_SAMPLE_BEGIN, EVENT_HEAP_PROF_SAMPLE_COST_CENTRE,
    EVENT_HEAP_PROF_SAMPLE_END, EVENT_HEAP_PROF_SAMPLE_STRING, EVENT_IPE, EVENT_LOG_MSG,
    EVENT_MEM_RETURN, EVENT_NONMOVING_HEAP_CENSUS, EVENT_NONMOVING_PRUNED_SEGMENTS,
    EVENT_PROF_BEGIN, EVENT_PROF_SAMPLE_COST_CENTRE, EVENT_SPARK_COUNTERS, EVENT_TASK_CREATE,
    EVENT_TASK_DELETE, EVENT_TASK_MIGRATE, EVENT_THREAD_LABEL, EVENT_TICKY_COUNTER_BEGIN_SAMPLE,
    EVENT_TICKY_COUNTER_DEF, EVENT_TICKY_COUNTER_SAMPLE, EVENT_WALL_CLOCK_TIME,
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
use crate::ffi::rts::ipe::{InfoProvEnt, formatClosureDescIpe};
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch};
use crate::ffi::rts::os_threads::{Mutex, OS_TRY_ACQUIRE_LOCK, initMutex};
use crate::ffi::rts::prof::ccs::{CCS_MAIN, CostCentreStack};
use crate::ffi::rts::storage::closure_macros::INFO_PTR_TO_STRUCT;
use crate::ffi::rts::storage::tso::StgThreadID;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::ticky::StgEntCounter;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{
    StgBool, StgInt, StgInt8, StgInt32, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::get_time::getUnixEpochTime;
use crate::prelude::*;
use crate::rts_flags::{PROFILING_FLAGS, RtsFlags};
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::schedule::{
    SCHED_SHUTTING_DOWN, getSchedState, releaseAllCapabilities, stopAllCapabilitiesWith,
};
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::sparks::SparkCounters;
use crate::stats::stat_getElapsedTime;
use crate::task::getMyTask;

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
    size: u32,
    desc: *mut c_char,
}

static mut state_change_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut eventlog_enabled: bool = false;

static mut event_log_writer: *const EventLogWriter = null::<EventLogWriter>();

static mut eventlog_header_funcs: *mut eventlog_init_func_t = null_mut::<eventlog_init_func_t>();

const EVENT_LOG_SIZE: i32 = 2 * (1024 * 1024);

static mut flushCount: i32 = 0;

static mut capEventBuf: *mut EventsBuf = null_mut::<EventsBuf>();

static mut eventBuf: EventsBuf = _EventsBuf {
    begin: null_mut::<StgInt8>(),
    pos: null_mut::<StgInt8>(),
    marker: null_mut::<StgInt8>(),
    size: 0,
    capno: 0,
};

static mut eventBufMutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

#[inline]
unsafe fn postWord8(mut eb: *mut EventsBuf, mut i: StgWord8) {
    let fresh0 = (*eb).pos;
    (*eb).pos = (*eb).pos.offset(1);
    *fresh0 = i as StgInt8;
}

#[inline]
unsafe fn postWord16(mut eb: *mut EventsBuf, mut i: StgWord16) {
    postWord8(eb, (i as i32 >> 8) as StgWord8);
    postWord8(eb, i as StgWord8);
}

#[inline]
unsafe fn postWord32(mut eb: *mut EventsBuf, mut i: StgWord32) {
    postWord16(eb, (i >> 16) as StgWord16);
    postWord16(eb, i as StgWord16);
}

#[inline]
unsafe fn postWord64(mut eb: *mut EventsBuf, mut i: StgWord64) {
    postWord32(eb, (i >> 32) as StgWord32);
    postWord32(eb, i as StgWord32);
}

#[inline]
unsafe fn postBuf(mut eb: *mut EventsBuf, mut buf: *const StgWord8, mut size: u32) {
    memcpy(
        (*eb).pos as *mut c_void,
        buf as *const c_void,
        size as usize,
    );
    (*eb).pos = (*eb).pos.offset(size as isize);
}

#[inline]
unsafe fn postStringLen(mut eb: *mut EventsBuf, mut buf: *const c_char, mut len: StgWord) {
    if !buf.is_null() {
        if ((*eb).begin.offset((*eb).size as isize) > (*eb).pos.offset(len as isize).offset(1))
            as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 193);
        }

        memcpy((*eb).pos as *mut c_void, buf as *const c_void, len as usize);
        (*eb).pos = (*eb).pos.offset(len as isize);
    }

    *(*eb).pos = 0;
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

unsafe fn writeEventLog(mut eventlog: *mut c_void, mut eventlog_size: usize) -> bool {
    if !event_log_writer.is_null() && (*event_log_writer).writeEventLog.is_some() {
        return (*event_log_writer)
            .writeEventLog
            .expect("non-null function pointer")(eventlog, eventlog_size);
    } else {
        return false;
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

    let mut t = 0;

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
    let mut __r = pthread_mutex_lock(&raw mut state_change_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            318,
            __r,
        );
    }

    let mut new_func = null_mut::<eventlog_init_func_t>();

    new_func = stgMallocBytes(
        size_of::<eventlog_init_func_t>() as usize,
        c"eventlog_init_func".as_ptr(),
    ) as *mut eventlog_init_func_t;

    (*new_func).init_func = post_init;
    (*new_func).next = eventlog_header_funcs as *mut eventlog_init_func;
    eventlog_header_funcs = new_func;

    if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            328,
        );
    }

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

unsafe fn get_n_capabilities() -> u32 {
    let mut n = getNumCapabilities();

    return if n != 0 {
        n as u32
    } else {
        RtsFlags.ParFlags.nCapabilities
    };
}

unsafe fn initEventLogging() {
    moreCapEventBufs(0, get_n_capabilities());
    initEventsBuf(
        &raw mut eventBuf,
        EVENT_LOG_SIZE as StgWord64,
        -1 as EventCapNo,
    );
    initMutex(&raw mut eventBufMutex);
    initMutex(&raw mut state_change_mutex);
}

unsafe fn eventLogStatus() -> EventLogStatus {
    if (&raw mut eventlog_enabled).load(Ordering::Relaxed) {
        return EVENTLOG_RUNNING;
    } else {
        return EVENTLOG_NOT_CONFIGURED;
    };
}

unsafe fn startEventLogging_() -> bool {
    initEventLogWriter();

    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            410,
            __r,
        );
    }

    postHeaderEvents();
    printAndClearEventBuf(&raw mut eventBuf);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            419,
        );
    }

    return true;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startEventLogging(mut ev_writer: *const EventLogWriter) -> bool {
    if OS_TRY_ACQUIRE_LOCK(&raw mut state_change_mutex) != 0 {
        return false;
    }

    if eventlog_enabled as i32 != 0 || !event_log_writer.is_null() {
        if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/eventlog/EventLog.c".as_ptr(),
                434,
            );
        }

        return false;
    }

    event_log_writer = ev_writer;

    let mut ret = startEventLogging_();
    eventlog_enabled = true;
    repostInitEvents();

    if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            442,
        );
    }

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
        let mut c: u32 = 0;

        while c < getNumCapabilities() as u32 {
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
    let mut __r = pthread_mutex_lock(&raw mut state_change_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            479,
            __r,
        );
    }

    if !eventlog_enabled {
        if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/eventlog/EventLog.c".as_ptr(),
                481,
            );
        }

        return;
    }

    eventlog_enabled = false;

    if getSchedState() as u32 != SCHED_SHUTTING_DOWN as i32 as u32 {
        flushEventLog(null_mut::<*mut Capability>());
    }

    let mut __r_0 = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r_0 != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            495,
            __r_0,
        );
    }

    postEventTypeNum(&raw mut eventBuf, EVENT_DATA_END as EventTypeNum);
    printAndClearEventBuf(&raw mut eventBuf);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            503,
        );
    }

    stopEventLogWriter();
    event_log_writer = null::<EventLogWriter>();

    if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            508,
        );
    }
}

unsafe fn moreCapEventBufs(mut from: u32, mut to: u32) {
    if from > 0 {
        capEventBuf = stgReallocBytes(
            capEventBuf as *mut c_void,
            (to as usize).wrapping_mul(size_of::<EventsBuf>() as usize),
            c"moreCapEventBufs".as_ptr(),
        ) as *mut EventsBuf;
    } else {
        capEventBuf = stgMallocBytes(
            (to as usize).wrapping_mul(size_of::<EventsBuf>() as usize),
            c"moreCapEventBufs".as_ptr(),
        ) as *mut EventsBuf;
    }

    let mut c: u32 = from;

    while c < to {
        initEventsBuf(
            capEventBuf.offset(c as isize) as *mut EventsBuf,
            EVENT_LOG_SIZE as StgWord64,
            c as EventCapNo,
        );

        c = c.wrapping_add(1);
    }

    if from > 0 {
        let mut c_0: u32 = from;

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

    match tag as i32 {
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
            barf(c"postSchedEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    };
}

unsafe fn postSparkEvent(mut cap: *mut Capability, mut tag: EventTypeNum, mut info1: StgWord) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag as i32 {
        EVENT_CREATE_SPARK_THREAD => {
            postThreadID(eb, info1 as EventThreadID);
        }
        EVENT_SPARK_STEAL => {
            postCapNo(eb, info1 as EventCapNo);
        }

        EVENT_SPARK_CREATE | EVENT_SPARK_DUD | EVENT_SPARK_OVERFLOW | EVENT_SPARK_RUN
        | EVENT_SPARK_FIZZLE | EVENT_SPARK_GC => {}
        _ => {
            barf(c"postSparkEvent: unknown event tag %d".as_ptr(), tag as i32);
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
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            666,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);

    match tag as i32 {
        EVENT_CAP_CREATE | EVENT_CAP_DELETE | EVENT_CAP_ENABLE | EVENT_CAP_DISABLE => {
            postCapNo(&raw mut eventBuf, capno);
        }
        _ => {
            barf(c"postCapEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            685,
        );
    }
}

unsafe fn postCapsetEvent(mut tag: EventTypeNum, mut capset: EventCapsetID, mut info: StgWord) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            692,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);
    postCapsetID(&raw mut eventBuf, capset);

    match tag as i32 {
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
                c"postCapsetEvent: unknown event tag %d".as_ptr(),
                tag as i32,
            );
        }
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            726,
        );
    }
}

unsafe fn postCapsetStrEvent(
    mut tag: EventTypeNum,
    mut capset: EventCapsetID,
    mut msg: *mut c_char,
) {
    let mut strsize = strlen(msg) as i32;
    let mut size = (strsize as usize).wrapping_add(size_of::<EventCapsetID>() as usize) as i32;

    if size > EVENT_PAYLOAD_SIZE_MAX {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            740,
            __r,
        );
    }

    if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
        printAndClearEventBuf(&raw mut eventBuf);

        if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());

            if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/eventlog/EventLog.c".as_ptr(),
                    747,
                );
            }

            return;
        }
    }

    postEventHeader(&raw mut eventBuf, tag);
    postPayloadSize(&raw mut eventBuf, size as EventPayloadSize);
    postCapsetID(&raw mut eventBuf, capset);
    postBuf(&raw mut eventBuf, msg as *mut StgWord8, strsize as u32);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            758,
        );
    }
}

unsafe fn postCapsetVecEvent(
    mut tag: EventTypeNum,
    mut capset: EventCapsetID,
    mut argc: i32,
    mut argv: *mut *mut c_char,
) {
    let mut size = size_of::<EventCapsetID>() as i32;
    let mut i = 0;

    while i < argc {
        let mut increment = (1 as usize).wrapping_add(strlen(*argv.offset(i as isize))) as i32;

        if size + increment > EVENT_PAYLOAD_SIZE_MAX {
            errorBelch(
                c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, record only %llu out of %llu args"
                    .as_ptr(),
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

    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            783,
            __r,
        );
    }

    if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
        printAndClearEventBuf(&raw mut eventBuf);

        if hasRoomForVariableEvent(&raw mut eventBuf, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());

            if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/eventlog/EventLog.c".as_ptr(),
                    790,
                );
            }

            return;
        }
    }

    postEventHeader(&raw mut eventBuf, tag);
    postPayloadSize(&raw mut eventBuf, size as EventPayloadSize);
    postCapsetID(&raw mut eventBuf, capset);

    let mut i_0 = 0;

    while i_0 < argc {
        postBuf(
            &raw mut eventBuf,
            *argv.offset(i_0 as isize) as *mut StgWord8,
            (1 as usize).wrapping_add(strlen(*argv.offset(i_0 as isize))) as u32,
        );

        i_0 += 1;
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            804,
        );
    }
}

unsafe fn postWallClockTime(mut capset: EventCapsetID) {
    let mut ts: StgWord64 = 0;
    let mut sec: StgWord64 = 0;
    let mut nsec: StgWord32 = 0;
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            813,
            __r,
        );
    }

    getUnixEpochTime(&raw mut sec, &raw mut nsec);
    ts = time_ns();
    ensureRoomForEvent(&raw mut eventBuf, EVENT_WALL_CLOCK_TIME as EventTypeNum);
    postEventTypeNum(&raw mut eventBuf, EVENT_WALL_CLOCK_TIME as EventTypeNum);
    postWord64(&raw mut eventBuf, ts);
    postCapsetID(&raw mut eventBuf, capset);
    postWord64(&raw mut eventBuf, sec);
    postWord32(&raw mut eventBuf, nsec);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            846,
        );
    }
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

    match tag as i32 {
        EVENT_HEAP_ALLOCATED | EVENT_HEAP_SIZE | EVENT_BLOCKS_SIZE | EVENT_HEAP_LIVE => {
            postCapsetID(eb, heap_capset);
            postWord64(eb, info1 as StgWord64);
        }
        _ => {
            barf(c"postHeapEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    };
}

unsafe fn postEventHeapInfo(
    mut heap_capset: EventCapsetID,
    mut gens: u32,
    mut maxHeapSize: W_,
    mut allocAreaSize: W_,
    mut mblockSize: W_,
    mut blockSize: W_,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            885,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EVENT_HEAP_INFO_GHC as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_HEAP_INFO_GHC as EventTypeNum);
    postCapsetID(&raw mut eventBuf, heap_capset);
    postWord16(&raw mut eventBuf, gens as StgWord16);
    postWord64(&raw mut eventBuf, maxHeapSize as StgWord64);
    postWord64(&raw mut eventBuf, allocAreaSize as StgWord64);
    postWord64(&raw mut eventBuf, mblockSize as StgWord64);
    postWord64(&raw mut eventBuf, blockSize as StgWord64);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            899,
        );
    }
}

unsafe fn postEventGcStats(
    mut cap: *mut Capability,
    mut heap_capset: EventCapsetID,
    mut r#gen: u32,
    mut copied: W_,
    mut slop: W_,
    mut fragmentation: W_,
    mut par_n_threads: u32,
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
    mut current_mblocks: u32,
    mut needed_mblocks: u32,
    mut returned_mblocks: u32,
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
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            952,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_CREATE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_CREATE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);
    postCapNo(&raw mut eventBuf, capno);
    postKernelThreadId(&raw mut eventBuf, tid);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            961,
        );
    }
}

unsafe fn postTaskMigrateEvent(
    mut taskId: EventTaskId,
    mut capno: EventCapNo,
    mut new_capno: EventCapNo,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            968,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_MIGRATE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_MIGRATE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);
    postCapNo(&raw mut eventBuf, capno);
    postCapNo(&raw mut eventBuf, new_capno);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            977,
        );
    }
}

unsafe fn postTaskDeleteEvent(mut taskId: EventTaskId) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            982,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EVENT_TASK_DELETE as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_TASK_DELETE as EventTypeNum);
    postTaskId(&raw mut eventBuf, taskId);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            989,
        );
    }
}

unsafe fn postEventNoCap(mut tag: EventTypeNum) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            995,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, tag);
    postEventHeader(&raw mut eventBuf, tag);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            998,
        );
    }
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

const BUF: i32 = 512;

unsafe fn postLogMsg(
    mut eb: *mut EventsBuf,
    mut r#type: EventTypeNum,
    mut msg: *mut c_char,
    mut ap: VaList,
) {
    let mut buf: [c_char; 512] = [0; 512];

    let mut size: u32 = vsnprintf(
        &raw mut buf as *mut c_char,
        BUF as usize,
        msg,
        ap.as_va_list(),
    ) as u32;

    if size > BUF as u32 {
        buf[(BUF - 1) as usize] = '\0' as i32 as c_char;
        size = BUF as u32;
    }

    ensureRoomForVariableEvent(eb, size as StgWord);
    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, &raw mut buf as *mut c_char as *mut StgWord8, size);
}

unsafe fn postMsg(mut msg: *mut c_char, mut ap: VaList) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1042,
            __r,
        );
    }

    postLogMsg(
        &raw mut eventBuf,
        EVENT_LOG_MSG as EventTypeNum,
        msg,
        ap.as_va_list(),
    );

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1044,
        );
    }
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
    let size = strlen(msg) as usize;

    if size > EVENT_PAYLOAD_SIZE_MAX as usize {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());
            return;
        }
    }

    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, msg as *mut StgWord8, size as u32);
}

unsafe fn postUserBinaryEvent(
    mut cap: *mut Capability,
    mut r#type: EventTypeNum,
    mut msg: *mut u8,
    mut size: usize,
) {
    if size > EVENT_PAYLOAD_SIZE_MAX as usize {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());
            return;
        }
    }

    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, msg as *mut StgWord8, size as u32);
}

unsafe fn postThreadLabel(
    mut cap: *mut Capability,
    mut id: EventThreadID,
    mut label: *mut c_char,
    mut len: usize,
) {
    let strsize = len as i32;
    let size = (strsize as usize).wrapping_add(size_of::<EventThreadID>() as usize) as i32;

    if size > EVENT_PAYLOAD_SIZE_MAX {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());
            return;
        }
    }

    postEventHeader(eb, EVENT_THREAD_LABEL as EventTypeNum);
    postPayloadSize(eb, size as EventPayloadSize);
    postThreadID(eb, id);
    postBuf(eb, label as *mut StgWord8, strsize as u32);
}

unsafe fn postConcUpdRemSetFlush(mut cap: *mut Capability) {
    let mut eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EVENT_CONC_UPD_REM_SET_FLUSH as EventTypeNum);
    postEventHeader(eb, EVENT_CONC_UPD_REM_SET_FLUSH as EventTypeNum);
    postCapNo(eb, (*cap).no as EventCapNo);
}

unsafe fn postConcMarkEnd(mut marked_obj_count: StgWord32) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1138,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EVENT_CONC_MARK_END as EventTypeNum);
    postEventHeader(&raw mut eventBuf, EVENT_CONC_MARK_END as EventTypeNum);
    postWord32(&raw mut eventBuf, marked_obj_count);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1142,
        );
    }
}

unsafe fn postNonmovingHeapCensus(mut blk_size: u16, mut census: *const NonmovingAllocCensus) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1148,
            __r,
        );
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_NONMOVING_HEAP_CENSUS as EventTypeNum,
    );
    postWord16(&raw mut eventBuf, blk_size as StgWord16);
    postWord32(&raw mut eventBuf, (*census).n_active_segs as StgWord32);
    postWord32(&raw mut eventBuf, (*census).n_filled_segs as StgWord32);
    postWord32(&raw mut eventBuf, (*census).n_live_blocks as StgWord32);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1154,
        );
    }
}

unsafe fn postNonmovingPrunedSegments(mut pruned_segments: u32, mut free_segments: u32) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1159,
            __r,
        );
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_NONMOVING_PRUNED_SEGMENTS as EventTypeNum,
    );
    postWord32(&raw mut eventBuf, pruned_segments as StgWord32);
    postWord32(&raw mut eventBuf, free_segments as StgWord32);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1163,
        );
    }
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
            save_pos.offset_from((*ebuf).marker) as i64 as StgWord32,
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
    postWord32(eb, 0);
    postWord64(eb, 0);
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
            barf(c"getHeapProfBreakdown: unknown heap profiling mode".as_ptr());
        }
    };
}

unsafe fn postHeapProfBegin(mut profile_id: StgWord8) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1224,
            __r,
        );
    }

    let mut flags: *mut PROFILING_FLAGS = &raw mut RtsFlags.ProfFlags;

    let mut modSelector_len: StgWord = (if !(*flags).modSelector.is_null() {
        strlen((*flags).modSelector)
    } else {
        0
    }) as StgWord;

    let mut descrSelector_len: StgWord = (if !(*flags).descrSelector.is_null() {
        strlen((*flags).descrSelector)
    } else {
        0
    }) as StgWord;

    let mut typeSelector_len: StgWord = (if !(*flags).typeSelector.is_null() {
        strlen((*flags).typeSelector)
    } else {
        0
    }) as StgWord;

    let mut ccSelector_len: StgWord = (if !(*flags).ccSelector.is_null() {
        strlen((*flags).ccSelector)
    } else {
        0
    }) as StgWord;

    let mut ccsSelector_len: StgWord = (if !(*flags).ccsSelector.is_null() {
        strlen((*flags).ccsSelector)
    } else {
        0
    }) as StgWord;

    let mut retainerSelector_len: StgWord = (if !(*flags).retainerSelector.is_null() {
        strlen((*flags).retainerSelector)
    } else {
        0
    }) as StgWord;

    let mut bioSelector_len: StgWord = (if !(*flags).bioSelector.is_null() {
        strlen((*flags).bioSelector)
    } else {
        0
    }) as StgWord;

    let mut len: StgWord = ((1 as i32 + 8 as i32 + 4 as i32) as StgWord)
        .wrapping_add(modSelector_len)
        .wrapping_add(descrSelector_len)
        .wrapping_add(typeSelector_len)
        .wrapping_add(ccSelector_len)
        .wrapping_add(ccsSelector_len)
        .wrapping_add(retainerSelector_len)
        .wrapping_add(bioSelector_len)
        .wrapping_add(7 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1244);
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

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1257,
        );
    }
}

unsafe fn postHeapProfSampleBegin(mut era: StgInt) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1262,
            __r,
        );
    }

    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_BEGIN as EventTypeNum,
    );
    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_BEGIN as EventTypeNum,
    );
    postWord64(&raw mut eventBuf, era as StgWord64);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1266,
        );
    }
}

unsafe fn postHeapBioProfSampleBegin(mut era: StgInt, mut time: StgWord64) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1272,
            __r,
        );
    }

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

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1277,
        );
    }
}

unsafe fn postHeapProfSampleEnd(mut era: StgInt) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1282,
            __r,
        );
    }

    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_END as EventTypeNum,
    );
    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_END as EventTypeNum,
    );
    postWord64(&raw mut eventBuf, era as StgWord64);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1286,
        );
    }
}

unsafe fn postHeapProfSampleString(
    mut profile_id: StgWord8,
    mut label: *const c_char,
    mut residency: StgWord64,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1293,
            __r,
        );
    }

    let mut label_len: StgWord = strlen(label) as StgWord;
    let mut len: StgWord = ((1 as i32 + 8 as i32) as StgWord)
        .wrapping_add(label_len)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1296);
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_STRING as EventTypeNum,
    );
    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord8(&raw mut eventBuf, profile_id);
    postWord64(&raw mut eventBuf, residency);
    postStringLen(&raw mut eventBuf, label, label_len);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1302,
        );
    }
}

unsafe fn postHeapProfCostCentre(
    mut ccID: StgWord32,
    mut label: *const c_char,
    mut module: *const c_char,
    mut srcloc: *const c_char,
    mut is_caf: StgBool,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1312,
            __r,
        );
    }

    let mut label_len: StgWord = strlen(label) as StgWord;
    let mut module_len: StgWord = strlen(module) as StgWord;
    let mut srcloc_len: StgWord = strlen(srcloc) as StgWord;
    let mut len: StgWord = (4 as StgWord)
        .wrapping_add(label_len)
        .wrapping_add(module_len)
        .wrapping_add(srcloc_len)
        .wrapping_add(3 as StgWord)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1317);
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_COST_CENTRE as EventTypeNum,
    );
    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord32(&raw mut eventBuf, ccID);
    postStringLen(&raw mut eventBuf, label, label_len);
    postStringLen(&raw mut eventBuf, module, module_len);
    postStringLen(&raw mut eventBuf, srcloc, srcloc_len);
    postWord8(&raw mut eventBuf, is_caf as StgWord8);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1325,
        );
    }
}

unsafe fn postHeapProfSampleCostCentre(
    mut profile_id: StgWord8,
    mut stack: *mut CostCentreStack,
    mut residency: StgWord64,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1332,
            __r,
        );
    }

    let mut depth: StgWord = 0;
    let mut ccs = null_mut::<CostCentreStack>();
    ccs = stack;

    while !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        depth = depth.wrapping_add(1);
        ccs = (*ccs).prevStack as *mut CostCentreStack;
    }

    if depth > 0xff {
        depth = 0xff;
    }

    let mut len: StgWord = ((1 as i32 + 8 as i32 + 1 as i32) as StgWord)
        .wrapping_add(depth.wrapping_mul(4 as StgWord));

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1340);
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_HEAP_PROF_SAMPLE_COST_CENTRE as EventTypeNum,
    );

    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord8(&raw mut eventBuf, profile_id);
    postWord64(&raw mut eventBuf, residency);
    postWord8(&raw mut eventBuf, depth as StgWord8);
    ccs = stack;

    while depth > 0 && !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        postWord32(&raw mut eventBuf, (*(*ccs).cc).ccID as StgWord32);
        ccs = (*ccs).prevStack as *mut CostCentreStack;
        depth = depth.wrapping_sub(1);
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1350,
        );
    }
}

unsafe fn postProfSampleCostCentre(
    mut cap: *mut Capability,
    mut stack: *mut CostCentreStack,
    mut tick: StgWord64,
) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1358,
            __r,
        );
    }

    let mut depth: StgWord = 0;
    let mut ccs = null_mut::<CostCentreStack>();
    ccs = stack;

    while !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        depth = depth.wrapping_add(1);
        ccs = (*ccs).prevStack as *mut CostCentreStack;
    }

    if depth > 0xff {
        depth = 0xff;
    }

    let mut len: StgWord = ((4 as i32 + 8 as i32 + 1 as i32) as StgWord)
        .wrapping_add(depth.wrapping_mul(4 as StgWord));

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1366);
    }

    postEventHeader(
        &raw mut eventBuf,
        EVENT_PROF_SAMPLE_COST_CENTRE as EventTypeNum,
    );
    postPayloadSize(&raw mut eventBuf, len as EventPayloadSize);
    postWord32(&raw mut eventBuf, (*cap).no as StgWord32);
    postWord64(&raw mut eventBuf, tick);
    postWord8(&raw mut eventBuf, depth as StgWord8);
    ccs = stack;

    while depth > 0 && !ccs.is_null() && ccs != &raw mut CCS_MAIN as *mut CostCentreStack {
        postWord32(&raw mut eventBuf, (*(*ccs).cc).ccID as StgWord32);
        ccs = (*ccs).prevStack as *mut CostCentreStack;
        depth = depth.wrapping_sub(1);
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1376,
        );
    }
}

unsafe fn postProfBegin() {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1384,
            __r,
        );
    }

    postEventHeader(&raw mut eventBuf, EVENT_PROF_BEGIN as EventTypeNum);
    postWord64(
        &raw mut eventBuf,
        RtsFlags.MiscFlags.tickInterval as StgWord64,
    );

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1388,
        );
    }
}

unsafe fn postTickyCounterDef(mut eb: *mut EventsBuf, mut p: *mut StgEntCounter) {
    let mut arg_kinds_len: StgWord = strlen((*p).arg_kinds) as StgWord;
    let mut str_len: StgWord = strlen((*p).str) as StgWord;
    let mut ticky_json_len: StgWord = strlen((*p).ticky_json) as StgWord;
    let mut len: StgWord = ((8 as i32 + 2 as i32) as StgWord)
        .wrapping_add(arg_kinds_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(str_len)
        .wrapping_add(1 as StgWord)
        .wrapping_add(8 as StgWord)
        .wrapping_add(ticky_json_len)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(eb, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1399);
    }

    postEventHeader(eb, EVENT_TICKY_COUNTER_DEF as EventTypeNum);
    postPayloadSize(eb, len as EventPayloadSize);
    postWord64(eb, p as usize as StgWord64);
    postWord16(eb, (*p).arity as StgWord16);
    postStringLen(eb, (*p).arg_kinds, arg_kinds_len);
    postStringLen(eb, (*p).str, str_len);
    postWord64(eb, INFO_PTR_TO_STRUCT((*p).info) as StgWord64);
    postStringLen(eb, (*p).ticky_json, ticky_json_len);
}

unsafe fn postTickyCounterDefs(mut counters: *mut StgEntCounter) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1414,
            __r,
        );
    }

    let mut p = counters;

    while !p.is_null() {
        postTickyCounterDef(&raw mut eventBuf, p);
        p = (*p).link as *mut StgEntCounter;
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1418,
        );
    }
}

unsafe fn postTickyCounterSample(mut eb: *mut EventsBuf, mut p: *mut StgEntCounter) {
    if (*p).entry_count == 0 && (*p).allocs == 0 && (*p).allocd == 0 {
        return;
    }

    ensureRoomForEvent(eb, EVENT_TICKY_COUNTER_SAMPLE as EventTypeNum);
    postEventHeader(eb, EVENT_TICKY_COUNTER_SAMPLE as EventTypeNum);
    postWord64(eb, p as usize as StgWord64);
    postWord64(eb, (*p).entry_count as StgWord64);
    postWord64(eb, (*p).allocs as StgWord64);
    postWord64(eb, (*p).allocd as StgWord64);
    (*p).entry_count = 0;
    (*p).allocs = 0;
    (*p).allocd = 0;
}

unsafe fn postTickyCounterSamples(mut counters: *mut StgEntCounter) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1442,
            __r,
        );
    }

    ensureRoomForEvent(
        &raw mut eventBuf,
        EVENT_TICKY_COUNTER_SAMPLE as EventTypeNum,
    );
    postEventHeader(
        &raw mut eventBuf,
        EVENT_TICKY_COUNTER_BEGIN_SAMPLE as EventTypeNum,
    );

    let mut p = counters;

    while !p.is_null() {
        postTickyCounterSample(&raw mut eventBuf, p);
        p = (*p).link as *mut StgEntCounter;
    }

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1448,
        );
    }
}

unsafe fn postIPE(mut ipe: *const InfoProvEnt) {
    let mut closure_desc_buf: [c_char; 11] = [0; 11];
    formatClosureDescIpe(ipe, &raw mut closure_desc_buf as *mut c_char);

    let MAX_IPE_STRING_LEN: StgWord = 65535;
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1458,
            __r,
        );
    }

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

    let mut extra_comma: StgWord = 1;
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

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1472);
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
        src_file_len as u32,
    );

    let mut colon: StgWord8 = ':' as i32 as StgWord8;
    postBuf(&raw mut eventBuf, &raw mut colon, 1);
    postStringLen(&raw mut eventBuf, (*ipe).prov.src_span, src_span_len);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1488,
        );
    }
}

unsafe fn printAndClearEventBuf(mut ebuf: *mut EventsBuf) {
    closeBlockMarker(ebuf);

    if !(*ebuf).begin.is_null() && (*ebuf).pos != (*ebuf).begin {
        let mut elog_size: usize = (*ebuf).pos.offset_from((*ebuf).begin) as i64 as usize;

        if !writeEventLog((*ebuf).begin as *mut c_void, elog_size) {
            debugBelch(c"printAndClearEventLog: could not flush event log\n".as_ptr());
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
    (*eb).pos = stgMallocBytes(size as usize, c"initEventsBuf".as_ptr()) as *mut StgInt8;
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
    let mut size: u32 = (size_of::<EventTypeNum>() as usize)
        .wrapping_add(size_of::<EventTimestamp>() as usize)
        .wrapping_add(eventTypes[eNum as usize].size as usize) as u32;

    if (*eb).pos.offset(size as isize) > (*eb).begin.offset((*eb).size as isize) {
        return 0;
    } else {
        return 1;
    };
}

unsafe fn hasRoomForVariableEvent(mut eb: *mut EventsBuf, mut payload_bytes: StgWord) -> StgBool {
    let mut size: StgWord = ((size_of::<EventTypeNum>() as usize)
        .wrapping_add(size_of::<EventTimestamp>() as usize)
        .wrapping_add(size_of::<EventPayloadSize>() as usize)
        as StgWord)
        .wrapping_add(payload_bytes);

    if (*eb).pos.offset(size as isize) > (*eb).begin.offset((*eb).size as isize) {
        return 0;
    } else {
        return 1;
    };
}

unsafe fn ensureRoomForEvent(mut eb: *mut EventsBuf, mut tag: EventTypeNum) {
    if hasRoomForEvent(eb, tag) == 0 {
        printAndClearEventBuf(eb);

        if (hasRoomForEvent(eb, tag) != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1559);
        }
    }
}

unsafe fn ensureRoomForVariableEvent(mut eb: *mut EventsBuf, mut size: StgWord) -> i32 {
    if hasRoomForVariableEvent(eb, size) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size) == 0 {
            return 1;
        }
    }

    return 0;
}

unsafe fn postEventType(mut eb: *mut EventsBuf, mut et: *mut EventType) {
    postInt32(eb, EVENT_ET_BEGIN as StgInt32);
    postEventTypeNum(eb, (*et).etNum);
    postWord16(eb, (*et).size as StgWord16);

    let desclen = strlen((*et).desc) as i32;
    postWord32(eb, desclen as StgWord32);

    let mut d = 0;

    while d < desclen {
        postInt8(eb, *(*et).desc.offset(d as isize) as StgInt8);
        d += 1;
    }

    postWord32(eb, 0);
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

    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1605,
            __r,
        );
    }

    printAndClearEventBuf(&raw mut eventBuf);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1607,
        );
    }

    let mut i = 0;

    while i < getNumCapabilities() {
        flushLocalEventsBuf(getCapability(i as u32));
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

    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1621,
            __r,
        );
    }

    printAndClearEventBuf(&raw mut eventBuf);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1623,
        );
    }

    let mut task = getMyTask();
    stopAllCapabilitiesWith(cap, task, SYNC_FLUSH_EVENT_LOG);
    flushAllCapsEventsBufs();

    releaseAllCapabilities(
        getNumCapabilities() as u32,
        if !cap.is_null() {
            *cap
        } else {
            null_mut::<Capability>()
        },
        task,
    );

    flushEventLogWriter();
}

unsafe fn run_static_initializers() {
    eventTypes = [
        _EventType {
            etNum: 0,
            size: size_of::<EventThreadID>() as u32,
            desc: c"Create thread".as_ptr(),
        },
        _EventType {
            etNum: 1,
            size: size_of::<EventThreadID>() as u32,
            desc: c"Run thread".as_ptr(),
        },
        _EventType {
            etNum: 2,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<EventThreadID>() as usize) as u32,
            desc: c"Stop thread".as_ptr(),
        },
        _EventType {
            etNum: 3,
            size: size_of::<EventThreadID>() as u32,
            desc: c"Thread runnable".as_ptr(),
        },
        _EventType {
            etNum: 4,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Migrate thread".as_ptr(),
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
            etNum: 8,
            size: (size_of::<EventThreadID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Wakeup thread".as_ptr(),
        },
        _EventType {
            etNum: 9,
            size: 0,
            desc: c"Starting GC".as_ptr(),
        },
        _EventType {
            etNum: 10,
            size: 0,
            desc: c"Finished GC".as_ptr(),
        },
        _EventType {
            etNum: 11,
            size: 0,
            desc: c"Request sequential GC".as_ptr(),
        },
        _EventType {
            etNum: 12,
            size: 0,
            desc: c"Request parallel GC".as_ptr(),
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
            etNum: 15,
            size: size_of::<EventThreadID>() as u32,
            desc: c"Create spark thread".as_ptr(),
        },
        _EventType {
            etNum: 16,
            size: 0xffff,
            desc: c"Log message".as_ptr(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 18,
            size: (size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<EventTimestamp>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Block marker".as_ptr(),
        },
        _EventType {
            etNum: 19,
            size: 0xffff,
            desc: c"User message".as_ptr(),
        },
        _EventType {
            etNum: 20,
            size: 0,
            desc: c"GC idle".as_ptr(),
        },
        _EventType {
            etNum: 21,
            size: 0,
            desc: c"GC working".as_ptr(),
        },
        _EventType {
            etNum: 22,
            size: 0,
            desc: c"GC done".as_ptr(),
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
            etNum: 25,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapsetType>() as usize) as u32,
            desc: c"Create capability set".as_ptr(),
        },
        _EventType {
            etNum: 26,
            size: size_of::<EventCapsetID>() as u32,
            desc: c"Delete capability set".as_ptr(),
        },
        _EventType {
            etNum: 27,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Add capability to capability set".as_ptr(),
        },
        _EventType {
            etNum: 28,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Remove capability from capability set".as_ptr(),
        },
        _EventType {
            etNum: 29,
            size: 0xffff,
            desc: c"RTS name and version".as_ptr(),
        },
        _EventType {
            etNum: 30,
            size: 0xffff,
            desc: c"Program arguments".as_ptr(),
        },
        _EventType {
            etNum: 31,
            size: 0xffff,
            desc: c"Program environment variables".as_ptr(),
        },
        _EventType {
            etNum: 32,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as u32,
            desc: c"Process ID".as_ptr(),
        },
        _EventType {
            etNum: 33,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as u32,
            desc: c"Parent process ID".as_ptr(),
        },
        _EventType {
            etNum: 34,
            size: (size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Spark counters".as_ptr(),
        },
        _EventType {
            etNum: 35,
            size: 0,
            desc: c"Spark create".as_ptr(),
        },
        _EventType {
            etNum: 36,
            size: 0,
            desc: c"Spark dud".as_ptr(),
        },
        _EventType {
            etNum: 37,
            size: 0,
            desc: c"Spark overflow".as_ptr(),
        },
        _EventType {
            etNum: 38,
            size: 0,
            desc: c"Spark run".as_ptr(),
        },
        _EventType {
            etNum: 39,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Spark steal".as_ptr(),
        },
        _EventType {
            etNum: 40,
            size: 0,
            desc: c"Spark fizzle".as_ptr(),
        },
        _EventType {
            etNum: 41,
            size: 0,
            desc: c"Spark GC".as_ptr(),
        },
        _EventType {
            etNum: 42,
            size: 0xffff,
            desc: c"Intern string".as_ptr(),
        },
        _EventType {
            etNum: 43,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as u32,
            desc: c"Wall clock time".as_ptr(),
        },
        _EventType {
            etNum: 44,
            size: 0xffff,
            desc: c"Thread label".as_ptr(),
        },
        _EventType {
            etNum: 45,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Create capability".as_ptr(),
        },
        _EventType {
            etNum: 46,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Delete capability".as_ptr(),
        },
        _EventType {
            etNum: 47,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Disable capability".as_ptr(),
        },
        _EventType {
            etNum: 48,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Enable capability".as_ptr(),
        },
        _EventType {
            etNum: 49,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Total heap memory ever allocated".as_ptr(),
        },
        _EventType {
            etNum: 50,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Current heap size (number of allocated mblocks)".as_ptr(),
        },
        _EventType {
            etNum: 51,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Current heap live data".as_ptr(),
        },
        _EventType {
            etNum: 52,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Heap static parameters".as_ptr(),
        },
        _EventType {
            etNum: 53,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"GC statistics".as_ptr(),
        },
        _EventType {
            etNum: 54,
            size: 0,
            desc: c"Synchronise stop-the-world GC".as_ptr(),
        },
        _EventType {
            etNum: 55,
            size: (size_of::<EventTaskId>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize)
                .wrapping_add(size_of::<EventKernelThreadId>() as usize) as u32,
            desc: c"Task create".as_ptr(),
        },
        _EventType {
            etNum: 56,
            size: (size_of::<EventTaskId>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize)
                .wrapping_add(size_of::<EventCapNo>() as usize) as u32,
            desc: c"Task migrate".as_ptr(),
        },
        _EventType {
            etNum: 57,
            size: size_of::<EventTaskId>() as u32,
            desc: c"Task delete".as_ptr(),
        },
        _EventType {
            etNum: 58,
            size: 0xffff,
            desc: c"User marker".as_ptr(),
        },
        _EventType {
            etNum: 59,
            size: 0,
            desc: c"Empty event for bug #9003".as_ptr(),
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
            etNum: 90,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as u32,
            desc: c"The RTS attempted to return heap memory to the OS".as_ptr(),
        },
        _EventType {
            etNum: 91,
            size: (size_of::<EventCapsetID>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Report the size of the heap in blocks".as_ptr(),
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
            etNum: 160,
            size: 0xffff,
            desc: c"Start of heap profile".as_ptr(),
        },
        _EventType {
            etNum: 161,
            size: 0xffff,
            desc: c"Cost-centre definition".as_ptr(),
        },
        _EventType {
            etNum: 162,
            size: size_of::<StgWord64>() as u32,
            desc: c"Start of heap profile sample".as_ptr(),
        },
        _EventType {
            etNum: 163,
            size: 0xffff,
            desc: c"Heap profile cost-centre sample".as_ptr(),
        },
        _EventType {
            etNum: 164,
            size: 0xffff,
            desc: c"Heap profile string sample".as_ptr(),
        },
        _EventType {
            etNum: 165,
            size: size_of::<StgWord64>() as u32,
            desc: c"End of heap profile sample".as_ptr(),
        },
        _EventType {
            etNum: 166,
            size: (size_of::<StgWord64>() as usize).wrapping_add(size_of::<StgWord64>() as usize)
                as u32,
            desc: c"Start of heap profile (biographical) sample".as_ptr(),
        },
        _EventType {
            etNum: 167,
            size: 0xffff,
            desc: c"Time profile cost-centre stack".as_ptr(),
        },
        _EventType {
            etNum: 168,
            size: size_of::<StgWord64>() as u32,
            desc: c"Start of a time profile".as_ptr(),
        },
        _EventType {
            etNum: 169,
            size: 0xffff,
            desc: c"An IPE entry".as_ptr(),
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
            etNum: 181,
            size: 0xffff,
            desc: c"User binary message".as_ptr(),
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
            etNum: 200,
            size: 0,
            desc: c"Begin concurrent mark phase".as_ptr(),
        },
        _EventType {
            etNum: 201,
            size: size_of::<StgWord32>() as u32,
            desc: c"End concurrent mark phase".as_ptr(),
        },
        _EventType {
            etNum: 202,
            size: 0,
            desc: c"Begin concurrent GC synchronisation".as_ptr(),
        },
        _EventType {
            etNum: 203,
            size: 0,
            desc: c"End concurrent mark synchronisation".as_ptr(),
        },
        _EventType {
            etNum: 204,
            size: 0,
            desc: c"Begin concurrent sweep phase".as_ptr(),
        },
        _EventType {
            etNum: 205,
            size: 0,
            desc: c"End concurrent sweep phase".as_ptr(),
        },
        _EventType {
            etNum: 206,
            size: size_of::<EventCapNo>() as u32,
            desc: c"Update remembered set flushed".as_ptr(),
        },
        _EventType {
            etNum: 207,
            size: (size_of::<StgWord16>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize)
                .wrapping_add(size_of::<StgWord32>() as usize) as u32,
            desc: c"Nonmoving heap census".as_ptr(),
        },
        _EventType {
            etNum: 208,
            size: (size_of::<StgWord32>() as usize).wrapping_add(size_of::<StgWord32>() as usize)
                as u32,
            desc: c"Report the amount of segments pruned and remaining on the free list.".as_ptr(),
        },
        _EventType {
            etNum: 0,
            size: 0,
            desc: null_mut::<c_char>(),
        },
        _EventType {
            etNum: 210,
            size: 0xffff,
            desc: c"Ticky-ticky entry counter definition".as_ptr(),
        },
        _EventType {
            etNum: 211,
            size: (size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<StgWord64>() as usize) as u32,
            desc: c"Ticky-ticky entry counter sample".as_ptr(),
        },
        _EventType {
            etNum: 212,
            size: 0,
            desc: c"Ticky-ticky entry counter begin sample".as_ptr(),
        },
    ];
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
