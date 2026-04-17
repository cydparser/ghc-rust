use crate::capability::Capability;
use crate::capability::Capability;
use crate::capability::{
    getCapability, n_numa_nodes, recordClosureMutated, recordMutableCap, regTableToCapability,
};
use crate::eventlog::event_log::postInitEvent;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::non_moving::{nonmoving_write_barrier_enabled, updateRemembSetPushClosure_};
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::prof::ccs::{era, user_era};
use crate::ffi::rts::spin_lock::initSpinLock;
use crate::ffi::rts::storage::block::{
    BF_EVACUATED, BF_LARGE, BF_PINNED, BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK,
    Bdescr, LARGE_OBJECT_THRESHOLD, MBLOCK_SIZE, allocBlockOnNode, allocGroupOnNode, bdescr,
    bdescr_, dbl_link_onto, freeGroup, initBlockAllocator,
};
use crate::ffi::rts::storage::block::{BLOCK_SIZE, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    INFO_PTR_TO_STRUCT, LOOKS_LIKE_CLOSURE_PTR, LOOKS_LIKE_INFO_PTR_NOT_NULL, SET_INFO_RELAXED,
    SET_INFO_RELEASE, doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
    itbl_to_thunk_itbl,
};
use crate::ffi::rts::storage::closures::{
    StgClosure_, StgInd, StgIndStatic, StgMVar, StgMutVar, StgTVar,
};
use crate::ffi::rts::storage::gc::{
    AdjustorExecutable, ListBlocksCb, generation, generation_, initBdescr, memcount, nursery,
    nursery_,
};
use crate::ffi::rts::storage::heap_alloc::{gc_alloc_block_sync, mblock_address_space};
use crate::ffi::rts::storage::m_block::{freeAllMBlocks, initMBlocks};
use crate::ffi::rts::storage::tso::{StgStack, StgTSO_};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts::{_assertFail, exitHeapOverflow};
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure,
    stg_IND_STATIC_info, stg_MUT_VAR_DIRTY_info, stg_TVAR_CLEAN_info, stg_TVAR_DIRTY_info,
    stg_WHITEHOLE_info, stg_dummy_ret_closure,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::StgWord32;
use crate::ffi::stg::types::{
    StgHalfWord, StgInt64, StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord16, StgWord32,
    StgWord64,
};
use crate::ffi::stg::{ASSIGN_Int64, BITS_PER_BYTE, PK_Int64, W_};
use crate::hs_ffi::{HS_INT32_MAX, HS_WORD_MAX};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::sm::block_alloc::{allocLargeChunkOnNode, clear_free_list, countBlocks};
use crate::sm::gc::{N, freeGcThreads, initGcThreads};
use crate::sm::gc_thread::{gc_threads, gen_workspace};
use crate::sm::non_moving::{
    NonmovingAllocator, NonmovingSegment, nonmoving_alloca_cnt, nonmovingClearSegment,
    nonmovingClearSegmentFreeBlocks, nonmovingExit, nonmovingHeap, nonmovingInit,
};
use crate::sm::non_moving_allocate::{nonmovingAllocate, nonmovingInitCapability};
use crate::sm::non_moving_mark::{
    n_nonmoving_compact_blocks, n_nonmoving_large_blocks, n_nonmoving_marked_compact_blocks,
    n_nonmoving_marked_large_blocks, nonmoving_compact_objects, nonmoving_compact_words,
    nonmoving_large_objects, nonmoving_large_words, updateRemembSetPushClosure,
    updateRemembSetPushStack, updateRemembSetPushTSO,
};
use crate::sm::sanity::checkNurserySanity;
use crate::sm::storage::{
    END_OF_CAF_LIST, STATIC_FLAG_LIST, clear_blocks, finishedNurseryBlock, newNurseryBlock,
};
use crate::stats::{stat_exitReport, statDescribeGens};
use crate::trace::{
    CAPSET_HEAP_DEFAULT, DEBUG_RTS, trace_, traceEventHeapAllocated, traceEventHeapInfo,
};

#[cfg(test)]
mod tests;

#[inline]
pub(crate) unsafe fn doYouWantToGC(mut cap: *mut Capability) -> bool {
    return (*(*cap).r.rCurrentNursery).link.is_null() && !getNewNursery(cap)
        || (&raw mut (*g0).n_new_large_words).load(Ordering::Relaxed) >= large_alloc_lim;
}

#[inline]
pub(crate) unsafe fn finishedNurseryBlock(mut cap: *mut Capability, mut bd: *mut bdescr) {
    (*cap).total_allocated = (*cap)
        .total_allocated
        .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as u64);
}

#[inline]
pub(crate) unsafe fn newNurseryBlock(mut bd: *mut bdescr) {
    (&raw mut (*bd).c2rust_unnamed.free).store((*bd).start, Ordering::Relaxed);
}

pub(crate) const STATIC_FLAG_LIST: i32 = 3;

pub(crate) const END_OF_CAF_LIST: *mut StgClosure = STATIC_FLAG_LIST as *mut StgClosure;

#[inline]
pub(crate) unsafe fn clear_blocks(mut bd: *mut bdescr) {
    memset(
        (*bd).start as *mut c_void,
        if RtsFlags.DebugFlags.zero_on_gc as i32 != 0 {
            0xaa
        } else {
            0
        },
        (BLOCK_SIZE as usize).wrapping_mul((*bd).blocks as usize),
    );
}

static mut dyn_caf_list: *mut StgIndStatic = null_mut::<StgIndStatic>();

static mut debug_caf_list: *mut StgIndStatic = null_mut::<StgIndStatic>();

static mut revertible_caf_list: *mut StgIndStatic = null_mut::<StgIndStatic>();

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = false;

static mut highMemDynamic: bool = false;

static mut large_alloc_lim: W_ = 0;

static mut exec_block: *mut bdescr = null_mut::<bdescr>();

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static mut generations: *mut generation = null_mut::<generation>();

#[ffi(compiler, ghc_lib, utils)]
#[unsafe(no_mangle)]
pub static mut g0: *mut generation = null_mut::<generation>();

static mut oldest_gen: *mut generation = null_mut::<generation>();

static mut nurseries: *mut nursery = null_mut::<nursery>();

static mut n_nurseries: u32 = 0;

const PINNED_EMPTY_SIZE: W_ = BLOCKS_PER_MBLOCK;

static mut next_nursery: [StgWord; 16] = [0; 16];

