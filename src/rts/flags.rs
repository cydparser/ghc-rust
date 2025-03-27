use crate::stg::types;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
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

pub(crate) const DEFAULT_LINKER_ALWAYS_PIC: u32 = 0;

pub(crate) const STATS_FILENAME_MAXLEN: u32 = 128;

pub(crate) const GR_FILENAME_FMT: &[u8; 11] = b"%0.124s.gr\0";

pub(crate) const HP_FILENAME_FMT: &[u8; 11] = b"%0.124s.hp\0";

pub(crate) const LIFE_FILENAME_FMT: &[u8; 13] = b"%0.122s.life\0";

pub(crate) const PROF_FILENAME_FMT: &[u8; 13] = b"%0.122s.prof\0";

pub(crate) const PROF_FILENAME_FMT_GUM: &[u8; 18] = b"%0.118s.%03d.prof\0";

pub(crate) const QP_FILENAME_FMT: &[u8; 11] = b"%0.124s.qp\0";

pub(crate) const STAT_FILENAME_FMT: &[u8; 13] = b"%0.122s.stat\0";

pub(crate) const TICKY_FILENAME_FMT: &[u8; 14] = b"%0.121s.ticky\0";

pub(crate) const TIME_FILENAME_FMT: &[u8; 13] = b"%0.122s.time\0";

pub(crate) const TIME_FILENAME_FMT_GUM: &[u8; 18] = b"%0.118s.%03d.time\0";

