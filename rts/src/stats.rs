use crate::capability::getCapability;
use crate::ffi::rts::flags::{
    NO_GC_STATS, ONELINE_GC_STATS, RtsFlags, SUMMARY_GC_STATS, VERBOSE_GC_STATS,
};
use crate::ffi::rts::messages::{debugBelch, vdebugBelch};
use crate::ffi::rts::storage::block::{
    BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, MBLOCK_SIZE, bdescr,
};
use crate::ffi::rts::storage::gc::{generation, generations};
use crate::ffi::rts::storage::m_block::{mblocks_allocated, peak_mblocks_allocated};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::Time;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time, getProcessElapsedTime};
use crate::ffi::rts_api::{_RTSStats, Capability, GCDetails_, RTSStats};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::get_time::{getCurrentThreadCPUTime, getPageFaults, getProcessTimes};
use crate::prelude::*;
use crate::rts_flags::rtsConfig;
use crate::rts_utils::{showStgWord64, stgFree, stgMallocBytes};
use crate::sm::block_alloc::{hw_alloc_blocks, n_alloc_blocks};
use crate::sm::gc_thread::gc_thread;
use crate::sm::storage::{
    calcTotalAllocated, calcTotalCompactW, calcTotalLargeObjectsW, countOccupied,
    gcThreadLiveBlocks, gcThreadLiveWords, genLiveBlocks, genLiveWords, updateNurseriesStats,
};
use crate::stats::{
    GenerationSummaryStats, GenerationSummaryStats_, RTSSummaryStats, RTSSummaryStats_,
};
use crate::trace::{
    CAPSET_HEAP_DEFAULT, traceConcSyncBegin, traceConcSyncEnd, traceEventBlocksSize,
    traceEventGcEndAtT, traceEventGcGlobalSync, traceEventGcStartAtT, traceEventGcStats,
    traceEventHeapLive, traceEventHeapSize,
};

#[cfg(test)]
mod tests;

pub(crate) type RTSSummaryStats = RTSSummaryStats_;

/// cbindgen:no-export
pub(crate) struct RTSSummaryStats_ {
    pub(crate) rp_cpu_ns: Time,
    pub(crate) rp_elapsed_ns: Time,
    pub(crate) hc_cpu_ns: Time,
    pub(crate) hc_elapsed_ns: Time,
    pub(crate) exit_cpu_ns: Time,
    pub(crate) exit_elapsed_ns: Time,
    pub(crate) gc_cpu_percent: c_double,
    pub(crate) gc_elapsed_percent: c_double,
    pub(crate) fragmentation_bytes: uint64_t,
    pub(crate) average_bytes_used: uint64_t,
    pub(crate) alloc_rate: uint64_t,
    pub(crate) productivity_cpu_percent: c_double,
    pub(crate) productivity_elapsed_percent: c_double,
    pub(crate) gc_summary_stats: *mut GenerationSummaryStats,
}

pub(crate) type GenerationSummaryStats = GenerationSummaryStats_;

/// cbindgen:no-export
pub(crate) struct GenerationSummaryStats_ {
    pub(crate) collections: uint32_t,
    pub(crate) par_collections: uint32_t,
    pub(crate) cpu_ns: Time,
    pub(crate) elapsed_ns: Time,
    pub(crate) max_pause_ns: Time,
    pub(crate) avg_pause_ns: Time,
}

static mut start_exit_cpu: Time = 0;

static mut end_exit_elapsed: Time = 0;

static mut end_init_cpu: Time = 0;

static mut end_exit_cpu: Time = 0;

static mut start_exit_elapsed: Time = 0;

static mut start_exit_gc_elapsed: Time = 0;

static mut start_exit_gc_cpu: Time = 0;

static mut start_nonmoving_gc_sync_elapsed: Time = 0;

static mut start_init_elapsed: Time = 0;

static mut end_init_elapsed: Time = 0;

static mut start_nonmoving_gc_elapsed: Time = 0;

static mut start_nonmoving_gc_cpu: Time = 0;

static mut start_init_cpu: Time = 0;

static mut stats: RTSStats = _RTSStats {
    gcs: 0,
    major_gcs: 0,
    allocated_bytes: 0,
    max_live_bytes: 0,
    max_large_objects_bytes: 0,
    max_compact_bytes: 0,
    max_slop_bytes: 0,
    max_mem_in_use_bytes: 0,
    cumulative_live_bytes: 0,
    copied_bytes: 0,
    par_copied_bytes: 0,
    cumulative_par_max_copied_bytes: 0,
    cumulative_par_balanced_copied_bytes: 0,
    init_cpu_ns: 0,
    init_elapsed_ns: 0,
    mutator_cpu_ns: 0,
    mutator_elapsed_ns: 0,
    gc_cpu_ns: 0,
    gc_elapsed_ns: 0,
    cpu_ns: 0,
    elapsed_ns: 0,
    gc: GCDetails_ {
        r#gen: 0,
        threads: 0,
        allocated_bytes: 0,
        live_bytes: 0,
        large_objects_bytes: 0,
        compact_bytes: 0,
        slop_bytes: 0,
        mem_in_use_bytes: 0,
        copied_bytes: 0,
        block_fragmentation_bytes: 0,
        par_max_copied_bytes: 0,
        par_balanced_copied_bytes: 0,
        sync_elapsed_ns: 0,
        cpu_ns: 0,
        elapsed_ns: 0,
        nonmoving_gc_sync_cpu_ns: 0,
        nonmoving_gc_sync_elapsed_ns: 0,
        nonmoving_gc_cpu_ns: 0,
        nonmoving_gc_elapsed_ns: 0,
    },
    any_work: 0,
    scav_find_work: 0,
    max_n_todo_overflow: 0,
    nonmoving_gc_sync_cpu_ns: 0,
    nonmoving_gc_sync_elapsed_ns: 0,
    nonmoving_gc_sync_max_elapsed_ns: 0,
    nonmoving_gc_cpu_ns: 0,
    nonmoving_gc_elapsed_ns: 0,
    nonmoving_gc_max_elapsed_ns: 0,
};

static mut GC_end_faults: W_ = 0 as W_;

static mut GC_coll_cpu: *mut Time = null::<Time>() as *mut Time;

static mut GC_coll_elapsed: *mut Time = null::<Time>() as *mut Time;

static mut GC_coll_max_pause: *mut Time = null::<Time>() as *mut Time;

unsafe fn stat_getElapsedTime() -> Time {
    return getProcessElapsedTime() - start_init_elapsed;
}

