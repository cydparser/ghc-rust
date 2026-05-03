use std::fs::{self, File};
use std::io::{self, Write};

use errno::{Errno, errno, set_errno};
use printf_compat as printf;

use crate::capability::n_numa_nodes;
use crate::config::RtsOptsEnabledEnum;
use crate::eventlog::event_log_writer::{EventLogWriter, FileEventLogWriter};
use crate::ffi::rts::constants::MAX_NUMA_NODES;
use crate::ffi::rts::os_threads::getNumberOfProcessors;
use crate::ffi::rts::storage::block::{BLOCK_SIZE, MBLOCK_SIZE};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgDouble, StgWord, StgWord64};
use crate::hooks::hooks::{
    FlagDefaultsHook, LongGCSync, LongGCSyncEnd, MallocFailHook, OnExitHook, OutOfHeapHook,
    StackOverflowHook,
};
use crate::hs_ffi::{HS_INT_MAX, HS_WORD_MAX, HS_WORD64_MAX, HsBool};
use crate::io_manager::{IOManagerAvailable, IOManagerUnavailable, parseIOManagerFlag};
use crate::prelude::*;
use crate::rts_api::{
    RtsConfig, RtsOptsAll, RtsOptsIgnore, RtsOptsIgnoreAll, RtsOptsNone, RtsOptsSafeOnly,
};
use crate::rts_messages::{barf, errorBelch, vdebugBelch};
use crate::rts_startup::{EXIT_FAILURE, stg_exit};
use crate::rts_utils::{
    printRtsInfo, stgCallocBytes, stgFree, stgMallocBytes, stgReallocBytes, stgStrndup,
};
use crate::sm::os_mem::{getPhysicalMemorySize, osBuiltWithNumaSupport, osNumaAvailable};
use crate::time::{Time, fsecondsToTime};

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
pub const NO_GC_STATS: u32 = 0;

#[ffi(ghc_lib)]
pub const COLLECT_GC_STATS: u32 = 1;

#[ffi(compiler, ghc_lib)]
pub const ONELINE_GC_STATS: u32 = 2;

#[ffi(ghc_lib)]
pub const SUMMARY_GC_STATS: u32 = 3;

#[ffi(ghc_lib)]
pub const VERBOSE_GC_STATS: u32 = 4;

#[ffi(ghc_lib)]
pub const COST_CENTRES_NONE: u32 = 0;

#[ffi(ghc_lib)]
pub const COST_CENTRES_SUMMARY: u32 = 1;

#[ffi(ghc_lib)]
pub const COST_CENTRES_VERBOSE: u32 = 2;

#[ffi(ghc_lib)]
pub const COST_CENTRES_ALL: u32 = 3;

#[ffi(ghc_lib)]
pub const COST_CENTRES_JSON: u32 = 4;

#[ffi(ghc_lib)]
pub const NO_HEAP_PROFILING: u32 = 0;

#[ffi(ghc_lib)]
pub const HEAP_BY_CCS: u32 = 1;

#[ffi(ghc_lib)]
pub const HEAP_BY_MOD: u32 = 2;

#[ffi(ghc_lib)]
pub const HEAP_BY_DESCR: u32 = 4;

#[ffi(ghc_lib)]
pub const HEAP_BY_TYPE: u32 = 5;

#[ffi(ghc_lib)]
pub const HEAP_BY_RETAINER: u32 = 6;

#[ffi(ghc_lib)]
pub const HEAP_BY_LDV: u32 = 7;

#[ffi(ghc_lib)]
pub const HEAP_BY_CLOSURE_TYPE: u32 = 8;

#[ffi(ghc_lib)]
pub const HEAP_BY_INFO_TABLE: u32 = 9;

#[ffi(ghc_lib)]
pub const HEAP_BY_ERA: u32 = 10;

#[ffi(ghc_lib)]
pub const TRACE_NONE: i32 = 0;

#[ffi(ghc_lib)]
pub const TRACE_EVENTLOG: i32 = 1;

#[ffi(ghc_lib)]
pub const TRACE_STDERR: i32 = 2;

const DEFAULT_TICK_INTERVAL: Time = cfg_select! {
    target_family = "wasm" => 0,
    _ => 10_000 * 1000,
};

pub(crate) const DEFAULT_LINKER_ALWAYS_PIC: u32 = 1;

pub(crate) const STATS_FILENAME_MAXLEN: u32 = 128;

#[ffi(compiler)]
#[repr(C)]
pub struct _GC_FLAGS {
    pub statsFile: Option<&'static mut File>,
    pub giveStats: u32,
    pub maxStkSize: u32,
    pub initialStkSize: u32,
    pub stkChunkSize: u32,
    pub stkChunkBufferSize: u32,
    pub maxHeapSize: u32,
    pub minAllocAreaSize: u32,
    pub largeAllocLim: u32,
    pub nurseryChunkSize: u32,
    pub minOldGenSize: u32,
    pub heapSizeSuggestion: u32,
    pub heapSizeSuggestionAuto: bool,
    pub oldGenFactor: f64,
    pub returnDecayFactor: f64,
    pub pcFreeHeap: f64,
    pub useNonmoving: bool,
    pub nonmovingDenseAllocatorCount: u16,
    pub generations: u32,
    pub squeezeUpdFrames: bool,
    pub compact: bool,
    pub compactThreshold: f64,
    pub sweep: bool,
    pub ringBell: bool,
    pub idleGCDelayTime: Time,
    pub interIdleGCWait: Time,
    pub doIdleGC: bool,
    pub longGCSync: Time,
    pub heapBase: StgWord,
    pub allocLimitGrace: StgWord,
    pub heapLimitGrace: StgWord,
    pub numa: bool,
    pub numaMask: StgWord,
    pub addressSpaceSize: StgWord64,
}

#[ffi(compiler, ghc_lib)]
pub type GC_FLAGS = _GC_FLAGS;

#[ffi(compiler, ghc_lib)]
pub type DEBUG_FLAGS = _DEBUG_FLAGS;

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct _DEBUG_FLAGS {
    pub scheduler: bool,
    pub interpreter: bool,
    pub weak: bool,
    pub gccafs: bool,
    pub gc: bool,
    pub nonmoving_gc: bool,
    pub block_alloc: bool,
    pub sanity: bool,
    pub zero_on_gc: bool,
    pub stable: bool,
    pub prof: bool,
    pub linker: bool,
    pub linker_verbose: bool,
    pub apply: bool,
    pub stm: bool,
    pub squeeze: bool,
    pub hpc: bool,
    pub sparks: bool,
    pub numa: bool,
    pub compact: bool,
    pub continuation: bool,
    pub iomanager: bool,
}

#[cfg(test)]
impl Arbitrary for _DEBUG_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _DEBUG_FLAGS {
            scheduler: Arbitrary::arbitrary(g),
            interpreter: Arbitrary::arbitrary(g),
            weak: Arbitrary::arbitrary(g),
            gccafs: Arbitrary::arbitrary(g),
            gc: Arbitrary::arbitrary(g),
            nonmoving_gc: Arbitrary::arbitrary(g),
            block_alloc: Arbitrary::arbitrary(g),
            sanity: Arbitrary::arbitrary(g),
            zero_on_gc: Arbitrary::arbitrary(g),
            stable: Arbitrary::arbitrary(g),
            prof: Arbitrary::arbitrary(g),
            linker: Arbitrary::arbitrary(g),
            linker_verbose: Arbitrary::arbitrary(g),
            apply: Arbitrary::arbitrary(g),
            stm: Arbitrary::arbitrary(g),
            squeeze: Arbitrary::arbitrary(g),
            hpc: Arbitrary::arbitrary(g),
            sparks: Arbitrary::arbitrary(g),
            numa: Arbitrary::arbitrary(g),
            compact: Arbitrary::arbitrary(g),
            continuation: Arbitrary::arbitrary(g),
            iomanager: Arbitrary::arbitrary(g),
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _COST_CENTRE_FLAGS {
    doCostCentres: u32,
    profilerTicks: i32,
    msecsPerTick: i32,
    outputFileNameStem: *const c_char,
}

#[ffi(compiler, ghc_lib)]
pub type COST_CENTRE_FLAGS = _COST_CENTRE_FLAGS;

/// cbindgen:no-export
#[repr(C)]
pub struct _PROFILING_FLAGS {
    doHeapProfile: u32,
    heapProfileInterval: Time,
    heapProfileIntervalTicks: u32,
    startHeapProfileAtStartup: bool,
    startTimeProfileAtStartup: bool,
    incrementUserEra: bool,
    showCCSOnException: bool,
    maxRetainerSetSize: u32,
    ccsLength: u32,
    modSelector: *const c_char,
    descrSelector: *const c_char,
    typeSelector: *const c_char,
    ccSelector: *const c_char,
    ccsSelector: *const c_char,
    retainerSelector: *const c_char,
    eraSelector: StgWord,
    bioSelector: *const c_char,
}

#[ffi(compiler, ghc_lib)]
pub type PROFILING_FLAGS = _PROFILING_FLAGS;

/// cbindgen:no-export
#[repr(C)]
pub struct _TRACE_FLAGS {
    tracing: i32,
    timestamp: bool,
    scheduler: bool,
    gc: bool,
    nonmoving_gc: bool,
    sparks_sampled: bool,
    sparks_full: bool,
    ticky: bool,
    user: bool,
    eventlogFlushTime: Time,
    eventlogFlushTicks: i32,
    trace_output: *mut c_char,
    nullWriter: bool,
}

#[ffi(compiler, ghc_lib)]
pub type TRACE_FLAGS = _TRACE_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct _CONCURRENT_FLAGS {
    ctxtSwitchTime: Time,
    ctxtSwitchTicks: i32,
}

#[cfg(test)]
impl Arbitrary for _CONCURRENT_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _CONCURRENT_FLAGS {
            ctxtSwitchTime: Arbitrary::arbitrary(g),
            ctxtSwitchTicks: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler, ghc_lib)]
pub type CONCURRENT_FLAGS = _CONCURRENT_FLAGS;

pub(crate) type IO_MANAGER_FLAG = _IO_MANAGER_FLAG;

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum _IO_MANAGER_FLAG {
    IO_MNGR_FLAG_AUTO = 0,
    IO_MNGR_FLAG_SELECT = 1,
    IO_MNGR_FLAG_MIO = 2,
    IO_MNGR_FLAG_WINIO = 3,
    IO_MNGR_FLAG_WIN32_LEGACY = 4,
}

