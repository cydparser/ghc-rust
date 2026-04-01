use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::spin_lock::{ACQUIRE_SPIN_LOCK, RELEASE_SPIN_LOCK};
use crate::ffi::rts::storage::block::{
    BF_NONMOVING, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, Bdescr, allocMBlockAlignedGroupOnNode, bdescr,
};
use crate::ffi::rts::storage::closure_macros::GET_CLOSURE_TAG;
use crate::ffi::rts::storage::gc::{initBdescr, memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::gc_alloc_block_sync;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::smp::{atomic_inc, cas};
use crate::ffi::stg::types::{StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord32};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::sm::non_moving::{
    CURRENT, FREE, NONMOVING_ALLOCA0, NONMOVING_SEGMENT_BLOCKS, NonmovingAllocator,
    NonmovingSegment, log2_ceil, nonmoving_alloca_cnt, nonmoving_alloca_dense_cnt,
    nonmoving_block_idx, nonmovingAllocatorForSize, nonmovingHeap, nonmovingPushFilledSegment,
    nonmovingPushFreeSegment, nonmovingSegmentBlockCount, nonmovingSegmentBlockSize,
    nonmovingSegmentGetBlock, nonmovingSegmentGetBlock_, nonmovingSegmentInfo,
};
use crate::sm::non_moving_mark::nonmovingInitUpdRemSet;
use crate::sm::storage::{accountAllocation, sm_mutex};

type AllocLockMode = u32;

const SM_LOCK: AllocLockMode = 2;

const ALLOC_SPIN_LOCK: AllocLockMode = 1;

const NO_LOCK: AllocLockMode = 0;

#[inline]
unsafe fn acquire_alloc_lock(mut mode: AllocLockMode) {
    match mode as u32 {
        2 => {
            let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

            if __r != 0 {
                barf(
                    c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                    c"rts/sm/NonMovingAllocate.c".as_ptr(),
                    32,
                    __r,
                );
            }
        }
        1 => {
            ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
        }
        0 | _ => {}
    };
}

#[inline]
unsafe fn release_alloc_lock(mut mode: AllocLockMode) {
    match mode as u32 {
        2 => {
            if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/sm/NonMovingAllocate.c".as_ptr(),
                    45,
                );
            }
        }
        1 => {
            RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
        }
        0 | _ => {}
    };
}

unsafe fn nonmovingAllocSegment(mut mode: AllocLockMode, mut node: u32) -> *mut NonmovingSegment {
    let mut ret = null_mut::<NonmovingSegment>();
    ret = nonmovingPopFreeSegment();

    if ret.is_null() {
        acquire_alloc_lock(mode);
        ret = nonmovingPopFreeSegment();

        if !ret.is_null() {
            release_alloc_lock(mode);

            if ((ret as usize).wrapping_rem((1 as i32 as usize) << 15 as u64) == 0) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 77);
            }

            return ret;
        }

        let mut bd = allocMBlockAlignedGroupOnNode(node, NONMOVING_SEGMENT_BLOCKS as W_);
        release_alloc_lock(mode);

        let mut alloc_blocks: W_ = BLOCKS_PER_MBLOCK
            .wrapping_sub(BLOCKS_PER_MBLOCK.wrapping_rem(NONMOVING_SEGMENT_BLOCKS as W_));

        (*oldest_gen).n_blocks = ((*oldest_gen).n_blocks as StgWord)
            .wrapping_add(alloc_blocks as StgWord) as memcount
            as memcount;
        (*oldest_gen).n_words = ((*oldest_gen).n_words as StgWord)
            .wrapping_add((BLOCK_SIZE_W as W_).wrapping_mul(alloc_blocks) as StgWord)
            as memcount as memcount;

        let mut i: StgWord32 = 0;

        while (i as W_) < alloc_blocks {
            initBdescr(bd.offset(i as isize) as *mut bdescr, oldest_gen, oldest_gen);
            (*bd.offset(i as isize)).flags = BF_NONMOVING as StgWord16;
            i = i.wrapping_add(1);
        }

        while !(*bd).link.is_null() {
            let mut next_bd = (*bd).link as *mut bdescr;
            (*bd).link = null_mut::<bdescr_>();
            nonmovingPushFreeSegment((*bd).start as *mut NonmovingSegment);
            bd = next_bd;
        }

        ret = (*bd).start as *mut NonmovingSegment;
    }

    if ((ret as usize).wrapping_rem((1 as i32 as usize) << 15 as u64) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 108);
    }

    return ret;
}

