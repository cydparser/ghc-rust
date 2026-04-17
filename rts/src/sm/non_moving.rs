use crate::capability::{getCapability, markCapability};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::os_threads::{
    Condition, Mutex, OSThreadId, broadcastCondition, closeCondition, closeMutex, createOSThread,
    initCondition, initMutex, signalCondition, waitCondition,
};
use crate::ffi::rts::storage::block::{
    BF_LARGE, BF_MARKED, BF_NONMOVING, BF_NONMOVING_SWEEPING, BLOCK_SIZE, BLOCK_SIZE_W,
    BLOCKS_PER_MBLOCK, Bdescr, MBLOCK_MASK, NonmovingSegmentInfo, bdescr, dbl_link_onto, freeGroup,
};
use crate::ffi::rts::storage::block::{
    BF_LARGE, BF_NONMOVING, BF_NONMOVING_SWEEPING, BLOCK_SIZE, Bdescr, NonmovingSegmentInfo, bdescr,
};
use crate::ffi::rts::storage::closures::{StgIndStatic, StgWeak};
use crate::ffi::rts::storage::gc::{memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::threads::{getNumCapabilities, n_capabilities};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::{StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord32, StgWord64};
use crate::ffi::stg::types::{StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord32, StgWord64};
use crate::ffi::stg::{P_, W_};
use crate::ghcautoconf::SIZEOF_VOID_P;
use crate::ghcautoconf::SIZEOF_VOID_P;
use crate::prelude::*;
use crate::printer::printClosure;
use crate::rts_flags::RtsFlags;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::schedule::{SCHED_RUNNING, getSchedState, releaseAllCapabilities, resurrectThreads};
use crate::sm::gc::{markCAFs, resizeGenerations};
use crate::sm::non_moving::{
    ACTIVE, FILLED, FILLED_SWEEPING, FREE, NONMOVING_ALLOCA_DIVIDE_SHIFT, NONMOVING_ALLOCA0,
    NONMOVING_SEGMENT_BITS, NONMOVING_SEGMENT_BLOCKS, NONMOVING_SEGMENT_MASK,
    NONMOVING_SEGMENT_SIZE, NONMOVING_SEGMENT_SIZE_W, NonmovingAllocator, NonmovingHeap,
    NonmovingSegment, NonmovingSegmentState, log2_ceil, nonmoving_block_idx,
    nonmoving_first_sparse_allocator_size, nonmovingAllocatorForSize, nonmovingBlockCount,
    nonmovingGetBlockIdx, nonmovingGetClosureMark, nonmovingGetMark, nonmovingGetSegment,
    nonmovingGetSegment_unchecked, nonmovingIsInSegment, nonmovingSegmentAllocator,
    nonmovingSegmentBeingSwept, nonmovingSegmentBlockCount, nonmovingSegmentBlockSize,
    nonmovingSegmentDivideBySize, nonmovingSegmentGetBlock, nonmovingSegmentGetBlock_,
    nonmovingSegmentInfo, nonmovingSetMark,
};
use crate::sm::non_moving_census::{nonmovingPrintAllocatorCensus, nonmovingTraceAllocatorCensus};
use crate::sm::non_moving_mark::{
    MarkBudget, MarkQueue, MarkQueue_, UNLIMITED_MARK_BUDGET, current_mark_queue,
    debug_caf_list_snapshot, freeMarkQueue, initMarkQueue, markQueueAddRoot,
    markQueuePushClosureGC, n_nonmoving_compact_blocks, n_nonmoving_large_blocks,
    n_nonmoving_marked_compact_blocks, n_nonmoving_marked_large_blocks, nonmoving_compact_objects,
    nonmoving_compact_words, nonmoving_large_objects, nonmoving_large_words,
    nonmoving_marked_compact_objects, nonmoving_marked_large_objects, nonmoving_old_threads,
    nonmoving_old_weak_ptr_list, nonmoving_threads, nonmoving_weak_ptr_list, nonmovingBeginFlush,
    nonmovingFinishFlush, nonmovingMark, nonmovingMarkDeadWeaks, nonmovingMarkInit,
    nonmovingMarkUnlimitedBudget, nonmovingMarkWeakPtrList, nonmovingResurrectThreads,
    nonmovingTidyThreads, nonmovingTidyWeaks, nonmovingWaitForFlush,
};
use crate::sm::non_moving_sweep::{
    nonmovingGcCafs, nonmovingSweep, nonmovingSweepCompactObjects, nonmovingSweepLargeObjects,
    nonmovingSweepMutLists, nonmovingSweepStableNameTable,
};
use crate::sm::storage::{
    END_OF_CAF_LIST, STATIC_FLAG_A, STATIC_FLAG_B, countOccupied, debug_caf_list, prev_static_flag,
    sm_mutex, static_flag,
};
use crate::sparks::pruneSparkQueue;
use crate::stable_ptr::markStablePtrTable;
use crate::stats::{stat_endNonmovingGc, stat_endNonmovingGcSync, stat_startNonmovingGc};
use crate::task::{myTask, newBoundTask};
use crate::trace::{
    DEBUG_RTS, TRACE_nonmoving_gc, trace_, traceConcSweepBegin, traceConcSweepEnd,
    traceConcSyncEnd, traceNonmovingPrunedSegments,
};
use crate::weak::scheduleFinalizers;

/// cbindgen:no-export
pub(crate) struct NonmovingSegment {
    pub(crate) link: *mut NonmovingSegment,
    pub(crate) todo_link: *mut NonmovingSegment,
    pub(crate) next_free: nonmoving_block_idx,
    pub(crate) state: NonmovingSegmentState,
    pub(crate) bitmap: [u8; 0],
}

pub(crate) type NonmovingSegmentState = u32;

pub(crate) const FILLED_SWEEPING: NonmovingSegmentState = 4;

pub(crate) const FILLED: NonmovingSegmentState = 3;

pub(crate) const ACTIVE: NonmovingSegmentState = 2;

pub(crate) const CURRENT: NonmovingSegmentState = 1;

pub(crate) const FREE: NonmovingSegmentState = 0;

pub(crate) type nonmoving_block_idx = u16;

/// cbindgen:no-export
pub(crate) struct NonmovingAllocator {
    pub(crate) filled: *mut NonmovingSegment,
    pub(crate) saved_filled: *mut NonmovingSegment,
    pub(crate) active: *mut NonmovingSegment,
    pub(crate) block_size: StgWord16,
    pub(crate) block_count: StgWord16,
    pub(crate) block_division_constant: StgWord32,
}

/// cbindgen:no-export
pub(crate) struct NonmovingHeap {
    pub(crate) allocators: *mut NonmovingAllocator,
    pub(crate) free: *mut NonmovingSegment,
    pub(crate) saved_free: *mut NonmovingSegment,
    pub(crate) n_caps: u32,
    pub(crate) sweep_list: *mut NonmovingSegment,
}

pub(crate) const NONMOVING_SEGMENT_BITS: u64 = 15;

pub(crate) const NONMOVING_SEGMENT_MASK: usize =
    ((1 as i32 as usize) << NONMOVING_SEGMENT_BITS).wrapping_sub(1 as i32 as usize);

pub(crate) const NONMOVING_SEGMENT_SIZE: usize = 1 << NONMOVING_SEGMENT_BITS;

pub(crate) const NONMOVING_SEGMENT_SIZE_W: usize =
    ((1 as i32 as usize) << NONMOVING_SEGMENT_BITS).wrapping_div(SIZEOF_VOID_P as usize);

pub(crate) const NONMOVING_SEGMENT_BLOCKS: usize =
    NONMOVING_SEGMENT_SIZE.wrapping_div(BLOCK_SIZE as usize);

pub(crate) const NONMOVING_ALLOCA0: i32 = 8;

pub(crate) const NONMOVING_ALLOCA_DIVIDE_SHIFT: i32 = 32;

#[inline]
pub(crate) unsafe fn log2_ceil(mut x: u64) -> u64 {
    return (size_of::<u64>() as u64)
        .wrapping_mul(8 as u64)
        .wrapping_sub(x.wrapping_sub(1 as u64).leading_zeros() as i32 as u64);
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentInfo(
    mut seg: *mut NonmovingSegment,
) -> *mut NonmovingSegmentInfo {
    return &raw mut (*(Bdescr as unsafe extern "C" fn(StgPtr) -> *mut bdescr)(seg as StgPtr))
        .c2rust_unnamed
        .nonmoving_segment;
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentAllocator(
    mut seg: *mut NonmovingSegment,
) -> NonmovingAllocator {
    return *nonmovingHeap
        .allocators
        .offset((*nonmovingSegmentInfo(seg)).allocator_idx as isize);
}

#[inline]
pub(crate) unsafe fn nonmovingAllocatorForSize(mut block_size: u16) -> u8 {
    if block_size as i32 - NONMOVING_ALLOCA0
        < nonmoving_alloca_dense_cnt as i32 * size_of::<StgWord>() as u16 as i32
    {
        return ((block_size as i32 - NONMOVING_ALLOCA0) as usize)
            .wrapping_div(size_of::<StgWord>() as usize) as u8;
    } else {
        return log2_ceil(block_size as u64)
            .wrapping_sub(log2_ceil((NONMOVING_ALLOCA0 as u64).wrapping_add(
                (size_of::<StgWord>() as u64).wrapping_mul(nonmoving_alloca_dense_cnt as u64),
            )))
            .wrapping_add(nonmoving_alloca_dense_cnt as u64) as u8;
    };
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentBlockSize(mut seg: *mut NonmovingSegment) -> u32 {
    return nonmovingSegmentAllocator(seg).block_size as u32;
}

#[inline]
pub(crate) unsafe fn nonmovingPushActiveSegment(mut seg: *mut NonmovingSegment) {
    let mut alloc: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset((nonmovingAllocatorForSize
            as unsafe extern "C" fn(c_ushort) -> u8)(
            (nonmovingSegmentBlockSize as unsafe extern "C" fn(*mut NonmovingSegment) -> c_uint)(
                seg,
            ) as u16,
        ) as isize) as *mut NonmovingAllocator;

    (&raw mut (*seg).state).store(ACTIVE, Ordering::Relaxed);

    loop {
        let mut current_active = (&raw mut (*alloc).active).load(Ordering::Relaxed);
        (*seg).link = current_active;

        if cas(
            &raw mut (*alloc).active as StgVolatilePtr,
            current_active as StgWord,
            seg as StgWord,
        ) == current_active as StgWord
        {
            break;
        }
    }
}

#[inline]
pub(crate) unsafe fn nonmovingPushFilledSegment(mut seg: *mut NonmovingSegment) {
    let mut alloc: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset((nonmovingAllocatorForSize
            as unsafe extern "C" fn(c_ushort) -> u8)(
            (nonmovingSegmentBlockSize as unsafe extern "C" fn(*mut NonmovingSegment) -> c_uint)(
                seg,
            ) as u16,
        ) as isize) as *mut NonmovingAllocator;

    (&raw mut (*seg).state).store(FILLED, Ordering::Relaxed);

    loop {
        let mut current_filled = (&raw mut (*alloc).filled).load(Ordering::Relaxed);
        (&raw mut (*seg).link).store(current_filled, Ordering::Relaxed);

        if cas(
            &raw mut (*alloc).filled as StgVolatilePtr,
            current_filled as StgWord,
            seg as StgWord,
        ) == current_filled as StgWord
        {
            break;
        }
    }
}

#[inline]
pub(crate) unsafe fn nonmovingBlockCount(mut block_size: u16) -> u32 {
    let mut segment_data_size =
        NONMOVING_SEGMENT_SIZE.wrapping_sub(size_of::<NonmovingSegment>() as usize) as u32;
    segment_data_size =
        segment_data_size.wrapping_sub(segment_data_size.wrapping_rem(SIZEOF_VOID_P as u32));

    let mut block_count = segment_data_size.wrapping_div((block_size as i32 + 1 as i32) as u32);

    if (block_count < 0xfff) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.h".as_ptr(), 257);
    }

    return block_count;
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentBlockCount(mut seg: *mut NonmovingSegment) -> u32 {
    return nonmovingSegmentAllocator(seg).block_count as u32;
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentGetBlock_(
    mut seg: *mut NonmovingSegment,
    mut block_size: u16,
    mut block_count: u16,
    mut i: nonmoving_block_idx,
) -> *mut c_void {
    if (block_size as u32 == nonmovingSegmentBlockSize(seg)) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.h".as_ptr(), 272);
    }

    let mut bitmap_size: W_ = (block_count as usize).wrapping_mul(size_of::<u8>() as usize) as W_;

    let mut data: W_ = (seg as W_)
        .wrapping_add(size_of::<NonmovingSegment>() as W_)
        .wrapping_add(bitmap_size)
        .wrapping_add(size_of::<W_>() as W_)
        .wrapping_sub(1 as W_)
        .wrapping_div(size_of::<W_>() as W_)
        .wrapping_mul(size_of::<W_>() as W_);

    return data.wrapping_add((i as i32 * block_size as i32) as W_) as *mut c_void;
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentGetBlock(
    mut seg: *mut NonmovingSegment,
    mut i: nonmoving_block_idx,
) -> *mut c_void {
    return nonmovingSegmentGetBlock_(
        seg,
        nonmovingSegmentBlockSize(seg) as u16,
        nonmovingSegmentBlockCount(seg) as u16,
        i,
    );
}

#[inline]
pub(crate) unsafe fn nonmovingGetSegment_unchecked(mut p: StgPtr) -> *mut NonmovingSegment {
    let mask: usize = !NONMOVING_SEGMENT_MASK;

    return (p as usize & mask) as *mut NonmovingSegment;
}

#[inline]
pub(crate) unsafe fn nonmovingIsInSegment(mut p: StgPtr) -> bool {
    let mut bd = Bdescr(p);

    return p as W_ >= mblock_address_space.0.begin
        && (p as W_) < mblock_address_space.0.end
        && (*bd).flags as i32 & BF_NONMOVING != 0
        && (*bd).flags as i32 & BF_LARGE == 0;
}

#[inline]
pub(crate) unsafe fn nonmovingGetSegment(mut p: StgPtr) -> *mut NonmovingSegment {
    if nonmovingIsInSegment(p) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.h".as_ptr(), 307);
    }

    return nonmovingGetSegment_unchecked(p);
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentDivideBySize(
    mut seg: *mut NonmovingSegment,
    mut x: u16,
) -> u16 {
    return ((x as StgWord64)
        .wrapping_mul(nonmovingSegmentAllocator(seg).block_division_constant as StgWord64)
        >> NONMOVING_ALLOCA_DIVIDE_SHIFT) as u16;
}

#[inline]
pub(crate) unsafe fn nonmovingGetBlockIdx(mut p: StgPtr) -> nonmoving_block_idx {
    let mut seg = nonmovingGetSegment(p);
    let mut blk0: ptrdiff_t = nonmovingSegmentGetBlock(seg, 0) as ptrdiff_t;
    let mut offset: ptrdiff_t = p as ptrdiff_t - blk0;

    return nonmovingSegmentDivideBySize(seg, offset as u16);
}

#[inline]
pub(crate) unsafe fn nonmoving_first_sparse_allocator_size() -> u16 {
    return log2_ceil(
        (NONMOVING_ALLOCA0 as u64)
            .wrapping_add(
                ((nonmoving_alloca_dense_cnt as i32 - 1 as i32) as u64).wrapping_mul(size_of::<
                    StgWord,
                >(
                )
                    as u64),
            )
            .wrapping_add(1 as u64),
    ) as u16;
}

#[inline]
pub(crate) unsafe fn nonmovingSetMark(mut seg: *mut NonmovingSegment, mut i: nonmoving_block_idx) {
    ((&raw mut (*seg).bitmap as *mut u8).offset(i as isize) as *mut u8)
        .store(nonmovingMarkEpoch, Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn nonmovingGetMark(
    mut seg: *mut NonmovingSegment,
    mut i: nonmoving_block_idx,
) -> u8 {
    return ((&raw mut (*seg).bitmap as *mut u8).offset(i as isize) as *mut u8)
        .load(Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn nonmovingSetClosureMark(mut p: StgPtr) {
    nonmovingSetMark(nonmovingGetSegment(p), nonmovingGetBlockIdx(p));
}

#[inline]
pub(crate) unsafe fn nonmovingGetClosureMark(mut p: StgPtr) -> u8 {
    let mut seg = nonmovingGetSegment(p);
    let mut blk_idx = nonmovingGetBlockIdx(p);

    return nonmovingGetMark(seg, blk_idx);
}

#[inline]
pub(crate) unsafe fn nonmovingClosureMarkedThisCycle(mut p: StgPtr) -> bool {
    return nonmovingGetClosureMark(p) as i32 == nonmovingMarkEpoch as i32;
}

#[inline]
pub(crate) unsafe fn nonmovingSegmentBeingSwept(mut seg: *mut NonmovingSegment) -> bool {
    let mut seginfo = nonmovingSegmentInfo(seg);
    let mut n = nonmovingSegmentBlockCount(seg);

    return (*seginfo).next_free_snap as u32 >= n;
}

#[inline]
pub(crate) unsafe fn nonmovingClosureBeingSwept(mut p: *mut StgClosure) -> bool {
    let mut bd = Bdescr(p as StgPtr);

    if p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end {
        if (*bd).flags as i32 & BF_NONMOVING_SWEEPING != 0 {
            return true;
        } else if (*bd).flags as i32 & BF_NONMOVING != 0 {
            let mut seg = nonmovingGetSegment(p as StgPtr);

            return nonmovingSegmentBeingSwept(seg);
        } else {
            return false;
        }
    } else {
        return true;
    };
}

#[inline]
pub(crate) unsafe fn isNonmovingClosure(mut p: *mut StgClosure) -> bool {
    return RtsFlags.GcFlags.useNonmoving as i32 != 0
        && (!(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end)
            || (*Bdescr(p as StgPtr)).flags as i32 & BF_NONMOVING != 0);
}

extern "C" {
    pub(crate) type NonMovingSegment;
}

const CONCURRENT_WORKER_RUNNING: ConcurrentWorkerState = 1;

type ConcurrentWorkerState = u32;

const CONCURRENT_WORKER_STOPPED: ConcurrentWorkerState = 2;

const CONCURRENT_WORKER_IDLE: ConcurrentWorkerState = 0;

static mut nonmovingHeap: NonmovingHeap = NonmovingHeap {
    allocators: null_mut::<NonmovingAllocator>(),
    free: null_mut::<NonmovingSegment>(),
    saved_free: null_mut::<NonmovingSegment>(),
    n_caps: 0,
    sweep_list: null_mut::<NonmovingSegment>(),
};

static mut nonmovingMarkEpoch: u8 = 1;

static mut nonmoving_alloca_dense_cnt: u8 = 0;

static mut nonmoving_alloca_cnt: u8 = 0;

unsafe fn nonmovingBumpEpoch() {
    nonmovingMarkEpoch = (if nonmovingMarkEpoch as i32 == 1 { 2 } else { 1 }) as u8;
}

static mut nonmoving_segment_live_words: memcount = 0;

static mut sync_phase_marking_budget: MarkBudget = 200000;

unsafe fn nonmovingPushFreeSegment(mut seg: *mut NonmovingSegment) {
    (&raw mut (*seg).state).store(FREE, Ordering::Relaxed);

    loop {
        let mut old = nonmovingHeap.free;
        (*seg).link = old;

        if cas(
            &raw mut nonmovingHeap.free as StgVolatilePtr,
            old as StgWord,
            seg as StgWord,
        ) == old as StgWord
        {
            break;
        }
    }
}

unsafe fn cmp_segment_ptr(mut x: *const c_void, mut y: *const c_void) -> i32 {
    let mut p1 = *(x as *mut *const NonMovingSegment);
    let mut p2 = *(y as *mut *const NonMovingSegment);

    if p1 > p2 {
        return 1;
    } else if p1 < p2 {
        return -1;
    } else {
        return 0;
    };
}

unsafe fn nonmovingPruneFreeSegmentList() {
    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Pruning free segment list.".as_ptr());
    }

    let mut free = null_mut::<NonmovingSegment>();

    loop {
        free = (&raw mut nonmovingHeap.free).load(Ordering::Acquire);

        if cas(
            &raw mut nonmovingHeap.free as StgVolatilePtr,
            free as StgWord,
            NULL as StgWord,
        ) == free as StgWord
        {
            break;
        }
    }

    nonmovingHeap.saved_free = free;

    let mut length: usize = 0;
    let mut free1 = free;

    while !free1.is_null() {
        length = length.wrapping_add(1);
        free1 = (*free1).link;
    }

    let mut sorted = stgMallocBytes(
        (size_of::<*mut NonmovingSegment>() as usize).wrapping_mul(length),
        c"sorted free segment list".as_ptr(),
    ) as *mut *mut NonmovingSegment;

    let mut i: usize = 0;

    while i < length {
        let ref mut fresh6 = *sorted.offset(i as isize);
        *fresh6 = free;
        free = (*free).link;
        i = i.wrapping_add(1);
    }

    if free.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 652);
    }

    qsort(
        sorted as *mut c_void,
        length,
        size_of::<*mut NonmovingSegment>() as usize,
        Some(cmp_segment_ptr as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );

    let mut new_length: usize = 0;
    let mut free_in_megablock: usize = 0;
    let mut i_0: usize = 0;

    while i_0 < length {
        free_in_megablock = 1;

        while i_0.wrapping_add(free_in_megablock) < length {
            if *sorted.offset(i_0 as isize) as W_ & !MBLOCK_MASK as W_
                != *sorted.offset(i_0.wrapping_add(free_in_megablock) as isize) as W_
                    & !MBLOCK_MASK as W_
            {
                break;
            }

            free_in_megablock = free_in_megablock.wrapping_add(1);
        }

        if (free_in_megablock as W_)
            < BLOCKS_PER_MBLOCK.wrapping_div(NONMOVING_SEGMENT_BLOCKS as W_)
        {
            let mut j: usize = 0;

            while j < free_in_megablock {
                let mut last = free;
                free = *sorted.offset(i_0.wrapping_add(j) as isize);
                (*free).link = last;
                new_length = new_length.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        } else {
            let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

            if __r != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/sm/NonMoving.c".as_ptr(),
                    679,
                    __r,
                );
            }

            let mut j_0: usize = 0;

            while j_0 < free_in_megablock {
                let mut bd = Bdescr(*sorted.offset(i_0.wrapping_add(j_0) as isize) as StgPtr);

                freeGroup(bd);
                j_0 = j_0.wrapping_add(1);
            }

            if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/sm/NonMoving.c".as_ptr(),
                    684,
                );
            }
        }

        i_0 = i_0.wrapping_add(free_in_megablock);
    }

    stgFree(sorted as *mut c_void);

    if !free.is_null() {
        let mut tail = free;

        while !(*tail).link.is_null() {
            tail = (*tail).link;
        }

        loop {
            let mut rest = (&raw mut nonmovingHeap.free).load(Ordering::Acquire);
            (*tail).link = rest;

            if cas(
                &raw mut nonmovingHeap.free as StgVolatilePtr,
                rest as StgWord,
                free as StgWord,
            ) == rest as StgWord
            {
                break;
            }
        }
    }

    let mut pruned_segments: usize = length.wrapping_sub(new_length);
    (*oldest_gen).n_blocks = (*oldest_gen)
        .n_blocks
        .wrapping_sub(pruned_segments.wrapping_mul(NONMOVING_SEGMENT_BLOCKS as usize) as memcount);

    (*oldest_gen).n_words = (*oldest_gen)
        .n_words
        .wrapping_sub(pruned_segments.wrapping_mul(NONMOVING_SEGMENT_SIZE as usize) as memcount);

    nonmovingHeap.saved_free = null_mut::<NonmovingSegment>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(
            c"Pruned %d free segments, leaving %d on the free segment list.".as_ptr(),
            pruned_segments,
            new_length,
        );
    }

    traceNonmovingPrunedSegments(pruned_segments as u32, new_length as u32);

    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Finished pruning free segment list.".as_ptr());
    }
}

unsafe fn nonmovingInitAllocator(mut alloc: *mut NonmovingAllocator, mut block_size: u16) {
    *alloc = NonmovingAllocator {
        filled: null_mut::<NonmovingSegment>(),
        saved_filled: null_mut::<NonmovingSegment>(),
        active: null_mut::<NonmovingSegment>(),
        block_size: block_size as StgWord16,
        block_count: nonmovingBlockCount(block_size) as StgWord16,
        block_division_constant: (-(1 as i32) as StgWord32)
            .wrapping_div(block_size as StgWord32)
            .wrapping_add(1 as StgWord32),
    };
}

unsafe fn nonmovingInitAllocators() {
    nonmoving_alloca_dense_cnt = RtsFlags.GcFlags.nonmovingDenseAllocatorCount as u8;

    let mut first_sparse_allocator = nonmoving_first_sparse_allocator_size();
    let mut nonmoving_alloca_sparse_cnt: u16 =
        log2_ceil(NONMOVING_SEGMENT_SIZE as u64).wrapping_sub(first_sparse_allocator as u64) as u16;
    nonmoving_alloca_cnt =
        (nonmoving_alloca_dense_cnt as i32 + nonmoving_alloca_sparse_cnt as i32) as u8;

    nonmovingHeap.allocators = stgMallocBytes(
        (size_of::<NonmovingAllocator>() as usize).wrapping_mul(nonmoving_alloca_cnt as usize),
        c"allocators array".as_ptr(),
    ) as *mut NonmovingAllocator;

    let mut i = 0;

    while i < nonmoving_alloca_dense_cnt as u32 {
        nonmovingInitAllocator(
            nonmovingHeap.allocators.offset(i as isize) as *mut NonmovingAllocator,
            (NONMOVING_ALLOCA0 as usize)
                .wrapping_add((i as usize).wrapping_mul(size_of::<StgWord>() as usize))
                as u16,
        );

        i = i.wrapping_add(1);
    }

    let mut i_0 = nonmoving_alloca_dense_cnt as u32;

    while i_0 < nonmoving_alloca_cnt as u32 {
        let mut block_size: u16 =
            (1 << i_0
                .wrapping_add(first_sparse_allocator as u32)
                .wrapping_sub(nonmoving_alloca_dense_cnt as u32)) as u16;

        nonmovingInitAllocator(
            nonmovingHeap.allocators.offset(i_0 as isize) as *mut NonmovingAllocator,
            block_size,
        );

        i_0 = i_0.wrapping_add(1);
    }
}

unsafe fn nonmovingInit() {
    if !RtsFlags.GcFlags.useNonmoving {
        return;
    }

    nonmovingInitAllocators();
    nonmovingInitConcurrentWorker();
    nonmovingMarkInit();
}

unsafe fn nonmovingExit() {
    if !RtsFlags.GcFlags.useNonmoving {
        return;
    }

    nonmovingExitConcurrentWorker();
}

unsafe fn nonmovingPrepareMark() {
    prev_static_flag = static_flag;

    static_flag = (if static_flag == STATIC_FLAG_A as u32 {
        STATIC_FLAG_B
    } else {
        STATIC_FLAG_A
    }) as u32;

    if nonmovingHeap.sweep_list.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 772);
    }

    nonmovingHeap.n_caps = n_capabilities as u32;
    nonmovingBumpEpoch();

    let mut alloca_idx = 0;

    while alloca_idx < nonmoving_alloca_cnt as i32 {
        let mut alloca: *mut NonmovingAllocator =
            nonmovingHeap.allocators.offset(alloca_idx as isize) as *mut NonmovingAllocator;

        let mut cap_n: u32 = 0;

        while cap_n < nonmovingHeap.n_caps as u32 {
            let mut cap = getCapability(cap_n);
            let mut seg = *(*cap).current_segments.offset(alloca_idx as isize);
            (*nonmovingSegmentInfo(seg)).next_free_snap = (*seg).next_free as StgWord16;
            cap_n = cap_n.wrapping_add(1);
        }

        if (*alloca).saved_filled.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 788);
        }

        (*alloca).saved_filled = (*alloca).filled;
        (*alloca).filled = null_mut::<NonmovingSegment>();
        alloca_idx += 1;
    }

    let mut bd = nonmoving_large_objects;

    while !bd.is_null() {
        (*bd).flags = ((*bd).flags as i32 & !BF_MARKED) as StgWord16;
        bd = (*bd).link as *mut bdescr;
    }

    let mut next = null_mut::<bdescr>();

    if (*oldest_gen).scavenged_large_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 804);
    }

    let mut bd_0 = (*oldest_gen).large_objects;

    while !bd_0.is_null() {
        next = (*bd_0).link as *mut bdescr;
        (*bd_0).flags = ((*bd_0).flags as i32 | BF_NONMOVING_SWEEPING) as StgWord16;
        (*bd_0).flags = ((*bd_0).flags as i32 & !BF_MARKED) as StgWord16;
        dbl_link_onto(bd_0, &raw mut nonmoving_large_objects);
        bd_0 = next;
    }

    n_nonmoving_large_blocks = n_nonmoving_large_blocks.wrapping_add((*oldest_gen).n_large_blocks);
    nonmoving_large_words = nonmoving_large_words.wrapping_add((*oldest_gen).n_large_words);
    (*oldest_gen).large_objects = null_mut::<bdescr>();
    (*oldest_gen).n_large_words = 0;
    (*oldest_gen).n_large_blocks = 0;
    nonmoving_segment_live_words = 0;

    let mut bd_1 = nonmoving_compact_objects;

    while !bd_1.is_null() {
        (*bd_1).flags = ((*bd_1).flags as i32 & !BF_MARKED) as StgWord16;
        bd_1 = (*bd_1).link as *mut bdescr;
    }

    let mut bd_2 = (*oldest_gen).compact_objects;

    while !bd_2.is_null() {
        next = (*bd_2).link as *mut bdescr;
        (*bd_2).flags = ((*bd_2).flags as i32 | BF_NONMOVING_SWEEPING) as StgWord16;
        (*bd_2).flags = ((*bd_2).flags as i32 & !BF_MARKED) as StgWord16;
        dbl_link_onto(bd_2, &raw mut nonmoving_compact_objects);
        bd_2 = next;
    }

    n_nonmoving_compact_blocks =
        n_nonmoving_compact_blocks.wrapping_add((*oldest_gen).n_compact_blocks);
    nonmoving_compact_words = nonmoving_compact_words.wrapping_add(
        (*oldest_gen)
            .n_compact_blocks
            .wrapping_mul(BLOCK_SIZE_W as memcount),
    );

    (*oldest_gen).n_compact_blocks = 0;
    (*oldest_gen).compact_objects = null_mut::<bdescr>();
    debug_caf_list_snapshot = debug_caf_list;
    debug_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
}