static mut sm_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn initGeneration(mut r#gen: *mut generation, mut g: i32) {
    (*r#gen).no = g as u32;
    (*r#gen).collections = 0;
    (*r#gen).par_collections = 0;
    (*r#gen).failed_promotions = 0;
    (*r#gen).max_blocks = 0;
    (*r#gen).blocks = null_mut::<bdescr>();
    (*r#gen).n_blocks = 0;
    (*r#gen).n_words = 0;
    (*r#gen).live_estimate = 0;
    (*r#gen).old_blocks = null_mut::<bdescr>();
    (*r#gen).n_old_blocks = 0;
    (*r#gen).large_objects = null_mut::<bdescr>();
    (*r#gen).n_large_blocks = 0;
    (*r#gen).n_large_words = 0;
    (*r#gen).n_new_large_words = 0;
    (*r#gen).compact_objects = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks = 0;
    (*r#gen).compact_blocks_in_import = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks_in_import = 0;
    (*r#gen).scavenged_large_objects = null_mut::<bdescr>();
    (*r#gen).n_scavenged_large_blocks = 0;
    (*r#gen).live_compact_objects = null_mut::<bdescr>();
    (*r#gen).n_live_compact_blocks = 0;
    (*r#gen).compact_blocks_in_import = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks_in_import = 0;
    (*r#gen).mark = 0;
    (*r#gen).compact = 0;
    (*r#gen).bitmap = null_mut::<bdescr>();
    initSpinLock(&raw mut (*r#gen).sync);
    (*r#gen).threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*r#gen).old_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*r#gen).weak_ptr_list = null_mut::<StgWeak>();
    (*r#gen).old_weak_ptr_list = null_mut::<StgWeak>();
}

unsafe fn traceHeapInfo() {
    traceEventHeapInfo(
        CAPSET_HEAP_DEFAULT,
        RtsFlags.GcFlags.generations,
        (RtsFlags.GcFlags.maxHeapSize as u64).wrapping_mul(BLOCK_SIZE) as W_,
        (RtsFlags.GcFlags.minAllocAreaSize as u64).wrapping_mul(BLOCK_SIZE) as W_,
        MBLOCK_SIZE as W_,
        BLOCK_SIZE as W_,
    );
}

unsafe fn initStorage() {
    let mut g: u32 = 0;
    let mut n: u32 = 0;

    if !generations.is_null() {
        return;
    }

    initMBlocks();

    if LOOKS_LIKE_INFO_PTR_NOT_NULL(&raw const stg_BLOCKING_QUEUE_CLEAN_info as StgWord) as i32
        as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 188);
    }

    if LOOKS_LIKE_CLOSURE_PTR(&raw mut stg_dummy_ret_closure as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 189);
    }

    if !(&raw mut stg_dummy_ret_closure as W_ >= mblock_address_space.0.begin
        && (&raw mut stg_dummy_ret_closure as W_) < mblock_address_space.0.end) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 190);
    }

    initBlockAllocator();
    initMutex(&raw mut sm_mutex);

    generations = stgMallocBytes(
        (RtsFlags.GcFlags.generations as usize).wrapping_mul(size_of::<generation_>() as usize),
        c"initStorage: gens".as_ptr(),
    ) as *mut generation;

    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            204,
            __r,
        );
    }

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        initGeneration(generations.offset(g as isize) as *mut generation, g as i32);
        g = g.wrapping_add(1);
    }

    g0 = generations.offset(0) as *mut generation;
    oldest_gen = generations.offset(RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) as isize)
        as *mut generation;
    g = 0;

    while g < RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) {
        let ref mut fresh12 = (*generations.offset(g as isize)).to;
        *fresh12 = generations.offset(g.wrapping_add(1 as u32) as isize) as *mut generation
            as *mut generation_;
        g = g.wrapping_add(1);
    }

    (*oldest_gen).to = oldest_gen as *mut generation_;
    initSpinLock(&raw mut gc_alloc_block_sync);
    nonmovingInit();

    if RtsFlags.GcFlags.compact as i32 != 0 || RtsFlags.GcFlags.sweep as i32 != 0 {
        if RtsFlags.GcFlags.generations == 1 {
            errorBelch(c"WARNING: compact/sweep is incompatible with -G1; disabled".as_ptr());
        } else {
            (*oldest_gen).mark = 1;

            if RtsFlags.GcFlags.compact {
                (*oldest_gen).compact = 1;
            }
        }
    }

    (*generations.offset(0)).max_blocks = 0;
    dyn_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
    debug_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
    revertible_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;

    if RtsFlags.GcFlags.largeAllocLim > 0 {
        large_alloc_lim =
            (RtsFlags.GcFlags.largeAllocLim as usize).wrapping_mul(BLOCK_SIZE_W) as W_;
    } else {
        large_alloc_lim =
            (RtsFlags.GcFlags.minAllocAreaSize as usize).wrapping_mul(BLOCK_SIZE_W) as W_;
    }

    exec_block = null_mut::<bdescr>();
    N = 0;
    n = 0;

    while n < n_numa_nodes {
        write_volatile(&mut next_nursery[n as usize] as *mut StgWord, n as StgWord);
        n = n.wrapping_add(1);
    }

    storageAddCapabilities(0, getNumCapabilities() as u32);

    if RtsFlags.DebugFlags.gc {
        statDescribeGens();
    }

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            262,
        );
    }

    postInitEvent(Some(traceHeapInfo as unsafe extern "C" fn() -> ()));
}

unsafe fn storageAddCapabilities(mut from: u32, mut to: u32) {
    let mut n: u32 = 0;
    let mut g: u32 = 0;
    let mut i: u32 = 0;
    let mut new_n_nurseries: u32 = 0;
    let mut old_nurseries = null_mut::<nursery>();

    if RtsFlags.GcFlags.nurseryChunkSize == 0 {
        new_n_nurseries = to;
    } else {
        let mut total_alloc: memcount =
            to.wrapping_mul(RtsFlags.GcFlags.minAllocAreaSize) as memcount;

        new_n_nurseries = ({
            let mut _a: u32 = to as u32;
            let mut _b: u32 =
                total_alloc.wrapping_div(RtsFlags.GcFlags.nurseryChunkSize as memcount) as u32;

            if _a <= _b { _b } else { _a as u32 }
        });
    }

    old_nurseries = nurseries;

    if from > 0 {
        nurseries = stgReallocBytes(
            nurseries as *mut c_void,
            (new_n_nurseries as usize).wrapping_mul(size_of::<nursery_>() as usize),
            c"storageAddCapabilities".as_ptr(),
        ) as *mut nursery;
    } else {
        nurseries = stgMallocBytes(
            (new_n_nurseries as usize).wrapping_mul(size_of::<nursery_>() as usize),
            c"storageAddCapabilities".as_ptr(),
        ) as *mut nursery;
    }

    i = 0;

    while i < from {
        let mut index: u32 =
            (*getCapability(i)).r.rNursery.offset_from(old_nurseries) as i64 as u32;

        let ref mut fresh6 = (*getCapability(i)).r.rNursery;
        *fresh6 = nurseries.offset(index as isize) as *mut nursery as *mut nursery_;
        i = i.wrapping_add(1);
    }

    allocNurseries(n_nurseries, new_n_nurseries);
    n_nurseries = new_n_nurseries;
    assignNurseriesToCapabilities(from, to);
    n = from;

    while n < to {
        g = 1;

        while g < RtsFlags.GcFlags.generations {
            let ref mut fresh7 = *(*getCapability(n)).mut_lists.offset(g as isize);
            *fresh7 = allocBlockOnNode(n.wrapping_rem(n_numa_nodes));
            g = g.wrapping_add(1);
        }

        n = n.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        i = from;

        while i < to {
            nonmovingInitCapability(getCapability(i));
            i = i.wrapping_add(1);
        }
    }

    initGcThreads(from, to);
}

unsafe fn exitStorage() {
    nonmovingExit();
    updateNurseriesStats();
    stat_exitReport();
}

unsafe fn freeStorage(mut free_heap: bool) {
    stgFree(generations as *mut c_void);

    if free_heap {
        freeAllMBlocks();
    }

    closeMutex(&raw mut sm_mutex);
    stgFree(nurseries as *mut c_void);
    freeGcThreads();
}

unsafe fn listGenBlocks(mut cb: ListBlocksCb, mut user: *mut c_void, mut r#gen: *mut generation) {
    cb.expect("non-null function pointer")(user, (*r#gen).blocks);
    cb.expect("non-null function pointer")(user, (*r#gen).large_objects);
    cb.expect("non-null function pointer")(user, (*r#gen).compact_objects);
    cb.expect("non-null function pointer")(user, (*r#gen).compact_blocks_in_import);
}

unsafe fn listSegmentBlocks(
    mut cb: ListBlocksCb,
    mut user: *mut c_void,
    mut seg: *mut NonmovingSegment,
) {
    while !seg.is_null() {
        cb.expect("non-null function pointer")(user, Bdescr(seg as StgPtr));
        seg = (*seg).link;
    }
}

unsafe fn listAllBlocks(mut cb: ListBlocksCb, mut user: *mut c_void) {
    let mut g: u32 = 0;
    let mut i: u32 = 0;
    let mut s: u32 = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        i = 0;

        while i < getNumCapabilities() as u32 {
            cb.expect("non-null function pointer")(
                user,
                *(*getCapability(i)).mut_lists.offset(g as isize),
            );

            cb.expect("non-null function pointer")(
                user,
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .part_list,
            );

            cb.expect("non-null function pointer")(
                user,
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .scavd_list,
            );

            cb.expect("non-null function pointer")(
                user,
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_bd,
            );

            i = i.wrapping_add(1);
        }

        listGenBlocks(cb, user, generations.offset(g as isize) as *mut generation);
        g = g.wrapping_add(1);
    }

    i = 0;

    while i < n_nurseries {
        cb.expect("non-null function pointer")(user, (*nurseries.offset(i as isize)).blocks);

        i = i.wrapping_add(1);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        if !(*getCapability(i)).pinned_object_block.is_null() {
            cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_block);
        }

        cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_blocks);

        cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_empty);

        if RtsFlags.GcFlags.useNonmoving {
            s = 0;

            while s < nonmoving_alloca_cnt as u32 {
                listSegmentBlocks(
                    cb,
                    user,
                    *(*getCapability(i)).current_segments.offset(s as isize),
                );

                s = s.wrapping_add(1);
            }
        }

        i = i.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        s = 0;

        while s < nonmoving_alloca_cnt as u32 {
            listSegmentBlocks(
                cb,
                user,
                (*nonmovingHeap.allocators.offset(s as isize)).filled,
            );

            listSegmentBlocks(
                cb,
                user,
                (*nonmovingHeap.allocators.offset(s as isize)).saved_filled,
            );

            listSegmentBlocks(
                cb,
                user,
                (*nonmovingHeap.allocators.offset(s as isize)).active,
            );

            s = s.wrapping_add(1);
        }

        cb.expect("non-null function pointer")(user, nonmoving_large_objects);
        cb.expect("non-null function pointer")(user, nonmoving_compact_objects);
    }
}

unsafe fn lockCAF(mut reg: *mut StgRegTable, mut caf: *mut StgIndStatic) -> *mut StgInd {
    let mut orig_info = null::<StgInfoTable>();
    let mut cap = regTableToCapability(reg);
    let mut bh = null_mut::<StgInd>();
    orig_info = (&raw mut (*caf).header.info).load(Ordering::Relaxed);

    let mut cur_info = null::<StgInfoTable>();

    if orig_info == &raw const stg_IND_STATIC_info || orig_info == &raw const stg_WHITEHOLE_info {
        return null_mut::<StgInd>();
    }

    cur_info = cas(
        &raw mut (*caf).header.info as StgVolatilePtr,
        orig_info as StgWord,
        &raw const stg_WHITEHOLE_info as StgWord,
    ) as *const StgInfoTable;

    if cur_info != orig_info {
        return null_mut::<StgInd>();
    }

    let mut orig_info_tbl: *const StgInfoTable = INFO_PTR_TO_STRUCT(orig_info);

    if ((*orig_info_tbl).r#type == 21) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 566);
    }

    if ((*orig_info_tbl).layout.payload.ptrs == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 569);
    }

    if nonmoving_write_barrier_enabled as i64 != 0 {
        let mut thunk_info = itbl_to_thunk_itbl(orig_info_tbl);

        if (*thunk_info).i.srt != 0 {
            updateRemembSetPushClosure(
                cap,
                (thunk_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*thunk_info).i.srt as StgWord)
                    as *mut StgClosure,
            );
        }
    }

    (*caf).saved_info = orig_info;

    if RtsFlags.GcFlags.useNonmoving {
        bh = nonmovingAllocate(
            cap,
            (size_of::<StgInd>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgWord,
        ) as *mut StgInd;

        recordMutableCap(
            bh as *mut StgClosure,
            regTableToCapability(reg),
            (*oldest_gen).no,
        );
    } else {
        bh = allocate(
            cap,
            (size_of::<StgInd>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as W_,
        ) as *mut StgInd;
    }

    (*bh).indirectee = (*cap).r.rCurrentTSO as *mut StgClosure;

    let ref mut fresh19 = (*(bh as *mut StgClosure)).header.prof.ccs;
    *fresh19 = (*caf).header.prof.ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(bh as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(bh as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(bh as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*bh).header.info).store(&raw const stg_CAF_BLACKHOLE_info, Ordering::Relaxed);
    (&raw mut (*caf).indirectee).store(bh as *mut StgClosure, Ordering::Release);
    SET_INFO_RELEASE(caf as *mut StgClosure, &raw const stg_IND_STATIC_info);

    return bh;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn newCAF(
    mut reg: *mut StgRegTable,
    mut caf: *mut StgIndStatic,
) -> *mut StgInd {
    let mut bh = null_mut::<StgInd>();
    bh = lockCAF(reg, caf);

    if bh.is_null() {
        return null_mut::<StgInd>();
    }

    if keepCAFs as i32 != 0 && !(highMemDynamic as i32 != 0 && caf as *mut c_void > 0x80000000) {
        let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                621,
                __r,
            );
        }

        (*caf).static_link = dyn_caf_list as *mut StgClosure;
        dyn_caf_list = (caf as StgWord | STATIC_FLAG_LIST as StgWord) as *mut StgIndStatic;

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                624,
            );
        }
    } else {
        if (*oldest_gen).no != 0 && !RtsFlags.GcFlags.useNonmoving {
            recordMutableCap(
                caf as *mut StgClosure,
                regTableToCapability(reg),
                (*oldest_gen).no,
            );
        }

        let mut __r_0 = pthread_mutex_lock(&raw mut sm_mutex);

        if __r_0 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                645,
                __r_0,
            );
        }

        (*caf).saved_info = debug_caf_list as *const StgInfoTable;
        debug_caf_list = caf;

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                648,
            );
        }
    }

    return bh;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    keepCAFs = 1 != 0;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    highMemDynamic = 1 != 0;
}

unsafe fn newRetainedCAF(mut reg: *mut StgRegTable, mut caf: *mut StgIndStatic) -> *mut StgInd {
    let mut bh = null_mut::<StgInd>();
    bh = lockCAF(reg, caf);

    if bh.is_null() {
        return null_mut::<StgInd>();
    }

    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            685,
            __r,
        );
    }

    (*caf).static_link = revertible_caf_list as *mut StgClosure;
    revertible_caf_list = (caf as StgWord | STATIC_FLAG_LIST as StgWord) as *mut StgIndStatic;

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            690,
        );
    }

    return bh;
}

unsafe fn newGCdCAF(mut reg: *mut StgRegTable, mut caf: *mut StgIndStatic) -> *mut StgInd {
    let mut bh = null_mut::<StgInd>();
    bh = lockCAF(reg, caf);

    if bh.is_null() {
        return null_mut::<StgInd>();
    }

    if (*oldest_gen).no != 0 && !RtsFlags.GcFlags.useNonmoving {
        recordMutableCap(
            caf as *mut StgClosure,
            regTableToCapability(reg),
            (*oldest_gen).no,
        );
    }

    return bh;
}

unsafe fn allocNursery(mut node: u32, mut tail: *mut bdescr, mut blocks: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut i: W_ = 0;
    let mut n: W_ = 0;

    while blocks > 0 {
        n = ({
            let mut _a: W_ = (((1 as u64) << 20 as i32) as W_)
                .wrapping_sub(
                    ((0x40 as u64).wrapping_mul(
                        ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                    ) as W_)
                        .wrapping_add(((1 as u64) << 12 as i32) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                )
                .wrapping_div(((1 as u64) << 12 as i32) as W_);

            let mut _b: W_ = blocks as W_;

            if _a <= _b { _a } else { _b as W_ }
        });

        bd = allocLargeChunkOnNode(node, 1, n);
        n = (*bd).blocks as W_;
        blocks = blocks.wrapping_sub(n);
        i = 0;

        while i < n {
            initBdescr(bd.offset(i as isize) as *mut bdescr, g0, g0);
            (*bd.offset(i as isize)).blocks = 1;
            (*bd.offset(i as isize)).flags = 0;

            if i > 0 {
                let ref mut fresh7 = (*bd.offset(i as isize)).u.back;
                *fresh7 =
                    bd.offset(i.wrapping_sub(1 as W_) as isize) as *mut bdescr as *mut bdescr_;
            } else {
                let ref mut fresh8 = (*bd.offset(i as isize)).u.back;
                *fresh8 = null_mut::<bdescr_>();
            }

            if i.wrapping_add(1 as W_) < n {
                let ref mut fresh9 = (*bd.offset(i as isize)).link;
                *fresh9 =
                    bd.offset(i.wrapping_add(1 as W_) as isize) as *mut bdescr as *mut bdescr_;
            } else {
                let ref mut fresh10 = (*bd.offset(i as isize)).link;
                *fresh10 = tail as *mut bdescr_;

                if !tail.is_null() {
                    (*tail).u.back = bd.offset(i as isize) as *mut bdescr as *mut bdescr_;
                }
            }

            let ref mut fresh11 = (*bd.offset(i as isize)).c2rust_unnamed.free;
            *fresh11 = (*bd.offset(i as isize)).start;
            i = i.wrapping_add(1);
        }

        tail = bd.offset(0) as *mut bdescr;
    }

    return bd.offset(0) as *mut bdescr;
}

unsafe fn assignNurseryToCapability(mut cap: *mut Capability, mut n: u32) {
    if (n < n_nurseries) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 782);
    }

    (*cap).r.rNursery = nurseries.offset(n as isize) as *mut nursery as *mut nursery_;
    (*cap).r.rCurrentNursery = (*nurseries.offset(n as isize)).blocks as *mut bdescr_;
    newNurseryBlock((*nurseries.offset(n as isize)).blocks);
    (*cap).r.rCurrentAlloc = null_mut::<bdescr_>();

    if ((*(*cap).r.rCurrentNursery).node as u32 == (*cap).node) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 787);
    };
}

unsafe fn assignNurseriesToCapabilities(mut from: u32, mut to: u32) {
    let mut i: u32 = 0;
    let mut node: u32 = 0;
    i = from;

    while i < to {
        node = (*getCapability(i)).node;
        assignNurseryToCapability(getCapability(i), next_nursery[node as usize] as u32);

        write_volatile(
            &mut next_nursery[node as usize] as *mut StgWord,
            read_volatile::<StgWord>(&next_nursery[node as usize] as *const StgWord)
                .wrapping_add(n_numa_nodes as StgWord),
        );

        i = i.wrapping_add(1);
    }
}

unsafe fn allocNurseries(mut from: u32, mut to: u32) {
    let mut i: u32 = 0;
    let mut n_blocks: memcount = 0;

    if RtsFlags.GcFlags.nurseryChunkSize != 0 {
        n_blocks = RtsFlags.GcFlags.nurseryChunkSize as memcount;
    } else {
        n_blocks = RtsFlags.GcFlags.minAllocAreaSize as memcount;
    }

    i = from;

    while i < to {
        let ref mut fresh15 = (*nurseries.offset(i as isize)).blocks;

        *fresh15 = allocNursery(
            i.wrapping_rem(n_numa_nodes),
            null_mut::<bdescr>(),
            n_blocks as W_,
        );

        (*nurseries.offset(i as isize)).n_blocks = n_blocks;
        i = i.wrapping_add(1);
    }
}

unsafe fn resetNurseries() {
    let mut n: u32 = 0;
    n = 0;

    while n < n_numa_nodes {
        write_volatile(&mut next_nursery[n as usize] as *mut StgWord, n as StgWord);
        n = n.wrapping_add(1);
    }

    assignNurseriesToCapabilities(0, getNumCapabilities() as u32);

    let mut bd = null_mut::<bdescr>();
    n = 0;

    while n < n_nurseries {
        bd = (*nurseries.offset(n as isize)).blocks;

        while !bd.is_null() {
            if ((*bd).gen_no as i32 == 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Storage.c".as_ptr(), 838);
            }

            if ((*bd).r#gen == g0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Storage.c".as_ptr(), 839);
            }

            if ((*bd).node as u32 == n.wrapping_rem(n_numa_nodes)) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Storage.c".as_ptr(), 840);
            }

            if RtsFlags.DebugFlags.zero_on_gc {
                memset((*bd).start as *mut c_void, 0xaa, 1 << 12);
            }

            bd = (*bd).link as *mut bdescr;
        }

        n = n.wrapping_add(1);
    }
}

unsafe fn countNurseryBlocks() -> StgWord {
    let mut i: u32 = 0;
    let mut blocks: W_ = 0;
    i = 0;

    while i < n_nurseries {
        blocks = (blocks as StgWord)
            .wrapping_add((*nurseries.offset(i as isize)).n_blocks as StgWord)
            as W_ as W_;
        i = i.wrapping_add(1);
    }

    return blocks as StgWord;
}

unsafe fn resizeNurseriesEach(mut blocks: W_) {
    let mut i: u32 = 0;
    let mut node: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    let mut nursery_blocks: W_ = 0;
    let mut nursery = null_mut::<nursery>();
    i = 0;

    while i < n_nurseries {
        nursery = nurseries.offset(i as isize) as *mut nursery;
        nursery_blocks = (*nursery).n_blocks as W_;

        if !(nursery_blocks == blocks) {
            node = i.wrapping_rem(n_numa_nodes);

            if nursery_blocks < blocks {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(
                        c"increasing size of nursery from %d to %d blocks".as_ptr(),
                        nursery_blocks,
                        blocks,
                    );
                }

                (*nursery).blocks =
                    allocNursery(node, (*nursery).blocks, blocks.wrapping_sub(nursery_blocks));
            } else {
                let mut next_bd = null_mut::<bdescr>();

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                    trace_(
                        c"decreasing size of nursery from %d to %d blocks".as_ptr(),
                        nursery_blocks,
                        blocks,
                    );
                }

                bd = (*nursery).blocks;

                while nursery_blocks > blocks {
                    next_bd = (*bd).link as *mut bdescr;
                    (*next_bd).u.back = null_mut::<bdescr_>();
                    nursery_blocks = nursery_blocks.wrapping_sub((*bd).blocks as W_);
                    freeGroup(bd);
                    bd = next_bd;
                }

                (*nursery).blocks = bd;

                if nursery_blocks < blocks {
                    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                        trace_(
                            c"reincreasing size of nursery from %d to %d blocks".as_ptr(),
                            nursery_blocks,
                            blocks,
                        );
                    }

                    (*nursery).blocks =
                        allocNursery(node, (*nursery).blocks, blocks.wrapping_sub(nursery_blocks));
                }
            }

            (*nursery).n_blocks = blocks as memcount;

            if (countBlocks((*nursery).blocks) == (*nursery).n_blocks) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Storage.c".as_ptr(), 908);
            }
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn resizeNurseriesFixed() {
    let mut blocks: u32 = 0;

    if RtsFlags.GcFlags.nurseryChunkSize != 0 {
        blocks = RtsFlags.GcFlags.nurseryChunkSize;
    } else {
        blocks = RtsFlags.GcFlags.minAllocAreaSize;
    }

    resizeNurseriesEach(blocks as W_);
}

unsafe fn resizeNurseries(mut blocks: W_) {
    resizeNurseriesEach(blocks.wrapping_div(n_nurseries as W_));
}

unsafe fn getNewNursery(mut cap: *mut Capability) -> bool {
    let mut i: StgWord = 0;
    let mut node: u32 = (*cap).node;
    let mut n: u32 = 0;

    loop {
        i = next_nursery[node as usize];

        if i < n_nurseries as StgWord {
            if cas(
                (&raw mut next_nursery as *mut StgWord).offset(node as isize) as StgVolatilePtr,
                i,
                i.wrapping_add(n_numa_nodes as StgWord),
            ) == i
            {
                assignNurseryToCapability(cap, i as u32);

                return true;
            }
        } else if n_numa_nodes > 1 {
            let mut lost = false;
            n = 0;

            while n < n_numa_nodes {
                if !(n == node) {
                    i = next_nursery[n as usize];

                    if i < n_nurseries as StgWord {
                        if cas(
                            (&raw mut next_nursery as *mut StgWord).offset(n as isize)
                                as StgVolatilePtr,
                            i,
                            i.wrapping_add(n_numa_nodes as StgWord),
                        ) == i
                        {
                            assignNurseryToCapability(cap, i as u32);

                            return true;
                        } else {
                            lost = true;
                        }
                    }
                }

                n = n.wrapping_add(1);
            }

            if !lost {
                return false;
            }
        } else {
            return false;
        }
    }
}

unsafe fn move_STACK(mut src: *mut StgStack, mut dest: *mut StgStack) {
    let mut diff: ptrdiff_t = 0;
    diff = (dest as StgPtr).offset_from(src as StgPtr) as i64 as ptrdiff_t;
    (*dest).sp = (*dest).sp.offset(diff as isize);
}

unsafe fn accountAllocation(mut cap: *mut Capability, mut n: W_) {
    (*(*cap).r.rCCCS).mem_alloc = ((*(*cap).r.rCCCS).mem_alloc as u64).wrapping_add(
        n.wrapping_sub(
            (size_of::<StgProfHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as W_,
        ) as u64,
    ) as StgWord64 as StgWord64;

    if !(*cap).r.rCurrentTSO.is_null() {
        ASSIGN_Int64(
            &raw mut (*(*cap).r.rCurrentTSO).alloc_limit as *mut W_,
            (PK_Int64(&raw mut (*(*cap).r.rCurrentTSO).alloc_limit as *mut W_) as W_)
                .wrapping_sub(n.wrapping_mul(size_of::<W_>() as W_)) as StgInt64,
        );
    }
}

#[ffi(compiler, docs, ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocate(mut cap: *mut Capability, mut n: W_) -> StgPtr {
    let mut p = allocateMightFail(cap, n);

    if (p == null_mut::<c_void>() as StgPtr) as i32 as i64 != 0 {
        exitHeapOverflow();
    }

    return p;
}

unsafe fn allocateMightFail(mut cap: *mut Capability, mut n: W_) -> StgPtr {
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();

    if (n
        >= (((1 as u64) << 12 as i32)
            .wrapping_mul(8 as u64)
            .wrapping_div(10 as u64) as u32 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_) as i32 as i64
        != 0
    {
        let mut max_words: W_ = (HS_WORD_MAX as W_ & !BLOCK_SIZE.wrapping_sub(1 as u64) as W_)
            .wrapping_div(size_of::<W_>() as W_);

        let mut req_blocks: W_ = 0;

        if n > max_words {
            req_blocks = HS_WORD_MAX as W_;
        } else {
            req_blocks = (n
                .wrapping_mul(size_of::<W_>() as W_)
                .wrapping_add(BLOCK_SIZE as W_)
                .wrapping_sub(1 as W_)
                & !BLOCK_MASK as W_)
                .wrapping_div(BLOCK_SIZE as W_);
        }

        if RtsFlags.GcFlags.maxHeapSize > 0 && req_blocks >= RtsFlags.GcFlags.maxHeapSize as W_
            || req_blocks >= HS_INT32_MAX as W_
        {
            return null_mut::<StgWord>();
        }

        accountAllocation(cap, n);

        let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                1117,
                __r,
            );
        }

        bd = allocGroupOnNode((*cap).node, req_blocks);
        dbl_link_onto(bd, &raw mut (*g0).large_objects);
        (*g0).n_large_blocks = (*g0).n_large_blocks.wrapping_add((*bd).blocks as memcount);
        (*g0).n_new_large_words =
            ((*g0).n_new_large_words as StgWord).wrapping_add(n as StgWord) as memcount as memcount;

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                1122,
            );
        }

        initBdescr(bd, g0, g0);
        (&raw mut (*bd).flags).store(2, Ordering::Relaxed);
        (&raw mut (*bd).c2rust_unnamed.free)
            .store((*bd).start.offset(n as isize), Ordering::Relaxed);
        (*cap).total_allocated = (*cap).total_allocated.wrapping_add(n as u64);

        return (*bd).start;
    }

    accountAllocation(cap, n);
    bd = (*cap).r.rCurrentAlloc as *mut bdescr;

    if (bd.is_null()
        || (*bd).c2rust_unnamed.free.offset(n as isize)
            > (*bd).start.offset(
                ((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize) as isize,
            )) as i32 as i64
        != 0
    {
        if !bd.is_null() {
            finishedNurseryBlock(cap, bd);
        }

        bd = (*(*cap).r.rCurrentNursery).link as *mut bdescr;

        if bd.is_null() {
            let mut __r_0 = pthread_mutex_lock(&raw mut sm_mutex);

            if __r_0 != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/sm/Storage.c".as_ptr(),
                    1146,
                    __r_0,
                );
            }

            bd = allocBlockOnNode((*cap).node);
            (*(*cap).r.rNursery).n_blocks = (*(*cap).r.rNursery).n_blocks.wrapping_add(1);

            if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/sm/Storage.c".as_ptr(),
                    1149,
                );
            }

            initBdescr(bd, g0, g0);
            (*bd).flags = 0;
        } else {
            newNurseryBlock(bd);
            (*(*cap).r.rCurrentNursery).link = (*bd).link;

            if !(*bd).link.is_null() {
                (*(*bd).link).u.back = (*cap).r.rCurrentNursery as *mut bdescr_;
            }
        }

        dbl_link_onto(bd, &raw mut (*(*cap).r.rNursery).blocks);
        (*cap).r.rCurrentAlloc = bd as *mut bdescr_;

        if RtsFlags.DebugFlags.sanity {
            checkNurserySanity((*cap).r.rNursery as *mut nursery);
        }
    }

    p = (*bd).c2rust_unnamed.free;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(n as isize);

    if RtsFlags.DebugFlags.sanity {
        if (*(p as *mut StgWord8) as i32 == 0xaa) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Storage.c".as_ptr(), 1194);
        }
    }

    return p;
}