unsafe fn nonmovingClearBitmap(mut seg: *mut NonmovingSegment) {
    let mut n = nonmovingSegmentBlockCount(seg);
    memset(
        &raw mut (*seg).bitmap as *mut u8 as *mut c_void,
        0,
        n as usize,
    );
}

unsafe fn nonmovingInitSegment(mut seg: *mut NonmovingSegment, mut allocator_idx: u16) {
    let mut bd = Bdescr(seg as StgPtr);
    (*seg).link = null_mut::<NonmovingSegment>();
    (*seg).todo_link = null_mut::<NonmovingSegment>();
    (*seg).next_free = 0;
    (&raw mut (*seg).state).store(FREE, Ordering::Relaxed);
    (*bd).c2rust_unnamed.nonmoving_segment.allocator_idx = allocator_idx as StgWord16;
    (*bd).c2rust_unnamed.nonmoving_segment.next_free_snap = 0;
    (*bd).u.scan = nonmovingSegmentGetBlock(seg, 0) as StgPtr;
    nonmovingClearBitmap(seg);
}

unsafe fn nonmovingInitCapability(mut cap: *mut Capability) {
    let mut segs = stgMallocBytes(
        (size_of::<*mut NonmovingSegment>() as usize).wrapping_mul(nonmoving_alloca_cnt as usize),
        c"current segment array".as_ptr(),
    ) as *mut *mut NonmovingSegment;

    let mut i = 0;

    while i < nonmoving_alloca_cnt as u32 {
        let ref mut fresh9 = *segs.offset(i as isize);
        *fresh9 = nonmovingAllocSegment(NO_LOCK, (*cap).node);
        nonmovingInitSegment(*segs.offset(i as isize), i as u16);
        (&raw mut (**segs.offset(i as isize)).state).store(CURRENT, Ordering::Relaxed);
        i = i.wrapping_add(1);
    }

    (*cap).current_segments = segs;
    (*cap).upd_rem_set.queue.blocks = null_mut::<bdescr>();
    nonmovingInitUpdRemSet(&raw mut (*cap).upd_rem_set);
}

unsafe fn advance_next_free(mut seg: *mut NonmovingSegment, blk_count: u32) -> bool {
    let mut bitmap: *const u8 = &raw mut (*seg).bitmap as *mut u8;

    if (blk_count == nonmovingSegmentBlockCount(seg)) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 153);
    }

    let mut c = memchr(
        bitmap.offset(((*seg).next_free as i32 + 1) as isize) as *const u8 as *const c_void,
        0,
        blk_count
            .wrapping_sub((*seg).next_free as u32)
            .wrapping_sub(1 as u32) as usize,
    ) as *const u8;

    if c.is_null() {
        (*seg).next_free = blk_count as nonmoving_block_idx;

        return true;
    } else {
        (*seg).next_free = c.offset_from(bitmap) as i64 as nonmoving_block_idx;

        return false;
    };
}

unsafe fn nonmovingPopFreeSegment() -> *mut NonmovingSegment {
    loop {
        let mut seg = (&raw mut nonmovingHeap.free).load(Ordering::Acquire);

        if seg.is_null() {
            return null_mut::<NonmovingSegment>();
        }

        if cas(
            &raw mut nonmovingHeap.free as StgVolatilePtr,
            seg as StgWord,
            (*seg).link as StgWord,
        ) == seg as StgWord
        {
            return seg;
        }
    }
}