unsafe fn nonmovingCollect(
    mut dead_weaks: *mut *mut StgWeak,
    mut resurrected_threads: *mut *mut StgTSO,
    mut concurrent: bool,
) {
    if nonmovingConcurrentMarkIsRunning() {
        if TRACE_nonmoving_gc as i64 != 0 {
            trace_(c"Aborted nonmoving collection due to on-going collection".as_ptr());
        }
    } else if getSchedState() as u32 > SCHED_RUNNING as i32 as u32 {
        if TRACE_nonmoving_gc as i64 != 0 {
            trace_(c"Aborted nonmoving collection due to on-going shutdown".as_ptr());
        }

        return;
    }

    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Starting nonmoving GC preparation".as_ptr());
    }

    resizeGenerations();
    nonmovingPrepareMark();

    if nonmoving_marked_large_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 865);
    }

    if (n_nonmoving_marked_large_blocks == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 866);
    }

    if nonmoving_marked_compact_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 867);
    }

    if (n_nonmoving_marked_compact_blocks == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 868);
    }

    let mut mark_queue =
        stgMallocBytes(size_of::<MarkQueue>() as usize, c"mark queue".as_ptr()) as *mut MarkQueue;

    (*mark_queue).blocks = null_mut::<bdescr>();
    initMarkQueue(mark_queue);
    current_mark_queue = mark_queue;

    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Marking roots for nonmoving GC".as_ptr());
    }

    markCAFs(
        transmute::<
            Option<unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> ()>,
            evac_fn,
        >(Some(
            markQueueAddRoot as unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> (),
        )),
        mark_queue as *mut c_void,
    );

    let mut n = 0;

    while n < getNumCapabilities() {
        markCapability(
            transmute::<
                Option<unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> ()>,
                evac_fn,
            >(Some(
                markQueueAddRoot
                    as unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> (),
            )),
            mark_queue as *mut c_void,
            getCapability(n as u32),
            true,
        );

        n = n.wrapping_add(1);
    }

    markStablePtrTable(
        transmute::<
            Option<unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> ()>,
            evac_fn,
        >(Some(
            markQueueAddRoot as unsafe extern "C" fn(*mut MarkQueue, *mut *mut StgClosure) -> (),
        )),
        mark_queue as *mut c_void,
    );

    let mut w = *dead_weaks;

    while !w.is_null() {
        if ((*Bdescr(w as StgPtr)).r#gen != oldest_gen) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 888);
        }

        w = (*w).link as *mut StgWeak;
    }

    let mut tso = *resurrected_threads;

    while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        markQueuePushClosureGC(mark_queue, tso as *mut StgClosure);
        tso = (*tso).global_link as *mut StgTSO;
    }

    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Finished marking roots for nonmoving GC".as_ptr());
    }

    if ((*oldest_gen).old_threads
        == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 907);
    }

    if (nonmoving_old_threads == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO)
        as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 908);
    }

    nonmoving_old_threads = (*oldest_gen).threads;
    (*oldest_gen).threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;

    if (*oldest_gen).old_weak_ptr_list.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 915);
    }

    if nonmoving_old_weak_ptr_list.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 916);
    }

    let mut weaks: *mut *mut StgWeak = &raw mut (*oldest_gen).weak_ptr_list;
    let mut n_0: u32 = 0;

    while !(*weaks).is_null() {
        weaks = &raw mut (**weaks).link as *mut *mut StgWeak;
        n_0 = n_0.wrapping_add(1);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(c"%d new nonmoving weaks".as_ptr(), n_0);
    }

    *weaks = nonmoving_weak_ptr_list;
    nonmoving_old_weak_ptr_list = (*oldest_gen).weak_ptr_list;
    nonmoving_weak_ptr_list = null_mut::<StgWeak>();
    (*oldest_gen).weak_ptr_list = null_mut::<StgWeak>();

    if TRACE_nonmoving_gc as i64 != 0 {
        trace_(c"Finished nonmoving GC preparation".as_ptr());
    }

    if getSchedState() as u32 != SCHED_RUNNING as i32 as u32 {
        concurrent = false;
    }

    if concurrent {
        nonmovingStartConcurrentMark(mark_queue);
    } else {
        if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/NonMoving.c".as_ptr(),
                956,
            );
        }

        nonmovingMark_(mark_queue, dead_weaks, resurrected_threads, false);

        let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/NonMoving.c".as_ptr(),
                962,
                __r,
            );
        }
    };
}