#[cfg(feature = "sys")]
impl From<_IO_MANAGER_FLAG> for sys::_IO_MANAGER_FLAG {
    fn from(v: _IO_MANAGER_FLAG) -> Self {
        use _IO_MANAGER_FLAG::*;

        match v {
            IO_MNGR_FLAG_AUTO => sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_AUTO,
            IO_MNGR_FLAG_SELECT => sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_SELECT,
            IO_MNGR_FLAG_MIO => sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_MIO,
            IO_MNGR_FLAG_WINIO => sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_WINIO,
            IO_MNGR_FLAG_WIN32_LEGACY => sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_WIN32_LEGACY,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::_IO_MANAGER_FLAG> for _IO_MANAGER_FLAG {
    fn from(v: sys::_IO_MANAGER_FLAG) -> Self {
        use _IO_MANAGER_FLAG::*;

        match v {
            sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_AUTO => IO_MNGR_FLAG_AUTO,
            sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_SELECT => IO_MNGR_FLAG_SELECT,
            sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_MIO => IO_MNGR_FLAG_MIO,
            sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_WINIO => IO_MNGR_FLAG_WINIO,
            sys::_IO_MANAGER_FLAG::IO_MNGR_FLAG_WIN32_LEGACY => IO_MNGR_FLAG_WIN32_LEGACY,
        }
    }
}

impl TryFrom<u32> for _IO_MANAGER_FLAG {
    type Error = ();

    fn try_from(d: u32) -> Result<_IO_MANAGER_FLAG, ()> {
        use _IO_MANAGER_FLAG::*;

        match d {
            0 => Ok(IO_MNGR_FLAG_AUTO),
            1 => Ok(IO_MNGR_FLAG_SELECT),
            2 => Ok(IO_MNGR_FLAG_MIO),
            3 => Ok(IO_MNGR_FLAG_WINIO),
            4 => Ok(IO_MNGR_FLAG_WIN32_LEGACY),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for _IO_MANAGER_FLAG {
    fn arbitrary(g: &mut Gen) -> Self {
        use _IO_MANAGER_FLAG::*;

        match usize::arbitrary(g) % 5 {
            0 => IO_MNGR_FLAG_AUTO,
            1 => IO_MNGR_FLAG_SELECT,
            2 => IO_MNGR_FLAG_MIO,
            3 => IO_MNGR_FLAG_WINIO,
            4.. => IO_MNGR_FLAG_WIN32_LEGACY,
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct _MISC_FLAGS {
    tickInterval: Time,
    install_signal_handlers: bool,
    install_seh_handlers: bool,
    generate_dump_file: bool,
    generate_stack_trace: bool,
    machineReadable: bool,
    disableDelayedOsMemoryReturn: bool,
    internalCounters: bool,
    linkerAlwaysPic: bool,
    linkerOptimistic: bool,
    linkerMemBase: StgWord,
    ioManager: IO_MANAGER_FLAG,
    numIoWorkerThreads: u32,
}

#[cfg(test)]
impl Arbitrary for _MISC_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _MISC_FLAGS {
            tickInterval: Arbitrary::arbitrary(g),
            install_signal_handlers: Arbitrary::arbitrary(g),
            install_seh_handlers: Arbitrary::arbitrary(g),
            generate_dump_file: Arbitrary::arbitrary(g),
            generate_stack_trace: Arbitrary::arbitrary(g),
            machineReadable: Arbitrary::arbitrary(g),
            disableDelayedOsMemoryReturn: Arbitrary::arbitrary(g),
            internalCounters: Arbitrary::arbitrary(g),
            linkerAlwaysPic: Arbitrary::arbitrary(g),
            linkerOptimistic: Arbitrary::arbitrary(g),
            linkerMemBase: Arbitrary::arbitrary(g),
            ioManager: Arbitrary::arbitrary(g),
            numIoWorkerThreads: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler, ghc_lib)]
pub type MISC_FLAGS = _MISC_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct _PAR_FLAGS {
    nCapabilities: u32,
    migrate: bool,
    maxLocalSparks: u32,
    parGcEnabled: bool,
    parGcGen: u32,
    parGcLoadBalancingEnabled: bool,
    parGcLoadBalancingGen: u32,
    parGcNoSyncWithIdle: u32,
    parGcThreads: u32,
    setAffinity: bool,
}

#[cfg(test)]
impl Arbitrary for _PAR_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _PAR_FLAGS {
            nCapabilities: Arbitrary::arbitrary(g),
            migrate: Arbitrary::arbitrary(g),
            maxLocalSparks: Arbitrary::arbitrary(g),
            parGcEnabled: Arbitrary::arbitrary(g),
            parGcGen: Arbitrary::arbitrary(g),
            parGcLoadBalancingEnabled: Arbitrary::arbitrary(g),
            parGcLoadBalancingGen: Arbitrary::arbitrary(g),
            parGcNoSyncWithIdle: Arbitrary::arbitrary(g),
            parGcThreads: Arbitrary::arbitrary(g),
            setAffinity: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler, ghc_lib)]
pub type PAR_FLAGS = _PAR_FLAGS;

pub(crate) type HPC_READ_FILE = _HPC_READ_FILE;

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum _HPC_READ_FILE {
    HPC_NO_EXPLICIT = 0,
    HPC_YES_IMPLICIT = 1,
    HPC_YES_EXPLICIT = 2,
}

#[cfg(feature = "sys")]
impl From<_HPC_READ_FILE> for sys::_HPC_READ_FILE {
    fn from(v: _HPC_READ_FILE) -> Self {
        use _HPC_READ_FILE::*;

        match v {
            HPC_NO_EXPLICIT => sys::_HPC_READ_FILE::HPC_NO_EXPLICIT,
            HPC_YES_IMPLICIT => sys::_HPC_READ_FILE::HPC_YES_IMPLICIT,
            HPC_YES_EXPLICIT => sys::_HPC_READ_FILE::HPC_YES_EXPLICIT,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::_HPC_READ_FILE> for _HPC_READ_FILE {
    fn from(v: sys::_HPC_READ_FILE) -> Self {
        use _HPC_READ_FILE::*;

        match v {
            sys::_HPC_READ_FILE::HPC_NO_EXPLICIT => HPC_NO_EXPLICIT,
            sys::_HPC_READ_FILE::HPC_YES_IMPLICIT => HPC_YES_IMPLICIT,
            sys::_HPC_READ_FILE::HPC_YES_EXPLICIT => HPC_YES_EXPLICIT,
        }
    }
}

impl TryFrom<u32> for _HPC_READ_FILE {
    type Error = ();

    fn try_from(d: u32) -> Result<_HPC_READ_FILE, ()> {
        use _HPC_READ_FILE::*;

        match d {
            0 => Ok(HPC_NO_EXPLICIT),
            1 => Ok(HPC_YES_IMPLICIT),
            2 => Ok(HPC_YES_EXPLICIT),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for _HPC_READ_FILE {
    fn arbitrary(g: &mut Gen) -> Self {
        use _HPC_READ_FILE::*;

        match usize::arbitrary(g) % 3 {
            0 => HPC_NO_EXPLICIT,
            1 => HPC_YES_IMPLICIT,
            2.. => HPC_YES_EXPLICIT,
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct _HPC_FLAGS {
    writeTixFile: bool,
    readTixFile: HPC_READ_FILE,
}

#[cfg(test)]
impl Arbitrary for _HPC_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _HPC_FLAGS {
            writeTixFile: Arbitrary::arbitrary(g),
            readTixFile: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(compiler, ghc_lib)]
pub type HPC_FLAGS = _HPC_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _TICKY_FLAGS {
    pub(crate) showTickyStats: bool,
    pub(crate) tickyFile: Option<&'static mut File>,
}

#[ffi(compiler, ghc_lib)]
pub type TICKY_FLAGS = _TICKY_FLAGS;

#[ffi(compiler)]
#[repr(C)]
pub struct _RTS_FLAGS {
    pub GcFlags: GC_FLAGS,
    pub ConcFlags: CONCURRENT_FLAGS,
    pub MiscFlags: MISC_FLAGS,
    pub DebugFlags: DEBUG_FLAGS,
    pub CcFlags: COST_CENTRE_FLAGS,
    pub ProfFlags: PROFILING_FLAGS,
    pub TraceFlags: TRACE_FLAGS,
    pub TickyFlags: TICKY_FLAGS,
    pub ParFlags: PAR_FLAGS,
    pub HpcFlags: HPC_FLAGS,
}

#[ffi(compiler, ghc_lib)]
pub type RTS_FLAGS = _RTS_FLAGS;

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub static mut RtsFlags: RTS_FLAGS = _RTS_FLAGS {
    GcFlags: _GC_FLAGS {
        statsFile: None,
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
        ioManager: _IO_MANAGER_FLAG::IO_MNGR_FLAG_AUTO,
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
        trace_output: null_mut::<c_char>(),
        nullWriter: false,
    },
    TickyFlags: _TICKY_FLAGS {
        showTickyStats: false,
        tickyFile: None,
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
        readTixFile: _HPC_READ_FILE::HPC_NO_EXPLICIT,
    },
};

static mut prog_argc: u32 = 0;

static mut prog_argv: *mut *mut c_char = null_mut::<*mut c_char>();

static mut full_prog_argc: u32 = 0;

static mut full_prog_argv: *mut *mut c_char = null_mut::<*mut c_char>();

static mut prog_name: *mut c_char = null_mut::<c_char>();

static mut rts_argc: i32 = 0;

static mut rts_argv: *mut *mut c_char = null_mut::<*mut c_char>();

static mut rts_argv_size: u32 = 0;

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
        rts_opts_suggestions: true,
        rts_opts: null::<c_char>(),
        rts_hs_main: false,
        keep_cafs: false,
        eventlog_writer: &raw const FileEventLogWriter,
        defaultsHook: Some(FlagDefaultsHook as unsafe extern "C" fn() -> ()),
        onExitHook: Some(OnExitHook as unsafe extern "C" fn() -> ()),
        stackOverflowHook: Some(StackOverflowHook as unsafe extern "C" fn(W_) -> ()),
        outOfHeapHook: Some(OutOfHeapHook as unsafe extern "C" fn(W_, W_) -> ()),
        mallocFailHook: Some(MallocFailHook as unsafe extern "C" fn(W_, *const c_char) -> ()),
        gcDoneHook: None,
        longGCSync: Some(LongGCSync as unsafe extern "C" fn(c_uint, Time) -> ()),
        longGCSyncEnd: Some(LongGCSyncEnd as unsafe extern "C" fn(Time) -> ()),
    }
};

const RTS: i32 = 1;

const PGM: i32 = 0;

unsafe fn initRtsFlagsDefaults() {
    let mut maxStkSize: StgWord64 = (8 as StgWord64)
        .wrapping_mul(getPhysicalMemorySize())
        .wrapping_div(10 as StgWord64);

    if maxStkSize == 0 {
        maxStkSize = (8 * 1024 * 1024) as StgWord64;
    } else if maxStkSize > (u32::MAX as StgWord64).wrapping_mul(size_of::<W_>() as StgWord64) {
        maxStkSize = (u32::MAX as StgWord64).wrapping_mul(size_of::<W_>() as StgWord64);
    }

    RtsFlags.GcFlags.statsFile = None;
    RtsFlags.GcFlags.giveStats = NO_GC_STATS as u32;
    RtsFlags.GcFlags.maxStkSize = maxStkSize.wrapping_div(size_of::<W_>() as StgWord64) as u32;
    RtsFlags.GcFlags.initialStkSize = (1024 as usize).wrapping_div(size_of::<W_>() as usize) as u32;
    RtsFlags.GcFlags.stkChunkSize =
        ((32 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize) as u32;
    RtsFlags.GcFlags.stkChunkBufferSize =
        ((1 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize) as u32;
    RtsFlags.GcFlags.minAllocAreaSize = (4_u32 * 1024 * 1024).wrapping_div(BLOCK_SIZE);
    RtsFlags.GcFlags.largeAllocLim = 0;
    RtsFlags.GcFlags.nurseryChunkSize = 0;
    RtsFlags.GcFlags.minOldGenSize = (1024_u32 * 1024).wrapping_div(BLOCK_SIZE);
    RtsFlags.GcFlags.maxHeapSize = 0;
    RtsFlags.GcFlags.heapLimitGrace = (1024 * 1024) as StgWord;
    RtsFlags.GcFlags.heapSizeSuggestion = 0;
    RtsFlags.GcFlags.heapSizeSuggestionAuto = false;
    RtsFlags.GcFlags.pcFreeHeap = 3.;
    RtsFlags.GcFlags.oldGenFactor = 2.;
    RtsFlags.GcFlags.returnDecayFactor = 4.;
    RtsFlags.GcFlags.useNonmoving = false;
    RtsFlags.GcFlags.nonmovingDenseAllocatorCount = 16;
    RtsFlags.GcFlags.generations = 2;
    RtsFlags.GcFlags.squeezeUpdFrames = true;
    RtsFlags.GcFlags.compact = false;
    RtsFlags.GcFlags.compactThreshold = 30.0f64;
    RtsFlags.GcFlags.sweep = false;
    RtsFlags.GcFlags.idleGCDelayTime = 300000 * 1000;
    RtsFlags.GcFlags.interIdleGCWait = 0;
    RtsFlags.GcFlags.doIdleGC = true;
    RtsFlags.GcFlags.heapBase = 0;
    RtsFlags.GcFlags.allocLimitGrace = (100_u32 * 1024).wrapping_div(BLOCK_SIZE) as StgWord;
    RtsFlags.GcFlags.numa = false;
    RtsFlags.GcFlags.numaMask = 1;
    RtsFlags.GcFlags.ringBell = false;
    RtsFlags.GcFlags.longGCSync = 0;
    RtsFlags.GcFlags.addressSpaceSize = 1 << 40;
    RtsFlags.DebugFlags.scheduler = false;
    RtsFlags.DebugFlags.interpreter = false;
    RtsFlags.DebugFlags.weak = false;
    RtsFlags.DebugFlags.gccafs = false;
    RtsFlags.DebugFlags.gc = false;
    RtsFlags.DebugFlags.nonmoving_gc = false;
    RtsFlags.DebugFlags.block_alloc = false;
    RtsFlags.DebugFlags.sanity = false;
    RtsFlags.DebugFlags.zero_on_gc = false;
    RtsFlags.DebugFlags.stable = false;
    RtsFlags.DebugFlags.stm = false;
    RtsFlags.DebugFlags.prof = false;
    RtsFlags.DebugFlags.apply = false;
    RtsFlags.DebugFlags.linker = false;
    RtsFlags.DebugFlags.linker_verbose = false;
    RtsFlags.DebugFlags.squeeze = false;
    RtsFlags.DebugFlags.hpc = false;
    RtsFlags.DebugFlags.sparks = false;
    RtsFlags.DebugFlags.numa = false;
    RtsFlags.DebugFlags.compact = false;
    RtsFlags.DebugFlags.continuation = false;
    RtsFlags.CcFlags.doCostCentres = COST_CENTRES_NONE as u32;
    RtsFlags.CcFlags.outputFileNameStem = null::<c_char>();
    RtsFlags.ProfFlags.doHeapProfile = NO_HEAP_PROFILING;
    RtsFlags.ProfFlags.heapProfileInterval = 100000 * 1000;
    RtsFlags.ProfFlags.startHeapProfileAtStartup = true;
    RtsFlags.ProfFlags.startTimeProfileAtStartup = true;
    RtsFlags.ProfFlags.incrementUserEra = false;
    RtsFlags.ProfFlags.showCCSOnException = false;
    RtsFlags.ProfFlags.maxRetainerSetSize = 8;
    RtsFlags.ProfFlags.ccsLength = 25;
    RtsFlags.ProfFlags.modSelector = null::<c_char>();
    RtsFlags.ProfFlags.descrSelector = null::<c_char>();
    RtsFlags.ProfFlags.typeSelector = null::<c_char>();
    RtsFlags.ProfFlags.ccSelector = null::<c_char>();
    RtsFlags.ProfFlags.ccsSelector = null::<c_char>();
    RtsFlags.ProfFlags.retainerSelector = null::<c_char>();
    RtsFlags.ProfFlags.bioSelector = null::<c_char>();
    RtsFlags.ProfFlags.eraSelector = 0;
    RtsFlags.TraceFlags.tracing = TRACE_NONE;
    RtsFlags.TraceFlags.timestamp = false;
    RtsFlags.TraceFlags.scheduler = false;
    RtsFlags.TraceFlags.gc = false;
    RtsFlags.TraceFlags.nonmoving_gc = false;
    RtsFlags.TraceFlags.sparks_sampled = false;
    RtsFlags.TraceFlags.sparks_full = false;
    RtsFlags.TraceFlags.user = false;
    RtsFlags.TraceFlags.ticky = false;
    RtsFlags.TraceFlags.trace_output = null_mut::<c_char>();
    RtsFlags.TraceFlags.eventlogFlushTime = 0;
    RtsFlags.TraceFlags.nullWriter = false;
    RtsFlags.MiscFlags.tickInterval = 1000 * 1000;
    RtsFlags.ConcFlags.ctxtSwitchTime = 20000 * 1000;
    RtsFlags.MiscFlags.install_signal_handlers = true;
    RtsFlags.MiscFlags.install_seh_handlers = true;
    RtsFlags.MiscFlags.generate_stack_trace = true;
    RtsFlags.MiscFlags.generate_dump_file = false;
    RtsFlags.MiscFlags.machineReadable = false;
    RtsFlags.MiscFlags.disableDelayedOsMemoryReturn = false;
    RtsFlags.MiscFlags.internalCounters = false;
    RtsFlags.MiscFlags.linkerAlwaysPic = DEFAULT_LINKER_ALWAYS_PIC != 0;
    RtsFlags.MiscFlags.linkerOptimistic = false;
    RtsFlags.MiscFlags.linkerMemBase = 0;
    RtsFlags.MiscFlags.ioManager = _IO_MANAGER_FLAG::IO_MNGR_FLAG_AUTO;
    RtsFlags.MiscFlags.numIoWorkerThreads = 1;
    RtsFlags.ParFlags.nCapabilities = 1;
    RtsFlags.ParFlags.migrate = true;
    RtsFlags.ParFlags.parGcEnabled = 1 != 0;
    RtsFlags.ParFlags.parGcGen = 0;
    RtsFlags.ParFlags.parGcLoadBalancingEnabled = true;
    RtsFlags.ParFlags.parGcLoadBalancingGen = !0 as u32;
    RtsFlags.ParFlags.parGcNoSyncWithIdle = 0;
    RtsFlags.ParFlags.parGcThreads = 0;
    RtsFlags.ParFlags.setAffinity = 0 != 0;
    RtsFlags.ParFlags.maxLocalSparks = 4096;
    RtsFlags.TickyFlags.showTickyStats = false;
    RtsFlags.TickyFlags.tickyFile = None;
    RtsFlags.HpcFlags.readTixFile = _HPC_READ_FILE::HPC_YES_IMPLICIT;
    RtsFlags.HpcFlags.writeTixFile = true;
}

static mut usage_text: [*const c_char; 213] = [
    c"".as_ptr(),
    c"Usage: <prog> <args> [+RTS <rtsopts> | -RTS <args>] ... --RTS <args>".as_ptr(),
    c"".as_ptr(),
    c"   +RTS     Indicates run time system options follow".as_ptr(),
    c"   -RTS     Indicates program arguments follow".as_ptr(),
    c"  --RTS     Indicates that ALL subsequent arguments will be given to the".as_ptr(),
    c"            program (including any of these RTS flags)".as_ptr(),
    c"".as_ptr(),
    c"The following run time system options may be available (note that some".as_ptr(),
    c"of these may not be usable unless this program was linked with the -rtsopts"
        .as_ptr(),
    c"flag):".as_ptr(),
    c"".as_ptr(),
    c"  -?        Prints this message and exits; the program is not executed".as_ptr(),
    c"  --info    Print information about the RTS used by this program".as_ptr(),
    c"".as_ptr(),
    c"  --nonmoving-gc".as_ptr(),
    c"            Selects the non-moving mark-and-sweep garbage collector to".as_ptr(),
    c"            manage the oldest generation.".as_ptr(),
    c"  --copying-gc".as_ptr(),
    c"            Selects the copying garbage collector to manage all generations."
        .as_ptr(),
    c"".as_ptr(),
    c"  -K<size>  Sets the maximum stack size (default: 80% of the heap)".as_ptr(),
    c"            e.g.: -K32k -K512k -K8M".as_ptr(),
    c"  -ki<size> Sets the initial thread stack size (default 1k)  e.g.: -ki4k -ki2m"
        .as_ptr(),
    c"  -kc<size> Sets the stack chunk size (default 32k)".as_ptr(),
    c"  -kb<size> Sets the stack chunk buffer size (default 1k)".as_ptr(),
    c"".as_ptr(),
    c"  -A<size>  Sets the minimum allocation area size (default 4m) e.g.: -A20m -A10k"
        .as_ptr(),
    c"  -AL<size> Sets the amount of large-object memory that can be allocated".as_ptr(),
    c"            before a GC is triggered (default: the value of -A)".as_ptr(),
    c"  -F<n>     Sets the collecting threshold for old generations as a factor of"
        .as_ptr(),
    c"            the live data in that generation the last time it was collected"
        .as_ptr(),
    c"            (default: 2.0)".as_ptr(),
    c"  -Fd<n>    Sets the inverse rate which memory is returned to the OS after being"
        .as_ptr(),
    c"            optimistically retained after being allocated. Subsequent major"
        .as_ptr(),
    c"            collections not caused by heap overflow will return an amount of"
        .as_ptr(),
    c"            memory controlled by this factor (higher is slower). Setting the factor"
        .as_ptr(),
    c"            to 0 means memory is not returned.".as_ptr(),
    c"            (default 4.0)".as_ptr(),
    c"  -n<size>  Allocation area chunk size (0 = disabled, default: 0)".as_ptr(),
    c"  -O<size>  Sets the minimum size of the old generation (default 1M)".as_ptr(),
    c"  -M<size>  Sets the maximum heap size (default unlimited)  e.g.: -M256k -M1G"
        .as_ptr(),
    c"  -H<size>  Sets the minimum heap size (default 0M)   e.g.: -H24m  -H1G".as_ptr(),
    c"  -xb<addr> Sets the address from which a suitable start for the heap memory"
        .as_ptr(),
    c"            will be searched from. This is useful if the default address".as_ptr(),
    c"            clashes with some third-party library.".as_ptr(),
    c"  -xn       Use the non-moving collector for the old generation.".as_ptr(),
    c"  -m<n>     Minimum % of heap which must be available (default 3%)".as_ptr(),
    c"  -G<n>     Number of generations (default: 2)".as_ptr(),
    c"  -c<n>     Use in-place compaction instead of copying in the oldest generation"
        .as_ptr(),
    c"            when live data is at least <n>% of the maximum heap size set with"
        .as_ptr(),
    c"            -M (default: 30%)".as_ptr(),
    c"  -c        Use in-place compaction for all oldest generation collections"
        .as_ptr(),
    c"            (the default is to use copying)".as_ptr(),
    c"  -w        Use mark-region for the oldest generation (experimental)".as_ptr(),
    c"  -I<sec>   Perform full GC after <sec> idle time (default: 0.3, 0 == off)"
        .as_ptr(),
    c"  -Iw<sec>  Minimum wait time between idle GC runs (default: 0, 0 == no min wait time)"
        .as_ptr(),
    c"".as_ptr(),
    c"  -T         Collect GC statistics (useful for in-program statistics access)"
        .as_ptr(),
    c"  -t[<file>] One-line GC statistics (if <file> omitted, uses stderr)".as_ptr(),
    c"  -s[<file>] Summary  GC statistics (if <file> omitted, uses stderr)".as_ptr(),
    c"  -S[<file>] Detailed GC statistics (if <file> omitted, uses stderr)".as_ptr(),
    c"".as_ptr(),
    c"".as_ptr(),
    c"  -Z         Don't squeeze out update frames on context switch".as_ptr(),
    c"  -B         Sound the bell at the start of each garbage collection".as_ptr(),
    c"".as_ptr(),
    c"  -p         Time/allocation profile in tree format ".as_ptr(),
    c"             (output file <output prefix>.prof)".as_ptr(),
    c"  -po<file>  Override profiling output file name prefix (program name by default)"
        .as_ptr(),
    c"  -P         More detailed Time/Allocation profile in tree format".as_ptr(),
    c"  -Pa        Give information about *all* cost centres in tree format".as_ptr(),
    c"  -pj        Output cost-center profile in JSON format".as_ptr(),
    c"".as_ptr(),
    c"  -h         Heap residency profile, by cost centre stack".as_ptr(),
    c"  -h<break-down> Heap residency profile (hp2ps) (output file <program>.hp)"
        .as_ptr(),
    c"     break-down: c = cost centre stack (default)".as_ptr(),
    c"                 m = module".as_ptr(),
    c"                 T = closure type".as_ptr(),
    c"                 d = closure description".as_ptr(),
    c"                 y = type description".as_ptr(),
    c"                 i = info table".as_ptr(),
    c"                 e = era".as_ptr(),
    c"                 r = retainer".as_ptr(),
    c"                 b = biography (LAG,DRAG,VOID,USE)".as_ptr(),
    c"  A subset of closures may be selected thusly:".as_ptr(),
    c"    -hc<cc>,...  specific cost centre(s) (top of stack only)".as_ptr(),
    c"    -hC<cc>,...  specific cost centre(s) (anywhere in stack)".as_ptr(),
    c"    -hm<mod>...  all cost centres from the specified modules(s)".as_ptr(),
    c"    -hd<des>,... closures with specified closure descriptions".as_ptr(),
    c"    -hy<typ>...  closures with specified type descriptions".as_ptr(),
    c"    -hr<cc>...   closures with specified retainers".as_ptr(),
    c"    -hb<bio>...  closures with specified biographies (lag,drag,void,use)".as_ptr(),
    c"    -he<era>...  closures with specified era".as_ptr(),
    c"".as_ptr(),
    c"  -R<size>       Set the maximum retainer set size (default: 8)".as_ptr(),
    c"".as_ptr(),
    c"  -L<chars>      Maximum length of a cost-centre stack in a heap profile".as_ptr(),
    c"                 (default: 25)".as_ptr(),
    c"".as_ptr(),
    c"  -xt            Include threads (TSOs) in a heap profile".as_ptr(),
    c"".as_ptr(),
    c"  --automatic-era-increment Increment the era on each major garbage collection"
        .as_ptr(),
    c"".as_ptr(),
    c"  -xc      Show current cost centre stack on raising an exception".as_ptr(),
    c"  -i<sec>  Time between heap profile samples (seconds, default: 0.1)".as_ptr(),
    c"  --no-automatic-heap-samples".as_ptr(),
    c"           Do not start the heap profile interval timer on start-up,".as_ptr(),
    c"           Rather, the application will be responsible for triggering".as_ptr(),
    c"           heap profiler samples.".as_ptr(),
    c"  -ol<file>  Send binary eventlog to <file> (default: <program>.eventlog)"
        .as_ptr(),
    c"  -l[flags]  Log events to a file".as_ptr(),
    c"  -v[flags]  Log events to stderr".as_ptr(),
    c"             where [flags] can contain:".as_ptr(),
    c"                s    scheduler events".as_ptr(),
    c"                g    GC and heap events".as_ptr(),
    c"                n    non-moving GC heap census events".as_ptr(),
    c"                p    par spark events (sampled)".as_ptr(),
    c"                f    par spark events (full detail)".as_ptr(),
    c"                u    user events (emitted from Haskell code)".as_ptr(),
    c"                T    ticky-ticky counter samples".as_ptr(),
    c"                a    all event classes above".as_ptr(),
    c"                t    add time stamps (only useful with -v)".as_ptr(),
    c"               -x    disable an event class, for any flag above".as_ptr(),
    c"             the initial enabled event classes are 'sgpu'".as_ptr(),
    c" --eventlog-flush-interval=<secs>".as_ptr(),
    c"             Periodically flush the eventlog at the specified interval.".as_ptr(),
    c"".as_ptr(),
    c"  -r<file>  Produce ticky-ticky statistics (with -rstderr for stderr)".as_ptr(),
    c"".as_ptr(),
    c"  -C<secs>  Context-switch interval in seconds.".as_ptr(),
    c"            0 or no argument means switch as often as possible.".as_ptr(),
    c"            Default: 0.02 sec.".as_ptr(),
    c"  -V<secs>  Master tick interval in seconds (0 == disable timer).".as_ptr(),
    c"            This sets the resolution for -C and the heap profile timer -i,"
        .as_ptr(),
    c"            and is the frequency of time profile samples.".as_ptr(),
    c"            Default: 0.001 sec.".as_ptr(),
    c"".as_ptr(),
    c"  -Ds  DEBUG: scheduler".as_ptr(),
    c"  -Di  DEBUG: interpreter".as_ptr(),
    c"  -Dw  DEBUG: weak".as_ptr(),
    c"  -DG  DEBUG: gccafs".as_ptr(),
    c"  -Dg  DEBUG: gc".as_ptr(),
    c"  -Dn  DEBUG: non-moving gc".as_ptr(),
    c"  -Db  DEBUG: block".as_ptr(),
    c"  -DS  DEBUG: sanity".as_ptr(),
    c"  -DZ  DEBUG: zero freed memory during GC".as_ptr(),
    c"  -Dt  DEBUG: stable".as_ptr(),
    c"  -Dp  DEBUG: prof".as_ptr(),
    c"  -Da  DEBUG: apply".as_ptr(),
    c"  -Dl  DEBUG: linker".as_ptr(),
    c"  -DL  DEBUG: linker (verbose)".as_ptr(),
    c"  -Dm  DEBUG: stm".as_ptr(),
    c"  -Dz  DEBUG: stack squeezing".as_ptr(),
    c"  -Dc  DEBUG: program coverage".as_ptr(),
    c"  -Dr  DEBUG: sparks".as_ptr(),
    c"  -DC  DEBUG: compact".as_ptr(),
    c"  -Dk  DEBUG: continuation".as_ptr(),
    c"  -Do  DEBUG: iomanager".as_ptr(),
    c"".as_ptr(),
    c"     NOTE: DEBUG events are sent to stderr by default; add -l to create a"
        .as_ptr(),
    c"     binary event log file instead.".as_ptr(),
    c"".as_ptr(),
    c"  -N[<n>]    Use <n> processors (default: 1, -N alone determines".as_ptr(),
    c"             the number of processors to use automatically)".as_ptr(),
    c"  -maxN[<n>] Use up to <n> processors automatically".as_ptr(),
    c"  -qg[<n>]   Use parallel GC only for generations >= <n>".as_ptr(),
    c"             (default: 0, -qg alone turns off parallel GC)".as_ptr(),
    c"  -qb[<n>]   Use load-balancing in the parallel GC only for generations >= <n>"
        .as_ptr(),
    c"             (default: 1 for -A < 32M, 0 otherwise;".as_ptr(),
    c"              -qb alone turns off load-balancing)".as_ptr(),
    c"  -qn<n>     Use <n> threads for parallel GC (defaults to value of -N)".as_ptr(),
    c"  -qa        Use the OS to set thread affinity (experimental)".as_ptr(),
    c"  -qm        Don't automatically migrate threads between CPUs".as_ptr(),
    c"  -qi<n>     If a processor has been idle for the last <n> GCs, do not".as_ptr(),
    c"             wake it up for a non-load-balancing parallel GC.".as_ptr(),
    c"             (0 disables,  default: 0)".as_ptr(),
    c"  --numa[=<node_mask>]".as_ptr(),
    c"             Use NUMA, nodes given by <node_mask> (default: off)".as_ptr(),
    c"  --debug-numa[=<num_nodes>]".as_ptr(),
    c"             Pretend NUMA: like --numa, but without the system calls.".as_ptr(),
    c"             Can be used on non-NUMA systems for debugging.".as_ptr(),
    c"".as_ptr(),
    c"  --install-signal-handlers=<yes|no>".as_ptr(),
    c"             Install signal handlers (default: yes)".as_ptr(),
    c"  --io-manager=<name>".as_ptr(),
    c"             The I/O manager to use.".as_ptr(),
    c"             Options available: auto mio (default: mio)".as_ptr(),
    c"  -e<n>      Maximum number of outstanding local sparks (default: 4096)".as_ptr(),
    c"  -xq        The allocation limit given to a thread after it receives".as_ptr(),
    c"             an AllocationLimitExceeded exception. (default: 100k)".as_ptr(),
    c"".as_ptr(),
    c"  -xr        The size of virtual memory address space reserved by the".as_ptr(),
    c"             two step allocator (default: 1T)".as_ptr(),
    c"".as_ptr(),
    c"  -Mgrace=<n>".as_ptr(),
    c"             The amount of allocation after the program receives a".as_ptr(),
    c"             HeapOverflow exception before the exception is thrown again, if"
        .as_ptr(),
    c"             the program is still exceeding the heap limit.".as_ptr(),
    c"".as_ptr(),
    c"  --read-tix-file=<yes|no>".as_ptr(),
    c"             Whether to initialize HPC datastructures from  <program>.tix              at the start of execution. (default: yes)"
        .as_ptr(),
    c"".as_ptr(),
    c"  --write-tix-file=<yes|no>".as_ptr(),
    c"             Whether to write <program>.tix at the end of execution.".as_ptr(),
    c"             (default: yes)".as_ptr(),
    c"".as_ptr(),
    c"RTS options may also be specified using the GHCRTS environment variable.".as_ptr(),
    c"".as_ptr(),
    c"Other RTS options may be available for programs compiled a different way."
        .as_ptr(),
    c"The GHC User's Guide has full details.".as_ptr(),
    c"".as_ptr(),
    null::<c_char>(),
];

unsafe fn strequal(a: *const c_char, b: *const c_char) -> bool {
    libc::strcmp(a, b) == 0
}

unsafe fn appendRtsArg(mut arg: *mut c_char) {
    if rts_argc == rts_argv_size {
        rts_argv_size *= 2;

        rts_argv = stgReallocBytes(
            rts_argv as *mut c_void,
            (rts_argv_size as usize).wrapping_mul(size_of::<*mut c_char>() as usize),
            c"RtsFlags.c:appendRtsArg".as_ptr(),
        ) as *mut *mut c_char;
    }

    let fresh12 = rts_argc;
    rts_argc += 1;

    let ref mut fresh13 = *rts_argv.offset(fresh12 as isize);
    *fresh13 = arg;
}

unsafe fn splitRtsFlags(mut s: *const c_char) {
    let mut c1 = null::<c_char>();
    let mut c2 = null::<c_char>();
    let mut t = null_mut::<c_char>();
    c1 = s;

    loop {
        while libc::isspace(*c1 as i32) != 0 {
            c1 = c1.offset(1);
        }

        c2 = c1;

        while libc::isspace(*c2 as i32) == 0 && *c2 as i32 != '\0' as i32 {
            c2 = c2.offset(1);
        }

        if c1 == c2 {
            break;
        }

        t = stgMallocBytes(
            (c2.offset_from(c1) as i64 + 1) as usize,
            c"RtsFlags.c:splitRtsFlags()".as_ptr(),
        ) as *mut c_char;

        libc::strncpy(t, c1, c2.offset_from(c1) as i64 as usize);
        *t.offset(c2.offset_from(c1) as i64 as isize) = '\0' as i32 as c_char;
        appendRtsArg(argv, t);
        c1 = c2;

        if !(*c1 as i32 != '\0' as i32) {
            break;
        }
    }
}

unsafe fn errorRtsOptsDisabled(mut s: *const c_char) {
    let mut advice;

    if rtsConfig.rts_hs_main != 0 {
        advice = c"Link with -rtsopts to enable them.".as_ptr();
    } else {
        advice = c"Use hs_init_with_rtsopts() to enable them.".as_ptr();
    }

    errorBelch(s, advice);
}

unsafe fn setupRtsFlags(mut argc: *mut i32, mut argv: *mut *mut c_char, mut rts_config: RtsConfig) {
    let mut mode: u32 = 0;
    let mut total_arg: u32 = *argc as u32;
    let mut arg: u32 = 1;
    let mut rts_argc0: u32 = 0;
    rtsConfig = rts_config;
    setProgName(argv);

    if *argc > 1 {
        *argc = 1;
    }

    rts_argc = 0;
    rts_argv_size = total_arg.wrapping_add(1 as u32) as i32;

    rts_argv = stgMallocBytes(
        (rts_argv_size as usize).wrapping_mul(size_of::<*mut c_char>() as usize),
        c"setupRtsFlags".as_ptr(),
    ) as *mut *mut c_char;

    rts_argc0 = rts_argc as u32;

    if !rtsConfig.rts_opts.is_null() {
        splitRtsFlags(rtsConfig.rts_opts);
        procRtsOpts(rts_argc0 as i32, RtsOptsAll);
        rts_argc0 = rts_argc as u32;
    }

    if rtsConfig.rts_opts_enabled as u32 != RtsOptsIgnoreAll as i32 as u32 {
        let mut ghc_rts = libc::getenv(c"GHCRTS".as_ptr());

        if !ghc_rts.is_null() {
            if rtsConfig.rts_opts_enabled as u32 == RtsOptsNone as i32 as u32 {
                errorRtsOptsDisabled(
                    c"Warning: Ignoring GHCRTS variable as RTS options are disabled.\n         %s"
                        .as_ptr(),
                );
            } else {
                splitRtsFlags(ghc_rts);
                procRtsOpts(rts_argc0 as i32, rtsConfig.rts_opts_enabled);
                rts_argc0 = rts_argc as u32;
            }
        }
    }

    if !(rtsConfig.rts_opts_enabled as u32 == RtsOptsIgnoreAll as i32 as u32
        || rtsConfig.rts_opts_enabled as u32 == RtsOptsIgnore as i32 as u32)
    {
        mode = PGM as u32;

        while arg < total_arg {
            if strequal(c"--RTS".as_ptr(), *argv.offset(arg as isize)) {
                arg = arg.wrapping_add(1);
                break;
            } else {
                if strequal(c"--".as_ptr(), *argv.offset(arg as isize)) {
                    break;
                }

                if strequal(c"+RTS".as_ptr(), *argv.offset(arg as isize)) {
                    mode = RTS as u32;
                } else if strequal(c"-RTS".as_ptr(), *argv.offset(arg as isize)) {
                    mode = PGM as u32;
                } else if mode == RTS as u32 {
                    appendRtsArg(copyArg(*argv.offset(arg as isize)));
                } else {
                    let fresh0 = *argc;
                    *argc = *argc + 1;

                    let ref mut fresh1 = *argv.offset(fresh0 as isize);
                    *fresh1 = *argv.offset(arg as isize);
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
    procRtsOpts(rts_argc0 as i32, rtsConfig.rts_opts_enabled);
    appendRtsArg(null_mut::<c_char>());
    rts_argc -= 1;
    normaliseRtsOpts();
    setProgArgv(*argc, argv);

    if let Some(statsFile) = RtsFlags.GcFlags.statsFile {
        initStatsFile(statsFile);
    }

    if let Some(tickyFile) = RtsFlags.TickyFlags.tickyFile {
        initStatsFile(tickyFile);
    }
}

unsafe fn checkSuid(mut enabled: RtsOptsEnabledEnum) {
    if enabled as u32 == RtsOptsSafeOnly as i32 as u32 {
        if libc::getuid() != libc::geteuid() || libc::getgid() != libc::getegid() {
            errorRtsOptsDisabled(c"RTS options are disabled for setuid binaries. %s".as_ptr());

            stg_exit(EXIT_FAILURE);
        }
    }
}

unsafe fn checkUnsafe(mut enabled: RtsOptsEnabledEnum) {
    if enabled as u32 == RtsOptsSafeOnly as i32 as u32 {
        errorRtsOptsDisabled(c"Most RTS options are disabled. %s".as_ptr());
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn procRtsOpts(mut rts_argc0: i32, mut rtsOptsEnabled: RtsOptsEnabledEnum) {
    let mut current_block: u64;
    let mut error = false;
    let mut arg: i32 = 0;
    let mut unchecked_arg_start: i32 = 0;

    if !(rts_argc0 < rts_argc) {
        return;
    }

    if rtsOptsEnabled as u32 == RtsOptsNone as i32 as u32 {
        errorRtsOptsDisabled(c"RTS options are disabled. %s".as_ptr());
        stg_exit(EXIT_FAILURE);
    }

    checkSuid(rtsOptsEnabled);
    arg = rts_argc0;

    while arg < rts_argc {
        let mut option_checked = false;

        if *(*rts_argv.offset(arg as isize)).offset(0) as i32 != '-' as i32 {
            io::stdout().flush();

            errorBelch(
                c"unexpected RTS argument: %s".as_ptr(),
                *rts_argv.offset(arg as isize),
            );

            error = true;
        } else {
            unchecked_arg_start = 1;

            match *(*rts_argv.offset(arg as isize)).offset(1) as i32 {
                63 => {
                    option_checked = true;
                    error = true;
                    current_block = 1841612074772515791;
                }
                45 => {
                    if strequal(
                        c"install-signal-handlers=yes".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.install_signal_handlers = true;
                    } else if strequal(
                        c"install-signal-handlers=no".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.install_signal_handlers = false;
                    } else if strequal(
                        c"install-seh-handlers=yes".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.install_seh_handlers = true;
                    } else if strequal(
                        c"install-seh-handlers=no".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.install_seh_handlers = false;
                    } else if strequal(
                        c"generate-stack-traces=yes".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.generate_stack_trace = true;
                    } else if strequal(
                        c"generate-stack-traces=no".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.generate_stack_trace = false;
                    } else if strequal(
                        c"generate-crash-dumps".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.generate_dump_file = true;
                    } else if strequal(
                        c"optimistic-linking".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.linkerOptimistic = true;
                    } else if strequal(
                        c"null-eventlog-writer".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.TraceFlags.nullWriter = true;
                    } else if strequal(
                        c"machine-readable".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.machineReadable = true;
                    } else if strequal(
                        c"disable-delayed-os-memory-return".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.MiscFlags.disableDelayedOsMemoryReturn = true;
                    } else if strequal(
                        c"internal-counters".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.MiscFlags.internalCounters = true;
                    } else if libc::strncmp(
                        c"io-manager=".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        11,
                    ) == 0
                    {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;

                        let mut iomgrstr: *mut c_char =
                            (*rts_argv.offset(arg as isize)).offset(13) as *mut c_char;

                        let mut iomgrflag = _IO_MANAGER_FLAG::IO_MNGR_FLAG_AUTO;
                        let mut availability = IOManagerAvailable;
                        availability = parseIOManagerFlag(iomgrstr, &raw mut iomgrflag);

                        if availability as u32 == IOManagerAvailable as i32 as u32 {
                            RtsFlags.MiscFlags.ioManager = iomgrflag;
                        } else {
                            errorBelch(
                                c"%s choice '%s' for --io-manager=\nThe choices are: auto%s"
                                    .as_ptr(),
                                if availability as u32 == IOManagerUnavailable as i32 as u32 {
                                    c"unavailable".as_ptr()
                                } else {
                                    c"unrecognised".as_ptr()
                                },
                                iomgrstr,
                                c" mio".as_ptr(),
                            );

                            stg_exit(EXIT_FAILURE);
                        }
                    } else if strequal(
                        c"info".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        printRtsInfo(rtsConfig);
                        stg_exit(0);
                    } else if libc::strncmp(
                        c"eventlog-flush-interval=".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        24,
                    ) == 0
                    {
                        option_checked = true;

                        let mut intervalSeconds = parseDouble(
                            (*rts_argv.offset(arg as isize)).offset(26),
                            &raw mut error,
                        );

                        if error {
                            errorBelch(c"bad value for --eventlog-flush-interval".as_ptr());
                        }

                        RtsFlags.TraceFlags.eventlogFlushTime = fsecondsToTime(intervalSeconds);
                    } else if strequal(
                        c"copying-gc".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.GcFlags.useNonmoving = false;
                    } else if strequal(
                        c"nonmoving-gc".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.GcFlags.useNonmoving = true;
                    } else if libc::strncmp(
                        c"nonmoving-dense-allocator-count=".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        32,
                    ) == 0
                    {
                        option_checked = true;

                        let mut threshold: i32 = libc::strtol(
                            (*rts_argv.offset(arg as isize)).offset(34),
                            null_mut(),
                            10,
                        ) as i32;

                        if threshold < 1 || threshold > u16::MAX as i32 {
                            errorBelch(c"bad value for --nonmoving-dense-allocator-count".as_ptr());

                            error = true;
                        } else {
                            RtsFlags.GcFlags.nonmovingDenseAllocatorCount = threshold as u16;
                        }
                    } else if strequal(
                        c"read-tix-file=yes".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.HpcFlags.readTixFile = _HPC_READ_FILE::HPC_YES_EXPLICIT;
                    } else if strequal(
                        c"read-tix-file=no".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.HpcFlags.readTixFile = _HPC_READ_FILE::HPC_NO_EXPLICIT;
                    } else if strequal(
                        c"write-tix-file=yes".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.HpcFlags.writeTixFile = true;
                    } else if strequal(
                        c"write-tix-file=no".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                        RtsFlags.HpcFlags.writeTixFile = false;
                    } else if libc::strncmp(
                        c"numa".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        4,
                    ) == 0
                    {
                        if !osBuiltWithNumaSupport() {
                            errorBelch(
                                c"%s: This GHC build was compiled without NUMA support.".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = true;
                        } else {
                            option_checked = true;

                            let mut mask: StgWord = 0;

                            if *(*rts_argv.offset(arg as isize)).offset(6) as i32 == '=' as i32 {
                                mask = libc::strtol(
                                    (*rts_argv.offset(arg as isize)).offset(7),
                                    null_mut::<*mut c_char>(),
                                    10,
                                ) as StgWord;

                                current_block = 17808209642927821499;
                            } else if *(*rts_argv.offset(arg as isize)).offset(6) as i32
                                == '\0' as i32
                            {
                                mask = !0 as StgWord;
                                current_block = 17808209642927821499;
                            } else {
                                errorBelch(
                                    c"%s: unknown flag".as_ptr(),
                                    *rts_argv.offset(arg as isize),
                                );

                                error = true;
                                current_block = 1841612074772515791;
                            }

                            match current_block {
                                1841612074772515791 => {}
                                _ => {
                                    if !osNumaAvailable() {
                                        errorBelch(
                                            c"%s: OS reports NUMA is not available".as_ptr(),
                                            *rts_argv.offset(arg as isize),
                                        );

                                        error = true;
                                    } else {
                                        RtsFlags.GcFlags.numa = true;
                                        RtsFlags.GcFlags.numaMask = mask;
                                    }
                                }
                            }
                        }
                    } else if libc::strncmp(
                        c"debug-numa".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        10,
                    ) == 0
                    {
                        option_checked = true;

                        let mut nNodes: usize = 0;

                        if *(*rts_argv.offset(arg as isize)).offset(12) as i32 == '=' as i32
                            && libc::isdigit(*(*rts_argv.offset(arg as isize)).offset(13) as i32)
                                != 0
                        {
                            nNodes = libc::strtol(
                                (*rts_argv.offset(arg as isize)).offset(13),
                                null_mut::<*mut c_char>(),
                                10,
                            ) as StgWord as usize;

                            if nNodes > MAX_NUMA_NODES as usize {
                                errorBelch(
                                    c"%s: Too many NUMA nodes (max %d)".as_ptr(),
                                    *rts_argv.offset(arg as isize),
                                    MAX_NUMA_NODES,
                                );

                                error = true;
                            } else {
                                RtsFlags.GcFlags.numa = true;
                                RtsFlags.DebugFlags.numa = true;
                                RtsFlags.GcFlags.numaMask = ((1 << nNodes) - 1) as StgWord;
                                n_numa_nodes = nNodes as u32;
                            }
                        } else {
                            errorBelch(
                                c"%s: missing number of nodes".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = true;
                        }
                    } else if libc::strncmp(
                        c"long-gc-sync=".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                        13,
                    ) == 0
                    {
                        option_checked = true;

                        if !(*(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32) {
                            RtsFlags.GcFlags.longGCSync = fsecondsToTime(libc::atof(
                                (*rts_argv.offset(arg as isize)).offset(16),
                            ));
                        }
                    } else if strequal(
                        c"no-automatic-heap-samples".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.ProfFlags.startHeapProfileAtStartup = false;
                    } else if strequal(
                        c"no-automatic-time-samples".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.ProfFlags.startTimeProfileAtStartup = false;
                    } else if strequal(
                        c"automatic-era-increment".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2) as *mut c_char,
                    ) {
                        option_checked = true;
                        RtsFlags.ProfFlags.incrementUserEra = true;
                    } else {
                        option_checked = true;

                        errorBelch(
                            c"unknown RTS option: %s".as_ptr(),
                            *rts_argv.offset(arg as isize),
                        );

                        error = true;
                    }

                    current_block = 1841612074772515791;
                }
                65 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 == 'L' as i32 {
                        RtsFlags.GcFlags.largeAllocLim = decodeSize(
                            *rts_argv.offset(arg as isize),
                            3 as u32,
                            (2 as StgWord64).wrapping_mul(BLOCK_SIZE as StgWord64),
                            HS_INT_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as u32;
                    } else {
                        RtsFlags.GcFlags.minAllocAreaSize = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as u32,
                            (2 as StgWord64).wrapping_mul(BLOCK_SIZE as StgWord64),
                            HS_INT_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as u32;
                    }

                    current_block = 1841612074772515791;
                }
                110 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    RtsFlags.GcFlags.nurseryChunkSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as u32,
                        (2 as StgWord64).wrapping_mul(BLOCK_SIZE as StgWord64),
                        HS_INT_MAX as StgWord64,
                    )
                    .wrapping_div(BLOCK_SIZE as StgWord64)
                        as u32;
                    current_block = 1841612074772515791;
                }
                66 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;
                    RtsFlags.GcFlags.ringBell = true;
                    unchecked_arg_start += 1;
                    current_block = 6015864261243718670;
                }
                99 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 != '\0' as i32 {
                        RtsFlags.GcFlags.compactThreshold =
                            libc::atof((*rts_argv.offset(arg as isize)).offset(2));
                    } else {
                        RtsFlags.GcFlags.compact = true;
                    }

                    current_block = 1841612074772515791;
                }
                119 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;
                    RtsFlags.GcFlags.sweep = true;
                    unchecked_arg_start += 1;
                    current_block = 6015864261243718670;
                }
                70 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        100 => {
                            RtsFlags.GcFlags.returnDecayFactor =
                                libc::atof((*rts_argv.offset(arg as isize)).offset(3));

                            if RtsFlags.GcFlags.returnDecayFactor < 0.0 {
                                bad_option(*rts_argv.offset(arg as isize));
                            }
                        }
                        _ => {
                            RtsFlags.GcFlags.oldGenFactor =
                                libc::atof((*rts_argv.offset(arg as isize)).offset(2));

                            if RtsFlags.GcFlags.oldGenFactor < 0.0 {
                                bad_option(*rts_argv.offset(arg as isize));
                            }
                        }
                    }

                    current_block = 1841612074772515791;
                }
                68 => {
                    option_checked = true;
                    read_debug_flags(*rts_argv.offset(arg as isize));
                    current_block = 1841612074772515791;
                }
                75 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    RtsFlags.GcFlags.maxStkSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as u32,
                        0 as StgWord64,
                        u32::MAX as StgWord64,
                    )
                    .wrapping_div(size_of::<W_>() as StgWord64)
                        as u32;
                    current_block = 1841612074772515791;
                }
                107 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        99 => {
                            RtsFlags.GcFlags.stkChunkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as u32,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as u32;
                        }
                        98 => {
                            RtsFlags.GcFlags.stkChunkBufferSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as u32,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as u32;
                        }
                        105 => {
                            RtsFlags.GcFlags.initialStkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as u32,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as u32;
                        }
                        _ => {
                            RtsFlags.GcFlags.initialStkSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                2 as u32,
                                size_of::<W_>() as StgWord64,
                                HS_WORD_MAX as StgWord64,
                            )
                            .wrapping_div(size_of::<W_>() as StgWord64)
                                as u32;
                        }
                    }

                    current_block = 1841612074772515791;
                }
                77 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if 0 == libc::strncmp(
                        c"grace=".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(2),
                        6,
                    ) {
                        RtsFlags.GcFlags.heapLimitGrace = decodeSize(
                            *rts_argv.offset(arg as isize),
                            8,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        ) as StgWord;
                    } else {
                        RtsFlags.GcFlags.maxHeapSize = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as u32,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as u32;
                    }

                    current_block = 1841612074772515791;
                }
                109 => {
                    if libc::strncmp(
                        c"maxN".as_ptr(),
                        (*rts_argv.offset(arg as isize)).offset(1) as *mut c_char,
                        4,
                    ) == 0
                    {
                        option_checked = true;

                        let mut nCapabilities: i32 = 0;
                        let mut proc = getNumberOfProcessors() as i32;

                        nCapabilities = libc::strtol(
                            (*rts_argv.offset(arg as isize)).offset(5),
                            null_mut::<c_void>() as *mut *mut c_char,
                            10,
                        ) as i32;

                        if nCapabilities > proc {
                            nCapabilities = proc;
                        }

                        if nCapabilities <= 0 {
                            errorBelch(c"bad value for -maxN".as_ptr());
                            error = 1 != 0;
                        }

                        RtsFlags.ParFlags.nCapabilities = 1;
                    } else {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;

                        RtsFlags.GcFlags.pcFreeHeap =
                            libc::atof((*rts_argv.offset(arg as isize)).offset(2));

                        if RtsFlags.GcFlags.pcFreeHeap == 0.0f64
                            && *(*rts_argv.offset(arg as isize)).offset(2) as i32 != '0' as i32
                        {
                            bad_option(*rts_argv.offset(arg as isize));
                        }

                        if RtsFlags.GcFlags.pcFreeHeap < 0 || RtsFlags.GcFlags.pcFreeHeap > 100 {
                            bad_option(*rts_argv.offset(arg as isize));
                        }
                    }

                    current_block = 1841612074772515791;
                }
                71 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    RtsFlags.GcFlags.generations = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2,
                        1,
                        HS_INT_MAX as StgWord64,
                    ) as u32;

                    current_block = 1841612074772515791;
                }
                72 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32 {
                        RtsFlags.GcFlags.heapSizeSuggestionAuto = true;
                    } else {
                        RtsFlags.GcFlags.heapSizeSuggestion = decodeSize(
                            *rts_argv.offset(arg as isize),
                            2 as u32,
                            BLOCK_SIZE as StgWord64,
                            HS_WORD_MAX as StgWord64,
                        )
                        .wrapping_div(BLOCK_SIZE as StgWord64)
                            as u32;
                    }

                    current_block = 1841612074772515791;
                }
                79 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    RtsFlags.GcFlags.minOldGenSize = decodeSize(
                        *rts_argv.offset(arg as isize),
                        2 as u32,
                        BLOCK_SIZE as StgWord64,
                        HS_WORD_MAX as StgWord64,
                    )
                    .wrapping_div(BLOCK_SIZE as StgWord64)
                        as u32;
                    current_block = 1841612074772515791;
                }
                73 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        119 => {
                            if !(*(*rts_argv.offset(arg as isize)).offset(3) as i32 == '\0' as i32)
                            {
                                RtsFlags.GcFlags.interIdleGCWait = fsecondsToTime(libc::atof(
                                    (*rts_argv.offset(arg as isize)).offset(3),
                                ));
                            }
                        }
                        0 => {}
                        _ => {
                            let mut t = fsecondsToTime(libc::atof(
                                (*rts_argv.offset(arg as isize)).offset(2),
                            ));

                            if t == 0 {
                                RtsFlags.GcFlags.doIdleGC = false;
                            } else {
                                RtsFlags.GcFlags.doIdleGC = true;
                                RtsFlags.GcFlags.idleGCDelayTime = t;
                            }
                        }
                    }

                    current_block = 1841612074772515791;
                }
                84 => {
                    option_checked = true;
                    RtsFlags.GcFlags.giveStats = COLLECT_GC_STATS as u32;
                    unchecked_arg_start += 1;
                    current_block = 6015864261243718670;
                }
                83 => {
                    option_checked = true;
                    RtsFlags.GcFlags.giveStats = VERBOSE_GC_STATS as u32;
                    current_block = 4518748666845553757;
                }
                115 => {
                    option_checked = true;
                    RtsFlags.GcFlags.giveStats = SUMMARY_GC_STATS as u32;
                    current_block = 4518748666845553757;
                }
                116 => {
                    option_checked = true;
                    RtsFlags.GcFlags.giveStats = ONELINE_GC_STATS as u32;
                    current_block = 4518748666845553757;
                }
                90 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;
                    RtsFlags.GcFlags.squeezeUpdFrames = false;
                    unchecked_arg_start += 1;
                    current_block = 6015864261243718670;
                }
                80 | 112 => {
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        97 => {
                            RtsFlags.CcFlags.doCostCentres = 3;

                            if *(*rts_argv.offset(arg as isize)).offset(3) as i32 != '\0' as i32 {
                                errorBelch(
                                    c"flag -Pa given an argument when none was expected: %s"
                                        .as_ptr(),
                                    *rts_argv.offset(arg as isize),
                                );

                                error = 1 != 0;
                            }

                            current_block = 1841612074772515791;
                        }
                        106 => {
                            RtsFlags.CcFlags.doCostCentres = 4;
                            current_block = 1841612074772515791;
                        }
                        111 => {
                            if *(*rts_argv.offset(arg as isize)).offset(3) as i32 == '\0' as i32 {
                                errorBelch(c"flag -po expects an argument".as_ptr());
                                error = 1 != 0;
                            } else {
                                RtsFlags.CcFlags.outputFileNameStem =
                                    (*rts_argv.offset(arg as isize)).offset(3);
                            }

                            current_block = 1841612074772515791;
                        }
                        0 => {
                            if *(*rts_argv.offset(arg as isize)).offset(1) as i32 == 'P' as i32 {
                                RtsFlags.CcFlags.doCostCentres = 2;
                            } else {
                                RtsFlags.CcFlags.doCostCentres = 1;
                            }

                            current_block = 1841612074772515791;
                        }
                        _ => {
                            unchecked_arg_start += 1;
                            current_block = 6015864261243718670;
                        }
                    }
                }
                82 => {
                    option_checked = true;

                    RtsFlags.ProfFlags.maxRetainerSetSize =
                        libc::atof((*rts_argv.offset(arg as isize)).offset(2)) as u32;

                    current_block = 1841612074772515791;
                }
                76 => {
                    option_checked = true;

                    RtsFlags.ProfFlags.ccsLength =
                        libc::atof((*rts_argv.offset(arg as isize)).offset(2)) as u32;

                    if RtsFlags.ProfFlags.ccsLength <= 0 {
                        bad_option(*rts_argv.offset(arg as isize));
                    }

                    current_block = 1841612074772515791;
                }
                104 => {
                    option_checked = true;
                    error = read_heap_profiling_flag(*rts_argv.offset(arg as isize));
                    current_block = 1841612074772515791;
                }
                105 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if !(*(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32) {
                        let mut intervalSeconds_0 =
                            parseDouble((*rts_argv.offset(arg as isize)).offset(2), &raw mut error);

                        if error {
                            errorBelch(c"bad value for -i".as_ptr());
                        }

                        RtsFlags.ProfFlags.heapProfileInterval = fsecondsToTime(intervalSeconds_0);
                    }

                    current_block = 1841612074772515791;
                }
                67 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32 {
                        RtsFlags.ConcFlags.ctxtSwitchTime = 0;
                    } else {
                        let mut intervalSeconds_1 =
                            parseDouble((*rts_argv.offset(arg as isize)).offset(2), &raw mut error);

                        if error {
                            errorBelch(c"bad value for -C".as_ptr());
                        }

                        RtsFlags.ConcFlags.ctxtSwitchTime = fsecondsToTime(intervalSeconds_1);
                    }

                    current_block = 1841612074772515791;
                }
                86 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32 {
                        RtsFlags.MiscFlags.tickInterval = 0;
                    } else {
                        let mut intervalSeconds_2 =
                            parseDouble((*rts_argv.offset(arg as isize)).offset(2), &raw mut error);

                        if error {
                            errorBelch(c"bad value for -V".as_ptr());
                        }

                        RtsFlags.MiscFlags.tickInterval = fsecondsToTime(intervalSeconds_2);
                    }

                    current_block = 1841612074772515791;
                }
                78 => {
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 == '\0' as i32 {
                        RtsFlags.ParFlags.nCapabilities = getNumberOfProcessors();
                    } else {
                        let mut nCapabilities_0: i32 = 0;
                        option_checked = 1 != 0;

                        nCapabilities_0 = libc::strtol(
                            (*rts_argv.offset(arg as isize)).offset(2),
                            null_mut::<c_void>() as *mut *mut c_char,
                            10,
                        ) as i32;

                        if nCapabilities_0 <= 0 {
                            errorBelch(c"bad value for -N".as_ptr());
                            error = 1 != 0;
                        }

                        if rtsOptsEnabled as u32 == RtsOptsSafeOnly as i32 as u32
                            && nCapabilities_0 > getNumberOfProcessors() as i32
                        {
                            errorRtsOptsDisabled(
                                c"Using large values for -N is not allowed by default. %s".as_ptr(),
                            );

                            stg_exit(1);
                        }

                        RtsFlags.ParFlags.nCapabilities = nCapabilities_0 as u32;
                    }

                    current_block = 1841612074772515791;
                }
                103 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        49 => {
                            RtsFlags.ParFlags.parGcEnabled = 0 != 0;
                        }
                        _ => {
                            errorBelch(
                                c"unknown RTS option: %s".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = 1 != 0;
                        }
                    }

                    current_block = 1841612074772515791;
                }
                113 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        0 => {
                            errorBelch(
                                c"incomplete RTS option: %s".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = 1 != 0;
                        }
                        103 => {
                            if *(*rts_argv.offset(arg as isize)).offset(3) as i32 == '\0' as i32 {
                                RtsFlags.ParFlags.parGcEnabled = 0 != 0;
                            } else {
                                RtsFlags.ParFlags.parGcEnabled = 1 != 0;

                                RtsFlags.ParFlags.parGcGen = libc::strtol(
                                    (*rts_argv.offset(arg as isize)).offset(3),
                                    null_mut::<c_void>() as *mut *mut c_char,
                                    10,
                                )
                                    as u32;
                            }
                        }
                        98 => {
                            if *(*rts_argv.offset(arg as isize)).offset(3) as i32 == '\0' as i32 {
                                RtsFlags.ParFlags.parGcLoadBalancingEnabled = 0 != 0;
                            } else {
                                RtsFlags.ParFlags.parGcLoadBalancingEnabled = 1 != 0;

                                RtsFlags.ParFlags.parGcLoadBalancingGen = libc::strtol(
                                    (*rts_argv.offset(arg as isize)).offset(3),
                                    null_mut::<c_void>() as *mut *mut c_char,
                                    10,
                                )
                                    as u32;
                            }
                        }
                        105 => {
                            RtsFlags.ParFlags.parGcNoSyncWithIdle = libc::strtol(
                                (*rts_argv.offset(arg as isize)).offset(3),
                                null_mut::<c_void>() as *mut *mut c_char,
                                10,
                            )
                                as u32;
                        }
                        110 => {
                            let mut threads: i32 = 0;

                            threads = libc::strtol(
                                (*rts_argv.offset(arg as isize)).offset(3),
                                null_mut::<c_void>() as *mut *mut c_char,
                                10,
                            ) as i32;

                            if threads <= 0 {
                                errorBelch(c"-qn must be 1 or greater".as_ptr());
                                error = 1 != 0;
                            } else {
                                RtsFlags.ParFlags.parGcThreads = threads as u32;
                            }
                        }
                        97 => {
                            RtsFlags.ParFlags.setAffinity = 1 != 0;
                        }
                        109 => {
                            RtsFlags.ParFlags.migrate = 0 != 0;
                        }
                        119 => {}
                        _ => {
                            errorBelch(
                                c"unknown RTS option: %s".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = 1 != 0;
                        }
                    }

                    current_block = 1841612074772515791;
                }
                101 => {
                    checkUnsafe(rtsOptsEnabled);
                    option_checked = true;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 != '\0' as i32 {
                        RtsFlags.ParFlags.maxLocalSparks = libc::strtol(
                            (*rts_argv.offset(arg as isize)).offset(2),
                            null_mut::<c_void>() as *mut *mut c_char,
                            10,
                        ) as u32;

                        if RtsFlags.ParFlags.maxLocalSparks <= 0 {
                            errorBelch(c"bad value for -e".as_ptr());
                            error = 1 != 0;
                        }
                    }

                    current_block = 1841612074772515791;
                }
                114 => {
                    option_checked = true;
                    RtsFlags.TickyFlags.showTickyStats = 1 != 0;

                    let mut r_0: i32 = 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 != '\0' as i32 {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = 1 != 0;
                    }

                    r_0 = openStatsFile(
                        (*rts_argv.offset(arg as isize)).offset(2),
                        &format!("{prog_name}.ticky"),
                        &mut RtsFlags.TickyFlags.tickyFile,
                    );

                    if r_0 == -1 {
                        error = 1 != 0;
                    }

                    current_block = 1841612074772515791;
                }
                111 => {
                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        108 => {
                            option_checked = true;

                            if libc::strlen(
                                (*rts_argv.offset(arg as isize)).offset(3) as *mut c_char
                            ) == 0
                            {
                                errorBelch(c"-ol expects filename".as_ptr());
                                error = 1 != 0;
                            } else {
                                RtsFlags.TraceFlags.trace_output = libc::strdup(
                                    (*rts_argv.offset(arg as isize)).offset(3) as *mut c_char,
                                );
                            }
                        }
                        _ => {
                            errorBelch(
                                c"Unknown output flag -o%c".as_ptr(),
                                *(*rts_argv.offset(arg as isize)).offset(2) as i32,
                            );

                            error = true;
                        }
                    }

                    current_block = 1841612074772515791;
                }
                108 => {
                    option_checked = true;
                    RtsFlags.TraceFlags.tracing = 1;

                    read_trace_flags((*rts_argv.offset(arg as isize)).offset(2) as *mut c_char);

                    current_block = 1841612074772515791;
                }
                118 => {
                    option_checked = true;
                    RtsFlags.TraceFlags.tracing = 2;

                    read_trace_flags((*rts_argv.offset(arg as isize)).offset(2) as *mut c_char);

                    current_block = 1841612074772515791;
                }
                120 => {
                    unchecked_arg_start += 1;

                    match *(*rts_argv.offset(arg as isize)).offset(2) as i32 {
                        0 => {
                            option_checked = true;

                            errorBelch(
                                c"incomplete RTS option: %s".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = true;
                            current_block = 1841612074772515791;
                        }
                        98 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = true;

                            if *(*rts_argv.offset(arg as isize)).offset(3) as i32 != '\0' as i32 {
                                RtsFlags.GcFlags.heapBase = libc::strtoull(
                                    (*rts_argv.offset(arg as isize)).offset(3),
                                    null_mut(),
                                    0,
                                )
                                    as StgWord;
                            } else {
                                errorBelch(c"-xb: requires argument".as_ptr());
                                error = true;
                            }

                            current_block = 1841612074772515791;
                        }
                        110 => {
                            option_checked = true;
                            RtsFlags.GcFlags.useNonmoving = true;
                            unchecked_arg_start += 1;
                            current_block = 1841612074772515791;
                        }
                        99 => {
                            option_checked = true;
                            RtsFlags.ProfFlags.showCCSOnException = 1 != 0;
                            unchecked_arg_start += 1;
                            current_block = 6015864261243718670;
                        }
                        116 => {
                            option_checked = true;

                            errorBelch(c"The -xt option has been removed (#16795)".as_ptr());

                            error = true;
                            current_block = 1841612074772515791;
                        }
                        113 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = true;

                            RtsFlags.GcFlags.allocLimitGrace = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3 as u32,
                                BLOCK_SIZE as StgWord64,
                                HS_INT_MAX as StgWord64,
                            )
                            .wrapping_div(BLOCK_SIZE as StgWord64)
                                as StgWord;
                            current_block = 1841612074772515791;
                        }
                        114 => {
                            checkUnsafe(rtsOptsEnabled);
                            option_checked = true;

                            RtsFlags.GcFlags.addressSpaceSize = decodeSize(
                                *rts_argv.offset(arg as isize),
                                3,
                                MBLOCK_SIZE as StgWord64,
                                HS_WORD64_MAX as StgWord64,
                            );

                            current_block = 1841612074772515791;
                        }
                        _ => {
                            option_checked = true;

                            errorBelch(
                                c"unknown RTS option: %s".as_ptr(),
                                *rts_argv.offset(arg as isize),
                            );

                            error = true;
                            current_block = 1841612074772515791;
                        }
                    }
                }
                _ => {
                    option_checked = true;

                    errorBelch(
                        c"unknown RTS option: %s".as_ptr(),
                        *rts_argv.offset(arg as isize),
                    );

                    error = true;
                    current_block = 1841612074772515791;
                }
            }

            match current_block {
                6015864261243718670 => {
                    if *(*rts_argv.offset(arg as isize)).offset(unchecked_arg_start as isize) as i32
                        != '\0' as i32
                    {
                        errorBelch(
                            c"flag -%c given an argument when none was expected: %s".as_ptr(),
                            *(*rts_argv.offset(arg as isize)).offset(1) as i32,
                            *rts_argv.offset(arg as isize),
                        );

                        error = true;
                    }
                }
                4518748666845553757 => {
                    let mut r: i32 = 0;

                    if *(*rts_argv.offset(arg as isize)).offset(2) as i32 != '\0' as i32 {
                        checkUnsafe(rtsOptsEnabled);
                        option_checked = true;
                    }

                    r = openStatsFile(
                        (*rts_argv.offset(arg as isize)).offset(2),
                        null::<c_char>(),
                        &mut RtsFlags.GcFlags.statsFile,
                    );

                    if r == -1 {
                        error = true;
                    }
                }
                _ => {}
            }

            if !option_checked {
                errorBelch(c"Internal error in the RTS options parser".as_ptr());
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
    if RtsFlags.MiscFlags.tickInterval < 0 {
        RtsFlags.MiscFlags.tickInterval = DEFAULT_TICK_INTERVAL;
    }

    if RtsFlags.MiscFlags.tickInterval == 0 {
        RtsFlags.ConcFlags.ctxtSwitchTime = 0;
        RtsFlags.GcFlags.idleGCDelayTime = 0;
        RtsFlags.ProfFlags.heapProfileInterval = 0;
    }

    if RtsFlags.ConcFlags.ctxtSwitchTime > 0 {
        RtsFlags.MiscFlags.tickInterval = RtsFlags
            .ConcFlags
            .ctxtSwitchTime
            .min(RtsFlags.MiscFlags.tickInterval);
    }

    if RtsFlags.GcFlags.idleGCDelayTime > 0 {
        RtsFlags.MiscFlags.tickInterval = RtsFlags
            .GcFlags
            .idleGCDelayTime
            .min(RtsFlags.MiscFlags.tickInterval);
    }

    if RtsFlags.ProfFlags.heapProfileInterval > 0 {
        RtsFlags.MiscFlags.tickInterval = RtsFlags
            .ProfFlags
            .heapProfileInterval
            .min(RtsFlags.MiscFlags.tickInterval);
    }

    if RtsFlags.ConcFlags.ctxtSwitchTime > 0 && RtsFlags.MiscFlags.tickInterval != 0 {
        RtsFlags.ConcFlags.ctxtSwitchTicks =
            (RtsFlags.ConcFlags.ctxtSwitchTime / RtsFlags.MiscFlags.tickInterval) as i32;
    } else {
        RtsFlags.ConcFlags.ctxtSwitchTicks = 0;
    }

    if RtsFlags.ProfFlags.heapProfileInterval > 0 && RtsFlags.MiscFlags.tickInterval != 0 {
        RtsFlags.ProfFlags.heapProfileIntervalTicks =
            (RtsFlags.ProfFlags.heapProfileInterval / RtsFlags.MiscFlags.tickInterval) as u32;
    } else {
        RtsFlags.ProfFlags.heapProfileIntervalTicks = 0;
    }

    if RtsFlags.TraceFlags.eventlogFlushTime > 0 && RtsFlags.MiscFlags.tickInterval != 0 {
        RtsFlags.TraceFlags.eventlogFlushTicks =
            (RtsFlags.TraceFlags.eventlogFlushTime / RtsFlags.MiscFlags.tickInterval) as i32;
    } else {
        RtsFlags.TraceFlags.eventlogFlushTicks = 0;
    }

    if RtsFlags.GcFlags.stkChunkBufferSize > RtsFlags.GcFlags.stkChunkSize.wrapping_div(2 as u32) {
        errorBelch(
            c"stack chunk buffer size (-kb) must be less than 50%%\nof the stack chunk size (-kc)"
                .as_ptr(),
        );

        errorUsage();
    }

    if RtsFlags.GcFlags.maxHeapSize != 0
        && RtsFlags.GcFlags.heapSizeSuggestion > RtsFlags.GcFlags.maxHeapSize
    {
        errorBelch(
            c"Maximum heap size (-M) is smaller than suggested heap size (-H)\nSetting maximum heap size to suggested heap size ( %llu )"
                .as_ptr(),
            (RtsFlags.GcFlags.maxHeapSize as StgWord64)
                .wrapping_mul(BLOCK_SIZE as StgWord64),
        );

        RtsFlags.GcFlags.maxHeapSize = RtsFlags.GcFlags.heapSizeSuggestion;
    }

    if RtsFlags.GcFlags.maxHeapSize != 0
        && RtsFlags.GcFlags.minAllocAreaSize > RtsFlags.GcFlags.maxHeapSize
    {
        errorBelch(c"maximum heap size (-M) is smaller than minimum alloc area size (-A)".as_ptr());

        RtsFlags.GcFlags.minAllocAreaSize = RtsFlags.GcFlags.maxHeapSize;
    }

    if RtsFlags.GcFlags.minAllocAreaSize as u64
        >= (16_u64 * 1024 * 1024).wrapping_div(BLOCK_SIZE as u64)
    {
        RtsFlags.GcFlags.nurseryChunkSize = (4 * 1024 * 1024) / BLOCK_SIZE;
    }

    if RtsFlags.ParFlags.parGcLoadBalancingGen == !0 {
        let mut alloc_area_bytes: u64 =
            (RtsFlags.GcFlags.minAllocAreaSize as u64).wrapping_mul(BLOCK_SIZE as u64);

        if alloc_area_bytes >= (32_u64 * 1024 * 1024) {
            RtsFlags.ParFlags.parGcLoadBalancingGen = 0;
        } else {
            RtsFlags.ParFlags.parGcLoadBalancingGen = 1;
        }
    }

    if RtsFlags.MiscFlags.generate_dump_file {
        RtsFlags.MiscFlags.install_seh_handlers = true;
    }

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && RtsFlags.GcFlags.generations == 1 {
        barf(c"The non-moving collector doesn't support -G1".as_ptr());
    }

    if RtsFlags.GcFlags.compact as i32 != 0 && RtsFlags.GcFlags.useNonmoving as i32 != 0 {
        errorBelch(
            c"The non-moving collector cannot be used in conjunction with\nthe compacting collector."
                .as_ptr(),
        );

        errorUsage();
    }

    if RtsFlags.TraceFlags.ticky as i32 != 0 && RtsFlags.TickyFlags.showTickyStats as i32 != 0 {
        barf(
            c"The ticky-ticky eventlog output cannot be used in conjunction with\n+RTS -r<file>."
                .as_ptr(),
        );
    }
}

