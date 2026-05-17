use crate::event_log::format::{
    EventCapNo, EventCapsetType, EventKernelThreadId, EventTaskId, EventThreadID, EventTimestamp,
    NUM_GHC_EVENT_TAGS,
};
use crate::stg::types::{StgWord8, StgWord16, StgWord32, StgWord64};

pub(crate) type EventType = _EventType;

/// cbindgen:no-export
pub(super) struct _EventType {
    pub(super) etNum: EventTypeNum,
    pub(super) size: u32,
    pub(super) desc: &'static str,
}

type VariableLength = [u8; u16::MAX as usize];
type ThreadId = EventThreadID;
type KernelThreadId = EventKernelThreadId;
type CapNo = EventCapNo;
type CapsetType = EventCapsetType;
type Timestamp = EventTimestamp;
type CapsetId = EventCapsetID;
type TaskId = EventTaskId;
type Word8 = StgWord8;
type Word16 = StgWord16;
type Word32 = StgWord32;
type Word64 = StgWord64;

type Word64x3 = [StgWord64; 3];
type Word64x4 = [StgWord64; 4];

macro_rules! gen_event_types {
    ($( ( $n:literal, $name:ident, [ $($ty:ty),* $(,)? ], $desc:literal ), )+) => {
        #[repr(u16)]
        pub(crate) enum EventTypeNum {
            $(
                #[doc = $desc]
                $name = $n,
            )+
        }

        impl TryFrom<u16> for EventTypeNum {
            type Error = ();

            fn try_from(i: u16) -> ::std::result::Result<Self, ()> {
                match i {
                    $( $n => Ok(EventTypeNum::$name), )+
                    _ => Err(()),
                }
            }
        }

        pub(super) static eventTypes: [Option<_EventType>; NUM_GHC_EVENT_TAGS] = {
            let mut types = [const { None }; _];

            $(
                types[$n as usize] = Some(_EventType {
                    etNum: EventTypeNum::$name,
                    size: ($( size_of::<$ty>() + )* 0) as u32,
                    desc: $desc,
                });
            )+

            types
        };
    };
}

