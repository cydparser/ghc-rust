use std::ffi::VaList;

use crate::capability::Capability;
use crate::capability::{SYNC_FLUSH_EVENT_LOG, getCapability};
use crate::event_log::format::{
    EVENT_DATA_BEGIN, EVENT_ET_BEGIN, EVENT_HEADER_BEGIN, EVENT_HEADER_END, EVENT_HET_BEGIN,
    EVENT_HET_END, EVENT_PAYLOAD_SIZE_MAX, EventCapNo, EventCapsetID, EventCapsetType,
    EventKernelThreadId, EventPayloadSize, EventTaskId, EventThreadID, EventTimestamp,
    HeapProfBreakdown,
};
use crate::event_log::types::{EventType, EventTypeNum, eventTypes};
use crate::event_log::writer::EventLogWriter;
use crate::ffi::rts::ipe::{InfoProvEnt, formatClosureDescIpe};
use crate::ffi::rts::os_threads::{Mutex, OS_TRY_ACQUIRE_LOCK, initMutex};
use crate::ffi::rts::prof::ccs::{CCS_MAIN, CostCentreStack};
use crate::ffi::rts::storage::closure_macros::INFO_PTR_TO_STRUCT;
use crate::ffi::rts::storage::tso::StgThreadID;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::ticky::StgEntCounter;
use crate::ffi::stg::W_;
use crate::get_time::getUnixEpochTime;
use crate::prelude::*;
use crate::rts_flags::{PROFILING_FLAGS, RtsFlags};
use crate::rts_messages::{_assertFail, barf, debugBelch, errorBelch};
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::schedule::{
    SCHED_SHUTTING_DOWN, getSchedState, releaseAllCapabilities, stopAllCapabilitiesWith,
};
use crate::sm::non_moving_census::NonmovingAllocCensus;
use crate::sparks::SparkCounters;
use crate::stats::stat_getElapsedTime;
use crate::stg::types::{
    StgBool, StgInt, StgInt8, StgInt32, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::task::getMyTask;

pub(crate) mod format;
pub(crate) mod types;
pub(crate) mod writer;

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

static mut state_change_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static eventlog_enabled: AtomicBool = AtomicBool::new(false);

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
unsafe fn postWord8(eb: *mut EventsBuf, i: StgWord8) {
    let fresh0 = (*eb).pos;
    (*eb).pos = (*eb).pos.offset(1);
    *fresh0 = i as StgInt8;
}

#[inline]
unsafe fn postWord16(eb: *mut EventsBuf, i: StgWord16) {
    postWord8(eb, (i as i32 >> 8) as StgWord8);
    postWord8(eb, i as StgWord8);
}

#[inline]
unsafe fn postWord32(eb: *mut EventsBuf, i: StgWord32) {
    postWord16(eb, (i >> 16) as StgWord16);
    postWord16(eb, i as StgWord16);
}

#[inline]
unsafe fn postWord64(eb: *mut EventsBuf, i: StgWord64) {
    postWord32(eb, (i >> 32) as StgWord32);
    postWord32(eb, i as StgWord32);
}

#[inline]
unsafe fn postBuf(eb: *mut EventsBuf, buf: *const StgWord8, size: u32) {
    memcpy(
        (*eb).pos as *mut c_void,
        buf as *const c_void,
        size as usize,
    );
    (*eb).pos = (*eb).pos.offset(size as isize);
}

#[inline]
unsafe fn postStringLen(eb: *mut EventsBuf, buf: *const c_char, len: StgWord) {
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
unsafe fn postEventTypeNum(eb: *mut EventsBuf, etNum: EventTypeNum) {
    postWord16(eb, etNum as StgWord16);
}

#[inline]
unsafe fn postTimestamp(eb: *mut EventsBuf) {
    postWord64(eb, time_ns());
}

#[inline]
unsafe fn postThreadID(eb: *mut EventsBuf, id: EventThreadID) {
    postWord32(eb, id as StgWord32);
}

#[inline]
unsafe fn postCapNo(eb: *mut EventsBuf, no: EventCapNo) {
    postWord16(eb, no as StgWord16);
}

#[inline]
unsafe fn postCapsetID(eb: *mut EventsBuf, id: EventCapsetID) {
    postWord32(eb, id as StgWord32);
}

#[inline]
unsafe fn postCapsetType(eb: *mut EventsBuf, r#type: EventCapsetType) {
    postWord16(eb, r#type as StgWord16);
}

#[inline]
unsafe fn postOSProcessId(eb: *mut EventsBuf, pid: pid_t) {
    postWord32(eb, pid as StgWord32);
}

#[inline]
unsafe fn postKernelThreadId(eb: *mut EventsBuf, tid: EventKernelThreadId) {
    postWord64(eb, tid as StgWord64);
}

#[inline]
unsafe fn postTaskId(eb: *mut EventsBuf, tUniq: EventTaskId) {
    postWord64(eb, tUniq as StgWord64);
}

#[inline]
unsafe fn postPayloadSize(eb: *mut EventsBuf, size: EventPayloadSize) {
    postWord16(eb, size as StgWord16);
}

#[inline]
unsafe fn postEventHeader(eb: *mut EventsBuf, r#type: EventTypeNum) {
    postEventTypeNum(eb, r#type);
    postTimestamp(eb);
}

#[inline]
unsafe fn postInt8(eb: *mut EventsBuf, i: StgInt8) {
    postWord8(eb, i as StgWord8);
}

#[inline]
unsafe fn postInt32(eb: *mut EventsBuf, i: StgInt32) {
    postWord32(eb, i as StgWord32);
}

unsafe fn initEventLogWriter() {
    if !event_log_writer.is_null() && (*event_log_writer).initEventLogWriter.is_some() {
        (*event_log_writer)
            .initEventLogWriter
            .expect("non-null function pointer")();
    }
}

unsafe fn writeEventLog(eventlog: *mut c_void, eventlog_size: usize) -> bool {
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

    for event_type in eventTypes.iter() {
        let Some(event_type) = event_type else {
            continue;
        };
        postEventType(&raw mut eventBuf, event_type);
    }

    postInt32(&raw mut eventBuf, EVENT_HET_END as StgInt32);
    postInt32(&raw mut eventBuf, EVENT_HEADER_END as StgInt32);
    postInt32(&raw mut eventBuf, EVENT_DATA_BEGIN as StgInt32);
}

unsafe fn postInitEvent(post_init: EventlogInitPost) {
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

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum EventLogStatus {
    EVENTLOG_NOT_SUPPORTED = 0,
    EVENTLOG_NOT_CONFIGURED = 1,
    EVENTLOG_RUNNING = 2,
}

pub(crate) unsafe fn eventLogStatus() -> EventLogStatus {
    if eventlog_enabled.load(Ordering::Relaxed) {
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

    if !eventlog_enabled.load(Ordering::Relaxed) || !event_log_writer.is_null() {
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
    eventlog_enabled.store(true, Ordering::Relaxed);
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
    if eventlog_enabled.load(Ordering::Relaxed) {
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

    if !eventlog_enabled.load(Ordering::Relaxed) {
        if pthread_mutex_unlock(&raw mut state_change_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/eventlog/EventLog.c".as_ptr(),
                481,
            );
        }

        return;
    }

    eventlog_enabled.store(false, Ordering::Relaxed);

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

    postEventTypeNum(&raw mut eventBuf, EventTypeNum::DATA_END);
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
    tag: EventTypeNum,
    mut thread: StgThreadID,
    mut info1: StgWord,
    mut info2: StgWord,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag {
        EventTypeNum::CREATE_THREAD | EventTypeNum::RUN_THREAD | EventTypeNum::THREAD_RUNNABLE => {
            postThreadID(eb, thread as EventThreadID);
        }
        EventTypeNum::CREATE_SPARK_THREAD => {
            postThreadID(eb, info1 as EventThreadID);
        }
        EventTypeNum::MIGRATE_THREAD | EventTypeNum::THREAD_WAKEUP => {
            postThreadID(eb, thread as EventThreadID);
            postCapNo(eb, info1 as EventCapNo);
        }
        EventTypeNum::STOP_THREAD => {
            postThreadID(eb, thread as EventThreadID);
            postWord16(eb, info1 as StgWord16);
            postThreadID(eb, info2 as EventThreadID);
        }
        _ => {
            barf(c"postSchedEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    };
}

unsafe fn postSparkEvent(cap: *mut Capability, tag: EventTypeNum, info1: StgWord) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag {
        EventTypeNum::CREATE_SPARK_THREAD => {
            postThreadID(eb, info1 as EventThreadID);
        }
        EventTypeNum::SPARK_STEAL => {
            postCapNo(eb, info1 as EventCapNo);
        }

        EventTypeNum::SPARK_CREATE
        | EventTypeNum::SPARK_DUD
        | EventTypeNum::SPARK_OVERFLOW
        | EventTypeNum::SPARK_RUN
        | EventTypeNum::SPARK_FIZZLE
        | EventTypeNum::SPARK_GC => {}
        _ => {
            barf(c"postSparkEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    };
}

unsafe fn postSparkCountersEvent(
    cap: *mut Capability,
    counters: SparkCounters,
    remaining: StgWord,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EventTypeNum::SPARK_COUNTERS);
    postEventHeader(eb, EventTypeNum::SPARK_COUNTERS);
    postWord64(eb, counters.created as StgWord64);
    postWord64(eb, counters.dud as StgWord64);
    postWord64(eb, counters.overflowed as StgWord64);
    postWord64(eb, counters.converted as StgWord64);
    postWord64(eb, counters.gcd as StgWord64);
    postWord64(eb, counters.fizzled as StgWord64);
    postWord64(eb, remaining as StgWord64);
}

unsafe fn postCapEvent(tag: EventTypeNum, capno: EventCapNo) {
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

    match tag {
        EventTypeNum::CAP_CREATE
        | EventTypeNum::CAP_DELETE
        | EventTypeNum::CAP_ENABLE
        | EventTypeNum::CAP_DISABLE => {
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

unsafe fn postCapsetEvent(tag: EventTypeNum, capset: EventCapsetID, info: StgWord) {
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

    match tag {
        EventTypeNum::CAPSET_CREATE => {
            postCapsetType(&raw mut eventBuf, info as EventCapsetType);
        }
        EventTypeNum::CAPSET_DELETE => {}
        EventTypeNum::CAPSET_ASSIGN_CAP | EventTypeNum::CAPSET_REMOVE_CAP => {
            postCapNo(&raw mut eventBuf, info as EventCapNo);
        }
        EventTypeNum::OSPROCESS_PID | EventTypeNum::OSPROCESS_PPID => {
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

unsafe fn postCapsetStrEvent(tag: EventTypeNum, capset: EventCapsetID, msg: *const c_char) {
    let mut strsize = libc::strlen(msg) as i32;
    let mut size = (strsize as usize).wrapping_add(size_of::<EventCapsetID>() as usize);

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
    tag: EventTypeNum,
    capset: EventCapsetID,
    argc: i32,
    argv: *mut *mut c_char,
) {
    let size = size_of::<EventCapsetID>();
    let mut i = 0;

    while i < argc {
        let mut increment = (1 as usize).wrapping_add(libc::strlen(*argv.offset(i as isize)));

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

    for i in 0..(argc as isize) {
        let buf = *argv.offset(i);

        postBuf(
            &raw mut eventBuf,
            buf.cast(),
            (libc::strlen(buf) + 1) as u32,
        );
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
    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::WALL_CLOCK_TIME);
    postEventTypeNum(&raw mut eventBuf, EventTypeNum::WALL_CLOCK_TIME);
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
    cap: *mut Capability,
    tag: EventTypeNum,
    heap_capset: EventCapsetID,
    info1: W_,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);

    match tag {
        EventTypeNum::HEAP_ALLOCATED
        | EventTypeNum::HEAP_SIZE
        | EventTypeNum::BLOCKS_SIZE
        | EventTypeNum::HEAP_LIVE => {
            postCapsetID(eb, heap_capset);
            postWord64(eb, info1 as StgWord64);
        }
        _ => {
            barf(c"postHeapEvent: unknown event tag %d".as_ptr(), tag as i32);
        }
    };
}

unsafe fn postEventHeapInfo(
    heap_capset: EventCapsetID,
    gens: u32,
    maxHeapSize: W_,
    allocAreaSize: W_,
    mblockSize: W_,
    blockSize: W_,
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::HEAP_INFO_GHC);
    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_INFO_GHC);
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
    cap: *mut Capability,
    heap_capset: EventCapsetID,
    r#gen: u32,
    copied: W_,
    slop: W_,
    fragmentation: W_,
    par_n_threads: u32,
    par_max_copied: W_,
    par_tot_copied: W_,
    par_balanced_copied: W_,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EventTypeNum::GC_STATS_GHC);
    postEventHeader(eb, EventTypeNum::GC_STATS_GHC);
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
    cap: *mut Capability,
    heap_capset: EventCapsetID,
    current_mblocks: u32,
    needed_mblocks: u32,
    returned_mblocks: u32,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EventTypeNum::MEM_RETURN);
    postEventHeader(eb, EventTypeNum::MEM_RETURN);
    postCapsetID(eb, heap_capset);
    postWord32(eb, current_mblocks as StgWord32);
    postWord32(eb, needed_mblocks as StgWord32);
    postWord32(eb, returned_mblocks as StgWord32);
}

unsafe fn postTaskCreateEvent(taskId: EventTaskId, capno: EventCapNo, tid: EventKernelThreadId) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            952,
            __r,
        );
    }

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::TASK_CREATE);
    postEventHeader(&raw mut eventBuf, EventTypeNum::TASK_CREATE);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::TASK_MIGRATE);
    postEventHeader(&raw mut eventBuf, EventTypeNum::TASK_MIGRATE);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::TASK_DELETE);
    postEventHeader(&raw mut eventBuf, EventTypeNum::TASK_DELETE);
    postTaskId(&raw mut eventBuf, taskId);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            989,
        );
    }
}

unsafe fn postEventNoCap(tag: EventTypeNum) {
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

unsafe fn postEvent(mut cap: *mut Capability, tag: EventTypeNum) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventHeader(eb, tag);
}

unsafe fn postEventAtTimestamp(
    mut cap: *mut Capability,
    mut ts: EventTimestamp,
    tag: EventTypeNum,
) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, tag);
    postEventTypeNum(eb, tag);
    postWord64(eb, ts as StgWord64);
}

const BUF: usize = 512;

unsafe fn postLogMsg(eb: *mut EventsBuf, r#type: EventTypeNum, msg: *const c_char, ap: VaList) {
    let mut buf: [c_char; BUF] = [0; _];

    let mut size = vsnprintf(&raw mut buf as *mut c_char, BUF, msg, ap);

    if size > BUF {
        buf[(BUF - 1) as usize] = '\0' as i32 as c_char;
        size = BUF as u32;
    }

    ensureRoomForVariableEvent(eb, size as StgWord);
    postEventHeader(eb, r#type);
    postPayloadSize(eb, size as EventPayloadSize);
    postBuf(eb, &raw mut buf as *mut c_char as *mut StgWord8, size);
}

unsafe fn postMsg(mut msg: *const c_char, mut ap: VaList) {
    let mut __r = pthread_mutex_lock(&raw mut eventBufMutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1042,
            __r,
        );
    }

    postLogMsg(&raw mut eventBuf, EventTypeNum::LOG_MSG, msg, ap);

    if pthread_mutex_unlock(&raw mut eventBufMutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/eventlog/EventLog.c".as_ptr(),
            1044,
        );
    }
}

unsafe fn postCapMsg(cap: *mut Capability, msg: *const c_char, ap: VaList) {
    postLogMsg(
        capEventBuf.offset((*cap).no as isize) as *mut EventsBuf,
        EventTypeNum::LOG_MSG,
        msg,
        ap,
    );
}

unsafe fn postUserEvent(cap: *const Capability, r#type: EventTypeNum, msg: *const c_char) {
    let size = libc::strlen(msg) as usize;

    if size > EVENT_PAYLOAD_SIZE_MAX as usize {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

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
    cap: *mut Capability,
    r#type: EventTypeNum,
    msg: *mut u8,
    size: usize,
) {
    if size > EVENT_PAYLOAD_SIZE_MAX as usize {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

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

unsafe fn postThreadLabel(cap: *mut Capability, id: EventThreadID, label: *mut c_char, len: usize) {
    let strsize = len;
    let size = strsize + size_of::<EventThreadID>();

    if size > EVENT_PAYLOAD_SIZE_MAX {
        errorBelch(c"Event size exceeds EVENT_PAYLOAD_SIZE_MAX, bail out".as_ptr());
        return;
    }

    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;

    if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size as StgWord) == 0 {
            errorBelch(c"Event size exceeds buffer size, bail out".as_ptr());
            return;
        }
    }

    postEventHeader(eb, EventTypeNum::THREAD_LABEL);
    postPayloadSize(eb, size as EventPayloadSize);
    postThreadID(eb, id);
    postBuf(eb, label as *mut StgWord8, strsize as u32);
}

unsafe fn postConcUpdRemSetFlush(mut cap: *mut Capability) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    ensureRoomForEvent(eb, EventTypeNum::CONC_UPD_REM_SET_FLUSH);
    postEventHeader(eb, EventTypeNum::CONC_UPD_REM_SET_FLUSH);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::CONC_MARK_END);
    postEventHeader(&raw mut eventBuf, EventTypeNum::CONC_MARK_END);
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::NONMOVING_HEAP_CENSUS);
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::NONMOVING_PRUNED_SEGMENTS);
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