#[repr(C)]
pub(crate) struct _GC_FLAGS {
    pub statsFile: *mut FILE,
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

#[cfg(feature = "sys")]
impl From<_GC_FLAGS> for sys::_GC_FLAGS {
    fn from(x: _GC_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _GC_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _GC_FLAGS {
            statsFile: Arbitrary::arbitrary(g),
            giveStats: Arbitrary::arbitrary(g),
            maxStkSize: Arbitrary::arbitrary(g),
            initialStkSize: Arbitrary::arbitrary(g),
            stkChunkSize: Arbitrary::arbitrary(g),
            stkChunkBufferSize: Arbitrary::arbitrary(g),
            maxHeapSize: Arbitrary::arbitrary(g),
            minAllocAreaSize: Arbitrary::arbitrary(g),
            largeAllocLim: Arbitrary::arbitrary(g),
            nurseryChunkSize: Arbitrary::arbitrary(g),
            minOldGenSize: Arbitrary::arbitrary(g),
            heapSizeSuggestion: Arbitrary::arbitrary(g),
            heapSizeSuggestionAuto: Arbitrary::arbitrary(g),
            oldGenFactor: Arbitrary::arbitrary(g),
            returnDecayFactor: Arbitrary::arbitrary(g),
            pcFreeHeap: Arbitrary::arbitrary(g),
            useNonmoving: Arbitrary::arbitrary(g),
            nonmovingDenseAllocatorCount: Arbitrary::arbitrary(g),
            generations: Arbitrary::arbitrary(g),
            squeezeUpdFrames: Arbitrary::arbitrary(g),
            compact: Arbitrary::arbitrary(g),
            compactThreshold: Arbitrary::arbitrary(g),
            sweep: Arbitrary::arbitrary(g),
            ringBell: Arbitrary::arbitrary(g),
            idleGCDelayTime: Arbitrary::arbitrary(g),
            interIdleGCWait: Arbitrary::arbitrary(g),
            doIdleGC: Arbitrary::arbitrary(g),
            longGCSync: Arbitrary::arbitrary(g),
            heapBase: Arbitrary::arbitrary(g),
            allocLimitGrace: Arbitrary::arbitrary(g),
            heapLimitGrace: Arbitrary::arbitrary(g),
            numa: Arbitrary::arbitrary(g),
            numaMask: Arbitrary::arbitrary(g),
            addressSpaceSize: Arbitrary::arbitrary(g),
        }
    }
}

pub type GC_FLAGS = _GC_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _DEBUG_FLAGS {
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

pub type DEBUG_FLAGS = _DEBUG_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _COST_CENTRE_FLAGS {
    pub doCostCentres: u32,
    pub profilerTicks: ::core::ffi::c_int,
    pub msecsPerTick: ::core::ffi::c_int,
    pub outputFileNameStem: *const ::core::ffi::c_char,
}

#[cfg(feature = "sys")]
impl From<_COST_CENTRE_FLAGS> for sys::_COST_CENTRE_FLAGS {
    fn from(x: _COST_CENTRE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _COST_CENTRE_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _COST_CENTRE_FLAGS {
            doCostCentres: Arbitrary::arbitrary(g),
            profilerTicks: Arbitrary::arbitrary(g),
            msecsPerTick: Arbitrary::arbitrary(g),
            outputFileNameStem: Arbitrary::arbitrary(g),
        }
    }
}

pub type COST_CENTRE_FLAGS = _COST_CENTRE_FLAGS;

#[repr(C)]
pub(crate) struct _PROFILING_FLAGS {
    pub doHeapProfile: u32,
    pub heapProfileInterval: Time,
    pub heapProfileIntervalTicks: u32,
    pub startHeapProfileAtStartup: bool,
    pub startTimeProfileAtStartup: bool,
    pub incrementUserEra: bool,
    pub showCCSOnException: bool,
    pub maxRetainerSetSize: u32,
    pub ccsLength: u32,
    pub modSelector: *const ::core::ffi::c_char,
    pub descrSelector: *const ::core::ffi::c_char,
    pub typeSelector: *const ::core::ffi::c_char,
    pub ccSelector: *const ::core::ffi::c_char,
    pub ccsSelector: *const ::core::ffi::c_char,
    pub retainerSelector: *const ::core::ffi::c_char,
    pub eraSelector: StgWord,
    pub bioSelector: *const ::core::ffi::c_char,
}

#[cfg(feature = "sys")]
impl From<_PROFILING_FLAGS> for sys::_PROFILING_FLAGS {
    fn from(x: _PROFILING_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _PROFILING_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _PROFILING_FLAGS {
            doHeapProfile: Arbitrary::arbitrary(g),
            heapProfileInterval: Arbitrary::arbitrary(g),
            heapProfileIntervalTicks: Arbitrary::arbitrary(g),
            startHeapProfileAtStartup: Arbitrary::arbitrary(g),
            startTimeProfileAtStartup: Arbitrary::arbitrary(g),
            incrementUserEra: Arbitrary::arbitrary(g),
            showCCSOnException: Arbitrary::arbitrary(g),
            maxRetainerSetSize: Arbitrary::arbitrary(g),
            ccsLength: Arbitrary::arbitrary(g),
            modSelector: Arbitrary::arbitrary(g),
            descrSelector: Arbitrary::arbitrary(g),
            typeSelector: Arbitrary::arbitrary(g),
            ccSelector: Arbitrary::arbitrary(g),
            ccsSelector: Arbitrary::arbitrary(g),
            retainerSelector: Arbitrary::arbitrary(g),
            eraSelector: Arbitrary::arbitrary(g),
            bioSelector: Arbitrary::arbitrary(g),
        }
    }
}

pub type PROFILING_FLAGS = _PROFILING_FLAGS;

#[repr(C)]
pub(crate) struct _TRACE_FLAGS {
    pub tracing: ::core::ffi::c_int,
    pub timestamp: bool,
    pub scheduler: bool,
    pub gc: bool,
    pub nonmoving_gc: bool,
    pub sparks_sampled: bool,
    pub sparks_full: bool,
    pub ticky: bool,
    pub user: bool,
    pub eventlogFlushTime: Time,
    pub eventlogFlushTicks: ::core::ffi::c_int,
    pub trace_output: *mut ::core::ffi::c_char,
    pub nullWriter: bool,
}

#[cfg(feature = "sys")]
impl From<_TRACE_FLAGS> for sys::_TRACE_FLAGS {
    fn from(x: _TRACE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _TRACE_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _TRACE_FLAGS {
            tracing: Arbitrary::arbitrary(g),
            timestamp: Arbitrary::arbitrary(g),
            scheduler: Arbitrary::arbitrary(g),
            gc: Arbitrary::arbitrary(g),
            nonmoving_gc: Arbitrary::arbitrary(g),
            sparks_sampled: Arbitrary::arbitrary(g),
            sparks_full: Arbitrary::arbitrary(g),
            ticky: Arbitrary::arbitrary(g),
            user: Arbitrary::arbitrary(g),
            eventlogFlushTime: Arbitrary::arbitrary(g),
            eventlogFlushTicks: Arbitrary::arbitrary(g),
            trace_output: Arbitrary::arbitrary(g),
            nullWriter: Arbitrary::arbitrary(g),
        }
    }
}

pub type TRACE_FLAGS = _TRACE_FLAGS;

#[repr(C)]
pub(crate) struct _CONCURRENT_FLAGS {
    pub ctxtSwitchTime: Time,
    pub ctxtSwitchTicks: ::core::ffi::c_int,
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

pub type CONCURRENT_FLAGS = _CONCURRENT_FLAGS;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum _IO_MANAGER_FLAG {
    IO_MNGR_FLAG_AUTO = 0,
    IO_MNGR_FLAG_SELECT = 1,
    IO_MNGR_FLAG_MIO = 2,
    IO_MNGR_FLAG_WINIO = 3,
    IO_MNGR_FLAG_WIN32_LEGACY = 4,
}

pub use self::_IO_MANAGER_FLAG as IO_MANAGER_FLAG;

#[repr(C)]
pub(crate) struct _MISC_FLAGS {
    pub tickInterval: Time,
    pub install_signal_handlers: bool,
    pub install_seh_handlers: bool,
    pub generate_dump_file: bool,
    pub generate_stack_trace: bool,
    pub machineReadable: bool,
    pub disableDelayedOsMemoryReturn: bool,
    pub internalCounters: bool,
    pub linkerAlwaysPic: bool,
    pub linkerMemBase: StgWord,
    pub ioManager: IO_MANAGER_FLAG,
    pub numIoWorkerThreads: u32,
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

pub type MISC_FLAGS = _MISC_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _PAR_FLAGS {
    pub nCapabilities: u32,
    pub migrate: bool,
    pub maxLocalSparks: u32,
    pub parGcEnabled: bool,
    pub parGcGen: u32,
    pub parGcLoadBalancingEnabled: bool,
    pub parGcLoadBalancingGen: u32,
    pub parGcNoSyncWithIdle: u32,
    pub parGcThreads: u32,
    pub setAffinity: bool,
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

pub type PAR_FLAGS = _PAR_FLAGS;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum _HPC_READ_FILE {
    HPC_NO_EXPLICIT = 0,
    HPC_YES_IMPLICIT = 1,
    HPC_YES_EXPLICIT = 2,
}

pub use self::_HPC_READ_FILE as HPC_READ_FILE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _HPC_FLAGS {
    pub writeTixFile: bool,
    pub readTixFile: HPC_READ_FILE,
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

pub type HPC_FLAGS = _HPC_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _TICKY_FLAGS {
    pub showTickyStats: bool,
    pub tickyFile: *mut FILE,
}

#[cfg(feature = "sys")]
impl From<_TICKY_FLAGS> for sys::_TICKY_FLAGS {
    fn from(x: _TICKY_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _TICKY_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _TICKY_FLAGS {
            showTickyStats: Arbitrary::arbitrary(g),
            tickyFile: Arbitrary::arbitrary(g),
        }
    }
}

pub type TICKY_FLAGS = _TICKY_FLAGS;

#[repr(C)]
pub(crate) struct _RTS_FLAGS {
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

#[cfg(feature = "sys")]
impl From<_RTS_FLAGS> for sys::_RTS_FLAGS {
    fn from(x: _RTS_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _RTS_FLAGS {
    fn arbitrary(g: &mut Gen) -> Self {
        _RTS_FLAGS {
            GcFlags: Arbitrary::arbitrary(g),
            ConcFlags: Arbitrary::arbitrary(g),
            MiscFlags: Arbitrary::arbitrary(g),
            DebugFlags: Arbitrary::arbitrary(g),
            CcFlags: Arbitrary::arbitrary(g),
            ProfFlags: Arbitrary::arbitrary(g),
            TraceFlags: Arbitrary::arbitrary(g),
            TickyFlags: Arbitrary::arbitrary(g),
            ParFlags: Arbitrary::arbitrary(g),
            HpcFlags: Arbitrary::arbitrary(g),
        }
    }
}

pub type RTS_FLAGS = _RTS_FLAGS;

#[unsafe(no_mangle)]
pub static mut RtsFlags: RTS_FLAGS = sys::RtsFlags;

static mut rts_argc: ::core::ffi::c_int = sys::rts_argc;

static mut rts_argv: *mut *mut ::core::ffi::c_char = sys::rts_argv;
