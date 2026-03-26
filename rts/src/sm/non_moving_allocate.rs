use crate::ffi::rts::storage::block::{
    BF_NONMOVING, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, Bdescr, allocMBlockAlignedGroupOnNode, bdescr,
};
use crate::ffi::rts::storage::gc::{initBdescr, memcount, oldest_gen};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::smp::{atomic_inc, cas};
use crate::ffi::stg::types::{StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord32};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::sm::non_moving::{
    NONMOVING_ALLOCA0, NONMOVING_SEGMENT_BLOCKS, NonmovingAllocator, NonmovingSegment, log2_ceil,
    nonmoving_alloca_cnt, nonmoving_alloca_dense_cnt, nonmoving_block_idx,
    nonmovingAllocatorForSize, nonmovingHeap, nonmovingPushFilledSegment, nonmovingPushFreeSegment,
    nonmovingSegmentBlockCount, nonmovingSegmentGetBlock, nonmovingSegmentGetBlock_,
    nonmovingSegmentInfo,
};
use crate::sm::non_moving_mark::nonmovingInitUpdRemSet;
use crate::sm::storage::accountAllocation;

type AllocLockMode = c_uint;

const SM_LOCK: AllocLockMode = 2;

const ALLOC_SPIN_LOCK: AllocLockMode = 1;

const NO_LOCK: AllocLockMode = 0;

#[inline]
unsafe fn acquire_alloc_lock(mut mode: AllocLockMode) {
    match mode as c_uint {
        2 | 1 | 0 | _ => {}
    };
}

#[inline]
unsafe fn release_alloc_lock(mut mode: AllocLockMode) {
    match mode as c_uint {
        2 | 1 | 0 | _ => {}
    };
}

unsafe fn nonmovingAllocSegment(
    mut mode: AllocLockMode,
    mut node: uint32_t,
) -> *mut NonmovingSegment {
    let mut ret = null_mut::<NonmovingSegment>();
    ret = nonmovingPopFreeSegment();

    if ret.is_null() {
        acquire_alloc_lock(mode);
        ret = nonmovingPopFreeSegment();

        if !ret.is_null() {
            release_alloc_lock(mode);

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

        let mut i: StgWord32 = 0 as StgWord32;

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

    return ret;
}

unsafe fn nonmovingClearBitmap(mut seg: *mut NonmovingSegment) {
    let mut n = nonmovingSegmentBlockCount(seg);

    memset(
        &raw mut (*seg).bitmap as *mut uint8_t as *mut c_void,
        0 as c_int,
        n as size_t,
    );
}

unsafe fn nonmovingInitSegment(mut seg: *mut NonmovingSegment, mut allocator_idx: uint16_t) {
    let mut bd = Bdescr(seg as StgPtr);
    (*seg).link = null_mut::<NonmovingSegment>();
    (*seg).todo_link = null_mut::<NonmovingSegment>();
    (*seg).next_free = 0 as nonmoving_block_idx;
    (*bd).c2rust_unnamed.nonmoving_segment.allocator_idx = allocator_idx as StgWord16;
    (*bd).c2rust_unnamed.nonmoving_segment.next_free_snap = 0 as StgWord16;
    (*bd).u.scan = nonmovingSegmentGetBlock(seg, 0 as nonmoving_block_idx) as StgPtr;
    nonmovingClearBitmap(seg);
}

unsafe fn nonmovingInitCapability(mut cap: *mut Capability) {
    let mut segs = stgMallocBytes(
        (size_of::<*mut NonmovingSegment>() as size_t).wrapping_mul(nonmoving_alloca_cnt as size_t),
        b"current segment array\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut NonmovingSegment;

    let mut i = 0 as c_uint;

    while i < nonmoving_alloca_cnt as c_uint {
        let ref mut fresh9 = *segs.offset(i as isize);
        *fresh9 = nonmovingAllocSegment(NO_LOCK, (*cap).node);
        nonmovingInitSegment(*segs.offset(i as isize), i as uint16_t);
        i = i.wrapping_add(1);
    }

    (*cap).current_segments = segs;
    (*cap).upd_rem_set.queue.blocks = null_mut::<bdescr>();
    nonmovingInitUpdRemSet(&raw mut (*cap).upd_rem_set);
}

unsafe fn advance_next_free(mut seg: *mut NonmovingSegment, blk_count: c_uint) -> bool {
    let mut bitmap: *const uint8_t = &raw mut (*seg).bitmap as *mut uint8_t;

    let mut c = memchr(
        bitmap.offset(((*seg).next_free as c_int + 1 as c_int) as isize) as *const uint8_t
            as *const c_void,
        0 as c_int,
        blk_count
            .wrapping_sub((*seg).next_free as c_uint)
            .wrapping_sub(1 as c_uint) as size_t,
    ) as *const uint8_t;

    if c.is_null() {
        (*seg).next_free = blk_count as nonmoving_block_idx;

        return r#true != 0;
    } else {
        (*seg).next_free = c.offset_from(bitmap) as c_long as nonmoving_block_idx;

        return r#false != 0;
    };
}

unsafe fn nonmovingPopFreeSegment() -> *mut NonmovingSegment {
    loop {
        let mut seg = nonmovingHeap.free;

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
        let mut seg = (*alloca).active;

        if seg.is_null() {
            return null_mut::<NonmovingSegment>();
        }

        let mut next = (*seg).link;

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
    let mut block_size: c_uint = 0;

    if sz.wrapping_mul(size_of::<StgWord>() as StgWord)
        <= (NONMOVING_ALLOCA0 as usize).wrapping_add(
            ((nonmoving_alloca_dense_cnt as c_int - 1 as c_int) as usize).wrapping_mul(size_of::<
                StgWord,
            >(
            )
                as usize),
        ) as StgWord
    {
        block_size = (size_of::<StgWord>() as StgWord).wrapping_mul(sz) as c_uint;
    } else {
        let mut log_block_size =
            log2_ceil(sz.wrapping_mul(size_of::<StgWord>() as StgWord) as c_ulong) as c_uint;
        block_size = ((1 as c_int) << log_block_size) as c_uint;
    }

    let mut alloca_idx = nonmovingAllocatorForSize(block_size as uint16_t) as c_uint;
    let mut alloca: *mut NonmovingAllocator =
        nonmovingHeap.allocators.offset(alloca_idx as isize) as *mut NonmovingAllocator;

    let mut current = *(*cap).current_segments.offset(alloca_idx as isize);
    let mut block_count = nonmovingSegmentBlockCount(current);

    let mut ret = nonmovingSegmentGetBlock_(
        current,
        block_size as uint16_t,
        block_count as uint16_t,
        (*current).next_free,
    );

    let mut full = advance_next_free(current, block_count);

    if full {
        let mut new_blocks =
            block_count.wrapping_sub((*nonmovingSegmentInfo(current)).next_free_snap as c_uint);

        atomic_inc(
            &raw mut (*oldest_gen).live_estimate as StgVolatilePtr,
            (new_blocks.wrapping_mul(block_size) as usize).wrapping_div(size_of::<W_>() as usize)
                as StgWord,
        );

        nonmovingPushFilledSegment(current);

        let mut new_current = pop_active_segment(alloca);

        if new_current.is_null() {
            new_current = nonmovingAllocSegment(mode, (*cap).node);
            nonmovingInitSegment(new_current, alloca_idx as uint16_t);
        }

        (*new_current).link = null_mut::<NonmovingSegment>();

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
    (*cap).total_allocated = (*cap).total_allocated.wrapping_add(sz as uint64_t);

    return nonmovingAllocate_(SM_LOCK, cap, sz);
}
