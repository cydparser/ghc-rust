use crate::ffi::hs_ffi::{HsBool, HsWord};
use crate::ffi::hs_ffi::{
    HsChar, HsDouble, HsFloat, HsFunPtr, HsInt, HsInt8, HsInt16, HsInt32, HsInt64, HsPtr,
    HsStablePtr, HsWord8, HsWord16, HsWord32, HsWord64,
};
use crate::ffi::rts::event_log_writer::EventLogWriter;
use crate::ffi::rts::storage::closures::{StgClosure, StgClosure_};
use crate::ffi::rts::storage::tso::StgTSO;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::W_;
use crate::ffi::stg::regs::{StgFunTable, StgRegTable};
use crate::prelude::*;

pub use crate::capability::Capability;

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum SchedulerStatus {
    NoStatus = 0,
    Success = 1,
    Killed = 2,
    Interrupted = 3,
    HeapExhausted = 4,
    SchedulerStatus_End = 5,
}

#[cfg(feature = "sys")]
impl From<SchedulerStatus> for sys::SchedulerStatus {
    fn from(v: SchedulerStatus) -> Self {
        use SchedulerStatus::*;
        match v {
            NoStatus => sys::SchedulerStatus::NoStatus,
            Success => sys::SchedulerStatus::Success,
            Killed => sys::SchedulerStatus::Killed,
            Interrupted => sys::SchedulerStatus::Interrupted,
            HeapExhausted => sys::SchedulerStatus::HeapExhausted,
            SchedulerStatus_End => sys::SchedulerStatus::SchedulerStatus_End,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::SchedulerStatus> for SchedulerStatus {
    fn from(v: sys::SchedulerStatus) -> Self {
        use SchedulerStatus::*;
        match v {
            sys::SchedulerStatus::NoStatus => NoStatus,
            sys::SchedulerStatus::Success => Success,
            sys::SchedulerStatus::Killed => Killed,
            sys::SchedulerStatus::Interrupted => Interrupted,
            sys::SchedulerStatus::HeapExhausted => HeapExhausted,
            sys::SchedulerStatus::SchedulerStatus_End => SchedulerStatus_End,
        }
    }
}

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

#[cfg(test)]
impl Arbitrary for SchedulerStatus {
    fn arbitrary(g: &mut Gen) -> Self {
        use SchedulerStatus::*;
        match usize::arbitrary(g) % 6 {
            0 => NoStatus,
            1 => Success,
            2 => Killed,
            3 => Interrupted,
            4 => HeapExhausted,
            5.. => SchedulerStatus_End,
        }
    }
}

#[ffi(compiler, ghc_lib, testsuite)]
pub type HaskellObj = *mut StgClosure_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct PauseToken_ {
    _unused: [u8; 0],
}

#[ffi(ghc_lib, testsuite)]
pub type PauseToken = PauseToken_;

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn pauseTokenCapability(pauseToken: *mut PauseToken) -> *mut Capability {
    sys! {
        pauseTokenCapability(pauseToken as * mut sys::PauseToken).cast()
    }
}

#[ffi(compiler, ghc_lib, libraries, utils)]
#[repr(C)]
pub struct CapabilityPublic_ {
    pub f: StgFunTable,
    pub r: StgRegTable,
}

#[expect(unused)]
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
    pub gcDoneHook: Option<unsafe extern "C" fn(stats: *const GCDetails_)>,
    pub longGCSync: Option<unsafe extern "C" fn(this_cap: u32, time_ns: Time)>,
    pub longGCSyncEnd: Option<unsafe extern "C" fn(time_ns: Time)>,
}

// #[ffi(compiler, docs, testsuite, utils)]
// #[unsafe(no_mangle)]
// TODO(rust): pub static defaultRtsConfig: RtsConfig = todo!();

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

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStats(s: *mut RTSStats) {
    sys! {
        getRTSStats(s as * mut sys::RTSStats)
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    sys! {
        getRTSStatsEnabled()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getAllocations() -> u64 {
    sys! {
        getAllocations()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_with_rtsopts(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    sys! {
        hs_init_with_rtsopts(argc, argv)
    }
}

#[ffi(compiler, docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_ghc(
    argc: *mut c_int,
    argv: *mut *mut *mut c_char,
    rts_config: RtsConfig,
) {
    sys! {
        hs_init_ghc(argc, argv, transmute(rts_config))
    }
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndExit(exitCode: c_int, fastExit: c_int) -> ! {
    before_exit("shutdownHaskellAndExit");
    sys! {
        shutdownHaskellAndExit(exitCode, fastExit)
    }
}

#[ffi(ghc_lib, utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdownHaskellAndSignal(sig: c_int, fastExit: c_int) -> ! {
    before_exit("shutdownHaskellAndSignal");
    sys! {
        shutdownHaskellAndSignal(sig, fastExit)
    }
}

#[ffi(ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    sys! {
        getProgArgv(argc, argv)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setProgArgv(argc: c_int, argv: *mut *mut c_char) {
    sys! {
        setProgArgv(argc, argv)
    }
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getFullProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    sys! {
        getFullProgArgv(argc, argv)
    }
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    sys! {
        rts_lock().cast()
    }
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unlock(token: *mut Capability) {
    sys! {
        rts_unlock(token as * mut sys::Capability)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    sys! {
        rts_unsafeGetMyCapability().cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) {
    sys! {
        rts_setInCallCapability(preferred_capability, affinity)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(node: c_int) {
    sys! {
        rts_pinThreadToNumaNode(node)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkChar(arg1: *mut Capability, c: HsChar) -> HaskellObj {
    sys! {
        rts_mkChar(arg1 as * mut sys::Capability, c).cast()
    }
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt(arg1: *mut Capability, i: HsInt) -> HaskellObj {
    sys! {
        rts_mkInt(arg1 as * mut sys::Capability, i).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt8(arg1: *mut Capability, i: HsInt8) -> HaskellObj {
    sys! {
        rts_mkInt8(arg1 as * mut sys::Capability, i).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt16(arg1: *mut Capability, i: HsInt16) -> HaskellObj {
    sys! {
        rts_mkInt16(arg1 as * mut sys::Capability, i).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt32(arg1: *mut Capability, i: HsInt32) -> HaskellObj {
    sys! {
        rts_mkInt32(arg1 as * mut sys::Capability, i).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt64(arg1: *mut Capability, i: HsInt64) -> HaskellObj {
    sys! {
        rts_mkInt64(arg1 as * mut sys::Capability, i).cast()
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord(arg1: *mut Capability, w: HsWord) -> HaskellObj {
    sys! {
        rts_mkWord(arg1 as * mut sys::Capability, w).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord8(arg1: *mut Capability, w: HsWord8) -> HaskellObj {
    sys! {
        rts_mkWord8(arg1 as * mut sys::Capability, w).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord16(arg1: *mut Capability, w: HsWord16) -> HaskellObj {
    sys! {
        rts_mkWord16(arg1 as * mut sys::Capability, w).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord32(arg1: *mut Capability, w: HsWord32) -> HaskellObj {
    sys! {
        rts_mkWord32(arg1 as * mut sys::Capability, w).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord64(arg1: *mut Capability, w: HsWord64) -> HaskellObj {
    sys! {
        rts_mkWord64(arg1 as * mut sys::Capability, w).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkPtr(arg1: *mut Capability, a: HsPtr) -> HaskellObj {
    sys! {
        rts_mkPtr(arg1 as * mut sys::Capability, a).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFunPtr(arg1: *mut Capability, a: HsFunPtr) -> HaskellObj {
    sys! {
        rts_mkFunPtr(arg1 as * mut sys::Capability, a).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFloat(arg1: *mut Capability, f: HsFloat) -> HaskellObj {
    sys! {
        rts_mkFloat(arg1 as * mut sys::Capability, f).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkDouble(arg1: *mut Capability, f: HsDouble) -> HaskellObj {
    sys! {
        rts_mkDouble(arg1 as * mut sys::Capability, f).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkStablePtr(arg1: *mut Capability, s: HsStablePtr) -> HaskellObj {
    sys! {
        rts_mkStablePtr(arg1 as * mut sys::Capability, s).cast()
    }
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkBool(arg1: *mut Capability, b: HsBool) -> HaskellObj {
    sys! {
        rts_mkBool(arg1 as * mut sys::Capability, b).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkString(arg1: *mut Capability, s: *mut c_char) -> HaskellObj {
    sys! {
        rts_mkString(arg1 as * mut sys::Capability, s).cast()
    }
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_apply(
    arg1: *mut Capability,
    arg2: HaskellObj,
    arg3: HaskellObj,
) -> HaskellObj {
    sys! {
        rts_apply(arg1 as * mut sys::Capability, arg2.cast(), arg3.cast()).cast()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getChar(arg1: HaskellObj) -> HsChar {
    sys! {
        rts_getChar(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt(arg1: HaskellObj) -> HsInt {
    sys! {
        rts_getInt(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt8(arg1: HaskellObj) -> HsInt8 {
    sys! {
        rts_getInt8(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt16(arg1: HaskellObj) -> HsInt16 {
    sys! {
        rts_getInt16(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt32(arg1: HaskellObj) -> HsInt32 {
    sys! {
        rts_getInt32(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt64(arg1: HaskellObj) -> HsInt64 {
    sys! {
        rts_getInt64(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord(arg1: HaskellObj) -> HsWord {
    sys! {
        rts_getWord(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord8(arg1: HaskellObj) -> HsWord8 {
    sys! {
        rts_getWord8(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord16(arg1: HaskellObj) -> HsWord16 {
    sys! {
        rts_getWord16(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord32(arg1: HaskellObj) -> HsWord32 {
    sys! {
        rts_getWord32(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord64(arg1: HaskellObj) -> HsWord64 {
    sys! {
        rts_getWord64(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getPtr(arg1: HaskellObj) -> HsPtr {
    sys! {
        rts_getPtr(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFunPtr(arg1: HaskellObj) -> HsFunPtr {
    sys! {
        rts_getFunPtr(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFloat(arg1: HaskellObj) -> HsFloat {
    sys! {
        rts_getFloat(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getDouble(arg1: HaskellObj) -> HsDouble {
    sys! {
        rts_getDouble(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getStablePtr(arg1: HaskellObj) -> HsStablePtr {
    sys! {
        rts_getStablePtr(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getBool(arg1: HaskellObj) -> HsBool {
    sys! {
        rts_getBool(arg1.cast())
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval(arg1: *mut *mut Capability, p: HaskellObj, ret: *mut HaskellObj) {
    sys! {
        rts_eval(arg1 as * mut * mut sys::Capability, p.cast(), ret as * mut
        sys::HaskellObj)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    sys! {
        rts_eval_(arg1 as * mut * mut sys::Capability, p.cast(), stack_size, ret as * mut
        sys::HaskellObj)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    sys! {
        rts_evalIO(arg1 as * mut * mut sys::Capability, p.cast(), ret as * mut
        sys::HaskellObj)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIOMain(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    sys! {
        rts_evalStableIOMain(arg1 as * mut * mut sys::Capability, s, ret)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIO(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    sys! {
        rts_evalStableIO(arg1 as * mut * mut sys::Capability, s, ret)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    sys! {
        rts_evalLazyIO(arg1 as * mut * mut sys::Capability, p.cast(), ret as * mut
        sys::HaskellObj)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    sys! {
        rts_evalLazyIO_(arg1 as * mut * mut sys::Capability, p.cast(), stack_size, ret as
        * mut sys::HaskellObj)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_inCall(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    sys! {
        rts_inCall(arg1 as * mut * mut sys::Capability, p.cast(), ret as * mut
        sys::HaskellObj)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_checkSchedStatus(site: *mut c_char, arg1: *mut Capability) {
    sys! {
        rts_checkSchedStatus(site, arg1 as * mut sys::Capability)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getSchedStatus(cap: *mut Capability) -> SchedulerStatus {
    sys! {
        transmute(rts_getSchedStatus(cap as * mut sys::Capability))
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    sys! {
        rts_pause().cast()
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_resume(pauseToken: *mut PauseToken) {
    sys! {
        rts_resume(pauseToken as * mut sys::PauseToken)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    sys! {
        rts_isPaused()
    }
}

#[ffi(testsuite)]
pub type ListThreadsCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgTSO)>;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listThreads(cb: ListThreadsCb, user: *mut c_void) {
    sys! {
        rts_listThreads(transmute(cb), user)
    }
}

#[ffi(testsuite)]
pub type ListRootsCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgClosure)>;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listMiscRoots(cb: ListRootsCb, user: *mut c_void) {
    sys! {
        rts_listMiscRoots(transmute(cb), user)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_clearMemory() {
    sys! {
        rts_clearMemory()
    }
}