unsafe fn postBlockMarker(eb: *mut EventsBuf) {
    ensureRoomForEvent(eb, EventTypeNum::BLOCK_MARKER);
    closeBlockMarker(eb);
    (*eb).marker = (*eb).pos;
    postEventHeader(eb, EventTypeNum::BLOCK_MARKER);
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
        libc::strlen((*flags).modSelector)
    } else {
        0
    }) as StgWord;

    let mut descrSelector_len: StgWord = (if !(*flags).descrSelector.is_null() {
        libc::strlen((*flags).descrSelector)
    } else {
        0
    }) as StgWord;

    let mut typeSelector_len: StgWord = (if !(*flags).typeSelector.is_null() {
        libc::strlen((*flags).typeSelector)
    } else {
        0
    }) as StgWord;

    let mut ccSelector_len: StgWord = (if !(*flags).ccSelector.is_null() {
        libc::strlen((*flags).ccSelector)
    } else {
        0
    }) as StgWord;

    let mut ccsSelector_len: StgWord = (if !(*flags).ccsSelector.is_null() {
        libc::strlen((*flags).ccsSelector)
    } else {
        0
    }) as StgWord;

    let mut retainerSelector_len: StgWord = (if !(*flags).retainerSelector.is_null() {
        libc::strlen((*flags).retainerSelector)
    } else {
        0
    }) as StgWord;

    let mut bioSelector_len: StgWord = (if !(*flags).bioSelector.is_null() {
        libc::strlen((*flags).bioSelector)
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_PROF_BEGIN);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::HEAP_PROF_SAMPLE_BEGIN);
    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_PROF_SAMPLE_BEGIN);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::HEAP_BIO_PROF_SAMPLE_BEGIN);

    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_BIO_PROF_SAMPLE_BEGIN);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::HEAP_PROF_SAMPLE_END);
    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_PROF_SAMPLE_END);
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

    let mut label_len: StgWord = libc::strlen(label) as StgWord;
    let mut len: StgWord = ((1 as i32 + 8 as i32) as StgWord)
        .wrapping_add(label_len)
        .wrapping_add(1 as StgWord);

    if (ensureRoomForVariableEvent(&raw mut eventBuf, len) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1296);
    }

    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_PROF_SAMPLE_STRING);
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

    let mut label_len: StgWord = libc::strlen(label) as StgWord;
    let mut module_len: StgWord = libc::strlen(module) as StgWord;
    let mut srcloc_len: StgWord = libc::strlen(srcloc) as StgWord;
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::HEAP_PROF_COST_CENTRE);
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
        EventTypeNum::HEAP_PROF_SAMPLE_COST_CENTRE,
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::PROF_SAMPLE_COST_CENTRE);
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::PROF_BEGIN);
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

