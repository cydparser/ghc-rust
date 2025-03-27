use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_PauseToken_() {
    assert_eq!(
        size_of::<sys::PauseToken_>(),
        size_of::<super::PauseToken_>()
    )
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_pauseTokenCapability(pauseToken: PauseToken) -> bool {
    let expected = unsafe { transmute(sys::pauseTokenCapability(&mut pauseToken.into())) };
    let actual = unsafe { super::pauseTokenCapability(&mut pauseToken) };
    actual == expected
}

#[test]
#[ignore]
fn test_pauseTokenCapability() {
    let pauseToken = Default::default();
    unsafe { super::pauseTokenCapability(&mut pauseToken) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_CapabilityPublic_() {
    assert_eq!(
        size_of::<sys::CapabilityPublic_>(),
        size_of::<super::CapabilityPublic_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CapabilityPublic_"][::core::mem::size_of::<CapabilityPublic_>() - 944usize];
    ["Alignment of CapabilityPublic_"][::core::mem::align_of::<CapabilityPublic_>() - 8usize];
    ["Offset of field: CapabilityPublic_::f"]
        [::core::mem::offset_of!(CapabilityPublic_, f) - 0usize];
    ["Offset of field: CapabilityPublic_::r"]
        [::core::mem::offset_of!(CapabilityPublic_, r) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_RtsConfig() {
    assert_eq!(size_of::<sys::RtsConfig>(), size_of::<super::RtsConfig>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RtsConfig"][::core::mem::size_of::<RtsConfig>() - 112usize];
    ["Alignment of RtsConfig"][::core::mem::align_of::<RtsConfig>() - 8usize];
    ["Offset of field: RtsConfig::rts_opts_enabled"]
        [::core::mem::offset_of!(RtsConfig, rts_opts_enabled) - 0usize];
    ["Offset of field: RtsConfig::rts_opts_suggestions"]
        [::core::mem::offset_of!(RtsConfig, rts_opts_suggestions) - 8usize];
    ["Offset of field: RtsConfig::rts_opts"]
        [::core::mem::offset_of!(RtsConfig, rts_opts) - 16usize];
    ["Offset of field: RtsConfig::rts_hs_main"]
        [::core::mem::offset_of!(RtsConfig, rts_hs_main) - 24usize];
    ["Offset of field: RtsConfig::keep_cafs"]
        [::core::mem::offset_of!(RtsConfig, keep_cafs) - 32usize];
    ["Offset of field: RtsConfig::eventlog_writer"]
        [::core::mem::offset_of!(RtsConfig, eventlog_writer) - 40usize];
    ["Offset of field: RtsConfig::defaultsHook"]
        [::core::mem::offset_of!(RtsConfig, defaultsHook) - 48usize];
    ["Offset of field: RtsConfig::onExitHook"]
        [::core::mem::offset_of!(RtsConfig, onExitHook) - 56usize];
    ["Offset of field: RtsConfig::stackOverflowHook"]
        [::core::mem::offset_of!(RtsConfig, stackOverflowHook) - 64usize];
    ["Offset of field: RtsConfig::outOfHeapHook"]
        [::core::mem::offset_of!(RtsConfig, outOfHeapHook) - 72usize];
    ["Offset of field: RtsConfig::mallocFailHook"]
        [::core::mem::offset_of!(RtsConfig, mallocFailHook) - 80usize];
    ["Offset of field: RtsConfig::gcDoneHook"]
        [::core::mem::offset_of!(RtsConfig, gcDoneHook) - 88usize];
    ["Offset of field: RtsConfig::longGCSync"]
        [::core::mem::offset_of!(RtsConfig, longGCSync) - 96usize];
    ["Offset of field: RtsConfig::longGCSyncEnd"]
        [::core::mem::offset_of!(RtsConfig, longGCSyncEnd) - 104usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_GCDetails_() {
    assert_eq!(size_of::<sys::GCDetails_>(), size_of::<super::GCDetails_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GCDetails_"][::core::mem::size_of::<GCDetails_>() - 144usize];
    ["Alignment of GCDetails_"][::core::mem::align_of::<GCDetails_>() - 8usize];
    ["Offset of field: GCDetails_::gen_"][::core::mem::offset_of!(GCDetails_, gen_) - 0usize];
    ["Offset of field: GCDetails_::threads"][::core::mem::offset_of!(GCDetails_, threads) - 4usize];
    ["Offset of field: GCDetails_::allocated_bytes"]
        [::core::mem::offset_of!(GCDetails_, allocated_bytes) - 8usize];
    ["Offset of field: GCDetails_::live_bytes"]
        [::core::mem::offset_of!(GCDetails_, live_bytes) - 16usize];
    ["Offset of field: GCDetails_::large_objects_bytes"]
        [::core::mem::offset_of!(GCDetails_, large_objects_bytes) - 24usize];
    ["Offset of field: GCDetails_::compact_bytes"]
        [::core::mem::offset_of!(GCDetails_, compact_bytes) - 32usize];
    ["Offset of field: GCDetails_::slop_bytes"]
        [::core::mem::offset_of!(GCDetails_, slop_bytes) - 40usize];
    ["Offset of field: GCDetails_::mem_in_use_bytes"]
        [::core::mem::offset_of!(GCDetails_, mem_in_use_bytes) - 48usize];
    ["Offset of field: GCDetails_::copied_bytes"]
        [::core::mem::offset_of!(GCDetails_, copied_bytes) - 56usize];
    ["Offset of field: GCDetails_::block_fragmentation_bytes"]
        [::core::mem::offset_of!(GCDetails_, block_fragmentation_bytes) - 64usize];
    ["Offset of field: GCDetails_::par_max_copied_bytes"]
        [::core::mem::offset_of!(GCDetails_, par_max_copied_bytes) - 72usize];
    ["Offset of field: GCDetails_::par_balanced_copied_bytes"]
        [::core::mem::offset_of!(GCDetails_, par_balanced_copied_bytes) - 80usize];
    ["Offset of field: GCDetails_::sync_elapsed_ns"]
        [::core::mem::offset_of!(GCDetails_, sync_elapsed_ns) - 88usize];
    ["Offset of field: GCDetails_::cpu_ns"][::core::mem::offset_of!(GCDetails_, cpu_ns) - 96usize];
    ["Offset of field: GCDetails_::elapsed_ns"]
        [::core::mem::offset_of!(GCDetails_, elapsed_ns) - 104usize];
    ["Offset of field: GCDetails_::nonmoving_gc_sync_cpu_ns"]
        [::core::mem::offset_of!(GCDetails_, nonmoving_gc_sync_cpu_ns) - 112usize];
    ["Offset of field: GCDetails_::nonmoving_gc_sync_elapsed_ns"]
        [::core::mem::offset_of!(GCDetails_, nonmoving_gc_sync_elapsed_ns) - 120usize];
    ["Offset of field: GCDetails_::nonmoving_gc_cpu_ns"]
        [::core::mem::offset_of!(GCDetails_, nonmoving_gc_cpu_ns) - 128usize];
    ["Offset of field: GCDetails_::nonmoving_gc_elapsed_ns"]
        [::core::mem::offset_of!(GCDetails_, nonmoving_gc_elapsed_ns) - 136usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__RTSStats() {
    assert_eq!(size_of::<sys::_RTSStats>(), size_of::<super::_RTSStats>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _RTSStats"][::core::mem::size_of::<_RTSStats>() - 376usize];
    ["Alignment of _RTSStats"][::core::mem::align_of::<_RTSStats>() - 8usize];
    ["Offset of field: _RTSStats::gcs"][::core::mem::offset_of!(_RTSStats, gcs) - 0usize];
    ["Offset of field: _RTSStats::major_gcs"]
        [::core::mem::offset_of!(_RTSStats, major_gcs) - 4usize];
    ["Offset of field: _RTSStats::allocated_bytes"]
        [::core::mem::offset_of!(_RTSStats, allocated_bytes) - 8usize];
    ["Offset of field: _RTSStats::max_live_bytes"]
        [::core::mem::offset_of!(_RTSStats, max_live_bytes) - 16usize];
    ["Offset of field: _RTSStats::max_large_objects_bytes"]
        [::core::mem::offset_of!(_RTSStats, max_large_objects_bytes) - 24usize];
    ["Offset of field: _RTSStats::max_compact_bytes"]
        [::core::mem::offset_of!(_RTSStats, max_compact_bytes) - 32usize];
    ["Offset of field: _RTSStats::max_slop_bytes"]
        [::core::mem::offset_of!(_RTSStats, max_slop_bytes) - 40usize];
    ["Offset of field: _RTSStats::max_mem_in_use_bytes"]
        [::core::mem::offset_of!(_RTSStats, max_mem_in_use_bytes) - 48usize];
    ["Offset of field: _RTSStats::cumulative_live_bytes"]
        [::core::mem::offset_of!(_RTSStats, cumulative_live_bytes) - 56usize];
    ["Offset of field: _RTSStats::copied_bytes"]
        [::core::mem::offset_of!(_RTSStats, copied_bytes) - 64usize];
    ["Offset of field: _RTSStats::par_copied_bytes"]
        [::core::mem::offset_of!(_RTSStats, par_copied_bytes) - 72usize];
    ["Offset of field: _RTSStats::cumulative_par_max_copied_bytes"]
        [::core::mem::offset_of!(_RTSStats, cumulative_par_max_copied_bytes) - 80usize];
    ["Offset of field: _RTSStats::cumulative_par_balanced_copied_bytes"]
        [::core::mem::offset_of!(_RTSStats, cumulative_par_balanced_copied_bytes) - 88usize];
    ["Offset of field: _RTSStats::init_cpu_ns"]
        [::core::mem::offset_of!(_RTSStats, init_cpu_ns) - 96usize];
    ["Offset of field: _RTSStats::init_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, init_elapsed_ns) - 104usize];
    ["Offset of field: _RTSStats::mutator_cpu_ns"]
        [::core::mem::offset_of!(_RTSStats, mutator_cpu_ns) - 112usize];
    ["Offset of field: _RTSStats::mutator_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, mutator_elapsed_ns) - 120usize];
    ["Offset of field: _RTSStats::gc_cpu_ns"]
        [::core::mem::offset_of!(_RTSStats, gc_cpu_ns) - 128usize];
    ["Offset of field: _RTSStats::gc_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, gc_elapsed_ns) - 136usize];
    ["Offset of field: _RTSStats::cpu_ns"][::core::mem::offset_of!(_RTSStats, cpu_ns) - 144usize];
    ["Offset of field: _RTSStats::elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, elapsed_ns) - 152usize];
    ["Offset of field: _RTSStats::gc"][::core::mem::offset_of!(_RTSStats, gc) - 160usize];
    ["Offset of field: _RTSStats::any_work"]
        [::core::mem::offset_of!(_RTSStats, any_work) - 304usize];
    ["Offset of field: _RTSStats::scav_find_work"]
        [::core::mem::offset_of!(_RTSStats, scav_find_work) - 312usize];
    ["Offset of field: _RTSStats::max_n_todo_overflow"]
        [::core::mem::offset_of!(_RTSStats, max_n_todo_overflow) - 320usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_cpu_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_sync_cpu_ns) - 328usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_sync_elapsed_ns) - 336usize];
    ["Offset of field: _RTSStats::nonmoving_gc_sync_max_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_sync_max_elapsed_ns) - 344usize];
    ["Offset of field: _RTSStats::nonmoving_gc_cpu_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_cpu_ns) - 352usize];
    ["Offset of field: _RTSStats::nonmoving_gc_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_elapsed_ns) - 360usize];
    ["Offset of field: _RTSStats::nonmoving_gc_max_elapsed_ns"]
        [::core::mem::offset_of!(_RTSStats, nonmoving_gc_max_elapsed_ns) - 368usize];
};

#[test]
#[ignore]
fn test_getRTSStats() {
    let s = Default::default();
    unsafe { super::getRTSStats(&mut s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getRTSStatsEnabled() -> bool {
    let expected = unsafe { transmute(sys::getRTSStatsEnabled()) };
    let actual = unsafe { super::getRTSStatsEnabled() };
    actual == expected
}

#[test]
#[ignore]
fn test_getRTSStatsEnabled() {
    unsafe { super::getRTSStatsEnabled() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getAllocations() -> bool {
    let expected = unsafe { transmute(sys::getAllocations()) };
    let actual = unsafe { super::getAllocations() };
    actual == expected
}

#[test]
#[ignore]
fn test_getAllocations() {
    unsafe { super::getAllocations() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startupHaskell() {
    let argc = Default::default();
    let argv = Default::default();
    let init_root = Default::default();
    unsafe { super::startupHaskell(argc, &mut &mut argv, init_root) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_shutdownHaskell() {
    unsafe { super::shutdownHaskell() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_init_with_rtsopts() {
    let argc = Default::default();
    let argv = Default::default();
    unsafe { super::hs_init_with_rtsopts(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_init_ghc() {
    let argc = Default::default();
    let argv = Default::default();
    let rts_config = Default::default();
    unsafe { super::hs_init_ghc(&mut argc, &mut &mut &mut argv, rts_config) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownHaskellAndExit(
    exitCode: ::core::ffi::c_int,
    fastExit: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::shutdownHaskellAndExit(
            exitCode.into(),
            fastExit.into(),
        ))
    };
    let actual = unsafe { super::shutdownHaskellAndExit(exitCode, fastExit) };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownHaskellAndExit() {
    let exitCode = Default::default();
    let fastExit = Default::default();
    unsafe { super::shutdownHaskellAndExit(exitCode, fastExit) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_shutdownHaskellAndSignal(
    sig: ::core::ffi::c_int,
    fastExit: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe { transmute(sys::shutdownHaskellAndSignal(sig.into(), fastExit.into())) };
    let actual = unsafe { super::shutdownHaskellAndSignal(sig, fastExit) };
    actual == expected
}

#[test]
#[ignore]
fn test_shutdownHaskellAndSignal() {
    let sig = Default::default();
    let fastExit = Default::default();
    unsafe { super::shutdownHaskellAndSignal(sig, fastExit) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getProgArgv() {
    let argc = Default::default();
    let argv = Default::default();
    unsafe { super::getProgArgv(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setProgArgv() {
    let argc = Default::default();
    let argv = Default::default();
    unsafe { super::setProgArgv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getFullProgArgv() {
    let argc = Default::default();
    let argv = Default::default();
    unsafe { super::getFullProgArgv(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setFullProgArgv() {
    let argc = Default::default();
    let argv = Default::default();
    unsafe { super::setFullProgArgv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeFullProgArgv() {
    unsafe { super::freeFullProgArgv() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_lock() -> bool {
    let expected = unsafe { transmute(sys::rts_lock()) };
    let actual = unsafe { super::rts_lock() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_lock() {
    unsafe { super::rts_lock() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_unlock() {
    let token = Default::default();
    unsafe { super::rts_unlock(&mut token) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_unsafeGetMyCapability() -> bool {
    let expected = unsafe { transmute(sys::rts_unsafeGetMyCapability()) };
    let actual = unsafe { super::rts_unsafeGetMyCapability() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_unsafeGetMyCapability() {
    unsafe { super::rts_unsafeGetMyCapability() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_setInCallCapability() {
    let preferred_capability = Default::default();
    let affinity = Default::default();
    unsafe { super::rts_setInCallCapability(preferred_capability, affinity) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_pinThreadToNumaNode() {
    let node = Default::default();
    unsafe { super::rts_pinThreadToNumaNode(node) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkChar(arg1: Capability, c: HsChar) -> bool {
    let expected = unsafe { transmute(sys::rts_mkChar(&mut arg1.into(), c.into())) };
    let actual = unsafe { super::rts_mkChar(&mut arg1, c) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkChar() {
    let arg1 = Default::default();
    let c = Default::default();
    unsafe { super::rts_mkChar(&mut arg1, c) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt(arg1: Capability, i: HsInt) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt(&mut arg1.into(), i.into())) };
    let actual = unsafe { super::rts_mkInt(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt() {
    let arg1 = Default::default();
    let i = Default::default();
    unsafe { super::rts_mkInt(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt8(arg1: Capability, i: HsInt8) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt8(&mut arg1.into(), i.into())) };
    let actual = unsafe { super::rts_mkInt8(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt8() {
    let arg1 = Default::default();
    let i = Default::default();
    unsafe { super::rts_mkInt8(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt16(arg1: Capability, i: HsInt16) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt16(&mut arg1.into(), i.into())) };
    let actual = unsafe { super::rts_mkInt16(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt16() {
    let arg1 = Default::default();
    let i = Default::default();
    unsafe { super::rts_mkInt16(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt32(arg1: Capability, i: HsInt32) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt32(&mut arg1.into(), i.into())) };
    let actual = unsafe { super::rts_mkInt32(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt32() {
    let arg1 = Default::default();
    let i = Default::default();
    unsafe { super::rts_mkInt32(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkInt64(arg1: Capability, i: HsInt64) -> bool {
    let expected = unsafe { transmute(sys::rts_mkInt64(&mut arg1.into(), i.into())) };
    let actual = unsafe { super::rts_mkInt64(&mut arg1, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt64() {
    let arg1 = Default::default();
    let i = Default::default();
    unsafe { super::rts_mkInt64(&mut arg1, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord(arg1: Capability, w: HsWord) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord(&mut arg1.into(), w.into())) };
    let actual = unsafe { super::rts_mkWord(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord() {
    let arg1 = Default::default();
    let w = Default::default();
    unsafe { super::rts_mkWord(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord8(arg1: Capability, w: HsWord8) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord8(&mut arg1.into(), w.into())) };
    let actual = unsafe { super::rts_mkWord8(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord8() {
    let arg1 = Default::default();
    let w = Default::default();
    unsafe { super::rts_mkWord8(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord16(arg1: Capability, w: HsWord16) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord16(&mut arg1.into(), w.into())) };
    let actual = unsafe { super::rts_mkWord16(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord16() {
    let arg1 = Default::default();
    let w = Default::default();
    unsafe { super::rts_mkWord16(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord32(arg1: Capability, w: HsWord32) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord32(&mut arg1.into(), w.into())) };
    let actual = unsafe { super::rts_mkWord32(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord32() {
    let arg1 = Default::default();
    let w = Default::default();
    unsafe { super::rts_mkWord32(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkWord64(arg1: Capability, w: HsWord64) -> bool {
    let expected = unsafe { transmute(sys::rts_mkWord64(&mut arg1.into(), w.into())) };
    let actual = unsafe { super::rts_mkWord64(&mut arg1, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord64() {
    let arg1 = Default::default();
    let w = Default::default();
    unsafe { super::rts_mkWord64(&mut arg1, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkPtr(arg1: Capability, a: HsPtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkPtr(&mut arg1.into(), a.into())) };
    let actual = unsafe { super::rts_mkPtr(&mut arg1, a) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkPtr() {
    let arg1 = Default::default();
    let a = Default::default();
    unsafe { super::rts_mkPtr(&mut arg1, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkFunPtr(arg1: Capability, a: HsFunPtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkFunPtr(&mut arg1.into(), a.into())) };
    let actual = unsafe { super::rts_mkFunPtr(&mut arg1, a) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkFunPtr() {
    let arg1 = Default::default();
    let a = Default::default();
    unsafe { super::rts_mkFunPtr(&mut arg1, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkFloat(arg1: Capability, f: HsFloat) -> bool {
    let expected = unsafe { transmute(sys::rts_mkFloat(&mut arg1.into(), f.into())) };
    let actual = unsafe { super::rts_mkFloat(&mut arg1, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkFloat() {
    let arg1 = Default::default();
    let f = Default::default();
    unsafe { super::rts_mkFloat(&mut arg1, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkDouble(arg1: Capability, f: HsDouble) -> bool {
    let expected = unsafe { transmute(sys::rts_mkDouble(&mut arg1.into(), f.into())) };
    let actual = unsafe { super::rts_mkDouble(&mut arg1, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkDouble() {
    let arg1 = Default::default();
    let f = Default::default();
    unsafe { super::rts_mkDouble(&mut arg1, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkStablePtr(arg1: Capability, s: HsStablePtr) -> bool {
    let expected = unsafe { transmute(sys::rts_mkStablePtr(&mut arg1.into(), s.into())) };
    let actual = unsafe { super::rts_mkStablePtr(&mut arg1, s) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkStablePtr() {
    let arg1 = Default::default();
    let s = Default::default();
    unsafe { super::rts_mkStablePtr(&mut arg1, s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkBool(arg1: Capability, b: HsBool) -> bool {
    let expected = unsafe { transmute(sys::rts_mkBool(&mut arg1.into(), b.into())) };
    let actual = unsafe { super::rts_mkBool(&mut arg1, b) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkBool() {
    let arg1 = Default::default();
    let b = Default::default();
    unsafe { super::rts_mkBool(&mut arg1, b) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_mkString(arg1: Capability, s: ::core::ffi::c_char) -> bool {
    let expected = unsafe { transmute(sys::rts_mkString(&mut arg1.into(), &mut s.into())) };
    let actual = unsafe { super::rts_mkString(&mut arg1, &mut s) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkString() {
    let arg1 = Default::default();
    let s = Default::default();
    unsafe { super::rts_mkString(&mut arg1, &mut s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_apply(arg1: Capability, arg2: HaskellObj, arg3: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_apply(&mut arg1.into(), arg2.into(), arg3.into())) };
    let actual = unsafe { super::rts_apply(&mut arg1, arg2, arg3) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_apply() {
    let arg1 = Default::default();
    let arg2 = Default::default();
    let arg3 = Default::default();
    unsafe { super::rts_apply(&mut arg1, arg2, arg3) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getChar(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getChar(arg1.into())) };
    let actual = unsafe { super::rts_getChar(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getChar() {
    let arg1 = Default::default();
    unsafe { super::rts_getChar(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getInt(arg1.into())) };
    let actual = unsafe { super::rts_getInt(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt() {
    let arg1 = Default::default();
    unsafe { super::rts_getInt(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt8(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getInt8(arg1.into())) };
    let actual = unsafe { super::rts_getInt8(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt8() {
    let arg1 = Default::default();
    unsafe { super::rts_getInt8(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt16(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getInt16(arg1.into())) };
    let actual = unsafe { super::rts_getInt16(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt16() {
    let arg1 = Default::default();
    unsafe { super::rts_getInt16(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt32(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getInt32(arg1.into())) };
    let actual = unsafe { super::rts_getInt32(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt32() {
    let arg1 = Default::default();
    unsafe { super::rts_getInt32(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getInt64(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getInt64(arg1.into())) };
    let actual = unsafe { super::rts_getInt64(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getInt64() {
    let arg1 = Default::default();
    unsafe { super::rts_getInt64(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getWord(arg1.into())) };
    let actual = unsafe { super::rts_getWord(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord() {
    let arg1 = Default::default();
    unsafe { super::rts_getWord(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord8(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getWord8(arg1.into())) };
    let actual = unsafe { super::rts_getWord8(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord8() {
    let arg1 = Default::default();
    unsafe { super::rts_getWord8(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord16(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getWord16(arg1.into())) };
    let actual = unsafe { super::rts_getWord16(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord16() {
    let arg1 = Default::default();
    unsafe { super::rts_getWord16(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord32(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getWord32(arg1.into())) };
    let actual = unsafe { super::rts_getWord32(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord32() {
    let arg1 = Default::default();
    unsafe { super::rts_getWord32(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getWord64(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getWord64(arg1.into())) };
    let actual = unsafe { super::rts_getWord64(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getWord64() {
    let arg1 = Default::default();
    unsafe { super::rts_getWord64(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getPtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getPtr(arg1.into())) };
    let actual = unsafe { super::rts_getPtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getPtr() {
    let arg1 = Default::default();
    unsafe { super::rts_getPtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getFunPtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getFunPtr(arg1.into())) };
    let actual = unsafe { super::rts_getFunPtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getFunPtr() {
    let arg1 = Default::default();
    unsafe { super::rts_getFunPtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getFloat(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getFloat(arg1.into())) };
    let actual = unsafe { super::rts_getFloat(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getFloat() {
    let arg1 = Default::default();
    unsafe { super::rts_getFloat(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getDouble(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getDouble(arg1.into())) };
    let actual = unsafe { super::rts_getDouble(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getDouble() {
    let arg1 = Default::default();
    unsafe { super::rts_getDouble(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getStablePtr(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getStablePtr(arg1.into())) };
    let actual = unsafe { super::rts_getStablePtr(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getStablePtr() {
    let arg1 = Default::default();
    unsafe { super::rts_getStablePtr(arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getBool(arg1: HaskellObj) -> bool {
    let expected = unsafe { transmute(sys::rts_getBool(arg1.into())) };
    let actual = unsafe { super::rts_getBool(arg1) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getBool() {
    let arg1 = Default::default();
    unsafe { super::rts_getBool(arg1) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval() {
    let arg1 = Default::default();
    let p = Default::default();
    let ret = Default::default();
    unsafe { super::rts_eval(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval_() {
    let arg1 = Default::default();
    let p = Default::default();
    let stack_size = Default::default();
    let ret = Default::default();
    unsafe { super::rts_eval_(&mut &mut arg1, p, stack_size, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalIO() {
    let arg1 = Default::default();
    let p = Default::default();
    let ret = Default::default();
    unsafe { super::rts_evalIO(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIOMain() {
    let arg1 = Default::default();
    let s = Default::default();
    let ret = Default::default();
    unsafe { super::rts_evalStableIOMain(&mut &mut arg1, s, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIO() {
    let arg1 = Default::default();
    let s = Default::default();
    let ret = Default::default();
    unsafe { super::rts_evalStableIO(&mut &mut arg1, s, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO() {
    let arg1 = Default::default();
    let p = Default::default();
    let ret = Default::default();
    unsafe { super::rts_evalLazyIO(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO_() {
    let arg1 = Default::default();
    let p = Default::default();
    let stack_size = Default::default();
    let ret = Default::default();
    unsafe { super::rts_evalLazyIO_(&mut &mut arg1, p, stack_size, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_inCall() {
    let arg1 = Default::default();
    let p = Default::default();
    let ret = Default::default();
    unsafe { super::rts_inCall(&mut &mut arg1, p, &mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_checkSchedStatus() {
    let site = Default::default();
    let arg1 = Default::default();
    unsafe { super::rts_checkSchedStatus(&mut site, &mut arg1) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_getSchedStatus(cap: Capability) -> bool {
    let expected = unsafe { transmute(sys::rts_getSchedStatus(&mut cap.into())) };
    let actual = unsafe { super::rts_getSchedStatus(&mut cap) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_getSchedStatus() {
    let cap = Default::default();
    unsafe { super::rts_getSchedStatus(&mut cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_pause() -> bool {
    let expected = unsafe { transmute(sys::rts_pause()) };
    let actual = unsafe { super::rts_pause() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_pause() {
    unsafe { super::rts_pause() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_resume() {
    let pauseToken = Default::default();
    unsafe { super::rts_resume(&mut pauseToken) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_rts_isPaused() -> bool {
    let expected = unsafe { transmute(sys::rts_isPaused()) };
    let actual = unsafe { super::rts_isPaused() };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_isPaused() {
    unsafe { super::rts_isPaused() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_listThreads() {
    let cb = Default::default();
    let user = Default::default();
    unsafe { super::rts_listThreads(cb, &mut user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_listMiscRoots() {
    let cb = Default::default();
    let user = Default::default();
    unsafe { super::rts_listMiscRoots(cb, &mut user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_done() {
    unsafe { super::rts_done() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_clearMemory() {
    unsafe { super::rts_clearMemory() };
    todo!("assert")
}