unsafe fn start_new_pinned_block(mut cap: *mut Capability) -> *mut bdescr {
    let mut bd = (*cap).pinned_object_block;

    if !bd.is_null() {
        finishedNurseryBlock(cap, bd);
        dbl_link_onto(bd, &raw mut (*cap).pinned_object_blocks);
    }

    bd = (*cap).pinned_object_empty;

    if bd.is_null() {
        let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                1242,
                __r,
            );
        }

        bd = allocNursery((*cap).node, null_mut::<bdescr>(), PINNED_EMPTY_SIZE);

        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/Storage.c".as_ptr(),
                1244,
            );
        }
    }

    let mut nbd = (*(*cap).r.rCurrentNursery).link as *mut bdescr;

    if !nbd.is_null() {
        newNurseryBlock(nbd);
        (*(*cap).r.rCurrentNursery).link = (*nbd).link;

        if !(*nbd).link.is_null() {
            (*(*nbd).link).u.back = (*cap).r.rCurrentNursery as *mut bdescr_;
        }

        dbl_link_onto(nbd, &raw mut (*(*cap).r.rNursery).blocks);

        if !(*cap).r.rCurrentAlloc.is_null() {
            finishedNurseryBlock(cap, (*cap).r.rCurrentAlloc as *mut bdescr);
        }

        (*cap).r.rCurrentAlloc = nbd as *mut bdescr_;
    }

    (*cap).pinned_object_empty = (*bd).link as *mut bdescr;
    newNurseryBlock(bd);

    if !(*bd).link.is_null() {
        (*(*bd).link).u.back = (*cap).pinned_object_empty as *mut bdescr_;
    }

    initBdescr(bd, g0, g0);
    (*cap).pinned_object_block = bd;
    (*bd).flags = (BF_PINNED | BF_LARGE | BF_EVACUATED) as StgWord16;

    return bd;
}