unsafe fn errorUsage() -> ! {
    io::stdout().flush();
    let mut p = &raw mut usage_text as *mut *const c_char;

    while !(*p).is_null() {
        errorBelch(c"%s".as_ptr(), *p);
        p = p.offset(1);
    }

    stg_exit(EXIT_FAILURE);
}

unsafe extern "C" fn stats_fprintf<W: io::Write>(f: Option<&mut W>, s: *const c_char, args: ...) {
    if let Some(f) = f {
        printf::format(s, args, printf::output::io_write(f));
    } else {
        vdebugBelch(s, args);
    };
}

unsafe fn openStatsFile(
    mut filename: *mut c_char,
    mut filename_fmt: *const c_char,
    mut file_ret: *mut *mut FILE,
) -> i32 {
    let mut f = null_mut::<FILE>();

    if strequal(filename, c"stderr".as_ptr()) as i32 != 0
        || filename_fmt.is_null() && *filename as i32 == '\0' as i32
    {
        f = null_mut::<FILE>();
    } else {
        if *filename as i32 != '\0' as i32 {
            f = __rts_fopen(filename, c"w+".as_ptr());
        } else {
            if filename_fmt.is_null() {
                errorBelch(c"Invalid stats filename format (NULL)\n".as_ptr());

                return -1;
            }

            let mut stats_filename: [c_char; 128] = [0; 128];

            snprintf(
                &raw mut stats_filename as *mut c_char,
                STATS_FILENAME_MAXLEN as usize,
                filename_fmt,
                prog_name,
            );

            f = __rts_fopen(&raw mut stats_filename as *mut c_char, c"w+".as_ptr());
        }

        if f.is_null() {
            errorBelch(c"Can't open stats file %s\n".as_ptr(), filename);

            return -1;
        }
    }

    *file_ret = f;

    return 0;
}