gen_event_types![
    (0, CREATE_THREAD, [ThreadId], "Create thread"),
    (1, RUN_THREAD, [ThreadId], "Run thread"),
    (2, STOP_THREAD, [ThreadId, Word16, ThreadId], "Stop thread"),
    (3, THREAD_RUNNABLE, [ThreadId], "Thread runnable"),
    (4, MIGRATE_THREAD, [ThreadId, CapNo], "Migrate thread"),
    // 5, 6, 7 deprecated
    (8, THREAD_WAKEUP, [ThreadId, CapNo], "Wakeup thread"),
    (9, GC_START, [], "Starting GC"),
    (10, GC_END, [], "Finished GC"),
    (11, REQUEST_SEQ_GC, [], "Request sequential GC"),
    (12, REQUEST_PAR_GC, [], "Request parallel GC"),
    // 13, 14 deprecated
    (15, CREATE_SPARK_THREAD, [ThreadId], "Create spark thread"),
    (16, LOG_MSG, [VariableLength], "Log message"),
    // 17 deprecated
    (18, BLOCK_MARKER, [Word32, Timestamp, CapNo], "Block marker"),
    (19, USER_MSG, [VariableLength], "User message"),
    (20, GC_IDLE, [], "GC idle"),
    (21, GC_WORK, [], "GC working"),
    (22, GC_DONE, [], "GC done"),
    // 23, 24 used by eden
    (
        25,
        CAPSET_CREATE,
        [CapsetId, CapsetType],
        "Create capability set"
    ),
    (26, CAPSET_DELETE, [CapsetId], "Delete capability set"),
    (
        27,
        CAPSET_ASSIGN_CAP,
        [CapsetId, CapNo],
        "Add capability to capability set"
    ),
    (
        28,
        CAPSET_REMOVE_CAP,
        [CapsetId, CapNo],
        "Remove capability from capability set"
    ),
    (29, RTS_IDENTIFIER, [VariableLength], "RTS name and version"),
    (30, PROGRAM_ARGS, [VariableLength], "Program arguments"),
    (
        31,
        PROGRAM_ENV,
        [VariableLength],
        "Program environment variables"
    ),
    (32, OSPROCESS_PID, [CapsetId, Word32], "Process ID"),
    (33, OSPROCESS_PPID, [CapsetId, Word32], "Parent process ID"),
    (34, SPARK_COUNTERS, [Word64x4, Word64x3], "Spark counters"),
    (35, SPARK_CREATE, [], "Spark create"),
    (36, SPARK_DUD, [], "Spark dud"),
    (37, SPARK_OVERFLOW, [], "Spark overflow"),
    (38, SPARK_RUN, [], "Spark run"),
    (39, SPARK_STEAL, [CapNo], "Spark steal"),
    (40, SPARK_FIZZLE, [], "Spark fizzle"),
    (41, SPARK_GC, [], "Spark GC"),
    (42, INTERN_STRING, [VariableLength], "Intern string"),
    (
        43,
        WALL_CLOCK_TIME,
        [CapsetId, Word64, Word32],
        "Wall clock time"
    ),
    (44, THREAD_LABEL, [VariableLength], "Thread label"),
    (45, CAP_CREATE, [CapNo], "Create capability"),
    (46, CAP_DELETE, [CapNo], "Delete capability"),
    (47, CAP_DISABLE, [CapNo], "Disable capability"),
    (48, CAP_ENABLE, [CapNo], "Enable capability"),
    (
        49,
        HEAP_ALLOCATED,
        [CapsetId, Word64],
        "Total heap memory ever allocated"
    ),
    (
        50,
        HEAP_SIZE,
        [CapsetId, Word64],
        "Current heap size (number of allocated mblocks)"
    ),
    (51, HEAP_LIVE, [CapsetId, Word64], "Current heap live data"),
    (
        52,
        HEAP_INFO_GHC,
        [CapsetId, Word16, Word64x4],
        "Heap static parameters"
    ),
    (
        53,
        GC_STATS_GHC,
        [CapsetId, Word16, Word64x3, Word32, Word64x3],
        "GC statistics"
    ),
    (54, GC_GLOBAL_SYNC, [], "Synchronise stop-the-world GC"),
    (
        55,
        TASK_CREATE,
        [TaskId, CapNo, KernelThreadId],
        "Task create"
    ),
    (56, TASK_MIGRATE, [TaskId, CapNo, CapNo], "Task migrate"),
    (57, TASK_DELETE, [TaskId], "Task delete"),
    (58, USER_MARKER, [VariableLength], "User marker"),
    (59, HACK_BUG_T9003, [], "Empty event for bug #9003"),
    // Range 60 - 80 is used by eden for parallel tracing.
    // See http://www.mathematik.uni-marburg.de/~eden/
    (
        90,
        MEM_RETURN,
        [CapsetId, Word32, Word32, Word32],
        "The RTS attempted to return heap memory to the OS"
    ),
    (
        91,
        BLOCKS_SIZE,
        [CapsetId, Word64],
        "Report the size of the heap in blocks"
    ),
    // Range 100 - 139 is reserved for Mercury.

    // Range 140 - 159 is reserved for Perf events.

    // Range 160 - 180 is reserved for cost-centre heap profiling events.

    // Cost-centre profiler
    (
        160,
        HEAP_PROF_BEGIN,
        [VariableLength],
        "Start of heap profile"
    ),
    (
        161,
        HEAP_PROF_COST_CENTRE,
        [VariableLength],
        "Cost-centre definition"
    ),
    (
        162,
        HEAP_PROF_SAMPLE_BEGIN,
        [Word64],
        "Start of heap profile sample"
    ),
    (
        163,
        HEAP_PROF_SAMPLE_COST_CENTRE,
        [VariableLength],
        "Heap profile cost-centre sample"
    ),
    (
        164,
        HEAP_PROF_SAMPLE_STRING,
        [VariableLength],
        "Heap profile string sample"
    ),
    (
        165,
        HEAP_PROF_SAMPLE_END,
        [Word64],
        "End of heap profile sample"
    ),
    (
        166,
        HEAP_BIO_PROF_SAMPLE_BEGIN,
        [Word64, Word64],
        "Start of heap profile (biographical) sample"
    ),
    (
        167,
        PROF_SAMPLE_COST_CENTRE,
        [VariableLength],
        "Time profile cost-centre stack"
    ),
    (168, PROF_BEGIN, [Word64], "Start of a time profile"),
    (169, IPE, [VariableLength], "An IPE entry"),
    (
        181,
        USER_BINARY_MSG,
        [VariableLength],
        "User binary message"
    ),
    // Non-moving GC
    (200, CONC_MARK_BEGIN, [], "Begin concurrent mark phase"),
    (201, CONC_MARK_END, [Word32], "End concurrent mark phase"),
    (
        202,
        CONC_SYNC_BEGIN,
        [],
        "Begin concurrent GC synchronisation"
    ),
    (
        203,
        CONC_SYNC_END,
        [],
        "End concurrent mark synchronisation"
    ),
    (204, CONC_SWEEP_BEGIN, [], "Begin concurrent sweep phase"),
    (205, CONC_SWEEP_END, [], "End concurrent sweep phase"),
    (
        206,
        CONC_UPD_REM_SET_FLUSH,
        [CapNo],
        "Update remembered set flushed"
    ),
    (
        207,
        NONMOVING_HEAP_CENSUS,
        [Word16, Word32, Word32, Word32],
        "Nonmoving heap census"
    ),
    (
        208,
        NONMOVING_PRUNED_SEGMENTS,
        [Word32, Word32],
        "Report the amount of segments pruned and remaining on the free list."
    ),
    // Ticky-ticky profiling
    (
        210,
        TICKY_COUNTER_DEF,
        [VariableLength],
        "Ticky-ticky entry counter definition"
    ),
    (
        211,
        TICKY_COUNTER_SAMPLE,
        [Word64x4],
        "Ticky-ticky entry counter sample"
    ),
    (
        212,
        TICKY_COUNTER_BEGIN_SAMPLE,
        [],
        "Ticky-ticky entry counter begin sample"
    ),
];
