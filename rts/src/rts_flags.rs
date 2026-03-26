use crate::ffi::hs_ffi::{HS_INT_MAX, HS_WORD_MAX, HS_WORD64_MAX, HsBool};
use crate::ffi::rts::event_log_writer::FileEventLogWriter;
use crate::ffi::rts::flags::{
    _CONCURRENT_FLAGS, _COST_CENTRE_FLAGS, _DEBUG_FLAGS, _GC_FLAGS, _HPC_FLAGS, _MISC_FLAGS,
    _PAR_FLAGS, _PROFILING_FLAGS, _RTS_FLAGS, _TICKY_FLAGS, _TRACE_FLAGS, COLLECT_GC_STATS,
    DEFAULT_LINKER_ALWAYS_PIC, DEFAULT_TICK_INTERVAL, HEAP_BY_CLOSURE_TYPE, HEAP_BY_INFO_TABLE,
    HPC_NO_EXPLICIT, HPC_YES_EXPLICIT, HPC_YES_IMPLICIT, IO_MNGR_FLAG_AUTO, NO_GC_STATS,
    ONELINE_GC_STATS, RTS_FLAGS, STATS_FILENAME_MAXLEN, SUMMARY_GC_STATS, TRACE_NONE, TRACE_STDERR,
    VERBOSE_GC_STATS,
};
use crate::ffi::rts::messages::{barf, errorBelch, vdebugBelch};
use crate::ffi::rts::stg_exit;
use crate::ffi::rts::storage::block::{BLOCK_SIZE, MBLOCK_SIZE};
use crate::ffi::rts::time::{Time, fsecondsToTime};
use crate::ffi::rts_api::{
    RtsConfig, RtsOptsAll, RtsOptsEnabledEnum, RtsOptsIgnore, RtsOptsIgnoreAll, RtsOptsNone,
    RtsOptsSafeOnly,
};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgDouble, StgWord, StgWord64};
use crate::fs::__rts_fopen;
use crate::hooks::hooks::{
    FlagDefaultsHook, LongGCSync, LongGCSyncEnd, MallocFailHook, OnExitHook, OutOfHeapHook,
    StackOverflowHook,
};
use crate::io_manager::{IOManagerAvailable, IOManagerUnavailable, parseIOManagerFlag};
use crate::prelude::*;
use crate::rts_utils::{printRtsInfo, stgCallocBytes, stgFree, stgMallocBytes, stgReallocBytes};
use crate::sm::os_mem::getPhysicalMemorySize;

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub static mut RtsFlags: RTS_FLAGS = _RTS_FLAGS {
    GcFlags: _GC_FLAGS {
        statsFile: null::<FILE>() as *mut FILE,
        giveStats: 0,
        maxStkSize: 0,
        initialStkSize: 0,
        stkChunkSize: 0,
        stkChunkBufferSize: 0,
        maxHeapSize: 0,
        minAllocAreaSize: 0,
        largeAllocLim: 0,
        nurseryChunkSize: 0,
        minOldGenSize: 0,
        heapSizeSuggestion: 0,
        heapSizeSuggestionAuto: false,
        oldGenFactor: 0.,
        returnDecayFactor: 0.,
        pcFreeHeap: 0.,
        useNonmoving: false,
        nonmovingDenseAllocatorCount: 0,
        generations: 0,
        squeezeUpdFrames: false,
        compact: false,
        compactThreshold: 0.,
        sweep: false,
        ringBell: false,
        idleGCDelayTime: 0,
        interIdleGCWait: 0,
        doIdleGC: false,
        longGCSync: 0,
        heapBase: 0,
        allocLimitGrace: 0,
        heapLimitGrace: 0,
        numa: false,
        numaMask: 0,
        addressSpaceSize: 0,
    },
    ConcFlags: _CONCURRENT_FLAGS {
        ctxtSwitchTime: 0,
        ctxtSwitchTicks: 0,
    },
    MiscFlags: _MISC_FLAGS {
        tickInterval: 0,
        install_signal_handlers: false,
        install_seh_handlers: false,
        generate_dump_file: false,
        generate_stack_trace: false,
        machineReadable: false,
        disableDelayedOsMemoryReturn: false,
        internalCounters: false,
        linkerAlwaysPic: false,
        linkerOptimistic: false,
        linkerMemBase: 0,
        ioManager: IO_MNGR_FLAG_AUTO,
        numIoWorkerThreads: 0,
    },
    DebugFlags: _DEBUG_FLAGS {
        scheduler: false,
        interpreter: false,
        weak: false,
        gccafs: false,
        gc: false,
        nonmoving_gc: false,
        block_alloc: false,
        sanity: false,
        zero_on_gc: false,
        stable: false,
        prof: false,
        linker: false,
        linker_verbose: false,
        apply: false,
        stm: false,
        squeeze: false,
        hpc: false,
        sparks: false,
        numa: false,
        compact: false,
        continuation: false,
        iomanager: false,
    },
    CcFlags: _COST_CENTRE_FLAGS {
        doCostCentres: 0,
        profilerTicks: 0,
        msecsPerTick: 0,
        outputFileNameStem: null::<c_char>(),
    },
    ProfFlags: _PROFILING_FLAGS {
        doHeapProfile: 0,
        heapProfileInterval: 0,
        heapProfileIntervalTicks: 0,
        startHeapProfileAtStartup: false,
        startTimeProfileAtStartup: false,
        incrementUserEra: false,
        showCCSOnException: false,
        maxRetainerSetSize: 0,
        ccsLength: 0,
        modSelector: null::<c_char>(),
        descrSelector: null::<c_char>(),
        typeSelector: null::<c_char>(),
        ccSelector: null::<c_char>(),
        ccsSelector: null::<c_char>(),
        retainerSelector: null::<c_char>(),
        eraSelector: 0,
        bioSelector: null::<c_char>(),
    },
    TraceFlags: _TRACE_FLAGS {
        tracing: 0,
        timestamp: false,
        scheduler: false,
        gc: false,
        nonmoving_gc: false,
        sparks_sampled: false,
        sparks_full: false,
        ticky: false,
        user: false,
        eventlogFlushTime: 0,
        eventlogFlushTicks: 0,
        trace_output: null::<c_char>() as *mut c_char,
        nullWriter: false,
    },
    TickyFlags: _TICKY_FLAGS {
        showTickyStats: false,
        tickyFile: null::<FILE>() as *mut FILE,
    },
    ParFlags: _PAR_FLAGS {
        nCapabilities: 0,
        migrate: false,
        maxLocalSparks: 0,
        parGcEnabled: false,
        parGcGen: 0,
        parGcLoadBalancingEnabled: false,
        parGcLoadBalancingGen: 0,
        parGcNoSyncWithIdle: 0,
        parGcThreads: 0,
        setAffinity: false,
    },
    HpcFlags: _HPC_FLAGS {
        writeTixFile: false,
        readTixFile: HPC_NO_EXPLICIT,
    },
};

static mut prog_argc: c_int = 0 as c_int;

static mut prog_argv: *mut *mut c_char = null::<*mut c_char>() as *mut *mut c_char;

static mut full_prog_argc: c_int = 0 as c_int;

static mut full_prog_argv: *mut *mut c_char = null::<*mut c_char>() as *mut *mut c_char;

static mut prog_name: *mut c_char = null::<c_char>() as *mut c_char;

static mut rts_argc: c_int = 0 as c_int;

static mut rts_argv: *mut *mut c_char = null::<*mut c_char>() as *mut *mut c_char;

static mut rts_argv_size: c_int = 0 as c_int;

static mut rtsConfig: RtsConfig = RtsConfig {
    rts_opts_enabled: RtsOptsNone,
    rts_opts_suggestions: 0,
    rts_opts: null::<c_char>(),
    rts_hs_main: 0,
    keep_cafs: 0,
    eventlog_writer: null::<EventLogWriter>(),
    defaultsHook: None,
    onExitHook: None,
    stackOverflowHook: None,
    outOfHeapHook: None,
    mallocFailHook: None,
    gcDoneHook: None,
    longGCSync: None,
    longGCSyncEnd: None,
};

#[ffi(compiler, docs, testsuite, utils)]
#[unsafe(no_mangle)]
pub static mut defaultRtsConfig: RtsConfig = unsafe {
    RtsConfig {
        rts_opts_enabled: RtsOptsSafeOnly,
        rts_opts_suggestions: r#true as HsBool,
        rts_opts: null::<c_char>(),
        rts_hs_main: r#false as HsBool,
        keep_cafs: r#false as HsBool,
        eventlog_writer: &raw const FileEventLogWriter,
        defaultsHook: Some(FlagDefaultsHook as unsafe extern "C" fn() -> ()),
        onExitHook: Some(OnExitHook as unsafe extern "C" fn() -> ()),
        stackOverflowHook: Some(StackOverflowHook as unsafe extern "C" fn(W_) -> ()),
        outOfHeapHook: Some(OutOfHeapHook as unsafe extern "C" fn(W_, W_) -> ()),
        mallocFailHook: Some(MallocFailHook as unsafe extern "C" fn(W_, *const c_char) -> ()),
        gcDoneHook: None,
        longGCSync: Some(LongGCSync as unsafe extern "C" fn(uint32_t, Time) -> ()),
        longGCSyncEnd: Some(LongGCSyncEnd as unsafe extern "C" fn(Time) -> ()),
    }
};

const RTS: c_int = 1 as c_int;

const PGM: c_int = 0 as c_int;

