use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_PauseToken_() {
    assert_eq!(size_of::<sys::PauseToken_>(), size_of::<PauseToken_>())
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_pauseTokenCapability() {
    todo!()
}

#[test]
#[ignore]
fn test_pauseTokenCapability() {
    let pauseToken = null_mut();
    unsafe { pauseTokenCapability(pauseToken) };
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
    let s = null_mut();
    unsafe { getRTSStats(s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
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
#[ignore]
fn equivalent_getAllocations() -> bool {
    let expected = unsafe { sys::getAllocations() };
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
    let argv = null_mut();
    let init_root = Default::default();
    unsafe { startupHaskell(argc, argv, init_root) };
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
    let argc = null_mut();
    let argv = null_mut();
    unsafe { hs_init_with_rtsopts(argc, argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_init_ghc() {
    let argc = null_mut();
    let argv = null_mut();
    let rts_config = defaultRtsConfig.clone();
    unsafe { hs_init_ghc(argc, argv, rts_config) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getProgArgv() {
    let argc = null_mut();
    let mut argv = null_mut();
    unsafe { getProgArgv(argc, &raw mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setProgArgv() {
    let argc = Default::default();
    let mut argv = null_mut();
    unsafe { setProgArgv(argc, &raw mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_getFullProgArgv() {
    let argc = null_mut();
    let mut argv = null_mut();
    unsafe { getFullProgArgv(argc, &raw mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setFullProgArgv() {
    let argc = Default::default();
    let mut argv = null_mut();
    unsafe { setFullProgArgv(argc, &raw mut argv) };
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
#[ignore]
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
    let token = null_mut();
    unsafe { rts_unlock(token) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
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
#[ignore]
fn equivalent_rts_mkChar(c: HsChar) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkChar(cap, c)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkChar(cap, c) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkChar() {
    let cap = null_mut();
    let c = Default::default();
    unsafe { rts_mkChar(cap, c) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkInt(i: HsInt) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkInt(cap, i)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkInt(cap, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt() {
    let cap = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt(cap, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkInt8(i: HsInt8) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkInt8(cap, i)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkInt8(cap, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt8() {
    let cap = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt8(cap, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkInt16(i: HsInt16) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkInt16(cap, i)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkInt16(cap, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt16() {
    let cap = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt16(cap, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkInt32(i: HsInt32) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkInt32(cap, i)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkInt32(cap, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt32() {
    let cap = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt32(cap, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkInt64(i: HsInt64) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkInt64(cap, i)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkInt64(cap, i) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkInt64() {
    let cap = null_mut();
    let i = Default::default();
    unsafe { rts_mkInt64(cap, i) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkWord(w: HsWord) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkWord(cap, w)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkWord(cap, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord() {
    let cap = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord(cap, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkWord8(w: HsWord8) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkWord8(cap, w)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkWord8(cap, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord8() {
    let cap = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord8(cap, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkWord16(w: HsWord16) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkWord16(cap, w)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkWord16(cap, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord16() {
    let cap = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord16(cap, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkWord32(w: HsWord32) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkWord32(cap, w)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkWord32(cap, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord32() {
    let cap = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord32(cap, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkWord64(w: HsWord64) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkWord64(cap, w)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkWord64(cap, w) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkWord64() {
    let cap = null_mut();
    let w = Default::default();
    unsafe { rts_mkWord64(cap, w) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_mkPtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_mkPtr() {
    let cap = null_mut();
    let a = Default::default();
    unsafe { rts_mkPtr(cap, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_mkFunPtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_mkFunPtr() {
    let cap = null_mut();
    let a = Default::default();
    unsafe { rts_mkFunPtr(cap, a) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkFloat(f: HsFloat) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkFloat(cap, f)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkFloat(cap, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkFloat() {
    let cap = null_mut();
    let f = Default::default();
    unsafe { rts_mkFloat(cap, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkDouble(f: HsDouble) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkDouble(cap, f)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkDouble(cap, f) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkDouble() {
    let cap = null_mut();
    let f = Default::default();
    unsafe { rts_mkDouble(cap, f) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_mkStablePtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_mkStablePtr() {
    let cap = null_mut();
    let s = Default::default();
    unsafe { rts_mkStablePtr(cap, s) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkBool(b: HsBool) -> bool {
    let cap = null_mut();
    let expected = unsafe { transmute(sys::rts_mkBool(cap, b)) };
    let cap = null_mut();
    let actual = unsafe { rts_mkBool(cap, b) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkBool() {
    let cap = null_mut();
    let b = Default::default();
    unsafe { rts_mkBool(cap, b) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_mkString(s: CString) -> bool {
    let cap = null_mut();
    let expected = {
        let s = s.clone();
        unsafe { transmute(sys::rts_mkString(cap, s.as_ptr() as *mut c_char)) }
    };
    let cap = null_mut();
    let actual = unsafe { rts_mkString(cap, s.as_ptr()) };
    actual == expected
}

#[test]
#[ignore]
fn test_rts_mkString() {
    let cap = null_mut();
    let s = CString::new("test_rts_mkString").unwrap();
    unsafe { rts_mkString(cap, s.as_ptr()) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_apply() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_apply() {
    let cap = null_mut();
    let arg2 = null_mut();
    let arg3 = null_mut();
    unsafe { rts_apply(cap, arg2, arg3) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getChar() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getChar() {
    let obj = null_mut();
    unsafe { rts_getChar(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getInt() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getInt() {
    let obj = null_mut();
    unsafe { rts_getInt(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getInt8() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getInt8() {
    let obj = null_mut();
    unsafe { rts_getInt8(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getInt16() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getInt16() {
    let obj = null_mut();
    unsafe { rts_getInt16(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getInt32() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getInt32() {
    let obj = null_mut();
    unsafe { rts_getInt32(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getInt64() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getInt64() {
    let obj = null_mut();
    unsafe { rts_getInt64(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getWord() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getWord() {
    let obj = null_mut();
    unsafe { rts_getWord(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getWord8() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getWord8() {
    let obj = null_mut();
    unsafe { rts_getWord8(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getWord16() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getWord16() {
    let obj = null_mut();
    unsafe { rts_getWord16(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getWord32() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getWord32() {
    let obj = null_mut();
    unsafe { rts_getWord32(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getWord64() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getWord64() {
    let obj = null_mut();
    unsafe { rts_getWord64(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getPtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getPtr() {
    let obj = null_mut();
    unsafe { rts_getPtr(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getFunPtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getFunPtr() {
    let obj = null_mut();
    unsafe { rts_getFunPtr(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getFloat() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getFloat() {
    let obj = null_mut();
    unsafe { rts_getFloat(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getDouble() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getDouble() {
    let obj = null_mut();
    unsafe { rts_getDouble(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getStablePtr() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getStablePtr() {
    let obj = null_mut();
    unsafe { rts_getStablePtr(obj) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getBool() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getBool() {
    let obj = null_mut();
    unsafe { rts_getBool(obj) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval() {
    let cap = null_mut();
    let p = null_mut();
    let mut ret = null_mut();
    unsafe { rts_eval(cap, p, &raw mut ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_eval_() {
    let cap = null_mut();
    let p = null_mut();
    let stack_size = Default::default();
    let ret = null_mut();
    unsafe { rts_eval_(cap, p, stack_size, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalIO() {
    let cap = null_mut();
    let p = null_mut();
    let ret = null_mut();
    unsafe { rts_evalIO(cap, p, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIOMain() {
    let mut cap = null_mut();
    let s = Default::default();
    let ret = null_mut();
    unsafe { rts_evalStableIOMain(&raw mut cap, s, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalStableIO() {
    let cap = null_mut();
    let s = Default::default();
    let ret = null_mut();
    unsafe { rts_evalStableIO(cap, s, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO() {
    let cap = null_mut();
    let p = null_mut();
    let ret = null_mut();
    unsafe { rts_evalLazyIO(cap, p, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_evalLazyIO_() {
    let cap = null_mut();
    let p = null_mut();
    let stack_size = Default::default();
    let ret = null_mut();
    unsafe { rts_evalLazyIO_(cap, p, stack_size, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_inCall() {
    let cap = null_mut();
    let p = null_mut();
    let ret = null_mut();
    unsafe { rts_inCall(cap, p, ret) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_checkSchedStatus() {
    let site = null_mut();
    let cap = null_mut();
    unsafe { rts_checkSchedStatus(site, cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rts_getSchedStatus() {
    todo!()
}

#[test]
#[ignore]
fn test_rts_getSchedStatus() {
    let cap = null_mut();
    unsafe { rts_getSchedStatus(cap) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[ignore]
fn equivalent_rts_pause() {
    let expected = unsafe { transmute(sys::rts_pause()) };
    let actual = unsafe { rts_pause() };
    assert_eq!(actual, expected)
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
    let pauseToken = null_mut();
    unsafe { rts_resume(pauseToken) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_rts_isPaused() -> bool {
    let expected = unsafe { sys::rts_isPaused() };
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
    let cb = None;
    let user = null_mut();
    unsafe { rts_listThreads(cb, user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_rts_listMiscRoots() {
    let cb = None;
    let user = null_mut();
    unsafe { rts_listMiscRoots(cb, user) };
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