unsafe fn postTickyCounterDef(eb: *mut EventsBuf, mut p: *mut StgEntCounter) {
    let mut arg_kinds_len: StgWord = libc::strlen((*p).arg_kinds) as StgWord;
    let mut str_len: StgWord = libc::strlen((*p).str) as StgWord;
    let mut ticky_json_len: StgWord = libc::strlen((*p).ticky_json) as StgWord;
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

    postEventHeader(eb, EventTypeNum::TICKY_COUNTER_DEF);
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

unsafe fn postTickyCounterSample(eb: *mut EventsBuf, mut p: *mut StgEntCounter) {
    if (*p).entry_count == 0 && (*p).allocs == 0 && (*p).allocd == 0 {
        return;
    }

    ensureRoomForEvent(eb, EventTypeNum::TICKY_COUNTER_SAMPLE);
    postEventHeader(eb, EventTypeNum::TICKY_COUNTER_SAMPLE);
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

    ensureRoomForEvent(&raw mut eventBuf, EventTypeNum::TICKY_COUNTER_SAMPLE);
    postEventHeader(&raw mut eventBuf, EventTypeNum::TICKY_COUNTER_BEGIN_SAMPLE);

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
        if (libc::strlen((*ipe).prov.table_name) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.table_name) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut closure_desc_len: StgWord =
        if (libc::strlen(&raw mut closure_desc_buf as *mut c_char) as StgWord) < MAX_IPE_STRING_LEN
        {
            libc::strlen(&raw mut closure_desc_buf as *mut c_char) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut ty_desc_len: StgWord =
        if (libc::strlen((*ipe).prov.ty_desc) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.ty_desc) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut label_len: StgWord =
        if (libc::strlen((*ipe).prov.label) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.label) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut module_len: StgWord =
        if (libc::strlen((*ipe).prov.module) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.module) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut src_file_len: StgWord =
        if (libc::strlen((*ipe).prov.src_file) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.src_file) as StgWord
        } else {
            MAX_IPE_STRING_LEN
        };

    let mut src_span_len: StgWord =
        if (libc::strlen((*ipe).prov.src_span) as StgWord) < MAX_IPE_STRING_LEN {
            libc::strlen((*ipe).prov.src_span) as StgWord
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

    postEventHeader(&raw mut eventBuf, EventTypeNum::IPE);
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

unsafe fn printAndClearEventBuf(ebuf: *mut EventsBuf) {
    closeBlockMarker(ebuf);

    if !(*ebuf).begin.is_null() && (*ebuf).pos != (*ebuf).begin {
        let elog_size: usize = (*ebuf).pos.offset_from((*ebuf).begin) as i64 as usize;

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

unsafe fn initEventsBuf(eb: *mut EventsBuf, mut size: StgWord64, mut capno: EventCapNo) {
    (*eb).pos = stgMallocBytes(size as usize, c"initEventsBuf".as_ptr()) as *mut StgInt8;
    (*eb).begin = (*eb).pos;
    (*eb).size = size;
    (*eb).marker = null_mut::<StgInt8>();
    (*eb).capno = capno;
    postBlockMarker(eb);
}

unsafe fn resetEventsBuf(eb: *mut EventsBuf) {
    (*eb).pos = (*eb).begin;
    (*eb).marker = null_mut::<StgInt8>();
}

unsafe fn hasRoomForEvent(eb: *mut EventsBuf, mut eNum: EventTypeNum) -> StgBool {
    let mut size: u32 = (size_of::<EventTypeNum>() as usize)
        .wrapping_add(size_of::<EventTimestamp>() as usize)
        .wrapping_add(eventTypes[eNum as usize].size as usize) as u32;

    if (*eb).pos.offset(size as isize) > (*eb).begin.offset((*eb).size as isize) {
        return 0;
    } else {
        return 1;
    };
}

unsafe fn hasRoomForVariableEvent(eb: *mut EventsBuf, mut payload_bytes: StgWord) -> StgBool {
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

unsafe fn ensureRoomForEvent(eb: *mut EventsBuf, tag: EventTypeNum) {
    if hasRoomForEvent(eb, tag) == 0 {
        printAndClearEventBuf(eb);

        if (hasRoomForEvent(eb, tag) != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/eventlog/EventLog.c".as_ptr(), 1559);
        }
    }
}

unsafe fn ensureRoomForVariableEvent(eb: *mut EventsBuf, size: StgWord) -> i32 {
    if hasRoomForVariableEvent(eb, size) == 0 {
        printAndClearEventBuf(eb);

        if hasRoomForVariableEvent(eb, size) == 0 {
            return 1;
        }
    }

    return 0;
}

unsafe fn postEventType(eb: *mut EventsBuf, et: &EventType) {
    postInt32(eb, EVENT_ET_BEGIN as StgInt32);
    postEventTypeNum(eb, et.etNum);
    postWord16(eb, et.size as StgWord16);

    let desc_bytes = et.desc.as_bytes();
    postWord32(eb, desc_bytes.len() as StgWord32);

    for b in desc_bytes.into_iter().copied() {
        postInt8(eb, b as StgInt8);
    }

    postWord32(eb, 0);
    postInt32(eb, EventTypeNum::ET_END as StgInt32);
}

unsafe fn flushLocalEventsBuf(cap: *const Capability) {
    let eb: *mut EventsBuf = capEventBuf.offset((*cap).no as isize) as *mut EventsBuf;
    printAndClearEventBuf(eb);
}

pub(crate) unsafe fn flushAllCapsEventsBufs() {
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

    for i in 0..getNumCapabilities() {
        flushLocalEventsBuf(getCapability(i));
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
