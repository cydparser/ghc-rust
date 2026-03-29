use crate::arena::arenaBlocks;
use crate::capability::{getCapability, markCapability, n_numa_nodes};
use crate::check_unload::{checkUnload, prepareUnloadCheck};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::block_signals::{blockUserSignals, unblockUserSignals};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::block::{
    BF_EVACUATED, BF_FRAGMENTED, BF_MARKED, BF_NONMOVING, BF_SWEPT, BLOCK_MASK, BLOCK_SIZE,
    BLOCK_SIZE_W, MBLOCK_MASK, MBLOCK_SIZE, allocBlock, allocBlockOnNode, allocGroup, bdescr,
    bdescr_, dbl_link_onto, freeChain, freeGroup,
};
use crate::ffi::rts::storage::closure_macros::{SET_INFO, get_itbl};
use crate::ffi::rts::storage::closures::StgWeak;
use crate::ffi::rts::storage::closures::{
    StgCompactNFData, StgCompactNFDataBlock, StgIndStatic, StgWeak,
};
use crate::ffi::rts::storage::gc::{
    g0, generation, generation_, generations, initBdescr, memcount, nursery_, oldest_gen,
};
use crate::ffi::rts::storage::m_block::mblocks_allocated;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::time::Time;
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::{_RTSStats, Capability, GCDetails_, getRTSStats};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{stg_END_TSO_QUEUE_closure, stg_GCD_CAF_info};
use crate::ffi::stg::smp::{atomic_dec, atomic_inc};
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord16};
use crate::ffi::stg::types::{StgWord, StgWord16};
use crate::ffi::stg::{BITS_PER_BYTE, W_};
use crate::prelude::*;
use crate::prof_heap::heapCensus;
use crate::proftimer::performTickySample;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::schedule::{heap_overflow, resurrectThreads};
use crate::sm::block_alloc::{
    commitMBlockFreeing, countAllocdBlocks, countBlocks, deferMBlockFreeing, returnMemoryToOS,
};
use crate::sm::cnf::compactFree;
use crate::sm::compact::compact;
use crate::sm::evac::evacuate;
use crate::sm::gc::{GcConfig, MutListScavStats, markCAFs};
use crate::sm::gc_thread::{gc_thread, gc_thread_, gen_workspace};
use crate::sm::gc_utils::{alloc_todo_block, allocBlockOnNode_sync};
use crate::sm::mark_weak::{
    collectFreshWeakPtrs, initWeakForGC, markWeakPtrList, traverseWeakPtrList,
};
use crate::sm::non_moving::{END_NONMOVING_TODO_LIST, NonmovingSegment, nonmovingCollect};
use crate::sm::non_moving_mark::nonmovingAddUpdRemSetBlocks;
use crate::sm::sanity::{checkSanity, memInventory};
use crate::sm::scav::{scavenge_capability_mut_lists, scavenge_loop};
use crate::sm::storage::{
    END_OF_CAF_LIST, STATIC_BITS, STATIC_FLAG_A, STATIC_FLAG_B, calcNeeded, countNurseryBlocks,
    countOccupied, debug_caf_list, exec_block, gcThreadLiveBlocks, gcThreadLiveWords,
    genLiveBlocks, genLiveCopiedWords, genLiveUncopiedWords, genLiveWords, n_nurseries, nurseries,
    resetNurseries, resizeNurseries, resizeNurseriesFixed,
};
use crate::sm::sweep::sweep;
use crate::stable_name::{
    gcStableNameTable, rememberOldStableNameAddresses, updateStableNameTable,
};
use crate::stable_ptr::{markStablePtrTable, stablePtrLock, stablePtrUnlock};
use crate::stats::{stat_endGC, stat_endGCWorker, stat_startGC, statDescribeGens};
use crate::ticky::emitTickyCounterSamples;
use crate::trace::{
    DEBUG_RTS, trace_, traceEventGcDone, traceEventGcIdle, traceEventGcWork, traceEventMemReturn,
};
use crate::weak::{runSomeFinalizers, scheduleFinalizers};
use crate::ws_deque::{freeWSDeque, newWSDeque};

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[repr(C)]
pub struct generation_ {
    pub no: u32,
    pub blocks: *mut bdescr,
    pub n_blocks: memcount,
    pub n_words: memcount,
    pub large_objects: *mut bdescr,
    pub n_large_blocks: memcount,
    pub n_large_words: memcount,
    pub n_new_large_words: memcount,
    pub compact_objects: *mut bdescr,
    pub n_compact_blocks: memcount,
    pub compact_blocks_in_import: *mut bdescr,
    pub n_compact_blocks_in_import: memcount,
    pub max_blocks: memcount,
    pub threads: *mut StgTSO,
    pub weak_ptr_list: *mut StgWeak,
    pub to: *mut generation_,
    pub collections: u32,
    pub par_collections: u32,
    pub failed_promotions: u32,
    pub mark: i32,
    pub compact: i32,
    pub old_blocks: *mut bdescr,
    pub n_old_blocks: memcount,
    pub live_estimate: memcount,
    pub scavenged_large_objects: *mut bdescr,
    pub n_scavenged_large_blocks: memcount,
    pub live_compact_objects: *mut bdescr,
    pub n_live_compact_blocks: memcount,
    pub bitmap: *mut bdescr,
    pub old_threads: *mut StgTSO,
    pub old_weak_ptr_list: *mut StgWeak,
}

pub(crate) type memcount = StgWord;

#[ffi(compiler)]
#[repr(C)]
pub struct nursery_ {
    pub blocks: *mut bdescr,
    pub n_blocks: memcount,
}

#[ffi(compiler)]
pub type nursery = nursery_;

#[ffi(compiler, ghc_lib)]
pub type generation = generation_;

