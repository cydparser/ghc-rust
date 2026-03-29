use crate::prelude::*;

pub(crate) const EVENT_BLOCKS_SIZE: i32 = 91;

pub(crate) const EVENT_BLOCK_MARKER: i32 = 18;

pub(crate) const EVENT_CAPSET_ASSIGN_CAP: i32 = 27;

pub(crate) const EVENT_CAPSET_CREATE: i32 = 25;

pub(crate) const EVENT_CAPSET_DELETE: i32 = 26;

pub(crate) const EVENT_CAPSET_REMOVE_CAP: i32 = 28;

pub(crate) const EVENT_CAP_CREATE: i32 = 45;

pub(crate) const EVENT_CAP_DELETE: i32 = 46;

pub(crate) const EVENT_CAP_DISABLE: i32 = 47;

pub(crate) const EVENT_CAP_ENABLE: i32 = 48;

pub(crate) const EVENT_CONC_MARK_BEGIN: i32 = 200;

pub(crate) const EVENT_CONC_MARK_END: i32 = 201;

pub(crate) const EVENT_CONC_SWEEP_BEGIN: i32 = 204;

pub(crate) const EVENT_CONC_SWEEP_END: i32 = 205;

pub(crate) const EVENT_CONC_SYNC_BEGIN: i32 = 202;

pub(crate) const EVENT_CONC_SYNC_END: i32 = 203;

pub(crate) const EVENT_CONC_UPD_REM_SET_FLUSH: i32 = 206;

pub(crate) const EVENT_CREATE_SPARK_THREAD: i32 = 15;

pub(crate) const EVENT_CREATE_THREAD: i32 = 0;

pub(crate) const EVENT_GC_DONE: i32 = 22;

pub(crate) const EVENT_GC_END: i32 = 10;

pub(crate) const EVENT_GC_GLOBAL_SYNC: i32 = 54;

pub(crate) const EVENT_GC_IDLE: i32 = 20;

pub(crate) const EVENT_GC_START: i32 = 9;

pub(crate) const EVENT_GC_STATS_GHC: i32 = 53;

pub(crate) const EVENT_GC_WORK: i32 = 21;

pub(crate) const EVENT_HEAP_ALLOCATED: i32 = 49;

pub(crate) const EVENT_HEAP_BIO_PROF_SAMPLE_BEGIN: i32 = 166;

pub(crate) const EVENT_HEAP_INFO_GHC: i32 = 52;

pub(crate) const EVENT_HEAP_LIVE: i32 = 51;

pub(crate) const EVENT_HEAP_PROF_BEGIN: i32 = 160;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_BEGIN: i32 = 162;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_END: i32 = 165;

pub(crate) const EVENT_HEAP_PROF_SAMPLE_STRING: i32 = 164;

pub(crate) const EVENT_HEAP_SIZE: i32 = 50;

pub(crate) const EVENT_IPE: i32 = 169;

pub(crate) const EVENT_LOG_MSG: i32 = 16;

pub(crate) const EVENT_MEM_RETURN: i32 = 90;

pub(crate) const EVENT_MIGRATE_THREAD: i32 = 4;

pub(crate) const EVENT_NONMOVING_HEAP_CENSUS: i32 = 207;

pub(crate) const EVENT_NONMOVING_PRUNED_SEGMENTS: i32 = 208;

pub(crate) const EVENT_OSPROCESS_PID: i32 = 32;

pub(crate) const EVENT_OSPROCESS_PPID: i32 = 33;

pub(crate) const EVENT_PROGRAM_ARGS: i32 = 30;

pub(crate) const EVENT_REQUEST_PAR_GC: i32 = 12;

pub(crate) const EVENT_REQUEST_SEQ_GC: i32 = 11;

pub(crate) const EVENT_RTS_IDENTIFIER: i32 = 29;

pub(crate) const EVENT_RUN_THREAD: i32 = 1;

pub(crate) const EVENT_SPARK_COUNTERS: i32 = 34;

pub(crate) const EVENT_SPARK_CREATE: i32 = 35;

pub(crate) const EVENT_SPARK_DUD: i32 = 36;

pub(crate) const EVENT_SPARK_FIZZLE: i32 = 40;

pub(crate) const EVENT_SPARK_GC: i32 = 41;

pub(crate) const EVENT_SPARK_OVERFLOW: i32 = 37;

pub(crate) const EVENT_SPARK_RUN: i32 = 38;

pub(crate) const EVENT_SPARK_STEAL: i32 = 39;

pub(crate) const EVENT_STOP_THREAD: i32 = 2;

pub(crate) const EVENT_TASK_CREATE: i32 = 55;

pub(crate) const EVENT_TASK_DELETE: i32 = 57;

pub(crate) const EVENT_TASK_MIGRATE: i32 = 56;

pub(crate) const EVENT_THREAD_LABEL: i32 = 44;

pub(crate) const EVENT_THREAD_RUNNABLE: i32 = 3;

pub(crate) const EVENT_THREAD_WAKEUP: i32 = 8;

pub(crate) const EVENT_USER_BINARY_MSG: i32 = 181;

pub(crate) const EVENT_USER_MARKER: i32 = 58;

pub(crate) const EVENT_USER_MSG: i32 = 19;

pub(crate) const EVENT_WALL_CLOCK_TIME: i32 = 43;