unsafe fn nonmovingMarkThreadsWeaks(
    mut budget: *mut MarkBudget,
    mut mark_queue: *mut MarkQueue,
) -> bool {
    loop {
        nonmovingMark(budget, mark_queue as *mut MarkQueue_);

        if *budget == 0 {
            return false;
        }

        nonmovingTidyThreads();

        if !nonmovingTidyWeaks(mark_queue as *mut MarkQueue_) {
            return true;
        }
    }
}

static mut concurrent_coll_lock: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut concurrent_mark_roots: *mut MarkQueue = null_mut::<MarkQueue>();

static mut start_concurrent_mark_cond: Condition = Condition {
    cond: _opaque_pthread_cond_t {
        __sig: 0,
        __opaque: [0; 40],
    },
};

static mut concurrent_coll_finished_cond: Condition = Condition {
    cond: _opaque_pthread_cond_t {
        __sig: 0,
        __opaque: [0; 40],
    },
};

static mut concurrent_worker_state: ConcurrentWorkerState = CONCURRENT_WORKER_IDLE;

static mut stop_concurrent_worker: bool = false;

static mut concurrent_worker_thread: OSThreadId = null_mut::<_opaque_pthread_t>();

unsafe fn nonmovingConcurrentMarkWorker(mut data: *mut c_void) -> *mut c_void {
    newBoundTask();

    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1027,
            __r,
        );
    }

    loop {
        concurrent_worker_state = CONCURRENT_WORKER_IDLE;

        waitCondition(
            &raw mut start_concurrent_mark_cond,
            &raw mut concurrent_coll_lock,
        );

        if stop_concurrent_worker {
            concurrent_worker_state = CONCURRENT_WORKER_STOPPED;
            concurrent_worker_thread = null_mut::<_opaque_pthread_t>();
            broadcastCondition(&raw mut concurrent_coll_finished_cond);

            if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/sm/NonMoving.c".as_ptr(),
                    1035,
                );
            }

            return NULL;
        }

        if (concurrent_worker_state as u32 == CONCURRENT_WORKER_RUNNING as i32 as u32) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1039);
        }

        let mut mark_queue = concurrent_mark_roots;
        concurrent_mark_roots = null_mut::<MarkQueue>();

        if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/NonMoving.c".as_ptr(),
                1042,
            );
        }

        let mut dead_weaks = null_mut::<StgWeak>();
        let mut resurrected_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut StgTSO;

        nonmovingMark_(
            mark_queue,
            &raw mut dead_weaks,
            &raw mut resurrected_threads,
            true,
        );

        let mut __r_0 = pthread_mutex_lock(&raw mut concurrent_coll_lock);

        if __r_0 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/sm/NonMoving.c".as_ptr(),
                1048,
                __r_0,
            );
        }

        broadcastCondition(&raw mut concurrent_coll_finished_cond);
    }
}