unsafe fn initStats0() {
    start_init_cpu = 0 as Time;
    start_init_elapsed = 0 as Time;
    end_init_cpu = 0 as Time;
    end_init_elapsed = 0 as Time;
    start_nonmoving_gc_cpu = 0 as Time;
    start_nonmoving_gc_elapsed = 0 as Time;
    start_nonmoving_gc_sync_elapsed = 0 as Time;
    start_exit_cpu = 0 as Time;
    start_exit_elapsed = 0 as Time;
    start_exit_gc_cpu = 0 as Time;
    start_exit_gc_elapsed = 0 as Time;
    end_exit_cpu = 0 as Time;
    end_exit_elapsed = 0 as Time;
    GC_end_faults = 0 as W_;

    stats = _RTSStats {
        gcs: 0 as uint32_t,
        major_gcs: 0 as uint32_t,
        allocated_bytes: 0 as uint64_t,
        max_live_bytes: 0 as uint64_t,
        max_large_objects_bytes: 0 as uint64_t,
        max_compact_bytes: 0 as uint64_t,
        max_slop_bytes: 0 as uint64_t,
        max_mem_in_use_bytes: 0 as uint64_t,
        cumulative_live_bytes: 0 as uint64_t,
        copied_bytes: 0 as uint64_t,
        par_copied_bytes: 0 as uint64_t,
        cumulative_par_max_copied_bytes: 0 as uint64_t,
        cumulative_par_balanced_copied_bytes: 0 as uint64_t,
        init_cpu_ns: 0 as Time,
        init_elapsed_ns: 0 as Time,
        mutator_cpu_ns: 0 as Time,
        mutator_elapsed_ns: 0 as Time,
        gc_cpu_ns: 0 as Time,
        gc_elapsed_ns: 0 as Time,
        cpu_ns: 0 as Time,
        elapsed_ns: 0 as Time,
        gc: GCDetails_ {
            r#gen: 0 as uint32_t,
            threads: 0 as uint32_t,
            allocated_bytes: 0 as uint64_t,
            live_bytes: 0 as uint64_t,
            large_objects_bytes: 0 as uint64_t,
            compact_bytes: 0 as uint64_t,
            slop_bytes: 0 as uint64_t,
            mem_in_use_bytes: 0 as uint64_t,
            copied_bytes: 0 as uint64_t,
            block_fragmentation_bytes: 0 as uint64_t,
            par_max_copied_bytes: 0 as uint64_t,
            par_balanced_copied_bytes: 0 as uint64_t,
            sync_elapsed_ns: 0 as Time,
            cpu_ns: 0 as Time,
            elapsed_ns: 0 as Time,
            nonmoving_gc_sync_cpu_ns: 0,
            nonmoving_gc_sync_elapsed_ns: 0 as Time,
            nonmoving_gc_cpu_ns: 0 as Time,
            nonmoving_gc_elapsed_ns: 0 as Time,
        },
        any_work: 0 as uint64_t,
        scav_find_work: 0 as uint64_t,
        max_n_todo_overflow: 0 as uint64_t,
        nonmoving_gc_sync_cpu_ns: 0,
        nonmoving_gc_sync_elapsed_ns: 0 as Time,
        nonmoving_gc_sync_max_elapsed_ns: 0 as Time,
        nonmoving_gc_cpu_ns: 0 as Time,
        nonmoving_gc_elapsed_ns: 0 as Time,
        nonmoving_gc_max_elapsed_ns: 0 as Time,
    };
}

unsafe fn initStats1() {
    if RtsFlags.GcFlags.giveStats >= VERBOSE_GC_STATS as uint32_t {
        statsPrintf(
            b"    Alloc    Copied     Live     GC     GC      TOT      TOT  Page Flts\n\0"
                as *const u8 as *const c_char as *mut c_char,
        );

        statsPrintf(
            b"    bytes     bytes     bytes   user   elap     user     elap\n\0" as *const u8
                as *const c_char as *mut c_char,
        );
    }

    GC_coll_cpu = stgMallocBytes(
        (size_of::<Time>() as size_t).wrapping_mul(RtsFlags.GcFlags.generations as size_t),
        b"initStats\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Time;

    GC_coll_elapsed = stgMallocBytes(
        (size_of::<Time>() as size_t).wrapping_mul(RtsFlags.GcFlags.generations as size_t),
        b"initStats\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Time;

    GC_coll_max_pause = stgMallocBytes(
        (size_of::<Time>() as size_t).wrapping_mul(RtsFlags.GcFlags.generations as size_t),
        b"initStats\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Time;

    initGenerationStats();
}

unsafe fn initGenerationStats() {
    let mut i: uint32_t = 0 as uint32_t;

    while i < RtsFlags.GcFlags.generations {
        *GC_coll_cpu.offset(i as isize) = 0 as Time;
        *GC_coll_elapsed.offset(i as isize) = 0 as Time;
        *GC_coll_max_pause.offset(i as isize) = 0 as Time;
        i = i.wrapping_add(1);
    }
}

unsafe fn resetChildProcessStats() {
    initStats0();
    initGenerationStats();
}

unsafe fn stat_startInit() {
    getProcessTimes(&raw mut start_init_cpu, &raw mut start_init_elapsed);
}

unsafe fn stat_endInit() {
    getProcessTimes(&raw mut end_init_cpu, &raw mut end_init_elapsed);
    stats.init_cpu_ns = end_init_cpu - start_init_cpu;
    stats.init_elapsed_ns = end_init_elapsed - start_init_elapsed;
}

unsafe fn stat_startExit() {
    getProcessTimes(&raw mut start_exit_cpu, &raw mut start_exit_elapsed);
    start_exit_gc_elapsed = stats.gc_elapsed_ns;
    start_exit_gc_cpu = stats.gc_cpu_ns;
}

unsafe fn stat_endExit() {
    getProcessTimes(&raw mut end_exit_cpu, &raw mut end_exit_elapsed);
}

unsafe fn stat_startGCSync(mut gct: *mut gc_thread) {
    (*gct).gc_sync_start_elapsed = getProcessElapsedTime();
}

unsafe fn stat_startNonmovingGc() {
    start_nonmoving_gc_cpu = getCurrentThreadCPUTime();
    start_nonmoving_gc_elapsed = getProcessElapsedTime();
}

unsafe fn stat_endNonmovingGc() {
    let mut cpu = getCurrentThreadCPUTime();
    let mut elapsed = getProcessElapsedTime();
    stats.gc.nonmoving_gc_elapsed_ns = elapsed - start_nonmoving_gc_elapsed;
    stats.nonmoving_gc_elapsed_ns += stats.gc.nonmoving_gc_elapsed_ns;
    stats.gc.nonmoving_gc_cpu_ns = cpu - start_nonmoving_gc_cpu;
    stats.nonmoving_gc_cpu_ns += stats.gc.nonmoving_gc_cpu_ns;

    stats.nonmoving_gc_max_elapsed_ns = ({
        let mut _a: Time = stats.gc.nonmoving_gc_elapsed_ns as Time;
        let mut _b: Time = stats.nonmoving_gc_max_elapsed_ns as Time;

        if _a <= _b { _b } else { _a as Time }
    });
}

unsafe fn stat_startNonmovingGcSync() {
    start_nonmoving_gc_sync_elapsed = getProcessElapsedTime();
    traceConcSyncBegin();
}

unsafe fn stat_endNonmovingGcSync() {
    let mut end_elapsed = getProcessElapsedTime();
    stats.gc.nonmoving_gc_sync_elapsed_ns = end_elapsed - start_nonmoving_gc_sync_elapsed;
    stats.nonmoving_gc_sync_elapsed_ns += stats.gc.nonmoving_gc_sync_elapsed_ns;

    stats.nonmoving_gc_sync_max_elapsed_ns = ({
        let mut _a: Time = stats.gc.nonmoving_gc_sync_elapsed_ns as Time;
        let mut _b: Time = stats.nonmoving_gc_sync_max_elapsed_ns as Time;

        if _a <= _b { _b } else { _a as Time }
    });

    let mut sync_elapsed: Time = stats.gc.nonmoving_gc_sync_elapsed_ns;

    if RtsFlags.GcFlags.giveStats == VERBOSE_GC_STATS as uint32_t {
        statsPrintf(
            b"# sync %6.3f\n\0" as *const u8 as *const c_char as *mut c_char,
            sync_elapsed as c_double / TIME_RESOLUTION as c_double,
        );
    }

    traceConcSyncEnd();
}

unsafe fn stat_startGCWorker(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as c_int != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_start_cpu = getCurrentThreadCPUTime();
    }
}

unsafe fn stat_endGCWorker(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as c_int != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_end_cpu = getCurrentThreadCPUTime();
    }
}

unsafe fn stat_startGC(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    if RtsFlags.GcFlags.ringBell {
        debugBelch(b"\x07\0" as *const u8 as *const c_char);
    }

    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as c_int != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_start_cpu = getCurrentThreadCPUTime();
    }

    (*gct).gc_start_elapsed = getProcessElapsedTime();

    traceEventGcStartAtT(
        cap,
        ((*gct).gc_start_elapsed - start_init_elapsed) as StgWord64,
    );

    if RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t {
        (*gct).gc_start_faults = getPageFaults();
    }

    updateNurseriesStats();
}