unsafe fn stats_fprintf_escape(f: &mut File, mut s: *const c_char) {
    stats_fprintf(Some(f), c"'".as_ptr());

    while *s as i32 != '\0' as i32 {
        match *s as i32 {
            39 => {
                stats_fprintf(Some(f), c"'''".as_ptr());
            }
            _ => {
                stats_fprintf(Some(f), c"%c".as_ptr(), *s as i32);
            }
        }

        s = s.offset(1);
    }

    stats_fprintf(Some(f), c"' ".as_ptr());
}

unsafe fn initStatsFile(f: &mut File) {
    let mut count: i32 = 0;

    while count < prog_argc {
        stats_fprintf_escape(f, *prog_argv.offset(count as isize));
        count += 1;
    }

    stats_fprintf(Some(f), c"+RTS ".as_ptr());
    count = 0;

    while count < rts_argc {
        stats_fprintf_escape(f, *rts_argv.offset(count as isize));
        count += 1;
    }

    stats_fprintf(Some(f), c"\n".as_ptr());
}

unsafe fn decodeSize(
    flag: *const c_char,
    offset: u32,
    min: StgWord64,
    max: StgWord64,
) -> StgWord64 {
    let s = flag.offset(offset as isize);
    let mut m: StgDouble;

    if *s == 0 {
        m = 0.;
    } else {
        let mut end = null_mut::<c_char>();
        m = libc::strtod(s, &raw mut end) as StgDouble;

        if end == s as *mut c_char {
            errorBelch(
                c"error in RTS option %s: unable to parse number '%s'".as_ptr(),
                flag,
                s,
            );

            stg_exit(EXIT_FAILURE);
        }

        let unit: StgWord64;

        match *end as i32 {
            116 | 84 => {
                unit = 1024 * 1024 * 1024 * 1024;
            }
            103 | 71 => {
                unit = 1024 * 1024 * 1024;
            }
            109 | 77 => {
                unit = 1024 * 1024;
            }
            107 | 75 => {
                unit = 1024;
            }
            119 | 87 => {
                unit = size_of::<W_>() as StgWord64;
            }
            98 | 66 | 0 => {
                unit = 1;
            }
            _ => {
                errorBelch(
                    c"error in RTS option %s: unknown unit suffix '%c'".as_ptr(),
                    flag,
                    *end as i32,
                );

                stg_exit(EXIT_FAILURE);
            }
        }

        m *= unit as StgDouble;
    }

    let val = m as StgWord64;

    if m < 0.0 || val < min || val > max {
        errorBelch(
            c"error in RTS option %s: size outside allowed range (%llu - %llu)".as_ptr(),
            flag,
            min,
            max,
        );

        stg_exit(EXIT_FAILURE);
    }

    return val;
}