unsafe fn allocatePinned(
    mut cap: *mut Capability,
    mut n: W_,
    mut alignment: W_,
    mut align_off: W_,
) -> StgPtr {
    if (alignment != 0 && alignment & alignment.wrapping_sub(1 as W_) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 1312);
    }

    if (align_off & align_off.wrapping_sub(1 as W_) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 1313);
    }

    if (alignment >= size_of::<W_>() as W_) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Storage.c".as_ptr(), 1315);
    }

    let mut bd = (*cap).pinned_object_block;

    if bd.is_null() {
        bd = start_new_pinned_block(cap);
    }

    let alignment_w: StgWord = (alignment as StgWord).wrapping_div(size_of::<W_>() as StgWord);

    let mut off_w: W_ = ((((*bd).c2rust_unnamed.free as usize).wrapping_neg() as W_)
        .wrapping_sub(align_off)
        & alignment.wrapping_sub(1 as W_))
    .wrapping_div(size_of::<W_>() as W_);

    if n.wrapping_add(off_w)
        < (LARGE_OBJECT_THRESHOLD as usize).wrapping_div(size_of::<W_>() as usize) as W_
    {
        if (*bd)
            .c2rust_unnamed
            .free
            .offset(off_w as isize)
            .offset(n as isize)
            > (*bd).start.offset(BLOCK_SIZE_W as isize)
        {
            bd = start_new_pinned_block(cap);
            off_w = ((((*bd).c2rust_unnamed.free as usize).wrapping_neg() as W_)
                .wrapping_sub(align_off)
                & alignment.wrapping_sub(1 as W_))
            .wrapping_div(size_of::<W_>() as W_);
        }

        if n.wrapping_add(off_w)
            < (LARGE_OBJECT_THRESHOLD as usize).wrapping_div(size_of::<W_>() as usize) as W_
        {
            let mut p = (*bd).c2rust_unnamed.free;

            memset(
                p as *mut c_void,
                0,
                off_w.wrapping_mul(size_of::<W_>() as W_) as usize,
            );

            n = n.wrapping_add(off_w);
            p = p.offset(off_w as isize);
            (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(n as isize);

            if ((*bd).c2rust_unnamed.free
                <= (*bd).start.offset(((*bd).blocks as usize).wrapping_mul(
                    ((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize),
                ) as isize)) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Storage.c".as_ptr(), 1359);
            }

            accountAllocation(cap, n);

            return p;
        }
    }

    let mut p_0 = allocateMightFail(cap, n.wrapping_add(alignment_w as W_).wrapping_sub(1 as W_));

    if p_0.is_null() {
        return null_mut::<StgWord>();
    } else {
        let ref mut fresh6 = (*Bdescr(p_0)).flags;
        *fresh6 = (*fresh6 as i32 | BF_PINNED) as StgWord16;
        off_w = (((p_0 as usize).wrapping_neg() as W_).wrapping_sub(align_off)
            & alignment.wrapping_sub(1 as W_))
        .wrapping_div(size_of::<W_>() as W_);

        memset(
            p_0 as *mut c_void,
            0,
            off_w.wrapping_mul(size_of::<W_>() as W_) as usize,
        );

        p_0 = p_0.offset(off_w as isize);

        memset(
            p_0.offset(n as isize) as *mut c_void,
            0,
            alignment_w
                .wrapping_sub(off_w as StgWord)
                .wrapping_sub(1 as StgWord)
                .wrapping_mul(size_of::<W_>() as StgWord) as usize,
        );

        return p_0;
    };
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn dirty_MUT_VAR(
    mut reg: *mut StgRegTable,
    mut mvar: *mut StgMutVar,
    mut old: *mut StgClosure,
) {
    let mut cap = regTableToCapability(reg);
    SET_INFO_RELAXED(mvar as *mut StgClosure, &raw const stg_MUT_VAR_DIRTY_info);
    recordClosureMutated(cap, mvar as *mut StgClosure);

    if nonmoving_write_barrier_enabled as i64 != 0 {
        updateRemembSetPushClosure_(reg, old as *mut StgClosure_);
    }
}

unsafe fn dirty_TVAR(mut cap: *mut Capability, mut p: *mut StgTVar, mut old: *mut StgClosure) {
    if (&raw mut (*p).header.info).load(Ordering::Relaxed) == &raw const stg_TVAR_CLEAN_info {
        SET_INFO_RELAXED(p as *mut StgClosure, &raw const stg_TVAR_DIRTY_info);
        recordClosureMutated(cap, p as *mut StgClosure);

        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, old);
        }
    }
}