unsafe fn nonmovingInitConcurrentWorker() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(c"Starting concurrent mark thread".as_ptr());
    }

    initMutex(&raw mut concurrent_coll_lock);

    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1057,
            __r,
        );
    }

    initCondition(&raw mut start_concurrent_mark_cond);
    initCondition(&raw mut concurrent_coll_finished_cond);
    stop_concurrent_worker = false;
    concurrent_worker_state = CONCURRENT_WORKER_IDLE;
    concurrent_mark_roots = null_mut::<MarkQueue>();

    if createOSThread(
        &raw mut concurrent_worker_thread,
        c"nonmoving-mark".as_ptr(),
        Some(nonmovingConcurrentMarkWorker as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
        NULL,
    ) != 0
    {
        barf(
            c"nonmovingInitConcurrentWorker: failed to spawn mark thread: %s".as_ptr(),
            strerror(*__error()),
        );
    }

    if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1068,
        );
    }
}

unsafe fn nonmovingExitConcurrentWorker() {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(c"waiting for nonmoving collector thread to terminate".as_ptr());
    }

    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1075,
            __r,
        );
    }

    while concurrent_worker_state as u32 != CONCURRENT_WORKER_STOPPED as i32 as u32 {
        stop_concurrent_worker = true;
        signalCondition(&raw mut start_concurrent_mark_cond);

        waitCondition(
            &raw mut concurrent_coll_finished_cond,
            &raw mut concurrent_coll_lock,
        );
    }

    if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1081,
        );
    }

    closeMutex(&raw mut concurrent_coll_lock);
    closeCondition(&raw mut start_concurrent_mark_cond);
    closeCondition(&raw mut concurrent_coll_finished_cond);
}