unsafe fn stat_endGC(
    mut cap: *mut Capability,
    mut initiating_gct: *mut gc_thread,
    mut live: W_,
    mut copied: W_,
    mut slop: W_,
    mut r#gen: uint32_t,
    mut par_n_threads: uint32_t,
    mut gc_threads: *mut *mut gc_thread,
    mut par_max_copied: W_,
    mut par_balanced_copied: W_,
    mut any_work: W_,
    mut scav_find_work: W_,
    mut max_n_todo_overflow: W_,
) {
    stats.gc.r#gen = r#gen;
    stats.gc.threads = par_n_threads;

    let mut tot_alloc_bytes: uint64_t =
        calcTotalAllocated().wrapping_mul(size_of::<W_>() as uint64_t);
    stats.gc.allocated_bytes = tot_alloc_bytes.wrapping_sub(stats.allocated_bytes);
    stats.gc.live_bytes = live.wrapping_mul(size_of::<W_>() as W_) as uint64_t;
    stats.gc.large_objects_bytes =
        calcTotalLargeObjectsW().wrapping_mul(size_of::<W_>() as StgWord) as uint64_t;
    stats.gc.compact_bytes =
        calcTotalCompactW().wrapping_mul(size_of::<W_>() as StgWord) as uint64_t;
    stats.gc.slop_bytes = slop.wrapping_mul(size_of::<W_>() as W_) as uint64_t;
    stats.gc.mem_in_use_bytes = mblocks_allocated.wrapping_mul(MBLOCK_SIZE as W_) as uint64_t;
    stats.gc.copied_bytes = copied.wrapping_mul(size_of::<W_>() as W_) as uint64_t;
    stats.gc.par_max_copied_bytes = par_max_copied.wrapping_mul(size_of::<W_>() as W_) as uint64_t;
    stats.gc.par_balanced_copied_bytes =
        par_balanced_copied.wrapping_mul(size_of::<W_>() as W_) as uint64_t;
    stats.gc.block_fragmentation_bytes = mblocks_allocated
        .wrapping_mul(BLOCKS_PER_MBLOCK)
        .wrapping_sub(n_alloc_blocks)
        .wrapping_mul(BLOCK_SIZE as W_) as uint64_t;

    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as c_int != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        let mut current_cpu: Time = 0;
        let mut current_elapsed: Time = 0;
        getProcessTimes(&raw mut current_cpu, &raw mut current_elapsed);
        stats.cpu_ns = current_cpu - start_init_cpu;
        stats.elapsed_ns = current_elapsed - start_init_elapsed;
        stats.gc.sync_elapsed_ns =
            (*initiating_gct).gc_start_elapsed - (*initiating_gct).gc_sync_start_elapsed;
        stats.gc.elapsed_ns = current_elapsed - (*initiating_gct).gc_start_elapsed;
        stats.gc.cpu_ns = 0 as Time;

        let mut i = 0 as c_uint;

        while (i as uint32_t) < par_n_threads {
            let mut gct = *gc_threads.offset(i as isize);
            stats.gc.cpu_ns += (*gct).gc_end_cpu - (*gct).gc_start_cpu;
            (*gct).gc_end_cpu = 0 as Time;
            (*gct).gc_start_cpu = 0 as Time;
            i = i.wrapping_add(1);
        }
    }

    stats.gcs = stats.gcs.wrapping_add(1);
    stats.allocated_bytes = tot_alloc_bytes;
    stats.max_mem_in_use_bytes = peak_mblocks_allocated.wrapping_mul(MBLOCK_SIZE as W_) as uint64_t;
    *GC_coll_cpu.offset(r#gen as isize) += stats.gc.cpu_ns;
    *GC_coll_elapsed.offset(r#gen as isize) += stats.gc.elapsed_ns;

    if *GC_coll_max_pause.offset(r#gen as isize) < stats.gc.elapsed_ns {
        *GC_coll_max_pause.offset(r#gen as isize) = stats.gc.elapsed_ns;
    }

    stats.copied_bytes = stats.copied_bytes.wrapping_add(stats.gc.copied_bytes);

    if par_n_threads > 1 as uint32_t {
        stats.par_copied_bytes = stats.par_copied_bytes.wrapping_add(stats.gc.copied_bytes);
        stats.cumulative_par_max_copied_bytes = stats
            .cumulative_par_max_copied_bytes
            .wrapping_add(stats.gc.par_max_copied_bytes);
        stats.cumulative_par_balanced_copied_bytes = stats
            .cumulative_par_balanced_copied_bytes
            .wrapping_add(stats.gc.par_balanced_copied_bytes);
        stats.any_work = stats.any_work.wrapping_add(any_work as uint64_t);
        stats.scav_find_work = stats
            .scav_find_work
            .wrapping_add(scav_find_work as uint64_t);

        stats.max_n_todo_overflow = stats.max_n_todo_overflow.wrapping_add(
            ({
                let mut _a: W_ = max_n_todo_overflow as W_;
                let mut _b: W_ = stats.max_n_todo_overflow as W_;

                if _a <= _b { _b } else { _a as W_ }
            }) as uint64_t,
        );
    }

    stats.gc_cpu_ns += stats.gc.cpu_ns;
    stats.gc_elapsed_ns += stats.gc.elapsed_ns;

    if r#gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) {
        stats.major_gcs = stats.major_gcs.wrapping_add(1);

        if stats.gc.live_bytes > stats.max_live_bytes {
            stats.max_live_bytes = stats.gc.live_bytes;
        }

        if stats.gc.large_objects_bytes > stats.max_large_objects_bytes {
            stats.max_large_objects_bytes = stats.gc.large_objects_bytes;
        }

        if stats.gc.compact_bytes > stats.max_compact_bytes {
            stats.max_compact_bytes = stats.gc.compact_bytes;
        }

        if stats.gc.slop_bytes > stats.max_slop_bytes {
            stats.max_slop_bytes = stats.gc.slop_bytes;
        }

        stats.cumulative_live_bytes = stats
            .cumulative_live_bytes
            .wrapping_add(stats.gc.live_bytes);
    }

    if stats_enabled {
        traceEventGcGlobalSync(cap);

        traceEventGcStats(
            cap,
            CAPSET_HEAP_DEFAULT,
            stats.gc.r#gen,
            stats.gc.copied_bytes as W_,
            stats.gc.slop_bytes as W_,
            stats.gc.block_fragmentation_bytes as W_,
            par_n_threads,
            stats.gc.par_max_copied_bytes as W_,
            stats.gc.copied_bytes as W_,
            stats.gc.par_balanced_copied_bytes as W_,
        );

        traceEventGcEndAtT(cap, stats.elapsed_ns as StgWord64);

        if r#gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) {
            traceEventHeapLive(cap, CAPSET_HEAP_DEFAULT, stats.gc.live_bytes as W_);
        }

        if RtsFlags.GcFlags.giveStats == VERBOSE_GC_STATS as uint32_t {
            let mut faults = getPageFaults();

            statsPrintf(
                b"%9llu %9llu %9llu\0" as *const u8 as *const c_char as *mut c_char,
                stats.gc.allocated_bytes,
                stats.gc.copied_bytes,
                stats.gc.live_bytes,
            );

            statsPrintf(
                b" %6.3f %6.3f %8.3f %8.3f %4llu %4llu  (Gen: %2d)\n\0" as *const u8
                    as *const c_char as *mut c_char,
                stats.gc.cpu_ns as c_double / TIME_RESOLUTION as c_double,
                stats.gc.elapsed_ns as c_double / TIME_RESOLUTION as c_double,
                stats.cpu_ns as c_double / TIME_RESOLUTION as c_double,
                stats.elapsed_ns as c_double / TIME_RESOLUTION as c_double,
                faults.wrapping_sub((*initiating_gct).gc_start_faults),
                (*initiating_gct)
                    .gc_start_faults
                    .wrapping_sub(GC_end_faults),
                r#gen,
            );

            GC_end_faults = faults;
            statsFlush();
        }

        if rtsConfig.gcDoneHook.is_some() {
            rtsConfig.gcDoneHook.expect("non-null function pointer")(&raw mut stats.gc);
        }

        traceEventHeapSize(
            cap,
            CAPSET_HEAP_DEFAULT,
            mblocks_allocated.wrapping_mul(MBLOCK_SIZE as W_),
        );

        traceEventBlocksSize(
            cap,
            CAPSET_HEAP_DEFAULT,
            n_alloc_blocks.wrapping_mul(BLOCK_SIZE as W_),
        );
    }
}