unsafe fn initRtsFlagsDefaults() {
    let mut maxStkSize: StgWord64 = (8 as StgWord64)
        .wrapping_mul(getPhysicalMemorySize())
        .wrapping_div(10 as StgWord64);

    if maxStkSize == 0 as StgWord64 {
        maxStkSize = (8 as c_int * 1024 as c_int * 1024 as c_int) as StgWord64;
    } else if maxStkSize > (UINT32_MAX as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord64
    {
        maxStkSize = (UINT32_MAX as usize).wrapping_mul(size_of::<W_>() as usize) as StgWord64;
    }

    RtsFlags.GcFlags.statsFile = null_mut::<FILE>();
    RtsFlags.GcFlags.giveStats = NO_GC_STATS as uint32_t;
    RtsFlags.GcFlags.maxStkSize = maxStkSize.wrapping_div(size_of::<W_>() as StgWord64) as uint32_t;
    RtsFlags.GcFlags.initialStkSize =
        (1024 as usize).wrapping_div(size_of::<W_>() as usize) as uint32_t;
    RtsFlags.GcFlags.stkChunkSize =
        ((32 as c_int * 1024 as c_int) as usize).wrapping_div(size_of::<W_>() as usize) as uint32_t;
    RtsFlags.GcFlags.stkChunkBufferSize =
        ((1 as c_int * 1024 as c_int) as usize).wrapping_div(size_of::<W_>() as usize) as uint32_t;
    RtsFlags.GcFlags.minAllocAreaSize = ((4 as c_int * 1024 as c_int * 1024 as c_int) as c_ulong)
        .wrapping_div(BLOCK_SIZE) as uint32_t;
    RtsFlags.GcFlags.largeAllocLim = 0 as uint32_t;
    RtsFlags.GcFlags.nurseryChunkSize = 0 as uint32_t;
    RtsFlags.GcFlags.minOldGenSize =
        ((1024 as c_int * 1024 as c_int) as c_ulong).wrapping_div(BLOCK_SIZE) as uint32_t;
    RtsFlags.GcFlags.maxHeapSize = 0 as uint32_t;
    RtsFlags.GcFlags.heapLimitGrace = (1024 as c_int * 1024 as c_int) as StgWord;
    RtsFlags.GcFlags.heapSizeSuggestion = 0 as uint32_t;
    RtsFlags.GcFlags.heapSizeSuggestionAuto = r#false != 0;
    RtsFlags.GcFlags.pcFreeHeap = 3 as c_int as c_double;
    RtsFlags.GcFlags.oldGenFactor = 2 as c_int as c_double;
    RtsFlags.GcFlags.returnDecayFactor = 4 as c_int as c_double;
    RtsFlags.GcFlags.useNonmoving = r#false != 0;
    RtsFlags.GcFlags.nonmovingDenseAllocatorCount = 16 as uint16_t;
    RtsFlags.GcFlags.generations = 2 as uint32_t;
    RtsFlags.GcFlags.squeezeUpdFrames = r#true != 0;
    RtsFlags.GcFlags.compact = r#false != 0;
    RtsFlags.GcFlags.compactThreshold = 30.0f64;
    RtsFlags.GcFlags.sweep = r#false != 0;
    RtsFlags.GcFlags.idleGCDelayTime = 300000 as c_int as Time * 1000 as Time;
    RtsFlags.GcFlags.interIdleGCWait = 0 as Time;
    RtsFlags.GcFlags.doIdleGC = r#false != 0;
    RtsFlags.GcFlags.heapBase = 0 as StgWord;
    RtsFlags.GcFlags.allocLimitGrace =
        ((100 as c_int * 1024 as c_int) as c_ulong).wrapping_div(BLOCK_SIZE) as StgWord;
    RtsFlags.GcFlags.numa = r#false != 0;
    RtsFlags.GcFlags.numaMask = 1 as StgWord;
    RtsFlags.GcFlags.ringBell = r#false != 0;
    RtsFlags.GcFlags.longGCSync = 0 as Time;
    RtsFlags.GcFlags.addressSpaceSize = (1 as c_int as StgWord64) << 40 as c_int;
    RtsFlags.DebugFlags.scheduler = r#false != 0;
    RtsFlags.DebugFlags.interpreter = r#false != 0;
    RtsFlags.DebugFlags.weak = r#false != 0;
    RtsFlags.DebugFlags.gccafs = r#false != 0;
    RtsFlags.DebugFlags.gc = r#false != 0;
    RtsFlags.DebugFlags.nonmoving_gc = r#false != 0;
    RtsFlags.DebugFlags.block_alloc = r#false != 0;
    RtsFlags.DebugFlags.sanity = r#false != 0;
    RtsFlags.DebugFlags.zero_on_gc = r#false != 0;
    RtsFlags.DebugFlags.stable = r#false != 0;
    RtsFlags.DebugFlags.stm = r#false != 0;
    RtsFlags.DebugFlags.prof = r#false != 0;
    RtsFlags.DebugFlags.apply = r#false != 0;
    RtsFlags.DebugFlags.linker = r#false != 0;
    RtsFlags.DebugFlags.linker_verbose = r#false != 0;
    RtsFlags.DebugFlags.squeeze = r#false != 0;
    RtsFlags.DebugFlags.hpc = r#false != 0;
    RtsFlags.DebugFlags.sparks = r#false != 0;
    RtsFlags.DebugFlags.numa = r#false != 0;
    RtsFlags.DebugFlags.compact = r#false != 0;
    RtsFlags.DebugFlags.continuation = r#false != 0;
    RtsFlags.ProfFlags.doHeapProfile = r#false as uint32_t;
    RtsFlags.ProfFlags.heapProfileInterval = 100000 as c_int as Time * 1000 as Time;
    RtsFlags.ProfFlags.startHeapProfileAtStartup = r#true != 0;
    RtsFlags.ProfFlags.startTimeProfileAtStartup = r#true != 0;
    RtsFlags.ProfFlags.incrementUserEra = r#false != 0;
    RtsFlags.TraceFlags.tracing = TRACE_NONE;
    RtsFlags.TraceFlags.timestamp = r#false != 0;
    RtsFlags.TraceFlags.scheduler = r#false != 0;
    RtsFlags.TraceFlags.gc = r#false != 0;
    RtsFlags.TraceFlags.nonmoving_gc = r#false != 0;
    RtsFlags.TraceFlags.sparks_sampled = r#false != 0;
    RtsFlags.TraceFlags.sparks_full = r#false != 0;
    RtsFlags.TraceFlags.user = r#false != 0;
    RtsFlags.TraceFlags.ticky = r#false != 0;
    RtsFlags.TraceFlags.trace_output = null_mut::<c_char>();
    RtsFlags.TraceFlags.eventlogFlushTime = 0 as Time;
    RtsFlags.TraceFlags.nullWriter = r#false != 0;
    RtsFlags.MiscFlags.tickInterval = DEFAULT_TICK_INTERVAL;
    RtsFlags.ConcFlags.ctxtSwitchTime = 20000 as c_int as Time * 1000 as Time;
    RtsFlags.MiscFlags.install_signal_handlers = r#true != 0;
    RtsFlags.MiscFlags.install_seh_handlers = r#true != 0;
    RtsFlags.MiscFlags.generate_stack_trace = r#true != 0;
    RtsFlags.MiscFlags.generate_dump_file = r#false != 0;
    RtsFlags.MiscFlags.machineReadable = r#false != 0;
    RtsFlags.MiscFlags.disableDelayedOsMemoryReturn = r#false != 0;
    RtsFlags.MiscFlags.internalCounters = r#false != 0;
    RtsFlags.MiscFlags.linkerAlwaysPic = DEFAULT_LINKER_ALWAYS_PIC != 0;
    RtsFlags.MiscFlags.linkerOptimistic = r#false != 0;
    RtsFlags.MiscFlags.linkerMemBase = 0 as StgWord;
    RtsFlags.MiscFlags.ioManager = IO_MNGR_FLAG_AUTO;
    RtsFlags.MiscFlags.numIoWorkerThreads = 1 as uint32_t;
    RtsFlags.TickyFlags.showTickyStats = r#false != 0;
    RtsFlags.TickyFlags.tickyFile = null_mut::<FILE>();
    RtsFlags.HpcFlags.readTixFile = HPC_YES_IMPLICIT;
    RtsFlags.HpcFlags.writeTixFile = r#true != 0;
}

static mut usage_text: [*const c_char; 155] = [
    b"\0" as *const u8 as *const c_char,
    b"Usage: <prog> <args> [+RTS <rtsopts> | -RTS <args>] ... --RTS <args>\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"   +RTS     Indicates run time system options follow\0" as *const u8
        as *const c_char,
    b"   -RTS     Indicates program arguments follow\0" as *const u8
        as *const c_char,
    b"  --RTS     Indicates that ALL subsequent arguments will be given to the\0"
        as *const u8 as *const c_char,
    b"            program (including any of these RTS flags)\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"The following run time system options may be available (note that some\0"
        as *const u8 as *const c_char,
    b"of these may not be usable unless this program was linked with the -rtsopts\0"
        as *const u8 as *const c_char,
    b"flag):\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -?        Prints this message and exits; the program is not executed\0"
        as *const u8 as *const c_char,
    b"  --info    Print information about the RTS used by this program\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  --nonmoving-gc\0" as *const u8 as *const c_char,
    b"            Selects the non-moving mark-and-sweep garbage collector to\0"
        as *const u8 as *const c_char,
    b"            manage the oldest generation.\0" as *const u8
        as *const c_char,
    b"  --copying-gc\0" as *const u8 as *const c_char,
    b"            Selects the copying garbage collector to manage all generations.\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -K<size>  Sets the maximum stack size (default: 80% of the heap)\0" as *const u8
        as *const c_char,
    b"            e.g.: -K32k -K512k -K8M\0" as *const u8 as *const c_char,
    b"  -ki<size> Sets the initial thread stack size (default 1k)  e.g.: -ki4k -ki2m\0"
        as *const u8 as *const c_char,
    b"  -kc<size> Sets the stack chunk size (default 32k)\0" as *const u8
        as *const c_char,
    b"  -kb<size> Sets the stack chunk buffer size (default 1k)\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -A<size>  Sets the minimum allocation area size (default 4m) e.g.: -A20m -A10k\0"
        as *const u8 as *const c_char,
    b"  -AL<size> Sets the amount of large-object memory that can be allocated\0"
        as *const u8 as *const c_char,
    b"            before a GC is triggered (default: the value of -A)\0" as *const u8
        as *const c_char,
    b"  -F<n>     Sets the collecting threshold for old generations as a factor of\0"
        as *const u8 as *const c_char,
    b"            the live data in that generation the last time it was collected\0"
        as *const u8 as *const c_char,
    b"            (default: 2.0)\0" as *const u8 as *const c_char,
    b"  -Fd<n>    Sets the inverse rate which memory is returned to the OS after being\0"
        as *const u8 as *const c_char,
    b"            optimistically retained after being allocated. Subsequent major\0"
        as *const u8 as *const c_char,
    b"            collections not caused by heap overflow will return an amount of\0"
        as *const u8 as *const c_char,
    b"            memory controlled by this factor (higher is slower). Setting the factor\0"
        as *const u8 as *const c_char,
    b"            to 0 means memory is not returned.\0" as *const u8
        as *const c_char,
    b"            (default 4.0)\0" as *const u8 as *const c_char,
    b"  -n<size>  Allocation area chunk size (0 = disabled, default: 0)\0" as *const u8
        as *const c_char,
    b"  -O<size>  Sets the minimum size of the old generation (default 1M)\0"
        as *const u8 as *const c_char,
    b"  -M<size>  Sets the maximum heap size (default unlimited)  e.g.: -M256k -M1G\0"
        as *const u8 as *const c_char,
    b"  -H<size>  Sets the minimum heap size (default 0M)   e.g.: -H24m  -H1G\0"
        as *const u8 as *const c_char,
    b"  -xb<addr> Sets the address from which a suitable start for the heap memory\0"
        as *const u8 as *const c_char,
    b"            will be searched from. This is useful if the default address\0"
        as *const u8 as *const c_char,
    b"            clashes with some third-party library.\0" as *const u8
        as *const c_char,
    b"  -xn       Use the non-moving collector for the old generation.\0" as *const u8
        as *const c_char,
    b"  -m<n>     Minimum % of heap which must be available (default 3%)\0" as *const u8
        as *const c_char,
    b"  -G<n>     Number of generations (default: 2)\0" as *const u8
        as *const c_char,
    b"  -c<n>     Use in-place compaction instead of copying in the oldest generation\0"
        as *const u8 as *const c_char,
    b"            when live data is at least <n>% of the maximum heap size set with\0"
        as *const u8 as *const c_char,
    b"            -M (default: 30%)\0" as *const u8 as *const c_char,
    b"  -c        Use in-place compaction for all oldest generation collections\0"
        as *const u8 as *const c_char,
    b"            (the default is to use copying)\0" as *const u8
        as *const c_char,
    b"  -w        Use mark-region for the oldest generation (experimental)\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -T         Collect GC statistics (useful for in-program statistics access)\0"
        as *const u8 as *const c_char,
    b"  -t[<file>] One-line GC statistics (if <file> omitted, uses stderr)\0"
        as *const u8 as *const c_char,
    b"  -s[<file>] Summary  GC statistics (if <file> omitted, uses stderr)\0"
        as *const u8 as *const c_char,
    b"  -S[<file>] Detailed GC statistics (if <file> omitted, uses stderr)\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -Z         Don't squeeze out update frames on context switch\0" as *const u8
        as *const c_char,
    b"  -B         Sound the bell at the start of each garbage collection\0" as *const u8
        as *const c_char,
    b"  -h       Heap residency profile (output file <program>.hp)\0" as *const u8
        as *const c_char,
    b"  -hT      Produce a heap profile grouped by closure type\0" as *const u8
        as *const c_char,
    b"  -hi      Produce a heap profile grouped by info table address\0" as *const u8
        as *const c_char,
    b"  -po<file>  Override profiling output file name prefix (program name by default)\0"
        as *const u8 as *const c_char,
    b"  -i<sec>  Time between heap profile samples (seconds, default: 0.1)\0"
        as *const u8 as *const c_char,
    b"  --no-automatic-heap-samples\0" as *const u8 as *const c_char,
    b"           Do not start the heap profile interval timer on start-up,\0"
        as *const u8 as *const c_char,
    b"           Rather, the application will be responsible for triggering\0"
        as *const u8 as *const c_char,
    b"           heap profiler samples.\0" as *const u8 as *const c_char,
    b"  -ol<file>  Send binary eventlog to <file> (default: <program>.eventlog)\0"
        as *const u8 as *const c_char,
    b"  -l[flags]  Log events to a file\0" as *const u8 as *const c_char,
    b"  -v[flags]  Log events to stderr\0" as *const u8 as *const c_char,
    b"             where [flags] can contain:\0" as *const u8
        as *const c_char,
    b"                s    scheduler events\0" as *const u8
        as *const c_char,
    b"                g    GC and heap events\0" as *const u8
        as *const c_char,
    b"                n    non-moving GC heap census events\0" as *const u8
        as *const c_char,
    b"                p    par spark events (sampled)\0" as *const u8
        as *const c_char,
    b"                f    par spark events (full detail)\0" as *const u8
        as *const c_char,
    b"                u    user events (emitted from Haskell code)\0" as *const u8
        as *const c_char,
    b"                T    ticky-ticky counter samples\0" as *const u8
        as *const c_char,
    b"                a    all event classes above\0" as *const u8
        as *const c_char,
    b"                t    add time stamps (only useful with -v)\0" as *const u8
        as *const c_char,
    b"               -x    disable an event class, for any flag above\0" as *const u8
        as *const c_char,
    b"             the initial enabled event classes are 'sgpu'\0" as *const u8
        as *const c_char,
    b" --eventlog-flush-interval=<secs>\0" as *const u8 as *const c_char,
    b"             Periodically flush the eventlog at the specified interval.\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -r<file>  Produce ticky-ticky statistics (with -rstderr for stderr)\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -C<secs>  Context-switch interval in seconds.\0" as *const u8
        as *const c_char,
    b"            0 or no argument means switch as often as possible.\0" as *const u8
        as *const c_char,
    b"            Default: 0.02 sec.\0" as *const u8 as *const c_char,
    b"  -V<secs>  Master tick interval in seconds (0 == disable timer).\0" as *const u8
        as *const c_char,
    b"            This sets the resolution for -C and the heap profile timer -i,\0"
        as *const u8 as *const c_char,
    b"            and is the frequency of time profile samples.\0" as *const u8
        as *const c_char,
    b"            Default: 0.01 sec.\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -Ds  DEBUG: scheduler\0" as *const u8 as *const c_char,
    b"  -Di  DEBUG: interpreter\0" as *const u8 as *const c_char,
    b"  -Dw  DEBUG: weak\0" as *const u8 as *const c_char,
    b"  -DG  DEBUG: gccafs\0" as *const u8 as *const c_char,
    b"  -Dg  DEBUG: gc\0" as *const u8 as *const c_char,
    b"  -Dn  DEBUG: non-moving gc\0" as *const u8 as *const c_char,
    b"  -Db  DEBUG: block\0" as *const u8 as *const c_char,
    b"  -DS  DEBUG: sanity\0" as *const u8 as *const c_char,
    b"  -DZ  DEBUG: zero freed memory during GC\0" as *const u8
        as *const c_char,
    b"  -Dt  DEBUG: stable\0" as *const u8 as *const c_char,
    b"  -Dp  DEBUG: prof\0" as *const u8 as *const c_char,
    b"  -Da  DEBUG: apply\0" as *const u8 as *const c_char,
    b"  -Dl  DEBUG: linker\0" as *const u8 as *const c_char,
    b"  -DL  DEBUG: linker (verbose)\0" as *const u8 as *const c_char,
    b"  -Dm  DEBUG: stm\0" as *const u8 as *const c_char,
    b"  -Dz  DEBUG: stack squeezing\0" as *const u8 as *const c_char,
    b"  -Dc  DEBUG: program coverage\0" as *const u8 as *const c_char,
    b"  -Dr  DEBUG: sparks\0" as *const u8 as *const c_char,
    b"  -DC  DEBUG: compact\0" as *const u8 as *const c_char,
    b"  -Dk  DEBUG: continuation\0" as *const u8 as *const c_char,
    b"  -Do  DEBUG: iomanager\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"     NOTE: DEBUG events are sent to stderr by default; add -l to create a\0"
        as *const u8 as *const c_char,
    b"     binary event log file instead.\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  --install-signal-handlers=<yes|no>\0" as *const u8 as *const c_char,
    b"             Install signal handlers (default: yes)\0" as *const u8
        as *const c_char,
    b"  --io-manager=<name>\0" as *const u8 as *const c_char,
    b"             The I/O manager to use.\0" as *const u8 as *const c_char,
    b"             Options available: auto select (default: select)\0" as *const u8
        as *const c_char,
    b"  -xq        The allocation limit given to a thread after it receives\0"
        as *const u8 as *const c_char,
    b"             an AllocationLimitExceeded exception. (default: 100k)\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -xr        The size of virtual memory address space reserved by the\0"
        as *const u8 as *const c_char,
    b"             two step allocator (default: 1T)\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  -Mgrace=<n>\0" as *const u8 as *const c_char,
    b"             The amount of allocation after the program receives a\0" as *const u8
        as *const c_char,
    b"             HeapOverflow exception before the exception is thrown again, if\0"
        as *const u8 as *const c_char,
    b"             the program is still exceeding the heap limit.\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  --read-tix-file=<yes|no>\0" as *const u8 as *const c_char,
    b"             Whether to initialize HPC datastructures from  <program>.tix              at the start of execution. (default: yes)\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"  --write-tix-file=<yes|no>\0" as *const u8 as *const c_char,
    b"             Whether to write <program>.tix at the end of execution.\0"
        as *const u8 as *const c_char,
    b"             (default: yes)\0" as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"RTS options may also be specified using the GHCRTS environment variable.\0"
        as *const u8 as *const c_char,
    b"\0" as *const u8 as *const c_char,
    b"Other RTS options may be available for programs compiled a different way.\0"
        as *const u8 as *const c_char,
    b"The GHC User's Guide has full details.\0" as *const u8
        as *const c_char,
    b"\0" as *const u8 as *const c_char,
    null::<c_char>(),
];

unsafe fn strequal(mut a: *const c_char, mut b: *const c_char) -> bool {
    return strcmp(a, b) == 0 as c_int;
}

unsafe fn appendRtsArg(mut arg: *mut c_char) {
    if rts_argc == rts_argv_size {
        rts_argv_size *= 2 as c_int;

        rts_argv = stgReallocBytes(
            rts_argv as *mut c_void,
            (rts_argv_size as size_t).wrapping_mul(size_of::<*mut c_char>() as size_t),
            b"RtsFlags.c:appendRtsArg\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut *mut c_char;
    }

    let fresh12 = rts_argc;
    rts_argc = rts_argc + 1;

    let ref mut fresh13 = *rts_argv.offset(fresh12 as isize);
    *fresh13 = arg;
}

unsafe fn splitRtsFlags(mut s: *const c_char) {
    let mut c1 = null::<c_char>();
    let mut c2 = null::<c_char>();
    let mut t = null_mut::<c_char>();
    c1 = s;

    loop {
        while isspace(*c1 as c_int) != 0 {
            c1 = c1.offset(1);
        }

        c2 = c1;

        while isspace(*c2 as c_int) == 0 && *c2 as c_int != '\0' as i32 {
            c2 = c2.offset(1);
        }

        if c1 == c2 {
            break;
        }

        t = stgMallocBytes(
            (c2.offset_from(c1) as c_long + 1 as c_long) as size_t,
            b"RtsFlags.c:splitRtsFlags()\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut c_char;

        strncpy(t, c1, c2.offset_from(c1) as c_long as size_t);
        *t.offset(c2.offset_from(c1) as c_long as isize) = '\0' as i32 as c_char;
        appendRtsArg(t);
        c1 = c2;

        if !(*c1 as c_int != '\0' as i32) {
            break;
        }
    }
}

unsafe fn errorRtsOptsDisabled(mut s: *const c_char) {
    let mut advice = null_mut::<c_char>();

    if rtsConfig.rts_hs_main != 0 {
        advice =
            b"Link with -rtsopts to enable them.\0" as *const u8 as *const c_char as *mut c_char;
    } else {
        advice = b"Use hs_init_with_rtsopts() to enable them.\0" as *const u8 as *const c_char
            as *mut c_char;
    }

    errorBelch(s, advice);
}

unsafe fn setupRtsFlags(
    mut argc: *mut c_int,
    mut argv: *mut *mut c_char,
    mut rts_config: RtsConfig,
) {
    let mut mode: uint32_t = 0;
    let mut total_arg: uint32_t = 0;
    let mut arg: uint32_t = 0;
    let mut rts_argc0: uint32_t = 0;
    rtsConfig = rts_config;
    setProgName(argv);
    total_arg = *argc as uint32_t;
    arg = 1 as uint32_t;

    if *argc > 1 as c_int {
        *argc = 1 as c_int;
    }

    rts_argc = 0 as c_int;
    rts_argv_size = total_arg.wrapping_add(1 as uint32_t) as c_int;

    rts_argv = stgMallocBytes(
        (rts_argv_size as size_t).wrapping_mul(size_of::<*mut c_char>() as size_t),
        b"setupRtsFlags\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut c_char;

    rts_argc0 = rts_argc as uint32_t;

    if !rtsConfig.rts_opts.is_null() {
        splitRtsFlags(rtsConfig.rts_opts);
        procRtsOpts(rts_argc0 as c_int, RtsOptsAll);
        rts_argc0 = rts_argc as uint32_t;
    }

    if rtsConfig.rts_opts_enabled as c_uint != RtsOptsIgnoreAll as c_int as c_uint {
        let mut ghc_rts = getenv(b"GHCRTS\0" as *const u8 as *const c_char);

        if !ghc_rts.is_null() {
            if rtsConfig.rts_opts_enabled as c_uint == RtsOptsNone as c_int as c_uint {
                errorRtsOptsDisabled(
                    b"Warning: Ignoring GHCRTS variable as RTS options are disabled.\n         %s\0"
                        as *const u8 as *const c_char,
                );
            } else {
                splitRtsFlags(ghc_rts);
                procRtsOpts(rts_argc0 as c_int, rtsConfig.rts_opts_enabled);
                rts_argc0 = rts_argc as uint32_t;
            }
        }
    }

    if !(rtsConfig.rts_opts_enabled as c_uint == RtsOptsIgnoreAll as c_int as c_uint
        || rtsConfig.rts_opts_enabled as c_uint == RtsOptsIgnore as c_int as c_uint)
    {
        mode = PGM as uint32_t;

        while arg < total_arg {
            if strequal(
                b"--RTS\0" as *const u8 as *const c_char,
                *argv.offset(arg as isize),
            ) {
                arg = arg.wrapping_add(1);
                break;
            } else {
                if strequal(
                    b"--\0" as *const u8 as *const c_char,
                    *argv.offset(arg as isize),
                ) {
                    break;
                }

                if strequal(
                    b"+RTS\0" as *const u8 as *const c_char,
                    *argv.offset(arg as isize),
                ) {
                    mode = RTS as uint32_t;
                } else if strequal(
                    b"-RTS\0" as *const u8 as *const c_char,
                    *argv.offset(arg as isize),
                ) {
                    mode = PGM as uint32_t;
                } else if mode == RTS as uint32_t {
                    appendRtsArg(copyArg(*argv.offset(arg as isize)));
                } else {
                    let fresh7 = *argc;
                    *argc = *argc + 1;

                    let ref mut fresh8 = *argv.offset(fresh7 as isize);
                    *fresh8 = *argv.offset(arg as isize);
                }

                arg = arg.wrapping_add(1);
            }
        }
    }

    while arg < total_arg {
        let fresh9 = *argc;
        *argc = *argc + 1;

        let ref mut fresh10 = *argv.offset(fresh9 as isize);
        *fresh10 = *argv.offset(arg as isize);
        arg = arg.wrapping_add(1);
    }

    let ref mut fresh11 = *argv.offset(*argc as isize);
    *fresh11 = null_mut::<c_char>();
    procRtsOpts(rts_argc0 as c_int, rtsConfig.rts_opts_enabled);
    appendRtsArg(null_mut::<c_char>());
    rts_argc -= 1;
    normaliseRtsOpts();
    setProgArgv(*argc, argv);

    if !RtsFlags.GcFlags.statsFile.is_null() {
        initStatsFile(RtsFlags.GcFlags.statsFile);
    }

    if !RtsFlags.TickyFlags.tickyFile.is_null() {
        initStatsFile(RtsFlags.TickyFlags.tickyFile);
    }
}

unsafe fn checkSuid(mut enabled: RtsOptsEnabledEnum) {
    if enabled as c_uint == RtsOptsSafeOnly as c_int as c_uint {
        if getuid() != geteuid() || getgid() != getegid() {
            errorRtsOptsDisabled(
                b"RTS options are disabled for setuid binaries. %s\0" as *const u8 as *const c_char,
            );

            stg_exit(EXIT_FAILURE);
        }
    }
}

unsafe fn checkUnsafe(mut enabled: RtsOptsEnabledEnum) {
    if enabled as c_uint == RtsOptsSafeOnly as c_int as c_uint {
        errorRtsOptsDisabled(b"Most RTS options are disabled. %s\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn procRtsOpts(mut rts_argc0: c_int, mut rtsOptsEnabled: RtsOptsEnabledEnum) {
    let mut current_block: u64;
    let mut error = r#false != 0;
    let mut arg: c_int = 0;
    let mut unchecked_arg_start: c_int = 0;

    if !(rts_argc0 < rts_argc) {
        return;
    }

    if rtsOptsEnabled as c_uint == RtsOptsNone as c_int as c_uint {
        errorRtsOptsDisabled(b"RTS options are disabled. %s\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }

    checkSuid(rtsOptsEnabled);
    arg = rts_argc0;

    while arg < rts_argc {
        let mut option_checked = r#false != 0;

        if *(*rts_argv.offset(arg as isize)).offset(0 as c_int as isize) as c_int != '-' as i32 {
            fflush(__stdoutp);

            errorBelch(
                b"unexpected RTS argument: %s\0" as *const u8 as *const c_char,
                *rts_argv.offset(arg as isize),
            );

            error = r#true != 0;
        } else {
            unchecked_arg_start = 1 as c_int;

            match *(*rts_argv.offset(arg as isize)).offset(1 as c_int as isize) as c_int {
                63 => {
                    option_checked = r#true != 0;
                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                45 => {
                    if strequal(
                        b"install-signal-handlers=yes\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.install_signal_handlers = r#true != 0;
                    } else if strequal(
                        b"install-signal-handlers=no\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.install_signal_handlers = r#false != 0;
                    } else if strequal(
                        b"install-seh-handlers=yes\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.install_seh_handlers = r#true != 0;
                    } else if strequal(
                        b"install-seh-handlers=no\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.install_seh_handlers = r#false != 0;
                    } else if strequal(
                        b"generate-stack-traces=yes\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.generate_stack_trace = r#true != 0;
                    } else if strequal(
                        b"generate-stack-traces=no\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.generate_stack_trace = r#false != 0;
                    } else if strequal(
                        b"generate-crash-dumps\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.generate_dump_file = r#true != 0;
                    } else if strequal(
                        b"optimistic-linking\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.linkerOptimistic = r#true != 0;
                    } else if strequal(
                        b"null-eventlog-writer\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.TraceFlags.nullWriter = r#true != 0;
                    } else if strequal(
                        b"machine-readable\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.machineReadable = r#true != 0;
                    } else if strequal(
                        b"disable-delayed-os-memory-return\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.disableDelayedOsMemoryReturn = r#true != 0;
                    } else if strequal(
                        b"internal-counters\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.MiscFlags.internalCounters = r#true != 0;
                    } else if strncmp(
                        b"io-manager=\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                        11 as size_t,
                    ) == 0
                    {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;

                        let mut iomgrstr: *mut c_char = (*rts_argv.offset(arg as isize))
                            .offset(13 as c_int as isize)
                            as *mut c_char;

                        let mut iomgrflag = IO_MNGR_FLAG_AUTO;
                        let mut availability = IOManagerAvailable;
                        availability = parseIOManagerFlag(iomgrstr, &raw mut iomgrflag);

                        if availability as c_uint == IOManagerAvailable as c_int as c_uint {
                            RtsFlags.MiscFlags.ioManager = iomgrflag;
                        } else {
                            errorBelch(
                                b"%s choice '%s' for --io-manager=\nThe choices are: auto%s\0"
                                    as *const u8 as *const c_char,
                                if availability as c_uint == IOManagerUnavailable as c_int as c_uint
                                {
                                    b"unavailable\0" as *const u8 as *const c_char
                                } else {
                                    b"unrecognised\0" as *const u8 as *const c_char
                                },
                                iomgrstr,
                                b" select\0" as *const u8 as *const c_char,
                            );

                            stg_exit(EXIT_FAILURE);
                        }
                    } else if strequal(
                        b"info\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        printRtsInfo(rtsConfig);
                        stg_exit(0 as c_int);
                    } else if strncmp(
                        b"eventlog-flush-interval=\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                        24 as size_t,
                    ) == 0
                    {
                        option_checked = r#true != 0;

                        let mut intervalSeconds = parseDouble(
                            (*rts_argv.offset(arg as isize)).offset(26 as c_int as isize),
                            &raw mut error,
                        );

                        if error {
                            errorBelch(
                                b"bad value for --eventlog-flush-interval\0" as *const u8
                                    as *const c_char,
                            );
                        }

                        RtsFlags.TraceFlags.eventlogFlushTime = fsecondsToTime(intervalSeconds);
                    } else if strequal(
                        b"copying-gc\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.GcFlags.useNonmoving = r#false != 0;
                    } else if strequal(
                        b"nonmoving-gc\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.GcFlags.useNonmoving = r#true != 0;
                    } else if strncmp(
                        b"nonmoving-dense-allocator-count=\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                        32 as size_t,
                    ) == 0
                    {
                        option_checked = r#true != 0;

                        let mut threshold: int32_t = strtol(
                            (*rts_argv.offset(arg as isize)).offset(34 as c_int as isize),
                            NULL as *mut *mut c_char,
                            10 as c_int,
                        ) as int32_t;

                        if threshold < 1 as int32_t
                            || threshold > -(1 as c_int) as uint16_t as int32_t
                        {
                            errorBelch(
                                b"bad value for --nonmoving-dense-allocator-count\0" as *const u8
                                    as *const c_char,
                            );

                            error = r#true != 0;
                        } else {
                            RtsFlags.GcFlags.nonmovingDenseAllocatorCount = threshold as uint16_t;
                        }
                    } else if strequal(
                        b"read-tix-file=yes\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.HpcFlags.readTixFile = HPC_YES_EXPLICIT;
                    } else if strequal(
                        b"read-tix-file=no\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.HpcFlags.readTixFile = HPC_NO_EXPLICIT;
                    } else if strequal(
                        b"write-tix-file=yes\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.HpcFlags.writeTixFile = r#true != 0;
                    } else if strequal(
                        b"write-tix-file=no\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.HpcFlags.writeTixFile = r#false != 0;
                    } else if strncmp(
                        b"long-gc-sync=\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                        13 as size_t,
                    ) == 0
                    {
                        option_checked = r#true != 0;

                        if !(*(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                            == '\0' as i32)
                        {
                            RtsFlags.GcFlags.longGCSync = fsecondsToTime(atof(
                                (*rts_argv.offset(arg as isize)).offset(16 as c_int as isize),
                            ));
                        }
                    } else if strequal(
                        b"no-automatic-heap-samples\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.ProfFlags.startHeapProfileAtStartup = r#false != 0;
                    } else if strequal(
                        b"no-automatic-time-samples\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.ProfFlags.startTimeProfileAtStartup = r#false != 0;
                    } else if strequal(
                        b"automatic-era-increment\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char,
                    ) {
                        option_checked = r#true != 0;
                        RtsFlags.ProfFlags.incrementUserEra = r#true != 0;
                    } else {
                        option_checked = r#true != 0;

                        errorBelch(
                            b"unknown RTS option: %s\0" as *const u8 as *const c_char,
                            *rts_argv.offset(arg as isize),
                        );

                        error = r#true != 0;
                    }

                    current_block = 6501678289274187771;
                }
                65 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        == 'L' as i32
                    {
                        RtsFlags.GcFlags.largeAllocLim = decodeSize(
                            *rts_argv.offset(arg as isize),
                            3 as uint32_t,
                            (2 as c_ulong).wrapping_mul(BLOCK_SIZE) as StgWord64,
                            HS_INT_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as uint32_t;
                    } else {
                        RtsFlags.GcFlags.minAllocAreaSize = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as uint32_t,
                            (2 as c_ulong).wrapping_mul(BLOCK_SIZE) as StgWord64,
                            HS_INT_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as uint32_t;
                    }

                    current_block = 6501678289274187771;
                }
                110 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    RtsFlags.GcFlags.nurseryChunkSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as uint32_t,
                        (2 as c_ulong).wrapping_mul(BLOCK_SIZE) as StgWord64,
                        HS_INT_MAX as StgWord64,
                    )
                    .wrapping_div(BLOCK_SIZE as StgWord64)
                        as uint32_t;
                    current_block = 6501678289274187771;
                }
                66 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.ringBell = r#true != 0;
                    unchecked_arg_start += 1;
                    current_block = 166025003974853318;
                }
                99 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        != '\0' as i32
                    {
                        RtsFlags.GcFlags.compactThreshold =
                            atof((*rts_argv.offset(arg as isize)).offset(2 as c_int as isize));
                    } else {
                        RtsFlags.GcFlags.compact = r#true != 0;
                    }

                    current_block = 6501678289274187771;
                }
                119 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.sweep = r#true != 0;
                    unchecked_arg_start += 1;
                    current_block = 166025003974853318;
                }
                70 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        100 => {
                            RtsFlags.GcFlags.returnDecayFactor =
                                atof((*rts_argv.offset(arg as isize)).offset(3 as c_int as isize));

                            if RtsFlags.GcFlags.returnDecayFactor < 0 as c_int as c_double {
                                bad_option(*rts_argv.offset(arg as isize));
                            }
                        }
                        _ => {
                            RtsFlags.GcFlags.oldGenFactor =
                                atof((*rts_argv.offset(arg as isize)).offset(2 as c_int as isize));

                            if RtsFlags.GcFlags.oldGenFactor < 0 as c_int as c_double {
                                bad_option(*rts_argv.offset(arg as isize));
                            }
                        }
                    }

                    current_block = 6501678289274187771;
                }
                68 => {
                    option_checked = r#true != 0;
                    read_debug_flags(*rts_argv.offset(arg as isize));
                    current_block = 6501678289274187771;
                }
                75 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    RtsFlags.GcFlags.maxStkSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as uint32_t,
                        0 as StgWord64,
                        UINT32_MAX as StgWord64,
                    )
                    .wrapping_div(size_of::<W_>() as StgWord64)
                        as uint32_t;
                    current_block = 6501678289274187771;
                }
                107 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        99 => {
                            RtsFlags.GcFlags.stkChunkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as uint32_t,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as uint32_t;
                        }
                        98 => {
                            RtsFlags.GcFlags.stkChunkBufferSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as uint32_t,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as uint32_t;
                        }
                        105 => {
                            RtsFlags.GcFlags.initialStkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as uint32_t,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as uint32_t;
                        }
                        _ => {
                            RtsFlags.GcFlags.initialStkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                2 as uint32_t,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as uint32_t;
                        }
                    }

                    current_block = 6501678289274187771;
                }
                77 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if 0 as c_int
                        == strncmp(
                            b"grace=\0" as *const u8 as *const c_char,
                            (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                            6 as size_t,
                        )
                    {
                        RtsFlags.GcFlags.heapLimitGrace = decodeSize(
                            *rts_argv.offset(arg as isize),
                            8 as uint32_t,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        ) as StgWord;
                    } else {
                        RtsFlags.GcFlags.maxHeapSize = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as uint32_t,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as uint32_t;
                    }

                    current_block = 6501678289274187771;
                }
                109 => {
                    if strncmp(
                        b"maxN\0" as *const u8 as *const c_char,
                        (*rts_argv.offset(arg as isize)).offset(1 as c_int as isize) as *mut c_char,
                        4 as size_t,
                    ) == 0 as c_int
                    {
                        option_checked = r#true != 0;

                        errorBelch(
                            b"the flag %s requires the program to be built with -threaded\0"
                                as *const u8 as *const c_char,
                            *rts_argv.offset(arg as isize),
                        );

                        error = r#true != 0;
                    } else {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                        RtsFlags.GcFlags.pcFreeHeap =
                            atof((*rts_argv.offset(arg as isize)).offset(2 as c_int as isize));

                        if RtsFlags.GcFlags.pcFreeHeap == 0.0f64
                            && *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize)
                                as c_int
                                != '0' as i32
                        {
                            bad_option(*rts_argv.offset(arg as isize));
                        }

                        if RtsFlags.GcFlags.pcFreeHeap < 0 as c_int as c_double
                            || RtsFlags.GcFlags.pcFreeHeap > 100 as c_int as c_double
                        {
                            bad_option(*rts_argv.offset(arg as isize));
                        }
                    }

                    current_block = 6501678289274187771;
                }
                71 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    RtsFlags.GcFlags.generations = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as uint32_t,
                        1 as StgWord64,
                        HS_INT_MAX as StgWord64,
                    ) as uint32_t;

                    current_block = 6501678289274187771;
                }
                72 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        == '\0' as i32
                    {
                        RtsFlags.GcFlags.heapSizeSuggestionAuto = r#true != 0;
                    } else {
                        RtsFlags.GcFlags.heapSizeSuggestion = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as uint32_t,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as uint32_t;
                    }

                    current_block = 6501678289274187771;
                }
                79 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    RtsFlags.GcFlags.minOldGenSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as uint32_t,
                        BLOCK_SIZE as StgWord64,
                        HS_WORD_MAX as StgWord64,
                    )
                    .wrapping_div(BLOCK_SIZE as StgWord64)
                        as uint32_t;
                    current_block = 6501678289274187771;
                }
                73 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        119 => {
                            if !(*(*rts_argv.offset(arg as isize)).offset(3 as c_int as isize)
                                as c_int
                                == '\0' as i32)
                            {
                                RtsFlags.GcFlags.interIdleGCWait = fsecondsToTime(atof(
                                    (*rts_argv.offset(arg as isize)).offset(3 as c_int as isize),
                                ));
                            }
                        }
                        0 => {}
                        _ => {
                            let mut t = fsecondsToTime(atof(
                                (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                            ));

                            if t == 0 as Time {
                                RtsFlags.GcFlags.doIdleGC = r#false != 0;
                            } else {
                                RtsFlags.GcFlags.doIdleGC = r#true != 0;
                                RtsFlags.GcFlags.idleGCDelayTime = t;
                            }
                        }
                    }

                    current_block = 6501678289274187771;
                }
                84 => {
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.giveStats = COLLECT_GC_STATS as uint32_t;
                    unchecked_arg_start += 1;
                    current_block = 166025003974853318;
                }
                83 => {
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.giveStats = VERBOSE_GC_STATS as uint32_t;
                    current_block = 8446064538627958008;
                }
                115 => {
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.giveStats = SUMMARY_GC_STATS as uint32_t;
                    current_block = 8446064538627958008;
                }
                116 => {
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.giveStats = ONELINE_GC_STATS as uint32_t;
                    current_block = 8446064538627958008;
                }
                90 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;
                    RtsFlags.GcFlags.squeezeUpdFrames = r#false != 0;
                    unchecked_arg_start += 1;
                    current_block = 166025003974853318;
                }
                80 | 112 => {
                    option_checked = r#true != 0;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        111 => {
                            if *(*rts_argv.offset(arg as isize)).offset(3 as c_int as isize)
                                as c_int
                                == '\0' as i32
                            {
                                errorBelch(
                                    b"flag -po expects an argument\0" as *const u8 as *const c_char,
                                );

                                error = r#true != 0;
                            } else {
                                RtsFlags.CcFlags.outputFileNameStem =
                                    (*rts_argv.offset(arg as isize)).offset(3 as c_int as isize);
                            }
                        }
                        _ => {
                            errorBelch(
                                b"the flag %s requires the program to be built with -prof\0"
                                    as *const u8 as *const c_char,
                                *rts_argv.offset(arg as isize),
                            );

                            error = r#true != 0;
                        }
                    }

                    current_block = 6501678289274187771;
                }
                82 => {
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -prof\0" as *const u8
                            as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                76 => {
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -prof\0" as *const u8
                            as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                104 => {
                    let mut current_block_321: u64;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        0 => {
                            errorBelch(
                                b"-h is deprecated, use -hT instead.\0" as *const u8
                                    as *const c_char,
                            );

                            current_block_321 = 4347121995186969965;
                        }
                        84 => {
                            current_block_321 = 4347121995186969965;
                        }
                        105 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = r#true != 0;
                            RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_INFO_TABLE as uint32_t;
                            current_block_321 = 9260825484694736987;
                        }
                        _ => {
                            option_checked = r#true != 0;

                            errorBelch(
                                b"the flag %s requires the program to be built with -prof\0"
                                    as *const u8 as *const c_char,
                                *rts_argv.offset(arg as isize),
                            );

                            error = r#true != 0;
                            current_block_321 = 9260825484694736987;
                        }
                    }

                    match current_block_321 {
                        4347121995186969965 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = r#true != 0;
                            RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_CLOSURE_TYPE as uint32_t;
                        }
                        _ => {}
                    }

                    current_block = 6501678289274187771;
                }
                105 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if !(*(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        == '\0' as i32)
                    {
                        let mut intervalSeconds_0 = parseDouble(
                            (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                            &raw mut error,
                        );

                        if error {
                            errorBelch(b"bad value for -i\0" as *const u8 as *const c_char);
                        }

                        RtsFlags.ProfFlags.heapProfileInterval = fsecondsToTime(intervalSeconds_0);
                    }

                    current_block = 6501678289274187771;
                }
                67 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        == '\0' as i32
                    {
                        RtsFlags.ConcFlags.ctxtSwitchTime = 0 as Time;
                    } else {
                        let mut intervalSeconds_1 = parseDouble(
                            (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                            &raw mut error,
                        );

                        if error {
                            errorBelch(b"bad value for -C\0" as *const u8 as *const c_char);
                        }

                        RtsFlags.ConcFlags.ctxtSwitchTime = fsecondsToTime(intervalSeconds_1);
                    }

                    current_block = 6501678289274187771;
                }
                86 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        == '\0' as i32
                    {
                        RtsFlags.MiscFlags.tickInterval = 0 as Time;
                    } else {
                        let mut intervalSeconds_2 = parseDouble(
                            (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                            &raw mut error,
                        );

                        if error {
                            errorBelch(b"bad value for -V\0" as *const u8 as *const c_char);
                        }

                        RtsFlags.MiscFlags.tickInterval = fsecondsToTime(intervalSeconds_2);
                    }

                    current_block = 6501678289274187771;
                }
                78 => {
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -threaded\0"
                            as *const u8 as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                103 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -threaded\0"
                            as *const u8 as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                113 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -threaded\0"
                            as *const u8 as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                101 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = r#true != 0;

                    errorBelch(
                        b"the flag %s requires the program to be built with -threaded\0"
                            as *const u8 as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
                114 => {
                    option_checked = r#true != 0;
                    RtsFlags.TickyFlags.showTickyStats = 1 as c_int != 0;

                    let mut r_0: c_int = 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        != '\0' as i32
                    {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = 1 as c_int != 0;
                    }

                    r_0 = openStatsFile(
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                        b"%0.121s.ticky\0" as *const u8 as *const c_char,
                        &raw mut RtsFlags.TickyFlags.tickyFile,
                    );

                    if r_0 == -(1 as c_int) {
                        error = 1 as c_int != 0;
                    }

                    current_block = 6501678289274187771;
                }
                111 => {
                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        108 => {
                            option_checked = r#true != 0;

                            if strlen((*rts_argv.offset(arg as isize)).offset(3 as c_int as isize)
                                as *mut c_char)
                                == 0 as size_t
                            {
                                errorBelch(b"-ol expects filename\0" as *const u8 as *const c_char);
                                error = 1 as c_int != 0;
                            } else {
                                RtsFlags.TraceFlags.trace_output = strdup(
                                    (*rts_argv.offset(arg as isize)).offset(3 as c_int as isize)
                                        as *mut c_char,
                                );
                            }
                        }
                        _ => {
                            errorBelch(
                                b"Unknown output flag -o%c\0" as *const u8 as *const c_char,
                                *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize)
                                    as c_int,
                            );

                            error = r#true != 0;
                        }
                    }

                    current_block = 6501678289274187771;
                }
                108 => {
                    option_checked = r#true != 0;
                    RtsFlags.TraceFlags.tracing = 1 as c_int;

                    read_trace_flags(
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char
                    );

                    current_block = 6501678289274187771;
                }
                118 => {
                    option_checked = r#true != 0;
                    RtsFlags.TraceFlags.tracing = 2 as c_int;

                    read_trace_flags(
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as *mut c_char
                    );

                    current_block = 6501678289274187771;
                }
                120 => {
                    unchecked_arg_start += 1;

                    match *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int {
                        0 => {
                            option_checked = r#true != 0;

                            errorBelch(
                                b"incomplete RTS option: %s\0" as *const u8 as *const c_char,
                                *rts_argv.offset(arg as isize),
                            );

                            error = r#true != 0;
                            current_block = 6501678289274187771;
                        }
                        98 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = r#true != 0;

                            if *(*rts_argv.offset(arg as isize)).offset(3 as c_int as isize)
                                as c_int
                                != '\0' as i32
                            {
                                RtsFlags.GcFlags.heapBase = strtoull(
                                    (*rts_argv.offset(arg as isize)).offset(3 as c_int as isize),
                                    NULL as *mut *mut c_char,
                                    0 as c_int,
                                )
                                    as StgWord;
                            } else {
                                errorBelch(
                                    b"-xb: requires argument\0" as *const u8 as *const c_char,
                                );

                                error = r#true != 0;
                            }

                            current_block = 6501678289274187771;
                        }
                        110 => {
                            option_checked = r#true != 0;
                            RtsFlags.GcFlags.useNonmoving = r#true != 0;
                            unchecked_arg_start += 1;
                            current_block = 6501678289274187771;
                        }
                        99 => {
                            option_checked = r#true != 0;

                            errorBelch(
                                b"the flag %s requires the program to be built with -prof\0"
                                    as *const u8 as *const c_char,
                                *rts_argv.offset(arg as isize),
                            );

                            error = r#true != 0;
                            unchecked_arg_start += 1;
                            current_block = 166025003974853318;
                        }
                        116 => {
                            option_checked = r#true != 0;

                            errorBelch(
                                b"The -xt option has been removed (#16795)\0" as *const u8
                                    as *const c_char,
                            );

                            error = r#true != 0;
                            current_block = 6501678289274187771;
                        }
                        113 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = r#true != 0;

                            RtsFlags.GcFlags.allocLimitGrace = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as uint32_t,
                                BLOCK_SIZE as StgWord64,
                                HS_INT_MAX as StgWord64,
                            )
                            .wrapping_div(BLOCK_SIZE as StgWord64)
                                as StgWord;
                            current_block = 6501678289274187771;
                        }
                        114 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = r#true != 0;

                            RtsFlags.GcFlags.addressSpaceSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as uint32_t,
                                MBLOCK_SIZE as StgWord64,
                                HS_WORD64_MAX as StgWord64,
                            );

                            current_block = 6501678289274187771;
                        }
                        _ => {
                            option_checked = r#true != 0;

                            errorBelch(
                                b"unknown RTS option: %s\0" as *const u8 as *const c_char,
                                *rts_argv.offset(arg as isize),
                            );

                            error = r#true != 0;
                            current_block = 6501678289274187771;
                        }
                    }
                }
                _ => {
                    option_checked = r#true != 0;

                    errorBelch(
                        b"unknown RTS option: %s\0" as *const u8 as *const c_char,
                        *rts_argv.offset(arg as isize),
                    );

                    error = r#true != 0;
                    current_block = 6501678289274187771;
                }
            }

            match current_block {
                166025003974853318 => {
                    if *(*rts_argv.offset(arg as isize)).offset(unchecked_arg_start as isize)
                        as c_int
                        != '\0' as i32
                    {
                        errorBelch(
                            b"flag -%c given an argument when none was expected: %s\0" as *const u8
                                as *const c_char,
                            *(*rts_argv.offset(arg as isize)).offset(1 as c_int as isize) as c_int,
                            *rts_argv.offset(arg as isize),
                        );

                        error = r#true != 0;
                    }
                }
                8446064538627958008 => {
                    let mut r: c_int = 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2 as c_int as isize) as c_int
                        != '\0' as i32
                    {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = r#true != 0;
                    }

                    r = openStatsFile(
                        (*rts_argv.offset(arg as isize)).offset(2 as c_int as isize),
                        null::<c_char>(),
                        &raw mut RtsFlags.GcFlags.statsFile,
                    );

                    if r == -(1 as c_int) {
                        error = r#true != 0;
                    }
                }
                _ => {}
            }

            if !option_checked {
                errorBelch(
                    b"Internal error in the RTS options parser\0" as *const u8 as *const c_char,
                );

                stg_exit(EXIT_FAILURE);
            }
        }

        arg += 1;
    }

    if error {
        errorUsage();
    }
}