unsafe fn nonmovingStartConcurrentMark(mut roots: *mut MarkQueue) {
    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1090,
            __r,
        );
    }

    if (concurrent_worker_state as u32 != CONCURRENT_WORKER_RUNNING as i32 as u32) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1091);
    }

    concurrent_worker_state = CONCURRENT_WORKER_RUNNING;
    concurrent_mark_roots = roots;
    (&raw mut nonmoving_write_barrier_enabled).store(1, Ordering::Relaxed);
    signalCondition(&raw mut start_concurrent_mark_cond);

    if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1096,
        );
    }
}

unsafe fn nonmovingConcurrentMarkIsRunning() -> bool {
    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1101,
            __r,
        );
    }

    let mut running = concurrent_worker_state as u32 == CONCURRENT_WORKER_RUNNING as i32 as u32;

    if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1103,
        );
    }

    return running;
}

unsafe fn nonmovingBlockConcurrentMark(mut wait: bool) -> bool {
    if !RtsFlags.GcFlags.useNonmoving {
        return true;
    }

    let mut __r = pthread_mutex_lock(&raw mut concurrent_coll_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1116,
            __r,
        );
    }

    if wait {
        while concurrent_worker_state as u32 == CONCURRENT_WORKER_RUNNING as i32 as u32 {
            waitCondition(
                &raw mut concurrent_coll_finished_cond,
                &raw mut concurrent_coll_lock,
            );
        }
    }

    let mut running = concurrent_worker_state as u32 == CONCURRENT_WORKER_RUNNING as i32 as u32;

    if running {
        if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/sm/NonMoving.c".as_ptr(),
                1125,
            );
        }

        return false;
    } else {
        return true;
    };
}

