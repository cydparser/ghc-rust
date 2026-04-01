use crate::capability::getCapability;
use crate::ffi::rts::flags::{
    NO_GC_STATS, ONELINE_GC_STATS, RtsFlags, SUMMARY_GC_STATS, VERBOSE_GC_STATS,
};
use crate::ffi::rts::messages::{barf, debugBelch, vdebugBelch};
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::storage::block::{
    BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, MBLOCK_SIZE, bdescr,
};
use crate::ffi::rts::storage::gc::{generation, generations};
use crate::ffi::rts::storage::heap_alloc::gc_alloc_block_sync;
use crate::ffi::rts::storage::m_block::{mblocks_allocated, peak_mblocks_allocated};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::Time;
use crate::ffi::rts::time::{TIME_RESOLUTION, Time, getProcessElapsedTime};
use crate::ffi::rts::{_assertFail, _warnFail};
use crate::ffi::rts_api::{_RTSStats, Capability, GCDetails_, RTSStats};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgInt, StgWord, StgWord64};
use crate::get_time::{getCurrentThreadCPUTime, getPageFaults, getProcessTimes};
use crate::prelude::*;
use crate::profiling::prof_file;
use crate::rts_flags::rtsConfig;
use crate::rts_utils::{showStgWord64, stgFree, stgMallocBytes};
use crate::sm::block_alloc::{hw_alloc_blocks, n_alloc_blocks};
use crate::sm::gc::{waitForGcThreads_spin, waitForGcThreads_yield, whitehole_gc_spin};
use crate::sm::gc_thread::gc_thread;
use crate::sm::storage::{
    calcTotalAllocated, calcTotalCompactW, calcTotalLargeObjectsW, countOccupied,
    gcThreadLiveBlocks, gcThreadLiveWords, genLiveBlocks, genLiveWords, updateNurseriesStats,
};
use crate::sparks::SparkCounters;
use crate::sparks::SparkCounters;
use crate::stats::{
    GenerationSummaryStats, GenerationSummaryStats_, RTSSummaryStats, RTSSummaryStats_,
};
use crate::task::{all_tasks_mutex, peakWorkerCount, taskCount, workerCount};
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
    pub(crate) bound_task_count: u32,
    pub(crate) sparks_count: u64,
    pub(crate) sparks: SparkCounters,
    pub(crate) work_balance: f64,
    pub(crate) fragmentation_bytes: u64,
    pub(crate) average_bytes_used: u64,
    pub(crate) alloc_rate: u64,
    pub(crate) productivity_cpu_percent: f64,
    pub(crate) productivity_elapsed_percent: f64,
    pub(crate) gc_summary_stats: *mut GenerationSummaryStats,
}

pub(crate) type GenerationSummaryStats = GenerationSummaryStats_;

/// cbindgen:no-export
pub(crate) struct GenerationSummaryStats_ {
    pub(crate) collections: u32,
    pub(crate) par_collections: u32,
    pub(crate) cpu_ns: Time,
    pub(crate) elapsed_ns: Time,
    pub(crate) max_pause_ns: Time,
    pub(crate) avg_pause_ns: Time,
    pub(crate) sync_spin: u64,
    pub(crate) sync_yield: u64,
}

static mut stats_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut start_init_elapsed: Time = 0;

static mut start_nonmoving_gc_cpu: Time = 0;

static mut start_init_cpu: Time = 0;

static mut end_init_elapsed: Time = 0;

static mut end_init_cpu: Time = 0;

static mut end_exit_cpu: Time = 0;

static mut end_exit_elapsed: Time = 0;

static mut start_exit_cpu: Time = 0;

static mut start_exit_elapsed: Time = 0;

static mut start_exit_gc_elapsed: Time = 0;

static mut start_exit_gc_cpu: Time = 0;

static mut start_nonmoving_gc_sync_elapsed: Time = 0;

static mut start_nonmoving_gc_elapsed: Time = 0;

static mut RP_tot_time: Time = 0;

static mut RP_start_time: Time = 0;

static mut RPe_start_time: Time = 0;

static mut RPe_tot_time: Time = 0;

static mut HC_start_time: Time = 0;

static mut HC_tot_time: Time = 0;

static mut HCe_tot_time: Time = 0;

static mut HCe_start_time: Time = 0;

static mut whitehole_lockClosure_spin: StgWord64 = 0;

static mut whitehole_lockClosure_yield: StgWord64 = 0;

static mut whitehole_threadPaused_spin: StgWord64 = 0;

static mut whitehole_executeMessage_spin: StgWord64 = 0;

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

static mut GC_end_faults: W_ = 0;

static mut GC_coll_cpu: *mut Time = null_mut::<Time>();

static mut GC_coll_elapsed: *mut Time = null_mut::<Time>();

static mut GC_coll_max_pause: *mut Time = null_mut::<Time>();

unsafe fn stat_getElapsedTime() -> Time {
    return getProcessElapsedTime() - start_init_elapsed;
}

unsafe fn mut_user_time_during_RP() -> f64 {
    return (RP_start_time - stats.gc_cpu_ns - RP_tot_time) as f64 / TIME_RESOLUTION as f64;
}

unsafe fn initStats0() {
    initMutex(&raw mut stats_mutex);
    start_init_cpu = 0;
    start_init_elapsed = 0;
    end_init_cpu = 0;
    end_init_elapsed = 0;
    start_nonmoving_gc_cpu = 0;
    start_nonmoving_gc_elapsed = 0;
    start_nonmoving_gc_sync_elapsed = 0;
    start_exit_cpu = 0;
    start_exit_elapsed = 0;
    start_exit_gc_cpu = 0;
    start_exit_gc_elapsed = 0;
    end_exit_cpu = 0;
    end_exit_elapsed = 0;
    RP_start_time = 0;
    RP_tot_time = 0;
    RPe_start_time = 0;
    RPe_tot_time = 0;
    HC_start_time = 0;
    HC_tot_time = 0;
    HCe_start_time = 0;
    HCe_tot_time = 0;
    GC_end_faults = 0;

    stats = _RTSStats {
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
}

unsafe fn initStats1() {
    if RtsFlags.GcFlags.giveStats >= VERBOSE_GC_STATS as u32 {
        statsPrintf(
            c"    Alloc    Copied     Live     GC     GC      TOT      TOT  Page Flts\n".as_ptr(),
        );

        statsPrintf(c"    bytes     bytes     bytes   user   elap     user     elap\n".as_ptr());
    }

    GC_coll_cpu = stgMallocBytes(
        (size_of::<Time>() as usize).wrapping_mul(RtsFlags.GcFlags.generations as usize),
        c"initStats".as_ptr(),
    ) as *mut Time;

    GC_coll_elapsed = stgMallocBytes(
        (size_of::<Time>() as usize).wrapping_mul(RtsFlags.GcFlags.generations as usize),
        c"initStats".as_ptr(),
    ) as *mut Time;

    GC_coll_max_pause = stgMallocBytes(
        (size_of::<Time>() as usize).wrapping_mul(RtsFlags.GcFlags.generations as usize),
        c"initStats".as_ptr(),
    ) as *mut Time;

    initGenerationStats();
}

unsafe fn initGenerationStats() {
    let mut i: u32 = 0;

    while i < RtsFlags.GcFlags.generations {
        *GC_coll_cpu.offset(i as isize) = 0;
        *GC_coll_elapsed.offset(i as isize) = 0;
        *GC_coll_max_pause.offset(i as isize) = 0;
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
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            271,
            __r,
        );
    }

    getProcessTimes(&raw mut start_exit_cpu, &raw mut start_exit_elapsed);
    start_exit_gc_elapsed = stats.gc_elapsed_ns;
    start_exit_gc_cpu = stats.gc_cpu_ns;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            275,
        );
    }
}