unsafe fn normaliseRtsOpts() {
    if RtsFlags.MiscFlags.tickInterval < 0 as Time {
        RtsFlags.MiscFlags.tickInterval = DEFAULT_TICK_INTERVAL;
    }

    if RtsFlags.MiscFlags.tickInterval == 0 as Time {
        RtsFlags.ConcFlags.ctxtSwitchTime = 0 as Time;
        RtsFlags.GcFlags.idleGCDelayTime = 0 as Time;
        RtsFlags.ProfFlags.heapProfileInterval = 0 as Time;
    }

    if RtsFlags.ConcFlags.ctxtSwitchTime > 0 as Time {
        RtsFlags.MiscFlags.tickInterval = ({
            let mut _a: Time = RtsFlags.ConcFlags.ctxtSwitchTime as Time;
            let mut _b: Time = RtsFlags.MiscFlags.tickInterval as Time;

            if _a <= _b { _a } else { _b as Time }
        });
    }

    if RtsFlags.GcFlags.idleGCDelayTime > 0 as Time {
        RtsFlags.MiscFlags.tickInterval = ({
            let mut _a: Time = RtsFlags.GcFlags.idleGCDelayTime as Time;
            let mut _b: Time = RtsFlags.MiscFlags.tickInterval as Time;

            if _a <= _b { _a } else { _b as Time }
        });
    }

    if RtsFlags.ProfFlags.heapProfileInterval > 0 as Time {
        RtsFlags.MiscFlags.tickInterval = ({
            let mut _a: Time = RtsFlags.ProfFlags.heapProfileInterval as Time;
            let mut _b: Time = RtsFlags.MiscFlags.tickInterval as Time;

            if _a <= _b { _a } else { _b as Time }
        });
    }

    if RtsFlags.ConcFlags.ctxtSwitchTime > 0 as Time && RtsFlags.MiscFlags.tickInterval != 0 as Time
    {
        RtsFlags.ConcFlags.ctxtSwitchTicks =
            (RtsFlags.ConcFlags.ctxtSwitchTime / RtsFlags.MiscFlags.tickInterval) as c_int;
    } else {
        RtsFlags.ConcFlags.ctxtSwitchTicks = 0 as c_int;
    }

    if RtsFlags.ProfFlags.heapProfileInterval > 0 as Time
        && RtsFlags.MiscFlags.tickInterval != 0 as Time
    {
        RtsFlags.ProfFlags.heapProfileIntervalTicks =
            (RtsFlags.ProfFlags.heapProfileInterval / RtsFlags.MiscFlags.tickInterval) as uint32_t;
    } else {
        RtsFlags.ProfFlags.heapProfileIntervalTicks = 0 as uint32_t;
    }

    if RtsFlags.TraceFlags.eventlogFlushTime > 0 as Time
        && RtsFlags.MiscFlags.tickInterval != 0 as Time
    {
        RtsFlags.TraceFlags.eventlogFlushTicks =
            (RtsFlags.TraceFlags.eventlogFlushTime / RtsFlags.MiscFlags.tickInterval) as c_int;
    } else {
        RtsFlags.TraceFlags.eventlogFlushTicks = 0 as c_int;
    }

    if RtsFlags.GcFlags.stkChunkBufferSize
        > RtsFlags.GcFlags.stkChunkSize.wrapping_div(2 as uint32_t)
    {
        errorBelch(
            b"stack chunk buffer size (-kb) must be less than 50%%\nof the stack chunk size (-kc)\0"
                as *const u8 as *const c_char,
        );

        errorUsage();
    }

    if RtsFlags.GcFlags.maxHeapSize != 0 as uint32_t
        && RtsFlags.GcFlags.heapSizeSuggestion > RtsFlags.GcFlags.maxHeapSize
    {
        errorBelch(
            b"Maximum heap size (-M) is smaller than suggested heap size (-H)\nSetting maximum heap size to suggested heap size ( %llu )\0"
                as *const u8 as *const c_char,
            (RtsFlags.GcFlags.maxHeapSize as StgWord64)
                .wrapping_mul(BLOCK_SIZE as StgWord64),
        );

        RtsFlags.GcFlags.maxHeapSize = RtsFlags.GcFlags.heapSizeSuggestion;
    }

    if RtsFlags.GcFlags.maxHeapSize != 0 as uint32_t
        && RtsFlags.GcFlags.minAllocAreaSize > RtsFlags.GcFlags.maxHeapSize
    {
        errorBelch(
            b"maximum heap size (-M) is smaller than minimum alloc area size (-A)\0" as *const u8
                as *const c_char,
        );

        RtsFlags.GcFlags.minAllocAreaSize = RtsFlags.GcFlags.maxHeapSize;
    }

    if RtsFlags.GcFlags.minAllocAreaSize as c_ulong
        >= ((16 as c_int * 1024 as c_int * 1024 as c_int) as c_ulong).wrapping_div(BLOCK_SIZE)
    {
        RtsFlags.GcFlags.nurseryChunkSize = ((4 as c_int * 1024 as c_int * 1024 as c_int)
            as c_ulong)
            .wrapping_div(BLOCK_SIZE) as uint32_t;
    }

    if RtsFlags.ParFlags.parGcLoadBalancingGen == !(0 as uint32_t) {
        let mut alloc_area_bytes: StgWord =
            (RtsFlags.GcFlags.minAllocAreaSize as c_ulong).wrapping_mul(BLOCK_SIZE) as StgWord;

        if alloc_area_bytes >= (32 as c_int * 1024 as c_int * 1024 as c_int) as StgWord {
            RtsFlags.ParFlags.parGcLoadBalancingGen = 0 as uint32_t;
        } else {
            RtsFlags.ParFlags.parGcLoadBalancingGen = 1 as uint32_t;
        }
    }

    if RtsFlags.MiscFlags.generate_dump_file {
        RtsFlags.MiscFlags.install_seh_handlers = r#true != 0;
    }

    if RtsFlags.GcFlags.useNonmoving as c_int != 0 && RtsFlags.GcFlags.generations == 1 as uint32_t
    {
        barf(b"The non-moving collector doesn't support -G1\0" as *const u8 as *const c_char);
    }

    if RtsFlags.GcFlags.compact as c_int != 0 && RtsFlags.GcFlags.useNonmoving as c_int != 0 {
        errorBelch(
            b"The non-moving collector cannot be used in conjunction with\nthe compacting collector.\0"
                as *const u8 as *const c_char,
        );

        errorUsage();
    }

    if RtsFlags.TraceFlags.ticky as c_int != 0 && RtsFlags.TickyFlags.showTickyStats as c_int != 0 {
        barf(
            b"The ticky-ticky eventlog output cannot be used in conjunction with\n+RTS -r<file>.\0"
                as *const u8 as *const c_char,
        );
    }
}