unsafe fn nonmovingUnblockConcurrentMark() {
    if !RtsFlags.GcFlags.useNonmoving {
        return;
    }

    if pthread_mutex_unlock(&raw mut concurrent_coll_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/NonMoving.c".as_ptr(),
            1137,
        );
    }
}

unsafe fn appendWeakList(mut w1: *mut *mut StgWeak, mut w2: *mut StgWeak) {
    while !(*w1).is_null() {
        w1 = &raw mut (**w1).link as *mut *mut StgWeak;
    }

    *w1 = w2;
}

unsafe fn nonmovingMark_(
    mut mark_queue: *mut MarkQueue,
    mut dead_weaks: *mut *mut StgWeak,
    mut resurrected_threads: *mut *mut StgTSO,
    mut concurrent: bool,
) {
    let mut current_block: u64;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
        trace_(c"Starting mark...".as_ptr());
    }

    stat_startNonmovingGc();

    let mut alloca_idx = 0;

    while alloca_idx < nonmoving_alloca_cnt as i32 {
        let mut filled = (*nonmovingHeap.allocators.offset(alloca_idx as isize)).saved_filled;

        if !filled.is_null() {
            let mut seg = filled;

            loop {
                (*nonmovingSegmentInfo(seg)).next_free_snap = (*seg).next_free as StgWord16;
                (&raw mut (*seg).state).store(FILLED_SWEEPING, Ordering::Relaxed);

                if (*seg).link.is_null() {
                    break;
                }

                seg = (*seg).link;
            }

            (*seg).link = nonmovingHeap.sweep_list;
            nonmovingHeap.sweep_list = filled;
        }

        let ref mut fresh5 = (*nonmovingHeap.allocators.offset(alloca_idx as isize)).saved_filled;
        *fresh5 = null_mut::<NonmovingSegment>();
        alloca_idx += 1;
    }

    nonmovingMarkWeakPtrList(mark_queue as *mut MarkQueue_);

    '_concurrent_marking: loop {
        let mut budget: MarkBudget = UNLIMITED_MARK_BUDGET as MarkBudget;
        nonmovingMarkThreadsWeaks(&raw mut budget, mark_queue);

        if !concurrent {
            current_block = 4488286894823169796;
            break;
        }

        if getSchedState() as u32 > SCHED_RUNNING as i32 as u32 {
            appendWeakList(
                &raw mut nonmoving_weak_ptr_list,
                nonmoving_old_weak_ptr_list,
            );

            current_block = 13353976948851021734;
            break;
        } else {
            nonmovingBeginFlush(myTask());

            let mut all_caps_syncd: bool = false;
            let mut sync_marking_budget: MarkBudget = sync_phase_marking_budget;

            loop {
                all_caps_syncd = nonmovingWaitForFlush();

                if nonmovingMarkThreadsWeaks(&raw mut sync_marking_budget, mark_queue) as i32
                    == false
                {
                    traceConcSyncEnd();
                    stat_endNonmovingGcSync();

                    releaseAllCapabilities(n_capabilities, null_mut::<Capability>(), myTask());

                    break;
                } else if all_caps_syncd {
                    current_block = 4488286894823169796;
                    break '_concurrent_marking;
                }
            }
        }
    }

    match current_block {
        4488286894823169796 => {
            nonmovingResurrectThreads(mark_queue as *mut MarkQueue_, resurrected_threads);

            loop {
                nonmovingMarkUnlimitedBudget(mark_queue as *mut MarkQueue_);

                if !nonmovingTidyWeaks(mark_queue as *mut MarkQueue_) {
                    break;
                }
            }

            nonmovingMarkDeadWeaks(mark_queue as *mut MarkQueue_, dead_weaks);
            nonmovingMarkUnlimitedBudget(mark_queue as *mut MarkQueue_);
            nonmovingSweepMutLists();

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
                trace_(
                    c"Done marking, resurrecting threads before releasing capabilities".as_ptr(),
                );
            }

            if concurrent {
                scheduleFinalizers(getCapability(0), *dead_weaks);
                resurrectThreads(*resurrected_threads);
            }

            nonmovingGcCafs();

            if ((*(*mark_queue).top).head == 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1270);
            }

            if (*(*mark_queue).blocks).link.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1271);
            }

            let mut threads: *mut *mut StgTSO = &raw mut (*oldest_gen).threads;

            while *threads != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
                threads = &raw mut (**threads).global_link as *mut *mut StgTSO;
            }

            *threads = nonmoving_threads;
            nonmoving_threads = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            nonmoving_old_threads =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
            nonmoving_old_weak_ptr_list = null_mut::<StgWeak>();

            if concurrent {
                let mut n: u32 = 0;

                while n < getNumCapabilities() as u32 {
                    pruneSparkQueue(true, getCapability(n));
                    n = n.wrapping_add(1);
                }
            }

            if concurrent {
                nonmoving_write_barrier_enabled = false;
                nonmovingFinishFlush(myTask());
            }

            current_mark_queue = null_mut::<MarkQueue>();
            freeMarkQueue(mark_queue);
            stgFree(mark_queue as *mut c_void);
            nonmoving_large_words = countOccupied(nonmoving_marked_large_objects) as memcount;
            nonmoving_compact_words =
                n_nonmoving_marked_compact_blocks.wrapping_mul(BLOCK_SIZE_W as memcount);
            (*oldest_gen).live_estimate = nonmoving_segment_live_words
                .wrapping_add(nonmoving_large_words)
                .wrapping_add(nonmoving_compact_words);
            (*oldest_gen).n_old_blocks = 0;
            resizeGenerations();
            traceConcSweepBegin();
            nonmovingSweepLargeObjects();
            nonmovingSweepCompactObjects();
            nonmovingSweepStableNameTable();
            nonmovingSweep();
            nonmovingPruneFreeSegmentList();

            if nonmovingHeap.sweep_list.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1333);
            }

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
                trace_(c"Finished sweeping.".as_ptr());
            }

            traceConcSweepEnd();

            if RtsFlags.DebugFlags.nonmoving_gc {
                nonmovingPrintAllocatorCensus(!concurrent);
            }

            if RtsFlags.TraceFlags.nonmoving_gc {
                nonmovingTraceAllocatorCensus();
            }
        }
        _ => {}
    }

    stat_endNonmovingGc();
}

