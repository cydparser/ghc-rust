pub use crate::capability::Capability;
pub use crate::capability::rts_unsafeGetMyCapability;
use crate::ffi::hs_ffi::HsBool;
use crate::ffi::rts::event_log_writer::EventLogWriter;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::regs::{StgFunTable, StgRegTable};
use crate::prelude::*;
pub use crate::rts_api::{
    HaskellObj, ListRootsCb, ListThreadsCb, PauseToken, PauseToken_, SchedulerStatus,
    pauseTokenCapability, rts_apply, rts_checkSchedStatus, rts_eval, rts_eval_, rts_evalIO,
    rts_evalLazyIO, rts_evalLazyIO_, rts_evalStableIO, rts_evalStableIOMain, rts_getBool,
    rts_getChar, rts_getDouble, rts_getFloat, rts_getFunPtr, rts_getInt, rts_getInt8, rts_getInt16,
    rts_getInt32, rts_getInt64, rts_getPtr, rts_getSchedStatus, rts_getStablePtr, rts_getWord,
    rts_getWord8, rts_getWord16, rts_getWord32, rts_getWord64, rts_inCall, rts_isPaused,
    rts_listMiscRoots, rts_listThreads, rts_lock, rts_mkBool, rts_mkChar, rts_mkDouble,
    rts_mkFloat, rts_mkFunPtr, rts_mkInt, rts_mkInt8, rts_mkInt16, rts_mkInt32, rts_mkInt64,
    rts_mkPtr, rts_mkStablePtr, rts_mkString, rts_mkWord, rts_mkWord8, rts_mkWord16, rts_mkWord32,
    rts_mkWord64, rts_pause, rts_resume, rts_unlock,
};
pub use crate::rts_flags::{getFullProgArgv, getProgArgv, setProgArgv};
pub use crate::rts_startup::{
    hs_init_ghc, hs_init_with_rtsopts, shutdownHaskellAndExit, shutdownHaskellAndSignal,
};
pub use crate::sm::storage::rts_clearMemory;
pub use crate::stats::{getAllocations, getRTSStats, getRTSStatsEnabled};
pub use crate::task::{rts_pinThreadToNumaNode, rts_setInCallCapability};

#[cfg(test)]
mod tests;

impl TryFrom<u32> for SchedulerStatus {
    type Error = ();