unsafe fn parseDouble(arg: *const c_char, error: *mut bool) -> f64 {
    let mut endptr = null_mut::<c_char>();

    let out: f64 = libc::strtod(arg, &raw mut endptr);

    set_errno(Errno(0));

    if errno().0 != 0 || endptr == arg as *mut c_char {
        *error = true;

        return out;
    }

    while libc::isspace(*endptr as u8 as i32) != 0 {
        endptr = endptr.offset(1);
    }

    if *endptr as i32 != 0 {
        *error = true;
    }

    return out;
}

unsafe fn read_debug_flags(arg: *const c_char) {
    let mut c = arg.offset(2);

    while *c as i32 != '\0' as i32 {
        match *c as i32 {
            115 => {
                RtsFlags.DebugFlags.scheduler = true;
            }
            105 => {
                RtsFlags.DebugFlags.interpreter = true;
            }
            119 => {
                RtsFlags.DebugFlags.weak = true;
            }
            71 => {
                RtsFlags.DebugFlags.gccafs = true;
            }
            103 => {
                RtsFlags.DebugFlags.gc = true;
            }
            110 => {
                RtsFlags.DebugFlags.nonmoving_gc = true;
            }
            98 => {
                RtsFlags.DebugFlags.block_alloc = true;
            }
            83 => {
                RtsFlags.DebugFlags.sanity = true;
            }
            90 => {
                RtsFlags.DebugFlags.zero_on_gc = true;
            }
            116 => {
                RtsFlags.DebugFlags.stable = true;
            }
            112 => {
                RtsFlags.DebugFlags.prof = true;
            }
            108 => {
                RtsFlags.DebugFlags.linker = true;
            }
            76 => {
                RtsFlags.DebugFlags.linker_verbose = true;
                RtsFlags.DebugFlags.linker = true;
            }
            97 => {
                RtsFlags.DebugFlags.apply = true;
            }
            109 => {
                RtsFlags.DebugFlags.stm = true;
            }
            122 => {
                RtsFlags.DebugFlags.squeeze = true;
            }
            99 => {
                RtsFlags.DebugFlags.hpc = true;
            }
            114 => {
                RtsFlags.DebugFlags.sparks = true;
            }
            67 => {
                RtsFlags.DebugFlags.compact = true;
            }
            107 => {
                RtsFlags.DebugFlags.continuation = true;
            }
            111 => {
                RtsFlags.DebugFlags.iomanager = true;
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
        RtsFlags.DebugFlags.zero_on_gc = true;
    }
}

unsafe fn read_heap_profiling_flag(arg: *const c_char) -> bool {
    let mut error = false;
    let current_block_31: u64;

    match *arg.offset(2) as i32 {
        0 => {
            errorBelch(c"-h is deprecated, use -hc instead.".as_ptr());
            current_block_31 = 6028434607431139856;
        }

        67 | 99 | 77 | 109 | 68 | 100 | 89 | 121 | 105 | 82 | 114 | 66 | 98 | 101 | 84 => {
            current_block_31 = 6028434607431139856;
        }
        _ => {
            errorBelch(c"invalid heap profile option: %s".as_ptr(), arg);
            error = true;
            current_block_31 = 1724319918354933278;
        }
    }

    match current_block_31 {
        6028434607431139856 => {
            if *arg.offset(2) as i32 != '\0' as i32 && *arg.offset(3) as i32 != '\0' as i32 {
                let mut left: *const c_char = libc::strchr(arg, '{' as i32);
                let mut right: *const c_char = libc::strrchr(arg, '}' as i32);

                if !left.is_null() {
                    left = left.offset(1);
                } else {
                    left = arg.offset(3);
                }

                if right.is_null() {
                    right = arg.offset(libc::strlen(arg) as isize);
                }

                let selector = stgStrndup(left, (right.offset_from(left) as i64 + 1) as usize);

                match *arg.offset(2) as i32 {
                    99 => {
                        RtsFlags.ProfFlags.ccSelector = selector;
                    }
                    67 => {
                        RtsFlags.ProfFlags.ccsSelector = selector;
                    }
                    77 | 109 => {
                        RtsFlags.ProfFlags.modSelector = selector;
                    }
                    68 | 100 => {
                        RtsFlags.ProfFlags.descrSelector = selector;
                    }
                    89 | 121 => {
                        RtsFlags.ProfFlags.typeSelector = selector;
                    }
                    82 | 114 => {
                        RtsFlags.ProfFlags.retainerSelector = selector;
                    }
                    66 | 98 => {
                        RtsFlags.ProfFlags.bioSelector = selector;
                    }
                    69 | 101 => {
                        RtsFlags.ProfFlags.eraSelector =
                            libc::strtoul(selector, null_mut::<*mut c_char>(), 10) as StgWord;
                    }
                    _ => {
                        stgFree(selector as *mut c_void);
                    }
                }
            } else if RtsFlags.ProfFlags.doHeapProfile != 0 {
                errorBelch(c"multiple heap profile options".as_ptr());
                error = true;
            } else {
                match *arg.offset(2) as i32 {
                    0 | 67 | 99 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_CCS as u32;
                    }
                    77 | 109 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_MOD as u32;
                    }
                    68 | 100 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_DESCR as u32;
                    }
                    89 | 121 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_TYPE as u32;
                    }
                    105 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_INFO_TABLE as u32;
                    }
                    82 | 114 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_RETAINER as u32;
                    }
                    66 | 98 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_LDV as u32;
                    }
                    84 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_CLOSURE_TYPE as u32;
                    }
                    101 => {
                        RtsFlags.ProfFlags.doHeapProfile = HEAP_BY_ERA as u32;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    return error;
}