unsafe fn init_RTSSummaryStats(mut sum: *mut RTSSummaryStats) {
    let sizeof_gc_summary_stats: size_t = (RtsFlags.GcFlags.generations as size_t)
        .wrapping_mul(size_of::<GenerationSummaryStats>() as size_t);

    memset(
        sum as *mut c_void,
        0 as c_int,
        size_of::<RTSSummaryStats>() as size_t,
    );

    (*sum).gc_summary_stats = stgMallocBytes(
        sizeof_gc_summary_stats,
        b"alloc_RTSSummaryStats.gc_summary_stats\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut GenerationSummaryStats;

    memset(
        (*sum).gc_summary_stats as *mut c_void,
        0 as c_int,
        sizeof_gc_summary_stats,
    );
}

unsafe fn free_RTSSummaryStats(mut sum: *mut RTSSummaryStats) {
    stgFree((*sum).gc_summary_stats as *mut c_void);
    (*sum).gc_summary_stats = null_mut::<GenerationSummaryStats>();
}

unsafe fn report_summary(mut sum: *const RTSSummaryStats) {
    let mut g: uint32_t = 0;
    let mut temp: [c_char; 512] = [0; 512];

    showStgWord64(
        stats.allocated_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        r#true != 0,
    );

    statsPrintf(
        b"%16s bytes allocated in the heap\n\0" as *const u8 as *const c_char as *mut c_char,
        &raw mut temp as *mut c_char,
    );

    showStgWord64(
        stats.copied_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        r#true != 0,
    );

    statsPrintf(
        b"%16s bytes copied during GC\n\0" as *const u8 as *const c_char as *mut c_char,
        &raw mut temp as *mut c_char,
    );

    if stats.major_gcs > 0 as uint32_t {
        showStgWord64(
            stats.max_live_bytes as StgWord64,
            &raw mut temp as *mut c_char,
            r#true != 0,
        );

        statsPrintf(
            b"%16s bytes maximum residency (%u sample(s))\n\0" as *const u8 as *const c_char
                as *mut c_char,
            &raw mut temp as *mut c_char,
            stats.major_gcs,
        );
    }

    showStgWord64(
        stats.max_slop_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        r#true != 0,
    );

    statsPrintf(
        b"%16s bytes maximum slop\n\0" as *const u8 as *const c_char as *mut c_char,
        &raw mut temp as *mut c_char,
    );

    statsPrintf(
        b"%16llu MiB total memory in use (%llu MiB lost due to fragmentation)\n\n\0" as *const u8
            as *const c_char as *mut c_char,
        stats
            .max_mem_in_use_bytes
            .wrapping_div((1024 as c_int * 1024 as c_int) as uint64_t),
        (*sum)
            .fragmentation_bytes
            .wrapping_div((1024 as c_int * 1024 as c_int) as uint64_t),
    );

    statsPrintf(
        b"                                     Tot time (elapsed)  Avg pause  Max pause\n\0"
            as *const u8 as *const c_char as *mut c_char,
    );

    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        let mut gen_stats: *const GenerationSummaryStats =
            (*sum).gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;

        statsPrintf(
            b"  Gen %2d     %5d colls, %5d par   %6.3fs  %6.3fs     %3.4fs    %3.4fs\n\0"
                as *const u8 as *const c_char as *mut c_char,
            g,
            (*gen_stats).collections,
            (*gen_stats).par_collections,
            (*gen_stats).cpu_ns as c_double / TIME_RESOLUTION as c_double,
            (*gen_stats).elapsed_ns as c_double / TIME_RESOLUTION as c_double,
            (*gen_stats).avg_pause_ns as c_double / TIME_RESOLUTION as c_double,
            (*gen_stats).max_pause_ns as c_double / TIME_RESOLUTION as c_double,
        );

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let nonmoving_gen: uint32_t = RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t);
        let n_major_colls =
            (*(*sum).gc_summary_stats.offset(nonmoving_gen as isize)).collections as c_int;

        statsPrintf(
            b"  Gen %2d     %5d syncs,                      %6.3fs     %3.4fs    %3.4fs\n\0"
                as *const u8 as *const c_char as *mut c_char,
            nonmoving_gen,
            n_major_colls,
            stats.nonmoving_gc_sync_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
            stats.nonmoving_gc_sync_elapsed_ns as c_double
                / TIME_RESOLUTION as c_double
                / n_major_colls as c_double,
            stats.nonmoving_gc_sync_max_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
        );

        statsPrintf(
            b"  Gen %2d      concurrent,             %6.3fs  %6.3fs     %3.4fs    %3.4fs\n\0"
                as *const u8 as *const c_char as *mut c_char,
            nonmoving_gen,
            stats.nonmoving_gc_cpu_ns as c_double / TIME_RESOLUTION as c_double,
            stats.nonmoving_gc_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
            stats.nonmoving_gc_elapsed_ns as c_double
                / TIME_RESOLUTION as c_double
                / n_major_colls as c_double,
            stats.nonmoving_gc_max_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
        );
    }

    statsPrintf(b"\n\0" as *const u8 as *const c_char as *mut c_char);

    statsPrintf(
        b"  INIT    time  %7.3fs  (%7.3fs elapsed)\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.init_cpu_ns as c_double / TIME_RESOLUTION as c_double,
        stats.init_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
    );

    statsPrintf(
        b"  MUT     time  %7.3fs  (%7.3fs elapsed)\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.mutator_cpu_ns as c_double / TIME_RESOLUTION as c_double,
        stats.mutator_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
    );

    statsPrintf(
        b"  GC      time  %7.3fs  (%7.3fs elapsed)\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.gc_cpu_ns as c_double / TIME_RESOLUTION as c_double,
        stats.gc_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
    );

    if RtsFlags.GcFlags.useNonmoving {
        statsPrintf(
            b"  CONC GC time  %7.3fs  (%7.3fs elapsed)\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_cpu_ns as c_double / TIME_RESOLUTION as c_double,
            stats.nonmoving_gc_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
        );
    }

    statsPrintf(
        b"  EXIT    time  %7.3fs  (%7.3fs elapsed)\n\0" as *const u8 as *const c_char
            as *mut c_char,
        (*sum).exit_cpu_ns as c_double / TIME_RESOLUTION as c_double,
        (*sum).exit_elapsed_ns as c_double / TIME_RESOLUTION as c_double,
    );

    statsPrintf(
        b"  Total   time  %7.3fs  (%7.3fs elapsed)\n\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.cpu_ns as c_double / TIME_RESOLUTION as c_double,
        stats.elapsed_ns as c_double / TIME_RESOLUTION as c_double,
    );

    statsPrintf(
        b"  %%GC     time     %5.1f%%  (%.1f%% elapsed)\n\n\0" as *const u8 as *const c_char
            as *mut c_char,
        (*sum).gc_cpu_percent * 100 as c_int as c_double,
        (*sum).gc_elapsed_percent * 100 as c_int as c_double,
    );

    showStgWord64(
        (*sum).alloc_rate as StgWord64,
        &raw mut temp as *mut c_char,
        r#true != 0,
    );

    statsPrintf(
        b"  Alloc rate    %s bytes per MUT second\n\n\0" as *const u8 as *const c_char
            as *mut c_char,
        &raw mut temp as *mut c_char,
    );

    statsPrintf(
        b"  Productivity %5.1f%% of total user, %.1f%% of total elapsed\n\n\0" as *const u8
            as *const c_char as *mut c_char,
        (*sum).productivity_cpu_percent * 100 as c_int as c_double,
        (*sum).productivity_elapsed_percent * 100 as c_int as c_double,
    );

    if RtsFlags.MiscFlags.internalCounters {
        statsPrintf(
            b"Internal Counters require the threaded RTS\0" as *const u8 as *const c_char
                as *mut c_char,
        );
    }
}