unsafe fn stat_endExit() {
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            286,
            __r,
        );
    }

    getProcessTimes(&raw mut end_exit_cpu, &raw mut end_exit_elapsed);

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            288,
        );
    }
}

unsafe fn stat_startGCSync(mut gct: *mut gc_thread) {
    (*gct).gc_sync_start_elapsed = getProcessElapsedTime();
}

unsafe fn stat_startNonmovingGc() {
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            300,
            __r,
        );
    }

    start_nonmoving_gc_cpu = getCurrentThreadCPUTime();
    start_nonmoving_gc_elapsed = getProcessElapsedTime();

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            303,
        );
    }
}

unsafe fn stat_endNonmovingGc() {
    let mut cpu = getCurrentThreadCPUTime();
    let mut elapsed = getProcessElapsedTime();
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            312,
            __r,
        );
    }

    stats.gc.nonmoving_gc_elapsed_ns = elapsed - start_nonmoving_gc_elapsed;
    stats.nonmoving_gc_elapsed_ns += stats.gc.nonmoving_gc_elapsed_ns;
    stats.gc.nonmoving_gc_cpu_ns = cpu - start_nonmoving_gc_cpu;
    stats.nonmoving_gc_cpu_ns += stats.gc.nonmoving_gc_cpu_ns;

    stats.nonmoving_gc_max_elapsed_ns = ({
        let mut _a: Time = stats.gc.nonmoving_gc_elapsed_ns as Time;
        let mut _b: Time = stats.nonmoving_gc_max_elapsed_ns as Time;

        if _a <= _b { _b } else { _a as Time }
    });

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            322,
        );
    }
}

unsafe fn stat_startNonmovingGcSync() {
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            328,
            __r,
        );
    }

    start_nonmoving_gc_sync_elapsed = getProcessElapsedTime();

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            330,
        );
    }

    traceConcSyncBegin();
}

unsafe fn stat_endNonmovingGcSync() {
    let mut end_elapsed = getProcessElapsedTime();
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            338,
            __r,
        );
    }

    stats.gc.nonmoving_gc_sync_elapsed_ns = end_elapsed - start_nonmoving_gc_sync_elapsed;
    stats.nonmoving_gc_sync_elapsed_ns += stats.gc.nonmoving_gc_sync_elapsed_ns;

    stats.nonmoving_gc_sync_max_elapsed_ns = ({
        let mut _a: Time = stats.gc.nonmoving_gc_sync_elapsed_ns as Time;
        let mut _b: Time = stats.nonmoving_gc_sync_max_elapsed_ns as Time;

        if _a <= _b { _b } else { _a as Time }
    });

    let mut sync_elapsed: Time = stats.gc.nonmoving_gc_sync_elapsed_ns;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            345,
        );
    }

    if RtsFlags.GcFlags.giveStats == VERBOSE_GC_STATS as u32 {
        statsPrintf(
            c"# sync %6.3f\n".as_ptr(),
            sync_elapsed as f64 / TIME_RESOLUTION as f64,
        );
    }

    traceConcSyncEnd();
}

unsafe fn stat_startGCWorker(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as i32 != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_start_cpu = getCurrentThreadCPUTime();
    }
}

unsafe fn stat_endGCWorker(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as i32 != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_end_cpu = getCurrentThreadCPUTime();

        if ((*gct).gc_end_cpu >= (*gct).gc_start_cpu) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Stats.c".as_ptr(), 416);
        }
    }
}

