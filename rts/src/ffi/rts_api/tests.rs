use super::*;
use crate::ffi::hs_ffi::{HsBool, HsWord};

#[cfg(feature = "sys")]
#[test]
fn sys_size_PauseToken_() {
    assert_eq!(size_of::<sys::PauseToken_>(), size_of::<PauseToken_>())
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_pauseTokenCapability() {
    let expected = {
        let mut pauseToken: sys::PauseToken = todo!();
        let result: &Capability =
            unsafe { transmute(&*sys::pauseTokenCapability(&raw mut pauseToken)) };
        todo!()
    };
    let actual = {
        let mut pauseToken: PauseToken = todo!();
        let result: &Capability = unsafe { &*pauseTokenCapability(&raw mut pauseToken) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_pauseTokenCapability() {
    let actual = {
        let pauseToken: PauseToken = todo!();
        let result: &Capability = unsafe { &*pauseTokenCapability(&raw mut pauseToken) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
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

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getRTSStats(s: RTSStats) -> bool {
    let expected = {
        let mut s = s.clone().into();
        unsafe { sys::getRTSStats(&raw mut s) };
        todo!()
    };
    let actual = {
        let mut s = s.clone();
        unsafe { getRTSStats(&raw mut s) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getRTSStats() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut s: RTSStats = Arbitrary::arbitrary(g);
        unsafe { getRTSStats(&raw mut s) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getRTSStatsEnabled() {
    let expected: c_int = { unsafe { sys::getRTSStatsEnabled() } };
    let actual: c_int = { unsafe { getRTSStatsEnabled() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getRTSStatsEnabled() {
    let actual: c_int = { unsafe { getRTSStatsEnabled() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
fn equivalent_getAllocations() {
    let expected: u64 = { unsafe { sys::getAllocations() } };
    let actual: u64 = { unsafe { getAllocations() } };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getAllocations() {
    let actual: u64 = { unsafe { getAllocations() } };
    let expected: u64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init_with_rtsopts(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init_with_rtsopts() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { hs_init_with_rtsopts(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_init_ghc(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let argv = &raw mut argv;
        let rts_config: sys::RtsConfig = todo!();
        unsafe { sys::hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };
    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        let rts_config: RtsConfig = todo!();
        unsafe { hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_init_ghc() {
    let g = &mut Gen::new(100);
    let actual = {
        let argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let argv = &raw mut argv;
        let rts_config: RtsConfig = todo!();
        unsafe { hs_init_ghc(&raw mut argc, &raw mut argv, rts_config) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getProgArgv(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getProgArgv() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setProgArgv(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let mut argv = argv;
        let mut argv = &raw mut argv;
        unsafe { sys::setProgArgv(argc, &raw mut argv) };
        todo!()
    };
    let actual = {
        let mut argv = argv;
        let mut argv = &raw mut argv;
        unsafe { setProgArgv(argc, &raw mut argv) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setProgArgv() {
    let g = &mut Gen::new(100);
    let actual = {
        let argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        unsafe { setProgArgv(argc, &raw mut argv) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getFullProgArgv(argc: c_int, argv: c_char) -> bool {
    let expected = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { sys::getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let actual = {
        let mut argc = argc;
        let mut argv = argv;
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getFullProgArgv() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut argc: c_int = Arbitrary::arbitrary(g);
        let mut argv: c_char = Arbitrary::arbitrary(g);
        let mut argv = &raw mut argv;
        let mut argv = &raw mut argv;
        unsafe { getFullProgArgv(&raw mut argc, &raw mut argv) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_lock() {
    let expected = {
        let result: &Capability = unsafe { transmute(&*sys::rts_lock()) };
        todo!()
    };
    let actual = {
        let result: &Capability = unsafe { &*rts_lock() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_lock() {
    let actual = {
        let result: &Capability = unsafe { &*rts_lock() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_unlock() {
    let expected = {
        let mut token: sys::Capability = todo!();
        unsafe { sys::rts_unlock(&raw mut token) };
        todo!()
    };
    let actual = {
        let mut token: Capability = todo!();
        unsafe { rts_unlock(&raw mut token) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_unlock() {
    let actual = {
        let token: Capability = todo!();
        unsafe { rts_unlock(&raw mut token) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_unsafeGetMyCapability() {
    let expected = {
        let result: &Capability = unsafe { transmute(&*sys::rts_unsafeGetMyCapability()) };
        todo!()
    };
    let actual = {
        let result: &Capability = unsafe { &*rts_unsafeGetMyCapability() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_unsafeGetMyCapability() {
    let actual = {
        let result: &Capability = unsafe { &*rts_unsafeGetMyCapability() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_setInCallCapability(preferred_capability: c_int, affinity: c_int) -> bool {
    let expected = {
        unsafe { sys::rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };
    let actual = {
        unsafe { rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_setInCallCapability() {
    let g = &mut Gen::new(100);
    let actual = {
        let preferred_capability: c_int = Arbitrary::arbitrary(g);
        let affinity: c_int = Arbitrary::arbitrary(g);
        unsafe { rts_setInCallCapability(preferred_capability, affinity) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_pinThreadToNumaNode(node: c_int) -> bool {
    let expected = {
        unsafe { sys::rts_pinThreadToNumaNode(node) };
        todo!()
    };
    let actual = {
        unsafe { rts_pinThreadToNumaNode(node) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_pinThreadToNumaNode() {
    let g = &mut Gen::new(100);
    let actual = {
        let node: c_int = Arbitrary::arbitrary(g);
        unsafe { rts_pinThreadToNumaNode(node) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkChar(c: HsChar) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkChar(&raw mut arg1, c)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkChar(&raw mut arg1, c) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkChar() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let c: HsChar = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkChar(&raw mut arg1, c) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt(i: HsInt) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt(&raw mut arg1, i)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt(&raw mut arg1, i) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt(&raw mut arg1, i) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt8(i: HsInt8) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt8(&raw mut arg1, i)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt8(&raw mut arg1, i) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt8() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt8 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt8(&raw mut arg1, i) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt16(i: HsInt16) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt16(&raw mut arg1, i)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt16(&raw mut arg1, i) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt16() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt16 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt16(&raw mut arg1, i) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt32(i: HsInt32) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt32(&raw mut arg1, i)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt32(&raw mut arg1, i) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt32() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt32 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt32(&raw mut arg1, i) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkInt64(i: HsInt64) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkInt64(&raw mut arg1, i)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkInt64(&raw mut arg1, i) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkInt64() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let i: HsInt64 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkInt64(&raw mut arg1, i) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord(w: HsWord) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord(&raw mut arg1, w)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord(&raw mut arg1, w) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord(&raw mut arg1, w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord8(w: HsWord8) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord8(&raw mut arg1, w)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord8(&raw mut arg1, w) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord8() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord8 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord8(&raw mut arg1, w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord16(w: HsWord16) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord16(&raw mut arg1, w)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord16(&raw mut arg1, w) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord16() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord16 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord16(&raw mut arg1, w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord32(w: HsWord32) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord32(&raw mut arg1, w)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord32(&raw mut arg1, w) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord32() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord32 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord32(&raw mut arg1, w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkWord64(w: HsWord64) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkWord64(&raw mut arg1, w)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkWord64(&raw mut arg1, w) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkWord64() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let w: HsWord64 = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkWord64(&raw mut arg1, w) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkPtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkPtr(&raw mut arg1, a)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkPtr(&raw mut arg1, a) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkPtr() {
    let actual = {
        let arg1: Capability = todo!();
        let a: HsPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkPtr(&raw mut arg1, a) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkFunPtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkFunPtr(&raw mut arg1, a)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkFunPtr(&raw mut arg1, a) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkFunPtr() {
    let actual = {
        let arg1: Capability = todo!();
        let a: HsFunPtr = todo!();
        let result: HaskellObj = unsafe { rts_mkFunPtr(&raw mut arg1, a) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkFloat(f: HsFloat) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkFloat(&raw mut arg1, f)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkFloat(&raw mut arg1, f) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkFloat() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let f: HsFloat = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkFloat(&raw mut arg1, f) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkDouble(f: HsDouble) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkDouble(&raw mut arg1, f)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkDouble(&raw mut arg1, f) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkDouble() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let f: HsDouble = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkDouble(&raw mut arg1, f) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkStablePtr() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkStablePtr(&raw mut arg1, s)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { rts_mkStablePtr(&raw mut arg1, s) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkStablePtr() {
    let actual = {
        let arg1: Capability = todo!();
        let s: HsStablePtr = todo!();
        let result: HaskellObj = unsafe { rts_mkStablePtr(&raw mut arg1, s) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkBool(b: HsBool) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_mkBool(&raw mut arg1, b)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let result: HaskellObj = unsafe { rts_mkBool(&raw mut arg1, b) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkBool() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let b: HsBool = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkBool(&raw mut arg1, b) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_mkString(s: c_char) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut s = s;
        let result: HaskellObj = unsafe { transmute(sys::rts_mkString(&raw mut arg1, &raw mut s)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut s = s;
        let result: HaskellObj = unsafe { rts_mkString(&raw mut arg1, &raw mut s) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_mkString() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: Capability = todo!();
        let mut s: c_char = Arbitrary::arbitrary(g);
        let result: HaskellObj = unsafe { rts_mkString(&raw mut arg1, &raw mut s) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_apply() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let arg2: sys::HaskellObj = todo!();
        let arg3: sys::HaskellObj = todo!();
        let result: HaskellObj = unsafe { transmute(sys::rts_apply(&raw mut arg1, arg2, arg3)) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let arg2: HaskellObj = todo!();
        let arg3: HaskellObj = todo!();
        let result: HaskellObj = unsafe { rts_apply(&raw mut arg1, arg2, arg3) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_apply() {
    let actual = {
        let arg1: Capability = todo!();
        let arg2: HaskellObj = todo!();
        let arg3: HaskellObj = todo!();
        let result: HaskellObj = unsafe { rts_apply(&raw mut arg1, arg2, arg3) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getChar() {
    let expected: HsChar = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getChar(arg1) }
    };
    let actual: HsChar = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getChar(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getChar() {
    let actual: HsChar = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getChar(arg1) }
    };
    let expected: HsChar = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt() {
    let expected: HsInt = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt(arg1) }
    };
    let actual: HsInt = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt() {
    let actual: HsInt = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt(arg1) }
    };
    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt8() {
    let expected: HsInt8 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt8(arg1) }
    };
    let actual: HsInt8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt8(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt8() {
    let actual: HsInt8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt8(arg1) }
    };
    let expected: HsInt8 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt16() {
    let expected: HsInt16 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt16(arg1) }
    };
    let actual: HsInt16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt16(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt16() {
    let actual: HsInt16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt16(arg1) }
    };
    let expected: HsInt16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt32() {
    let expected: HsInt32 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt32(arg1) }
    };
    let actual: HsInt32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt32(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt32() {
    let actual: HsInt32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt32(arg1) }
    };
    let expected: HsInt32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getInt64() {
    let expected: HsInt64 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getInt64(arg1) }
    };
    let actual: HsInt64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt64(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getInt64() {
    let actual: HsInt64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getInt64(arg1) }
    };
    let expected: HsInt64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord() {
    let expected: HsWord = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord(arg1) }
    };
    let actual: HsWord = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord() {
    let actual: HsWord = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord(arg1) }
    };
    let expected: HsWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord8() {
    let expected: HsWord8 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord8(arg1) }
    };
    let actual: HsWord8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord8(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord8() {
    let actual: HsWord8 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord8(arg1) }
    };
    let expected: HsWord8 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord16() {
    let expected: HsWord16 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord16(arg1) }
    };
    let actual: HsWord16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord16(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord16() {
    let actual: HsWord16 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord16(arg1) }
    };
    let expected: HsWord16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord32() {
    let expected: HsWord32 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord32(arg1) }
    };
    let actual: HsWord32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord32(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord32() {
    let actual: HsWord32 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord32(arg1) }
    };
    let expected: HsWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getWord64() {
    let expected: HsWord64 = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getWord64(arg1) }
    };
    let actual: HsWord64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord64(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getWord64() {
    let actual: HsWord64 = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getWord64(arg1) }
    };
    let expected: HsWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getPtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsPtr = unsafe { sys::rts_getPtr(arg1) };
        todo!()
    };
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsPtr = unsafe { rts_getPtr(arg1) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getPtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsPtr = unsafe { rts_getPtr(arg1) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getFunPtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsFunPtr = unsafe { sys::rts_getFunPtr(arg1) };
        todo!()
    };
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsFunPtr = unsafe { rts_getFunPtr(arg1) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getFunPtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsFunPtr = unsafe { rts_getFunPtr(arg1) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getFloat() {
    let expected: HsFloat = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getFloat(arg1) }
    };
    let actual: HsFloat = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getFloat(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getFloat() {
    let actual: HsFloat = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getFloat(arg1) }
    };
    let expected: HsFloat = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getDouble() {
    let expected: HsDouble = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getDouble(arg1) }
    };
    let actual: HsDouble = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getDouble(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getDouble() {
    let actual: HsDouble = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getDouble(arg1) }
    };
    let expected: HsDouble = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getStablePtr() {
    let expected = {
        let arg1: sys::HaskellObj = todo!();
        let result: HsStablePtr = unsafe { sys::rts_getStablePtr(arg1) };
        todo!()
    };
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsStablePtr = unsafe { rts_getStablePtr(arg1) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getStablePtr() {
    let actual = {
        let arg1: HaskellObj = todo!();
        let result: HsStablePtr = unsafe { rts_getStablePtr(arg1) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getBool() {
    let expected: HsBool = {
        let arg1: sys::HaskellObj = todo!();
        unsafe { sys::rts_getBool(arg1) }
    };
    let actual: HsBool = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getBool(arg1) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getBool() {
    let actual: HsBool = {
        let arg1: HaskellObj = todo!();
        unsafe { rts_getBool(arg1) }
    };
    let expected: HsBool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_eval() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_eval() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_eval(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_eval_(stack_size: c_uint) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_eval_() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let stack_size: c_uint = Arbitrary::arbitrary(g);
        let ret: HaskellObj = todo!();
        unsafe { rts_eval_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_evalIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalStableIOMain() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { sys::rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalStableIOMain() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIOMain(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalStableIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { sys::rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let mut ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalStableIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let s: HsStablePtr = todo!();
        let ret: HsStablePtr = todo!();
        unsafe { rts_evalStableIO(&raw mut arg1, s, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalLazyIO() {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalLazyIO() {
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO(&raw mut arg1, p, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_evalLazyIO_(stack_size: c_uint) -> bool {
    let expected = {
        let mut arg1: sys::Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: sys::HaskellObj = todo!();
        let mut ret: sys::HaskellObj = todo!();
        unsafe { sys::rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let mut ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    expected == actual
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_evalLazyIO_() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut arg1: Capability = todo!();
        let mut arg1 = &raw mut arg1;
        let p: HaskellObj = todo!();
        let stack_size: c_uint = Arbitrary::arbitrary(g);
        let ret: HaskellObj = todo!();
        unsafe { rts_evalLazyIO_(&raw mut arg1, p, stack_size, &raw mut ret) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_getSchedStatus() {
    let expected: SchedulerStatus = {
        let mut cap: sys::Capability = todo!();
        unsafe { transmute(sys::rts_getSchedStatus(&raw mut cap)) }
    };
    let actual: SchedulerStatus = {
        let mut cap: Capability = todo!();
        unsafe { rts_getSchedStatus(&raw mut cap) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_getSchedStatus() {
    let actual: SchedulerStatus = {
        let cap: Capability = todo!();
        unsafe { rts_getSchedStatus(&raw mut cap) }
    };
    let expected: SchedulerStatus = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_pause() {
    let expected = {
        let result: &PauseToken = unsafe { transmute(&*sys::rts_pause()) };
        todo!()
    };
    let actual = {
        let result: &PauseToken = unsafe { &*rts_pause() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_pause() {
    let actual = {
        let result: &PauseToken = unsafe { &*rts_pause() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_resume() {
    let expected = {
        let mut pauseToken: sys::PauseToken = todo!();
        unsafe { sys::rts_resume(&raw mut pauseToken) };
        todo!()
    };
    let actual = {
        let mut pauseToken: PauseToken = todo!();
        unsafe { rts_resume(&raw mut pauseToken) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_resume() {
    let actual = {
        let pauseToken: PauseToken = todo!();
        unsafe { rts_resume(&raw mut pauseToken) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
fn equivalent_rts_isPaused() {
    let expected: bool = { unsafe { sys::rts_isPaused() } };
    let actual: bool = { unsafe { rts_isPaused() } };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_isPaused() {
    let actual: bool = { unsafe { rts_isPaused() } };
    let expected: bool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_listThreads() {
    let expected = {
        let cb: sys::ListThreadsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { sys::rts_listThreads(cb, &raw mut user) };
        todo!()
    };
    let actual = {
        let cb: ListThreadsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { rts_listThreads(cb, &raw mut user) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_listThreads() {
    let actual = {
        let cb: ListThreadsCb = todo!();
        let user: c_void = todo!();
        unsafe { rts_listThreads(cb, &raw mut user) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_listMiscRoots() {
    let expected = {
        let cb: sys::ListRootsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { sys::rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };
    let actual = {
        let cb: ListRootsCb = todo!();
        let mut user: c_void = todo!();
        unsafe { rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_listMiscRoots() {
    let actual = {
        let cb: ListRootsCb = todo!();
        let user: c_void = todo!();
        unsafe { rts_listMiscRoots(cb, &raw mut user) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_clearMemory() {
    let expected = {
        unsafe { sys::rts_clearMemory() };
        todo!()
    };
    let actual = {
        unsafe { rts_clearMemory() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_clearMemory() {
    let actual = {
        unsafe { rts_clearMemory() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
