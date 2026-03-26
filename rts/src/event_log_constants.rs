use crate::prelude::*;

pub(crate) const EVENT_BLOCKS_SIZE: c_int = 91;

pub(crate) const EVENT_BLOCK_MARKER: c_int = 18 as c_int;

pub(crate) const EVENT_CAPSET_ASSIGN_CAP: c_int = 27;

pub(crate) const EVENT_CAPSET_CREATE: c_int = 25;

pub(crate) const EVENT_CAPSET_DELETE: c_int = 26;

pub(crate) const EVENT_CAPSET_REMOVE_CAP: c_int = 28;

pub(crate) const EVENT_CAP_CREATE: c_int = 45;

pub(crate) const EVENT_CAP_DELETE: c_int = 46;

pub(crate) const EVENT_CAP_DISABLE: c_int = 47;

pub(crate) const EVENT_CAP_ENABLE: c_int = 48;

pub(crate) const EVENT_CONC_MARK_BEGIN: c_int = 200 as c_int;

pub(crate) const EVENT_CONC_MARK_END: c_int = 201 as c_int;

pub(crate) const EVENT_CONC_SWEEP_BEGIN: c_int = 204 as c_int;

pub(crate) const EVENT_CONC_SWEEP_END: c_int = 205 as c_int;

pub(crate) const EVENT_CONC_SYNC_BEGIN: c_int = 202 as c_int;

pub(crate) const EVENT_CONC_SYNC_END: c_int = 203 as c_int;

pub(crate) const EVENT_CONC_UPD_REM_SET_FLUSH: c_int = 206 as c_int;

pub(crate) const EVENT_CREATE_SPARK_THREAD: c_int = 15;

pub(crate) const EVENT_CREATE_THREAD: c_int = 0;

pub(crate) const EVENT_GC_DONE: c_int = 22;

pub(crate) const EVENT_GC_END: c_int = 10;

pub(crate) const EVENT_GC_GLOBAL_SYNC: c_int = 54;

pub(crate) const EVENT_GC_IDLE: c_int = 20;

pub(crate) const EVENT_GC_START: c_int = 9;

pub(crate) const EVENT_GC_STATS_GHC: c_int = 53 as c_int;

pub(crate) const EVENT_GC_WORK: c_int = 21;

pub(crate) const EVENT_HEAP_ALLOCATED: c_int = 49;

pub(crate) const EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN: c_int = 166 as c_int;

pub(crate) const EVENT_HEAP_INFO_GHC: c_int = 52 as c_int;

pub(crate) const EVENT_HEAP_LIVE: c_int = 51;

pub(crate) const EVENT_HEAP_PROF_BEGIN: c_int = 160 as c_int;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_BEGIN: c_int = 162 as c_int;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_END: c_int = 165 as c_int;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_STRING: c_int = 164 as c_int;

pub(crate) const EVENT_HEAP_SIZE: c_int = 50;

pub(crate) const EVENT_IPE: c_int = 169 as c_int;

pub(crate) const EVENT_LOG_MSG: c_int = 16 as c_int;

pub(crate) const EVENT_MEM_RETURN: c_int = 90 as c_int;

pub(crate) const EVENT_MIGRATE_THREAD: c_int = 4;

pub(crate) const EVENT_NONMOVING_HEAP_CENSUS: c_int = 207 as c_int;

pub(crate) const EVENT_NONMOVING_PRUNED_SEGMENTS: c_int = 208 as c_int;

pub(crate) const EVENT_OSPROCESS_PID: c_int = 32 as c_int;

pub(crate) const EVENT_OSPROCESS_PPID: c_int = 33 as c_int;

pub(crate) const EVENT_PROGRAM_ARGS: c_int = 30 as c_int;

pub(crate) const EVENT_REQUEST_PAR_GC: c_int = 12;

pub(crate) const EVENT_REQUEST_SEQ_GC: c_int = 11;

pub(crate) const EVENT_RTS_IDENTIFIER: c_int = 29 as c_int;

pub(crate) const EVENT_RUN_THREAD: c_int = 1;

pub(crate) const EVENT_SPARK_COUNTERS: c_int = 34 as c_int;

pub(crate) const EVENT_SPARK_CREATE: c_int = 35;

pub(crate) const EVENT_SPARK_DUD: c_int = 36;

pub(crate) const EVENT_SPARK_FIZZLE: c_int = 40;

pub(crate) const EVENT_SPARK_GC: c_int = 41;

pub(crate) const EVENT_SPARK_OVERFLOW: c_int = 37;

pub(crate) const EVENT_SPARK_RUN: c_int = 38;

pub(crate) const EVENT_SPARK_STEAL: c_int = 39;

pub(crate) const EVENT_STOP_THREAD: c_int = 2;

pub(crate) const EVENT_TASK_CREATE: c_int = 55 as c_int;

pub(crate) const EVENT_TASK_DELETE: c_int = 57 as c_int;

pub(crate) const EVENT_TASK_MIGRATE: c_int = 56 as c_int;

pub(crate) const EVENT_THREAD_LABEL: c_int = 44 as c_int;

pub(crate) const EVENT_THREAD_RUNNABLE: c_int = 3;

pub(crate) const EVENT_THREAD_WAKEUP: c_int = 8;

pub(crate) const EVENT_USER_BINARY_MSG: c_int = 181 as c_int;

pub(crate) const EVENT_USER_MARKER: c_int = 58 as c_int;

pub(crate) const EVENT_USER_MSG: c_int = 19 as c_int;

pub(crate) const EVENT_WALL_CLOCK_TIME: c_int = 43 as c_int;