unsafe fn errorUsage() -> ! {
    let mut p = null_mut::<*const c_char>();
    fflush(__stdoutp);
    p = &raw mut usage_text as *mut *const c_char;

    while !(*p).is_null() {
        errorBelch(b"%s\0" as *const u8 as *const c_char, *p);
        p = p.offset(1);
    }

    stg_exit(EXIT_FAILURE);
}

unsafe fn stats_fprintf(mut f: *mut FILE, mut s: *mut c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    if f.is_null() {
        vdebugBelch(s, ap.as_va_list());
    } else {
        vfprintf(f, s, ap.as_va_list());
    };
}

unsafe fn openStatsFile(
    mut filename: *mut c_char,
    mut filename_fmt: *const c_char,
    mut file_ret: *mut *mut FILE,
) -> c_int {
    let mut f = null_mut::<FILE>();

    if strequal(filename, b"stderr\0" as *const u8 as *const c_char) as c_int != 0
        || filename_fmt.is_null() && *filename as c_int == '\0' as i32
    {
        f = null_mut::<FILE>();
    } else {
        if *filename as c_int != '\0' as i32 {
            f = __rts_fopen(filename, b"w+\0" as *const u8 as *const c_char);
        } else {
            if filename_fmt.is_null() {
                errorBelch(
                    b"Invalid stats filename format (NULL)\n\0" as *const u8 as *const c_char,
                );

                return -(1 as c_int);
            }

            let mut stats_filename: [c_char; 128] = [0; 128];

            snprintf(
                &raw mut stats_filename as *mut c_char,
                STATS_FILENAME_MAXLEN as size_t,
                filename_fmt,
                prog_name,
            );

            f = __rts_fopen(
                &raw mut stats_filename as *mut c_char,
                b"w+\0" as *const u8 as *const c_char,
            );
        }

        if f.is_null() {
            errorBelch(
                b"Can't open stats file %s\n\0" as *const u8 as *const c_char,
                filename,
            );

            return -(1 as c_int);
        }
    }

    *file_ret = f;

    return 0 as c_int;
}

