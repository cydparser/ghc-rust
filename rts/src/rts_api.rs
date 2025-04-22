use crate::hs_ffi;
use crate::rts::event_log_writer;
use crate::rts::time;
use crate::rts::types;
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn pauseTokenCapability(pauseToken: *mut PauseToken) -> *mut Capability {
    unsafe { transmute(sys::pauseTokenCapability(&mut pauseToken.into())) }
}

#[repr(C)]
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
    pub rts_opts: *const ::core::ffi::c_char,
    pub rts_hs_main: HsBool,
    pub keep_cafs: HsBool,
    pub eventlog_writer: *const EventLogWriter,
    pub defaultsHook: ::core::option::Option<unsafe extern "C" fn()>,
    pub onExitHook: ::core::option::Option<unsafe extern "C" fn()>,
    pub stackOverflowHook: ::core::option::Option<unsafe extern "C" fn(stack_size: W_)>,
    pub outOfHeapHook:
        ::core::option::Option<unsafe extern "C" fn(request_size: W_, heap_size: W_)>,
    pub mallocFailHook: ::core::option::Option<
        unsafe extern "C" fn(request_size: W_, msg: *const ::core::ffi::c_char),
    >,
    pub gcDoneHook: ::core::option::Option<unsafe extern "C" fn(stats: *const GCDetails_)>,
    pub longGCSync: ::core::option::Option<unsafe extern "C" fn(this_cap: u32, time_ns: Time)>,
    pub longGCSyncEnd: ::core::option::Option<unsafe extern "C" fn(time_ns: Time)>,
}