unsafe fn report_machine_readable(mut sum: *const RTSSummaryStats) {
    let mut g: uint32_t = 0;

    statsPrintf(
        b" [(\"%s\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        b"bytes allocated\0" as *const u8 as *const c_char,
        stats.allocated_bytes,
    );

    statsPrintf(
        b" ,(\"num_GCs\", \"%u\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.gcs,
    );

    statsPrintf(
        b" ,(\"average_bytes_used\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).average_bytes_used,
    );

    statsPrintf(
        b" ,(\"max_bytes_used\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.max_live_bytes,
    );

    statsPrintf(
        b" ,(\"num_byte_usage_samples\", \"%u\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.major_gcs,
    );

    statsPrintf(
        b" ,(\"peak_megabytes_allocated\", \"%llu\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats
            .max_mem_in_use_bytes
            .wrapping_div((1024 as c_int * 1024 as c_int) as uint64_t),
    );

    statsPrintf(
        b" ,(\"init_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.init_cpu_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"init_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.init_elapsed_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"mut_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.mutator_cpu_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"mut_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.mutator_elapsed_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"GC_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.gc_cpu_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"GC_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.gc_elapsed_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"exit_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).exit_cpu_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"exit_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).exit_elapsed_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"total_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.cpu_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"total_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.elapsed_ns as c_double / 1000000000 as c_int as c_double,
    );

    statsPrintf(
        b" ,(\"major_gcs\", \"%u\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.major_gcs,
    );

    statsPrintf(
        b" ,(\"allocated_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.allocated_bytes,
    );

    statsPrintf(
        b" ,(\"max_live_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.max_live_bytes,
    );

    statsPrintf(
        b" ,(\"max_large_objects_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.max_large_objects_bytes,
    );

    statsPrintf(
        b" ,(\"max_compact_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.max_compact_bytes,
    );

    statsPrintf(
        b" ,(\"max_slop_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.max_slop_bytes,
    );

    statsPrintf(
        b" ,(\"max_mem_in_use_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.max_mem_in_use_bytes,
    );

    statsPrintf(
        b" ,(\"cumulative_live_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.cumulative_live_bytes,
    );

    statsPrintf(
        b" ,(\"copied_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.copied_bytes,
    );

    statsPrintf(
        b" ,(\"par_copied_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        stats.par_copied_bytes,
    );

    statsPrintf(
        b" ,(\"cumulative_par_max_copied_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.cumulative_par_max_copied_bytes,
    );

    statsPrintf(
        b" ,(\"cumulative_par_balanced_copied_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        stats.cumulative_par_balanced_copied_bytes,
    );

    statsPrintf(
        b" ,(\"gc_cpu_percent\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).gc_cpu_percent,
    );

    statsPrintf(
        b" ,(\"gc_wall_percent\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).gc_cpu_percent,
    );

    statsPrintf(
        b" ,(\"fragmentation_bytes\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).fragmentation_bytes,
    );

    statsPrintf(
        b" ,(\"alloc_rate\", \"%llu\")\n\0" as *const u8 as *const c_char as *mut c_char,
        (*sum).alloc_rate,
    );

    statsPrintf(
        b" ,(\"productivity_cpu_percent\", \"%f\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        (*sum).productivity_cpu_percent,
    );

    statsPrintf(
        b" ,(\"productivity_wall_percent\", \"%f\")\n\0" as *const u8 as *const c_char
            as *mut c_char,
        (*sum).productivity_elapsed_percent,
    );

    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        let mut gc_sum: *const GenerationSummaryStats =
            (*sum).gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;

        statsPrintf(
            b" ,(\"gen_%u_collections\", \"%u\")\n\0" as *const u8 as *const c_char as *mut c_char,
            g,
            (*gc_sum).collections,
        );

        statsPrintf(
            b" ,(\"gen_%u_par_collections\", \"%u\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            g,
            (*gc_sum).par_collections,
        );

        statsPrintf(
            b" ,(\"gen_%u_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
            g,
            (*gc_sum).cpu_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"gen_%u_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char as *mut c_char,
            g,
            (*gc_sum).elapsed_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"gen_%u_max_pause_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            g,
            (*gc_sum).max_pause_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"gen_%u_avg_pause_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            g,
            (*gc_sum).avg_pause_ns as c_double / 1000000000 as c_int as c_double,
        );

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let n_major_colls = (*(*sum)
            .gc_summary_stats
            .offset(RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) as isize))
        .collections as c_int;

        statsPrintf(
            b" ,(\"nonmoving_sync_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_sync_elapsed_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_sync_max_pause_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_sync_max_elapsed_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_sync_avg_pause_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_sync_elapsed_ns as c_double
                / 1000000000 as c_int as c_double
                / n_major_colls as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_concurrent_cpu_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_cpu_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_concurrent_wall_seconds\", \"%f\")\n\0" as *const u8 as *const c_char
                as *mut c_char,
            stats.nonmoving_gc_elapsed_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_concurrent_max_pause_seconds\", \"%f\")\n\0" as *const u8
                as *const c_char as *mut c_char,
            stats.nonmoving_gc_max_elapsed_ns as c_double / 1000000000 as c_int as c_double,
        );

        statsPrintf(
            b" ,(\"nonmoving_concurrent_avg_pause_seconds\", \"%f\")\n\0" as *const u8
                as *const c_char as *mut c_char,
            stats.nonmoving_gc_elapsed_ns as c_double
                / 1000000000 as c_int as c_double
                / n_major_colls as c_double,
        );
    }

    statsPrintf(b" ]\n\0" as *const u8 as *const c_char as *mut c_char);
}