unsafe fn stats_fprintf_escape(mut f: *mut FILE, mut s: *mut c_char) {
    stats_fprintf(f, b"'\0" as *const u8 as *const c_char as *mut c_char);

    while *s as c_int != '\0' as i32 {
        match *s as c_int {
            39 => {
                stats_fprintf(f, b"'\\''\0" as *const u8 as *const c_char as *mut c_char);
            }
            _ => {
                stats_fprintf(
                    f,
                    b"%c\0" as *const u8 as *const c_char as *mut c_char,
                    *s as c_int,
                );
            }
        }

        s = s.offset(1);
    }

    stats_fprintf(f, b"' \0" as *const u8 as *const c_char as *mut c_char);
}

unsafe fn initStatsFile(mut f: *mut FILE) {
    let mut count: c_int = 0;
    count = 0 as c_int;

    while count < prog_argc {
        stats_fprintf_escape(f, *prog_argv.offset(count as isize));
        count += 1;
    }

    stats_fprintf(f, b"+RTS \0" as *const u8 as *const c_char as *mut c_char);
    count = 0 as c_int;

    while count < rts_argc {
        stats_fprintf_escape(f, *rts_argv.offset(count as isize));
        count += 1;
    }

    stats_fprintf(f, b"\n\0" as *const u8 as *const c_char as *mut c_char);
}

