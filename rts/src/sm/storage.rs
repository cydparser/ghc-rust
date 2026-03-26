use crate::capability::{
    getCapability, n_numa_nodes, recordClosureMutated, recordMutableCap, regTableToCapability,
};
use crate::eventlog::event_log::postInitEvent;
use crate::ffi::hs_ffi::{HS_INT32_MAX, HS_WORD_MAX};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::errorBelch;
use crate::ffi::rts::storage::block::{
    BF_EVACUATED, BF_LARGE, BF_PINNED, BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK,
    Bdescr, LARGE_OBJECT_THRESHOLD, MBLOCK_SIZE, allocBlockOnNode, allocGroupOnNode, bdescr,
    bdescr_, dbl_link_onto, freeGroup, initBlockAllocator,
};
use crate::ffi::rts::storage::block::{BLOCK_SIZE, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    INFO_PTR_TO_STRUCT, SET_INFO_RELAXED, SET_INFO_RELEASE,
};
use crate::ffi::rts::storage::closures::{StgInd, StgIndStatic, StgMutVar, StgTVar};
use crate::ffi::rts::storage::gc::{
    AdjustorExecutable, ListBlocksCb, generation, generation_, initBdescr, memcount, nursery,
    nursery_,
};
use crate::ffi::rts::storage::m_block::{freeAllMBlocks, initMBlocks};
use crate::ffi::rts::storage::tso::{StgStack, StgTSO_};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts::{_assertFail, exitHeapOverflow};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    stg_CAF_BLACKHOLE_info, stg_END_TSO_QUEUE_closure, stg_IND_STATIC_info, stg_MUT_VAR_DIRTY_info,
    stg_TVAR_CLEAN_info, stg_TVAR_DIRTY_info,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::{
    StgInt64, StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord16, StgWord32,
};
use crate::ffi::stg::types::{StgPtr, StgWord32};
use crate::ffi::stg::{ASSIGN_Int64, BITS_PER_BYTE, PK_Int64, W_};
use crate::prelude::*;
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
    nonmoving_large_objects, nonmoving_large_words,
};
use crate::sm::storage::{
    END_OF_CAF_LIST, STATIC_FLAG_LIST, clear_blocks, finishedNurseryBlock, newNurseryBlock,
};
use crate::stats::stat_exitReport;
use crate::trace::{
    CAPSET_HEAP_DEFAULT, DEBUG_RTS, trace_, traceEventHeapAllocated, traceEventHeapInfo,
};

#[cfg(test)]
mod tests;

#[inline]
pub(crate) unsafe fn doYouWantToGC(mut cap: *mut Capability) -> bool {
    return (*(*cap).r.rCurrentNursery).link.is_null() && !getNewNursery(cap)
        || (*g0).n_new_large_words >= large_alloc_lim;
}

#[inline]
pub(crate) unsafe fn finishedNurseryBlock(mut cap: *mut Capability, mut bd: *mut bdescr) {
    (*cap).total_allocated = (*cap)
        .total_allocated
        .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as uint64_t);
}

#[inline]
pub(crate) unsafe fn newNurseryBlock(mut bd: *mut bdescr) {
    (*bd).c2rust_unnamed.free = (*bd).start;
}

pub(crate) const STATIC_FLAG_LIST: c_int = 3 as c_int;

pub(crate) const END_OF_CAF_LIST: *mut StgClosure = STATIC_FLAG_LIST as *mut StgClosure;

#[inline]
pub(crate) unsafe fn clear_blocks(mut bd: *mut bdescr) {
    memset(
        (*bd).start as *mut c_void,
        if RtsFlags.DebugFlags.zero_on_gc as c_int != 0 {
            0xaa as c_int
        } else {
            0 as c_int
        },
        (BLOCK_SIZE as size_t).wrapping_mul((*bd).blocks as size_t),
    );
}

static mut dyn_caf_list: *mut StgIndStatic = null::<StgIndStatic>() as *mut StgIndStatic;

static mut debug_caf_list: *mut StgIndStatic = null::<StgIndStatic>() as *mut StgIndStatic;

static mut revertible_caf_list: *mut StgIndStatic = null::<StgIndStatic>() as *mut StgIndStatic;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = false;

static mut highMemDynamic: bool = false;

static mut large_alloc_lim: W_ = 0;