unsafe fn stat_startGC(mut cap: *mut Capability, mut gct: *mut gc_thread) {
    if RtsFlags.GcFlags.ringBell {
        debugBelch(c"\u{7}".as_ptr());
    }

    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as i32 != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        (*gct).gc_start_cpu = getCurrentThreadCPUTime();
    }

    (*gct).gc_start_elapsed = getProcessElapsedTime();

    traceEventGcStartAtT(
        cap,
        ((*gct).gc_start_elapsed - start_init_elapsed) as StgWord64,
    );

    if RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 {
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
    mut r#gen: u32,
    mut par_n_threads: u32,
    mut gc_threads: *mut *mut gc_thread,
    mut par_max_copied: W_,
    mut par_balanced_copied: W_,
    mut any_work: W_,
    mut scav_find_work: W_,
    mut max_n_todo_overflow: W_,
) {
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            463,
            __r,
        );
    }

    stats.gc.r#gen = r#gen;
    stats.gc.threads = par_n_threads;

    let mut tot_alloc_bytes: u64 = calcTotalAllocated().wrapping_mul(size_of::<W_>() as u64);
    stats.gc.allocated_bytes = tot_alloc_bytes.wrapping_sub(stats.allocated_bytes);
    stats.gc.live_bytes = live.wrapping_mul(size_of::<W_>() as W_) as u64;
    stats.gc.large_objects_bytes =
        calcTotalLargeObjectsW().wrapping_mul(size_of::<W_>() as StgWord) as u64;
    stats.gc.compact_bytes = calcTotalCompactW().wrapping_mul(size_of::<W_>() as StgWord) as u64;
    stats.gc.slop_bytes = slop.wrapping_mul(size_of::<W_>() as W_) as u64;
    stats.gc.mem_in_use_bytes = mblocks_allocated.wrapping_mul(MBLOCK_SIZE as W_) as u64;
    stats.gc.copied_bytes = copied.wrapping_mul(size_of::<W_>() as W_) as u64;
    stats.gc.par_max_copied_bytes = par_max_copied.wrapping_mul(size_of::<W_>() as W_) as u64;
    stats.gc.par_balanced_copied_bytes =
        par_balanced_copied.wrapping_mul(size_of::<W_>() as W_) as u64;
    stats.gc.block_fragmentation_bytes = mblocks_allocated
        .wrapping_mul(BLOCKS_PER_MBLOCK)
        .wrapping_sub(n_alloc_blocks)
        .wrapping_mul(BLOCK_SIZE as W_) as u64;

    let mut stats_enabled =
        RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 || rtsConfig.gcDoneHook.is_some();

    if stats_enabled as i32 != 0 || RtsFlags.ProfFlags.doHeapProfile != 0 {
        let mut current_cpu: Time = 0;
        let mut current_elapsed: Time = 0;
        getProcessTimes(&raw mut current_cpu, &raw mut current_elapsed);
        stats.cpu_ns = current_cpu - start_init_cpu;
        stats.elapsed_ns = current_elapsed - start_init_elapsed;
        stats.gc.sync_elapsed_ns =
            (*initiating_gct).gc_start_elapsed - (*initiating_gct).gc_sync_start_elapsed;
        stats.gc.elapsed_ns = current_elapsed - (*initiating_gct).gc_start_elapsed;
        stats.gc.cpu_ns = 0;

        let mut i = 0;

        while (i as u32) < par_n_threads {
            let mut gct = *gc_threads.offset(i as isize);

            if ((*gct).gc_end_cpu >= (*gct).gc_start_cpu) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Stats.c".as_ptr(), 514);
            }

            stats.gc.cpu_ns += (*gct).gc_end_cpu - (*gct).gc_start_cpu;
            (*gct).gc_end_cpu = 0;
            (*gct).gc_start_cpu = 0;
            i = i.wrapping_add(1);
        }
    }

    stats.gcs = stats.gcs.wrapping_add(1);
    stats.allocated_bytes = tot_alloc_bytes;
    stats.max_mem_in_use_bytes = peak_mblocks_allocated.wrapping_mul(MBLOCK_SIZE as W_) as u64;
    *GC_coll_cpu.offset(r#gen as isize) += stats.gc.cpu_ns;
    *GC_coll_elapsed.offset(r#gen as isize) += stats.gc.elapsed_ns;

    if *GC_coll_max_pause.offset(r#gen as isize) < stats.gc.elapsed_ns {
        *GC_coll_max_pause.offset(r#gen as isize) = stats.gc.elapsed_ns;
    }

    stats.copied_bytes = stats.copied_bytes.wrapping_add(stats.gc.copied_bytes);

    if par_n_threads > 1 {
        stats.par_copied_bytes = stats.par_copied_bytes.wrapping_add(stats.gc.copied_bytes);
        stats.cumulative_par_max_copied_bytes = stats
            .cumulative_par_max_copied_bytes
            .wrapping_add(stats.gc.par_max_copied_bytes);
        stats.cumulative_par_balanced_copied_bytes = stats
            .cumulative_par_balanced_copied_bytes
            .wrapping_add(stats.gc.par_balanced_copied_bytes);
        stats.any_work = stats.any_work.wrapping_add(any_work as u64);
        stats.scav_find_work = stats.scav_find_work.wrapping_add(scav_find_work as u64);
        stats.max_n_todo_overflow = stats.max_n_todo_overflow.wrapping_add(
            ({
                let mut _a: W_ = max_n_todo_overflow as W_;
                let mut _b: W_ = stats.max_n_todo_overflow as W_;

                if _a <= _b { _b } else { _a as W_ }
            }) as u64,
        );
    }

    stats.gc_cpu_ns += stats.gc.cpu_ns;
    stats.gc_elapsed_ns += stats.gc.elapsed_ns;

    if r#gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) {
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

        if r#gen == RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) {
            traceEventHeapLive(cap, CAPSET_HEAP_DEFAULT, stats.gc.live_bytes as W_);
        }

        if RtsFlags.GcFlags.giveStats == VERBOSE_GC_STATS as u32 {
            let mut faults = getPageFaults();

            statsPrintf(
                c"%9llu %9llu %9llu".as_ptr(),
                stats.gc.allocated_bytes,
                stats.gc.copied_bytes,
                stats.gc.live_bytes,
            );

            statsPrintf(
                c" %6.3f %6.3f %8.3f %8.3f %4llu %4llu  (Gen: %2d)\n".as_ptr(),
                stats.gc.cpu_ns as f64 / TIME_RESOLUTION as f64,
                stats.gc.elapsed_ns as f64 / TIME_RESOLUTION as f64,
                stats.cpu_ns as f64 / TIME_RESOLUTION as f64,
                stats.elapsed_ns as f64 / TIME_RESOLUTION as f64,
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

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            644,
        );
    }
}

unsafe fn stat_startRP() {
    let mut user: Time = 0;
    let mut elapsed: Time = 0;
    getProcessTimes(&raw mut user, &raw mut elapsed);

    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            657,
            __r,
        );
    }

    RP_start_time = user;
    RPe_start_time = elapsed;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            660,
        );
    }
}

unsafe fn stat_endRP(mut retainerGeneration: u32, mut maxStackSize: i32, mut averageNumVisit: f64) {
    let mut user: Time = 0;
    let mut elapsed: Time = 0;
    getProcessTimes(&raw mut user, &raw mut elapsed);

    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            678,
            __r,
        );
    }

    RP_tot_time += user - RP_start_time;
    RPe_tot_time += elapsed - RPe_start_time;

    let mut mut_time_during_RP = mut_user_time_during_RP();

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            682,
        );
    }

    fprintf(
        prof_file,
        c"Retainer Profiling: %d, at %f seconds\n".as_ptr(),
        retainerGeneration,
        mut_time_during_RP,
    );

    fprintf(
        prof_file,
        c"\tMax auxiliary stack size = %u\n".as_ptr(),
        maxStackSize,
    );

    fprintf(
        prof_file,
        c"\tAverage number of visits per object = %f\n".as_ptr(),
        averageNumVisit,
    );
}

unsafe fn stat_startHeapCensus() {
    let mut user: Time = 0;
    let mut elapsed: Time = 0;
    getProcessTimes(&raw mut user, &raw mut elapsed);

    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            702,
            __r,
        );
    }

    HC_start_time = user;
    HCe_start_time = elapsed;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            705,
        );
    }
}

unsafe fn stat_endHeapCensus() {
    let mut user: Time = 0;
    let mut elapsed: Time = 0;
    getProcessTimes(&raw mut user, &raw mut elapsed);

    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            719,
            __r,
        );
    }

    HC_tot_time += user - HC_start_time;
    HCe_tot_time += elapsed - HCe_start_time;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            722,
        );
    }
}

static mut TAGGED_PTR_1: StgInt = 0;

static mut RIGHT_ARITY_1: StgInt = 1;

static mut SLOW_CALLS_1: StgInt = 1;

static mut TAGGED_PTR_2: StgInt = 0;

static mut RIGHT_ARITY_2: StgInt = 1;

static mut SLOW_CALLS_2: StgInt = 1;

static mut TOTAL_CALLS: StgInt = 1;

unsafe fn init_RTSSummaryStats(mut sum: *mut RTSSummaryStats) {
    let sizeof_gc_summary_stats: usize = (RtsFlags.GcFlags.generations as usize)
        .wrapping_mul(size_of::<GenerationSummaryStats>() as usize);
    memset(sum as *mut c_void, 0, size_of::<RTSSummaryStats>() as usize);

    (*sum).gc_summary_stats = stgMallocBytes(
        sizeof_gc_summary_stats,
        c"alloc_RTSSummaryStats.gc_summary_stats".as_ptr(),
    ) as *mut GenerationSummaryStats;

    memset(
        (*sum).gc_summary_stats as *mut c_void,
        0,
        sizeof_gc_summary_stats,
    );
}

unsafe fn free_RTSSummaryStats(mut sum: *mut RTSSummaryStats) {
    stgFree((*sum).gc_summary_stats as *mut c_void);
    (*sum).gc_summary_stats = null_mut::<GenerationSummaryStats>();
}

