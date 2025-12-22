#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_NO_GC_STATS_eq() {
    assert_eq!(NO_GC_STATS, sys::NO_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_NO_GC_STATS_layout() {
    assert_eq!(size_of_val(&NO_GC_STATS), size_of_val(&sys::NO_GC_STATS));
    assert_eq!(align_of_val(&NO_GC_STATS), align_of_val(&sys::NO_GC_STATS));
}

#[cfg(feature = "sys")]
#[test]
fn sys_COLLECT_GC_STATS_eq() {
    assert_eq!(COLLECT_GC_STATS, sys::COLLECT_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COLLECT_GC_STATS_layout() {
    assert_eq!(
        size_of_val(&COLLECT_GC_STATS),
        size_of_val(&sys::COLLECT_GC_STATS)
    );
    assert_eq!(
        align_of_val(&COLLECT_GC_STATS),
        align_of_val(&sys::COLLECT_GC_STATS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ONELINE_GC_STATS_eq() {
    assert_eq!(ONELINE_GC_STATS, sys::ONELINE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ONELINE_GC_STATS_layout() {
    assert_eq!(
        size_of_val(&ONELINE_GC_STATS),
        size_of_val(&sys::ONELINE_GC_STATS)
    );
    assert_eq!(
        align_of_val(&ONELINE_GC_STATS),
        align_of_val(&sys::ONELINE_GC_STATS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SUMMARY_GC_STATS_eq() {
    assert_eq!(SUMMARY_GC_STATS, sys::SUMMARY_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SUMMARY_GC_STATS_layout() {
    assert_eq!(
        size_of_val(&SUMMARY_GC_STATS),
        size_of_val(&sys::SUMMARY_GC_STATS)
    );
    assert_eq!(
        align_of_val(&SUMMARY_GC_STATS),
        align_of_val(&sys::SUMMARY_GC_STATS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_VERBOSE_GC_STATS_eq() {
    assert_eq!(VERBOSE_GC_STATS, sys::VERBOSE_GC_STATS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_VERBOSE_GC_STATS_layout() {
    assert_eq!(
        size_of_val(&VERBOSE_GC_STATS),
        size_of_val(&sys::VERBOSE_GC_STATS)
    );
    assert_eq!(
        align_of_val(&VERBOSE_GC_STATS),
        align_of_val(&sys::VERBOSE_GC_STATS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_NONE_eq() {
    assert_eq!(COST_CENTRES_NONE, sys::COST_CENTRES_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_NONE_layout() {
    assert_eq!(
        size_of_val(&COST_CENTRES_NONE),
        size_of_val(&sys::COST_CENTRES_NONE)
    );
    assert_eq!(
        align_of_val(&COST_CENTRES_NONE),
        align_of_val(&sys::COST_CENTRES_NONE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_SUMMARY_eq() {
    assert_eq!(COST_CENTRES_SUMMARY, sys::COST_CENTRES_SUMMARY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_SUMMARY_layout() {
    assert_eq!(
        size_of_val(&COST_CENTRES_SUMMARY),
        size_of_val(&sys::COST_CENTRES_SUMMARY)
    );
    assert_eq!(
        align_of_val(&COST_CENTRES_SUMMARY),
        align_of_val(&sys::COST_CENTRES_SUMMARY)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_VERBOSE_eq() {
    assert_eq!(COST_CENTRES_VERBOSE, sys::COST_CENTRES_VERBOSE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_VERBOSE_layout() {
    assert_eq!(
        size_of_val(&COST_CENTRES_VERBOSE),
        size_of_val(&sys::COST_CENTRES_VERBOSE)
    );
    assert_eq!(
        align_of_val(&COST_CENTRES_VERBOSE),
        align_of_val(&sys::COST_CENTRES_VERBOSE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_ALL_eq() {
    assert_eq!(COST_CENTRES_ALL, sys::COST_CENTRES_ALL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_ALL_layout() {
    assert_eq!(
        size_of_val(&COST_CENTRES_ALL),
        size_of_val(&sys::COST_CENTRES_ALL)
    );
    assert_eq!(
        align_of_val(&COST_CENTRES_ALL),
        align_of_val(&sys::COST_CENTRES_ALL)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_JSON_eq() {
    assert_eq!(COST_CENTRES_JSON, sys::COST_CENTRES_JSON);
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRES_JSON_layout() {
    assert_eq!(
        size_of_val(&COST_CENTRES_JSON),
        size_of_val(&sys::COST_CENTRES_JSON)
    );
    assert_eq!(
        align_of_val(&COST_CENTRES_JSON),
        align_of_val(&sys::COST_CENTRES_JSON)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_NO_HEAP_PROFILING_eq() {
    assert_eq!(NO_HEAP_PROFILING, sys::NO_HEAP_PROFILING);
}

#[cfg(feature = "sys")]
#[test]
fn sys_NO_HEAP_PROFILING_layout() {
    assert_eq!(
        size_of_val(&NO_HEAP_PROFILING),
        size_of_val(&sys::NO_HEAP_PROFILING)
    );
    assert_eq!(
        align_of_val(&NO_HEAP_PROFILING),
        align_of_val(&sys::NO_HEAP_PROFILING)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_CCS_eq() {
    assert_eq!(HEAP_BY_CCS, sys::HEAP_BY_CCS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_CCS_layout() {
    assert_eq!(size_of_val(&HEAP_BY_CCS), size_of_val(&sys::HEAP_BY_CCS));
    assert_eq!(align_of_val(&HEAP_BY_CCS), align_of_val(&sys::HEAP_BY_CCS));
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_MOD_eq() {
    assert_eq!(HEAP_BY_MOD, sys::HEAP_BY_MOD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_MOD_layout() {
    assert_eq!(size_of_val(&HEAP_BY_MOD), size_of_val(&sys::HEAP_BY_MOD));
    assert_eq!(align_of_val(&HEAP_BY_MOD), align_of_val(&sys::HEAP_BY_MOD));
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_DESCR_eq() {
    assert_eq!(HEAP_BY_DESCR, sys::HEAP_BY_DESCR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_DESCR_layout() {
    assert_eq!(
        size_of_val(&HEAP_BY_DESCR),
        size_of_val(&sys::HEAP_BY_DESCR)
    );
    assert_eq!(
        align_of_val(&HEAP_BY_DESCR),
        align_of_val(&sys::HEAP_BY_DESCR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_TYPE_eq() {
    assert_eq!(HEAP_BY_TYPE, sys::HEAP_BY_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_TYPE_layout() {
    assert_eq!(size_of_val(&HEAP_BY_TYPE), size_of_val(&sys::HEAP_BY_TYPE));
    assert_eq!(
        align_of_val(&HEAP_BY_TYPE),
        align_of_val(&sys::HEAP_BY_TYPE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_RETAINER_eq() {
    assert_eq!(HEAP_BY_RETAINER, sys::HEAP_BY_RETAINER);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_RETAINER_layout() {
    assert_eq!(
        size_of_val(&HEAP_BY_RETAINER),
        size_of_val(&sys::HEAP_BY_RETAINER)
    );
    assert_eq!(
        align_of_val(&HEAP_BY_RETAINER),
        align_of_val(&sys::HEAP_BY_RETAINER)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_LDV_eq() {
    assert_eq!(HEAP_BY_LDV, sys::HEAP_BY_LDV);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_LDV_layout() {
    assert_eq!(size_of_val(&HEAP_BY_LDV), size_of_val(&sys::HEAP_BY_LDV));
    assert_eq!(align_of_val(&HEAP_BY_LDV), align_of_val(&sys::HEAP_BY_LDV));
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_CLOSURE_TYPE_eq() {
    assert_eq!(HEAP_BY_CLOSURE_TYPE, sys::HEAP_BY_CLOSURE_TYPE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_CLOSURE_TYPE_layout() {
    assert_eq!(
        size_of_val(&HEAP_BY_CLOSURE_TYPE),
        size_of_val(&sys::HEAP_BY_CLOSURE_TYPE)
    );
    assert_eq!(
        align_of_val(&HEAP_BY_CLOSURE_TYPE),
        align_of_val(&sys::HEAP_BY_CLOSURE_TYPE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_INFO_TABLE_eq() {
    assert_eq!(HEAP_BY_INFO_TABLE, sys::HEAP_BY_INFO_TABLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_INFO_TABLE_layout() {
    assert_eq!(
        size_of_val(&HEAP_BY_INFO_TABLE),
        size_of_val(&sys::HEAP_BY_INFO_TABLE)
    );
    assert_eq!(
        align_of_val(&HEAP_BY_INFO_TABLE),
        align_of_val(&sys::HEAP_BY_INFO_TABLE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_ERA_eq() {
    assert_eq!(HEAP_BY_ERA, sys::HEAP_BY_ERA);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HEAP_BY_ERA_layout() {
    assert_eq!(size_of_val(&HEAP_BY_ERA), size_of_val(&sys::HEAP_BY_ERA));
    assert_eq!(align_of_val(&HEAP_BY_ERA), align_of_val(&sys::HEAP_BY_ERA));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_NONE_eq() {
    assert_eq!(TRACE_NONE, sys::TRACE_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_NONE_layout() {
    assert_eq!(size_of_val(&TRACE_NONE), size_of_val(&sys::TRACE_NONE));
    assert_eq!(align_of_val(&TRACE_NONE), align_of_val(&sys::TRACE_NONE));
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_EVENTLOG_eq() {
    assert_eq!(TRACE_EVENTLOG, sys::TRACE_EVENTLOG);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_EVENTLOG_layout() {
    assert_eq!(
        size_of_val(&TRACE_EVENTLOG),
        size_of_val(&sys::TRACE_EVENTLOG)
    );
    assert_eq!(
        align_of_val(&TRACE_EVENTLOG),
        align_of_val(&sys::TRACE_EVENTLOG)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_STDERR_eq() {
    assert_eq!(TRACE_STDERR, sys::TRACE_STDERR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_STDERR_layout() {
    assert_eq!(size_of_val(&TRACE_STDERR), size_of_val(&sys::TRACE_STDERR));
    assert_eq!(
        align_of_val(&TRACE_STDERR),
        align_of_val(&sys::TRACE_STDERR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys__GC_FLAGS_layout() {
    assert_eq!(size_of::<*mut FILE>(), size_of::<*mut sys::FILE>());
    assert_eq!(
        offset_of!(_GC_FLAGS, statsFile),
        offset_of!(sys::_GC_FLAGS, statsFile)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, giveStats),
        offset_of!(sys::_GC_FLAGS, giveStats)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, maxStkSize),
        offset_of!(sys::_GC_FLAGS, maxStkSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, initialStkSize),
        offset_of!(sys::_GC_FLAGS, initialStkSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, stkChunkSize),
        offset_of!(sys::_GC_FLAGS, stkChunkSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, stkChunkBufferSize),
        offset_of!(sys::_GC_FLAGS, stkChunkBufferSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, maxHeapSize),
        offset_of!(sys::_GC_FLAGS, maxHeapSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, minAllocAreaSize),
        offset_of!(sys::_GC_FLAGS, minAllocAreaSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, largeAllocLim),
        offset_of!(sys::_GC_FLAGS, largeAllocLim)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, nurseryChunkSize),
        offset_of!(sys::_GC_FLAGS, nurseryChunkSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, minOldGenSize),
        offset_of!(sys::_GC_FLAGS, minOldGenSize)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, heapSizeSuggestion),
        offset_of!(sys::_GC_FLAGS, heapSizeSuggestion)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, heapSizeSuggestionAuto),
        offset_of!(sys::_GC_FLAGS, heapSizeSuggestionAuto)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, oldGenFactor),
        offset_of!(sys::_GC_FLAGS, oldGenFactor)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, returnDecayFactor),
        offset_of!(sys::_GC_FLAGS, returnDecayFactor)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, pcFreeHeap),
        offset_of!(sys::_GC_FLAGS, pcFreeHeap)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, useNonmoving),
        offset_of!(sys::_GC_FLAGS, useNonmoving)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, nonmovingDenseAllocatorCount),
        offset_of!(sys::_GC_FLAGS, nonmovingDenseAllocatorCount)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, generations),
        offset_of!(sys::_GC_FLAGS, generations)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, squeezeUpdFrames),
        offset_of!(sys::_GC_FLAGS, squeezeUpdFrames)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, compact),
        offset_of!(sys::_GC_FLAGS, compact)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, compactThreshold),
        offset_of!(sys::_GC_FLAGS, compactThreshold)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, sweep),
        offset_of!(sys::_GC_FLAGS, sweep)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, ringBell),
        offset_of!(sys::_GC_FLAGS, ringBell)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, idleGCDelayTime),
        offset_of!(sys::_GC_FLAGS, idleGCDelayTime)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, interIdleGCWait),
        offset_of!(sys::_GC_FLAGS, interIdleGCWait)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, doIdleGC),
        offset_of!(sys::_GC_FLAGS, doIdleGC)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, longGCSync),
        offset_of!(sys::_GC_FLAGS, longGCSync)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, heapBase),
        offset_of!(sys::_GC_FLAGS, heapBase)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, allocLimitGrace),
        offset_of!(sys::_GC_FLAGS, allocLimitGrace)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, heapLimitGrace),
        offset_of!(sys::_GC_FLAGS, heapLimitGrace)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, numa),
        offset_of!(sys::_GC_FLAGS, numa)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, numaMask),
        offset_of!(sys::_GC_FLAGS, numaMask)
    );
    assert_eq!(
        offset_of!(_GC_FLAGS, addressSpaceSize),
        offset_of!(sys::_GC_FLAGS, addressSpaceSize)
    );
    assert_eq!(size_of::<_GC_FLAGS>(), size_of::<sys::_GC_FLAGS>());
    assert_eq!(align_of::<_GC_FLAGS>(), align_of::<sys::_GC_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_GC_FLAGS_layout() {
    assert_eq!(size_of::<GC_FLAGS>(), size_of::<sys::GC_FLAGS>());
    assert_eq!(align_of::<GC_FLAGS>(), align_of::<sys::GC_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys__DEBUG_FLAGS_layout() {
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, scheduler),
        offset_of!(sys::_DEBUG_FLAGS, scheduler)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, interpreter),
        offset_of!(sys::_DEBUG_FLAGS, interpreter)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, weak),
        offset_of!(sys::_DEBUG_FLAGS, weak)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, gccafs),
        offset_of!(sys::_DEBUG_FLAGS, gccafs)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, gc),
        offset_of!(sys::_DEBUG_FLAGS, gc)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, nonmoving_gc),
        offset_of!(sys::_DEBUG_FLAGS, nonmoving_gc)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, block_alloc),
        offset_of!(sys::_DEBUG_FLAGS, block_alloc)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, sanity),
        offset_of!(sys::_DEBUG_FLAGS, sanity)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, zero_on_gc),
        offset_of!(sys::_DEBUG_FLAGS, zero_on_gc)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, stable),
        offset_of!(sys::_DEBUG_FLAGS, stable)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, prof),
        offset_of!(sys::_DEBUG_FLAGS, prof)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, linker),
        offset_of!(sys::_DEBUG_FLAGS, linker)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, linker_verbose),
        offset_of!(sys::_DEBUG_FLAGS, linker_verbose)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, apply),
        offset_of!(sys::_DEBUG_FLAGS, apply)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, stm),
        offset_of!(sys::_DEBUG_FLAGS, stm)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, squeeze),
        offset_of!(sys::_DEBUG_FLAGS, squeeze)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, hpc),
        offset_of!(sys::_DEBUG_FLAGS, hpc)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, sparks),
        offset_of!(sys::_DEBUG_FLAGS, sparks)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, numa),
        offset_of!(sys::_DEBUG_FLAGS, numa)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, compact),
        offset_of!(sys::_DEBUG_FLAGS, compact)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, continuation),
        offset_of!(sys::_DEBUG_FLAGS, continuation)
    );
    assert_eq!(
        offset_of!(_DEBUG_FLAGS, iomanager),
        offset_of!(sys::_DEBUG_FLAGS, iomanager)
    );
    assert_eq!(size_of::<_DEBUG_FLAGS>(), size_of::<sys::_DEBUG_FLAGS>());
    assert_eq!(align_of::<_DEBUG_FLAGS>(), align_of::<sys::_DEBUG_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_DEBUG_FLAGS_layout() {
    assert_eq!(size_of::<DEBUG_FLAGS>(), size_of::<sys::DEBUG_FLAGS>());
    assert_eq!(align_of::<DEBUG_FLAGS>(), align_of::<sys::DEBUG_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_COST_CENTRE_FLAGS_layout() {
    assert_eq!(
        size_of::<COST_CENTRE_FLAGS>(),
        size_of::<sys::COST_CENTRE_FLAGS>()
    );
    assert_eq!(
        align_of::<COST_CENTRE_FLAGS>(),
        align_of::<sys::COST_CENTRE_FLAGS>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_PROFILING_FLAGS_layout() {
    assert_eq!(
        size_of::<PROFILING_FLAGS>(),
        size_of::<sys::PROFILING_FLAGS>()
    );
    assert_eq!(
        align_of::<PROFILING_FLAGS>(),
        align_of::<sys::PROFILING_FLAGS>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TRACE_FLAGS_layout() {
    assert_eq!(size_of::<TRACE_FLAGS>(), size_of::<sys::TRACE_FLAGS>());
    assert_eq!(align_of::<TRACE_FLAGS>(), align_of::<sys::TRACE_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_CONCURRENT_FLAGS_layout() {
    assert_eq!(
        size_of::<CONCURRENT_FLAGS>(),
        size_of::<sys::CONCURRENT_FLAGS>()
    );
    assert_eq!(
        align_of::<CONCURRENT_FLAGS>(),
        align_of::<sys::CONCURRENT_FLAGS>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_MISC_FLAGS_layout() {
    assert_eq!(size_of::<MISC_FLAGS>(), size_of::<sys::MISC_FLAGS>());
    assert_eq!(align_of::<MISC_FLAGS>(), align_of::<sys::MISC_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_PAR_FLAGS_layout() {
    assert_eq!(size_of::<PAR_FLAGS>(), size_of::<sys::PAR_FLAGS>());
    assert_eq!(align_of::<PAR_FLAGS>(), align_of::<sys::PAR_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HPC_FLAGS_layout() {
    assert_eq!(size_of::<HPC_FLAGS>(), size_of::<sys::HPC_FLAGS>());
    assert_eq!(align_of::<HPC_FLAGS>(), align_of::<sys::HPC_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_TICKY_FLAGS_layout() {
    assert_eq!(size_of::<TICKY_FLAGS>(), size_of::<sys::TICKY_FLAGS>());
    assert_eq!(align_of::<TICKY_FLAGS>(), align_of::<sys::TICKY_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys__RTS_FLAGS_layout() {
    assert_eq!(size_of::<GC_FLAGS>(), size_of::<sys::GC_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, GcFlags),
        offset_of!(sys::_RTS_FLAGS, GcFlags)
    );
    assert_eq!(
        size_of::<CONCURRENT_FLAGS>(),
        size_of::<sys::CONCURRENT_FLAGS>()
    );
    assert_eq!(
        offset_of!(_RTS_FLAGS, ConcFlags),
        offset_of!(sys::_RTS_FLAGS, ConcFlags)
    );
    assert_eq!(size_of::<MISC_FLAGS>(), size_of::<sys::MISC_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, MiscFlags),
        offset_of!(sys::_RTS_FLAGS, MiscFlags)
    );
    assert_eq!(size_of::<DEBUG_FLAGS>(), size_of::<sys::DEBUG_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, DebugFlags),
        offset_of!(sys::_RTS_FLAGS, DebugFlags)
    );
    assert_eq!(
        size_of::<COST_CENTRE_FLAGS>(),
        size_of::<sys::COST_CENTRE_FLAGS>()
    );
    assert_eq!(
        offset_of!(_RTS_FLAGS, CcFlags),
        offset_of!(sys::_RTS_FLAGS, CcFlags)
    );
    assert_eq!(
        size_of::<PROFILING_FLAGS>(),
        size_of::<sys::PROFILING_FLAGS>()
    );
    assert_eq!(
        offset_of!(_RTS_FLAGS, ProfFlags),
        offset_of!(sys::_RTS_FLAGS, ProfFlags)
    );
    assert_eq!(size_of::<TRACE_FLAGS>(), size_of::<sys::TRACE_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, TraceFlags),
        offset_of!(sys::_RTS_FLAGS, TraceFlags)
    );
    assert_eq!(size_of::<TICKY_FLAGS>(), size_of::<sys::TICKY_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, TickyFlags),
        offset_of!(sys::_RTS_FLAGS, TickyFlags)
    );
    assert_eq!(size_of::<PAR_FLAGS>(), size_of::<sys::PAR_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, ParFlags),
        offset_of!(sys::_RTS_FLAGS, ParFlags)
    );
    assert_eq!(size_of::<HPC_FLAGS>(), size_of::<sys::HPC_FLAGS>());
    assert_eq!(
        offset_of!(_RTS_FLAGS, HpcFlags),
        offset_of!(sys::_RTS_FLAGS, HpcFlags)
    );
    assert_eq!(size_of::<_RTS_FLAGS>(), size_of::<sys::_RTS_FLAGS>());
    assert_eq!(align_of::<_RTS_FLAGS>(), align_of::<sys::_RTS_FLAGS>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_RTS_FLAGS_layout() {
    assert_eq!(size_of::<RTS_FLAGS>(), size_of::<sys::RTS_FLAGS>());
    assert_eq!(align_of::<RTS_FLAGS>(), align_of::<sys::RTS_FLAGS>());
}