unsafe fn pop_active_segment(mut alloca: *mut NonmovingAllocator) -> *mut NonmovingSegment {
    loop {
        let mut seg = (&raw mut (*alloca).active).load(Ordering::Acquire);

        if seg.is_null() {
            return null_mut::<NonmovingSegment>();
        }

        let mut next = (&raw mut (*seg).link).load(Ordering::Relaxed);

        if cas(
            &raw mut (*alloca).active as StgVolatilePtr,
            seg as StgWord,
            next as StgWord,
        ) == seg as StgWord
        {
            return seg;
        }
    }
}

unsafe fn nonmovingAllocate_(
    mut mode: AllocLockMode,
    mut cap: *mut Capability,
    mut sz: StgWord,
) -> *mut c_void {
    let mut block_size: u32 = 0;

    if sz.wrapping_mul(size_of::<StgWord>() as StgWord)
        <= (NONMOVING_ALLOCA0 as usize).wrapping_add(
            ((nonmoving_alloca_dense_cnt as i32 - 1 as i32) as usize)
                .wrapping_mul(size_of::<StgWord>() as usize),
        ) as StgWord
    {
        block_size = (size_of::<StgWord>() as StgWord).wrapping_mul(sz) as u32;
    } else {
        let mut log_block_size =
            log2_ceil(sz.wrapping_mul(size_of::<StgWord>() as StgWord) as u64) as u32;

        block_size = (1 << log_block_size) as u32;
    }

    if ((block_size as usize) < 1 << 15) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 220);
    }

    let mut alloca_idx = nonmovingAllocatorForSize(block_size as u16) as u32;
    let mut alloca: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset(alloca_idx as isize) as *mut NonmovingAllocator;

    let mut current = *(*cap).current_segments.offset(alloca_idx as isize);

    if !current.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 227);
    }

    if (block_size == nonmovingSegmentBlockSize(current)) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 228);
    }

    let mut block_count = nonmovingSegmentBlockCount(current);

    let mut ret = nonmovingSegmentGetBlock_(
        current,
        block_size as u16,
        block_count as u16,
        (*current).next_free,
    );

    if (GET_CLOSURE_TAG(ret as *const StgClosure) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingAllocate.c".as_ptr(), 231);
    }

    let mut full = advance_next_free(current, block_count);

    if full {
        let mut new_blocks =
            block_count.wrapping_sub((*nonmovingSegmentInfo(current)).next_free_snap as u32);

        atomic_inc(
            &raw mut (*oldest_gen).live_estimate as StgVolatilePtr,
            (new_blocks.wrapping_mul(block_size) as usize).wrapping_div(size_of::<W_>() as usize)
                as StgWord,
        );

        nonmovingPushFilledSegment(current);

        let mut new_current = pop_active_segment(alloca);

        if new_current.is_null() {
            new_current = nonmovingAllocSegment(mode, (*cap).node);
            nonmovingInitSegment(new_current, alloca_idx as u16);
        }

        (*new_current).link = null_mut::<NonmovingSegment>();
        (&raw mut (*new_current).state).store(CURRENT, Ordering::Relaxed);

        let ref mut fresh8 = *(*cap).current_segments.offset(alloca_idx as isize);
        *fresh8 = new_current;
    }

    return ret;
}

unsafe fn nonmovingAllocateGC(mut cap: *mut Capability, mut sz: StgWord) -> *mut c_void {
    return nonmovingAllocate_(ALLOC_SPIN_LOCK, cap, sz);
}

unsafe fn nonmovingAllocate(mut cap: *mut Capability, mut sz: StgWord) -> *mut c_void {
    accountAllocation(cap, sz as W_);
    (*cap).total_allocated = (*cap).total_allocated.wrapping_add(sz as u64);

    return nonmovingAllocate_(SM_LOCK, cap, sz);
}
