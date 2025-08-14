use crate::prelude::*;
use crate::rts::time::Time;
use crate::stg::types::{StgWord, StgWord64};
use libc::FILE;

#[cfg(test)]
mod tests;

pub const NO_GC_STATS: u32 = 0;

pub const COLLECT_GC_STATS: u32 = 1;

pub const ONELINE_GC_STATS: u32 = 2;

pub const SUMMARY_GC_STATS: u32 = 3;

pub const VERBOSE_GC_STATS: u32 = 4;

pub const COST_CENTRES_NONE: u32 = 0;

pub const COST_CENTRES_SUMMARY: u32 = 1;

pub const COST_CENTRES_VERBOSE: u32 = 2;

pub const COST_CENTRES_ALL: u32 = 3;

pub const COST_CENTRES_JSON: u32 = 4;

pub const NO_HEAP_PROFILING: u32 = 0;

pub const HEAP_BY_CCS: u32 = 1;

pub const HEAP_BY_MOD: u32 = 2;

pub const HEAP_BY_DESCR: u32 = 4;

pub const HEAP_BY_TYPE: u32 = 5;

pub const HEAP_BY_RETAINER: u32 = 6;

pub const HEAP_BY_LDV: u32 = 7;

pub const HEAP_BY_CLOSURE_TYPE: u32 = 8;

pub const HEAP_BY_INFO_TABLE: u32 = 9;

pub const HEAP_BY_ERA: u32 = 10;

pub const TRACE_NONE: u32 = 0;

pub const TRACE_EVENTLOG: u32 = 1;

pub const TRACE_STDERR: u32 = 2;

pub(crate) const DEFAULT_LINKER_ALWAYS_PIC: u32 = 1;

pub(crate) const STATS_FILENAME_MAXLEN: u32 = 128;

pub type GC_FLAGS = _GC_FLAGS;

#[repr(C)]
///cbindgen:no-export
pub struct _GC_FLAGS {
    statsFile: *mut FILE,
    giveStats: u32,
    maxStkSize: u32,
    initialStkSize: u32,
    stkChunkSize: u32,
    stkChunkBufferSize: u32,
    maxHeapSize: u32,
    minAllocAreaSize: u32,
    largeAllocLim: u32,
    nurseryChunkSize: u32,
    minOldGenSize: u32,
    heapSizeSuggestion: u32,
    heapSizeSuggestionAuto: bool,
    oldGenFactor: f64,
    returnDecayFactor: f64,
    pcFreeHeap: f64,
    useNonmoving: bool,
    nonmovingDenseAllocatorCount: u16,
    generations: u32,
    squeezeUpdFrames: bool,
    compact: bool,
    compactThreshold: f64,
    sweep: bool,
    ringBell: bool,
    idleGCDelayTime: Time,
    interIdleGCWait: Time,
    doIdleGC: bool,
    longGCSync: Time,
    heapBase: StgWord,
    allocLimitGrace: StgWord,
    heapLimitGrace: StgWord,
    numa: bool,
    numaMask: StgWord,
    addressSpaceSize: StgWord64,
}