unsafe fn setTSOLink(mut cap: *mut Capability, mut tso: *mut StgTSO, mut target: *mut StgTSO) {
    if (&raw mut (*tso).dirty).load(Ordering::Relaxed) == 0 {
        (&raw mut (*tso).dirty).store(1, Ordering::Relaxed);
        recordClosureMutated(cap, tso as *mut StgClosure);

        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, (*tso)._link as *mut StgClosure);
        }
    }

    (*tso)._link = target as *mut StgTSO_;
}

unsafe fn setTSOPrev(mut cap: *mut Capability, mut tso: *mut StgTSO, mut target: *mut StgTSO) {
    if (&raw mut (*tso).dirty).load(Ordering::Relaxed) == 0 {
        (&raw mut (*tso).dirty).store(1, Ordering::Relaxed);
        recordClosureMutated(cap, tso as *mut StgClosure);

        if nonmoving_write_barrier_enabled as i64 != 0 {
            updateRemembSetPushClosure(cap, (*tso).block_info.prev as *mut StgClosure);
        }
    }

    (*tso).block_info.prev = target;
}

unsafe fn dirty_TSO(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if (&raw mut (*tso).dirty).load(Ordering::Relaxed) == 0 {
        (&raw mut (*tso).dirty).store(1, Ordering::Relaxed);
        recordClosureMutated(cap, tso as *mut StgClosure);
    }

    if nonmoving_write_barrier_enabled as i64 != 0 {
        updateRemembSetPushTSO(cap, tso);
    }
}