static mut exec_block: *mut bdescr = null::<bdescr>() as *mut bdescr;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static mut generations: *mut generation = null::<generation>() as *mut generation;

#[ffi(compiler, ghc_lib, utils)]
#[unsafe(no_mangle)]
pub static mut g0: *mut generation = null::<generation>() as *mut generation;

static mut oldest_gen: *mut generation = null::<generation>() as *mut generation;

static mut nurseries: *mut nursery = null::<nursery>() as *mut nursery;

static mut n_nurseries: uint32_t = 0;

const PINNED_EMPTY_SIZE: W_ = BLOCKS_PER_MBLOCK;

static mut next_nursery: [StgWord; 16] = [0; 16];

unsafe fn initGeneration(mut r#gen: *mut generation, mut g: c_int) {
    (*r#gen).no = g as uint32_t;
    (*r#gen).collections = 0 as uint32_t;
    (*r#gen).par_collections = 0 as uint32_t;
    (*r#gen).failed_promotions = 0 as uint32_t;
    (*r#gen).max_blocks = 0 as memcount;
    (*r#gen).blocks = null_mut::<bdescr>();
    (*r#gen).n_blocks = 0 as memcount;
    (*r#gen).n_words = 0 as memcount;
    (*r#gen).live_estimate = 0 as memcount;
    (*r#gen).old_blocks = null_mut::<bdescr>();
    (*r#gen).n_old_blocks = 0 as memcount;
    (*r#gen).large_objects = null_mut::<bdescr>();
    (*r#gen).n_large_blocks = 0 as memcount;
    (*r#gen).n_large_words = 0 as memcount;
    (*r#gen).n_new_large_words = 0 as memcount;
    (*r#gen).compact_objects = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks = 0 as memcount;
    (*r#gen).compact_blocks_in_import = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks_in_import = 0 as memcount;
    (*r#gen).scavenged_large_objects = null_mut::<bdescr>();
    (*r#gen).n_scavenged_large_blocks = 0 as memcount;
    (*r#gen).live_compact_objects = null_mut::<bdescr>();
    (*r#gen).n_live_compact_blocks = 0 as memcount;
    (*r#gen).compact_blocks_in_import = null_mut::<bdescr>();
    (*r#gen).n_compact_blocks_in_import = 0 as memcount;
    (*r#gen).mark = 0 as c_int;
    (*r#gen).compact = 0 as c_int;
    (*r#gen).bitmap = null_mut::<bdescr>();
    (*r#gen).threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*r#gen).old_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*r#gen).weak_ptr_list = null_mut::<StgWeak>();
    (*r#gen).old_weak_ptr_list = null_mut::<StgWeak>();
}

unsafe fn traceHeapInfo() {
    traceEventHeapInfo(
        CAPSET_HEAP_DEFAULT,
        RtsFlags.GcFlags.generations,
        (RtsFlags.GcFlags.maxHeapSize as c_ulong).wrapping_mul(BLOCK_SIZE) as W_,
        (RtsFlags.GcFlags.minAllocAreaSize as c_ulong).wrapping_mul(BLOCK_SIZE) as W_,
        MBLOCK_SIZE as W_,
        BLOCK_SIZE as W_,
    );
}

unsafe fn initStorage() {
    let mut g: uint32_t = 0;
    let mut n: uint32_t = 0;

    if !generations.is_null() {
        return;
    }

    initMBlocks();
    initBlockAllocator();

    generations = stgMallocBytes(
        (RtsFlags.GcFlags.generations as size_t).wrapping_mul(size_of::<generation_>() as size_t),
        b"initStorage: gens\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut generation;

    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        initGeneration(
            generations.offset(g as isize) as *mut generation,
            g as c_int,
        );

        g = g.wrapping_add(1);
    }

    g0 = generations.offset(0 as c_int as isize) as *mut generation;
    oldest_gen = generations
        .offset(RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) as isize)
        as *mut generation;
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) {
        let ref mut fresh12 = (*generations.offset(g as isize)).to;
        *fresh12 = generations.offset(g.wrapping_add(1 as uint32_t) as isize) as *mut generation
            as *mut generation_;
        g = g.wrapping_add(1);
    }

    (*oldest_gen).to = oldest_gen as *mut generation_;
    nonmovingInit();

    if RtsFlags.GcFlags.compact as c_int != 0 || RtsFlags.GcFlags.sweep as c_int != 0 {
        if RtsFlags.GcFlags.generations == 1 as uint32_t {
            errorBelch(
                b"WARNING: compact/sweep is incompatible with -G1; disabled\0" as *const u8
                    as *const c_char,
            );
        } else {
            (*oldest_gen).mark = 1 as c_int;

            if RtsFlags.GcFlags.compact {
                (*oldest_gen).compact = 1 as c_int;
            }
        }
    }

    (*generations.offset(0 as c_int as isize)).max_blocks = 0 as memcount;
    dyn_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
    debug_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
    revertible_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;

    if RtsFlags.GcFlags.largeAllocLim > 0 as uint32_t {
        large_alloc_lim =
            (RtsFlags.GcFlags.largeAllocLim as usize).wrapping_mul(BLOCK_SIZE_W) as W_;
    } else {
        large_alloc_lim =
            (RtsFlags.GcFlags.minAllocAreaSize as usize).wrapping_mul(BLOCK_SIZE_W) as W_;
    }

    exec_block = null_mut::<bdescr>();
    N = 0 as uint32_t;
    n = 0 as uint32_t;

    while n < n_numa_nodes {
        write_volatile(&mut next_nursery[n as usize] as *mut StgWord, n as StgWord);
        n = n.wrapping_add(1);
    }

    storageAddCapabilities(0 as uint32_t, getNumCapabilities() as uint32_t);
    postInitEvent(Some(traceHeapInfo as unsafe extern "C" fn() -> ()));
}

unsafe fn storageAddCapabilities(mut from: uint32_t, mut to: uint32_t) {
    let mut n: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut new_n_nurseries: uint32_t = 0;
    let mut old_nurseries = null_mut::<nursery>();

    if RtsFlags.GcFlags.nurseryChunkSize == 0 as uint32_t {
        new_n_nurseries = to;
    } else {
        let mut total_alloc: memcount =
            to.wrapping_mul(RtsFlags.GcFlags.minAllocAreaSize) as memcount;

        new_n_nurseries = ({
            let mut _a: uint32_t = to as uint32_t;
            let mut _b: uint32_t =
                total_alloc.wrapping_div(RtsFlags.GcFlags.nurseryChunkSize as memcount) as uint32_t;

            if _a <= _b { _b } else { _a as uint32_t }
        });
    }

    old_nurseries = nurseries;

    if from > 0 as uint32_t {
        nurseries = stgReallocBytes(
            nurseries as *mut c_void,
            (new_n_nurseries as size_t).wrapping_mul(size_of::<nursery_>() as size_t),
            b"storageAddCapabilities\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut nursery;
    } else {
        nurseries = stgMallocBytes(
            (new_n_nurseries as size_t).wrapping_mul(size_of::<nursery_>() as size_t),
            b"storageAddCapabilities\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut nursery;
    }

    i = 0 as uint32_t;

    while i < from {
        let mut index: uint32_t =
            (*getCapability(i)).r.rNursery.offset_from(old_nurseries) as c_long as uint32_t;

        let ref mut fresh13 = (*getCapability(i)).r.rNursery;
        *fresh13 = nurseries.offset(index as isize) as *mut nursery as *mut nursery_;
        i = i.wrapping_add(1);
    }

    allocNurseries(n_nurseries, new_n_nurseries);
    n_nurseries = new_n_nurseries;
    assignNurseriesToCapabilities(from, to);
    n = from;

    while n < to {
        g = 1 as uint32_t;

        while g < RtsFlags.GcFlags.generations {
            let ref mut fresh14 = *(*getCapability(n)).mut_lists.offset(g as isize);
            *fresh14 = allocBlockOnNode(n.wrapping_rem(n_numa_nodes));
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
    let mut g: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut s: uint32_t = 0;
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        i = 0 as uint32_t;

        while i < getNumCapabilities() as uint32_t {
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

    i = 0 as uint32_t;

    while i < n_nurseries {
        cb.expect("non-null function pointer")(user, (*nurseries.offset(i as isize)).blocks);
        i = i.wrapping_add(1);
    }

    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
        if !(*getCapability(i)).pinned_object_block.is_null() {
            cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_block);
        }

        cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_blocks);
        cb.expect("non-null function pointer")(user, (*getCapability(i)).pinned_object_empty);

        if RtsFlags.GcFlags.useNonmoving {
            s = 0 as uint32_t;

            while s < nonmoving_alloca_cnt as uint32_t {
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
        s = 0 as uint32_t;

        while s < nonmoving_alloca_cnt as uint32_t {
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
    orig_info = (*caf).header.info;

    let mut orig_info_tbl: *const StgInfoTable = INFO_PTR_TO_STRUCT(orig_info);
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
    (*bh).header.info = &raw const stg_CAF_BLACKHOLE_info;
    (*caf).indirectee = bh as *mut StgClosure;
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

    if keepCAFs as c_int != 0
        && !(highMemDynamic as c_int != 0
            && caf as *mut c_void > 0x80000000 as c_uint as *mut c_void)
    {
        (*caf).static_link = dyn_caf_list as *mut StgClosure;
        dyn_caf_list = (caf as StgWord | STATIC_FLAG_LIST as StgWord) as *mut StgIndStatic;
    } else if (*oldest_gen).no != 0 as uint32_t && !RtsFlags.GcFlags.useNonmoving {
        recordMutableCap(
            caf as *mut StgClosure,
            regTableToCapability(reg),
            (*oldest_gen).no,
        );
    }

    return bh;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    keepCAFs = 1 as c_int != 0;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    highMemDynamic = 1 as c_int != 0;
}

unsafe fn newRetainedCAF(mut reg: *mut StgRegTable, mut caf: *mut StgIndStatic) -> *mut StgInd {
    let mut bh = null_mut::<StgInd>();
    bh = lockCAF(reg, caf);

    if bh.is_null() {
        return null_mut::<StgInd>();
    }

    (*caf).static_link = revertible_caf_list as *mut StgClosure;
    revertible_caf_list = (caf as StgWord | STATIC_FLAG_LIST as StgWord) as *mut StgIndStatic;

    return bh;
}

unsafe fn newGCdCAF(mut reg: *mut StgRegTable, mut caf: *mut StgIndStatic) -> *mut StgInd {
    let mut bh = null_mut::<StgInd>();
    bh = lockCAF(reg, caf);

    if bh.is_null() {
        return null_mut::<StgInd>();
    }

    if (*oldest_gen).no != 0 as uint32_t && !RtsFlags.GcFlags.useNonmoving {
        recordMutableCap(
            caf as *mut StgClosure,
            regTableToCapability(reg),
            (*oldest_gen).no,
        );
    }

    return bh;
}

unsafe fn allocNursery(mut node: uint32_t, mut tail: *mut bdescr, mut blocks: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut i: W_ = 0;
    let mut n: W_ = 0;

    while blocks > 0 as W_ {
        n = ({
            let mut _a: W_ = (((1 as c_ulong) << 20 as c_int) as W_)
                .wrapping_sub(
                    ((0x40 as c_ulong).wrapping_mul(
                        ((1 as c_ulong) << 20 as c_int).wrapping_div((1 as c_ulong) << 12 as c_int),
                    ) as W_)
                        .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong) as W_,
                )
                .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_);

            let mut _b: W_ = blocks as W_;

            if _a <= _b { _a } else { _b as W_ }
        });

        bd = allocLargeChunkOnNode(node, 1 as W_, n);
        n = (*bd).blocks as W_;
        blocks = blocks.wrapping_sub(n);
        i = 0 as W_;

        while i < n {
            initBdescr(bd.offset(i as isize) as *mut bdescr, g0, g0);
            (*bd.offset(i as isize)).blocks = 1 as StgWord32;
            (*bd.offset(i as isize)).flags = 0 as StgWord16;

            if i > 0 as W_ {
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

        tail = bd.offset(0 as c_int as isize) as *mut bdescr;
    }

    return bd.offset(0 as c_int as isize) as *mut bdescr;
}

#[inline]
unsafe fn assignNurseryToCapability(mut cap: *mut Capability, mut n: uint32_t) {
    (*cap).r.rNursery = nurseries.offset(n as isize) as *mut nursery as *mut nursery_;
    (*cap).r.rCurrentNursery = (*nurseries.offset(n as isize)).blocks as *mut bdescr_;
    newNurseryBlock((*nurseries.offset(n as isize)).blocks);
    (*cap).r.rCurrentAlloc = null_mut::<bdescr_>();
}

unsafe fn assignNurseriesToCapabilities(mut from: uint32_t, mut to: uint32_t) {
    let mut i: uint32_t = 0;
    let mut node: uint32_t = 0;
    i = from;

    while i < to {
        node = (*getCapability(i)).node;
        assignNurseryToCapability(getCapability(i), next_nursery[node as usize] as uint32_t);

        write_volatile(
            &mut next_nursery[node as usize] as *mut StgWord,
            read_volatile::<StgWord>(&next_nursery[node as usize] as *const StgWord)
                .wrapping_add(n_numa_nodes as StgWord),
        );

        i = i.wrapping_add(1);
    }
}

unsafe fn allocNurseries(mut from: uint32_t, mut to: uint32_t) {
    let mut i: uint32_t = 0;
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
    let mut n: uint32_t = 0;
    n = 0 as uint32_t;

    while n < n_numa_nodes {
        write_volatile(&mut next_nursery[n as usize] as *mut StgWord, n as StgWord);
        n = n.wrapping_add(1);
    }

    assignNurseriesToCapabilities(0 as uint32_t, getNumCapabilities() as uint32_t);
}

unsafe fn countNurseryBlocks() -> StgWord {
    let mut i: uint32_t = 0;
    let mut blocks: W_ = 0 as W_;
    i = 0 as uint32_t;

    while i < n_nurseries {
        blocks = (blocks as StgWord)
            .wrapping_add((*nurseries.offset(i as isize)).n_blocks as StgWord)
            as W_ as W_;
        i = i.wrapping_add(1);
    }

    return blocks as StgWord;
}

unsafe fn resizeNurseriesEach(mut blocks: W_) {
    let mut i: uint32_t = 0;
    let mut node: uint32_t = 0;
    let mut bd = null_mut::<bdescr>();
    let mut nursery_blocks: W_ = 0;
    let mut nursery = null_mut::<nursery>();
    i = 0 as uint32_t;

    while i < n_nurseries {
        nursery = nurseries.offset(i as isize) as *mut nursery;
        nursery_blocks = (*nursery).n_blocks as W_;

        if !(nursery_blocks == blocks) {
            node = i.wrapping_rem(n_numa_nodes);

            if nursery_blocks < blocks {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
                    trace_(
                        b"increasing size of nursery from %d to %d blocks\0" as *const u8
                            as *const c_char as *mut c_char,
                        nursery_blocks,
                        blocks,
                    );
                }

                (*nursery).blocks =
                    allocNursery(node, (*nursery).blocks, blocks.wrapping_sub(nursery_blocks));
            } else {
                let mut next_bd = null_mut::<bdescr>();

                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
                    trace_(
                        b"decreasing size of nursery from %d to %d blocks\0" as *const u8
                            as *const c_char as *mut c_char,
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
                    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
                        trace_(
                            b"reincreasing size of nursery from %d to %d blocks\0" as *const u8
                                as *const c_char as *mut c_char,
                            nursery_blocks,
                            blocks,
                        );
                    }

                    (*nursery).blocks =
                        allocNursery(node, (*nursery).blocks, blocks.wrapping_sub(nursery_blocks));
                }
            }

            (*nursery).n_blocks = blocks as memcount;
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn resizeNurseriesFixed() {
    let mut blocks: uint32_t = 0;

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
    let mut node: uint32_t = (*cap).node;
    let mut n: uint32_t = 0;

    loop {
        i = next_nursery[node as usize];

        if i < n_nurseries as StgWord {
            if cas(
                (&raw mut next_nursery as *mut StgWord).offset(node as isize) as StgVolatilePtr,
                i,
                i.wrapping_add(n_numa_nodes as StgWord),
            ) == i
            {
                assignNurseryToCapability(cap, i as uint32_t);

                return r#true != 0;
            }
        } else if n_numa_nodes > 1 as uint32_t {
            let mut lost = r#false != 0;
            n = 0 as uint32_t;

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
                            assignNurseryToCapability(cap, i as uint32_t);

                            return r#true != 0;
                        } else {
                            lost = r#true != 0;
                        }
                    }
                }

                n = n.wrapping_add(1);
            }

            if !lost {
                return r#false != 0;
            }
        } else {
            return r#false != 0;
        }
    }
}

unsafe fn move_STACK(mut src: *mut StgStack, mut dest: *mut StgStack) {
    let mut diff: ptrdiff_t = 0;
    diff = (dest as StgPtr).offset_from(src as StgPtr) as c_long as ptrdiff_t;
    (*dest).sp = (*dest).sp.offset(diff as isize);
}

unsafe fn accountAllocation(mut cap: *mut Capability, mut n: W_) {
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

    if (p == null_mut::<c_void>() as StgPtr) as c_int as c_long != 0 {
        exitHeapOverflow();
    }

    return p;
}

unsafe fn allocateMightFail(mut cap: *mut Capability, mut n: W_) -> StgPtr {
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();

    if (n
        >= (((1 as c_ulong) << 12 as c_int)
            .wrapping_mul(8 as c_ulong)
            .wrapping_div(10 as c_ulong) as uint32_t as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_) as c_int as c_long
        != 0
    {
        let mut max_words: W_ = (HS_WORD_MAX as W_ & !BLOCK_SIZE.wrapping_sub(1 as c_ulong) as W_)
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

        if RtsFlags.GcFlags.maxHeapSize > 0 as uint32_t
            && req_blocks >= RtsFlags.GcFlags.maxHeapSize as W_
            || req_blocks >= HS_INT32_MAX as W_
        {
            return null_mut::<StgWord>();
        }

        accountAllocation(cap, n);
        bd = allocGroupOnNode((*cap).node, req_blocks);
        dbl_link_onto(bd, &raw mut (*g0).large_objects);
        (*g0).n_large_blocks = (*g0).n_large_blocks.wrapping_add((*bd).blocks as memcount);
        (*g0).n_new_large_words =
            ((*g0).n_new_large_words as StgWord).wrapping_add(n as StgWord) as memcount as memcount;
        initBdescr(bd, g0, g0);
        (*bd).flags = 2 as StgWord16;
        (*bd).c2rust_unnamed.free = (*bd).start.offset(n as isize);
        (*cap).total_allocated = (*cap).total_allocated.wrapping_add(n as uint64_t);

        return (*bd).start;
    }

    accountAllocation(cap, n);
    bd = (*cap).r.rCurrentAlloc as *mut bdescr;

    if (bd.is_null()
        || (*bd).c2rust_unnamed.free.offset(n as isize)
            > (*bd).start.offset(
                ((1 as usize) << 12 as c_int).wrapping_div(size_of::<W_>() as usize) as isize,
            )) as c_int as c_long
        != 0
    {
        if !bd.is_null() {
            finishedNurseryBlock(cap, bd);
        }

        bd = (*(*cap).r.rCurrentNursery).link as *mut bdescr;

        if bd.is_null() {
            bd = allocBlockOnNode((*cap).node);
            (*(*cap).r.rNursery).n_blocks = (*(*cap).r.rNursery).n_blocks.wrapping_add(1);
            initBdescr(bd, g0, g0);
            (*bd).flags = 0 as StgWord16;
        } else {
            newNurseryBlock(bd);
            (*(*cap).r.rCurrentNursery).link = (*bd).link;

            if !(*bd).link.is_null() {
                (*(*bd).link).u.back = (*cap).r.rCurrentNursery as *mut bdescr_;
            }
        }

        dbl_link_onto(bd, &raw mut (*(*cap).r.rNursery).blocks);
        (*cap).r.rCurrentAlloc = bd as *mut bdescr_;
    }

    p = (*bd).c2rust_unnamed.free;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(n as isize);

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
        bd = allocNursery((*cap).node, null_mut::<bdescr>(), PINNED_EMPTY_SIZE);
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
    if (alignment != 0 && alignment & alignment.wrapping_sub(1 as W_) == 0) as c_int as c_long != 0
    {
    } else {
        _assertFail(
            b"rts/sm/Storage.c\0" as *const u8 as *const c_char,
            1312 as c_uint,
        );
    }

    if (align_off & align_off.wrapping_sub(1 as W_) == 0) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/sm/Storage.c\0" as *const u8 as *const c_char,
            1313 as c_uint,
        );
    }

    if (alignment >= size_of::<W_>() as W_) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/sm/Storage.c\0" as *const u8 as *const c_char,
            1315 as c_uint,
        );
    }

    let mut bd = (*cap).pinned_object_block;

    if bd.is_null() {
        bd = start_new_pinned_block(cap);
    }

    let alignment_w: StgWord = (alignment as StgWord).wrapping_div(size_of::<W_>() as StgWord);
    let mut off_w: W_ = ((((*bd).c2rust_unnamed.free as uintptr_t).wrapping_neg() as W_)
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
            off_w = ((((*bd).c2rust_unnamed.free as uintptr_t).wrapping_neg() as W_)
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
                0 as c_int,
                off_w.wrapping_mul(size_of::<W_>() as W_) as size_t,
            );

            n = n.wrapping_add(off_w);
            p = p.offset(off_w as isize);
            (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(n as isize);
            accountAllocation(cap, n);

            return p;
        }
    }

    let mut p_0 = allocateMightFail(cap, n.wrapping_add(alignment_w as W_).wrapping_sub(1 as W_));

    if p_0.is_null() {
        return null_mut::<StgWord>();
    } else {
        let ref mut fresh6 = (*Bdescr(p_0)).flags;
        *fresh6 = (*fresh6 as c_int | BF_PINNED) as StgWord16;
        off_w = (((p_0 as uintptr_t).wrapping_neg() as W_).wrapping_sub(align_off)
            & alignment.wrapping_sub(1 as W_))
        .wrapping_div(size_of::<W_>() as W_);

        memset(
            p_0 as *mut c_void,
            0 as c_int,
            off_w.wrapping_mul(size_of::<W_>() as W_) as size_t,
        );

        p_0 = p_0.offset(off_w as isize);

        memset(
            p_0.offset(n as isize) as *mut c_void,
            0 as c_int,
            alignment_w
                .wrapping_sub(off_w as StgWord)
                .wrapping_sub(1 as StgWord)
                .wrapping_mul(size_of::<W_>() as StgWord) as size_t,
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
}

unsafe fn dirty_TVAR(mut cap: *mut Capability, mut p: *mut StgTVar, mut old: *mut StgClosure) {
    if (*p).header.info == &raw const stg_TVAR_CLEAN_info {
        SET_INFO_RELAXED(p as *mut StgClosure, &raw const stg_TVAR_DIRTY_info);
        recordClosureMutated(cap, p as *mut StgClosure);
    }
}

unsafe fn setTSOLink(mut cap: *mut Capability, mut tso: *mut StgTSO, mut target: *mut StgTSO) {
    if (*tso).dirty == 0 as StgWord32 {
        (*tso).dirty = 1 as StgWord32;
        recordClosureMutated(cap, tso as *mut StgClosure);
    }

    (*tso)._link = target as *mut StgTSO_;
}

unsafe fn setTSOPrev(mut cap: *mut Capability, mut tso: *mut StgTSO, mut target: *mut StgTSO) {
    if (*tso).dirty == 0 as StgWord32 {
        (*tso).dirty = 1 as StgWord32;
        recordClosureMutated(cap, tso as *mut StgClosure);
    }

    (*tso).block_info.prev = target;
}

unsafe fn dirty_TSO(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if (*tso).dirty == 0 as StgWord32 {
        (*tso).dirty = 1 as StgWord32;
        recordClosureMutated(cap, tso as *mut StgClosure);
    }
}

unsafe fn dirty_STACK(mut cap: *mut Capability, mut stack: *mut StgStack) {
    if (*stack).dirty as c_int == 0 as c_int {
        (*stack).dirty = 1 as StgWord8;
        recordClosureMutated(cap, stack as *mut StgClosure);
    }
}

unsafe fn update_MVAR(
    mut reg: *mut StgRegTable,
    mut p: *mut StgClosure,
    mut old_val: *mut StgClosure,
) {
    let mut cap = regTableToCapability(reg);
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

unsafe fn calcTotalAllocated() -> uint64_t {
    let mut tot_alloc: uint64_t = 0 as uint64_t;
    let mut n: W_ = 0;
    n = 0 as W_;

    while n < getNumCapabilities() as W_ {
        tot_alloc = tot_alloc.wrapping_add((*getCapability(n as uint32_t)).total_allocated);

        traceEventHeapAllocated(
            getCapability(n as uint32_t),
            CAPSET_HEAP_DEFAULT,
            ((*getCapability(n as uint32_t)).total_allocated as W_)
                .wrapping_mul(size_of::<W_>() as W_),
        );

        n = n.wrapping_add(1);
    }

    return tot_alloc;
}

unsafe fn updateNurseriesStats() {
    let mut i: uint32_t = 0;
    let mut bd = null_mut::<bdescr>();
    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
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
    words = 0 as W_;

    while !bd.is_null() {
        words =
            words.wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as W_);
        bd = (*bd).link as *mut bdescr;
    }

    return words as StgWord;
}

unsafe fn genLiveWords(mut r#gen: *mut generation) -> StgWord {
    return genLiveCopiedWords(r#gen).wrapping_add(genLiveUncopiedWords(r#gen));
}

unsafe fn genLiveCopiedWords(mut r#gen: *mut generation) -> StgWord {
    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as c_int != 0 {
        return 0 as StgWord;
    } else {
        return if (*r#gen).live_estimate != 0 {
            (*r#gen).live_estimate as StgWord
        } else {
            (*r#gen).n_words as StgWord
        };
    };
}

unsafe fn genLiveUncopiedWords(mut r#gen: *mut generation) -> StgWord {
    let mut nonmoving_blocks: W_ = 0 as W_;

    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as c_int != 0 {
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
    let mut nonmoving_blocks: W_ = 0 as W_;

    if r#gen == oldest_gen && RtsFlags.GcFlags.useNonmoving as c_int != 0 {
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

unsafe fn gcThreadLiveWords(mut i: uint32_t, mut g: uint32_t) -> StgWord {
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

unsafe fn gcThreadLiveBlocks(mut i: uint32_t, mut g: uint32_t) -> StgWord {
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
    let mut needed: W_ = 0 as W_;
    let mut N_0: uint32_t = 0;

    if force_major {
        N_0 = RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t);
    } else {
        N_0 = 0 as uint32_t;
    }

    let mut g: uint32_t = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        let mut r#gen: *mut generation = generations.offset(g as isize) as *mut generation;

        let mut blocks: W_ = if (*r#gen).live_estimate != 0 {
            ((*r#gen).live_estimate as W_).wrapping_div(BLOCK_SIZE_W as W_)
        } else {
            (*r#gen).n_blocks as W_
        };

        blocks = (blocks as StgWord).wrapping_add(
            (*r#gen)
                .n_large_blocks
                .wrapping_add((*r#gen).n_compact_blocks) as StgWord,
        ) as W_ as W_;

        needed = needed.wrapping_add(blocks);

        if g == 0 as uint32_t || blocks > (*r#gen).max_blocks {
            N_0 = ({
                let mut _a: uint32_t = N_0 as uint32_t;
                let mut _b: uint32_t = g as uint32_t;

                if _a <= _b { _b } else { _a as uint32_t }
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
                || RtsFlags.GcFlags.useNonmoving as c_int != 0 && r#gen == oldest_gen)
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
    let mut g: uint32_t = 0;
    let mut totalW: StgWord = 0 as StgWord;
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        totalW = totalW.wrapping_add((*generations.offset(g as isize)).n_large_words as StgWord);
        g = g.wrapping_add(1);
    }

    totalW = totalW.wrapping_add(nonmoving_large_words as StgWord);

    return totalW;
}

unsafe fn calcTotalCompactW() -> StgWord {
    let mut g: uint32_t = 0;
    let mut totalW: StgWord = 0 as StgWord;
    g = 0 as uint32_t;

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
    sys_icache_invalidate(exec_addr as *mut c_void, len as size_t);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_clearMemory() {
    clear_free_list();

    let mut i: uint32_t = 0 as uint32_t;

    while i < n_nurseries {
        let mut bd = (*nurseries.offset(i as isize)).blocks;

        while !bd.is_null() {
            clear_blocks(bd);
            bd = (*bd).link as *mut bdescr;
        }

        i = i.wrapping_add(1);
    }

    let mut i_0 = 0 as c_uint;

    while i_0 < getNumCapabilities() {
        let mut bd_0 = (*getCapability(i_0 as uint32_t)).pinned_object_empty;

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

        let mut i_1 = 0 as c_int;

        while i_1 < nonmoving_alloca_cnt as c_int {
            let mut alloc: *mut NonmovingAllocator =
                nonmovingHeap.allocators.offset(i_1 as isize) as *mut NonmovingAllocator;

            let mut seg_0 = (*alloc).active;

            while !seg_0.is_null() {
                nonmovingClearSegmentFreeBlocks(seg_0);
                seg_0 = (*seg_0).link;
            }

            let mut j = 0 as c_uint;

            while j < getNumCapabilities() {
                let mut cap = getCapability(j as uint32_t);
                nonmovingClearSegmentFreeBlocks(*(*cap).current_segments.offset(i_1 as isize));
                j = j.wrapping_add(1);
            }

            i_1 += 1;
        }
    }
}
