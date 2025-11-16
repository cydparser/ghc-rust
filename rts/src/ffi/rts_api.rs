use crate::ffi::hs_ffi::{HsBool, HsWord};
#[cfg(feature = "sys")]
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

/// - GHC_PLACES: {testsuite}
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

/// - GHC_PLACES: {libraries, testsuite}
pub type HaskellObj = *mut StgClosure_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct PauseToken_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<PauseToken_> for sys::PauseToken_ {
    fn from(x: PauseToken_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries, testsuite}
pub type PauseToken = PauseToken_;

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn pauseTokenCapability(pauseToken: *mut PauseToken) -> *mut Capability {
    #[cfg(feature = "sys")]
    unsafe {
        sys::pauseTokenCapability(pauseToken as *mut sys::PauseToken) as *mut Capability
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("pauseTokenCapability")
}

/// cbindgen:no-export
#[repr(C)]
pub struct CapabilityPublic_ {
    f: StgFunTable,
    r: StgRegTable,
}

#[cfg(feature = "sys")]
impl From<CapabilityPublic_> for sys::CapabilityPublic_ {
    fn from(x: CapabilityPublic_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type CapabilityPublic = CapabilityPublic_;

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum RtsOptsEnabledEnum {
    RtsOptsNone = 0,
    RtsOptsIgnore = 1,
    RtsOptsIgnoreAll = 2,
    RtsOptsSafeOnly = 3,
    RtsOptsAll = 4,
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

/// - GHC_PLACES: {driver, testsuite, utils}
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

#[cfg(feature = "sys")]
impl From<RtsConfig> for sys::RtsConfig {
    fn from(x: RtsConfig) -> Self {
        unsafe { transmute(x) }
    }
}

// /// - GHC_PLACES: {testsuite, utils}
// #[ffi]
// #[unsafe(no_mangle)]
// TODO(rust): pub static defaultRtsConfig: RtsConfig;

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

#[cfg(feature = "sys")]
impl From<GCDetails_> for sys::GCDetails_ {
    fn from(x: GCDetails_) -> Self {
        unsafe { transmute(x) }
    }
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

/// - GHC_PLACES: {libraries}
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

#[cfg(feature = "sys")]
impl From<_RTSStats> for sys::_RTSStats {
    fn from(x: _RTSStats) -> Self {
        unsafe { transmute(x) }
    }
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

/// - GHC_PLACES: {libraries, testsuite}
pub type RTSStats = _RTSStats;

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStats(s: *mut RTSStats) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getRTSStats(s as *mut sys::RTSStats)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getRTSStats")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getRTSStatsEnabled()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getRTSStatsEnabled")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getAllocations() -> u64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getAllocations()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getAllocations")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_with_rtsopts(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_init_with_rtsopts(argc, argv)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_init_with_rtsopts")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init_ghc(
    argc: *mut c_int,
    argv: *mut *mut *mut c_char,
    rts_config: RtsConfig,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_init_ghc(argc, argv, transmute(rts_config))
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_init_ghc")
}

/// - GHC_PLACES: {libraries, utils}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn shutdownHaskellAndExit(exitCode: c_int, fastExit: c_int) -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::shutdownHaskellAndExit(exitCode, fastExit)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("shutdownHaskellAndExit")
}

/// - GHC_PLACES: {libraries, utils}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn shutdownHaskellAndSignal(sig: c_int, fastExit: c_int) -> ! {
    #[cfg(feature = "sys")]
    unsafe {
        sys::shutdownHaskellAndSignal(sig, fastExit)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("shutdownHaskellAndSignal")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getProgArgv(argc, argv)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getProgArgv")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setProgArgv(argc: c_int, argv: *mut *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setProgArgv(argc, argv)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setProgArgv")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getFullProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getFullProgArgv(argc, argv)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getFullProgArgv")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_lock() as *mut Capability
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_lock")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unlock(token: *mut Capability) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_unlock(token as *mut sys::Capability)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_unlock")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_unsafeGetMyCapability() as *mut Capability
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_unsafeGetMyCapability")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_setInCallCapability(preferred_capability, affinity)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_setInCallCapability")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(node: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_pinThreadToNumaNode(node)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_pinThreadToNumaNode")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkChar(arg1: *mut Capability, c: HsChar) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkChar(arg1 as *mut sys::Capability, c) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkChar")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt(arg1: *mut Capability, i: HsInt) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkInt(arg1 as *mut sys::Capability, i) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkInt")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt8(arg1: *mut Capability, i: HsInt8) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkInt8(arg1 as *mut sys::Capability, i) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkInt8")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt16(arg1: *mut Capability, i: HsInt16) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkInt16(arg1 as *mut sys::Capability, i) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkInt16")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt32(arg1: *mut Capability, i: HsInt32) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkInt32(arg1 as *mut sys::Capability, i) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkInt32")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt64(arg1: *mut Capability, i: HsInt64) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkInt64(arg1 as *mut sys::Capability, i) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkInt64")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord(arg1: *mut Capability, w: HsWord) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkWord(arg1 as *mut sys::Capability, w) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkWord")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord8(arg1: *mut Capability, w: HsWord8) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkWord8(arg1 as *mut sys::Capability, w) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkWord8")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord16(arg1: *mut Capability, w: HsWord16) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkWord16(arg1 as *mut sys::Capability, w) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkWord16")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord32(arg1: *mut Capability, w: HsWord32) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkWord32(arg1 as *mut sys::Capability, w) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkWord32")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord64(arg1: *mut Capability, w: HsWord64) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkWord64(arg1 as *mut sys::Capability, w) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkWord64")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkPtr(arg1: *mut Capability, a: HsPtr) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkPtr(arg1 as *mut sys::Capability, a) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkPtr")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFunPtr(arg1: *mut Capability, a: HsFunPtr) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkFunPtr(arg1 as *mut sys::Capability, a) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkFunPtr")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFloat(arg1: *mut Capability, f: HsFloat) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkFloat(arg1 as *mut sys::Capability, f) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkFloat")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkDouble(arg1: *mut Capability, f: HsDouble) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkDouble(arg1 as *mut sys::Capability, f) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkDouble")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkStablePtr(arg1: *mut Capability, s: HsStablePtr) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkStablePtr(arg1 as *mut sys::Capability, s) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkStablePtr")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkBool(arg1: *mut Capability, b: HsBool) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkBool(arg1 as *mut sys::Capability, b) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkBool")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkString(arg1: *mut Capability, s: *mut c_char) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_mkString(arg1 as *mut sys::Capability, s) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_mkString")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_apply(
    arg1: *mut Capability,
    arg2: HaskellObj,
    arg3: HaskellObj,
) -> HaskellObj {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_apply(
            arg1 as *mut sys::Capability,
            arg2 as sys::HaskellObj,
            arg3 as sys::HaskellObj,
        ) as HaskellObj
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_apply")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getChar(arg1: HaskellObj) -> HsChar {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getChar(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getChar")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt(arg1: HaskellObj) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getInt(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getInt")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt8(arg1: HaskellObj) -> HsInt8 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getInt8(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getInt8")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt16(arg1: HaskellObj) -> HsInt16 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getInt16(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getInt16")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt32(arg1: HaskellObj) -> HsInt32 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getInt32(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getInt32")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt64(arg1: HaskellObj) -> HsInt64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getInt64(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getInt64")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord(arg1: HaskellObj) -> HsWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getWord(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getWord")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord8(arg1: HaskellObj) -> HsWord8 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getWord8(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getWord8")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord16(arg1: HaskellObj) -> HsWord16 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getWord16(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getWord16")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord32(arg1: HaskellObj) -> HsWord32 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getWord32(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getWord32")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord64(arg1: HaskellObj) -> HsWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getWord64(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getWord64")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getPtr(arg1: HaskellObj) -> HsPtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getPtr(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getPtr")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFunPtr(arg1: HaskellObj) -> HsFunPtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getFunPtr(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getFunPtr")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFloat(arg1: HaskellObj) -> HsFloat {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getFloat(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getFloat")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getDouble(arg1: HaskellObj) -> HsDouble {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getDouble(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getDouble")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getStablePtr(arg1: HaskellObj) -> HsStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getStablePtr(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getStablePtr")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getBool(arg1: HaskellObj) -> HsBool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getBool(arg1 as sys::HaskellObj)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getBool")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval(arg1: *mut *mut Capability, p: HaskellObj, ret: *mut HaskellObj) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_eval(
            arg1 as *mut *mut sys::Capability,
            p as sys::HaskellObj,
            ret as *mut sys::HaskellObj,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_eval")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_eval_(
            arg1 as *mut *mut sys::Capability,
            p as sys::HaskellObj,
            stack_size,
            ret as *mut sys::HaskellObj,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_eval_")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_evalIO(
            arg1 as *mut *mut sys::Capability,
            p as sys::HaskellObj,
            ret as *mut sys::HaskellObj,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_evalIO")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIOMain(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_evalStableIOMain(arg1 as *mut *mut sys::Capability, s, ret)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_evalStableIOMain")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIO(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_evalStableIO(arg1 as *mut *mut sys::Capability, s, ret)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_evalStableIO")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_evalLazyIO(
            arg1 as *mut *mut sys::Capability,
            p as sys::HaskellObj,
            ret as *mut sys::HaskellObj,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_evalLazyIO")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_evalLazyIO_(
            arg1 as *mut *mut sys::Capability,
            p as sys::HaskellObj,
            stack_size,
            ret as *mut sys::HaskellObj,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_evalLazyIO_")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getSchedStatus(cap: *mut Capability) -> SchedulerStatus {
    #[cfg(feature = "sys")]
    unsafe {
        transmute(sys::rts_getSchedStatus(cap as *mut sys::Capability))
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getSchedStatus")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_pause() as *mut PauseToken
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_pause")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_resume(pauseToken: *mut PauseToken) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_resume(pauseToken as *mut sys::PauseToken)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_resume")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_isPaused()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_isPaused")
}

pub(crate) type ListThreadsCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgTSO)>;
#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listThreads(cb: ListThreadsCb, user: *mut c_void) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_listThreads(transmute(cb), user)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_listThreads")
}

pub(crate) type ListRootsCb =
    Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgClosure)>;
#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listMiscRoots(cb: ListRootsCb, user: *mut c_void) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_listMiscRoots(transmute(cb), user)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_listMiscRoots")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_clearMemory() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_clearMemory()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_clearMemory")
}