unsafe fn read_trace_flags(arg: *const c_char) {
    let mut c;
    let mut enabled = true;
    RtsFlags.TraceFlags.scheduler = true;
    RtsFlags.TraceFlags.gc = true;
    RtsFlags.TraceFlags.sparks_sampled = true;
    RtsFlags.TraceFlags.user = true;
    c = arg;

    while *c as i32 != '\0' as i32 {
        match *c as i32 {
            0 => {}
            45 => {
                enabled = false;
            }
            97 => {
                RtsFlags.TraceFlags.scheduler = enabled;
                RtsFlags.TraceFlags.gc = enabled;
                RtsFlags.TraceFlags.sparks_sampled = enabled;
                RtsFlags.TraceFlags.sparks_full = enabled;
                RtsFlags.TraceFlags.user = enabled;
                RtsFlags.TraceFlags.nonmoving_gc = enabled;
                RtsFlags.TraceFlags.ticky = enabled;
                enabled = true;
            }
            115 => {
                RtsFlags.TraceFlags.scheduler = enabled;
                enabled = true;
            }
            112 => {
                RtsFlags.TraceFlags.sparks_sampled = enabled;
                enabled = true;
            }
            102 => {
                RtsFlags.TraceFlags.sparks_full = enabled;
                enabled = true;
            }
            116 => {
                RtsFlags.TraceFlags.timestamp = enabled;
                enabled = true;
            }
            103 => {
                RtsFlags.TraceFlags.gc = enabled;
                enabled = true;
            }
            110 => {
                RtsFlags.TraceFlags.nonmoving_gc = enabled;
                enabled = true;
            }
            117 => {
                RtsFlags.TraceFlags.user = enabled;
                enabled = true;
            }
            84 => {
                RtsFlags.TraceFlags.ticky = enabled;
                enabled = true;
            }
            _ => {
                errorBelch(c"unknown trace option: %c".as_ptr(), *c as i32);
            }
        }

        c = c.offset(1);
    }
}

