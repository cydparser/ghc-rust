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

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum SchedulerStatus {
    NoStatus = 0,
    Success = 1,
    Killed = 2,
    Interrupted = 3,
    HeapExhausted = 4,
}

pub type HaskellObj = *mut StgClosure_;

pub type Capability = Capability_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct PauseToken_ {
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
    fn arbitrary(g: &mut Gen) -> Self {
        PauseToken_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

pub type PauseToken = PauseToken_;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_pauseTokenCapability"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn pauseTokenCapability(pauseToken: *mut PauseToken) -> *mut Capability {
    unsafe {
        transmute(sys::pauseTokenCapability(
            pauseToken as *mut sys::PauseToken,
        ))
    }
}

#[repr(C)]
///cbindgen:no-export
pub(crate) struct CapabilityPublic_ {
    pub f: StgFunTable,
    pub r: StgRegTable,
}

#[cfg(feature = "sys")]
impl From<CapabilityPublic_> for sys::CapabilityPublic_ {
    fn from(x: CapabilityPublic_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for CapabilityPublic_ {
    fn arbitrary(g: &mut Gen) -> Self {
        CapabilityPublic_ {
            f: Arbitrary::arbitrary(g),
            r: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type CapabilityPublic = CapabilityPublic_;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum RtsOptsEnabledEnum {
    RtsOptsNone = 0,
    RtsOptsIgnore = 1,
    RtsOptsIgnoreAll = 2,
    RtsOptsSafeOnly = 3,
    RtsOptsAll = 4,
}

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

#[cfg(test)]
#[derive(Clone)]
struct RtsConfigOwned {
    pub rts_opts_enabled: RtsOptsEnabledEnum,
    pub rts_opts_suggestions: HsBool,
    pub rts_hs_main: HsBool,
    pub keep_cafs: HsBool,
    pub defaultsHook: Option<unsafe extern "C" fn()>,
    pub onExitHook: Option<unsafe extern "C" fn()>,
    pub stackOverflowHook: Option<unsafe extern "C" fn(stack_size: W_)>,
    pub outOfHeapHook: Option<unsafe extern "C" fn(request_size: W_, heap_size: W_)>,
    pub mallocFailHook: Option<unsafe extern "C" fn(request_size: W_, msg: *const c_char)>,
    pub gcDoneHook: Option<unsafe extern "C" fn(stats: *const GCDetails_)>,
    pub longGCSync: Option<unsafe extern "C" fn(this_cap: u32, time_ns: Time)>,
    pub longGCSyncEnd: Option<unsafe extern "C" fn(time_ns: Time)>,
}

#[cfg(test)]
impl Arbitrary for RtsConfigOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        RtsConfigOwned {
            rts_opts_enabled: Arbitrary::arbitrary(g),
            rts_opts_suggestions: Arbitrary::arbitrary(g),
            rts_hs_main: Arbitrary::arbitrary(g),
            keep_cafs: Arbitrary::arbitrary(g),
            defaultsHook: Arbitrary::arbitrary(g),
            onExitHook: Arbitrary::arbitrary(g),
            stackOverflowHook: Arbitrary::arbitrary(g),
            outOfHeapHook: Arbitrary::arbitrary(g),
            mallocFailHook: Arbitrary::arbitrary(g),
            gcDoneHook: Arbitrary::arbitrary(g),
            longGCSync: Arbitrary::arbitrary(g),
            longGCSyncEnd: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct RtsConfigPointees {
    pub rts_opts: c_char,
    pub eventlog_writer: EventLogWriter,
}

#[cfg(test)]
impl Arbitrary for RtsConfigPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        RtsConfigPointees {
            rts_opts: Arbitrary::arbitrary(g),
            eventlog_writer: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for RtsConfig {
    type Owned = RtsConfigOwned;
    type Pointees = RtsConfigPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            rts_opts_enabled: owned.rts_opts_enabled.clone(),
            rts_opts_suggestions: owned.rts_opts_suggestions,
            rts_hs_main: owned.rts_hs_main,
            keep_cafs: owned.keep_cafs,
            defaultsHook: owned.defaultsHook,
            onExitHook: owned.onExitHook,
            stackOverflowHook: owned.stackOverflowHook,
            outOfHeapHook: owned.outOfHeapHook,
            mallocFailHook: owned.mallocFailHook,
            gcDoneHook: owned.gcDoneHook,
            longGCSync: owned.longGCSync,
            longGCSyncEnd: owned.longGCSyncEnd,
            rts_opts: unsafe { &raw mut (*pointees).rts_opts },
            eventlog_writer: unsafe { &raw mut (*pointees).eventlog_writer },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            rts_opts_enabled: self.rts_opts_enabled.clone(),
            rts_opts_suggestions: self.rts_opts_suggestions,
            rts_hs_main: self.rts_hs_main,
            keep_cafs: self.keep_cafs,
            defaultsHook: self.defaultsHook,
            onExitHook: self.onExitHook,
            stackOverflowHook: self.stackOverflowHook,
            outOfHeapHook: self.outOfHeapHook,
            mallocFailHook: self.mallocFailHook,
            gcDoneHook: self.gcDoneHook,
            longGCSync: self.longGCSync,
            longGCSyncEnd: self.longGCSyncEnd,
        }
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_defaultRtsConfig"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static defaultRtsConfig: RtsConfig = 0;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct GCDetails_ {
    pub gen_: u32,
    pub threads: u32,
    pub allocated_bytes: u64,
    pub live_bytes: u64,
    pub large_objects_bytes: u64,
    pub compact_bytes: u64,
    pub slop_bytes: u64,
    pub mem_in_use_bytes: u64,
    pub copied_bytes: u64,
    pub block_fragmentation_bytes: u64,
    pub par_max_copied_bytes: u64,
    pub par_balanced_copied_bytes: u64,
    pub sync_elapsed_ns: Time,
    pub cpu_ns: Time,
    pub elapsed_ns: Time,
    pub nonmoving_gc_sync_cpu_ns: Time,
    pub nonmoving_gc_sync_elapsed_ns: Time,
    pub nonmoving_gc_cpu_ns: Time,
    pub nonmoving_gc_elapsed_ns: Time,
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

pub type GCDetails = GCDetails_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct _RTSStats {
    pub gcs: u32,
    pub major_gcs: u32,
    pub allocated_bytes: u64,
    pub max_live_bytes: u64,
    pub max_large_objects_bytes: u64,
    pub max_compact_bytes: u64,
    pub max_slop_bytes: u64,
    pub max_mem_in_use_bytes: u64,
    pub cumulative_live_bytes: u64,
    pub copied_bytes: u64,
    pub par_copied_bytes: u64,
    pub cumulative_par_max_copied_bytes: u64,
    pub cumulative_par_balanced_copied_bytes: u64,
    pub init_cpu_ns: Time,
    pub init_elapsed_ns: Time,
    pub mutator_cpu_ns: Time,
    pub mutator_elapsed_ns: Time,
    pub gc_cpu_ns: Time,
    pub gc_elapsed_ns: Time,
    pub cpu_ns: Time,
    pub elapsed_ns: Time,
    pub gc: GCDetails,
    pub any_work: u64,
    pub scav_find_work: u64,
    pub max_n_todo_overflow: u64,
    pub nonmoving_gc_sync_cpu_ns: Time,
    pub nonmoving_gc_sync_elapsed_ns: Time,
    pub nonmoving_gc_sync_max_elapsed_ns: Time,
    pub nonmoving_gc_cpu_ns: Time,
    pub nonmoving_gc_elapsed_ns: Time,
    pub nonmoving_gc_max_elapsed_ns: Time,
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
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getRTSStats(s: *mut RTSStats) {
    unsafe { sys::getRTSStats(s as *mut sys::RTSStats) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getRTSStatsEnabled"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    unsafe { sys::getRTSStatsEnabled() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getAllocations"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getAllocations() -> u64 {
    unsafe { transmute(sys::getAllocations()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn startupHaskell(
    argc: c_int,
    argv: *mut *mut c_char,
    init_root: Option<unsafe extern "C" fn()>,
) {
    unsafe { sys::startupHaskell(argc, argv, init_root) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn shutdownHaskell() {
    unsafe { sys::shutdownHaskell() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_init_with_rtsopts"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init_with_rtsopts(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::hs_init_with_rtsopts(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_init_ghc"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init_ghc(
    argc: *mut c_int,
    argv: *mut *mut *mut c_char,
    rts_config: RtsConfig,
) {
    unsafe { sys::hs_init_ghc(argc, argv, transmute(rts_config)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_shutdownHaskellAndExit"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn shutdownHaskellAndExit(exitCode: c_int, fastExit: c_int) -> ! {
    unsafe { sys::shutdownHaskellAndExit(exitCode, fastExit) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_shutdownHaskellAndSignal"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn shutdownHaskellAndSignal(sig: c_int, fastExit: c_int) -> ! {
    unsafe { sys::shutdownHaskellAndSignal(sig, fastExit) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::getProgArgv(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setProgArgv(argc: c_int, argv: *mut *mut c_char) {
    unsafe { sys::setProgArgv(argc, argv) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getFullProgArgv"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getFullProgArgv(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::getFullProgArgv(argc, argv) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setFullProgArgv(argc: c_int, argv: *mut *mut c_char) {
    unsafe { sys::setFullProgArgv(argc, argv) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeFullProgArgv() {
    unsafe { sys::freeFullProgArgv() }
}

static mut exitFn: Option<unsafe extern "C" fn(arg1: c_int)> = 0;
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_lock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    unsafe { transmute(sys::rts_lock()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_unlock"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_unlock(token: *mut Capability) {
    unsafe { sys::rts_unlock(token as *mut sys::Capability) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_unsafeGetMyCapability")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    unsafe { transmute(sys::rts_unsafeGetMyCapability()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_setInCallCapability"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) {
    unsafe { sys::rts_setInCallCapability(preferred_capability, affinity) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_pinThreadToNumaNode"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(node: c_int) {
    unsafe { sys::rts_pinThreadToNumaNode(node) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkChar"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkChar(arg1: *mut Capability, c: HsChar) -> HaskellObj {
    unsafe { transmute(sys::rts_mkChar(arg1 as *mut sys::Capability, c)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt(arg1: *mut Capability, i: HsInt) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt8(arg1: *mut Capability, i: HsInt8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt8(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt16(arg1: *mut Capability, i: HsInt16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt16(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt32(arg1: *mut Capability, i: HsInt32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt32(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkInt64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt64(arg1: *mut Capability, i: HsInt64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt64(arg1 as *mut sys::Capability, i)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord(arg1: *mut Capability, w: HsWord) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord8(arg1: *mut Capability, w: HsWord8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord8(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord16(arg1: *mut Capability, w: HsWord16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord16(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord32(arg1: *mut Capability, w: HsWord32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord32(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkWord64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord64(arg1: *mut Capability, w: HsWord64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord64(arg1 as *mut sys::Capability, w)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkPtr(arg1: *mut Capability, a: HsPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkPtr(arg1 as *mut sys::Capability, a)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkFunPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkFunPtr(arg1: *mut Capability, a: HsFunPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFunPtr(arg1 as *mut sys::Capability, a)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkFloat"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkFloat(arg1: *mut Capability, f: HsFloat) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFloat(arg1 as *mut sys::Capability, f)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkDouble(arg1: *mut Capability, f: HsDouble) -> HaskellObj {
    unsafe { transmute(sys::rts_mkDouble(arg1 as *mut sys::Capability, f)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkStablePtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkStablePtr(arg1: *mut Capability, s: HsStablePtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkStablePtr(arg1 as *mut sys::Capability, s)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkBool"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkBool(arg1: *mut Capability, b: HsBool) -> HaskellObj {
    unsafe { transmute(sys::rts_mkBool(arg1 as *mut sys::Capability, b)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_mkString"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkString(arg1: *mut Capability, s: *mut c_char) -> HaskellObj {
    unsafe { transmute(sys::rts_mkString(arg1 as *mut sys::Capability, s)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_apply"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getChar(arg1: HaskellObj) -> HsChar {
    unsafe { sys::rts_getChar(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt(arg1: HaskellObj) -> HsInt {
    unsafe { sys::rts_getInt(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt8(arg1: HaskellObj) -> HsInt8 {
    unsafe { sys::rts_getInt8(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt16(arg1: HaskellObj) -> HsInt16 {
    unsafe { sys::rts_getInt16(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt32(arg1: HaskellObj) -> HsInt32 {
    unsafe { sys::rts_getInt32(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getInt64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt64(arg1: HaskellObj) -> HsInt64 {
    unsafe { sys::rts_getInt64(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord(arg1: HaskellObj) -> HsWord {
    unsafe { sys::rts_getWord(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord8(arg1: HaskellObj) -> HsWord8 {
    unsafe { sys::rts_getWord8(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord16(arg1: HaskellObj) -> HsWord16 {
    unsafe { sys::rts_getWord16(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord32(arg1: HaskellObj) -> HsWord32 {
    unsafe { sys::rts_getWord32(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getWord64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord64(arg1: HaskellObj) -> HsWord64 {
    unsafe { sys::rts_getWord64(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getPtr(arg1: HaskellObj) -> HsPtr {
    unsafe { sys::rts_getPtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getFunPtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getFunPtr(arg1: HaskellObj) -> HsFunPtr {
    unsafe { sys::rts_getFunPtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getFloat"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getFloat(arg1: HaskellObj) -> HsFloat {
    unsafe { sys::rts_getFloat(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getDouble(arg1: HaskellObj) -> HsDouble {
    unsafe { sys::rts_getDouble(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getStablePtr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getStablePtr(arg1: HaskellObj) -> HsStablePtr {
    unsafe { sys::rts_getStablePtr(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getBool"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getBool(arg1: HaskellObj) -> HsBool {
    unsafe { sys::rts_getBool(transmute(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_eval"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalStableIOMain(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe { sys::rts_evalStableIOMain(arg1 as *mut *mut sys::Capability, s, ret) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalStableIO"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalStableIO(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe { sys::rts_evalStableIO(arg1 as *mut *mut sys::Capability, s, ret) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_evalLazyIO"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
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
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_checkSchedStatus(site: *mut c_char, arg1: *mut Capability) {
    unsafe { sys::rts_checkSchedStatus(site, arg1 as *mut sys::Capability) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getSchedStatus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getSchedStatus(cap: *mut Capability) -> SchedulerStatus {
    unsafe { transmute(sys::rts_getSchedStatus(cap as *mut sys::Capability)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_pause"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    unsafe { transmute(sys::rts_pause()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_resume"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_resume(pauseToken: *mut PauseToken) {
    unsafe { sys::rts_resume(pauseToken as *mut sys::PauseToken) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_isPaused"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    unsafe { transmute(sys::rts_isPaused()) }
}

pub(crate) type ListThreadsCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgTSO)>;
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_listThreads"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_listThreads(cb: ListThreadsCb, user: *mut c_void) {
    unsafe { sys::rts_listThreads(transmute(cb), user) }
}

pub(crate) type ListRootsCb =
    Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut StgClosure)>;
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_listMiscRoots"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_listMiscRoots(cb: ListRootsCb, user: *mut c_void) {
    unsafe { sys::rts_listMiscRoots(transmute(cb), user) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rts_done() {
    unsafe { sys::rts_done() }
}

static mut ghczminternal_GHCziInternalziTopHandler_runIO_closure: [StgWord; 0usize] = [];

static mut ghczminternal_GHCziInternalziTopHandler_runNonIO_closure: [StgWord; 0usize] = [];

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_clearMemory"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_clearMemory() {
    unsafe { sys::rts_clearMemory() }
}