unsafe fn report_summary(mut sum: *const RTSSummaryStats) {
    let mut g: u32 = 0;
    let mut temp: [c_char; 512] = [0; 512];

    showStgWord64(
        stats.allocated_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        true,
    );

    statsPrintf(
        c"%16s bytes allocated in the heap\n".as_ptr(),
        &raw mut temp as *mut c_char,
    );

    showStgWord64(
        stats.copied_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        true,
    );
    statsPrintf(
        c"%16s bytes copied during GC\n".as_ptr(),
        &raw mut temp as *mut c_char,
    );

    if stats.major_gcs > 0 {
        showStgWord64(
            stats.max_live_bytes as StgWord64,
            &raw mut temp as *mut c_char,
            true,
        );

        statsPrintf(
            c"%16s bytes maximum residency (%u sample(s))\n".as_ptr(),
            &raw mut temp as *mut c_char,
            stats.major_gcs,
        );
    }

    showStgWord64(
        stats.max_slop_bytes as StgWord64,
        &raw mut temp as *mut c_char,
        true,
    );
    statsPrintf(
        c"%16s bytes maximum slop\n".as_ptr(),
        &raw mut temp as *mut c_char,
    );

    statsPrintf(
        c"%16llu MiB total memory in use (%llu MiB lost due to fragmentation)\n\n".as_ptr(),
        stats
            .max_mem_in_use_bytes
            .wrapping_div((1024 as i32 * 1024 as i32) as u64),
        (*sum)
            .fragmentation_bytes
            .wrapping_div((1024 as i32 * 1024 as i32) as u64),
    );

    statsPrintf(
        c"                                     Tot time (elapsed)  Avg pause  Max pause\n".as_ptr(),
    );

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        let mut gen_stats: *const GenerationSummaryStats =
            (*sum).gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;

        statsPrintf(
            c"  Gen %2d     %5d colls, %5d par   %6.3fs  %6.3fs     %3.4fs    %3.4fs\n".as_ptr(),
            g,
            (*gen_stats).collections,
            (*gen_stats).par_collections,
            (*gen_stats).cpu_ns as f64 / TIME_RESOLUTION as f64,
            (*gen_stats).elapsed_ns as f64 / TIME_RESOLUTION as f64,
            (*gen_stats).avg_pause_ns as f64 / TIME_RESOLUTION as f64,
            (*gen_stats).max_pause_ns as f64 / TIME_RESOLUTION as f64,
        );

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let nonmoving_gen: u32 = RtsFlags.GcFlags.generations.wrapping_sub(1 as u32);
        let n_major_colls =
            (*(*sum).gc_summary_stats.offset(nonmoving_gen as isize)).collections as i32;

        statsPrintf(
            c"  Gen %2d     %5d syncs,                      %6.3fs     %3.4fs    %3.4fs\n".as_ptr(),
            nonmoving_gen,
            n_major_colls,
            stats.nonmoving_gc_sync_elapsed_ns as f64 / TIME_RESOLUTION as f64,
            stats.nonmoving_gc_sync_elapsed_ns as f64
                / TIME_RESOLUTION as f64
                / n_major_colls as f64,
            stats.nonmoving_gc_sync_max_elapsed_ns as f64 / TIME_RESOLUTION as f64,
        );

        statsPrintf(
            c"  Gen %2d      concurrent,             %6.3fs  %6.3fs     %3.4fs    %3.4fs\n"
                .as_ptr(),
            nonmoving_gen,
            stats.nonmoving_gc_cpu_ns as f64 / TIME_RESOLUTION as f64,
            stats.nonmoving_gc_elapsed_ns as f64 / TIME_RESOLUTION as f64,
            stats.nonmoving_gc_elapsed_ns as f64 / TIME_RESOLUTION as f64 / n_major_colls as f64,
            stats.nonmoving_gc_max_elapsed_ns as f64 / TIME_RESOLUTION as f64,
        );
    }

    statsPrintf(c"\n".as_ptr());

    if RtsFlags.ParFlags.parGcEnabled as i32 != 0 && (*sum).work_balance > 0 {
        statsPrintf(
            c"  Parallel GC work balance: %.2f%% (serial 0%%, perfect 100%%)\n\n".as_ptr(),
            (*sum).work_balance * 100,
        );
    }

    statsPrintf(
        c"  TASKS: %d (%d bound, %d peak workers (%d total), using -N%d)\n\n".as_ptr(),
        taskCount,
        (*sum).bound_task_count,
        peakWorkerCount,
        workerCount,
        getNumCapabilities(),
    );

    statsPrintf(
        c"  SPARKS: %llu (%llu converted, %llu overflowed, %llu dud, %llu GC'd, %llu fizzled)\n\n"
            .as_ptr(),
        (*sum).sparks_count,
        (*sum).sparks.converted,
        (*sum).sparks.overflowed,
        (*sum).sparks.dud,
        (*sum).sparks.gcd,
        (*sum).sparks.fizzled,
    );

    statsPrintf(
        c"  INIT    time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        stats.init_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.init_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    statsPrintf(
        c"  MUT     time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        stats.mutator_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.mutator_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    statsPrintf(
        c"  GC      time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        stats.gc_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.gc_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    if RtsFlags.GcFlags.useNonmoving {
        statsPrintf(
            c"  CONC GC time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
            stats.nonmoving_gc_cpu_ns as f64 / TIME_RESOLUTION as f64,
            stats.nonmoving_gc_elapsed_ns as f64 / TIME_RESOLUTION as f64,
        );
    }

    statsPrintf(
        c"  RP      time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        (*sum).rp_cpu_ns as f64 / TIME_RESOLUTION as f64,
        (*sum).rp_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    statsPrintf(
        c"  PROF    time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        (*sum).hc_cpu_ns as f64 / TIME_RESOLUTION as f64,
        (*sum).hc_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    statsPrintf(
        c"  EXIT    time  %7.3fs  (%7.3fs elapsed)\n".as_ptr(),
        (*sum).exit_cpu_ns as f64 / TIME_RESOLUTION as f64,
        (*sum).exit_elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    statsPrintf(
        c"  Total   time  %7.3fs  (%7.3fs elapsed)\n\n".as_ptr(),
        stats.cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.elapsed_ns as f64 / TIME_RESOLUTION as f64,
    );

    showStgWord64(
        (*sum).alloc_rate as StgWord64,
        &raw mut temp as *mut c_char,
        true,
    );

    statsPrintf(
        c"  Alloc rate    %s bytes per MUT second\n\n".as_ptr(),
        &raw mut temp as *mut c_char,
    );

    statsPrintf(
        c"  Productivity %5.1f%% of total user, %.1f%% of total elapsed\n\n".as_ptr(),
        (*sum).productivity_cpu_percent * 100,
        (*sum).productivity_elapsed_percent * 100,
    );

    if RtsFlags.MiscFlags.internalCounters {
        let col_width: [i32; 4] = [4, -30, 14, 14];
        statsPrintf(c"Internal Counters:\n".as_ptr());

        statsPrintf(
            c"%*s%*s%*s%*s\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"SpinLock".as_ptr(),
            col_width[2],
            c"Spins".as_ptr(),
            col_width[3],
            c"Yields".as_ptr(),
        );

        statsPrintf(
            c"%*s%*s%*llu%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"gc_alloc_block_sync".as_ptr(),
            col_width[2],
            gc_alloc_block_sync.spin,
            col_width[3],
            gc_alloc_block_sync.r#yield,
        );

        statsPrintf(
            c"%*s%*s%*llu%*s\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"whitehole_gc".as_ptr(),
            col_width[2],
            whitehole_gc_spin,
            col_width[3],
            c"n/a".as_ptr(),
        );

        statsPrintf(
            c"%*s%*s%*llu%*s\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"whitehole_threadPaused".as_ptr(),
            col_width[2],
            whitehole_threadPaused_spin,
            col_width[3],
            c"n/a".as_ptr(),
        );

        statsPrintf(
            c"%*s%*s%*llu%*s\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"whitehole_executeMessage".as_ptr(),
            col_width[2],
            whitehole_executeMessage_spin,
            col_width[3],
            c"n/a".as_ptr(),
        );

        statsPrintf(
            c"%*s%*s%*llu%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"whitehole_lockClosure".as_ptr(),
            col_width[2],
            whitehole_lockClosure_spin,
            col_width[3],
            whitehole_lockClosure_yield,
        );

        statsPrintf(
            c"%*s%*s%*llu%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"waitForGcThreads".as_ptr(),
            col_width[2],
            waitForGcThreads_spin,
            col_width[3],
            waitForGcThreads_yield,
        );

        g = 0;

        while g < RtsFlags.GcFlags.generations {
            let mut prefix_length = 0;

            prefix_length = statsPrintf(c"%*sgen[%u".as_ptr(), col_width[0], c"".as_ptr(), g);

            if prefix_length < 0 {
                prefix_length = 0;
            }

            prefix_length -= col_width[0] as i32;

            let mut suffix_length = col_width[1] + prefix_length;

            suffix_length = (if suffix_length > 0 {
                col_width[1]
            } else {
                suffix_length as i32
            }) as i32;

            statsPrintf(
                c"%*s%*llu%*llu\n".as_ptr(),
                suffix_length,
                c"].sync".as_ptr(),
                col_width[2],
                (*generations.offset(g as isize)).sync.spin,
                col_width[3],
                (*generations.offset(g as isize)).sync.r#yield,
            );

            g = g.wrapping_add(1);
        }

        statsPrintf(c"\n".as_ptr());

        statsPrintf(
            c"%*s%*s%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"any_work".as_ptr(),
            col_width[2],
            stats.any_work,
        );

        statsPrintf(
            c"%*s%*s%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"scav_find_work".as_ptr(),
            col_width[2],
            stats.scav_find_work,
        );

        statsPrintf(
            c"%*s%*s%*llu\n".as_ptr(),
            col_width[0],
            c"".as_ptr(),
            col_width[1],
            c"max_n_todo_overflow".as_ptr(),
            col_width[2],
            stats.max_n_todo_overflow,
        );
    }
}

unsafe fn report_machine_readable(mut sum: *const RTSSummaryStats) {
    let mut g: u32 = 0;

    statsPrintf(
        c" [(\"%s\", \"%llu\")\n".as_ptr(),
        c"bytes allocated".as_ptr(),
        stats.allocated_bytes,
    );

    statsPrintf(c" ,(\"num_GCs\", \"%u\")\n".as_ptr(), stats.gcs);

    statsPrintf(
        c" ,(\"average_bytes_used\", \"%llu\")\n".as_ptr(),
        (*sum).average_bytes_used,
    );

    statsPrintf(
        c" ,(\"max_bytes_used\", \"%llu\")\n".as_ptr(),
        stats.max_live_bytes,
    );
    statsPrintf(
        c" ,(\"num_byte_usage_samples\", \"%u\")\n".as_ptr(),
        stats.major_gcs,
    );

    statsPrintf(
        c" ,(\"peak_megabytes_allocated\", \"%llu\")\n".as_ptr(),
        stats
            .max_mem_in_use_bytes
            .wrapping_div((1024 as i32 * 1024 as i32) as u64),
    );

    statsPrintf(
        c" ,(\"init_cpu_seconds\", \"%f\")\n".as_ptr(),
        stats.init_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"init_wall_seconds\", \"%f\")\n".as_ptr(),
        stats.init_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"mut_cpu_seconds\", \"%f\")\n".as_ptr(),
        stats.mutator_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"mut_wall_seconds\", \"%f\")\n".as_ptr(),
        stats.mutator_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"GC_cpu_seconds\", \"%f\")\n".as_ptr(),
        stats.gc_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"GC_wall_seconds\", \"%f\")\n".as_ptr(),
        stats.gc_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"exit_cpu_seconds\", \"%f\")\n".as_ptr(),
        (*sum).exit_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"exit_wall_seconds\", \"%f\")\n".as_ptr(),
        (*sum).exit_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"rp_cpu_seconds\", \"%f\")\n".as_ptr(),
        (*sum).rp_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"rp_wall_seconds\", \"%f\")\n".as_ptr(),
        (*sum).rp_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"hc_cpu_seconds\", \"%f\")\n".as_ptr(),
        (*sum).hc_cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"hc_wall_seconds\", \"%f\")\n".as_ptr(),
        (*sum).hc_elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"total_cpu_seconds\", \"%f\")\n".as_ptr(),
        stats.cpu_ns as f64 / 1000000000,
    );

    statsPrintf(
        c" ,(\"total_wall_seconds\", \"%f\")\n".as_ptr(),
        stats.elapsed_ns as f64 / 1000000000,
    );

    statsPrintf(c" ,(\"major_gcs\", \"%u\")\n".as_ptr(), stats.major_gcs);
    statsPrintf(
        c" ,(\"allocated_bytes\", \"%llu\")\n".as_ptr(),
        stats.allocated_bytes,
    );
    statsPrintf(
        c" ,(\"max_live_bytes\", \"%llu\")\n".as_ptr(),
        stats.max_live_bytes,
    );

    statsPrintf(
        c" ,(\"max_large_objects_bytes\", \"%llu\")\n".as_ptr(),
        stats.max_large_objects_bytes,
    );

    statsPrintf(
        c" ,(\"max_compact_bytes\", \"%llu\")\n".as_ptr(),
        stats.max_compact_bytes,
    );

    statsPrintf(
        c" ,(\"max_slop_bytes\", \"%llu\")\n".as_ptr(),
        stats.max_slop_bytes,
    );

    statsPrintf(
        c" ,(\"max_mem_in_use_bytes\", \"%llu\")\n".as_ptr(),
        stats.max_mem_in_use_bytes,
    );

    statsPrintf(
        c" ,(\"cumulative_live_bytes\", \"%llu\")\n".as_ptr(),
        stats.cumulative_live_bytes,
    );

    statsPrintf(
        c" ,(\"copied_bytes\", \"%llu\")\n".as_ptr(),
        stats.copied_bytes,
    );

    statsPrintf(
        c" ,(\"par_copied_bytes\", \"%llu\")\n".as_ptr(),
        stats.par_copied_bytes,
    );

    statsPrintf(
        c" ,(\"cumulative_par_max_copied_bytes\", \"%llu\")\n".as_ptr(),
        stats.cumulative_par_max_copied_bytes,
    );

    statsPrintf(
        c" ,(\"cumulative_par_balanced_copied_bytes\", \"%llu\")\n".as_ptr(),
        stats.cumulative_par_balanced_copied_bytes,
    );

    statsPrintf(
        c" ,(\"fragmentation_bytes\", \"%llu\")\n".as_ptr(),
        (*sum).fragmentation_bytes,
    );

    statsPrintf(
        c" ,(\"alloc_rate\", \"%llu\")\n".as_ptr(),
        (*sum).alloc_rate,
    );

    statsPrintf(
        c" ,(\"productivity_cpu_percent\", \"%f\")\n".as_ptr(),
        (*sum).productivity_cpu_percent,
    );

    statsPrintf(
        c" ,(\"productivity_wall_percent\", \"%f\")\n".as_ptr(),
        (*sum).productivity_elapsed_percent,
    );

    statsPrintf(
        c" ,(\"bound_task_count\", \"%u\")\n".as_ptr(),
        (*sum).bound_task_count,
    );
    statsPrintf(
        c" ,(\"sparks_count\", \"%llu\")\n".as_ptr(),
        (*sum).sparks_count,
    );

    statsPrintf(
        c" ,(\"sparks_converted\", \"%llu\")\n".as_ptr(),
        (*sum).sparks.converted,
    );

    statsPrintf(
        c" ,(\"sparks_overflowed\", \"%llu\")\n".as_ptr(),
        (*sum).sparks.overflowed,
    );

    statsPrintf(
        c" ,(\"sparks_dud \", \"%llu\")\n".as_ptr(),
        (*sum).sparks.dud,
    );
    statsPrintf(
        c" ,(\"sparks_gcd\", \"%llu\")\n".as_ptr(),
        (*sum).sparks.gcd,
    );
    statsPrintf(
        c" ,(\"sparks_fizzled\", \"%llu\")\n".as_ptr(),
        (*sum).sparks.fizzled,
    );
    statsPrintf(
        c" ,(\"work_balance\", \"%f\")\n".as_ptr(),
        (*sum).work_balance,
    );
    statsPrintf(
        c" ,(\"n_capabilities\", \"%u\")\n".as_ptr(),
        getNumCapabilities(),
    );
    statsPrintf(c" ,(\"task_count\", \"%u\")\n".as_ptr(), taskCount);
    statsPrintf(
        c" ,(\"peak_worker_count\", \"%u\")\n".as_ptr(),
        peakWorkerCount,
    );
    statsPrintf(c" ,(\"worker_count\", \"%u\")\n".as_ptr(), workerCount);

    statsPrintf(
        c" ,(\"gc_alloc_block_sync_spin\", \"%llu\")\n".as_ptr(),
        gc_alloc_block_sync.spin,
    );

    statsPrintf(
        c" ,(\"gc_alloc_block_sync_yield\", \"%llu\")\n".as_ptr(),
        gc_alloc_block_sync.r#yield,
    );

    statsPrintf(
        c" ,(\"gc_alloc_block_sync_spin\", \"%llu\")\n".as_ptr(),
        gc_alloc_block_sync.spin,
    );

    statsPrintf(
        c" ,(\"waitForGcThreads_spin\", \"%llu\")\n".as_ptr(),
        waitForGcThreads_spin,
    );

    statsPrintf(
        c" ,(\"waitForGcThreads_yield\", \"%llu\")\n".as_ptr(),
        waitForGcThreads_yield,
    );

    statsPrintf(
        c" ,(\"whitehole_gc_spin\", \"%llu\")\n".as_ptr(),
        whitehole_gc_spin,
    );

    statsPrintf(
        c" ,(\"whitehole_lockClosure_spin\", \"%llu\")\n".as_ptr(),
        whitehole_lockClosure_spin,
    );

    statsPrintf(
        c" ,(\"whitehole_lockClosure_yield\", \"%llu\")\n".as_ptr(),
        whitehole_lockClosure_yield,
    );

    statsPrintf(
        c" ,(\"whitehole_executeMessage_spin\", \"%llu\")\n".as_ptr(),
        whitehole_executeMessage_spin,
    );

    statsPrintf(
        c" ,(\"whitehole_threadPaused_spin\", \"%llu\")\n".as_ptr(),
        whitehole_threadPaused_spin,
    );

    statsPrintf(c" ,(\"any_work\", \"%llu\")\n".as_ptr(), stats.any_work);
    statsPrintf(
        c" ,(\"scav_find_work\", \"%llu\")\n".as_ptr(),
        stats.scav_find_work,
    );

    statsPrintf(
        c" ,(\"max_n_todo_overflow\", \"%llu\")\n".as_ptr(),
        stats.max_n_todo_overflow,
    );

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        let mut gc_sum: *const GenerationSummaryStats =
            (*sum).gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;

        statsPrintf(
            c" ,(\"gen_%u_collections\", \"%u\")\n".as_ptr(),
            g,
            (*gc_sum).collections,
        );

        statsPrintf(
            c" ,(\"gen_%u_par_collections\", \"%u\")\n".as_ptr(),
            g,
            (*gc_sum).par_collections,
        );

        statsPrintf(
            c" ,(\"gen_%u_cpu_seconds\", \"%f\")\n".as_ptr(),
            g,
            (*gc_sum).cpu_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"gen_%u_wall_seconds\", \"%f\")\n".as_ptr(),
            g,
            (*gc_sum).elapsed_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"gen_%u_max_pause_seconds\", \"%f\")\n".as_ptr(),
            g,
            (*gc_sum).max_pause_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"gen_%u_avg_pause_seconds\", \"%f\")\n".as_ptr(),
            g,
            (*gc_sum).avg_pause_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"gen_%u_sync_spin\", \"%llu\")\n".as_ptr(),
            g,
            (*gc_sum).sync_spin,
        );

        statsPrintf(
            c" ,(\"gen_%u_sync_yield\", \"%llu\")\n".as_ptr(),
            g,
            (*gc_sum).sync_yield,
        );

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let n_major_colls = (*(*sum)
            .gc_summary_stats
            .offset(RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) as isize))
        .collections as i32;

        statsPrintf(
            c" ,(\"nonmoving_sync_wall_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_sync_elapsed_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"nonmoving_sync_max_pause_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_sync_max_elapsed_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"nonmoving_sync_avg_pause_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_sync_elapsed_ns as f64 / 1000000000 / n_major_colls as f64,
        );

        statsPrintf(
            c" ,(\"nonmoving_concurrent_cpu_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_cpu_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"nonmoving_concurrent_wall_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_elapsed_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"nonmoving_concurrent_max_pause_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_max_elapsed_ns as f64 / 1000000000,
        );

        statsPrintf(
            c" ,(\"nonmoving_concurrent_avg_pause_seconds\", \"%f\")\n".as_ptr(),
            stats.nonmoving_gc_elapsed_ns as f64 / 1000000000 / n_major_colls as f64,
        );
    }

    statsPrintf(c" ]\n".as_ptr());
}

