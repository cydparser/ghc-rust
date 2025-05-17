use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
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
///cbindgen:no-export
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
#[derive(Clone)]
struct _GC_FLAGSOwned {
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

#[cfg(test)]
impl Arbitrary for _GC_FLAGSOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _GC_FLAGSOwned {
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

#[cfg(test)]
#[derive(Clone)]
struct _GC_FLAGSPointees {
    pub statsFile: FILE,
}

#[cfg(test)]
impl Arbitrary for _GC_FLAGSPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _GC_FLAGSPointees {
            statsFile: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _GC_FLAGS {
    type Owned = _GC_FLAGSOwned;
    type Pointees = _GC_FLAGSPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            giveStats: owned.giveStats,
            maxStkSize: owned.maxStkSize,
            initialStkSize: owned.initialStkSize,
            stkChunkSize: owned.stkChunkSize,
            stkChunkBufferSize: owned.stkChunkBufferSize,
            maxHeapSize: owned.maxHeapSize,
            minAllocAreaSize: owned.minAllocAreaSize,
            largeAllocLim: owned.largeAllocLim,
            nurseryChunkSize: owned.nurseryChunkSize,
            minOldGenSize: owned.minOldGenSize,
            heapSizeSuggestion: owned.heapSizeSuggestion,
            heapSizeSuggestionAuto: owned.heapSizeSuggestionAuto.clone(),
            oldGenFactor: owned.oldGenFactor.clone(),
            returnDecayFactor: owned.returnDecayFactor.clone(),
            pcFreeHeap: owned.pcFreeHeap.clone(),
            useNonmoving: owned.useNonmoving.clone(),
            nonmovingDenseAllocatorCount: owned.nonmovingDenseAllocatorCount.clone(),
            generations: owned.generations,
            squeezeUpdFrames: owned.squeezeUpdFrames.clone(),
            compact: owned.compact.clone(),
            compactThreshold: owned.compactThreshold.clone(),
            sweep: owned.sweep.clone(),
            ringBell: owned.ringBell.clone(),
            idleGCDelayTime: owned.idleGCDelayTime,
            interIdleGCWait: owned.interIdleGCWait,
            doIdleGC: owned.doIdleGC.clone(),
            longGCSync: owned.longGCSync,
            heapBase: owned.heapBase,
            allocLimitGrace: owned.allocLimitGrace,
            heapLimitGrace: owned.heapLimitGrace,
            numa: owned.numa.clone(),
            numaMask: owned.numaMask,
            addressSpaceSize: owned.addressSpaceSize,
            statsFile: unsafe { &raw mut (*pointees).statsFile },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            giveStats: self.giveStats,
            maxStkSize: self.maxStkSize,
            initialStkSize: self.initialStkSize,
            stkChunkSize: self.stkChunkSize,
            stkChunkBufferSize: self.stkChunkBufferSize,
            maxHeapSize: self.maxHeapSize,
            minAllocAreaSize: self.minAllocAreaSize,
            largeAllocLim: self.largeAllocLim,
            nurseryChunkSize: self.nurseryChunkSize,
            minOldGenSize: self.minOldGenSize,
            heapSizeSuggestion: self.heapSizeSuggestion,
            heapSizeSuggestionAuto: self.heapSizeSuggestionAuto.clone(),
            oldGenFactor: self.oldGenFactor.clone(),
            returnDecayFactor: self.returnDecayFactor.clone(),
            pcFreeHeap: self.pcFreeHeap.clone(),
            useNonmoving: self.useNonmoving.clone(),
            nonmovingDenseAllocatorCount: self.nonmovingDenseAllocatorCount.clone(),
            generations: self.generations,
            squeezeUpdFrames: self.squeezeUpdFrames.clone(),
            compact: self.compact.clone(),
            compactThreshold: self.compactThreshold.clone(),
            sweep: self.sweep.clone(),
            ringBell: self.ringBell.clone(),
            idleGCDelayTime: self.idleGCDelayTime,
            interIdleGCWait: self.interIdleGCWait,
            doIdleGC: self.doIdleGC.clone(),
            longGCSync: self.longGCSync,
            heapBase: self.heapBase,
            allocLimitGrace: self.allocLimitGrace,
            heapLimitGrace: self.heapLimitGrace,
            numa: self.numa.clone(),
            numaMask: self.numaMask,
            addressSpaceSize: self.addressSpaceSize,
        }
    }
}

pub type GC_FLAGS = _GC_FLAGS;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
///cbindgen:no-export
pub(crate) struct _COST_CENTRE_FLAGS {
    pub doCostCentres: u32,
    pub profilerTicks: c_int,
    pub msecsPerTick: c_int,
    pub outputFileNameStem: *const c_char,
}

#[cfg(feature = "sys")]
impl From<_COST_CENTRE_FLAGS> for sys::_COST_CENTRE_FLAGS {
    fn from(x: _COST_CENTRE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _COST_CENTRE_FLAGSOwned {
    pub doCostCentres: u32,
    pub profilerTicks: c_int,
    pub msecsPerTick: c_int,
}

#[cfg(test)]
impl Arbitrary for _COST_CENTRE_FLAGSOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _COST_CENTRE_FLAGSOwned {
            doCostCentres: Arbitrary::arbitrary(g),
            profilerTicks: Arbitrary::arbitrary(g),
            msecsPerTick: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _COST_CENTRE_FLAGSPointees {
    pub outputFileNameStem: c_char,
}

#[cfg(test)]
impl Arbitrary for _COST_CENTRE_FLAGSPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _COST_CENTRE_FLAGSPointees {
            outputFileNameStem: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _COST_CENTRE_FLAGS {
    type Owned = _COST_CENTRE_FLAGSOwned;
    type Pointees = _COST_CENTRE_FLAGSPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            doCostCentres: owned.doCostCentres,
            profilerTicks: owned.profilerTicks,
            msecsPerTick: owned.msecsPerTick,
            outputFileNameStem: unsafe { &raw mut (*pointees).outputFileNameStem },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            doCostCentres: self.doCostCentres,
            profilerTicks: self.profilerTicks,
            msecsPerTick: self.msecsPerTick,
        }
    }
}

pub type COST_CENTRE_FLAGS = _COST_CENTRE_FLAGS;

#[repr(C)]
///cbindgen:no-export
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
    pub modSelector: *const c_char,
    pub descrSelector: *const c_char,
    pub typeSelector: *const c_char,
    pub ccSelector: *const c_char,
    pub ccsSelector: *const c_char,
    pub retainerSelector: *const c_char,
    pub eraSelector: StgWord,
    pub bioSelector: *const c_char,
}

#[cfg(feature = "sys")]
impl From<_PROFILING_FLAGS> for sys::_PROFILING_FLAGS {
    fn from(x: _PROFILING_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _PROFILING_FLAGSOwned {
    pub doHeapProfile: u32,
    pub heapProfileInterval: Time,
    pub heapProfileIntervalTicks: u32,
    pub startHeapProfileAtStartup: bool,
    pub startTimeProfileAtStartup: bool,
    pub incrementUserEra: bool,
    pub showCCSOnException: bool,
    pub maxRetainerSetSize: u32,
    pub ccsLength: u32,
    pub eraSelector: StgWord,
}

#[cfg(test)]
impl Arbitrary for _PROFILING_FLAGSOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _PROFILING_FLAGSOwned {
            doHeapProfile: Arbitrary::arbitrary(g),
            heapProfileInterval: Arbitrary::arbitrary(g),
            heapProfileIntervalTicks: Arbitrary::arbitrary(g),
            startHeapProfileAtStartup: Arbitrary::arbitrary(g),
            startTimeProfileAtStartup: Arbitrary::arbitrary(g),
            incrementUserEra: Arbitrary::arbitrary(g),
            showCCSOnException: Arbitrary::arbitrary(g),
            maxRetainerSetSize: Arbitrary::arbitrary(g),
            ccsLength: Arbitrary::arbitrary(g),
            eraSelector: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _PROFILING_FLAGSPointees {
    pub modSelector: c_char,
    pub descrSelector: c_char,
    pub typeSelector: c_char,
    pub ccSelector: c_char,
    pub ccsSelector: c_char,
    pub retainerSelector: c_char,
    pub bioSelector: c_char,
}

#[cfg(test)]
impl Arbitrary for _PROFILING_FLAGSPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _PROFILING_FLAGSPointees {
            modSelector: Arbitrary::arbitrary(g),
            descrSelector: Arbitrary::arbitrary(g),
            typeSelector: Arbitrary::arbitrary(g),
            ccSelector: Arbitrary::arbitrary(g),
            ccsSelector: Arbitrary::arbitrary(g),
            retainerSelector: Arbitrary::arbitrary(g),
            bioSelector: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _PROFILING_FLAGS {
    type Owned = _PROFILING_FLAGSOwned;
    type Pointees = _PROFILING_FLAGSPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            doHeapProfile: owned.doHeapProfile,
            heapProfileInterval: owned.heapProfileInterval,
            heapProfileIntervalTicks: owned.heapProfileIntervalTicks,
            startHeapProfileAtStartup: owned.startHeapProfileAtStartup.clone(),
            startTimeProfileAtStartup: owned.startTimeProfileAtStartup.clone(),
            incrementUserEra: owned.incrementUserEra.clone(),
            showCCSOnException: owned.showCCSOnException.clone(),
            maxRetainerSetSize: owned.maxRetainerSetSize,
            ccsLength: owned.ccsLength,
            eraSelector: owned.eraSelector,
            modSelector: unsafe { &raw mut (*pointees).modSelector },
            descrSelector: unsafe { &raw mut (*pointees).descrSelector },
            typeSelector: unsafe { &raw mut (*pointees).typeSelector },
            ccSelector: unsafe { &raw mut (*pointees).ccSelector },
            ccsSelector: unsafe { &raw mut (*pointees).ccsSelector },
            retainerSelector: unsafe { &raw mut (*pointees).retainerSelector },
            bioSelector: unsafe { &raw mut (*pointees).bioSelector },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            doHeapProfile: self.doHeapProfile,
            heapProfileInterval: self.heapProfileInterval,
            heapProfileIntervalTicks: self.heapProfileIntervalTicks,
            startHeapProfileAtStartup: self.startHeapProfileAtStartup.clone(),
            startTimeProfileAtStartup: self.startTimeProfileAtStartup.clone(),
            incrementUserEra: self.incrementUserEra.clone(),
            showCCSOnException: self.showCCSOnException.clone(),
            maxRetainerSetSize: self.maxRetainerSetSize,
            ccsLength: self.ccsLength,
            eraSelector: self.eraSelector,
        }
    }
}

pub type PROFILING_FLAGS = _PROFILING_FLAGS;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct _TRACE_FLAGS {
    pub tracing: c_int,
    pub timestamp: bool,
    pub scheduler: bool,
    pub gc: bool,
    pub nonmoving_gc: bool,
    pub sparks_sampled: bool,
    pub sparks_full: bool,
    pub ticky: bool,
    pub user: bool,
    pub eventlogFlushTime: Time,
    pub eventlogFlushTicks: c_int,
    pub trace_output: *mut c_char,
    pub nullWriter: bool,
}

#[cfg(feature = "sys")]
impl From<_TRACE_FLAGS> for sys::_TRACE_FLAGS {
    fn from(x: _TRACE_FLAGS) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _TRACE_FLAGSOwned {
    pub tracing: c_int,
    pub timestamp: bool,
    pub scheduler: bool,
    pub gc: bool,
    pub nonmoving_gc: bool,
    pub sparks_sampled: bool,
    pub sparks_full: bool,
    pub ticky: bool,
    pub user: bool,
    pub eventlogFlushTime: Time,
    pub eventlogFlushTicks: c_int,
    pub nullWriter: bool,
}

#[cfg(test)]
impl Arbitrary for _TRACE_FLAGSOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _TRACE_FLAGSOwned {
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
            nullWriter: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _TRACE_FLAGSPointees {
    pub trace_output: c_char,
}

#[cfg(test)]
impl Arbitrary for _TRACE_FLAGSPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _TRACE_FLAGSPointees {
            trace_output: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _TRACE_FLAGS {
    type Owned = _TRACE_FLAGSOwned;
    type Pointees = _TRACE_FLAGSPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            tracing: owned.tracing,
            timestamp: owned.timestamp.clone(),
            scheduler: owned.scheduler.clone(),
            gc: owned.gc.clone(),
            nonmoving_gc: owned.nonmoving_gc.clone(),
            sparks_sampled: owned.sparks_sampled.clone(),
            sparks_full: owned.sparks_full.clone(),
            ticky: owned.ticky.clone(),
            user: owned.user.clone(),
            eventlogFlushTime: owned.eventlogFlushTime,
            eventlogFlushTicks: owned.eventlogFlushTicks,
            nullWriter: owned.nullWriter.clone(),
            trace_output: unsafe { &raw mut (*pointees).trace_output },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            tracing: self.tracing,
            timestamp: self.timestamp.clone(),
            scheduler: self.scheduler.clone(),
            gc: self.gc.clone(),
            nonmoving_gc: self.nonmoving_gc.clone(),
            sparks_sampled: self.sparks_sampled.clone(),
            sparks_full: self.sparks_full.clone(),
            ticky: self.ticky.clone(),
            user: self.user.clone(),
            eventlogFlushTime: self.eventlogFlushTime,
            eventlogFlushTicks: self.eventlogFlushTicks,
            nullWriter: self.nullWriter.clone(),
        }
    }
}

pub type TRACE_FLAGS = _TRACE_FLAGS;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct _CONCURRENT_FLAGS {
    pub ctxtSwitchTime: Time,
    pub ctxtSwitchTicks: c_int,
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
///cbindgen:no-export
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
///cbindgen:no-export
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
///cbindgen:no-export
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
///cbindgen:no-export
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
#[derive(Clone)]
struct _TICKY_FLAGSOwned {
    pub showTickyStats: bool,
}

#[cfg(test)]
impl Arbitrary for _TICKY_FLAGSOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _TICKY_FLAGSOwned {
            showTickyStats: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _TICKY_FLAGSPointees {
    pub tickyFile: FILE,
}

#[cfg(test)]
impl Arbitrary for _TICKY_FLAGSPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _TICKY_FLAGSPointees {
            tickyFile: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _TICKY_FLAGS {
    type Owned = _TICKY_FLAGSOwned;
    type Pointees = _TICKY_FLAGSPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            showTickyStats: owned.showTickyStats.clone(),
            tickyFile: unsafe { &raw mut (*pointees).tickyFile },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            showTickyStats: self.showTickyStats.clone(),
        }
    }
}

pub type TICKY_FLAGS = _TICKY_FLAGS;

#[repr(C)]
///cbindgen:no-export
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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_RtsFlags"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut RtsFlags: RTS_FLAGS = 0;

static mut rts_argc: c_int = 0;

static mut rts_argv: *mut *mut c_char = null_mut();