unsafe fn dirty_STACK(mut cap: *mut Capability, mut stack: *mut StgStack) {
    if nonmoving_write_barrier_enabled as i64 != 0 {
        updateRemembSetPushStack(cap, stack);
    }

    if (&raw mut (*stack).dirty).load(Ordering::Relaxed) as i32 == 0 {
        (&raw mut (*stack).dirty).store(1, Ordering::Relaxed);
        recordClosureMutated(cap, stack as *mut StgClosure);
    }
}

unsafe fn update_MVAR(
    mut reg: *mut StgRegTable,
    mut p: *mut StgClosure,
    mut old_val: *mut StgClosure,
) {
    let mut cap = regTableToCapability(reg);

    if nonmoving_write_barrier_enabled as i64 != 0 {
        let mut mvar = p as *mut StgMVar;
        updateRemembSetPushClosure(cap, old_val);
        updateRemembSetPushClosure(cap, (*mvar).head as *mut StgClosure);
        updateRemembSetPushClosure(cap, (*mvar).tail as *mut StgClosure);
    }
}

unsafe fn dirty_MVAR(
    mut reg: *mut StgRegTable,
    mut p: *mut StgClosure,
    mut old_val: *mut StgClosure,
) {
    let mut cap = regTableToCapability(reg);
    update_MVAR(reg, p, old_val);
    recordClosureMutated(cap, p);
}

