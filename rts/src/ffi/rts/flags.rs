use crate::ffi::rts::time::Time;
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;
use libc::FILE;

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
pub const TRACE_NONE: u32 = 0;

#[ffi(ghc_lib)]
pub const TRACE_EVENTLOG: u32 = 1;

#[ffi(ghc_lib)]
pub const TRACE_STDERR: u32 = 2;

pub(crate) const DEFAULT_LINKER_ALWAYS_PIC: u32 = 1;

pub(crate) const STATS_FILENAME_MAXLEN: u32 = 128;

/// cbindgen:no-export
#[repr(C)]
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

#[ffi(ghc_lib)]
pub type GC_FLAGS = _GC_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
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

#[ffi(ghc_lib)]
pub type DEBUG_FLAGS = _DEBUG_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _COST_CENTRE_FLAGS {
    doCostCentres: u32,
    profilerTicks: c_int,
    msecsPerTick: c_int,
    outputFileNameStem: *const c_char,
}

#[ffi(ghc_lib)]
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

#[ffi(ghc_lib)]
pub type PROFILING_FLAGS = _PROFILING_FLAGS;

/// cbindgen:no-export
#[repr(C)]
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

#[ffi(ghc_lib)]
pub type TRACE_FLAGS = _TRACE_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct _CONCURRENT_FLAGS {
    ctxtSwitchTime: Time,
    ctxtSwitchTicks: c_int,
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

#[ffi(ghc_lib)]
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

#[ffi(ghc_lib)]
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

#[ffi(ghc_lib)]
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

#[ffi(ghc_lib)]
pub type HPC_FLAGS = _HPC_FLAGS;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _TICKY_FLAGS {
    showTickyStats: bool,
    tickyFile: *mut FILE,
}

#[ffi(ghc_lib)]
pub type TICKY_FLAGS = _TICKY_FLAGS;

/// cbindgen:no-export
#[repr(C)]
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

#[ffi(ghc_lib)]
pub type RTS_FLAGS = _RTS_FLAGS;

// #[ffi(compiler, ghc_lib)]
// #[unsafe(no_mangle)]
// TODO(rust): pub static mut RtsFlags: RTS_FLAGS = 0;