unsafe fn report_one_line(mut sum: *const RTSSummaryStats) {
    statsPrintf(
        b"<<ghc: %llu bytes, %u GCs, %llu/%llu avg/max bytes residency (%u samples), %lluM in use, %.3f INIT (%.3f elapsed), %.3f MUT (%.3f elapsed), %.3f GC (%.3f elapsed) :ghc>>\n\0"
            as *const u8 as *const c_char as *mut c_char,
        stats.allocated_bytes,
        stats.gcs,
        (*sum).average_bytes_used,
        stats.max_live_bytes,
        stats.major_gcs,
        stats
            .max_mem_in_use_bytes
            .wrapping_div(
                (1024 as c_int * 1024 as c_int) as uint64_t,
            ),
        stats.init_cpu_ns as c_double

            / TIME_RESOLUTION as c_double,
        stats.init_elapsed_ns as c_double

            / TIME_RESOLUTION as c_double,
        stats.mutator_cpu_ns as c_double

            / TIME_RESOLUTION as c_double,
        stats.mutator_elapsed_ns as c_double

            / TIME_RESOLUTION as c_double,
        stats.gc_cpu_ns as c_double

            / TIME_RESOLUTION as c_double,
        stats.gc_elapsed_ns as c_double

            / TIME_RESOLUTION as c_double,
    );
}