unsafe fn report_one_line(mut sum: *const RTSSummaryStats) {
    statsPrintf(
        c"<<ghc: %llu bytes, %u GCs, %llu/%llu avg/max bytes residency (%u samples), %lluM in use, %.3f INIT (%.3f elapsed), %.3f MUT (%.3f elapsed), %.3f GC (%.3f elapsed) :ghc>>\n"
            .as_ptr(),
        stats.allocated_bytes,
        stats.gcs,
        (*sum).average_bytes_used,
        stats.max_live_bytes,
        stats.major_gcs,
        stats.max_mem_in_use_bytes.wrapping_div((1024 as i32 * 1024 as i32) as u64),
        stats.init_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.init_elapsed_ns as f64 / TIME_RESOLUTION as f64,
        stats.mutator_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.mutator_elapsed_ns as f64 / TIME_RESOLUTION as f64,
        stats.gc_cpu_ns as f64 / TIME_RESOLUTION as f64,
        stats.gc_elapsed_ns as f64 / TIME_RESOLUTION as f64,
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
        bound_task_count: 0,
        sparks_count: 0,
        sparks: SparkCounters {
            created: 0,
            dud: 0,
            overflowed: 0,
            converted: 0,
            gcd: 0,
            fizzled: 0,
        },
        work_balance: 0.,
        fragmentation_bytes: 0,
        average_bytes_used: 0,
        alloc_rate: 0,
        productivity_cpu_percent: 0.,
        productivity_elapsed_percent: 0.,
        gc_summary_stats: null_mut::<GenerationSummaryStats>(),
    };

    init_RTSSummaryStats(&raw mut sum);

    let mut __r = pthread_mutex_lock(&raw mut all_tasks_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1249,
            __r,
        );
    }

    if RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32 {
        let mut now_cpu_ns: Time = 0;
        let mut now_elapsed_ns: Time = 0;
        getProcessTimes(&raw mut now_cpu_ns, &raw mut now_elapsed_ns);

        let mut __r_0 = pthread_mutex_lock(&raw mut stats_mutex);

        if __r_0 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Stats.c".as_ptr(),
                1259,
                __r_0,
            );
        }

        stats.cpu_ns = now_cpu_ns - start_init_cpu;
        stats.elapsed_ns = now_elapsed_ns - start_init_elapsed;

        if stats.cpu_ns <= 0 {
            stats.cpu_ns = 1;
        }

        if stats.elapsed_ns <= 0 {
            stats.elapsed_ns = 1;
        }

        sum.rp_cpu_ns = RP_tot_time;
        sum.rp_elapsed_ns = RPe_tot_time;
        sum.hc_cpu_ns = HC_tot_time;
        sum.hc_elapsed_ns = HCe_tot_time;

        let mut exit_gc_cpu: Time = stats.gc_cpu_ns - start_exit_gc_cpu;
        let mut exit_gc_elapsed: Time = stats.gc_elapsed_ns - start_exit_gc_elapsed;

        if !((exit_gc_elapsed > 0) as i32 as i64 != 0) {
            _warnFail(c"rts/Stats.c".as_ptr(), 1281);
        }

        sum.exit_cpu_ns = end_exit_cpu - start_exit_cpu - exit_gc_cpu;
        sum.exit_elapsed_ns = end_exit_elapsed - start_exit_elapsed - exit_gc_elapsed;

        if !((sum.exit_elapsed_ns >= 0) as i32 as i64 != 0) {
            _warnFail(c"rts/Stats.c".as_ptr(), 1290);
        }

        stats.mutator_cpu_ns = start_exit_cpu
            - end_init_cpu
            - (stats.gc_cpu_ns - exit_gc_cpu)
            - stats.nonmoving_gc_cpu_ns;
        stats.mutator_elapsed_ns =
            start_exit_elapsed - end_init_elapsed - (stats.gc_elapsed_ns - exit_gc_elapsed);

        if !((stats.mutator_elapsed_ns >= 0) as i32 as i64 != 0) {
            _warnFail(c"rts/Stats.c".as_ptr(), 1300);
        }

        if stats.mutator_cpu_ns < 0 {
            stats.mutator_cpu_ns = 0;
        }

        if !((stats.init_elapsed_ns
            + stats.mutator_elapsed_ns
            + stats.gc_elapsed_ns
            + sum.exit_elapsed_ns
            == end_exit_elapsed - start_init_elapsed) as i32 as i64
            != 0)
        {
            _warnFail(c"rts/Stats.c".as_ptr(), 1312);
        }

        let mut prof_cpu: Time = sum.rp_cpu_ns + sum.hc_cpu_ns;
        let mut prof_elapsed: Time = sum.rp_elapsed_ns + sum.hc_elapsed_ns;
        stats.gc_cpu_ns -= prof_cpu;
        stats.gc_elapsed_ns -= prof_elapsed;

        if !((stats.init_elapsed_ns
            + stats.mutator_elapsed_ns
            + stats.gc_elapsed_ns
            + sum.exit_elapsed_ns
            + (sum.rp_elapsed_ns + sum.hc_elapsed_ns)
            == end_exit_elapsed - start_init_elapsed) as i32 as i64
            != 0)
        {
            _warnFail(c"rts/Stats.c".as_ptr(), 1330);
        }

        let mut tot_alloc_bytes: u64 = calcTotalAllocated().wrapping_mul(size_of::<W_>() as u64);
        stats.gc.allocated_bytes = tot_alloc_bytes.wrapping_sub(stats.allocated_bytes);
        stats.allocated_bytes = tot_alloc_bytes;

        if RtsFlags.GcFlags.giveStats >= VERBOSE_GC_STATS as u32 {
            statsPrintf(
                c"%9llu %9.9s %9.9s".as_ptr(),
                stats.gc.allocated_bytes,
                c"".as_ptr(),
                c"".as_ptr(),
            );

            statsPrintf(c" %6.3f %6.3f\n\n".as_ptr(), 0.0f64, 0.0f64);
        }

        sum.bound_task_count = taskCount.wrapping_sub(workerCount);

        let mut i: u32 = 0;

        while i < getNumCapabilities() as u32 {
            sum.sparks.created = sum
                .sparks
                .created
                .wrapping_add((*getCapability(i)).spark_stats.created);
            sum.sparks.dud = sum
                .sparks
                .dud
                .wrapping_add((*getCapability(i)).spark_stats.dud);
            sum.sparks.overflowed = sum
                .sparks
                .overflowed
                .wrapping_add((*getCapability(i)).spark_stats.overflowed);
            sum.sparks.converted = sum
                .sparks
                .converted
                .wrapping_add((*getCapability(i)).spark_stats.converted);
            sum.sparks.gcd = sum
                .sparks
                .gcd
                .wrapping_add((*getCapability(i)).spark_stats.gcd);
            sum.sparks.fizzled = sum
                .sparks
                .fizzled
                .wrapping_add((*getCapability(i)).spark_stats.fizzled);
            i = i.wrapping_add(1);
        }

        sum.sparks_count = sum
            .sparks
            .created
            .wrapping_add(sum.sparks.dud)
            .wrapping_add(sum.sparks.overflowed) as u64;

        if RtsFlags.ParFlags.parGcEnabled as i32 != 0 && stats.par_copied_bytes > 0 {
            sum.work_balance =
                stats.cumulative_par_balanced_copied_bytes as f64 / stats.par_copied_bytes as f64;
        } else {
            sum.work_balance = 0;
        }

        sum.fragmentation_bytes = peak_mblocks_allocated
            .wrapping_mul(BLOCKS_PER_MBLOCK)
            .wrapping_mul(BLOCK_SIZE_W as W_)
            .wrapping_sub(hw_alloc_blocks.wrapping_mul(BLOCK_SIZE_W as W_))
            .wrapping_mul(size_of::<W_>() as u64);

        sum.average_bytes_used = (if stats.major_gcs == 0 {
            0
        } else {
            stats
                .cumulative_live_bytes
                .wrapping_div(stats.major_gcs as u64)
        });

        sum.alloc_rate = (if stats.mutator_cpu_ns == 0 {
            0
        } else {
            (stats.allocated_bytes as f64 / (stats.mutator_cpu_ns as f64 / TIME_RESOLUTION as f64))
                as u64
        });

        sum.productivity_cpu_percent =
            (stats.cpu_ns - stats.gc_cpu_ns - stats.init_cpu_ns - sum.exit_cpu_ns) as f64
                / TIME_RESOLUTION as f64
                / (stats.cpu_ns as f64 / TIME_RESOLUTION as f64);
        if !((sum.productivity_cpu_percent >= 0) as i32 as i64 != 0) {
            _warnFail(c"rts/Stats.c".as_ptr(), 1407);
        }

        sum.productivity_elapsed_percent =
            (stats.elapsed_ns - stats.gc_elapsed_ns - stats.init_elapsed_ns - sum.exit_elapsed_ns)
                as f64
                / TIME_RESOLUTION as f64
                / (stats.elapsed_ns as f64 / TIME_RESOLUTION as f64);
        if !((sum.productivity_elapsed_percent >= 0) as i32 as i64 != 0) {
            _warnFail(c"rts/Stats.c".as_ptr(), 1416);
        }

        let mut g: u32 = 0;

        while g < RtsFlags.GcFlags.generations {
            let mut r#gen: *const generation = generations.offset(g as isize) as *mut generation;

            let mut gen_stats: *mut GenerationSummaryStats =
                sum.gc_summary_stats.offset(g as isize) as *mut GenerationSummaryStats;
            (*gen_stats).collections = (*r#gen).collections;
            (*gen_stats).par_collections = (*r#gen).par_collections;
            (*gen_stats).cpu_ns = *GC_coll_cpu.offset(g as isize);
            (*gen_stats).elapsed_ns = *GC_coll_elapsed.offset(g as isize);
            (*gen_stats).max_pause_ns = *GC_coll_max_pause.offset(g as isize);

            (*gen_stats).avg_pause_ns = if (*r#gen).collections == 0 {
                0
            } else {
                *GC_coll_elapsed.offset(g as isize) / (*r#gen).collections as Time
            };

            (*gen_stats).sync_spin = (*r#gen).sync.spin as u64;
            (*gen_stats).sync_yield = (*r#gen).sync.r#yield as u64;
            g = g.wrapping_add(1);
        }

        if RtsFlags.GcFlags.giveStats >= SUMMARY_GC_STATS as u32 {
            report_summary(&raw mut sum);
        }

        if RtsFlags.GcFlags.giveStats == ONELINE_GC_STATS as u32 {
            if RtsFlags.MiscFlags.machineReadable {
                report_machine_readable(&raw mut sum);
            } else {
                report_one_line(&raw mut sum);
            }
        }

        if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Stats.c".as_ptr(),
                1448,
            );
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

    if pthread_mutex_unlock(&raw mut all_tasks_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1469,
        );
    }
}

unsafe fn stat_exit() {
    closeMutex(&raw mut stats_mutex);
}

unsafe fn statDescribeGens() {
    let mut g: u32 = 0;
    let mut r#mut: u32 = 0;
    let mut lge: u32 = 0;
    let mut compacts: u32 = 0;
    let mut i: u32 = 0;
    let mut gen_slop: W_ = 0;
    let mut tot_live: W_ = 0;
    let mut tot_slop: W_ = 0;
    let mut gen_live: W_ = 0;
    let mut gen_blocks: W_ = 0;
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();

    debugBelch(
        c"----------------------------------------------------------------------\n  Gen     Max  Mut-list  Blocks    Large  Compacts      Live      Slop\n       Blocks     Bytes          Objects                              \n----------------------------------------------------------------------\n"
            .as_ptr(),
    );

    tot_live = 0;
    tot_slop = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        r#gen = generations.offset(g as isize) as *mut generation;
        bd = (*r#gen).large_objects;
        lge = 0;

        while !bd.is_null() {
            lge = lge.wrapping_add(1);
            bd = (*bd).link as *mut bdescr;
        }

        bd = (*r#gen).compact_objects;
        compacts = 0;

        while !bd.is_null() {
            compacts = compacts.wrapping_add(1);
            bd = (*bd).link as *mut bdescr;
        }

        gen_live = genLiveWords(r#gen) as W_;
        gen_blocks = genLiveBlocks(r#gen) as W_;
        r#mut = 0;
        i = 0;

        while i < getNumCapabilities() as u32 {
            r#mut = (r#mut as StgWord).wrapping_add(countOccupied(
                *(*getCapability(i)).mut_lists.offset(g as isize),
            )) as u32 as u32;

            bd = (*getCapability(i)).pinned_object_block;

            if !bd.is_null() {
                gen_live = gen_live
                    .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as W_);

                gen_blocks = gen_blocks.wrapping_add((*bd).blocks as W_);
            }

            gen_live = (gen_live as StgWord).wrapping_add(gcThreadLiveWords(i, g)) as W_ as W_;
            gen_blocks = (gen_blocks as StgWord).wrapping_add(gcThreadLiveBlocks(i, g)) as W_ as W_;
            i = i.wrapping_add(1);
        }

        debugBelch(c"%5d %7llu %9d".as_ptr(), g, (*r#gen).max_blocks, r#mut);
        gen_slop = gen_blocks
            .wrapping_mul(BLOCK_SIZE_W as W_)
            .wrapping_sub(gen_live);

        debugBelch(
            c"%8llu %8d  %8d %9llu %9llu\n".as_ptr(),
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
        c"----------------------------------------------------------------------\n".as_ptr(),
    );

    debugBelch(
        c"%51s%9llu %9llu\n".as_ptr(),
        c"".as_ptr(),
        tot_live.wrapping_mul(size_of::<W_>() as W_),
        tot_slop.wrapping_mul(size_of::<W_>() as W_),
    );

    debugBelch(
        c"----------------------------------------------------------------------\n".as_ptr(),
    );

    debugBelch(c"\n".as_ptr());
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getAllocations() -> c_ulong {
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1690,
            __r,
        );
    }

    let mut n: StgWord64 = stats.allocated_bytes as StgWord64;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1692,
        );
    }

    return n as u64;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStatsEnabled() -> c_int {
    return (RtsFlags.GcFlags.giveStats != NO_GC_STATS as u32) as i32;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getRTSStats(mut s: *mut RTSStats) {
    let mut current_elapsed: Time = 0;
    let mut current_cpu: Time = 0;
    let mut __r = pthread_mutex_lock(&raw mut stats_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1706,
            __r,
        );
    }

    *s = stats;

    if pthread_mutex_unlock(&raw mut stats_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Stats.c".as_ptr(),
            1708,
        );
    }

    getProcessTimes(&raw mut current_cpu, &raw mut current_elapsed);
    (*s).cpu_ns = current_cpu - end_init_cpu;
    (*s).elapsed_ns = current_elapsed - end_init_elapsed;
    (*s).mutator_cpu_ns = current_cpu - end_init_cpu - stats.gc_cpu_ns - stats.nonmoving_gc_cpu_ns;
    (*s).mutator_elapsed_ns = current_elapsed - end_init_elapsed - stats.gc_elapsed_ns;
}

unsafe fn statsPrintf(mut s: *mut c_char, mut args: ...) -> i32 {
    let mut ret = 0;
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