unsafe fn decodeSize(
    mut flag: *const c_char,
    mut offset: uint32_t,
    mut min: StgWord64,
    mut max: StgWord64,
) -> StgWord64 {
    let mut s = null::<c_char>();
    let mut m: StgDouble = 0.;
    let mut val: StgWord64 = 0;
    s = flag.offset(offset as isize);

    if *s == 0 {
        m = 0 as c_int as StgDouble;
    } else {
        let mut end = null_mut::<c_char>();
        m = strtod(s, &raw mut end) as StgDouble;

        if end == s as *mut c_char {
            errorBelch(
                b"error in RTS option %s: unable to parse number '%s'\0" as *const u8
                    as *const c_char,
                flag,
                s,
            );

            stg_exit(EXIT_FAILURE);
        }

        let mut unit: StgWord64 = 0;

        match *end as c_int {
            116 | 84 => {
                unit = (1024 as c_int as StgWord64)
                    .wrapping_mul(1024 as StgWord64)
                    .wrapping_mul(1024 as StgWord64)
                    .wrapping_mul(1024 as StgWord64);
            }
            103 | 71 => {
                unit = (1024 as c_int * 1024 as c_int * 1024 as c_int) as StgWord64;
            }
            109 | 77 => {
                unit = (1024 as c_int * 1024 as c_int) as StgWord64;
            }
            107 | 75 => {
                unit = 1024 as StgWord64;
            }
            119 | 87 => {
                unit = size_of::<W_>() as StgWord64;
            }
            98 | 66 | 0 => {
                unit = 1 as StgWord64;
            }
            _ => {
                errorBelch(
                    b"error in RTS option %s: unknown unit suffix '%c'\0" as *const u8
                        as *const c_char,
                    flag,
                    *end as c_int,
                );

                stg_exit(EXIT_FAILURE);
            }
        }

        m *= unit as StgDouble;
    }

    val = m as StgWord64;

    if m < 0 as c_int as StgDouble || val < min || val > max {
        errorBelch(
            b"error in RTS option %s: size outside allowed range (%llu - %llu)\0" as *const u8
                as *const c_char,
            flag,
            min,
            max,
        );

        stg_exit(EXIT_FAILURE);
    }

    return val;
}