unsafe fn assert_in_nonmoving_heap(mut p: StgPtr) {
    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return;
    }

    let mut bd = Bdescr(p);

    if (*bd).flags as i32 & BF_LARGE != 0 {
        let mut cap: u32 = 0;

        while cap < getNumCapabilities() as u32 {
            if bd == (*getCapability(cap)).pinned_object_block {
                return;
            }

            cap = cap.wrapping_add(1);
        }

        if ((*bd).flags as i32 & 1024 != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/NonMoving.c".as_ptr(), 1381);
        }

        return;
    }

    let mut seg = nonmovingHeap.sweep_list;

    while !seg.is_null() {
        if p >= seg as P_ && p < (seg as P_).offset(NONMOVING_SEGMENT_SIZE_W as isize) {
            return;
        }

        seg = (*seg).link;
    }

    let mut alloca_idx = 0;

    while alloca_idx < nonmoving_alloca_cnt as i32 {
        let mut alloca: *mut NonmovingAllocator =
            nonmovingHeap.allocators.offset(alloca_idx as isize) as *mut NonmovingAllocator;

        let mut cap_idx: u32 = 0;

        while cap_idx < nonmovingHeap.n_caps as u32 {
            let mut cap_0 = getCapability(cap_idx);
            let mut seg_0 = *(*cap_0).current_segments.offset(alloca_idx as isize);

            if p >= seg_0 as P_ && p < (seg_0 as P_).offset(NONMOVING_SEGMENT_SIZE_W as isize) {
                return;
            }

            cap_idx = cap_idx.wrapping_add(1);
        }

        let mut seg_1 = (*alloca).active;

        while !seg_1.is_null() {
            if p >= seg_1 as P_ && p < (seg_1 as P_).offset(NONMOVING_SEGMENT_SIZE_W as isize) {
                return;
            }

            seg_1 = (*seg_1).link;
        }

        seg_1 = (*alloca).filled;

        while !seg_1.is_null() {
            if p >= seg_1 as P_ && p < (seg_1 as P_).offset(NONMOVING_SEGMENT_SIZE_W as isize) {
                return;
            }

            seg_1 = (*seg_1).link;
        }

        alloca_idx += 1;
    }

    barf(c"%p is not in nonmoving heap\n".as_ptr(), p as *mut c_void);
}