unsafe fn calcTotalAllocated() -> u64 {
    let mut tot_alloc: u64 = 0;
    let mut n: W_ = 0;
    n = 0;

    while n < getNumCapabilities() as W_ {
        tot_alloc = tot_alloc.wrapping_add((*getCapability(n as u32)).total_allocated);

        traceEventHeapAllocated(
            getCapability(n as u32),
            CAPSET_HEAP_DEFAULT,
            ((*getCapability(n as u32)).total_allocated as W_).wrapping_mul(size_of::<W_>() as W_),
        );

        n = n.wrapping_add(1);
    }

    return tot_alloc;
}

unsafe fn updateNurseriesStats() {
    let mut i: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    i = 0;

    while i < getNumCapabilities() as u32 {
        bd = (*getCapability(i)).r.rCurrentNursery as *mut bdescr;

        if !bd.is_null() {
            finishedNurseryBlock(getCapability(i), bd);
        }

        bd = (*getCapability(i)).r.rCurrentAlloc as *mut bdescr;

        if !bd.is_null() {
            finishedNurseryBlock(getCapability(i), bd);
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn countOccupied(mut bd: *mut bdescr) -> StgWord {
    let mut words: W_ = 0;
    words = 0;

    while !bd.is_null() {
        if ((*bd).c2rust_unnamed.free
            <= (*bd).start.offset(
                ((*bd).blocks as usize).wrapping_mul(
                    ((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize),
                ) as isize,
            )) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Storage.c".as_ptr(), 1623);
        }

        words = words.wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as W_);

        bd = (*bd).link as *mut bdescr;
    }

    return words as StgWord;
}

unsafe fn genLiveWords(mut r#gen: *mut generation) -> StgWord {
    return genLiveCopiedWords(r#gen).wrapping_add(genLiveUncopiedWords(r#gen));
}

unsafe fn genLiveCopiedWords(mut r#gen: *mut generation) -> StgWord {
    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as i32 != 0 {
        return 0;
    } else {
        return if (*r#gen).live_estimate != 0 {
            (*r#gen).live_estimate as StgWord
        } else {
            (*r#gen).n_words as StgWord
        };
    };
}

unsafe fn genLiveUncopiedWords(mut r#gen: *mut generation) -> StgWord {
    let mut nonmoving_blocks: W_ = 0;

    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as i32 != 0 {
        nonmoving_blocks = (if (*r#gen).live_estimate != 0 {
            (*r#gen).live_estimate
        } else {
            (*r#gen).n_words
        })
        .wrapping_add(nonmoving_large_words)
        .wrapping_add(nonmoving_compact_words) as W_;
    }

    return ((*r#gen).n_large_words as StgWord)
        .wrapping_add(((*r#gen).n_compact_blocks as StgWord).wrapping_mul(BLOCK_SIZE_W as StgWord))
        .wrapping_add(nonmoving_blocks as StgWord);
}

unsafe fn genLiveCopiedBlocks(mut r#gen: *mut generation) -> StgWord {
    return (*r#gen).n_blocks as StgWord;
}

unsafe fn genLiveUncopiedBlocks(mut r#gen: *mut generation) -> StgWord {
    let mut nonmoving_blocks: W_ = 0;

    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as i32 != 0 {
        nonmoving_blocks = n_nonmoving_large_blocks
            .wrapping_add(n_nonmoving_marked_large_blocks)
            .wrapping_add(n_nonmoving_compact_blocks)
            .wrapping_add(n_nonmoving_marked_compact_blocks) as W_;
    }

    return ((*r#gen).n_large_blocks as StgWord)
        .wrapping_add((*r#gen).n_compact_blocks as StgWord)
        .wrapping_add(nonmoving_blocks as StgWord);
}

unsafe fn genLiveBlocks(mut r#gen: *mut generation) -> StgWord {
    return genLiveCopiedBlocks(r#gen).wrapping_add(genLiveUncopiedBlocks(r#gen));
}

unsafe fn gcThreadLiveWords(mut i: u32, mut g: u32) -> StgWord {
    let mut a: W_ = 0;
    let mut b: W_ = 0;
    let mut c: W_ = 0;

    a = countOccupied(
        (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
            .offset(g as isize))
        .0
        .todo_bd,
    ) as W_;

    b = (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
        .offset(g as isize))
    .0
    .n_part_words as W_;
    c = (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
        .offset(g as isize))
    .0
    .n_scavd_words as W_;

    return (a as StgWord)
        .wrapping_add(b as StgWord)
        .wrapping_add(c as StgWord);
}

unsafe fn gcThreadLiveBlocks(mut i: u32, mut g: u32) -> StgWord {
    let mut blocks: W_ = 0;

    blocks = countBlocks(
        (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
            .offset(g as isize))
        .0
        .todo_bd,
    );

    blocks = (blocks as StgWord).wrapping_add(
        (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
            .offset(g as isize))
        .0
        .n_part_blocks,
    ) as W_ as W_;

    blocks = (blocks as StgWord).wrapping_add(
        (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
            .offset(g as isize))
        .0
        .n_scavd_blocks,
    ) as W_ as W_;

    return blocks as StgWord;
}

unsafe fn calcNeeded(mut force_major: bool, mut blocks_needed: *mut memcount) -> StgWord {
    let mut needed: W_ = 0;
    let mut N_0: u32 = 0;

    if force_major {
        N_0 = RtsFlags.GcFlags.generations.wrapping_sub(1 as u32);
    } else {
        N_0 = 0;
    }

    let mut g: u32 = 0;

    while g < RtsFlags.GcFlags.generations {
        let mut r#gen: *mut generation = generations.offset(g as isize) as *mut generation;

        let mut blocks: W_ = if (*r#gen).live_estimate != 0 {
            ((*r#gen).live_estimate as W_).wrapping_div(BLOCK_SIZE_W as W_)
        } else {
            (*r#gen).n_blocks as W_
        };

        blocks = (blocks as StgWord).wrapping_add(
            (&raw mut (*r#gen).n_large_blocks)
                .load(Ordering::Relaxed)
                .wrapping_add((&raw mut (*r#gen).n_compact_blocks).load(Ordering::Relaxed))
                as StgWord,
        ) as W_ as W_;

        needed = needed.wrapping_add(blocks);

        if g == 0 || blocks > (*r#gen).max_blocks {
            N_0 = ({
                let mut _a: u32 = N_0 as u32;
                let mut _b: u32 = g as u32;

                if _a <= _b { _b } else { _a as u32 }
            });

            if (*r#gen).mark != 0 {
                needed = (needed as StgWord).wrapping_add((*r#gen).n_blocks.wrapping_div(
                    (BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize) as memcount,
                ) as StgWord) as W_ as W_;

                needed = (needed as StgWord)
                    .wrapping_add((*r#gen).n_blocks.wrapping_div(100 as memcount) as StgWord)
                    as W_ as W_;
            }

            if !((*r#gen).compact != 0
                || RtsFlags.GcFlags.useNonmoving as i32 != 0 && r#gen == oldest_gen)
            {
                needed = (needed as StgWord).wrapping_add((*r#gen).n_blocks as StgWord) as W_ as W_;
            }
        }

        g = g.wrapping_add(1);
    }

    if !blocks_needed.is_null() {
        *blocks_needed = needed as memcount;
    }

    return N_0 as StgWord;
}

unsafe fn calcTotalLargeObjectsW() -> StgWord {
    let mut g: u32 = 0;
    let mut totalW: StgWord = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        totalW = totalW.wrapping_add((*generations.offset(g as isize)).n_large_words as StgWord);
        g = g.wrapping_add(1);
    }

    totalW = totalW.wrapping_add(nonmoving_large_words as StgWord);

    return totalW;
}

unsafe fn calcTotalCompactW() -> StgWord {
    let mut g: u32 = 0;
    let mut totalW: StgWord = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        totalW = totalW.wrapping_add(
            (*generations.offset(g as isize))
                .n_compact_blocks
                .wrapping_mul(BLOCK_SIZE_W as memcount) as StgWord,
        );

        g = g.wrapping_add(1);
    }

    totalW = totalW.wrapping_add(nonmoving_compact_words as StgWord);

    return totalW;
}

unsafe fn flushExec(mut len: W_, mut exec_addr: AdjustorExecutable) {
    sys_icache_invalidate(exec_addr as *mut c_void, len as usize);
}

unsafe fn _bdescr(mut p: StgPtr) -> *mut bdescr {
    return Bdescr(p);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_clearMemory() {
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            1957,
            __r,
        );
    }

    clear_free_list();

    let mut i: u32 = 0;

    while i < n_nurseries {
        let mut bd = (*nurseries.offset(i as isize)).blocks;

        while !bd.is_null() {
            clear_blocks(bd);
            bd = (*bd).link as *mut bdescr;
        }

        i = i.wrapping_add(1);
    }

    let mut i_0 = 0;

    while i_0 < getNumCapabilities() {
        let mut bd_0 = (*getCapability(i_0 as u32)).pinned_object_empty;

        while !bd_0.is_null() {
            clear_blocks(bd_0);
            bd_0 = (*bd_0).link as *mut bdescr;
        }

        let mut bd_1 = (**gc_threads.offset(i_0 as isize)).free_blocks;

        while !bd_1.is_null() {
            clear_blocks(bd_1);
            bd_1 = (*bd_1).link as *mut bdescr;
        }

        i_0 = i_0.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        let mut seg = nonmovingHeap.free;

        while !seg.is_null() {
            nonmovingClearSegment(seg);
            seg = (*seg).link;
        }

        let mut i_1 = 0;

        while i_1 < nonmoving_alloca_cnt as i32 {
            let mut alloc: *mut NonmovingAllocator =
                nonmovingHeap.allocators.offset(i_1 as isize) as *mut NonmovingAllocator;

            let mut seg_0 = (*alloc).active;

            while !seg_0.is_null() {
                nonmovingClearSegmentFreeBlocks(seg_0);
                seg_0 = (*seg_0).link;
            }

            let mut j = 0;

            while j < getNumCapabilities() {
                let mut cap = getCapability(j as u32);

                nonmovingClearSegmentFreeBlocks(*(*cap).current_segments.offset(i_1 as isize));

                j = j.wrapping_add(1);
            }

            i_1 += 1;
        }
    }

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/Storage.c".as_ptr(),
            1997,
        );
    }
}