unsafe fn parseDouble(mut arg: *const c_char, mut error: *mut bool) -> c_double {
    let mut endptr = null_mut::<c_char>();
    let mut out: c_double = 0.;
    *__error() = 0 as c_int;
    out = strtod(arg, &raw mut endptr);

    if *__error() != 0 as c_int || endptr == arg as *mut c_char {
        *error = r#true != 0;

        return out;
    }

    while isspace(*endptr as c_uchar as c_int) != 0 {
        endptr = endptr.offset(1);
    }

    if *endptr as c_int != 0 as c_int {
        *error = r#true != 0;
    }

    return out;
}

unsafe fn read_debug_flags(mut arg: *const c_char) {
    let mut c = null::<c_char>();
    c = arg.offset(2 as c_int as isize);

    while *c as c_int != '\0' as i32 {
        match *c as c_int {
            115 => {
                RtsFlags.DebugFlags.scheduler = r#true != 0;
            }
            105 => {
                RtsFlags.DebugFlags.interpreter = r#true != 0;
            }
            119 => {
                RtsFlags.DebugFlags.weak = r#true != 0;
            }
            71 => {
                RtsFlags.DebugFlags.gccafs = r#true != 0;
            }
            103 => {
                RtsFlags.DebugFlags.gc = r#true != 0;
            }
            110 => {
                RtsFlags.DebugFlags.nonmoving_gc = r#true != 0;
            }
            98 => {
                RtsFlags.DebugFlags.block_alloc = r#true != 0;
            }
            83 => {
                RtsFlags.DebugFlags.sanity = r#true != 0;
            }
            90 => {
                RtsFlags.DebugFlags.zero_on_gc = r#true != 0;
            }
            116 => {
                RtsFlags.DebugFlags.stable = r#true != 0;
            }
            112 => {
                RtsFlags.DebugFlags.prof = r#true != 0;
            }
            108 => {
                RtsFlags.DebugFlags.linker = r#true != 0;
            }
            76 => {
                RtsFlags.DebugFlags.linker_verbose = r#true != 0;
                RtsFlags.DebugFlags.linker = r#true != 0;
            }
            97 => {
                RtsFlags.DebugFlags.apply = r#true != 0;
            }
            109 => {
                RtsFlags.DebugFlags.stm = r#true != 0;
            }
            122 => {
                RtsFlags.DebugFlags.squeeze = r#true != 0;
            }
            99 => {
                RtsFlags.DebugFlags.hpc = r#true != 0;
            }
            114 => {
                RtsFlags.DebugFlags.sparks = r#true != 0;
            }
            67 => {
                RtsFlags.DebugFlags.compact = r#true != 0;
            }
            107 => {
                RtsFlags.DebugFlags.continuation = r#true != 0;
            }
            111 => {
                RtsFlags.DebugFlags.iomanager = r#true != 0;
            }
            _ => {
                bad_option(arg);
            }
        }

        c = c.offset(1);
    }

    if RtsFlags.TraceFlags.tracing == TRACE_NONE {
        RtsFlags.TraceFlags.tracing = TRACE_STDERR;
    }

    if RtsFlags.DebugFlags.sanity {
        RtsFlags.DebugFlags.zero_on_gc = r#true != 0;
    }
}

