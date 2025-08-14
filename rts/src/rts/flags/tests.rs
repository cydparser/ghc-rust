use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_NO_GC_STATS() {
    assert_eq!(sys::NO_GC_STATS, NO_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COLLECT_GC_STATS() {
    assert_eq!(sys::COLLECT_GC_STATS, COLLECT_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ONELINE_GC_STATS() {
    assert_eq!(sys::ONELINE_GC_STATS, ONELINE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SUMMARY_GC_STATS() {
    assert_eq!(sys::SUMMARY_GC_STATS, SUMMARY_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_VERBOSE_GC_STATS() {
    assert_eq!(sys::VERBOSE_GC_STATS, VERBOSE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COST_CENTRES_NONE() {
    assert_eq!(sys::COST_CENTRES_NONE, COST_CENTRES_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COST_CENTRES_SUMMARY() {
    assert_eq!(sys::COST_CENTRES_SUMMARY, COST_CENTRES_SUMMARY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COST_CENTRES_VERBOSE() {
    assert_eq!(sys::COST_CENTRES_VERBOSE, COST_CENTRES_VERBOSE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COST_CENTRES_ALL() {
    assert_eq!(sys::COST_CENTRES_ALL, COST_CENTRES_ALL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_COST_CENTRES_JSON() {
    assert_eq!(sys::COST_CENTRES_JSON, COST_CENTRES_JSON);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_NO_HEAP_PROFILING() {
    assert_eq!(sys::NO_HEAP_PROFILING, NO_HEAP_PROFILING);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_CCS() {
    assert_eq!(sys::HEAP_BY_CCS, HEAP_BY_CCS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_MOD() {
    assert_eq!(sys::HEAP_BY_MOD, HEAP_BY_MOD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_DESCR() {
    assert_eq!(sys::HEAP_BY_DESCR, HEAP_BY_DESCR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_TYPE() {
    assert_eq!(sys::HEAP_BY_TYPE, HEAP_BY_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_RETAINER() {
    assert_eq!(sys::HEAP_BY_RETAINER, HEAP_BY_RETAINER);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_LDV() {
    assert_eq!(sys::HEAP_BY_LDV, HEAP_BY_LDV);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_CLOSURE_TYPE() {
    assert_eq!(sys::HEAP_BY_CLOSURE_TYPE, HEAP_BY_CLOSURE_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_INFO_TABLE() {
    assert_eq!(sys::HEAP_BY_INFO_TABLE, HEAP_BY_INFO_TABLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_HEAP_BY_ERA() {
    assert_eq!(sys::HEAP_BY_ERA, HEAP_BY_ERA);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TRACE_NONE() {
    assert_eq!(sys::TRACE_NONE, TRACE_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TRACE_EVENTLOG() {
    assert_eq!(sys::TRACE_EVENTLOG, TRACE_EVENTLOG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TRACE_STDERR() {
    assert_eq!(sys::TRACE_STDERR, TRACE_STDERR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_DEFAULT_LINKER_ALWAYS_PIC() {
    assert_eq!(sys::DEFAULT_LINKER_ALWAYS_PIC, DEFAULT_LINKER_ALWAYS_PIC);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STATS_FILENAME_MAXLEN() {
    assert_eq!(sys::STATS_FILENAME_MAXLEN, STATS_FILENAME_MAXLEN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size__GC_FLAGS() {
    assert_eq!(size_of::<sys::_GC_FLAGS>(), size_of::<_GC_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _GC_FLAGS"][size_of::<_GC_FLAGS>() - 192usize];
    ["Alignment of _GC_FLAGS"][align_of::<_GC_FLAGS>() - 8usize];
    ["Offset of field: _GC_FLAGS::statsFile"][offset_of!(_GC_FLAGS, statsFile) - 0usize];
    ["Offset of field: _GC_FLAGS::giveStats"][offset_of!(_GC_FLAGS, giveStats) - 8usize];
    ["Offset of field: _GC_FLAGS::maxStkSize"][offset_of!(_GC_FLAGS, maxStkSize) - 12usize];
    ["Offset of field: _GC_FLAGS::initialStkSize"][offset_of!(_GC_FLAGS, initialStkSize) - 16usize];
    ["Offset of field: _GC_FLAGS::stkChunkSize"][offset_of!(_GC_FLAGS, stkChunkSize) - 20usize];
    ["Offset of field: _GC_FLAGS::stkChunkBufferSize"]
        [offset_of!(_GC_FLAGS, stkChunkBufferSize) - 24usize];
    ["Offset of field: _GC_FLAGS::maxHeapSize"][offset_of!(_GC_FLAGS, maxHeapSize) - 28usize];
    ["Offset of field: _GC_FLAGS::minAllocAreaSize"]
        [offset_of!(_GC_FLAGS, minAllocAreaSize) - 32usize];
    ["Offset of field: _GC_FLAGS::largeAllocLim"][offset_of!(_GC_FLAGS, largeAllocLim) - 36usize];
    ["Offset of field: _GC_FLAGS::nurseryChunkSize"]
        [offset_of!(_GC_FLAGS, nurseryChunkSize) - 40usize];
    ["Offset of field: _GC_FLAGS::minOldGenSize"][offset_of!(_GC_FLAGS, minOldGenSize) - 44usize];
    ["Offset of field: _GC_FLAGS::heapSizeSuggestion"]
        [offset_of!(_GC_FLAGS, heapSizeSuggestion) - 48usize];
    ["Offset of field: _GC_FLAGS::heapSizeSuggestionAuto"]
        [offset_of!(_GC_FLAGS, heapSizeSuggestionAuto) - 52usize];
    ["Offset of field: _GC_FLAGS::oldGenFactor"][offset_of!(_GC_FLAGS, oldGenFactor) - 56usize];
    ["Offset of field: _GC_FLAGS::returnDecayFactor"]
        [offset_of!(_GC_FLAGS, returnDecayFactor) - 64usize];
    ["Offset of field: _GC_FLAGS::pcFreeHeap"][offset_of!(_GC_FLAGS, pcFreeHeap) - 72usize];
    ["Offset of field: _GC_FLAGS::useNonmoving"][offset_of!(_GC_FLAGS, useNonmoving) - 80usize];
    ["Offset of field: _GC_FLAGS::nonmovingDenseAllocatorCount"]
        [offset_of!(_GC_FLAGS, nonmovingDenseAllocatorCount) - 82usize];
    ["Offset of field: _GC_FLAGS::generations"][offset_of!(_GC_FLAGS, generations) - 84usize];
    ["Offset of field: _GC_FLAGS::squeezeUpdFrames"]
        [offset_of!(_GC_FLAGS, squeezeUpdFrames) - 88usize];
    ["Offset of field: _GC_FLAGS::compact"][offset_of!(_GC_FLAGS, compact) - 89usize];
    ["Offset of field: _GC_FLAGS::compactThreshold"]
        [offset_of!(_GC_FLAGS, compactThreshold) - 96usize];
    ["Offset of field: _GC_FLAGS::sweep"][offset_of!(_GC_FLAGS, sweep) - 104usize];
    ["Offset of field: _GC_FLAGS::ringBell"][offset_of!(_GC_FLAGS, ringBell) - 105usize];
    ["Offset of field: _GC_FLAGS::idleGCDelayTime"]
        [offset_of!(_GC_FLAGS, idleGCDelayTime) - 112usize];
    ["Offset of field: _GC_FLAGS::interIdleGCWait"]
        [offset_of!(_GC_FLAGS, interIdleGCWait) - 120usize];
    ["Offset of field: _GC_FLAGS::doIdleGC"][offset_of!(_GC_FLAGS, doIdleGC) - 128usize];
    ["Offset of field: _GC_FLAGS::longGCSync"][offset_of!(_GC_FLAGS, longGCSync) - 136usize];
    ["Offset of field: _GC_FLAGS::heapBase"][offset_of!(_GC_FLAGS, heapBase) - 144usize];
    ["Offset of field: _GC_FLAGS::allocLimitGrace"]
        [offset_of!(_GC_FLAGS, allocLimitGrace) - 152usize];
    ["Offset of field: _GC_FLAGS::heapLimitGrace"]
        [offset_of!(_GC_FLAGS, heapLimitGrace) - 160usize];
    ["Offset of field: _GC_FLAGS::numa"][offset_of!(_GC_FLAGS, numa) - 168usize];
    ["Offset of field: _GC_FLAGS::numaMask"][offset_of!(_GC_FLAGS, numaMask) - 176usize];
    ["Offset of field: _GC_FLAGS::addressSpaceSize"]
        [offset_of!(_GC_FLAGS, addressSpaceSize) - 184usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__DEBUG_FLAGS() {
    assert_eq!(size_of::<sys::_DEBUG_FLAGS>(), size_of::<_DEBUG_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _DEBUG_FLAGS"][size_of::<_DEBUG_FLAGS>() - 22usize];
    ["Alignment of _DEBUG_FLAGS"][align_of::<_DEBUG_FLAGS>() - 1usize];
    ["Offset of field: _DEBUG_FLAGS::scheduler"][offset_of!(_DEBUG_FLAGS, scheduler) - 0usize];
    ["Offset of field: _DEBUG_FLAGS::interpreter"][offset_of!(_DEBUG_FLAGS, interpreter) - 1usize];
    ["Offset of field: _DEBUG_FLAGS::weak"][offset_of!(_DEBUG_FLAGS, weak) - 2usize];
    ["Offset of field: _DEBUG_FLAGS::gccafs"][offset_of!(_DEBUG_FLAGS, gccafs) - 3usize];
    ["Offset of field: _DEBUG_FLAGS::gc"][offset_of!(_DEBUG_FLAGS, gc) - 4usize];
    ["Offset of field: _DEBUG_FLAGS::nonmoving_gc"]
        [offset_of!(_DEBUG_FLAGS, nonmoving_gc) - 5usize];
    ["Offset of field: _DEBUG_FLAGS::block_alloc"][offset_of!(_DEBUG_FLAGS, block_alloc) - 6usize];
    ["Offset of field: _DEBUG_FLAGS::sanity"][offset_of!(_DEBUG_FLAGS, sanity) - 7usize];
    ["Offset of field: _DEBUG_FLAGS::zero_on_gc"][offset_of!(_DEBUG_FLAGS, zero_on_gc) - 8usize];
    ["Offset of field: _DEBUG_FLAGS::stable"][offset_of!(_DEBUG_FLAGS, stable) - 9usize];
    ["Offset of field: _DEBUG_FLAGS::prof"][offset_of!(_DEBUG_FLAGS, prof) - 10usize];
    ["Offset of field: _DEBUG_FLAGS::linker"][offset_of!(_DEBUG_FLAGS, linker) - 11usize];
    ["Offset of field: _DEBUG_FLAGS::linker_verbose"]
        [offset_of!(_DEBUG_FLAGS, linker_verbose) - 12usize];
    ["Offset of field: _DEBUG_FLAGS::apply"][offset_of!(_DEBUG_FLAGS, apply) - 13usize];
    ["Offset of field: _DEBUG_FLAGS::stm"][offset_of!(_DEBUG_FLAGS, stm) - 14usize];
    ["Offset of field: _DEBUG_FLAGS::squeeze"][offset_of!(_DEBUG_FLAGS, squeeze) - 15usize];
    ["Offset of field: _DEBUG_FLAGS::hpc"][offset_of!(_DEBUG_FLAGS, hpc) - 16usize];
    ["Offset of field: _DEBUG_FLAGS::sparks"][offset_of!(_DEBUG_FLAGS, sparks) - 17usize];
    ["Offset of field: _DEBUG_FLAGS::numa"][offset_of!(_DEBUG_FLAGS, numa) - 18usize];
    ["Offset of field: _DEBUG_FLAGS::compact"][offset_of!(_DEBUG_FLAGS, compact) - 19usize];
    ["Offset of field: _DEBUG_FLAGS::continuation"]
        [offset_of!(_DEBUG_FLAGS, continuation) - 20usize];
    ["Offset of field: _DEBUG_FLAGS::iomanager"][offset_of!(_DEBUG_FLAGS, iomanager) - 21usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__COST_CENTRE_FLAGS() {
    assert_eq!(
        size_of::<sys::_COST_CENTRE_FLAGS>(),
        size_of::<_COST_CENTRE_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _COST_CENTRE_FLAGS"][size_of::<_COST_CENTRE_FLAGS>() - 24usize];
    ["Alignment of _COST_CENTRE_FLAGS"][align_of::<_COST_CENTRE_FLAGS>() - 8usize];
    ["Offset of field: _COST_CENTRE_FLAGS::doCostCentres"]
        [offset_of!(_COST_CENTRE_FLAGS, doCostCentres) - 0usize];
    ["Offset of field: _COST_CENTRE_FLAGS::profilerTicks"]
        [offset_of!(_COST_CENTRE_FLAGS, profilerTicks) - 4usize];
    ["Offset of field: _COST_CENTRE_FLAGS::msecsPerTick"]
        [offset_of!(_COST_CENTRE_FLAGS, msecsPerTick) - 8usize];
    ["Offset of field: _COST_CENTRE_FLAGS::outputFileNameStem"]
        [offset_of!(_COST_CENTRE_FLAGS, outputFileNameStem) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__PROFILING_FLAGS() {
    assert_eq!(
        size_of::<sys::_PROFILING_FLAGS>(),
        size_of::<_PROFILING_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _PROFILING_FLAGS"][size_of::<_PROFILING_FLAGS>() - 96usize];
    ["Alignment of _PROFILING_FLAGS"][align_of::<_PROFILING_FLAGS>() - 8usize];
    ["Offset of field: _PROFILING_FLAGS::doHeapProfile"]
        [offset_of!(_PROFILING_FLAGS, doHeapProfile) - 0usize];
    ["Offset of field: _PROFILING_FLAGS::heapProfileInterval"]
        [offset_of!(_PROFILING_FLAGS, heapProfileInterval) - 8usize];
    ["Offset of field: _PROFILING_FLAGS::heapProfileIntervalTicks"]
        [offset_of!(_PROFILING_FLAGS, heapProfileIntervalTicks) - 16usize];
    ["Offset of field: _PROFILING_FLAGS::startHeapProfileAtStartup"]
        [offset_of!(_PROFILING_FLAGS, startHeapProfileAtStartup) - 20usize];
    ["Offset of field: _PROFILING_FLAGS::startTimeProfileAtStartup"]
        [offset_of!(_PROFILING_FLAGS, startTimeProfileAtStartup) - 21usize];
    ["Offset of field: _PROFILING_FLAGS::incrementUserEra"]
        [offset_of!(_PROFILING_FLAGS, incrementUserEra) - 22usize];
    ["Offset of field: _PROFILING_FLAGS::showCCSOnException"]
        [offset_of!(_PROFILING_FLAGS, showCCSOnException) - 23usize];
    ["Offset of field: _PROFILING_FLAGS::maxRetainerSetSize"]
        [offset_of!(_PROFILING_FLAGS, maxRetainerSetSize) - 24usize];
    ["Offset of field: _PROFILING_FLAGS::ccsLength"]
        [offset_of!(_PROFILING_FLAGS, ccsLength) - 28usize];
    ["Offset of field: _PROFILING_FLAGS::modSelector"]
        [offset_of!(_PROFILING_FLAGS, modSelector) - 32usize];
    ["Offset of field: _PROFILING_FLAGS::descrSelector"]
        [offset_of!(_PROFILING_FLAGS, descrSelector) - 40usize];
    ["Offset of field: _PROFILING_FLAGS::typeSelector"]
        [offset_of!(_PROFILING_FLAGS, typeSelector) - 48usize];
    ["Offset of field: _PROFILING_FLAGS::ccSelector"]
        [offset_of!(_PROFILING_FLAGS, ccSelector) - 56usize];
    ["Offset of field: _PROFILING_FLAGS::ccsSelector"]
        [offset_of!(_PROFILING_FLAGS, ccsSelector) - 64usize];
    ["Offset of field: _PROFILING_FLAGS::retainerSelector"]
        [offset_of!(_PROFILING_FLAGS, retainerSelector) - 72usize];
    ["Offset of field: _PROFILING_FLAGS::eraSelector"]
        [offset_of!(_PROFILING_FLAGS, eraSelector) - 80usize];
    ["Offset of field: _PROFILING_FLAGS::bioSelector"]
        [offset_of!(_PROFILING_FLAGS, bioSelector) - 88usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__TRACE_FLAGS() {
    assert_eq!(size_of::<sys::_TRACE_FLAGS>(), size_of::<_TRACE_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _TRACE_FLAGS"][size_of::<_TRACE_FLAGS>() - 48usize];
    ["Alignment of _TRACE_FLAGS"][align_of::<_TRACE_FLAGS>() - 8usize];
    ["Offset of field: _TRACE_FLAGS::tracing"][offset_of!(_TRACE_FLAGS, tracing) - 0usize];
    ["Offset of field: _TRACE_FLAGS::timestamp"][offset_of!(_TRACE_FLAGS, timestamp) - 4usize];
    ["Offset of field: _TRACE_FLAGS::scheduler"][offset_of!(_TRACE_FLAGS, scheduler) - 5usize];
    ["Offset of field: _TRACE_FLAGS::gc"][offset_of!(_TRACE_FLAGS, gc) - 6usize];
    ["Offset of field: _TRACE_FLAGS::nonmoving_gc"]
        [offset_of!(_TRACE_FLAGS, nonmoving_gc) - 7usize];
    ["Offset of field: _TRACE_FLAGS::sparks_sampled"]
        [offset_of!(_TRACE_FLAGS, sparks_sampled) - 8usize];
    ["Offset of field: _TRACE_FLAGS::sparks_full"][offset_of!(_TRACE_FLAGS, sparks_full) - 9usize];
    ["Offset of field: _TRACE_FLAGS::ticky"][offset_of!(_TRACE_FLAGS, ticky) - 10usize];
    ["Offset of field: _TRACE_FLAGS::user"][offset_of!(_TRACE_FLAGS, user) - 11usize];
    ["Offset of field: _TRACE_FLAGS::eventlogFlushTime"]
        [offset_of!(_TRACE_FLAGS, eventlogFlushTime) - 16usize];
    ["Offset of field: _TRACE_FLAGS::eventlogFlushTicks"]
        [offset_of!(_TRACE_FLAGS, eventlogFlushTicks) - 24usize];
    ["Offset of field: _TRACE_FLAGS::trace_output"]
        [offset_of!(_TRACE_FLAGS, trace_output) - 32usize];
    ["Offset of field: _TRACE_FLAGS::nullWriter"][offset_of!(_TRACE_FLAGS, nullWriter) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__CONCURRENT_FLAGS() {
    assert_eq!(
        size_of::<sys::_CONCURRENT_FLAGS>(),
        size_of::<_CONCURRENT_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _CONCURRENT_FLAGS"][size_of::<_CONCURRENT_FLAGS>() - 16usize];
    ["Alignment of _CONCURRENT_FLAGS"][align_of::<_CONCURRENT_FLAGS>() - 8usize];
    ["Offset of field: _CONCURRENT_FLAGS::ctxtSwitchTime"]
        [offset_of!(_CONCURRENT_FLAGS, ctxtSwitchTime) - 0usize];
    ["Offset of field: _CONCURRENT_FLAGS::ctxtSwitchTicks"]
        [offset_of!(_CONCURRENT_FLAGS, ctxtSwitchTicks) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__MISC_FLAGS() {
    assert_eq!(size_of::<sys::_MISC_FLAGS>(), size_of::<_MISC_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _MISC_FLAGS"][size_of::<_MISC_FLAGS>() - 32usize];
    ["Alignment of _MISC_FLAGS"][align_of::<_MISC_FLAGS>() - 8usize];
    ["Offset of field: _MISC_FLAGS::tickInterval"][offset_of!(_MISC_FLAGS, tickInterval) - 0usize];
    ["Offset of field: _MISC_FLAGS::install_signal_handlers"]
        [offset_of!(_MISC_FLAGS, install_signal_handlers) - 8usize];
    ["Offset of field: _MISC_FLAGS::install_seh_handlers"]
        [offset_of!(_MISC_FLAGS, install_seh_handlers) - 9usize];
    ["Offset of field: _MISC_FLAGS::generate_dump_file"]
        [offset_of!(_MISC_FLAGS, generate_dump_file) - 10usize];
    ["Offset of field: _MISC_FLAGS::generate_stack_trace"]
        [offset_of!(_MISC_FLAGS, generate_stack_trace) - 11usize];
    ["Offset of field: _MISC_FLAGS::machineReadable"]
        [offset_of!(_MISC_FLAGS, machineReadable) - 12usize];
    ["Offset of field: _MISC_FLAGS::disableDelayedOsMemoryReturn"]
        [offset_of!(_MISC_FLAGS, disableDelayedOsMemoryReturn) - 13usize];
    ["Offset of field: _MISC_FLAGS::internalCounters"]
        [offset_of!(_MISC_FLAGS, internalCounters) - 14usize];
    ["Offset of field: _MISC_FLAGS::linkerAlwaysPic"]
        [offset_of!(_MISC_FLAGS, linkerAlwaysPic) - 15usize];
    ["Offset of field: _MISC_FLAGS::linkerMemBase"]
        [offset_of!(_MISC_FLAGS, linkerMemBase) - 16usize];
    ["Offset of field: _MISC_FLAGS::ioManager"][offset_of!(_MISC_FLAGS, ioManager) - 24usize];
    ["Offset of field: _MISC_FLAGS::numIoWorkerThreads"]
        [offset_of!(_MISC_FLAGS, numIoWorkerThreads) - 28usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__PAR_FLAGS() {
    assert_eq!(size_of::<sys::_PAR_FLAGS>(), size_of::<_PAR_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _PAR_FLAGS"][size_of::<_PAR_FLAGS>() - 40usize];
    ["Alignment of _PAR_FLAGS"][align_of::<_PAR_FLAGS>() - 4usize];
    ["Offset of field: _PAR_FLAGS::nCapabilities"][offset_of!(_PAR_FLAGS, nCapabilities) - 0usize];
    ["Offset of field: _PAR_FLAGS::migrate"][offset_of!(_PAR_FLAGS, migrate) - 4usize];
    ["Offset of field: _PAR_FLAGS::maxLocalSparks"]
        [offset_of!(_PAR_FLAGS, maxLocalSparks) - 8usize];
    ["Offset of field: _PAR_FLAGS::parGcEnabled"][offset_of!(_PAR_FLAGS, parGcEnabled) - 12usize];
    ["Offset of field: _PAR_FLAGS::parGcGen"][offset_of!(_PAR_FLAGS, parGcGen) - 16usize];
    ["Offset of field: _PAR_FLAGS::parGcLoadBalancingEnabled"]
        [offset_of!(_PAR_FLAGS, parGcLoadBalancingEnabled) - 20usize];
    ["Offset of field: _PAR_FLAGS::parGcLoadBalancingGen"]
        [offset_of!(_PAR_FLAGS, parGcLoadBalancingGen) - 24usize];
    ["Offset of field: _PAR_FLAGS::parGcNoSyncWithIdle"]
        [offset_of!(_PAR_FLAGS, parGcNoSyncWithIdle) - 28usize];
    ["Offset of field: _PAR_FLAGS::parGcThreads"][offset_of!(_PAR_FLAGS, parGcThreads) - 32usize];
    ["Offset of field: _PAR_FLAGS::setAffinity"][offset_of!(_PAR_FLAGS, setAffinity) - 36usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__HPC_FLAGS() {
    assert_eq!(size_of::<sys::_HPC_FLAGS>(), size_of::<_HPC_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _HPC_FLAGS"][size_of::<_HPC_FLAGS>() - 8usize];
    ["Alignment of _HPC_FLAGS"][align_of::<_HPC_FLAGS>() - 4usize];
    ["Offset of field: _HPC_FLAGS::writeTixFile"][offset_of!(_HPC_FLAGS, writeTixFile) - 0usize];
    ["Offset of field: _HPC_FLAGS::readTixFile"][offset_of!(_HPC_FLAGS, readTixFile) - 4usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__TICKY_FLAGS() {
    assert_eq!(size_of::<sys::_TICKY_FLAGS>(), size_of::<_TICKY_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _TICKY_FLAGS"][size_of::<_TICKY_FLAGS>() - 16usize];
    ["Alignment of _TICKY_FLAGS"][align_of::<_TICKY_FLAGS>() - 8usize];
    ["Offset of field: _TICKY_FLAGS::showTickyStats"]
        [offset_of!(_TICKY_FLAGS, showTickyStats) - 0usize];
    ["Offset of field: _TICKY_FLAGS::tickyFile"][offset_of!(_TICKY_FLAGS, tickyFile) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__RTS_FLAGS() {
    assert_eq!(size_of::<sys::_RTS_FLAGS>(), size_of::<_RTS_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _RTS_FLAGS"][size_of::<_RTS_FLAGS>() - 496usize];
    ["Alignment of _RTS_FLAGS"][align_of::<_RTS_FLAGS>() - 8usize];
    ["Offset of field: _RTS_FLAGS::GcFlags"][offset_of!(_RTS_FLAGS, GcFlags) - 0usize];
    ["Offset of field: _RTS_FLAGS::ConcFlags"][offset_of!(_RTS_FLAGS, ConcFlags) - 192usize];
    ["Offset of field: _RTS_FLAGS::MiscFlags"][offset_of!(_RTS_FLAGS, MiscFlags) - 208usize];
    ["Offset of field: _RTS_FLAGS::DebugFlags"][offset_of!(_RTS_FLAGS, DebugFlags) - 240usize];
    ["Offset of field: _RTS_FLAGS::CcFlags"][offset_of!(_RTS_FLAGS, CcFlags) - 264usize];
    ["Offset of field: _RTS_FLAGS::ProfFlags"][offset_of!(_RTS_FLAGS, ProfFlags) - 288usize];
    ["Offset of field: _RTS_FLAGS::TraceFlags"][offset_of!(_RTS_FLAGS, TraceFlags) - 384usize];
    ["Offset of field: _RTS_FLAGS::TickyFlags"][offset_of!(_RTS_FLAGS, TickyFlags) - 432usize];
    ["Offset of field: _RTS_FLAGS::ParFlags"][offset_of!(_RTS_FLAGS, ParFlags) - 448usize];
    ["Offset of field: _RTS_FLAGS::HpcFlags"][offset_of!(_RTS_FLAGS, HpcFlags) - 488usize];
};