    fn try_from(d: u32) -> Result<SchedulerStatus, ()> {
        use SchedulerStatus::*;

        match d {
            0 => Ok(NoStatus),
            1 => Ok(Success),
            2 => Ok(Killed),
            3 => Ok(Interrupted),
            4 => Ok(HeapExhausted),
            5 => Ok(SchedulerStatus_End),
            _ => Err(()),
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
pub struct CapabilityPublic_ {
    f: StgFunTable,
    r: StgRegTable,
}

pub(crate) type CapabilityPublic = CapabilityPublic_;

#[ffi(compiler, docs, driver, testsuite, utils)]
#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum RtsOptsEnabledEnum {
    RtsOptsNone = 0,
    RtsOptsIgnore = 1,
    RtsOptsIgnoreAll = 2,
    RtsOptsSafeOnly = 3,
    RtsOptsAll = 4,
}

#[cfg(feature = "sys")]
impl From<RtsOptsEnabledEnum> for sys::RtsOptsEnabledEnum {
    fn from(v: RtsOptsEnabledEnum) -> Self {
        use RtsOptsEnabledEnum::*;

        match v {
            RtsOptsNone => sys::RtsOptsEnabledEnum::RtsOptsNone,
            RtsOptsIgnore => sys::RtsOptsEnabledEnum::RtsOptsIgnore,
            RtsOptsIgnoreAll => sys::RtsOptsEnabledEnum::RtsOptsIgnoreAll,
            RtsOptsSafeOnly => sys::RtsOptsEnabledEnum::RtsOptsSafeOnly,
            RtsOptsAll => sys::RtsOptsEnabledEnum::RtsOptsAll,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::RtsOptsEnabledEnum> for RtsOptsEnabledEnum {
    fn from(v: sys::RtsOptsEnabledEnum) -> Self {
        use RtsOptsEnabledEnum::*;

        match v {
            sys::RtsOptsEnabledEnum::RtsOptsNone => RtsOptsNone,
            sys::RtsOptsEnabledEnum::RtsOptsIgnore => RtsOptsIgnore,
            sys::RtsOptsEnabledEnum::RtsOptsIgnoreAll => RtsOptsIgnoreAll,
            sys::RtsOptsEnabledEnum::RtsOptsSafeOnly => RtsOptsSafeOnly,
            sys::RtsOptsEnabledEnum::RtsOptsAll => RtsOptsAll,
        }
    }
}

impl TryFrom<u32> for RtsOptsEnabledEnum {
    type Error = ();

    fn try_from(d: u32) -> Result<RtsOptsEnabledEnum, ()> {
        use RtsOptsEnabledEnum::*;

        match d {
            0 => Ok(RtsOptsNone),
            1 => Ok(RtsOptsIgnore),
            2 => Ok(RtsOptsIgnoreAll),
            3 => Ok(RtsOptsSafeOnly),
            4 => Ok(RtsOptsAll),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for RtsOptsEnabledEnum {
    fn arbitrary(g: &mut Gen) -> Self {
        use RtsOptsEnabledEnum::*;

        match usize::arbitrary(g) % 5 {
            0 => RtsOptsNone,
            1 => RtsOptsIgnore,
            2 => RtsOptsIgnoreAll,
            3 => RtsOptsSafeOnly,
            4.. => RtsOptsAll,
        }
    }
}

#[ffi(compiler, docs, driver, testsuite, utils)]
#[repr(C)]
pub struct RtsConfig {
    pub rts_opts_enabled: RtsOptsEnabledEnum,
    pub rts_opts_suggestions: HsBool,
    pub rts_opts: *const c_char,
    pub rts_hs_main: HsBool,
    pub keep_cafs: HsBool,
    pub eventlog_writer: *const EventLogWriter,
    pub defaultsHook: Option<unsafe extern "C" fn()>,
    pub onExitHook: Option<unsafe extern "C" fn()>,
    pub stackOverflowHook: Option<unsafe extern "C" fn(stack_size: W_)>,
    pub outOfHeapHook: Option<unsafe extern "C" fn(request_size: W_, heap_size: W_)>,
    pub mallocFailHook: Option<unsafe extern "C" fn(request_size: W_, msg: *const c_char)>,
    pub gcDoneHook: Option<unsafe extern "C" fn(stats: *const GCDetails)>,
    pub longGCSync: Option<unsafe extern "C" fn(this_cap: u32, time_ns: Time)>,
    pub longGCSyncEnd: Option<unsafe extern "C" fn(time_ns: Time)>,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct GCDetails_ {
    gen_: u32,
    threads: u32,
    allocated_bytes: u64,
    live_bytes: u64,
    large_objects_bytes: u64,
    compact_bytes: u64,
    slop_bytes: u64,
    mem_in_use_bytes: u64,
    copied_bytes: u64,
    block_fragmentation_bytes: u64,
    par_max_copied_bytes: u64,
    par_balanced_copied_bytes: u64,
    sync_elapsed_ns: Time,
    cpu_ns: Time,
    elapsed_ns: Time,
    nonmoving_gc_sync_cpu_ns: Time,
    nonmoving_gc_sync_elapsed_ns: Time,
    nonmoving_gc_cpu_ns: Time,
    nonmoving_gc_elapsed_ns: Time,
}

#[cfg(test)]
impl Arbitrary for GCDetails_ {
    fn arbitrary(g: &mut Gen) -> Self {
        GCDetails_ {
            gen_: Arbitrary::arbitrary(g),
            threads: Arbitrary::arbitrary(g),
            allocated_bytes: Arbitrary::arbitrary(g),
            live_bytes: Arbitrary::arbitrary(g),
            large_objects_bytes: Arbitrary::arbitrary(g),
            compact_bytes: Arbitrary::arbitrary(g),
            slop_bytes: Arbitrary::arbitrary(g),
            mem_in_use_bytes: Arbitrary::arbitrary(g),
            copied_bytes: Arbitrary::arbitrary(g),
            block_fragmentation_bytes: Arbitrary::arbitrary(g),
            par_max_copied_bytes: Arbitrary::arbitrary(g),
            par_balanced_copied_bytes: Arbitrary::arbitrary(g),
            sync_elapsed_ns: Arbitrary::arbitrary(g),
            cpu_ns: Arbitrary::arbitrary(g),
            elapsed_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_sync_cpu_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_sync_elapsed_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_cpu_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_elapsed_ns: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(ghc_lib)]
pub type GCDetails = GCDetails_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct _RTSStats {
    gcs: u32,
    major_gcs: u32,
    allocated_bytes: u64,
    max_live_bytes: u64,
    max_large_objects_bytes: u64,
    max_compact_bytes: u64,
    max_slop_bytes: u64,
    max_mem_in_use_bytes: u64,
    cumulative_live_bytes: u64,
    copied_bytes: u64,
    par_copied_bytes: u64,
    cumulative_par_max_copied_bytes: u64,
    cumulative_par_balanced_copied_bytes: u64,
    init_cpu_ns: Time,
    init_elapsed_ns: Time,
    mutator_cpu_ns: Time,
    mutator_elapsed_ns: Time,
    gc_cpu_ns: Time,
    gc_elapsed_ns: Time,
    cpu_ns: Time,
    elapsed_ns: Time,
    gc: GCDetails,
    any_work: u64,
    scav_find_work: u64,
    max_n_todo_overflow: u64,
    nonmoving_gc_sync_cpu_ns: Time,
    nonmoving_gc_sync_elapsed_ns: Time,
    nonmoving_gc_sync_max_elapsed_ns: Time,
    nonmoving_gc_cpu_ns: Time,
    nonmoving_gc_elapsed_ns: Time,
    nonmoving_gc_max_elapsed_ns: Time,
}

#[cfg(test)]
impl Arbitrary for _RTSStats {
    fn arbitrary(g: &mut Gen) -> Self {
        _RTSStats {
            gcs: Arbitrary::arbitrary(g),
            major_gcs: Arbitrary::arbitrary(g),
            allocated_bytes: Arbitrary::arbitrary(g),
            max_live_bytes: Arbitrary::arbitrary(g),
            max_large_objects_bytes: Arbitrary::arbitrary(g),
            max_compact_bytes: Arbitrary::arbitrary(g),
            max_slop_bytes: Arbitrary::arbitrary(g),
            max_mem_in_use_bytes: Arbitrary::arbitrary(g),
            cumulative_live_bytes: Arbitrary::arbitrary(g),
            copied_bytes: Arbitrary::arbitrary(g),
            par_copied_bytes: Arbitrary::arbitrary(g),
            cumulative_par_max_copied_bytes: Arbitrary::arbitrary(g),
            cumulative_par_balanced_copied_bytes: Arbitrary::arbitrary(g),
            init_cpu_ns: Arbitrary::arbitrary(g),
            init_elapsed_ns: Arbitrary::arbitrary(g),
            mutator_cpu_ns: Arbitrary::arbitrary(g),
            mutator_elapsed_ns: Arbitrary::arbitrary(g),
            gc_cpu_ns: Arbitrary::arbitrary(g),
            gc_elapsed_ns: Arbitrary::arbitrary(g),
            cpu_ns: Arbitrary::arbitrary(g),
            elapsed_ns: Arbitrary::arbitrary(g),
            gc: Arbitrary::arbitrary(g),
            any_work: Arbitrary::arbitrary(g),
            scav_find_work: Arbitrary::arbitrary(g),
            max_n_todo_overflow: Arbitrary::arbitrary(g),
            nonmoving_gc_sync_cpu_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_sync_elapsed_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_sync_max_elapsed_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_cpu_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_elapsed_ns: Arbitrary::arbitrary(g),
            nonmoving_gc_max_elapsed_ns: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(ghc_lib, testsuite)]
pub type RTSStats = _RTSStats;