unsafe fn stat_exitReport() {
    let mut sum = RTSSummaryStats_ {
        rp_cpu_ns: 0,
        rp_elapsed_ns: 0,
        hc_cpu_ns: 0,
        hc_elapsed_ns: 0,
        exit_cpu_ns: 0,
        exit_elapsed_ns: 0,
        gc_cpu_percent: 0.,
        gc_elapsed_percent: 0.,
        fragmentation_bytes: 0,
        average_bytes_used: 0,
        alloc_rate: 0,
        productivity_cpu_percent: 0.,
        productivity_elapsed_percent: 0.,
        gc_summary_stats: null_mut::<GenerationSummaryStats>(),
    };

    init_RTSSummaryStats(&raw mut sum);

    if RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t {
        let mut now_cpu_ns: Time = 0;
        let mut now_elapsed_ns: Time = 0;
        getProcessTimes(&raw mut now_cpu_ns, &raw mut now_elapsed_ns);
        stats.cpu_ns = now_cpu_ns - start_init_cpu;
        stats.elapsed_ns = now_elapsed_ns - start_init_elapsed;

        if stats.cpu_ns <= 0 as Time {
            stats.cpu_ns = 1 as Time;
        }

        if stats.elapsed_ns <= 0 as Time {
            stats.elapsed_ns = 1 as Time;
        }

        let mut exit_gc_cpu: Time = stats.gc_cpu_ns - start_exit_gc_cpu;
        let mut exit_gc_elapsed: Time = stats.gc_elapsed_ns - start_exit_gc_elapsed;
        sum.exit_cpu_ns = end_exit_cpu - start_exit_cpu - exit_gc_cpu;
        sum.exit_elapsed_ns = end_exit_elapsed - start_exit_elapsed - exit_gc_elapsed;
        stats.mutator_cpu_ns = start_exit_cpu
            - end_init_cpu
            - (stats.gc_cpu_ns - exit_gc_cpu)
            - stats.nonmoving_gc_cpu_ns;
        stats.mutator_elapsed_ns =
            start_exit_elapsed - end_init_elapsed - (stats.gc_elapsed_ns - exit_gc_elapsed);

        if stats.mutator_cpu_ns < 0 as Time {
            stats.mutator_cpu_ns = 0 as Time;
        }

        let mut prof_cpu: Time = sum.rp_cpu_ns + sum.hc_cpu_ns;
        let mut prof_elapsed: Time = sum.rp_elapsed_ns + sum.hc_elapsed_ns;
        stats.gc_cpu_ns -= prof_cpu;
        stats.gc_elapsed_ns -= prof_elapsed;

        let mut tot_alloc_bytes: uint64_t =
            calcTotalAllocated().wrapping_mul(size_of::<W_>() as uint64_t);
        stats.gc.allocated_bytes = tot_alloc_bytes.wrapping_sub(stats.allocated_bytes);
        stats.allocated_bytes = tot_alloc_bytes;

        if RtsFlags.GcFlags.giveStats >= VERBOSE_GC_STATS as uint32_t {
            statsPrintf(
                b"%9llu %9.9s %9.9s\0" as *const u8 as *const c_char as *mut c_char,
                stats.gc.allocated_bytes,
                b"\0" as *const u8 as *const c_char,
                b"\0" as *const u8 as *const c_char,
            );

            statsPrintf(
                b" %6.3f %6.3f\n\n\0" as *const u8 as *const c_char as *mut c_char,
                0.0f64,
                0.0f64,
            );
        }

        sum.gc_cpu_percent = (stats.gc_cpu_ns / stats.cpu_ns) as c_double;
        sum.gc_elapsed_percent = (stats.gc_elapsed_ns / stats.elapsed_ns) as c_double;
        sum.fragmentation_bytes = peak_mblocks_allocated
            .wrapping_mul(BLOCKS_PER_MBLOCK)
            .wrapping_mul(BLOCK_SIZE_W as W_)
            .wrapping_sub(hw_alloc_blocks.wrapping_mul(BLOCK_SIZE_W as W_))
            .wrapping_mul(size_of::<W_>() as uint64_t);

        sum.average_bytes_used = (if stats.major_gcs == 0 as uint32_t {
            0 as uint64_t
        } else {
            stats
                .cumulative_live_bytes
                .wrapping_div(stats.major_gcs as uint64_t)
        });

        sum.alloc_rate = (if stats.mutator_cpu_ns == 0 as Time {
            0 as uint64_t
        } else {
            (stats.allocated_bytes as c_double
                / (stats.mutator_cpu_ns as c_double / TIME_RESOLUTION as c_double))
                as uint64_t
        });

        sum.productivity_cpu_percent =
            (stats.cpu_ns - stats.gc_cpu_ns - stats.init_cpu_ns - sum.exit_cpu_ns) as c_double
                / TIME_RESOLUTION as c_double
                / (stats.cpu_ns as c_double / TIME_RESOLUTION as c_double);
        sum.productivity_elapsed_percent =
            (stats.elapsed_ns - stats.gc_elapsed_ns - stats.init_elapsed_ns - sum.exit_elapsed_ns)
                as c_double
                / TIME_RESOLUTION as c_double
                / (stats.elapsed_ns as c_double / TIME_RESOLUTION as c_double);
        let mut g: uint32_t = 0 as uint32_t;

        while g < RtsFlags.GcFlags.generations {
            let mut r#gen: *const generation = generations.offset(g as isize) as *mut generation;
            let mut gen_stats: *mut GenerationSummaryStats =
                sum.gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;
            (*gen_stats).collections = (*r#gen).collections;
            (*gen_stats).par_collections = (*r#gen).par_collections;
            (*gen_stats).cpu_ns = *GC_coll_cpu.offset(g as isize);
            (*gen_stats).elapsed_ns = *GC_coll_elapsed.offset(g as isize);
            (*gen_stats).max_pause_ns = *GC_coll_max_pause.offset(g as isize);

            (*gen_stats).avg_pause_ns = if (*r#gen).collections == 0 as uint32_t {
                0 as Time
            } else {
                *GC_coll_elapsed.offset(g as isize) / (*r#gen).collections as Time
            };

            g = g.wrapping_add(1);
        }

        if RtsFlags.GcFlags.giveStats >= SUMMARY_GC_STATS as uint32_t {
            report_summary(&raw mut sum);
        }

        if RtsFlags.GcFlags.giveStats == ONELINE_GC_STATS as uint32_t {
            if RtsFlags.MiscFlags.machineReadable {
                report_machine_readable(&raw mut sum);
            } else {
                report_one_line(&raw mut sum);
            }
        }

        statsFlush();
        statsClose();
    }

    free_RTSSummaryStats(&raw mut sum);

    if !GC_coll_cpu.is_null() {
        stgFree(GC_coll_cpu as *mut c_void);
        GC_coll_cpu = null_mut::<Time>();
    }

    if !GC_coll_elapsed.is_null() {
        stgFree(GC_coll_elapsed as *mut c_void);
        GC_coll_elapsed = null_mut::<Time>();
    }

    if !GC_coll_max_pause.is_null() {
        stgFree(GC_coll_max_pause as *mut c_void);
        GC_coll_max_pause = null_mut::<Time>();
    }
}

unsafe fn stat_exit() {}

unsafe fn statDescribeGens() {
    let mut g: uint32_t = 0;
    let mut r#mut: uint32_t = 0;
    let mut lge: uint32_t = 0;
    let mut compacts: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut gen_slop: W_ = 0;
    let mut tot_live: W_ = 0;
    let mut tot_slop: W_ = 0;
    let mut gen_live: W_ = 0;
    let mut gen_blocks: W_ = 0;
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();

    debugBelch(
        b"----------------------------------------------------------------------\n  Gen     Max  Mut-list  Blocks    Large  Compacts      Live      Slop\n       Blocks     Bytes          Objects                              \n----------------------------------------------------------------------\n\0"
            as *const u8 as *const c_char,
    );

    tot_live = 0 as W_;
    tot_slop = 0 as W_;
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        r#gen = generations.offset(g as isize) as *mut generation;
        bd = (*r#gen).large_objects;
        lge = 0 as uint32_t;

        while !bd.is_null() {
            lge = lge.wrapping_add(1);
            bd = (*bd).link as *mut bdescr;
        }

        bd = (*r#gen).compact_objects;
        compacts = 0 as uint32_t;

        while !bd.is_null() {
            compacts = compacts.wrapping_add(1);
            bd = (*bd).link as *mut bdescr;
        }

        gen_live = genLiveWords(r#gen) as W_;
        gen_blocks = genLiveBlocks(r#gen) as W_;
        r#mut = 0 as uint32_t;
        i = 0 as uint32_t;

        while i < getNumCapabilities() as uint32_t {
            r#mut = (r#mut as StgWord).wrapping_add(countOccupied(
                *(*getCapability(i)).mut_lists.offset(g as isize),
            )) as uint32_t as uint32_t;

            bd = (*getCapability(i)).pinned_object_block;

            if !bd.is_null() {
                gen_live = gen_live.wrapping_add(
                    (*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as W_,
                );

                gen_blocks = gen_blocks.wrapping_add((*bd).blocks as W_);
            }

            gen_live = (gen_live as StgWord).wrapping_add(gcThreadLiveWords(i, g)) as W_ as W_;
            gen_blocks = (gen_blocks as StgWord).wrapping_add(gcThreadLiveBlocks(i, g)) as W_ as W_;
            i = i.wrapping_add(1);
        }

        debugBelch(
            b"%5d %7llu %9d\0" as *const u8 as *const c_char,
            g,
            (*r#gen).max_blocks,
            r#mut,
        );

        gen_slop = gen_blocks
            .wrapping_mul(BLOCK_SIZE_W as W_)
            .wrapping_sub(gen_live);

        debugBelch(
            b"%8llu %8d  %8d %9llu %9llu\n\0" as *const u8 as *const c_char,
            gen_blocks,
            lge,
            compacts,
            gen_live.wrapping_mul(size_of::<W_>() as W_),
            gen_slop.wrapping_mul(size_of::<W_>() as W_),
        );

        tot_live = tot_live.wrapping_add(gen_live);
        tot_slop = tot_slop.wrapping_add(gen_slop);
        g = g.wrapping_add(1);
    }

    debugBelch(
        b"----------------------------------------------------------------------\n\0" as *const u8
            as *const c_char,
    );

    debugBelch(
        b"%51s%9llu %9llu\n\0" as *const u8 as *const c_char,
        b"\0" as *const u8 as *const c_char,
        tot_live.wrapping_mul(size_of::<W_>() as W_),
        tot_slop.wrapping_mul(size_of::<W_>() as W_),
    );

    debugBelch(
        b"----------------------------------------------------------------------\n\0" as *const u8
            as *const c_char,
    );

    debugBelch(b"\n\0" as *const u8 as *const c_char);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getAllocations() -> uint64_t {
    let mut n: StgWord64 = stats.allocated_bytes as StgWord64;

    return n as uint64_t;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    return (RtsFlags.GcFlags.giveStats != NO_GC_STATS as uint32_t) as c_int;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStats(mut s: *mut RTSStats) {
    let mut current_elapsed: Time = 0 as Time;
    let mut current_cpu: Time = 0 as Time;
    *s = stats;
    getProcessTimes(&raw mut current_cpu, &raw mut current_elapsed);
    (*s).cpu_ns = current_cpu - end_init_cpu;
    (*s).elapsed_ns = current_elapsed - end_init_elapsed;
    (*s).mutator_cpu_ns = current_cpu - end_init_cpu - stats.gc_cpu_ns - stats.nonmoving_gc_cpu_ns;
    (*s).mutator_elapsed_ns = current_elapsed - end_init_elapsed - stats.gc_elapsed_ns;
}

unsafe fn statsPrintf(mut s: *mut c_char, mut args: ...) -> c_int {
    let mut ret = 0 as c_int;
    let mut sf = RtsFlags.GcFlags.statsFile;
    let mut ap: VaListImpl;
    ap = args.clone();

    if sf.is_null() {
        ret = vdebugBelch(s, ap.as_va_list());
    } else {
        ret = vfprintf(sf, s, ap.as_va_list());
    }

    return ret;
}

unsafe fn statsFlush() {
    let mut sf = RtsFlags.GcFlags.statsFile;

    if !sf.is_null() {
        fflush(sf);
    }
}

unsafe fn statsClose() {
    let mut sf = RtsFlags.GcFlags.statsFile;

    if !sf.is_null() {
        fclose(sf);
    }
}