unsafe fn read_trace_flags(mut arg: *const c_char) {
    let mut c = null::<c_char>();
    let mut enabled = r#true != 0;
    RtsFlags.TraceFlags.scheduler = r#true != 0;
    RtsFlags.TraceFlags.gc = r#true != 0;
    RtsFlags.TraceFlags.sparks_sampled = r#true != 0;
    RtsFlags.TraceFlags.user = r#true != 0;
    c = arg;

    while *c as c_int != '\0' as i32 {
        match *c as c_int {
            0 => {}
            45 => {
                enabled = r#false != 0;
            }
            97 => {
                RtsFlags.TraceFlags.scheduler = enabled;
                RtsFlags.TraceFlags.gc = enabled;
                RtsFlags.TraceFlags.sparks_sampled = enabled;
                RtsFlags.TraceFlags.sparks_full = enabled;
                RtsFlags.TraceFlags.user = enabled;
                RtsFlags.TraceFlags.nonmoving_gc = enabled;
                RtsFlags.TraceFlags.ticky = enabled;
                enabled = r#true != 0;
            }
            115 => {
                RtsFlags.TraceFlags.scheduler = enabled;
                enabled = r#true != 0;
            }
            112 => {
                RtsFlags.TraceFlags.sparks_sampled = enabled;
                enabled = r#true != 0;
            }
            102 => {
                RtsFlags.TraceFlags.sparks_full = enabled;
                enabled = r#true != 0;
            }
            116 => {
                RtsFlags.TraceFlags.timestamp = enabled;
                enabled = r#true != 0;
            }
            103 => {
                RtsFlags.TraceFlags.gc = enabled;
                enabled = r#true != 0;
            }
            110 => {
                RtsFlags.TraceFlags.nonmoving_gc = enabled;
                enabled = r#true != 0;
            }
            117 => {
                RtsFlags.TraceFlags.user = enabled;
                enabled = r#true != 0;
            }
            84 => {
                RtsFlags.TraceFlags.ticky = enabled;
                enabled = r#true != 0;
            }
            _ => {
                errorBelch(
                    b"unknown trace option: %c\0" as *const u8 as *const c_char,
                    *c as c_int,
                );
            }
        }

        c = c.offset(1);
    }
}

unsafe fn bad_option(mut s: *const c_char) -> ! {
    errorBelch(b"bad RTS option: %s\0" as *const u8 as *const c_char, s);
    stg_exit(EXIT_FAILURE);
}

unsafe fn copyArg(mut arg: *mut c_char) -> *mut c_char {
    let mut new_arg = stgMallocBytes(
        strlen(arg).wrapping_add(1 as size_t),
        b"copyArg\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_char;

    strcpy(new_arg, arg);

    return new_arg;
}

unsafe fn copyArgv(mut argc: c_int, mut argv: *mut *mut c_char) -> *mut *mut c_char {
    let mut i: c_int = 0;
    let mut new_argv = null_mut::<*mut c_char>();

    new_argv = stgCallocBytes(
        (argc + 1 as c_int) as size_t,
        size_of::<*mut c_char>() as size_t,
        b"copyArgv 1\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut c_char;

    i = 0 as c_int;

    while i < argc {
        let ref mut fresh0 = *new_argv.offset(i as isize);
        *fresh0 = copyArg(*argv.offset(i as isize));
        i += 1;
    }

    let ref mut fresh1 = *new_argv.offset(argc as isize);
    *fresh1 = null_mut::<c_char>();

    return new_argv;
}

unsafe fn freeArgv(mut argc: c_int, mut argv: *mut *mut c_char) {
    let mut i: c_int = 0;

    if !argv.is_null() {
        i = 0 as c_int;

        while i < argc {
            stgFree(*argv.offset(i as isize) as *mut c_void);
            i += 1;
        }

        stgFree(argv as *mut c_void);
    }
}

unsafe fn setProgName(mut argv: *mut *mut c_char) {
    let mut last_slash = null_mut::<c_char>();

    if (*argv.offset(0 as c_int as isize)).is_null() {
        prog_name = b"\0" as *const u8 as *const c_char as *mut c_char;
        return;
    }

    last_slash = strrchr(*argv.offset(0 as c_int as isize), '/' as i32);

    if !last_slash.is_null() {
        prog_name = last_slash.offset(1 as c_int as isize);
    } else {
        prog_name = *argv.offset(0 as c_int as isize);
    };
}

#[ffi(ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProgArgv(mut argc: *mut c_int, mut argv: *mut *mut *mut c_char) {
    if !argc.is_null() {
        *argc = prog_argc;
    }

    if !argv.is_null() {
        *argv = prog_argv;
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setProgArgv(mut argc: c_int, mut argv: *mut *mut c_char) {
    freeArgv(prog_argc, prog_argv as *mut *mut c_char);
    prog_argc = argc;
    prog_argv = copyArgv(argc, argv);
    setProgName(prog_argv as *mut *mut c_char);
}

unsafe fn freeProgArgv() {
    freeArgv(prog_argc, prog_argv as *mut *mut c_char);
    prog_argc = 0 as c_int;
    prog_argv = null_mut::<*mut c_char>();
}

unsafe fn setFullProgArgv(mut argc: c_int, mut argv: *mut *mut c_char) {
    full_prog_argc = argc;
    full_prog_argv = copyArgv(argc, argv);
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getFullProgArgv(mut argc: *mut c_int, mut argv: *mut *mut *mut c_char) {
    if !argc.is_null() {
        *argc = full_prog_argc;
    }

    if !argv.is_null() {
        *argv = full_prog_argv;
    }
}

unsafe fn freeFullProgArgv() {
    freeArgv(full_prog_argc, full_prog_argv as *mut *mut c_char);
    full_prog_argc = 0 as c_int;
    full_prog_argv = null_mut::<*mut c_char>();
}

unsafe fn freeRtsArgv() {
    freeArgv(rts_argc, rts_argv as *mut *mut c_char);
    rts_argc = 0 as c_int;
    rts_argv = null_mut::<*mut c_char>();
    rts_argv_size = 0 as c_int;
}

unsafe fn freeRtsArgs() {
    freeFullProgArgv();
    freeProgArgv();
    freeRtsArgv();
}
