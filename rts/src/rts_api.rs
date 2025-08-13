use crate::hs_ffi::{
    HS_BOOL_FALSE, HS_BOOL_TRUE, HsBool, HsChar, HsDouble, HsFloat, HsFunPtr, HsInt, HsInt8,
    HsInt16, HsInt32, HsInt64, HsPtr, HsStablePtr, HsWord, HsWord8, HsWord16, HsWord32, HsWord64,
};
use crate::prelude::*;
use crate::rts::capability::Capability;
use crate::rts::event_log_writer::EventLogWriter;
use crate::rts::storage::closures::{StgClosure, StgClosure_};
use crate::rts::storage::tso::StgTSO;
use crate::rts::time::Time;
use crate::stg::W_;
use crate::stg::regs::{StgFunTable, StgRegTable};

#[cfg(test)]
mod tests;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SchedulerStatus {
    NoStatus = 0,
    Success = 1,
    Killed = 2,
    Interrupted = 3,
    HeapExhausted = 4,
}

pub type HaskellObj = *mut StgClosure_;

pub type PauseToken = PauseToken_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct PauseToken_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<PauseToken_> for sys::PauseToken_ {
    fn from(x: PauseToken_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for PauseToken_ {
    fn arbitrary(_g: &mut Gen) -> Self {
        PauseToken_ { _unused: [] }
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_pauseTokenCapability"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn pauseTokenCapability(pauseToken: *mut PauseToken) -> *mut Capability {
    unsafe {
        transmute(sys::pauseTokenCapability(
            pauseToken as *mut sys::PauseToken,
        ))
    }
}

pub(crate) type CapabilityPublic = CapabilityPublic_;

#[repr(C)]
///cbindgen:no-export
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

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RtsOptsEnabledEnum {
    RtsOptsNone = 0,
    RtsOptsIgnore = 1,
    RtsOptsIgnoreAll = 2,
    RtsOptsSafeOnly = 3,
    RtsOptsAll = 4,
}

#[repr(C)]
#[derive(Clone)]
pub struct RtsConfig {
    pub rts_opts_enabled: RtsOptsEnabledEnum,
    pub rts_opts_suggestions: HsBool,
    pub rts_opts: Option<&'static c_char>,
    pub rts_hs_main: HsBool,
    pub keep_cafs: HsBool,
    pub eventlog_writer: Option<&'static EventLogWriter>,
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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_defaultRtsConfig"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static defaultRtsConfig: RtsConfig = RtsConfig {
    rts_opts_enabled: RtsOptsEnabledEnum::RtsOptsSafeOnly,
    rts_opts_suggestions: HS_BOOL_TRUE,
    rts_opts: None,
    rts_hs_main: HS_BOOL_FALSE,
    keep_cafs: HS_BOOL_FALSE,
    eventlog_writer: None,   // TODO: FileEventLogWriter
    defaultsHook: None,      // TODO: FlagDefaultsHook
    onExitHook: None,        // TODO: OnExitHook
    stackOverflowHook: None, // TODO: StackOverflowHook
    outOfHeapHook: None,     // TODO: OutOfHeapHook
    mallocFailHook: None,    // TODO: MallocFailHook
    gcDoneHook: None,
    longGCSync: None,    // TODO: LongGCSync
    longGCSyncEnd: None, // TODO: LongGCSyncEnd
};

pub type GCDetails = GCDetails_;

#[repr(C)]
///cbindgen:no-export
#[derive(Clone)]
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

#[repr(C)]
///cbindgen:no-export
#[derive(Clone)]
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

pub type RTSStats = _RTSStats;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getRTSStats"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getRTSStats(s: *mut RTSStats) {
    unsafe { sys::getRTSStats(s as *mut sys::RTSStats) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getRTSStatsEnabled"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    unsafe { sys::getRTSStatsEnabled() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getAllocations"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getAllocations() -> u64 {
    unsafe { sys::getAllocations() }
}

#[instrument]
pub(crate) unsafe fn startupHaskell(
    argc: c_int,
    argv: *mut *mut c_char,
    init_root: Option<unsafe extern "C" fn()>,
) {
    unsafe { sys::startupHaskell(argc, argv, init_root) }
}

#[instrument]
pub(crate) unsafe fn shutdownHaskell() {
    unsafe { sys::shutdownHaskell() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_init_with_rtsopts"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_init_with_rtsopts(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::hs_init_with_rtsopts(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_init_ghc"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_init_ghc(
    argc: *mut c_int,
    argv: *mut *mut *mut c_char,
    rts_config: RtsConfig,
) {
    unsafe { sys::hs_init_ghc(argc, argv, transmute(rts_config)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_shutdownHaskellAndExit"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn shutdownHaskellAndExit(exitCode: c_int, fastExit: c_int) -> ! {
    unsafe { sys::shutdownHaskellAndExit(exitCode, fastExit) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_shutdownHaskellAndSignal"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn shutdownHaskellAndSignal(sig: c_int, fastExit: c_int) -> ! {
    unsafe { sys::shutdownHaskellAndSignal(sig, fastExit) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::getProgArgv(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setProgArgv(argc: c_int, argv: *mut *mut c_char) {
    unsafe { sys::setProgArgv(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getFullProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getFullProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::getFullProgArgv(argc, argv) }
}

#[instrument]
pub(crate) unsafe fn setFullProgArgv(argc: c_int, argv: *mut *mut c_char) {
    unsafe { sys::setFullProgArgv(argc, argv) }
}

#[instrument]
pub(crate) unsafe fn freeFullProgArgv() {
    unsafe { sys::freeFullProgArgv() }
}

static mut exitFn: Option<unsafe extern "C" fn(arg1: c_int)> = None;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_lock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    unsafe { transmute(sys::rts_lock()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_unlock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_unlock(token: *mut Capability) {
    unsafe { sys::rts_unlock(token as *mut sys::Capability) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_unsafeGetMyCapability")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    unsafe { transmute(sys::rts_unsafeGetMyCapability()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_setInCallCapability"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) {
    unsafe { sys::rts_setInCallCapability(preferred_capability, affinity) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_pinThreadToNumaNode"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(node: c_int) {
    unsafe { sys::rts_pinThreadToNumaNode(node) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkChar"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkChar(arg1: *mut Capability, c: HsChar) -> HaskellObj {
    unsafe { transmute(sys::rts_mkChar(arg1 as *mut sys::Capability, c)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkInt(arg1: *mut Capability, i: HsInt) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkInt8(arg1: *mut Capability, i: HsInt8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt8(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkInt16(arg1: *mut Capability, i: HsInt16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt16(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkInt32(arg1: *mut Capability, i: HsInt32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt32(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkInt64(arg1: *mut Capability, i: HsInt64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt64(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkWord(arg1: *mut Capability, w: HsWord) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkWord8(arg1: *mut Capability, w: HsWord8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord8(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkWord16(arg1: *mut Capability, w: HsWord16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord16(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkWord32(arg1: *mut Capability, w: HsWord32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord32(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkWord64(arg1: *mut Capability, w: HsWord64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord64(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkPtr(arg1: *mut Capability, a: HsPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkPtr(arg1 as *mut sys::Capability, a)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkFunPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkFunPtr(arg1: *mut Capability, a: HsFunPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFunPtr(arg1 as *mut sys::Capability, a)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkFloat"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkFloat(arg1: *mut Capability, f: HsFloat) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFloat(arg1 as *mut sys::Capability, f)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkDouble(arg1: *mut Capability, f: HsDouble) -> HaskellObj {
    unsafe { transmute(sys::rts_mkDouble(arg1 as *mut sys::Capability, f)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkStablePtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkStablePtr(arg1: *mut Capability, s: HsStablePtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkStablePtr(arg1 as *mut sys::Capability, s)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkBool"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkBool(arg1: *mut Capability, b: HsBool) -> HaskellObj {
    unsafe { transmute(sys::rts_mkBool(arg1 as *mut sys::Capability, b)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkString"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_mkString(arg1: *mut Capability, s: *const c_char) -> HaskellObj {
    unsafe {
        transmute(sys::rts_mkString(
            arg1 as *mut sys::Capability,
            s as *mut c_char,
        ))
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_apply"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_apply(
    arg1: *mut Capability,
    arg2: HaskellObj,
    arg3: HaskellObj,
) -> HaskellObj {
    unsafe {
        transmute(sys::rts_apply(
            arg1 as *mut sys::Capability,
            transmute(arg2),
            transmute(arg3),
        ))
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getChar"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getChar(arg1: HaskellObj) -> HsChar {
    unsafe { sys::rts_getChar(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getInt(arg1: HaskellObj) -> HsInt {
    unsafe { sys::rts_getInt(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getInt8(arg1: HaskellObj) -> HsInt8 {
    unsafe { sys::rts_getInt8(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getInt16(arg1: HaskellObj) -> HsInt16 {
    unsafe { sys::rts_getInt16(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getInt32(arg1: HaskellObj) -> HsInt32 {
    unsafe { sys::rts_getInt32(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getInt64(arg1: HaskellObj) -> HsInt64 {
    unsafe { sys::rts_getInt64(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getWord(arg1: HaskellObj) -> HsWord {
    unsafe { sys::rts_getWord(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getWord8(arg1: HaskellObj) -> HsWord8 {
    unsafe { sys::rts_getWord8(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getWord16(arg1: HaskellObj) -> HsWord16 {
    unsafe { sys::rts_getWord16(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getWord32(arg1: HaskellObj) -> HsWord32 {
    unsafe { sys::rts_getWord32(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getWord64(arg1: HaskellObj) -> HsWord64 {
    unsafe { sys::rts_getWord64(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getPtr(arg1: HaskellObj) -> HsPtr {
    unsafe { sys::rts_getPtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getFunPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getFunPtr(arg1: HaskellObj) -> HsFunPtr {
    unsafe { sys::rts_getFunPtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getFloat"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getFloat(arg1: HaskellObj) -> HsFloat {
    unsafe { sys::rts_getFloat(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getDouble(arg1: HaskellObj) -> HsDouble {
    unsafe { sys::rts_getDouble(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getStablePtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getStablePtr(arg1: HaskellObj) -> HsStablePtr {
    unsafe { sys::rts_getStablePtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getBool"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getBool(arg1: HaskellObj) -> HsBool {
    unsafe { sys::rts_getBool(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_eval"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_eval(arg1: *mut *mut Capability, p: HaskellObj, ret: *mut HaskellObj) {
    unsafe {
        sys::rts_eval(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_eval_"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_eval_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    unsafe {
        sys::rts_eval_(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            stack_size,
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalIO"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_evalIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        sys::rts_evalIO(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalStableIOMain"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIOMain(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe { sys::rts_evalStableIOMain(arg1 as *mut *mut sys::Capability, s, ret) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalStableIO"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIO(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe { sys::rts_evalStableIO(arg1 as *mut *mut sys::Capability, s, ret) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalLazyIO"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        sys::rts_evalLazyIO(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalLazyIO_"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: c_uint,
    ret: *mut HaskellObj,
) {
    unsafe {
        sys::rts_evalLazyIO_(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            stack_size,
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_inCall"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_inCall(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        sys::rts_inCall(
            arg1 as *mut *mut sys::Capability,
            transmute(p),
            ret as *mut sys::HaskellObj,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_checkSchedStatus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_checkSchedStatus(site: *mut c_char, arg1: *mut Capability) {
    unsafe { sys::rts_checkSchedStatus(site, arg1 as *mut sys::Capability) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getSchedStatus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getSchedStatus(cap: *mut Capability) -> SchedulerStatus {
    unsafe { transmute(sys::rts_getSchedStatus(cap as *mut sys::Capability)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_pause"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    unsafe { transmute(sys::rts_pause()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_resume"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_resume(pauseToken: *mut PauseToken) {
    unsafe { sys::rts_resume(pauseToken as *mut sys::PauseToken) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isPaused"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    unsafe { sys::rts_isPaused() }
}

pub(crate) type ListThreadsCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgTSO)>;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_listThreads"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_listThreads(cb: ListThreadsCb, user: *mut c_void) {
    unsafe { sys::rts_listThreads(transmute(cb), user) }
}

pub(crate) type ListRootsCb =
    Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgClosure)>;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_listMiscRoots"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_listMiscRoots(cb: ListRootsCb, user: *mut c_void) {
    unsafe { sys::rts_listMiscRoots(transmute(cb), user) }
}

#[instrument]
pub(crate) unsafe fn rts_done() {
    unsafe { sys::rts_done() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_clearMemory"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_clearMemory() {
    unsafe { sys::rts_clearMemory() }
}