unsafe fn bad_option(s: *const c_char) -> ! {
    errorBelch(c"bad RTS option: %s".as_ptr(), s);
    stg_exit(EXIT_FAILURE);
}

unsafe fn copyArg(arg: *const c_char) -> *mut c_char {
    let new_arg = stgMallocBytes(
        libc::strlen(arg).wrapping_add(1 as usize),
        c"copyArg".as_ptr(),
    ) as *mut c_char;

    libc::strcpy(new_arg, arg);

    return new_arg;
}

unsafe fn copyArgv(argc: i32, argv: *const *const c_char) -> *const *const c_char {
    let argc = argc as isize;
    let mut i: isize = 0;
    let new_argv = stgCallocBytes(
        (argc + 1) as usize,
        size_of::<*mut c_char>() as usize,
        c"copyArgv 1".as_ptr(),
    ) as *const *const c_char;

    while i < argc {
        let fresh0 = *new_argv.offset(i);
        *fresh0 = copyArg(*argv.offset(i));
        i += 1;
    }

    let ref mut fresh1 = *new_argv.offset(argc as isize);
    *fresh1 = null_mut::<c_char>();

    return new_argv;
}

unsafe fn freeArgv(argc: i32, argv: *const *const c_char) {
    let mut i: i32;

    if !argv.is_null() {
        i = 0;

        while i < argc {
            stgFree(*argv.offset(i as isize) as *mut c_void);
            i += 1;
        }

        stgFree(argv as *mut c_void);
    }
}