#[cfg(feature = "sys")]
impl From<RtsConfig> for sys::RtsConfig {
    fn from(x: RtsConfig) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for RtsConfig {
    fn arbitrary(g: &mut Gen) -> Self {
        RtsConfig {
            rts_opts_enabled: Arbitrary::arbitrary(g),
            rts_opts_suggestions: Arbitrary::arbitrary(g),
            rts_opts: Arbitrary::arbitrary(g),
            rts_hs_main: Arbitrary::arbitrary(g),
            keep_cafs: Arbitrary::arbitrary(g),
            eventlog_writer: Arbitrary::arbitrary(g),
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

#[unsafe(no_mangle)]
pub static defaultRtsConfig: RtsConfig = sys::defaultRtsConfig;

#[repr(C)]
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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getRTSStats(s: *mut RTSStats) {
    unsafe { transmute(sys::getRTSStats(&mut s.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getRTSStatsEnabled() -> ::core::ffi::c_int {
    unsafe { transmute(sys::getRTSStatsEnabled()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getAllocations() -> u64 {
    unsafe { transmute(sys::getAllocations()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn startupHaskell(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
    init_root: ::core::option::Option<unsafe extern "C" fn()>,
) {
    unsafe {
        transmute(sys::startupHaskell(
            argc.into(),
            &mut &mut argv.into(),
            init_root.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn shutdownHaskell() {
    unsafe { transmute(sys::shutdownHaskell()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init_with_rtsopts(
    argc: *mut ::core::ffi::c_int,
    argv: *mut *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        transmute(sys::hs_init_with_rtsopts(
            &mut argc.into(),
            &mut &mut &mut argv.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init_ghc(
    argc: *mut ::core::ffi::c_int,
    argv: *mut *mut *mut ::core::ffi::c_char,
    rts_config: RtsConfig,
) {
    unsafe {
        transmute(sys::hs_init_ghc(
            &mut argc.into(),
            &mut &mut &mut argv.into(),
            rts_config.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn shutdownHaskellAndExit(
    exitCode: ::core::ffi::c_int,
    fastExit: ::core::ffi::c_int,
) -> ! {
    unsafe {
        transmute(sys::shutdownHaskellAndExit(
            exitCode.into(),
            fastExit.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn shutdownHaskellAndSignal(
    sig: ::core::ffi::c_int,
    fastExit: ::core::ffi::c_int,
) -> ! {
    unsafe { transmute(sys::shutdownHaskellAndSignal(sig.into(), fastExit.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getProgArgv(
    argc: *mut ::core::ffi::c_int,
    argv: *mut *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        transmute(sys::getProgArgv(
            &mut argc.into(),
            &mut &mut &mut argv.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setProgArgv(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
) {
    unsafe { transmute(sys::setProgArgv(argc.into(), &mut &mut argv.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getFullProgArgv(
    argc: *mut ::core::ffi::c_int,
    argv: *mut *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        transmute(sys::getFullProgArgv(
            &mut argc.into(),
            &mut &mut &mut argv.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setFullProgArgv(
    argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char,
) {
    unsafe { transmute(sys::setFullProgArgv(argc.into(), &mut &mut argv.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeFullProgArgv() {
    unsafe { transmute(sys::freeFullProgArgv()) }
}

static mut exitFn: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)> =
    sys::exitFn;
#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    unsafe { transmute(sys::rts_lock()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_unlock(token: *mut Capability) {
    unsafe { transmute(sys::rts_unlock(&mut token.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    unsafe { transmute(sys::rts_unsafeGetMyCapability()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_setInCallCapability(
    preferred_capability: ::core::ffi::c_int,
    affinity: ::core::ffi::c_int,
) {
    unsafe {
        transmute(sys::rts_setInCallCapability(
            preferred_capability.into(),
            affinity.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_pinThreadToNumaNode(node: ::core::ffi::c_int) {
    unsafe { transmute(sys::rts_pinThreadToNumaNode(node.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkChar(arg1: *mut Capability, c: HsChar) -> HaskellObj {
    unsafe { transmute(sys::rts_mkChar(&mut arg1.into(), c.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt(arg1: *mut Capability, i: HsInt) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt(&mut arg1.into(), i.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt8(arg1: *mut Capability, i: HsInt8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt8(&mut arg1.into(), i.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt16(arg1: *mut Capability, i: HsInt16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt16(&mut arg1.into(), i.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt32(arg1: *mut Capability, i: HsInt32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt32(&mut arg1.into(), i.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkInt64(arg1: *mut Capability, i: HsInt64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkInt64(&mut arg1.into(), i.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord(arg1: *mut Capability, w: HsWord) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord(&mut arg1.into(), w.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord8(arg1: *mut Capability, w: HsWord8) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord8(&mut arg1.into(), w.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord16(arg1: *mut Capability, w: HsWord16) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord16(&mut arg1.into(), w.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord32(arg1: *mut Capability, w: HsWord32) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord32(&mut arg1.into(), w.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkWord64(arg1: *mut Capability, w: HsWord64) -> HaskellObj {
    unsafe { transmute(sys::rts_mkWord64(&mut arg1.into(), w.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkPtr(arg1: *mut Capability, a: HsPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkPtr(&mut arg1.into(), a.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkFunPtr(arg1: *mut Capability, a: HsFunPtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFunPtr(&mut arg1.into(), a.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkFloat(arg1: *mut Capability, f: HsFloat) -> HaskellObj {
    unsafe { transmute(sys::rts_mkFloat(&mut arg1.into(), f.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkDouble(arg1: *mut Capability, f: HsDouble) -> HaskellObj {
    unsafe { transmute(sys::rts_mkDouble(&mut arg1.into(), f.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkStablePtr(arg1: *mut Capability, s: HsStablePtr) -> HaskellObj {
    unsafe { transmute(sys::rts_mkStablePtr(&mut arg1.into(), s.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkBool(arg1: *mut Capability, b: HsBool) -> HaskellObj {
    unsafe { transmute(sys::rts_mkBool(&mut arg1.into(), b.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_mkString(
    arg1: *mut Capability,
    s: *mut ::core::ffi::c_char,
) -> HaskellObj {
    unsafe { transmute(sys::rts_mkString(&mut arg1.into(), &mut s.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_apply(
    arg1: *mut Capability,
    arg2: HaskellObj,
    arg3: HaskellObj,
) -> HaskellObj {
    unsafe { transmute(sys::rts_apply(&mut arg1.into(), arg2.into(), arg3.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getChar(arg1: HaskellObj) -> HsChar {
    unsafe { transmute(sys::rts_getChar(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt(arg1: HaskellObj) -> HsInt {
    unsafe { transmute(sys::rts_getInt(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt8(arg1: HaskellObj) -> HsInt8 {
    unsafe { transmute(sys::rts_getInt8(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt16(arg1: HaskellObj) -> HsInt16 {
    unsafe { transmute(sys::rts_getInt16(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt32(arg1: HaskellObj) -> HsInt32 {
    unsafe { transmute(sys::rts_getInt32(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getInt64(arg1: HaskellObj) -> HsInt64 {
    unsafe { transmute(sys::rts_getInt64(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord(arg1: HaskellObj) -> HsWord {
    unsafe { transmute(sys::rts_getWord(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord8(arg1: HaskellObj) -> HsWord8 {
    unsafe { transmute(sys::rts_getWord8(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord16(arg1: HaskellObj) -> HsWord16 {
    unsafe { transmute(sys::rts_getWord16(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord32(arg1: HaskellObj) -> HsWord32 {
    unsafe { transmute(sys::rts_getWord32(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getWord64(arg1: HaskellObj) -> HsWord64 {
    unsafe { transmute(sys::rts_getWord64(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getPtr(arg1: HaskellObj) -> HsPtr {
    unsafe { transmute(sys::rts_getPtr(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getFunPtr(arg1: HaskellObj) -> HsFunPtr {
    unsafe { transmute(sys::rts_getFunPtr(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getFloat(arg1: HaskellObj) -> HsFloat {
    unsafe { transmute(sys::rts_getFloat(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getDouble(arg1: HaskellObj) -> HsDouble {
    unsafe { transmute(sys::rts_getDouble(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getStablePtr(arg1: HaskellObj) -> HsStablePtr {
    unsafe { transmute(sys::rts_getStablePtr(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getBool(arg1: HaskellObj) -> HsBool {
    unsafe { transmute(sys::rts_getBool(arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_eval(arg1: *mut *mut Capability, p: HaskellObj, ret: *mut HaskellObj) {
    unsafe {
        transmute(sys::rts_eval(
            &mut &mut arg1.into(),
            p.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_eval_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: ::core::ffi::c_uint,
    ret: *mut HaskellObj,
) {
    unsafe {
        transmute(sys::rts_eval_(
            &mut &mut arg1.into(),
            p.into(),
            stack_size.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        transmute(sys::rts_evalIO(
            &mut &mut arg1.into(),
            p.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalStableIOMain(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe {
        transmute(sys::rts_evalStableIOMain(
            &mut &mut arg1.into(),
            s.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalStableIO(
    arg1: *mut *mut Capability,
    s: HsStablePtr,
    ret: *mut HsStablePtr,
) {
    unsafe {
        transmute(sys::rts_evalStableIO(
            &mut &mut arg1.into(),
            s.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalLazyIO(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        transmute(sys::rts_evalLazyIO(
            &mut &mut arg1.into(),
            p.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_evalLazyIO_(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    stack_size: ::core::ffi::c_uint,
    ret: *mut HaskellObj,
) {
    unsafe {
        transmute(sys::rts_evalLazyIO_(
            &mut &mut arg1.into(),
            p.into(),
            stack_size.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_inCall(
    arg1: *mut *mut Capability,
    p: HaskellObj,
    ret: *mut HaskellObj,
) {
    unsafe {
        transmute(sys::rts_inCall(
            &mut &mut arg1.into(),
            p.into(),
            &mut ret.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_checkSchedStatus(
    site: *mut ::core::ffi::c_char,
    arg1: *mut Capability,
) {
    unsafe {
        transmute(sys::rts_checkSchedStatus(
            &mut site.into(),
            &mut arg1.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getSchedStatus(cap: *mut Capability) -> SchedulerStatus {
    unsafe { transmute(sys::rts_getSchedStatus(&mut cap.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    unsafe { transmute(sys::rts_pause()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_resume(pauseToken: *mut PauseToken) {
    unsafe { transmute(sys::rts_resume(&mut pauseToken.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    unsafe { transmute(sys::rts_isPaused()) }
}

pub(crate) type ListThreadsCb =
    ::core::option::Option<unsafe extern "C" fn(user: *mut ::core::ffi::c_void, arg1: *mut StgTSO)>;
#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_listThreads(cb: ListThreadsCb, user: *mut ::core::ffi::c_void) {
    unsafe { transmute(sys::rts_listThreads(cb.into(), &mut user.into())) }
}

pub(crate) type ListRootsCb = ::core::option::Option<
    unsafe extern "C" fn(user: *mut ::core::ffi::c_void, arg1: *mut StgClosure),
>;
#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_listMiscRoots(cb: ListRootsCb, user: *mut ::core::ffi::c_void) {
    unsafe { transmute(sys::rts_listMiscRoots(cb.into(), &mut user.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rts_done() {
    unsafe { transmute(sys::rts_done()) }
}

static mut ghczminternal_GHCziInternalziTopHandler_runIO_closure: [StgWord; 0usize] =
    sys::ghczminternal_GHCziInternalziTopHandler_runIO_closure;

static mut ghczminternal_GHCziInternalziTopHandler_runNonIO_closure: [StgWord; 0usize] =
    sys::ghczminternal_GHCziInternalziTopHandler_runNonIO_closure;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_clearMemory() {
    unsafe { transmute(sys::rts_clearMemory()) }
}
