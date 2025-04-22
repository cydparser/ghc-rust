use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_NO_GC_STATS() {
    assert_eq!(sys::NO_GC_STATS, super::NO_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COLLECT_GC_STATS() {
    assert_eq!(sys::COLLECT_GC_STATS, super::COLLECT_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ONELINE_GC_STATS() {
    assert_eq!(sys::ONELINE_GC_STATS, super::ONELINE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SUMMARY_GC_STATS() {
    assert_eq!(sys::SUMMARY_GC_STATS, super::SUMMARY_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_VERBOSE_GC_STATS() {
    assert_eq!(sys::VERBOSE_GC_STATS, super::VERBOSE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COST_CENTRES_NONE() {
    assert_eq!(sys::COST_CENTRES_NONE, super::COST_CENTRES_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COST_CENTRES_SUMMARY() {
    assert_eq!(sys::COST_CENTRES_SUMMARY, super::COST_CENTRES_SUMMARY);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COST_CENTRES_VERBOSE() {
    assert_eq!(sys::COST_CENTRES_VERBOSE, super::COST_CENTRES_VERBOSE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COST_CENTRES_ALL() {
    assert_eq!(sys::COST_CENTRES_ALL, super::COST_CENTRES_ALL);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_COST_CENTRES_JSON() {
    assert_eq!(sys::COST_CENTRES_JSON, super::COST_CENTRES_JSON);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_NO_HEAP_PROFILING() {
    assert_eq!(sys::NO_HEAP_PROFILING, super::NO_HEAP_PROFILING);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_CCS() {
    assert_eq!(sys::HEAP_BY_CCS, super::HEAP_BY_CCS);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_MOD() {
    assert_eq!(sys::HEAP_BY_MOD, super::HEAP_BY_MOD);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_DESCR() {
    assert_eq!(sys::HEAP_BY_DESCR, super::HEAP_BY_DESCR);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_TYPE() {
    assert_eq!(sys::HEAP_BY_TYPE, super::HEAP_BY_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_RETAINER() {
    assert_eq!(sys::HEAP_BY_RETAINER, super::HEAP_BY_RETAINER);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_LDV() {
    assert_eq!(sys::HEAP_BY_LDV, super::HEAP_BY_LDV);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_CLOSURE_TYPE() {
    assert_eq!(sys::HEAP_BY_CLOSURE_TYPE, super::HEAP_BY_CLOSURE_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_INFO_TABLE() {
    assert_eq!(sys::HEAP_BY_INFO_TABLE, super::HEAP_BY_INFO_TABLE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HEAP_BY_ERA() {
    assert_eq!(sys::HEAP_BY_ERA, super::HEAP_BY_ERA);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TRACE_NONE() {
    assert_eq!(sys::TRACE_NONE, super::TRACE_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TRACE_EVENTLOG() {
    assert_eq!(sys::TRACE_EVENTLOG, super::TRACE_EVENTLOG);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TRACE_STDERR() {
    assert_eq!(sys::TRACE_STDERR, super::TRACE_STDERR);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_DEFAULT_LINKER_ALWAYS_PIC() {
    assert_eq!(
        sys::DEFAULT_LINKER_ALWAYS_PIC,
        super::DEFAULT_LINKER_ALWAYS_PIC
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STATS_FILENAME_MAXLEN() {
    assert_eq!(sys::STATS_FILENAME_MAXLEN, super::STATS_FILENAME_MAXLEN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_GR_FILENAME_FMT() {
    assert_eq!(sys::GR_FILENAME_FMT, super::GR_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HP_FILENAME_FMT() {
    assert_eq!(sys::HP_FILENAME_FMT, super::HP_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_LIFE_FILENAME_FMT() {
    assert_eq!(sys::LIFE_FILENAME_FMT, super::LIFE_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_PROF_FILENAME_FMT() {
    assert_eq!(sys::PROF_FILENAME_FMT, super::PROF_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_PROF_FILENAME_FMT_GUM() {
    assert_eq!(sys::PROF_FILENAME_FMT_GUM, super::PROF_FILENAME_FMT_GUM);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_QP_FILENAME_FMT() {
    assert_eq!(sys::QP_FILENAME_FMT, super::QP_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STAT_FILENAME_FMT() {
    assert_eq!(sys::STAT_FILENAME_FMT, super::STAT_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TICKY_FILENAME_FMT() {
    assert_eq!(sys::TICKY_FILENAME_FMT, super::TICKY_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TIME_FILENAME_FMT() {
    assert_eq!(sys::TIME_FILENAME_FMT, super::TIME_FILENAME_FMT);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TIME_FILENAME_FMT_GUM() {
    assert_eq!(sys::TIME_FILENAME_FMT_GUM, super::TIME_FILENAME_FMT_GUM);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of__GC_FLAGS() {
    assert_eq!(size_of::<sys::_GC_FLAGS>(), size_of::<super::_GC_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _GC_FLAGS"][::core::mem::size_of::<_GC_FLAGS>() - 192usize];
    ["Alignment of _GC_FLAGS"][::core::mem::align_of::<_GC_FLAGS>() - 8usize];
    ["Offset of field: _GC_FLAGS::statsFile"]
        [::core::mem::offset_of!(_GC_FLAGS, statsFile) - 0usize];
    ["Offset of field: _GC_FLAGS::giveStats"]
        [::core::mem::offset_of!(_GC_FLAGS, giveStats) - 8usize];
    ["Offset of field: _GC_FLAGS::maxStkSize"]
        [::core::mem::offset_of!(_GC_FLAGS, maxStkSize) - 12usize];
    ["Offset of field: _GC_FLAGS::initialStkSize"]
        [::core::mem::offset_of!(_GC_FLAGS, initialStkSize) - 16usize];
    ["Offset of field: _GC_FLAGS::stkChunkSize"]
        [::core::mem::offset_of!(_GC_FLAGS, stkChunkSize) - 20usize];
    ["Offset of field: _GC_FLAGS::stkChunkBufferSize"]
        [::core::mem::offset_of!(_GC_FLAGS, stkChunkBufferSize) - 24usize];
    ["Offset of field: _GC_FLAGS::maxHeapSize"]
        [::core::mem::offset_of!(_GC_FLAGS, maxHeapSize) - 28usize];
    ["Offset of field: _GC_FLAGS::minAllocAreaSize"]
        [::core::mem::offset_of!(_GC_FLAGS, minAllocAreaSize) - 32usize];
    ["Offset of field: _GC_FLAGS::largeAllocLim"]
        [::core::mem::offset_of!(_GC_FLAGS, largeAllocLim) - 36usize];
    ["Offset of field: _GC_FLAGS::nurseryChunkSize"]
        [::core::mem::offset_of!(_GC_FLAGS, nurseryChunkSize) - 40usize];
    ["Offset of field: _GC_FLAGS::minOldGenSize"]
        [::core::mem::offset_of!(_GC_FLAGS, minOldGenSize) - 44usize];
    ["Offset of field: _GC_FLAGS::heapSizeSuggestion"]
        [::core::mem::offset_of!(_GC_FLAGS, heapSizeSuggestion) - 48usize];
    ["Offset of field: _GC_FLAGS::heapSizeSuggestionAuto"]
        [::core::mem::offset_of!(_GC_FLAGS, heapSizeSuggestionAuto) - 52usize];
    ["Offset of field: _GC_FLAGS::oldGenFactor"]
        [::core::mem::offset_of!(_GC_FLAGS, oldGenFactor) - 56usize];
    ["Offset of field: _GC_FLAGS::returnDecayFactor"]
        [::core::mem::offset_of!(_GC_FLAGS, returnDecayFactor) - 64usize];
    ["Offset of field: _GC_FLAGS::pcFreeHeap"]
        [::core::mem::offset_of!(_GC_FLAGS, pcFreeHeap) - 72usize];
    ["Offset of field: _GC_FLAGS::useNonmoving"]
        [::core::mem::offset_of!(_GC_FLAGS, useNonmoving) - 80usize];
    ["Offset of field: _GC_FLAGS::nonmovingDenseAllocatorCount"]
        [::core::mem::offset_of!(_GC_FLAGS, nonmovingDenseAllocatorCount) - 82usize];
    ["Offset of field: _GC_FLAGS::generations"]
        [::core::mem::offset_of!(_GC_FLAGS, generations) - 84usize];
    ["Offset of field: _GC_FLAGS::squeezeUpdFrames"]
        [::core::mem::offset_of!(_GC_FLAGS, squeezeUpdFrames) - 88usize];
    ["Offset of field: _GC_FLAGS::compact"][::core::mem::offset_of!(_GC_FLAGS, compact) - 89usize];
    ["Offset of field: _GC_FLAGS::compactThreshold"]
        [::core::mem::offset_of!(_GC_FLAGS, compactThreshold) - 96usize];
    ["Offset of field: _GC_FLAGS::sweep"][::core::mem::offset_of!(_GC_FLAGS, sweep) - 104usize];
    ["Offset of field: _GC_FLAGS::ringBell"]
        [::core::mem::offset_of!(_GC_FLAGS, ringBell) - 105usize];
    ["Offset of field: _GC_FLAGS::idleGCDelayTime"]
        [::core::mem::offset_of!(_GC_FLAGS, idleGCDelayTime) - 112usize];
    ["Offset of field: _GC_FLAGS::interIdleGCWait"]
        [::core::mem::offset_of!(_GC_FLAGS, interIdleGCWait) - 120usize];
    ["Offset of field: _GC_FLAGS::doIdleGC"]
        [::core::mem::offset_of!(_GC_FLAGS, doIdleGC) - 128usize];
    ["Offset of field: _GC_FLAGS::longGCSync"]
        [::core::mem::offset_of!(_GC_FLAGS, longGCSync) - 136usize];
    ["Offset of field: _GC_FLAGS::heapBase"]
        [::core::mem::offset_of!(_GC_FLAGS, heapBase) - 144usize];
    ["Offset of field: _GC_FLAGS::allocLimitGrace"]
        [::core::mem::offset_of!(_GC_FLAGS, allocLimitGrace) - 152usize];
    ["Offset of field: _GC_FLAGS::heapLimitGrace"]
        [::core::mem::offset_of!(_GC_FLAGS, heapLimitGrace) - 160usize];
    ["Offset of field: _GC_FLAGS::numa"][::core::mem::offset_of!(_GC_FLAGS, numa) - 168usize];
    ["Offset of field: _GC_FLAGS::numaMask"]
        [::core::mem::offset_of!(_GC_FLAGS, numaMask) - 176usize];
    ["Offset of field: _GC_FLAGS::addressSpaceSize"]
        [::core::mem::offset_of!(_GC_FLAGS, addressSpaceSize) - 184usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__DEBUG_FLAGS() {
    assert_eq!(
        size_of::<sys::_DEBUG_FLAGS>(),
        size_of::<super::_DEBUG_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _DEBUG_FLAGS"][::core::mem::size_of::<_DEBUG_FLAGS>() - 22usize];
    ["Alignment of _DEBUG_FLAGS"][::core::mem::align_of::<_DEBUG_FLAGS>() - 1usize];
    ["Offset of field: _DEBUG_FLAGS::scheduler"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, scheduler) - 0usize];
    ["Offset of field: _DEBUG_FLAGS::interpreter"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, interpreter) - 1usize];
    ["Offset of field: _DEBUG_FLAGS::weak"][::core::mem::offset_of!(_DEBUG_FLAGS, weak) - 2usize];
    ["Offset of field: _DEBUG_FLAGS::gccafs"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, gccafs) - 3usize];
    ["Offset of field: _DEBUG_FLAGS::gc"][::core::mem::offset_of!(_DEBUG_FLAGS, gc) - 4usize];
    ["Offset of field: _DEBUG_FLAGS::nonmoving_gc"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, nonmoving_gc) - 5usize];
    ["Offset of field: _DEBUG_FLAGS::block_alloc"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, block_alloc) - 6usize];
    ["Offset of field: _DEBUG_FLAGS::sanity"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, sanity) - 7usize];
    ["Offset of field: _DEBUG_FLAGS::zero_on_gc"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, zero_on_gc) - 8usize];
    ["Offset of field: _DEBUG_FLAGS::stable"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, stable) - 9usize];
    ["Offset of field: _DEBUG_FLAGS::prof"][::core::mem::offset_of!(_DEBUG_FLAGS, prof) - 10usize];
    ["Offset of field: _DEBUG_FLAGS::linker"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, linker) - 11usize];
    ["Offset of field: _DEBUG_FLAGS::linker_verbose"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, linker_verbose) - 12usize];
    ["Offset of field: _DEBUG_FLAGS::apply"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, apply) - 13usize];
    ["Offset of field: _DEBUG_FLAGS::stm"][::core::mem::offset_of!(_DEBUG_FLAGS, stm) - 14usize];
    ["Offset of field: _DEBUG_FLAGS::squeeze"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, squeeze) - 15usize];
    ["Offset of field: _DEBUG_FLAGS::hpc"][::core::mem::offset_of!(_DEBUG_FLAGS, hpc) - 16usize];
    ["Offset of field: _DEBUG_FLAGS::sparks"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, sparks) - 17usize];
    ["Offset of field: _DEBUG_FLAGS::numa"][::core::mem::offset_of!(_DEBUG_FLAGS, numa) - 18usize];
    ["Offset of field: _DEBUG_FLAGS::compact"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, compact) - 19usize];
    ["Offset of field: _DEBUG_FLAGS::continuation"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, continuation) - 20usize];
    ["Offset of field: _DEBUG_FLAGS::iomanager"]
        [::core::mem::offset_of!(_DEBUG_FLAGS, iomanager) - 21usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__COST_CENTRE_FLAGS() {
    assert_eq!(
        size_of::<sys::_COST_CENTRE_FLAGS>(),
        size_of::<super::_COST_CENTRE_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _COST_CENTRE_FLAGS"][::core::mem::size_of::<_COST_CENTRE_FLAGS>() - 24usize];
    ["Alignment of _COST_CENTRE_FLAGS"][::core::mem::align_of::<_COST_CENTRE_FLAGS>() - 8usize];
    ["Offset of field: _COST_CENTRE_FLAGS::doCostCentres"]
        [::core::mem::offset_of!(_COST_CENTRE_FLAGS, doCostCentres) - 0usize];
    ["Offset of field: _COST_CENTRE_FLAGS::profilerTicks"]
        [::core::mem::offset_of!(_COST_CENTRE_FLAGS, profilerTicks) - 4usize];
    ["Offset of field: _COST_CENTRE_FLAGS::msecsPerTick"]
        [::core::mem::offset_of!(_COST_CENTRE_FLAGS, msecsPerTick) - 8usize];
    ["Offset of field: _COST_CENTRE_FLAGS::outputFileNameStem"]
        [::core::mem::offset_of!(_COST_CENTRE_FLAGS, outputFileNameStem) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__PROFILING_FLAGS() {
    assert_eq!(
        size_of::<sys::_PROFILING_FLAGS>(),
        size_of::<super::_PROFILING_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _PROFILING_FLAGS"][::core::mem::size_of::<_PROFILING_FLAGS>() - 96usize];
    ["Alignment of _PROFILING_FLAGS"][::core::mem::align_of::<_PROFILING_FLAGS>() - 8usize];
    ["Offset of field: _PROFILING_FLAGS::doHeapProfile"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, doHeapProfile) - 0usize];
    ["Offset of field: _PROFILING_FLAGS::heapProfileInterval"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, heapProfileInterval) - 8usize];
    ["Offset of field: _PROFILING_FLAGS::heapProfileIntervalTicks"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, heapProfileIntervalTicks) - 16usize];
    ["Offset of field: _PROFILING_FLAGS::startHeapProfileAtStartup"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, startHeapProfileAtStartup) - 20usize];
    ["Offset of field: _PROFILING_FLAGS::startTimeProfileAtStartup"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, startTimeProfileAtStartup) - 21usize];
    ["Offset of field: _PROFILING_FLAGS::incrementUserEra"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, incrementUserEra) - 22usize];
    ["Offset of field: _PROFILING_FLAGS::showCCSOnException"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, showCCSOnException) - 23usize];
    ["Offset of field: _PROFILING_FLAGS::maxRetainerSetSize"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, maxRetainerSetSize) - 24usize];
    ["Offset of field: _PROFILING_FLAGS::ccsLength"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, ccsLength) - 28usize];
    ["Offset of field: _PROFILING_FLAGS::modSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, modSelector) - 32usize];
    ["Offset of field: _PROFILING_FLAGS::descrSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, descrSelector) - 40usize];
    ["Offset of field: _PROFILING_FLAGS::typeSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, typeSelector) - 48usize];
    ["Offset of field: _PROFILING_FLAGS::ccSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, ccSelector) - 56usize];
    ["Offset of field: _PROFILING_FLAGS::ccsSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, ccsSelector) - 64usize];
    ["Offset of field: _PROFILING_FLAGS::retainerSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, retainerSelector) - 72usize];
    ["Offset of field: _PROFILING_FLAGS::eraSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, eraSelector) - 80usize];
    ["Offset of field: _PROFILING_FLAGS::bioSelector"]
        [::core::mem::offset_of!(_PROFILING_FLAGS, bioSelector) - 88usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__TRACE_FLAGS() {
    assert_eq!(
        size_of::<sys::_TRACE_FLAGS>(),
        size_of::<super::_TRACE_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _TRACE_FLAGS"][::core::mem::size_of::<_TRACE_FLAGS>() - 48usize];
    ["Alignment of _TRACE_FLAGS"][::core::mem::align_of::<_TRACE_FLAGS>() - 8usize];
    ["Offset of field: _TRACE_FLAGS::tracing"]
        [::core::mem::offset_of!(_TRACE_FLAGS, tracing) - 0usize];
    ["Offset of field: _TRACE_FLAGS::timestamp"]
        [::core::mem::offset_of!(_TRACE_FLAGS, timestamp) - 4usize];
    ["Offset of field: _TRACE_FLAGS::scheduler"]
        [::core::mem::offset_of!(_TRACE_FLAGS, scheduler) - 5usize];
    ["Offset of field: _TRACE_FLAGS::gc"][::core::mem::offset_of!(_TRACE_FLAGS, gc) - 6usize];
    ["Offset of field: _TRACE_FLAGS::nonmoving_gc"]
        [::core::mem::offset_of!(_TRACE_FLAGS, nonmoving_gc) - 7usize];
    ["Offset of field: _TRACE_FLAGS::sparks_sampled"]
        [::core::mem::offset_of!(_TRACE_FLAGS, sparks_sampled) - 8usize];
    ["Offset of field: _TRACE_FLAGS::sparks_full"]
        [::core::mem::offset_of!(_TRACE_FLAGS, sparks_full) - 9usize];
    ["Offset of field: _TRACE_FLAGS::ticky"]
        [::core::mem::offset_of!(_TRACE_FLAGS, ticky) - 10usize];
    ["Offset of field: _TRACE_FLAGS::user"][::core::mem::offset_of!(_TRACE_FLAGS, user) - 11usize];
    ["Offset of field: _TRACE_FLAGS::eventlogFlushTime"]
        [::core::mem::offset_of!(_TRACE_FLAGS, eventlogFlushTime) - 16usize];
    ["Offset of field: _TRACE_FLAGS::eventlogFlushTicks"]
        [::core::mem::offset_of!(_TRACE_FLAGS, eventlogFlushTicks) - 24usize];
    ["Offset of field: _TRACE_FLAGS::trace_output"]
        [::core::mem::offset_of!(_TRACE_FLAGS, trace_output) - 32usize];
    ["Offset of field: _TRACE_FLAGS::nullWriter"]
        [::core::mem::offset_of!(_TRACE_FLAGS, nullWriter) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__CONCURRENT_FLAGS() {
    assert_eq!(
        size_of::<sys::_CONCURRENT_FLAGS>(),
        size_of::<super::_CONCURRENT_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _CONCURRENT_FLAGS"][::core::mem::size_of::<_CONCURRENT_FLAGS>() - 16usize];
    ["Alignment of _CONCURRENT_FLAGS"][::core::mem::align_of::<_CONCURRENT_FLAGS>() - 8usize];
    ["Offset of field: _CONCURRENT_FLAGS::ctxtSwitchTime"]
        [::core::mem::offset_of!(_CONCURRENT_FLAGS, ctxtSwitchTime) - 0usize];
    ["Offset of field: _CONCURRENT_FLAGS::ctxtSwitchTicks"]
        [::core::mem::offset_of!(_CONCURRENT_FLAGS, ctxtSwitchTicks) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__MISC_FLAGS() {
    assert_eq!(
        size_of::<sys::_MISC_FLAGS>(),
        size_of::<super::_MISC_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _MISC_FLAGS"][::core::mem::size_of::<_MISC_FLAGS>() - 32usize];
    ["Alignment of _MISC_FLAGS"][::core::mem::align_of::<_MISC_FLAGS>() - 8usize];
    ["Offset of field: _MISC_FLAGS::tickInterval"]
        [::core::mem::offset_of!(_MISC_FLAGS, tickInterval) - 0usize];
    ["Offset of field: _MISC_FLAGS::install_signal_handlers"]
        [::core::mem::offset_of!(_MISC_FLAGS, install_signal_handlers) - 8usize];
    ["Offset of field: _MISC_FLAGS::install_seh_handlers"]
        [::core::mem::offset_of!(_MISC_FLAGS, install_seh_handlers) - 9usize];
    ["Offset of field: _MISC_FLAGS::generate_dump_file"]
        [::core::mem::offset_of!(_MISC_FLAGS, generate_dump_file) - 10usize];
    ["Offset of field: _MISC_FLAGS::generate_stack_trace"]
        [::core::mem::offset_of!(_MISC_FLAGS, generate_stack_trace) - 11usize];
    ["Offset of field: _MISC_FLAGS::machineReadable"]
        [::core::mem::offset_of!(_MISC_FLAGS, machineReadable) - 12usize];
    ["Offset of field: _MISC_FLAGS::disableDelayedOsMemoryReturn"]
        [::core::mem::offset_of!(_MISC_FLAGS, disableDelayedOsMemoryReturn) - 13usize];
    ["Offset of field: _MISC_FLAGS::internalCounters"]
        [::core::mem::offset_of!(_MISC_FLAGS, internalCounters) - 14usize];
    ["Offset of field: _MISC_FLAGS::linkerAlwaysPic"]
        [::core::mem::offset_of!(_MISC_FLAGS, linkerAlwaysPic) - 15usize];
    ["Offset of field: _MISC_FLAGS::linkerMemBase"]
        [::core::mem::offset_of!(_MISC_FLAGS, linkerMemBase) - 16usize];
    ["Offset of field: _MISC_FLAGS::ioManager"]
        [::core::mem::offset_of!(_MISC_FLAGS, ioManager) - 24usize];
    ["Offset of field: _MISC_FLAGS::numIoWorkerThreads"]
        [::core::mem::offset_of!(_MISC_FLAGS, numIoWorkerThreads) - 28usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__PAR_FLAGS() {
    assert_eq!(size_of::<sys::_PAR_FLAGS>(), size_of::<super::_PAR_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _PAR_FLAGS"][::core::mem::size_of::<_PAR_FLAGS>() - 40usize];
    ["Alignment of _PAR_FLAGS"][::core::mem::align_of::<_PAR_FLAGS>() - 4usize];
    ["Offset of field: _PAR_FLAGS::nCapabilities"]
        [::core::mem::offset_of!(_PAR_FLAGS, nCapabilities) - 0usize];
    ["Offset of field: _PAR_FLAGS::migrate"][::core::mem::offset_of!(_PAR_FLAGS, migrate) - 4usize];
    ["Offset of field: _PAR_FLAGS::maxLocalSparks"]
        [::core::mem::offset_of!(_PAR_FLAGS, maxLocalSparks) - 8usize];
    ["Offset of field: _PAR_FLAGS::parGcEnabled"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcEnabled) - 12usize];
    ["Offset of field: _PAR_FLAGS::parGcGen"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcGen) - 16usize];
    ["Offset of field: _PAR_FLAGS::parGcLoadBalancingEnabled"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcLoadBalancingEnabled) - 20usize];
    ["Offset of field: _PAR_FLAGS::parGcLoadBalancingGen"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcLoadBalancingGen) - 24usize];
    ["Offset of field: _PAR_FLAGS::parGcNoSyncWithIdle"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcNoSyncWithIdle) - 28usize];
    ["Offset of field: _PAR_FLAGS::parGcThreads"]
        [::core::mem::offset_of!(_PAR_FLAGS, parGcThreads) - 32usize];
    ["Offset of field: _PAR_FLAGS::setAffinity"]
        [::core::mem::offset_of!(_PAR_FLAGS, setAffinity) - 36usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__HPC_FLAGS() {
    assert_eq!(size_of::<sys::_HPC_FLAGS>(), size_of::<super::_HPC_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _HPC_FLAGS"][::core::mem::size_of::<_HPC_FLAGS>() - 8usize];
    ["Alignment of _HPC_FLAGS"][::core::mem::align_of::<_HPC_FLAGS>() - 4usize];
    ["Offset of field: _HPC_FLAGS::writeTixFile"]
        [::core::mem::offset_of!(_HPC_FLAGS, writeTixFile) - 0usize];
    ["Offset of field: _HPC_FLAGS::readTixFile"]
        [::core::mem::offset_of!(_HPC_FLAGS, readTixFile) - 4usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__TICKY_FLAGS() {
    assert_eq!(
        size_of::<sys::_TICKY_FLAGS>(),
        size_of::<super::_TICKY_FLAGS>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _TICKY_FLAGS"][::core::mem::size_of::<_TICKY_FLAGS>() - 16usize];
    ["Alignment of _TICKY_FLAGS"][::core::mem::align_of::<_TICKY_FLAGS>() - 8usize];
    ["Offset of field: _TICKY_FLAGS::showTickyStats"]
        [::core::mem::offset_of!(_TICKY_FLAGS, showTickyStats) - 0usize];
    ["Offset of field: _TICKY_FLAGS::tickyFile"]
        [::core::mem::offset_of!(_TICKY_FLAGS, tickyFile) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__RTS_FLAGS() {
    assert_eq!(size_of::<sys::_RTS_FLAGS>(), size_of::<super::_RTS_FLAGS>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _RTS_FLAGS"][::core::mem::size_of::<_RTS_FLAGS>() - 496usize];
    ["Alignment of _RTS_FLAGS"][::core::mem::align_of::<_RTS_FLAGS>() - 8usize];
    ["Offset of field: _RTS_FLAGS::GcFlags"][::core::mem::offset_of!(_RTS_FLAGS, GcFlags) - 0usize];
    ["Offset of field: _RTS_FLAGS::ConcFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, ConcFlags) - 192usize];
    ["Offset of field: _RTS_FLAGS::MiscFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, MiscFlags) - 208usize];
    ["Offset of field: _RTS_FLAGS::DebugFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, DebugFlags) - 240usize];
    ["Offset of field: _RTS_FLAGS::CcFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, CcFlags) - 264usize];
    ["Offset of field: _RTS_FLAGS::ProfFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, ProfFlags) - 288usize];
    ["Offset of field: _RTS_FLAGS::TraceFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, TraceFlags) - 384usize];
    ["Offset of field: _RTS_FLAGS::TickyFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, TickyFlags) - 432usize];
    ["Offset of field: _RTS_FLAGS::ParFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, ParFlags) - 448usize];
    ["Offset of field: _RTS_FLAGS::HpcFlags"]
        [::core::mem::offset_of!(_RTS_FLAGS, HpcFlags) - 488usize];
};