#[cfg(feature = "sys")]
impl From<_GC_FLAGS> for sys::_GC_FLAGS {
    fn from(x: _GC_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

pub type DEBUG_FLAGS = _DEBUG_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _DEBUG_FLAGS {
    scheduler: bool,
    interpreter: bool,
    weak: bool,
    gccafs: bool,
    gc: bool,
    nonmoving_gc: bool,
    block_alloc: bool,
    sanity: bool,
    zero_on_gc: bool,
    stable: bool,
    prof: bool,
    linker: bool,
    linker_verbose: bool,
    apply: bool,
    stm: bool,
    squeeze: bool,
    hpc: bool,
    sparks: bool,
    numa: bool,
    compact: bool,
    continuation: bool,
    iomanager: bool,
}

#[cfg(feature = "sys")]
impl From<_DEBUG_FLAGS> for sys::_DEBUG_FLAGS {
    fn from(x: _DEBUG_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
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

pub type COST_CENTRE_FLAGS = _COST_CENTRE_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _COST_CENTRE_FLAGS {
    doCostCentres: u32,
    profilerTicks: c_int,
    msecsPerTick: c_int,
    outputFileNameStem: *const c_char,
}

#[cfg(feature = "sys")]
impl From<_COST_CENTRE_FLAGS> for sys::_COST_CENTRE_FLAGS {
    fn from(x: _COST_CENTRE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

pub type PROFILING_FLAGS = _PROFILING_FLAGS;

#[repr(C)]
///cbindgen:no-export
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

#[cfg(feature = "sys")]
impl From<_PROFILING_FLAGS> for sys::_PROFILING_FLAGS {
    fn from(x: _PROFILING_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

pub type TRACE_FLAGS = _TRACE_FLAGS;

#[repr(C)]
///cbindgen:no-export
pub struct _TRACE_FLAGS {
    tracing: c_int,
    timestamp: bool,
    scheduler: bool,
    gc: bool,
    nonmoving_gc: bool,
    sparks_sampled: bool,
    sparks_full: bool,
    ticky: bool,
    user: bool,
    eventlogFlushTime: Time,
    eventlogFlushTicks: c_int,
    trace_output: *mut c_char,
    nullWriter: bool,
}

#[cfg(feature = "sys")]
impl From<_TRACE_FLAGS> for sys::_TRACE_FLAGS {
    fn from(x: _TRACE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

pub type CONCURRENT_FLAGS = _CONCURRENT_FLAGS;

#[repr(C)]
///cbindgen:no-export
#[derive(Clone)]
pub struct _CONCURRENT_FLAGS {
    ctxtSwitchTime: Time,
    ctxtSwitchTicks: c_int,
}

#[cfg(feature = "sys")]
impl From<_CONCURRENT_FLAGS> for sys::_CONCURRENT_FLAGS {
    fn from(x: _CONCURRENT_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
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

pub(crate) type IO_MANAGER_FLAG = _IO_MANAGER_FLAG;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum _IO_MANAGER_FLAG {
    IO_MNGR_FLAG_AUTO = 0,
    IO_MNGR_FLAG_SELECT = 1,
    IO_MNGR_FLAG_MIO = 2,
    IO_MNGR_FLAG_WINIO = 3,
    IO_MNGR_FLAG_WIN32_LEGACY = 4,
}

#[cfg(test)]
impl Arbitrary for _IO_MANAGER_FLAG {
    fn arbitrary(g: &mut Gen) -> Self {
        use _IO_MANAGER_FLAG::*;

        match u32::arbitrary(g) % 5 {
            0 => IO_MNGR_FLAG_AUTO,
            1 => IO_MNGR_FLAG_SELECT,
            2 => IO_MNGR_FLAG_MIO,
            3 => IO_MNGR_FLAG_WINIO,
            4.. => IO_MNGR_FLAG_WIN32_LEGACY,
        }
    }
}

pub type MISC_FLAGS = _MISC_FLAGS;

#[repr(C)]
///cbindgen:no-export
#[derive(Clone)]
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
    linkerMemBase: StgWord,
    ioManager: IO_MANAGER_FLAG,
    numIoWorkerThreads: u32,
}

#[cfg(feature = "sys")]
impl From<_MISC_FLAGS> for sys::_MISC_FLAGS {
    fn from(x: _MISC_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
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
            linkerMemBase: Arbitrary::arbitrary(g),
            ioManager: Arbitrary::arbitrary(g),
            numIoWorkerThreads: Arbitrary::arbitrary(g),
        }
    }
}

pub type PAR_FLAGS = _PAR_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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

#[cfg(feature = "sys")]
impl From<_PAR_FLAGS> for sys::_PAR_FLAGS {
    fn from(x: _PAR_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
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

pub(crate) type HPC_READ_FILE = _HPC_READ_FILE;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum _HPC_READ_FILE {
    HPC_NO_EXPLICIT = 0,
    HPC_YES_IMPLICIT = 1,
    HPC_YES_EXPLICIT = 2,
}

#[cfg(test)]
impl Arbitrary for _HPC_READ_FILE {
    fn arbitrary(g: &mut Gen) -> Self {
        use _HPC_READ_FILE::*;

        match u32::arbitrary(g) % 3 {
            0 => HPC_NO_EXPLICIT,
            1 => HPC_YES_IMPLICIT,
            2.. => HPC_YES_EXPLICIT,
        }
    }
}

pub type HPC_FLAGS = _HPC_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _HPC_FLAGS {
    writeTixFile: bool,
    readTixFile: HPC_READ_FILE,
}

#[cfg(feature = "sys")]
impl From<_HPC_FLAGS> for sys::_HPC_FLAGS {
    fn from(x: _HPC_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
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

pub type TICKY_FLAGS = _TICKY_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _TICKY_FLAGS {
    showTickyStats: bool,
    tickyFile: *mut FILE,
}

#[cfg(feature = "sys")]
impl From<_TICKY_FLAGS> for sys::_TICKY_FLAGS {
    fn from(x: _TICKY_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

pub type RTS_FLAGS = _RTS_FLAGS;

#[repr(C)]
///cbindgen:no-export
pub struct _RTS_FLAGS {
    GcFlags: GC_FLAGS,
    ConcFlags: CONCURRENT_FLAGS,
    MiscFlags: MISC_FLAGS,
    DebugFlags: DEBUG_FLAGS,
    CcFlags: COST_CENTRE_FLAGS,
    ProfFlags: PROFILING_FLAGS,
    TraceFlags: TRACE_FLAGS,
    TickyFlags: TICKY_FLAGS,
    ParFlags: PAR_FLAGS,
    HpcFlags: HPC_FLAGS,
}

#[cfg(feature = "sys")]
impl From<_RTS_FLAGS> for sys::_RTS_FLAGS {
    fn from(x: _RTS_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

// TODO: See [RtsFlags is a pointer in STG code]
// #[cfg_attr(feature = "sys", unsafe(export_name = "rust_RtsFlags"))]
// #[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
// pub static mut RtsFlags: RTS_FLAGS = todo!();

static mut rts_argc: c_int = 0;

static mut rts_argv: *mut *mut c_char = null_mut();