#[inline]
pub(crate) unsafe fn initBdescr(
    mut bd: *mut bdescr,
    mut r#gen: *mut generation,
    mut dest: *mut generation,
) {
    (*bd).r#gen = r#gen as *mut generation_;
    (*bd).gen_no = (*r#gen).no as StgWord16;
    (*bd).dest_no = (*dest).no as StgWord16;

    if ((*r#gen).no < RtsFlags.GcFlags.generations) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/include/rts/storage/GC.h".as_ptr(), 334);
    }

    if ((*dest).no < RtsFlags.GcFlags.generations) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/include/rts/storage/GC.h".as_ptr(), 335);
    };
}

extern "C" {
    pub(crate) static mut generations: *mut generation;

    pub(crate) static mut g0: *mut generation;

    pub(crate) static mut oldest_gen: *mut generation;

    pub(crate) static mut large_alloc_lim: W_;
}

static mut N: u32 = 0;

static mut major_gc: bool = false;

static mut deadlock_detect_gc: bool = false;

static mut unload_mark_needed: bool = false;

static mut g0_pcnt_kept: W_ = 30;

static mut consec_idle_gcs: i32 = 0;

static mut mutlist_scav_stats: MutListScavStats = MutListScavStats {
    n_MUTVAR: 0,
    n_MUTARR: 0,
    n_MVAR: 0,
    n_TVAR: 0,
    n_TREC_CHUNK: 0,
    n_TVAR_WATCH_QUEUE: 0,
    n_TREC_HEADER: 0,
    n_OTHERS: 0,
};

static mut gc_threads: *mut *mut gc_thread = null_mut::<*mut gc_thread>();

static mut gc_running_threads: StgWord = 0;

static mut the_gc_thread: [StgWord8; 8448] = [0; 8448];

static mut n_gc_threads: u32 = 0;

static mut n_gc_idle_threads: u32 = 0;

static mut work_stealing: bool = false;

unsafe fn is_par_gc() -> bool {
    return false;
}

static mut copied: i64 = 0;

static mut static_flag: u32 = STATIC_FLAG_B as u32;

static mut prev_static_flag: u32 = STATIC_FLAG_A as u32;

static mut mark_stack_top_bd: *mut bdescr = null_mut::<bdescr>();

static mut mark_stack_bd: *mut bdescr = null_mut::<bdescr>();

static mut mark_sp: StgPtr = null_mut::<StgWord>();

unsafe fn zeroMutListScavStats(mut src: *mut MutListScavStats) {
    memset(
        src as *mut c_void,
        0,
        size_of::<MutListScavStats>() as usize,
    );
}

unsafe fn addMutListScavStats(mut src: *const MutListScavStats, mut dest: *mut MutListScavStats) {
    (*dest).n_MUTVAR = (*dest).n_MUTVAR.wrapping_add((*src).n_MUTVAR);
    (*dest).n_MUTARR = (*dest).n_MUTARR.wrapping_add((*src).n_MUTARR);
    (*dest).n_MVAR = (*dest).n_MVAR.wrapping_add((*src).n_MVAR);
    (*dest).n_TVAR = (*dest).n_TVAR.wrapping_add((*src).n_TVAR);
    (*dest).n_TREC_CHUNK = (*dest).n_TREC_CHUNK.wrapping_add((*src).n_TREC_CHUNK);
    (*dest).n_TVAR_WATCH_QUEUE = (*dest)
        .n_TVAR_WATCH_QUEUE
        .wrapping_add((*src).n_TVAR_WATCH_QUEUE);
    (*dest).n_TREC_HEADER = (*dest).n_TREC_HEADER.wrapping_add((*src).n_TREC_HEADER);
    (*dest).n_OTHERS = (*dest).n_OTHERS.wrapping_add((*src).n_OTHERS);
}

unsafe fn GarbageCollect(mut config: GcConfig, mut cap: *mut Capability, mut idle_cap: *mut bool) {
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();
    let mut live_blocks: StgWord = 0;
    let mut live_words: StgWord = 0;
    let mut par_max_copied: StgWord = 0;
    let mut par_balanced_copied: StgWord = 0;
    let mut any_work: StgWord = 0;
    let mut scav_find_work: StgWord = 0;
    let mut max_n_todo_overflow: StgWord = 0;
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    let mut mut_time: Time = 0;

    if config.do_heap_census {
        let mut stats = _RTSStats {
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

        getRTSStats(&raw mut stats);
        mut_time = stats.mutator_cpu_ns;
    }

    if !config.parallel as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 308);
    }

    if RtsFlags.MiscFlags.install_signal_handlers {
        blockUserSignals();
    }

    if (size_of::<gen_workspace>() as usize
        == (16 as usize).wrapping_mul(size_of::<StgWord>() as usize)) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 324);
    }

    stat_startGC(cap, &raw mut the_gc_thread as *mut gc_thread_);
    stablePtrLock();
    zeroMutListScavStats(&raw mut mutlist_scav_stats);
    N = config.collect_gen;
    major_gc = N == RtsFlags.GcFlags.generations.wrapping_sub(1 as u32);
    deadlock_detect_gc = config.deadlock_detect;

    if major_gc as i32 != 0 && !RtsFlags.GcFlags.useNonmoving {
        prev_static_flag = static_flag;

        static_flag = (if static_flag == STATIC_FLAG_A as u32 {
            STATIC_FLAG_B
        } else {
            STATIC_FLAG_A
        }) as u32;
    }

    if major_gc as i32 != 0 && !RtsFlags.GcFlags.useNonmoving {
        unload_mark_needed = prepareUnloadCheck();
    } else {
        unload_mark_needed = false;
    }

    n_gc_threads = 1;
    work_stealing = false;
    n_gc_idle_threads = 0;
    gc_running_threads = 0;

    if (n_gc_threads > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 432);
    }

    if (n_gc_threads <= getNumCapabilities() as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 433);
    }

    if (n_gc_idle_threads < getNumCapabilities() as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 434);
    }

    if (!work_stealing || n_gc_threads.wrapping_sub(1 as u32) > n_gc_idle_threads) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 437);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"GC (gen %d, using %d thread(s), %s work stealing)".as_ptr(),
            N,
            getNumCapabilities() as i32 - n_gc_idle_threads as i32,
            if work_stealing as i32 != 0 {
                c"with".as_ptr()
            } else {
                c"without".as_ptr()
            },
        );
    }

    memInventory(RtsFlags.DebugFlags.gc);
    deferMBlockFreeing();
    collectFreshWeakPtrs();

    if RtsFlags.DebugFlags.sanity {
        checkSanity(0 != 0, major_gc);
    }

    collect_pinned_object_blocks();
    g = 0;

    while g <= N {
        prepare_collected_gen(generations.offset(g as isize) as *mut generation);
        g = g.wrapping_add(1);
    }

    g = N.wrapping_add(1 as u32);

    while g < RtsFlags.GcFlags.generations {
        prepare_uncollected_gen(generations.offset(g as isize) as *mut generation);
        g = g.wrapping_add(1);
    }

    init_gc_thread(&raw mut the_gc_thread as *mut gc_thread);

    if major_gc as i32 != 0 && (*oldest_gen).mark != 0 {
        mark_stack_bd = allocBlock();
        mark_stack_top_bd = mark_stack_bd;
        (*mark_stack_bd).link = null_mut::<bdescr_>();
        (*mark_stack_bd).u.back = null_mut::<bdescr_>();
        mark_sp = (*mark_stack_bd).start;
    } else {
        mark_stack_bd = null_mut::<bdescr>();
        mark_stack_top_bd = null_mut::<bdescr>();
        mark_sp = null_mut::<StgWord>();
    }

    inc_running();

    wakeup_gc_threads(
        (*(&raw mut the_gc_thread as *mut gc_thread)).thread_index,
        idle_cap,
    );

    traceEventGcWork((*(&raw mut the_gc_thread as *mut gc_thread)).cap);

    if !is_par_gc() {
        n = 0;

        while n < getNumCapabilities() as u32 {
            scavenge_capability_mut_lists(getCapability(n));
            n = n.wrapping_add(1);
        }
    } else {
        scavenge_capability_mut_lists((*(&raw mut the_gc_thread as *mut gc_thread)).cap);
        n = 0;

        while n < getNumCapabilities() as u32 {
            if *idle_cap.offset(n as isize) {
                markCapability(
                    Some(
                        mark_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> (),
                    ),
                    &raw mut the_gc_thread as *mut gc_thread as *mut c_void,
                    getCapability(n),
                    true,
                );

                scavenge_capability_mut_lists(getCapability(n));
            }

            n = n.wrapping_add(1);
        }
    }

    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = 0;

    markCAFs(
        Some(mark_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        &raw mut the_gc_thread as *mut gc_thread as *mut c_void,
    );

    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = 0;

    if !is_par_gc() {
        n = 0;

        while n < getNumCapabilities() as u32 {
            markCapability(
                Some(mark_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
                &raw mut the_gc_thread as *mut gc_thread as *mut c_void,
                getCapability(n),
                true,
            );

            n = n.wrapping_add(1);
        }
    } else {
        markCapability(
            Some(mark_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
            &raw mut the_gc_thread as *mut gc_thread as *mut c_void,
            cap,
            true,
        );
    }

    markWeakPtrList();
    initWeakForGC();

    markStablePtrTable(
        Some(mark_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        &raw mut the_gc_thread as *mut gc_thread as *mut c_void,
    );

    rememberOldStableNameAddresses();
    scavenge_until_all_done();

    shutdown_gc_threads(
        (*(&raw mut the_gc_thread as *mut gc_thread)).thread_index,
        idle_cap,
    );

    let mut dead_weak_ptr_list = null_mut::<StgWeak>();
    let mut resurrected_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    work_stealing = false;

    while traverseWeakPtrList(&raw mut dead_weak_ptr_list, &raw mut resurrected_threads) {
        inc_running();
        scavenge_until_all_done();
    }

    gcStableNameTable();

    if major_gc as i32 != 0 && (*oldest_gen).mark != 0 {
        if (*oldest_gen).compact != 0 {
            compact(
                (*(&raw mut the_gc_thread as *mut gc_thread)).scavenged_static_objects,
                &raw mut dead_weak_ptr_list,
                &raw mut resurrected_threads,
            );
        } else {
            sweep(oldest_gen);
        }
    }

    copied = 0;
    par_max_copied = 0;
    par_balanced_copied = 0;
    any_work = 0;
    scav_find_work = 0;
    max_n_todo_overflow = 0;

    let mut i: u32 = 0;
    let mut par_balanced_copied_acc: u64 = 0;
    let mut thread = null::<gc_thread>();

    if is_par_gc() {
        let mut other_active_threads = n_gc_threads
            .wrapping_sub(n_gc_idle_threads)
            .wrapping_sub(1 as u32) as i32;

        if (other_active_threads > 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 629);
        }

        i = 0;

        while i < n_gc_threads {
            if !*idle_cap.offset(i as isize) {
                copied = (copied as W_).wrapping_add((**gc_threads.offset(i as isize)).copied)
                    as i64 as i64;
            }

            i = i.wrapping_add(1);
        }

        i = 0;

        while i < n_gc_threads {
            if !*idle_cap.offset(i as isize) {
                thread = *gc_threads.offset(i as isize);

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(c"thread %d:".as_ptr(), i);
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(
                        c"   copied           %ld".as_ptr(),
                        (*thread).copied.wrapping_mul(size_of::<W_>() as W_),
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(
                        c"   scanned          %ld".as_ptr(),
                        (*thread).scanned.wrapping_mul(size_of::<W_>() as W_),
                    );
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(c"   any_work         %ld".as_ptr(), (*thread).any_work);
                }

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(c"   scav_find_work %ld".as_ptr(), (*thread).scav_find_work);
                }

                any_work = any_work.wrapping_add((*thread).any_work as StgWord);
                scav_find_work = scav_find_work.wrapping_add((*thread).scav_find_work as StgWord);

                max_n_todo_overflow = ({
                    let _a: W_ = (*thread).max_n_todo_overflow;
                    let _b: W_ = max_n_todo_overflow as W_;

                    if _a as W_ <= _b as W_ {
                        _b as W_
                    } else {
                        _a as W_
                    }
                }) as StgWord;

                par_max_copied = ({
                    let _a: W_ = (*thread).copied;
                    let _b: W_ = par_max_copied as W_;

                    if _a as W_ <= _b as W_ {
                        _b as W_
                    } else {
                        _a as W_
                    }
                }) as StgWord;

                par_balanced_copied_acc = par_balanced_copied_acc.wrapping_add(
                    ({
                        let mut _a: W_ = ((other_active_threads + 1 as i32) as W_)
                            .wrapping_mul((*thread).copied);

                        let mut _b: W_ = copied as W_;

                        if _a <= _b { _a } else { _b as W_ }
                    }) as u64,
                );
            }

            i = i.wrapping_add(1);
        }

        par_balanced_copied = par_balanced_copied_acc
            .wrapping_sub(copied as u64)
            .wrapping_add((other_active_threads / 2 as i32) as u64)
            .wrapping_div(other_active_threads as u64) as StgWord;
    } else {
        copied = (copied as W_).wrapping_add((*(&raw mut the_gc_thread as *mut gc_thread)).copied)
            as i64 as i64;
        any_work = any_work
            .wrapping_add((*(&raw mut the_gc_thread as *mut gc_thread)).any_work as StgWord);

        scav_find_work = scav_find_work
            .wrapping_add((*(&raw mut the_gc_thread as *mut gc_thread)).scav_find_work as StgWord);

        max_n_todo_overflow = max_n_todo_overflow.wrapping_add(
            (*(&raw mut the_gc_thread as *mut gc_thread)).max_n_todo_overflow as StgWord,
        );
    }

    live_words = 0;
    live_blocks = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        if g == N {
            let ref mut fresh5 = (*generations.offset(g as isize)).collections;
            *fresh5 = (*fresh5).wrapping_add(1);

            if is_par_gc() {
                let ref mut fresh6 = (*generations.offset(g as isize)).par_collections;
                *fresh6 = (*fresh6).wrapping_add(1);
            }
        }

        if g > 0 {
            let mut mut_list_size: W_ = 0;
            n = 0;

            while n < getNumCapabilities() as u32 {
                mut_list_size = (mut_list_size as StgWord).wrapping_add(countOccupied(
                    *(*getCapability(n)).mut_lists.offset(g as isize),
                )) as W_ as W_;

                n = n.wrapping_add(1);
            }

            copied = (copied as W_).wrapping_add(mut_list_size) as i64 as i64;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                trace_(
                    c"mut_list_size: %lu (%d vars, %d arrays, %d MVARs, %d TVARs, %d TVAR_WATCH_QUEUEs, %d TREC_CHUNKs, %d TREC_HEADERs, %d others)"
                        .as_ptr(),
                    mut_list_size.wrapping_mul(size_of::<W_>() as W_) as u64,
                    mutlist_scav_stats.n_MUTVAR,
                    mutlist_scav_stats.n_MUTARR,
                    mutlist_scav_stats.n_MVAR,
                    mutlist_scav_stats.n_TVAR,
                    mutlist_scav_stats.n_TVAR_WATCH_QUEUE,
                    mutlist_scav_stats.n_TREC_CHUNK,
                    mutlist_scav_stats.n_TREC_HEADER,
                    mutlist_scav_stats.n_OTHERS,
                );
            }
        }

        let mut next = null_mut::<bdescr>();
        let mut prev = null_mut::<bdescr>();
        r#gen = generations.offset(g as isize) as *mut generation;

        if g <= N && !(RtsFlags.GcFlags.useNonmoving as i32 != 0 && r#gen == oldest_gen) {
            if (*r#gen).mark != 0 {
                if !(*r#gen).old_blocks.is_null() {
                    prev = null_mut::<bdescr>();
                    bd = (*r#gen).old_blocks;

                    while !bd.is_null() {
                        next = (*bd).link as *mut bdescr;

                        if (*bd).flags as i32 & BF_MARKED == 0 {
                            if prev.is_null() {
                                (*r#gen).old_blocks = next;
                            } else {
                                (*prev).link = next as *mut bdescr_;
                            }

                            freeGroup(bd);
                            (*r#gen).n_old_blocks = (*r#gen).n_old_blocks.wrapping_sub(1);
                        } else {
                            (*r#gen).n_words = (*r#gen)
                                .n_words
                                .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start)
                                    as i64
                                    as memcount);

                            (*bd).flags = ((*bd).flags as i32 & !BF_MARKED) as StgWord16;
                            (*bd).flags = ((*bd).flags as i32 | BF_EVACUATED) as StgWord16;
                            prev = bd;
                        }

                        bd = next;
                    }

                    if !prev.is_null() {
                        (*prev).link = (*r#gen).blocks as *mut bdescr_;
                        (*r#gen).blocks = (*r#gen).old_blocks;
                    }
                }

                (*r#gen).n_blocks = (*r#gen).n_blocks.wrapping_add((*r#gen).n_old_blocks);

                if (countBlocks((*r#gen).blocks) == (*r#gen).n_blocks) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/GC.c".as_ptr(), 765);
                }

                if (countOccupied((*r#gen).blocks) == (*r#gen).n_words) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/GC.c".as_ptr(), 766);
                }
            } else {
                freeChain((*r#gen).old_blocks);
            }

            (*r#gen).old_blocks = null_mut::<bdescr>();
            (*r#gen).n_old_blocks = 0;
            freeChain((*r#gen).large_objects);
            (*r#gen).large_objects = (*r#gen).scavenged_large_objects;
            (*r#gen).n_large_blocks = (*r#gen).n_scavenged_large_blocks;
            (*r#gen).n_large_words = countOccupied((*r#gen).large_objects) as memcount;
            (*r#gen).n_new_large_words = 0;
            bd = (*r#gen).compact_objects;

            while !bd.is_null() {
                next = (*bd).link as *mut bdescr;

                compactFree(
                    (*((*bd).start as *mut StgCompactNFDataBlock)).owner as *mut StgCompactNFData,
                );

                bd = next;
            }

            (*r#gen).compact_objects = (*r#gen).live_compact_objects;
            (*r#gen).n_compact_blocks = (*r#gen).n_live_compact_blocks;
        } else {
            bd = (*r#gen).scavenged_large_objects;

            while !bd.is_null() {
                next = (*bd).link as *mut bdescr;
                dbl_link_onto(bd, &raw mut (*r#gen).large_objects);
                (*r#gen).n_large_words = (*r#gen).n_large_words.wrapping_add(
                    (*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as memcount,
                );

                bd = next;
            }

            bd = (*r#gen).live_compact_objects;

            while !bd.is_null() {
                next = (*bd).link as *mut bdescr;
                dbl_link_onto(bd, &raw mut (*r#gen).compact_objects);
                bd = next;
            }

            (*r#gen).n_large_blocks = (*r#gen)
                .n_large_blocks
                .wrapping_add((*r#gen).n_scavenged_large_blocks);
            (*r#gen).n_compact_blocks = (*r#gen)
                .n_compact_blocks
                .wrapping_add((*r#gen).n_live_compact_blocks);
        }

        if (countBlocks((*r#gen).large_objects) == (*r#gen).n_large_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 827);
        }

        if (countOccupied((*r#gen).large_objects) == (*r#gen).n_large_words) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 828);
        }

        (*r#gen).scavenged_large_objects = null_mut::<bdescr>();
        (*r#gen).n_scavenged_large_blocks = 0;
        (*r#gen).live_compact_objects = null_mut::<bdescr>();
        (*r#gen).n_live_compact_blocks = 0;
        live_words = live_words.wrapping_add(genLiveWords(r#gen));
        live_blocks = live_blocks.wrapping_add(genLiveBlocks(r#gen));

        let mut i_0: u32 = 0;

        while i_0 < getNumCapabilities() as u32 {
            live_words = live_words.wrapping_add(gcThreadLiveWords(i_0, (*r#gen).no));
            live_blocks = live_blocks.wrapping_add(gcThreadLiveBlocks(i_0, (*r#gen).no));
            i_0 = i_0.wrapping_add(1);
        }

        g = g.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        n = 0;

        while n < getNumCapabilities() as u32 {
            nonmovingAddUpdRemSetBlocks(
                &raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(n))
                    .upd_rem_set,
            );

            n = n.wrapping_add(1);
        }
    }

    if (*oldest_gen).scavenged_large_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 872);
    }

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && major_gc as i32 != 0 {
        let mut concurrent = false;

        if ((*oldest_gen).old_threads
            == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32
            as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 880);
        }

        if (*oldest_gen).old_weak_ptr_list.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 887);
        }

        nonmovingAddUpdRemSetBlocks(
            &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap).upd_rem_set,
        );

        nonmovingCollect(
            &raw mut dead_weak_ptr_list,
            &raw mut resurrected_threads,
            concurrent,
        );
    }

    if major_gc as i32 != 0 && RtsFlags.GcFlags.generations > 1 && !RtsFlags.GcFlags.useNonmoving {
        resizeGenerations();
    }

    if !mark_stack_top_bd.is_null() {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"mark stack: %d blocks".as_ptr(),
                countBlocks(mark_stack_top_bd),
            );
        }

        freeChain(mark_stack_top_bd);
    }

    g = 0;

    while g <= N {
        r#gen = generations.offset(g as isize) as *mut generation;

        if !(*r#gen).bitmap.is_null() {
            freeGroup((*r#gen).bitmap);
            (*r#gen).bitmap = null_mut::<bdescr>();
        }

        g = g.wrapping_add(1);
    }

    resize_nursery();
    resetNurseries();

    if major_gc as i32 != 0 && !RtsFlags.GcFlags.useNonmoving {
        gcCAFs();
    }

    updateStableNameTable(major_gc);
    stablePtrUnlock();

    if major_gc as i32 != 0 && !RtsFlags.GcFlags.useNonmoving {
        checkUnload();
    }

    scheduleFinalizers(cap, dead_weak_ptr_list);

    if RtsFlags.DebugFlags.sanity {
        checkSanity(
            1 != 0,
            major_gc as i32 != 0 && !RtsFlags.GcFlags.useNonmoving,
        );
    }

    if config.do_heap_census {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"performing heap census".as_ptr());
        }

        heapCensus(mut_time);
    }

    if performTickySample {
        emitTickyCounterSamples();
        performTickySample = 0 != 0;
    }

    resurrectThreads(resurrected_threads);
    commitMBlockFreeing();

    if major_gc {
        let mut need_prealloc: W_ = 0;
        let mut need_copied_live: W_ = 0;
        let mut need_uncopied_live: W_ = 0;
        let mut need: W_ = 0;
        let mut got: W_ = 0;
        let mut extra_needed: W_ = 0;
        let mut i_1: u32 = 0;
        need_copied_live = 0;
        need_uncopied_live = 0;
        i_1 = 0;

        while i_1 < RtsFlags.GcFlags.generations {
            need_copied_live = (need_copied_live as StgWord).wrapping_add(genLiveCopiedWords(
                generations.offset(i_1 as isize) as *mut generation,
            )) as W_ as W_;

            need_uncopied_live = (need_uncopied_live as StgWord).wrapping_add(genLiveUncopiedWords(
                generations.offset(i_1 as isize) as *mut generation,
            )) as W_ as W_;

            i_1 = i_1.wrapping_add(1);
        }

        need_copied_live = (need_copied_live
            .wrapping_add(BLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !BLOCK_MASK as W_)
            .wrapping_div(BLOCK_SIZE_W as W_);
        need_uncopied_live = (need_uncopied_live
            .wrapping_add(BLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !BLOCK_MASK as W_)
            .wrapping_div(BLOCK_SIZE_W as W_);

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"(before) copied_live: %d; uncopied_live: %d".as_ptr(),
                need_copied_live,
                need_uncopied_live,
            );
        }

        extra_needed = 0;

        if RtsFlags.GcFlags.minOldGenSize as W_ >= need_copied_live.wrapping_add(need_uncopied_live)
        {
            extra_needed = (RtsFlags.GcFlags.minOldGenSize as W_)
                .wrapping_sub(need_copied_live.wrapping_add(need_uncopied_live));
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"(minOldGen: %d; extra_needed: %d".as_ptr(),
                RtsFlags.GcFlags.minOldGenSize,
                extra_needed,
            );
        }

        if (*oldest_gen).compact != 0 || RtsFlags.GcFlags.useNonmoving as i32 != 0 {
            need_uncopied_live = need_uncopied_live.wrapping_add(extra_needed);
        } else {
            need_copied_live = need_copied_live.wrapping_add(extra_needed);
        }

        if (need_uncopied_live.wrapping_add(need_copied_live)
            >= RtsFlags.GcFlags.minOldGenSize as W_) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 1042);
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"(after) copied_live: %d; uncopied_live: %d".as_ptr(),
                need_copied_live,
                need_uncopied_live,
            );
        }

        need_prealloc = 0;
        i_1 = 0;

        while i_1 < n_nurseries {
            need_prealloc = (need_prealloc as StgWord)
                .wrapping_add((*nurseries.offset(i_1 as isize)).n_blocks as StgWord)
                as W_ as W_;
            i_1 = i_1.wrapping_add(1);
        }

        need_prealloc = need_prealloc.wrapping_add(RtsFlags.GcFlags.largeAllocLim as W_);
        need_prealloc = need_prealloc.wrapping_add(countAllocdBlocks(exec_block));
        need_prealloc = need_prealloc.wrapping_add(arenaBlocks() as W_);

        consec_idle_gcs = if config.overflow_gc as i32 != 0 {
            0
        } else {
            consec_idle_gcs + 1
        };

        let mut scaled_factor = if RtsFlags.GcFlags.returnDecayFactor > 0 {
            RtsFlags.GcFlags.oldGenFactor
                / pow(
                    2,
                    consec_idle_gcs as f32 as f64 / RtsFlags.GcFlags.returnDecayFactor,
                )
        } else {
            RtsFlags.GcFlags.oldGenFactor
        };

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"factors: %f %d %f".as_ptr(),
                RtsFlags.GcFlags.oldGenFactor,
                consec_idle_gcs,
                scaled_factor,
            );
        }

        let mut unavoidable_copied_need_factor = if (*oldest_gen).compact != 0 {
            1.2f64
        } else {
            2
        };

        let mut unavoidable_uncopied_need_factor = 1.2f64;
        let mut scaled_needed: W_ = ((scaled_factor + unavoidable_copied_need_factor)
            * need_copied_live as f64
            + (scaled_factor + unavoidable_uncopied_need_factor) * need_uncopied_live as f64)
            as W_;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"factors_2: %f %f".as_ptr(),
                (scaled_factor + unavoidable_copied_need_factor) * need_copied_live as f64,
                (scaled_factor + unavoidable_uncopied_need_factor) * need_uncopied_live as f64,
            );
        }

        need = need_prealloc.wrapping_add(scaled_needed);

        need = ({
            let mut _a: u32 = RtsFlags.GcFlags.heapSizeSuggestion as u32;
            let mut _b: u32 = need as u32;

            if _a <= _b { _b } else { _a as u32 }
        }) as W_;

        if RtsFlags.GcFlags.maxHeapSize != 0 {
            need = ({
                let mut _a: u32 = RtsFlags.GcFlags.maxHeapSize as u32;
                let mut _b: u32 = need as u32;

                if _a <= _b { _a } else { _b as u32 }
            }) as W_;
        }

        need = (1 as W_).wrapping_add(
            ((need
                .wrapping_sub(
                    (((1 as u64) << 20 as i32) as W_)
                        .wrapping_sub(
                            ((0x40 as u64).wrapping_mul(
                                ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                            ) as W_)
                                .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                .wrapping_sub(1 as W_)
                                & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                        )
                        .wrapping_div(((1 as u64) << 12 as i32) as W_),
                )
                .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                .wrapping_add(MBLOCK_SIZE as W_)
                .wrapping_sub(1 as W_)
                & !MBLOCK_MASK as W_) as *mut c_void as W_)
                .wrapping_div(MBLOCK_SIZE as W_),
        );

        got = mblocks_allocated;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(c"Returning: %d %d".as_ptr(), got, need);
        }

        let mut returned: u32 = 0;

        if got > need {
            returned = returnMemoryToOS(got.wrapping_sub(need) as u32);
        }

        traceEventMemReturn(cap, got as u32, need as u32, returned);

        let mut after: W_ = got.wrapping_sub(returned as W_);

        if RtsFlags.GcFlags.maxHeapSize != 0
            && after
                > (1 as W_).wrapping_add(
                    (((RtsFlags.GcFlags.maxHeapSize as W_)
                        .wrapping_sub(
                            (((1 as u64) << 20 as i32) as W_)
                                .wrapping_sub(
                                    ((0x40 as u64).wrapping_mul(
                                        ((1 as u64) << 20 as i32)
                                            .wrapping_div((1 as u64) << 12 as i32),
                                    ) as W_)
                                        .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                        .wrapping_sub(1 as W_)
                                        & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                                )
                                .wrapping_div(((1 as u64) << 12 as i32) as W_),
                        )
                        .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                        .wrapping_add(MBLOCK_SIZE as W_)
                        .wrapping_sub(1 as W_)
                        & !MBLOCK_MASK as W_) as *mut c_void as W_)
                        .wrapping_div(MBLOCK_SIZE as W_),
                )
        {
            heapOverflow();
        }
    }

    if RtsFlags.DebugFlags.gc {
        statDescribeGens();
    }

    memInventory(RtsFlags.DebugFlags.gc);
    stat_endGCWorker(cap, &raw mut the_gc_thread as *mut gc_thread_);

    stat_endGC(
        cap,
        &raw mut the_gc_thread as *mut gc_thread_,
        live_words as W_,
        copied as W_,
        (live_blocks as W_)
            .wrapping_mul(BLOCK_SIZE_W as W_)
            .wrapping_sub(live_words as W_),
        N,
        n_gc_threads,
        gc_threads as *mut *mut gc_thread_,
        par_max_copied as W_,
        par_balanced_copied as W_,
        any_work as W_,
        scav_find_work as W_,
        max_n_todo_overflow as W_,
    );

    if RtsFlags.MiscFlags.install_signal_handlers {
        unblockUserSignals();
    }
}

unsafe fn heapOverflow() {
    heap_overflow = true;
}

unsafe fn new_gc_thread(mut n: u32, mut t: *mut gc_thread) {
    let mut g: u32 = 0;
    let mut ws = null_mut::<gen_workspace>();
    (*t).cap = getCapability(n);
    (*t).thread_index = n;
    (*t).free_blocks = null_mut::<bdescr>();
    (*t).gc_count = 0;
    init_gc_thread(t);
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        ws = (&raw mut (*t).gens as *mut gen_workspace).offset(g as isize) as *mut gen_workspace;
        (*ws).0.r#gen = generations.offset(g as isize) as *mut generation;

        if (g == (*(*ws).0.r#gen).no) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 1195);
        }

        (*ws).0.my_gct = t as *mut gc_thread_;

        let mut bd = allocBlockOnNode(n.wrapping_rem(n_numa_nodes));
        initBdescr(bd, (*ws).0.r#gen, (*(*ws).0.r#gen).to as *mut generation);
        (*bd).flags = BF_EVACUATED as StgWord16;
        (*bd).c2rust_unnamed.free = (*bd).start;
        (*bd).u.scan = (*bd).c2rust_unnamed.free;
        (*ws).0.todo_bd = bd;
        (*ws).0.todo_free = (*bd).c2rust_unnamed.free;
        (*ws).0.todo_lim = (*bd).start.offset(BLOCK_SIZE_W as isize);
        (*ws).0.todo_q = newWSDeque(128);
        (*ws).0.todo_overflow = null_mut::<bdescr>();
        (*ws).0.n_todo_overflow = 0;
        (*ws).0.todo_large_objects = null_mut::<bdescr>();
        (*ws).0.todo_seg = END_NONMOVING_TODO_LIST as *mut NonmovingSegment;
        (*ws).0.part_list = null_mut::<bdescr>();
        (*ws).0.n_part_blocks = 0;
        (*ws).0.n_part_words = 0;
        (*ws).0.scavd_list = null_mut::<bdescr>();
        (*ws).0.n_scavd_blocks = 0;
        (*ws).0.n_scavd_words = 0;
        g = g.wrapping_add(1);
    }
}

unsafe fn initGcThreads(mut from: u32, mut to: u32) {
    if (from == 0 && to == 1) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1263);
    }

    gc_threads = stgMallocBytes(
        size_of::<*mut gc_thread>() as usize,
        c"alloc_gc_threads".as_ptr(),
    ) as *mut *mut gc_thread;

    let ref mut fresh11 = *gc_threads.offset(0);
    *fresh11 = &raw mut the_gc_thread as *mut gc_thread;
    new_gc_thread(0, *gc_threads.offset(0));
}

unsafe fn freeGcThreads() {
    let mut g: u32 = 0;

    if !gc_threads.is_null() {
        g = 0;

        while g < RtsFlags.GcFlags.generations {
            freeWSDeque(
                (*(&raw mut (**gc_threads.offset(0)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_q,
            );

            g = g.wrapping_add(1);
        }

        stgFree(gc_threads as *mut c_void);
        gc_threads = null_mut::<*mut gc_thread>();
    }
}

unsafe fn inc_running() -> StgWord {
    let mut new: StgWord = 0;
    new = atomic_inc(&raw mut gc_running_threads as StgVolatilePtr, 1);

    return new;
}

unsafe fn dec_running() -> StgWord {
    if (gc_running_threads != 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1322);
    }

    let mut r = atomic_dec(&raw mut gc_running_threads as StgVolatilePtr, 1);

    return r;
}

unsafe fn scavenge_until_all_done() {
    let mut r: u32 = 0;
    scavenge_loop();
    collect_gct_blocks();
    r = dec_running() as u32;
    traceEventGcIdle((*(&raw mut the_gc_thread as *mut gc_thread)).cap);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"%d GC threads still running".as_ptr(), r);
    }

    traceEventGcDone((*(&raw mut the_gc_thread as *mut gc_thread)).cap);
}

unsafe fn wakeup_gc_threads(mut me: u32, mut idle_cap: *mut bool) {}

unsafe fn shutdown_gc_threads(mut me: u32, mut idle_cap: *mut bool) {}

unsafe fn stash_mut_list(mut cap: *mut Capability, mut gen_no: u32) {
    let ref mut fresh7 = *(*cap).saved_mut_lists.offset(gen_no as isize);
    *fresh7 = *(*cap).mut_lists.offset(gen_no as isize);

    let ref mut fresh8 = *(*cap).mut_lists.offset(gen_no as isize);
    *fresh8 = allocBlockOnNode_sync((*cap).node);
}

unsafe fn prepare_collected_gen(mut r#gen: *mut generation) {
    let mut i: u32 = 0;
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    let mut ws = null_mut::<gen_workspace>();
    let mut bd = null_mut::<bdescr>();
    let mut next = null_mut::<bdescr>();
    g = (*r#gen).no;

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && g == (*oldest_gen).no {
        i = 0;

        while i < getNumCapabilities() as u32 {
            stash_mut_list(getCapability(i), g);
            i = i.wrapping_add(1);
        }
    } else if g != 0 {
        i = 0;

        while i < getNumCapabilities() as u32 {
            let mut old = *(*getCapability(i)).mut_lists.offset(g as isize);
            freeChain(old);

            let mut new = allocBlockOnNode(i.wrapping_rem(n_numa_nodes));
            let ref mut fresh9 = *(*getCapability(i)).mut_lists.offset(g as isize);
            *fresh9 = new;
            i = i.wrapping_add(1);
        }
    }

    r#gen = generations.offset(g as isize) as *mut generation;

    if ((*r#gen).no == g) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1693);
    }

    (*r#gen).old_threads = (*r#gen).threads;
    (*r#gen).threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;

    if !(RtsFlags.GcFlags.useNonmoving as i32 != 0 && g == (*oldest_gen).no) {
        (*r#gen).old_blocks = (*r#gen).blocks;
        (*r#gen).n_old_blocks = (*r#gen).n_blocks;
        (*r#gen).blocks = null_mut::<bdescr>();
        (*r#gen).n_blocks = 0;
        (*r#gen).n_words = 0;
        (*r#gen).live_estimate = 0;
    }

    if (*r#gen).scavenged_large_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1713);
    }

    if ((*r#gen).n_scavenged_large_blocks == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1714);
    }

    if (*r#gen).live_compact_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1715);
    }

    if ((*r#gen).n_live_compact_blocks == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1716);
    }

    n = 0;

    while n < getNumCapabilities() as u32 {
        ws = (&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
            .offset((*r#gen).no as isize) as *mut gen_workspace;
        bd = (*ws).0.part_list;

        while !bd.is_null() {
            next = (*bd).link as *mut bdescr;
            (*bd).link = (*r#gen).old_blocks as *mut bdescr_;
            (*r#gen).old_blocks = bd;
            (*r#gen).n_old_blocks = (*r#gen).n_old_blocks.wrapping_add((*bd).blocks as memcount);
            bd = next;
        }

        (*ws).0.part_list = null_mut::<bdescr>();
        (*ws).0.n_part_blocks = 0;
        (*ws).0.n_part_words = 0;

        if (*ws).0.scavd_list.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 1733);
        }

        if ((*ws).0.n_scavd_blocks == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 1734);
        }

        if ((*ws).0.n_scavd_words == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 1735);
        }

        if (*ws).0.todo_free != (*(*ws).0.todo_bd).start {
            (*(*ws).0.todo_bd).c2rust_unnamed.free = (*ws).0.todo_free;
            (*(*ws).0.todo_bd).link = (*r#gen).old_blocks as *mut bdescr_;
            (*r#gen).old_blocks = (*ws).0.todo_bd;
            (*r#gen).n_old_blocks = (*r#gen)
                .n_old_blocks
                .wrapping_add((*(*ws).0.todo_bd).blocks as memcount);
            alloc_todo_block(ws, 0);
        }

        n = n.wrapping_add(1);
    }

    bd = (*r#gen).old_blocks;

    while !bd.is_null() {
        (*bd).flags = ((*bd).flags as i32 & !BF_EVACUATED) as StgWord16;
        bd = (*bd).link as *mut bdescr;
    }

    bd = (*r#gen).large_objects;

    while !bd.is_null() {
        (*bd).flags = ((*bd).flags as i32 & !BF_EVACUATED) as StgWord16;
        bd = (*bd).link as *mut bdescr;
    }

    bd = (*r#gen).compact_objects;

    while !bd.is_null() {
        (*bd).flags = ((*bd).flags as i32 & !BF_EVACUATED) as StgWord16;
        bd = (*bd).link as *mut bdescr;
    }

    if (*r#gen).mark != 0 {
        let mut bitmap_size: StgWord = 0;
        let mut bitmap_bdescr = null_mut::<bdescr>();
        let mut bitmap = null_mut::<StgWord>();
        bitmap_size = (*r#gen)
            .n_old_blocks
            .wrapping_mul(BLOCK_SIZE as memcount)
            .wrapping_div(
                (BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize) as memcount,
            ) as StgWord;

        if bitmap_size > 0 {
            bitmap_bdescr = allocGroup(
                (bitmap_size
                    .wrapping_add(BLOCK_SIZE as W_)
                    .wrapping_sub(1 as W_)
                    & !BLOCK_MASK as W_)
                    .wrapping_div(BLOCK_SIZE as W_),
            );

            (*r#gen).bitmap = bitmap_bdescr;
            bitmap = (*bitmap_bdescr).start as *mut StgWord;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                trace_(c"bitmap_size: %d, bitmap: %p".as_ptr(), bitmap_size, bitmap);
            }

            memset(bitmap as *mut c_void, 0, bitmap_size as usize);
            bd = (*r#gen).old_blocks;

            while !bd.is_null() {
                (*bd).u.bitmap = bitmap;
                bitmap =
                    bitmap.offset(BLOCK_SIZE_W.wrapping_div(
                        (BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize),
                    ) as isize);

                if (*bd).flags as i32 & BF_FRAGMENTED == 0 {
                    (*bd).flags = ((*bd).flags as i32 | BF_MARKED) as StgWord16;
                }

                (*bd).flags = ((*bd).flags as i32 & !BF_SWEPT) as StgWord16;
                bd = (*bd).link as *mut bdescr;
            }
        }
    }
}

unsafe fn prepare_uncollected_gen(mut r#gen: *mut generation) {
    let mut i: u32 = 0;

    if ((*r#gen).no > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1814);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        stash_mut_list(getCapability(i), (*r#gen).no);
        i = i.wrapping_add(1);
    }

    if (*r#gen).scavenged_large_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1823);
    }

    if ((*r#gen).n_scavenged_large_blocks == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GC.c".as_ptr(), 1824);
    };
}

unsafe fn collect_gct_blocks() {
    let mut g: u32 = 0;
    let mut ws = null_mut::<gen_workspace>();
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        ws = (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
            .offset(g as isize) as *mut gen_workspace;

        if !(*ws).0.scavd_list.is_null() {
            if (*(&raw mut the_gc_thread as *mut gc_thread))
                .scan_bd
                .is_null() as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/GC.c".as_ptr(), 1848);
            }

            if (countBlocks((*ws).0.scavd_list) == (*ws).0.n_scavd_blocks) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/GC.c".as_ptr(), 1849);
            }

            prev = null_mut::<bdescr>();
            bd = (*ws).0.scavd_list;

            while !bd.is_null() {
                prev = bd;
                bd = (*bd).link as *mut bdescr;
            }

            if !prev.is_null() {
                (*prev).link = (*(*ws).0.r#gen).blocks as *mut bdescr_;
                (*(*ws).0.r#gen).blocks = (*ws).0.scavd_list;
            }

            (*(*ws).0.r#gen).n_blocks = ((*(*ws).0.r#gen).n_blocks as StgWord)
                .wrapping_add((*ws).0.n_scavd_blocks)
                as memcount as memcount;
            (*(*ws).0.r#gen).n_words = ((*(*ws).0.r#gen).n_words as StgWord)
                .wrapping_add((*ws).0.n_scavd_words)
                as memcount as memcount;
            (*ws).0.scavd_list = null_mut::<bdescr>();
            (*ws).0.n_scavd_blocks = 0;
            (*ws).0.n_scavd_words = 0;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn collect_pinned_object_blocks() {
    let use_nonmoving = RtsFlags.GcFlags.useNonmoving;

    let r#gen = if use_nonmoving as i32 != 0 && major_gc as i32 != 0 {
        oldest_gen
    } else {
        g0
    };

    let mut n: u32 = 0;

    while n < getNumCapabilities() as u32 {
        let mut last = null_mut::<bdescr>();

        if use_nonmoving as i32 != 0 && r#gen == oldest_gen {
            let mut bd = (*getCapability(n)).pinned_object_blocks;

            while !bd.is_null() {
                (*bd).flags = ((*bd).flags as i32 | BF_NONMOVING) as StgWord16;
                (*bd).r#gen = oldest_gen as *mut generation_;
                (*bd).gen_no = (*oldest_gen).no as StgWord16;
                (*oldest_gen).n_large_words = (*oldest_gen).n_large_words.wrapping_add(
                    (*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as memcount,
                );

                (*oldest_gen).n_large_blocks = (*oldest_gen)
                    .n_large_blocks
                    .wrapping_add((*bd).blocks as memcount);
                last = bd;
                bd = (*bd).link as *mut bdescr;
            }
        } else {
            let mut bd_0 = (*getCapability(n)).pinned_object_blocks;

            while !bd_0.is_null() {
                last = bd_0;
                bd_0 = (*bd_0).link as *mut bdescr;
            }
        }

        if !last.is_null() {
            (*last).link = (*r#gen).large_objects as *mut bdescr_;

            if !(*r#gen).large_objects.is_null() {
                (*(*r#gen).large_objects).u.back = last as *mut bdescr_;
            }

            (*r#gen).large_objects = (*getCapability(n)).pinned_object_blocks;

            let ref mut fresh10 = (*getCapability(n)).pinned_object_blocks;
            *fresh10 = null_mut::<bdescr>();
        }

        n = n.wrapping_add(1);
    }
}

unsafe fn init_gc_thread(mut t: *mut gc_thread) {
    (*t).static_objects = static_flag as StgWord as *mut StgClosure;
    (*t).scavenged_static_objects = static_flag as StgWord as *mut StgClosure;
    (*t).scan_bd = null_mut::<bdescr>();
    (*t).mut_lists = (*(*t).cap).mut_lists;
    (*t).evac_gen_no = 0;
    (*t).failed_to_evac = false;
    (*t).eager_promotion = true;
    (*t).thunk_selector_depth = 0;
    (*t).copied = 0;
    (*t).scanned = 0;
    (*t).any_work = 0;
    (*t).scav_find_work = 0;
    (*t).max_n_todo_overflow = 0;
}

unsafe fn mark_root(mut user: *mut c_void, mut root: *mut *mut StgClosure) {
    evacuate(root);
}

unsafe fn resizeGenerations() {
    let mut g: u32 = 0;
    let mut live: W_ = 0;
    let mut size: W_ = 0;
    let mut min_alloc: W_ = 0;
    let mut words: W_ = 0;
    let max: W_ = RtsFlags.GcFlags.maxHeapSize as W_;
    let gens: W_ = RtsFlags.GcFlags.generations as W_;

    if (*oldest_gen).live_estimate != 0 {
        words = (*oldest_gen).live_estimate as W_;
    } else {
        words = (*oldest_gen).n_words as W_;
    }

    live = words
        .wrapping_add(BLOCK_SIZE_W as W_)
        .wrapping_sub(1 as W_)
        .wrapping_div(BLOCK_SIZE_W as W_)
        .wrapping_add((*oldest_gen).n_large_blocks as W_)
        .wrapping_add((*oldest_gen).n_compact_blocks as W_);

    size = ({
        let mut _a = live as f64 * RtsFlags.GcFlags.oldGenFactor as f64;
        let mut _b = RtsFlags.GcFlags.minOldGenSize as f64;

        if _a <= _b { _b as f64 } else { _a as f64 }
    }) as W_;

    if RtsFlags.GcFlags.heapSizeSuggestionAuto {
        if max > 0 {
            RtsFlags.GcFlags.heapSizeSuggestion = ({
                let _a: W_ = max as W_;
                let _b: W_ = size as W_;

                if _a as W_ <= _b as W_ {
                    _a as W_
                } else {
                    _b as W_
                }
            }) as u32;
        } else {
            RtsFlags.GcFlags.heapSizeSuggestion = size as u32;
        }
    }

    min_alloc = ({
        let mut _a = RtsFlags.GcFlags.pcFreeHeap as f64 * max as f64 / 200;
        let mut _b = (RtsFlags.GcFlags.minAllocAreaSize as W_)
            .wrapping_mul(getNumCapabilities() as W_) as f64;

        if _a <= _b { _b as f64 } else { _a as f64 }
    }) as W_;

    if !RtsFlags.GcFlags.useNonmoving
        && (RtsFlags.GcFlags.compact as i32 != 0
            || max > 0
                && (*oldest_gen).n_blocks as f64
                    > RtsFlags.GcFlags.compactThreshold * max as f64 / 100)
    {
        (*oldest_gen).mark = 1;
        (*oldest_gen).compact = 1;
    } else {
        (*oldest_gen).mark = 0;
        (*oldest_gen).compact = 0;
    }

    if RtsFlags.GcFlags.sweep {
        (*oldest_gen).mark = 1;
    }

    if max != 0 {
        if max < min_alloc {
            heapOverflow();
        }

        if (*oldest_gen).compact != 0 || RtsFlags.GcFlags.useNonmoving as i32 != 0 {
            if size
                .wrapping_add(
                    size.wrapping_sub(1 as W_)
                        .wrapping_mul(gens.wrapping_sub(2 as W_))
                        .wrapping_mul(2 as W_),
                )
                .wrapping_add(min_alloc)
                > max
            {
                size = max.wrapping_sub(min_alloc).wrapping_div(
                    gens.wrapping_sub(1 as W_)
                        .wrapping_mul(2 as W_)
                        .wrapping_sub(1 as W_),
                );
            }
        } else if size
            .wrapping_mul(gens.wrapping_sub(1 as W_))
            .wrapping_mul(2 as W_)
            .wrapping_add(min_alloc)
            > max
        {
            size = max
                .wrapping_sub(min_alloc)
                .wrapping_div(gens.wrapping_sub(1 as W_).wrapping_mul(2 as W_));
        }

        if size < live {
            heapOverflow();
        }
    }

    g = 0;

    while (g as W_) < gens {
        (*generations.offset(g as isize)).max_blocks = size as memcount;
        g = g.wrapping_add(1);
    }
}

unsafe fn resize_nursery() {
    let min_nursery: StgWord = (RtsFlags.GcFlags.minAllocAreaSize as StgWord)
        .wrapping_mul(getNumCapabilities() as StgWord);

    if RtsFlags.GcFlags.generations == 1 {
        let mut blocks: W_ = 0;
        blocks = (*generations.offset(0)).n_blocks as W_;

        if RtsFlags.GcFlags.maxHeapSize != 0
            && blocks as f64 * RtsFlags.GcFlags.oldGenFactor * 2
                > RtsFlags.GcFlags.maxHeapSize as f64
        {
            let mut adjusted_blocks: i64 = 0;
            let mut pc_free: i32 = 0;
            adjusted_blocks = (RtsFlags.GcFlags.maxHeapSize as W_)
                .wrapping_sub((2 as W_).wrapping_mul(blocks)) as i64;

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                trace_(
                    c"near maximum heap size of 0x%x blocks, blocks = %d, adjusted to %ld".as_ptr(),
                    RtsFlags.GcFlags.maxHeapSize,
                    blocks,
                    adjusted_blocks,
                );
            }

            pc_free = (adjusted_blocks * 100 / RtsFlags.GcFlags.maxHeapSize as i64) as i32;

            if (pc_free as f64) < RtsFlags.GcFlags.pcFreeHeap {
                heapOverflow();
            }

            blocks = adjusted_blocks as W_;
        } else {
            blocks = (blocks as f64 * RtsFlags.GcFlags.oldGenFactor) as W_;

            if blocks < min_nursery {
                blocks = min_nursery as W_;
            }
        }

        resizeNurseries(blocks as StgWord);
    } else if RtsFlags.GcFlags.heapSizeSuggestion != 0 {
        let mut blocks_0: i64 = 0;
        let mut needed: StgWord = 0;
        calcNeeded(false, &raw mut needed);

        if N == 0 {
            g0_pcnt_kept = ((copied as usize)
                .wrapping_div(BLOCK_SIZE_W.wrapping_sub(10 as usize))
                .wrapping_mul(100 as usize) as StgWord)
                .wrapping_div(countNurseryBlocks()) as W_;
        }

        blocks_0 = (RtsFlags.GcFlags.heapSizeSuggestion as i64 - needed as i64) * 100
            / (100 + g0_pcnt_kept as i64);
        if blocks_0 < min_nursery as i64 {
            blocks_0 = min_nursery as i64;
        }

        resizeNurseries(blocks_0 as StgWord);
    } else {
        resizeNurseriesFixed();
    };
}

unsafe fn gcCAFs() {
    let mut i: u32 = 0;
    let mut prev = null_mut::<StgIndStatic>();
    let mut p = debug_caf_list;

    while p != END_OF_CAF_LIST as *mut StgIndStatic {
        let mut info = get_itbl(p as *mut StgClosure);

        if ((*info).r#type == 28) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GC.c".as_ptr(), 2215);
        }

        if (*p).static_link as StgWord & STATIC_BITS as StgWord | prev_static_flag as StgWord != 3 {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gccafs as i64 != 0 {
                trace_(c"CAF gc'd at %p".as_ptr(), p);
            }

            SET_INFO(p as *mut StgClosure, &raw const stg_GCD_CAF_info);

            if prev.is_null() {
                debug_caf_list = (*p).saved_info as *mut StgIndStatic;
            } else {
                (*prev).saved_info = (*p).saved_info;
            }
        } else {
            prev = p;
            i = i.wrapping_add(1);
        }

        p = (*p).saved_info as *mut StgIndStatic;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gccafs as i64 != 0 {
        trace_(c"%d CAFs live".as_ptr(), i);
    }
}

unsafe fn doIdleGCWork(mut cap: *mut Capability, mut all: bool) -> bool {
    return runSomeFinalizers(all);
}
