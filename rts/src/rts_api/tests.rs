use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[test]
fn sys_size_PauseToken_() {
    assert_eq!(size_of::<sys::PauseToken_>(), size_of::<PauseToken_>())
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_pauseTokenCapability(pauseToken: PauseToken) -> bool {
    let expected = unsafe { transmute(sys::pauseTokenCapability(&mut pauseToken.into())) };
    let actual = unsafe { pauseTokenCapability(&mut pauseToken) };
    actual == expected
}

#[test]
#[ignore]
fn test_pauseTokenCapability() {
    let mut pauseToken = null_mut();
    unsafe { pauseTokenCapability(&mut pauseToken) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_CapabilityPublic_() {
    assert_eq!(
        size_of::<sys::CapabilityPublic_>(),
        size_of::<CapabilityPublic_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CapabilityPublic_"][size_of::<CapabilityPublic_>() - 944usize];
    ["Alignment of CapabilityPublic_"][align_of::<CapabilityPublic_>() - 8usize];
    ["Offset of field: CapabilityPublic_::f"][offset_of!(CapabilityPublic_, f) - 0usize];
    ["Offset of field: CapabilityPublic_::r"][offset_of!(CapabilityPublic_, r) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_RtsConfig() {
    assert_eq!(size_of::<sys::RtsConfig>(), size_of::<RtsConfig>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RtsConfig"][size_of::<RtsConfig>() - 112usize];
    ["Alignment of RtsConfig"][align_of::<RtsConfig>() - 8usize];
    ["Offset of field: RtsConfig::rts_opts_enabled"]
        [offset_of!(RtsConfig, rts_opts_enabled) - 0usize];
    ["Offset of field: RtsConfig::rts_opts_suggestions"]
        [offset_of!(RtsConfig, rts_opts_suggestions) - 8usize];
    ["Offset of field: RtsConfig::rts_opts"][offset_of!(RtsConfig, rts_opts) - 16usize];
    ["Offset of field: RtsConfig::rts_hs_main"][offset_of!(RtsConfig, rts_hs_main) - 24usize];
    ["Offset of field: RtsConfig::keep_cafs"][offset_of!(RtsConfig, keep_cafs) - 32usize];
    ["Offset of field: RtsConfig::eventlog_writer"]
        [offset_of!(RtsConfig, eventlog_writer) - 40usize];
    ["Offset of field: RtsConfig::defaultsHook"][offset_of!(RtsConfig, defaultsHook) - 48usize];
    ["Offset of field: RtsConfig::onExitHook"][offset_of!(RtsConfig, onExitHook) - 56usize];
    ["Offset of field: RtsConfig::stackOverflowHook"]
        [offset_of!(RtsConfig, stackOverflowHook) - 64usize];
    ["Offset of field: RtsConfig::outOfHeapHook"][offset_of!(RtsConfig, outOfHeapHook) - 72usize];
    ["Offset of field: RtsConfig::mallocFailHook"][offset_of!(RtsConfig, mallocFailHook) - 80usize];
    ["Offset of field: RtsConfig::gcDoneHook"][offset_of!(RtsConfig, gcDoneHook) - 88usize];
    ["Offset of field: RtsConfig::longGCSync"][offset_of!(RtsConfig, longGCSync) - 96usize];
    ["Offset of field: RtsConfig::longGCSyncEnd"][offset_of!(RtsConfig, longGCSyncEnd) - 104usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_GCDetails_() {
    assert_eq!(size_of::<sys::GCDetails_>(), size_of::<GCDetails_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GCDetails_"][size_of::<GCDetails_>() - 144usize];
    ["Alignment of GCDetails_"][align_of::<GCDetails_>() - 8usize];
    ["Offset of field: GCDetails_::gen_"][offset_of!(GCDetails_, gen_) - 0usize];
    ["Offset of field: GCDetails_::threads"][offset_of!(GCDetails_, threads) - 4usize];
    ["Offset of field: GCDetails_::allocated_bytes"]
        [offset_of!(GCDetails_, allocated_bytes) - 8usize];
    ["Offset of field: GCDetails_::live_bytes"][offset_of!(GCDetails_, live_bytes) - 16usize];
    ["Offset of field: GCDetails_::large_objects_bytes"]
        [offset_of!(GCDetails_, large_objects_bytes) - 24usize];
    ["Offset of field: GCDetails_::compact_bytes"][offset_of!(GCDetails_, compact_bytes) - 32usize];
    ["Offset of field: GCDetails_::slop_bytes"][offset_of!(GCDetails_, slop_bytes) - 40usize];
    ["Offset of field: GCDetails_::mem_in_use_bytes"]
        [offset_of!(GCDetails_, mem_in_use_bytes) - 48usize];
    ["Offset of field: GCDetails_::copied_bytes"][offset_of!(GCDetails_, copied_bytes) - 56usize];
    ["Offset of field: GCDetails_::block_fragmentation_bytes"]
        [offset_of!(GCDetails_, block_fragmentation_bytes) - 64usize];
    ["Offset of field: GCDetails_::par_max_copied_bytes"]
        [offset_of!(GCDetails_, par_max_copied_bytes) - 72usize];
    ["Offset of field: GCDetails_::par_balanced_copied_bytes"]
        [offset_of!(GCDetails_, par_balanced_copied_bytes) - 80usize];
    ["Offset of field: GCDetails_::sync_elapsed_ns"]
        [offset_of!(GCDetails_, sync_elapsed_ns) - 88usize];
    ["Offset of field: GCDetails_::cpu_ns"][offset_of!(GCDetails_, cpu_ns) - 96usize];
    ["Offset of field: GCDetails_::elapsed_ns"][offset_of!(GCDetails_, elapsed_ns) - 104usize];
    ["Offset of field: GCDetails_::nonmoving_gc_sync_cpu_ns"]
        [offset_of!(GCDetails_, nonmoving_gc_sync_cpu_ns) - 112usize];
    ["Offset of field: GCDetails_::nonmoving_gc_sync_elapsed_ns"]
        [offset_of!(GCDetails_, nonmoving_gc_sync_elapsed_ns) - 120usize];
    ["Offset of field: GCDetails_::nonmoving_gc_cpu_ns"]
        [offset_of!(GCDetails_, nonmoving_gc_cpu_ns) - 128usize];
    ["Offset of field: GCDetails_::nonmoving_gc_elapsed_ns"]
        [offset_of!(GCDetails_, nonmoving_gc_elapsed_ns) - 136usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__RTSStats() {
    assert_eq!(size_of::<sys::_RTSStats>(), size_of::<_RTSStats>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _RTSStats"][size_of::<_RTSStats>() - 376usize];
    ["Alignment of _RTSStats"][align_of::<_RTSStats>() - 8usize];
    ["Offset of field: _RTSStats::gcs"][offset_of!(_RTSStats, gcs) - 0usize];
    ["Offset of field: _RTSStats::major_gcs"][offset_of!(_RTSStats, major_gcs) - 4usize];
    ["Offset of field: _RTSStats::allocated_bytes"]
        [offset_of!(_RTSStats, allocated_bytes) - 8usize];
    ["Offset of field: _RTSStats::max_live_bytes"][offset_of!(_RTSStats, max_live_bytes) - 16usize];
    ["Offset of field: _RTSStats::max_large_objects_bytes"]
        [offset_of!(_RTSStats, max_large_objects_bytes) - 24usize];
    ["Offset of field: _RTSStats::max_compact_bytes"]
        [offset_of!(_RTSStats, max_compact_bytes) - 32usize];
    ["Offset of field: _RTSStats::max_slop_bytes"][offset_of!(_RTSStats, max_slop_bytes) - 40usize];
    ["Offset of field: _RTSStats::max_mem_in_use_bytes"]
        [offset_of!(_RTSStats, max_mem_in_use_bytes) - 48usize];
    ["Offset of field: _RTSStats::cumulative_live_bytes"]
        [offset_of!(_RTSStats, cumulative_live_bytes) - 56usize];
    ["Offset of field: _RTSStats::copied_bytes"][offset_of!(_RTSStats, copied_bytes) - 64usize];
    ["Offset of field: _RTSStats::par_copied_bytes"]
        [offset_of!(_RTSStats, par_copied_bytes) - 72usize];
    ["Offset of field: _RTSStats::cumulative_par_max_copied_bytes"]
        [offset_of!(_RTSStats, cumulative_par_max_copied_bytes) - 80usize];
    ["Offset of field: _RTSStats::cumulative_par_balanced_copied_bytes"]
        [offset_of!(_RTSStats, cumulative_par_balanced_copied_bytes) - 88usize];
    ["Offset of field: _RTSStats::init_cpu_ns"][offset_of!(_RTSStats, init_cpu_ns) - 96usize];
    ["Offset of field: _RTSStats::init_elapsed_ns"]
        [offset_of!(_RTSStats, init_elapsed_ns) - 104usize];
    ["Offset of field: _RTSStats::mutator_cpu_ns"]
        [offset_of!(_RTSStats, mutator_cpu_ns) - 112usize];
    ["Offset of field: _RTSStats::mutator_elapsed_ns"]
        [offset_of!(_RTSStats, mutator_elapsed_ns) - 120usize];
    ["Offset of field: _RTSStats::gc_cpu_ns"][offset_of!(_RTSStats, gc_cpu_ns) - 128usize];
    ["Offset of field: _RTSStats::gc_elapsed_ns"][offset_of!(_RTSStats, gc_elapsed_ns) - 136usize];
    ["Offset of field: _RTSStats::cpu_ns"][offset_of!(_RTSStats, cpu_ns) - 144usize];
    ["Offset of field: _RTSStats::elapsed_ns"][offset_of!(_RTSStats, elapsed_ns) - 152usize];
    ["Offset of field: _RTSStats::gc"][offset_of!(_RTSStats, gc) - 160usize];
    ["Offset of field: _RTSStats::any_work"][offset_of!(_RTSStats, any_work) - 304usize];
    ["Offset of field: _RTSStats::scav_find_work"]
        [offset_of!(_RTSStats, scav_find_work) - 312usize];
    ["Offset of field: _RTSStats::max_n_todo_overflow"]
        [offset_of!(_RTSStats, max_n_todo_overflow) - 320usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_cpu_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_sync_cpu_ns) - 328usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_elapsed_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_sync_elapsed_ns) - 336usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_max_elapsed_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_sync_max_elapsed_ns) - 344usize];
    ["Offset of field: _RTSStats::nonmoving_gc_cpu_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_cpu_ns) - 352usize];
    ["Offset of field: _RTSStats::nonmoving_gc_elapsed_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_elapsed_ns) - 360usize];
    ["Offset of field: _RTSStats::nonmoving_gc_max_elapsed_ns"]
        [offset_of!(_RTSStats, nonmoving_gc_max_elapsed_ns) - 368usize];
};

#[test]
#[ignore]
fn test_getRTSStats() {
    let mut s = null_mut();
    unsafe { getRTSStats(&mut s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getRTSStatsEnabled() -> bool {
    let expected = unsafe { sys::getRTSStatsEnabled() };
    let actual = unsafe { getRTSStatsEnabled() };
    actual == expected
}

#[test]
#[ignore]
fn test_getRTSStatsEnabled() {
    unsafe { getRTSStatsEnabled() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getAllocations() -> bool {
    let expected = unsafe { transmute(sys::getAllocations()) };
    let actual = unsafe { getAllocations() };
    actual == expected
}

#[test]
#[ignore]
fn test_getAllocations() {
    unsafe { getAllocations() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startupHaskell() {
    let argc = Default::default();
    let mut argv = null_mut();
    let init_root = Default::default();
    unsafe { startupHaskell(argc, &mut &mut argv, init_root) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_shutdownHaskell() {
    unsafe { shutdownHaskell() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_init_with_rtsopts() {
    let mut argc = null_mut();
    let mut argv = null_mut();
    unsafe { hs_init_with_rtsopts(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_init_ghc() {
    let mut argc = null_mut();
    let mut argv = null_mut();
    let rts_config = todo!();
    unsafe { hs_init_ghc(&mut argc, &mut &mut &mut argv, rts_config) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownHaskellAndExit(exitCode: c_int, fastExit: c_int) -> bool {
    let expected = unsafe { sys::shutdownHaskellAndExit(exitCode, fastExit) };
    let actual = unsafe { shutdownHaskellAndExit(exitCode, fastExit) };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownHaskellAndExit() {
    let exitCode = Default::default();
    let fastExit = Default::default();
    unsafe { shutdownHaskellAndExit(exitCode, fastExit) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownHaskellAndSignal(sig: c_int, fastExit: c_int) -> bool {
    let expected = unsafe { sys::shutdownHaskellAndSignal(sig, fastExit) };
    let actual = unsafe { shutdownHaskellAndSignal(sig, fastExit) };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownHaskellAndSignal() {
    let sig = Default::default();
    let fastExit = Default::default();
    unsafe { shutdownHaskellAndSignal(sig, fastExit) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getProgArgv() {
    let mut argc = null_mut();
    let mut argv = null_mut();
    unsafe { getProgArgv(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setProgArgv() {
    let argc = Default::default();
    let mut argv = null_mut();
    unsafe { setProgArgv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getFullProgArgv() {
    let mut argc = null_mut();
    let mut argv = null_mut();
    unsafe { getFullProgArgv(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setFullProgArgv() {
    let argc = Default::default();
    let mut argv = null_mut();
    unsafe { setFullProgArgv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeFullProgArgv() {
    unsafe { freeFullProgArgv() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_lock() -> bool {
    let expected = unsafe { transmute(sys::rts_lock()) };
    let actual = unsafe { rts_lock() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_lock() {
    unsafe { rts_lock() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_unlock() {
    let mut token = null_mut();
    unsafe { rts_unlock(&mut token) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_unsafeGetMyCapability() -> bool {
    let expected = unsafe { transmute(sys::rts_unsafeGetMyCapability()) };
    let actual = unsafe { rts_unsafeGetMyCapability() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_unsafeGetMyCapability() {
    unsafe { rts_unsafeGetMyCapability() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_setInCallCapability() {
    let preferred_capability = Default::default();
    let affinity = Default::default();
    unsafe { rts_setInCallCapability(preferred_capability, affinity) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_pinThreadToNumaNode() {
    let node = Default::default();
    unsafe { rts_pinThreadToNumaNode(node) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkChar(arg1: Capability, c: HsChar) -> bool {
    let expected = unsafe { transmute(sys::rts_mkChar(&mut arg1.into(), c)) };
    let actual = unsafe { rts_mkChar(&mut arg1, c) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkChar() {
    let mut arg1 = null_mut();
    let c = Default::default();
    unsafe { rts_mkChar(&mut arg1, c) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt(arg1: Capability, i: HsInt) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt(&mut arg1.into(), i)) };
    let actual = unsafe { rts_mkInt(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt() {
    let mut arg1 = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt8(arg1: Capability, i: HsInt8) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt8(&mut arg1.into(), i)) };
    let actual = unsafe { rts_mkInt8(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt8() {
    let mut arg1 = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt8(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt16(arg1: Capability, i: HsInt16) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt16(&mut arg1.into(), i)) };
    let actual = unsafe { rts_mkInt16(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt16() {
    let mut arg1 = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt16(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt32(arg1: Capability, i: HsInt32) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt32(&mut arg1.into(), i)) };
    let actual = unsafe { rts_mkInt32(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt32() {
    let mut arg1 = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt32(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt64(arg1: Capability, i: HsInt64) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt64(&mut arg1.into(), i)) };
    let actual = unsafe { rts_mkInt64(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt64() {
    let mut arg1 = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt64(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord(arg1: Capability, w: HsWord) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord(&mut arg1.into(), w)) };
    let actual = unsafe { rts_mkWord(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord() {
    let mut arg1 = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord8(arg1: Capability, w: HsWord8) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord8(&mut arg1.into(), w)) };
    let actual = unsafe { rts_mkWord8(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord8() {
    let mut arg1 = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord8(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord16(arg1: Capability, w: HsWord16) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord16(&mut arg1.into(), w)) };
    let actual = unsafe { rts_mkWord16(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord16() {
    let mut arg1 = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord16(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord32(arg1: Capability, w: HsWord32) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord32(&mut arg1.into(), w)) };
    let actual = unsafe { rts_mkWord32(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord32() {
    let mut arg1 = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord32(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord64(arg1: Capability, w: HsWord64) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord64(&mut arg1.into(), w)) };
    let actual = unsafe { rts_mkWord64(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord64() {
    let mut arg1 = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord64(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkPtr(arg1: Capability, a: HsPtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkPtr(&mut arg1.into(), a)) };
    let actual = unsafe { rts_mkPtr(&mut arg1, a) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkPtr() {
    let mut arg1 = null_mut();
    let a = Default::default();
    unsafe { rts_mkPtr(&mut arg1, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkFunPtr(arg1: Capability, a: HsFunPtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkFunPtr(&mut arg1.into(), a)) };
    let actual = unsafe { rts_mkFunPtr(&mut arg1, a) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkFunPtr() {
    let mut arg1 = null_mut();
    let a = Default::default();
    unsafe { rts_mkFunPtr(&mut arg1, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkFloat(arg1: Capability, f: HsFloat) -> bool {
    let expected = unsafe { transmute(sys::rts_mkFloat(&mut arg1.into(), f)) };
    let actual = unsafe { rts_mkFloat(&mut arg1, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkFloat() {
    let mut arg1 = null_mut();
    let f = Default::default();
    unsafe { rts_mkFloat(&mut arg1, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkDouble(arg1: Capability, f: HsDouble) -> bool {
    let expected = unsafe { transmute(sys::rts_mkDouble(&mut arg1.into(), f)) };
    let actual = unsafe { rts_mkDouble(&mut arg1, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkDouble() {
    let mut arg1 = null_mut();
    let f = Default::default();
    unsafe { rts_mkDouble(&mut arg1, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkStablePtr(arg1: Capability, s: HsStablePtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkStablePtr(&mut arg1.into(), s)) };
    let actual = unsafe { rts_mkStablePtr(&mut arg1, s) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkStablePtr() {
    let mut arg1 = null_mut();
    let s = Default::default();
    unsafe { rts_mkStablePtr(&mut arg1, s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkBool(arg1: Capability, b: HsBool) -> bool {
    let expected = unsafe { transmute(sys::rts_mkBool(&mut arg1.into(), b)) };
    let actual = unsafe { rts_mkBool(&mut arg1, b) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkBool() {
    let mut arg1 = null_mut();
    let b = Default::default();
    unsafe { rts_mkBool(&mut arg1, b) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkString(arg1: Capability, s: c_char) -> bool {
    let expected = unsafe { transmute(sys::rts_mkString(&mut arg1.into(), &mut s)) };
    let actual = unsafe { rts_mkString(&mut arg1, &mut s) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkString() {
    let mut arg1 = null_mut();
    let mut s = null_mut();
    unsafe { rts_mkString(&mut arg1, &mut s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_apply(arg1: Capability, arg2: HaskellObj, arg3: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_apply(&mut arg1.into(), arg2.into(), arg3.into())) };
    let actual = unsafe { rts_apply(&mut arg1, arg2, arg3) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_apply() {
    let mut arg1 = null_mut();
    let arg2 = todo!();
    let arg3 = todo!();
    unsafe { rts_apply(&mut arg1, arg2, arg3) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getChar(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getChar(arg1.into()) };
    let actual = unsafe { rts_getChar(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getChar() {
    let arg1 = todo!();
    unsafe { rts_getChar(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getInt(arg1.into()) };
    let actual = unsafe { rts_getInt(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt() {
    let arg1 = todo!();
    unsafe { rts_getInt(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt8(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getInt8(arg1.into()) };
    let actual = unsafe { rts_getInt8(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt8() {
    let arg1 = todo!();
    unsafe { rts_getInt8(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt16(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getInt16(arg1.into()) };
    let actual = unsafe { rts_getInt16(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt16() {
    let arg1 = todo!();
    unsafe { rts_getInt16(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt32(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getInt32(arg1.into()) };
    let actual = unsafe { rts_getInt32(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt32() {
    let arg1 = todo!();
    unsafe { rts_getInt32(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt64(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getInt64(arg1.into()) };
    let actual = unsafe { rts_getInt64(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt64() {
    let arg1 = todo!();
    unsafe { rts_getInt64(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getWord(arg1.into()) };
    let actual = unsafe { rts_getWord(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord() {
    let arg1 = todo!();
    unsafe { rts_getWord(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord8(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getWord8(arg1.into()) };
    let actual = unsafe { rts_getWord8(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord8() {
    let arg1 = todo!();
    unsafe { rts_getWord8(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord16(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getWord16(arg1.into()) };
    let actual = unsafe { rts_getWord16(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord16() {
    let arg1 = todo!();
    unsafe { rts_getWord16(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord32(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getWord32(arg1.into()) };
    let actual = unsafe { rts_getWord32(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord32() {
    let arg1 = todo!();
    unsafe { rts_getWord32(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord64(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getWord64(arg1.into()) };
    let actual = unsafe { rts_getWord64(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord64() {
    let arg1 = todo!();
    unsafe { rts_getWord64(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getPtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getPtr(arg1.into()) };
    let actual = unsafe { rts_getPtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getPtr() {
    let arg1 = todo!();
    unsafe { rts_getPtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getFunPtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getFunPtr(arg1.into()) };
    let actual = unsafe { rts_getFunPtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getFunPtr() {
    let arg1 = todo!();
    unsafe { rts_getFunPtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getFloat(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getFloat(arg1.into()) };
    let actual = unsafe { rts_getFloat(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getFloat() {
    let arg1 = todo!();
    unsafe { rts_getFloat(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getDouble(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getDouble(arg1.into()) };
    let actual = unsafe { rts_getDouble(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getDouble() {
    let arg1 = todo!();
    unsafe { rts_getDouble(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getStablePtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getStablePtr(arg1.into()) };
    let actual = unsafe { rts_getStablePtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getStablePtr() {
    let arg1 = todo!();
    unsafe { rts_getStablePtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getBool(arg1: HaskellObj) -> bool {
    let expected = unsafe { sys::rts_getBool(arg1.into()) };
    let actual = unsafe { rts_getBool(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getBool() {
    let arg1 = todo!();
    unsafe { rts_getBool(arg1) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval() {
    let mut arg1 = null_mut();
    let p = todo!();
    let mut ret = null_mut();
    unsafe { rts_eval(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval_() {
    let mut arg1 = null_mut();
    let p = todo!();
    let stack_size = Default::default();
    let mut ret = null_mut();
    unsafe { rts_eval_(&mut &mut arg1, p, stack_size, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalIO() {
    let mut arg1 = null_mut();
    let p = todo!();
    let mut ret = null_mut();
    unsafe { rts_evalIO(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIOMain() {
    let mut arg1 = null_mut();
    let s = Default::default();
    let mut ret = null_mut();
    unsafe { rts_evalStableIOMain(&mut &mut arg1, s, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIO() {
    let mut arg1 = null_mut();
    let s = Default::default();
    let mut ret = null_mut();
    unsafe { rts_evalStableIO(&mut &mut arg1, s, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO() {
    let mut arg1 = null_mut();
    let p = todo!();
    let mut ret = null_mut();
    unsafe { rts_evalLazyIO(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO_() {
    let mut arg1 = null_mut();
    let p = todo!();
    let stack_size = Default::default();
    let mut ret = null_mut();
    unsafe { rts_evalLazyIO_(&mut &mut arg1, p, stack_size, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_inCall() {
    let mut arg1 = null_mut();
    let p = todo!();
    let mut ret = null_mut();
    unsafe { rts_inCall(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_checkSchedStatus() {
    let mut site = null_mut();
    let mut arg1 = null_mut();
    unsafe { rts_checkSchedStatus(&mut site, &mut arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getSchedStatus(cap: Capability) -> bool {
    let expected = unsafe { transmute(sys::rts_getSchedStatus(&mut cap.into())) };
    let actual = unsafe { rts_getSchedStatus(&mut cap) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getSchedStatus() {
    let mut cap = null_mut();
    unsafe { rts_getSchedStatus(&mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_pause() -> bool {
    let expected = unsafe { transmute(sys::rts_pause()) };
    let actual = unsafe { rts_pause() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_pause() {
    unsafe { rts_pause() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_resume() {
    let mut pauseToken = null_mut();
    unsafe { rts_resume(&mut pauseToken) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isPaused() -> bool {
    let expected = unsafe { transmute(sys::rts_isPaused()) };
    let actual = unsafe { rts_isPaused() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isPaused() {
    unsafe { rts_isPaused() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_listThreads() {
    let cb = todo!();
    let mut user = null_mut();
    unsafe { rts_listThreads(cb, &mut user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_listMiscRoots() {
    let cb = todo!();
    let mut user = null_mut();
    unsafe { rts_listMiscRoots(cb, &mut user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_done() {
    unsafe { rts_done() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_clearMemory() {
    unsafe { rts_clearMemory() };
    todo!("assert")
}