unsafe fn nonmovingPrintSegment(mut seg: *mut NonmovingSegment) {
    let mut num_blocks = nonmovingSegmentBlockCount(seg) as i32;
    let mut block_size: u16 = nonmovingSegmentBlockSize(seg) as u16;

    debugBelch(
        c"Segment with %d blocks of size: %d bytes, %u words, scan: %p\n".as_ptr(),
        num_blocks,
        block_size as i32,
        (block_size as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as u32,
        (*Bdescr(seg as StgPtr)).u.scan as *mut c_void,
    );

    let mut p_idx: nonmoving_block_idx = 0;

    while (p_idx as i32) < (*seg).next_free as i32 {
        let mut p = nonmovingSegmentGetBlock(seg, p_idx) as *mut StgClosure;

        if nonmovingGetMark(seg, p_idx) as i32 != 0 {
            debugBelch(c"%d (%p)* :\t".as_ptr(), p_idx as i32, p as *mut c_void);
        } else {
            debugBelch(c"%d (%p)  :\t".as_ptr(), p_idx as i32, p as *mut c_void);
        }

        printClosure(p);
        p_idx = p_idx.wrapping_add(1);
    }

    debugBelch(c"End of segment\n\n".as_ptr());
}

unsafe fn nonmovingPrintSweepList() {
    debugBelch(c"==== SWEEP LIST =====\n".as_ptr());

    let mut i = 0;
    let mut seg = nonmovingHeap.sweep_list;

    while !seg.is_null() {
        let fresh14 = i;
        i = i + 1;
        debugBelch(c"%d: %p\n".as_ptr(), fresh14, seg as *mut c_void);
        seg = (*seg).link;
    }

    debugBelch(c"= END OF SWEEP LIST =\n".as_ptr());
}