unsafe fn setProgName(argv: *const *const c_char) {
    if (*argv.offset(0)).is_null() {
        prog_name = DEFAULT_PROG_NAME;
        return;
    }

    let last_slash = libc::strrchr(*argv.offset(0), '/' as i32);

    let name = if !last_slash.is_null() {
        last_slash.offset(1);
    } else {
        *argv.offset(0);
    };

    prog_name = todo!();
}

#[ffi(ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProgArgv(argc: *mut c_int, argv: *mut *const *const c_char) {
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
pub unsafe extern "C" fn setProgArgv(argc: c_int, argv: *const *const c_char) {
    freeArgv(prog_argc, prog_argv);
    prog_argc = argc;
    prog_argv = copyArgv(argc, argv);
    setProgName(prog_argv);
}

unsafe fn freeProgArgv() {
    freeArgv(prog_argc, prog_argv);
    prog_argc = 0;
    prog_argv = null();
}

unsafe fn setFullProgArgv(argc: i32, argv: *const *const c_char) {
    full_prog_argc = argc;
    full_prog_argv = copyArgv(argc, argv);
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getFullProgArgv(argc: *mut c_int, argv: *mut *const *const c_char) {
    if !argc.is_null() {
        *argc = full_prog_argc;
    }

    if !argv.is_null() {
        *argv = full_prog_argv;
    }
}

unsafe fn freeFullProgArgv() {
    freeArgv(full_prog_argc, full_prog_argv);
    full_prog_argc = 0;
    full_prog_argv = null();
}

unsafe fn freeRtsArgv() {
    freeArgv(rts_argc, rts_argv);
    rts_argc = 0;
    rts_argv = null();
    rts_argv_size = 0;
}

pub(in crate::rts_startup) unsafe fn freeRtsArgs() {
    freeFullProgArgv();
    freeProgArgv();
    freeRtsArgv();
}

unsafe fn doingLDVProfiling() -> bool {
    return RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_LDV as u32
        || !RtsFlags.ProfFlags.bioSelector.is_null();
}

unsafe fn doingRetainerProfiling() -> bool {
    return RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_RETAINER as u32
        || !RtsFlags.ProfFlags.retainerSelector.is_null();
}

unsafe fn doingErasProfiling() -> bool {
    return RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_ERA as u32
        || RtsFlags.ProfFlags.eraSelector != 0;
}
