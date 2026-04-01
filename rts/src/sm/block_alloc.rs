use crate::capability::n_numa_nodes;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{BLOCK_SHIFT, MAX_NUMA_NODES, MBLOCK_SHIFT};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::storage::block::{
    BDESCR_SHIFT, BF_KNOWN, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, Bdescr, FIRST_BLOCK_OFF,
    MBLOCK_MASK, MBLOCK_SIZE, allocBlock, allocBlockOnNode, bdescr, bdescr_, dbl_link_onto,
    dbl_link_remove,
};
use crate::ffi::rts::storage::m_block::{
    freeMBlocks, getFirstMBlock, getMBlocks, getMBlocksOnNode, getNextMBlock, releaseFreeMemory,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8, StgWord16, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::prelude::*;
use crate::sm::storage::{clear_blocks, sm_mutex};

#[cfg(test)]
mod tests;

const NUM_FREE_LISTS: i32 = MBLOCK_SHIFT - BLOCK_SHIFT;

static mut free_list: [[*mut bdescr; 8]; 16] = [[null_mut::<bdescr>(); 8]; 16];

static mut free_mblock_list: [*mut bdescr; 16] = [null_mut::<bdescr>(); 16];

static mut defer_mblock_frees: bool = false;

static mut deferred_free_mblock_list: [*mut bdescr; 16] = [null_mut::<bdescr>(); 16];

static mut n_alloc_blocks: W_ = 0;

static mut hw_alloc_blocks: W_ = 0;

static mut n_alloc_blocks_by_node: [W_; 16] = [0; 16];

unsafe fn initBlockAllocator() {
    let mut i: u32 = 0;
    let mut node: u32 = 0;
    node = 0;

    while node < MAX_NUMA_NODES as u32 {
        i = 0;

        while i < NUM_FREE_LISTS as u32 {
            free_list[node as usize][i as usize] = null_mut::<bdescr>();
            i = i.wrapping_add(1);
        }

        free_mblock_list[node as usize] = null_mut::<bdescr>();
        n_alloc_blocks_by_node[node as usize] = 0;
        node = node.wrapping_add(1);
    }

    n_alloc_blocks = 0;
    hw_alloc_blocks = 0;
}

unsafe fn recordAllocatedBlocks(mut node: u32, mut n: u32) {
    n_alloc_blocks = n_alloc_blocks.wrapping_add(n as W_);
    n_alloc_blocks_by_node[node as usize] =
        n_alloc_blocks_by_node[node as usize].wrapping_add(n as W_);

    if n > 0 && n_alloc_blocks > hw_alloc_blocks {
        hw_alloc_blocks = n_alloc_blocks;
    }
}

unsafe fn recordFreedBlocks(mut node: u32, mut n: u32) {
    if (n_alloc_blocks >= n as W_) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 226);
    }

    n_alloc_blocks = n_alloc_blocks.wrapping_sub(n as W_);
    n_alloc_blocks_by_node[node as usize] =
        n_alloc_blocks_by_node[node as usize].wrapping_sub(n as W_);
}

unsafe fn tail_of(mut bd: *mut bdescr) -> *mut bdescr {
    return bd.offset((*bd).blocks as isize).offset(-1);
}

unsafe fn initGroup(mut head: *mut bdescr) {
    (*head).c2rust_unnamed.free = (*head).start;
    (*head).link = null_mut::<bdescr_>();

    if (*head).blocks > 1 && (*head).blocks as W_ <= BLOCKS_PER_MBLOCK {
        let mut last = tail_of(head);
        (*last).blocks = 0;
        (*last).link = head as *mut bdescr_;
    }

    let mut i: u32 = 0;

    while i < (*head).blocks {
        (*head.offset(i as isize)).flags = 0;
        i = i.wrapping_add(1);
    }
}

unsafe fn log_2(mut n: W_) -> u32 {
    if (n > 0 && n < (1 << 20 - 12) as W_) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 276);
    }

    return ((n as u64).leading_zeros() as i32 as usize
        ^ (size_of::<StgWord>() as usize)
            .wrapping_mul(8 as usize)
            .wrapping_sub(1 as usize)) as u32;
}

unsafe fn log_2_ceil(mut n: W_) -> u32 {
    if (n > 0 && n < (1 << 20 - 12) as W_) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 286);
    }

    let mut r = log_2(n);

    return if n & n.wrapping_sub(1 as W_) != 0 {
        r.wrapping_add(1 as u32)
    } else {
        r
    };
}

unsafe fn free_list_insert(mut node: u32, mut bd: *mut bdescr) {
    let mut ln: u32 = 0;

    if (((*bd).blocks as W_)
        < (((1 as u64) << 20 as i32) as W_)
            .wrapping_sub(
                ((0x40 as u64)
                    .wrapping_mul(((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32))
                    as W_)
                    .wrapping_add(((1 as u64) << 12 as i32) as W_)
                    .wrapping_sub(1 as W_)
                    & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
            )
            .wrapping_div(((1 as u64) << 12 as i32) as W_)) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 296);
    }

    ln = log_2((*bd).blocks as W_);

    dbl_link_onto(
        bd,
        (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
            as *mut *mut bdescr)
            .offset(ln as isize) as *mut *mut bdescr,
    );
}

unsafe fn setup_tail(mut bd: *mut bdescr) {
    let mut tail = null_mut::<bdescr>();
    tail = tail_of(bd);

    if tail != bd {
        (*tail).blocks = 0;
        (*tail).c2rust_unnamed.free = null_mut::<StgWord>();
        (*tail).link = bd as *mut bdescr_;
    }
}

unsafe fn split_free_block(
    mut bd: *mut bdescr,
    mut node: u32,
    mut n: W_,
    mut ln: u32,
) -> *mut bdescr {
    let mut fg = null_mut::<bdescr>();

    if ((*bd).blocks as W_ > n) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 325);
    }

    dbl_link_remove(
        bd,
        (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
            as *mut *mut bdescr)
            .offset(ln as isize) as *mut *mut bdescr,
    );

    fg = bd.offset((*bd).blocks as isize).offset(-(n as isize));
    (*fg).blocks = n as StgWord32;
    (*bd).blocks = ((*bd).blocks as W_).wrapping_sub(n) as StgWord32 as StgWord32;
    setup_tail(bd);
    ln = log_2((*bd).blocks as W_);

    dbl_link_onto(
        bd,
        (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
            as *mut *mut bdescr)
            .offset(ln as isize) as *mut *mut bdescr,
    );

    return fg;
}

unsafe fn split_block_high(mut bd: *mut bdescr, mut n: W_) -> *mut bdescr {
    if ((*bd).blocks as W_ > n) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 340);
    }

    let mut ret = bd.offset((*bd).blocks as isize).offset(-(n as isize));
    (*ret).blocks = n as StgWord32;
    (*ret).c2rust_unnamed.free = (*bd).start.offset(
        ((*bd).blocks as W_)
            .wrapping_sub(n)
            .wrapping_mul(BLOCK_SIZE_W as W_) as isize,
    );

    (*ret).start = (*ret).c2rust_unnamed.free;
    (*ret).link = null_mut::<bdescr_>();
    (*bd).blocks = ((*bd).blocks as W_).wrapping_sub(n) as StgWord32 as StgWord32;
    setup_tail(ret);
    setup_tail(bd);
    freeGroup(bd);

    return ret;
}

unsafe fn split_block_low(mut bd: *mut bdescr, mut n: W_) -> *mut bdescr {
    if ((*bd).blocks as W_ > n) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 361);
    }

    let mut bd_ = bd.offset(n as isize);
    (*bd_).blocks = ((*bd).blocks as W_).wrapping_sub(n) as StgWord32;
    (*bd_).c2rust_unnamed.free = (*bd)
        .start
        .offset(n.wrapping_mul(BLOCK_SIZE_W as W_) as isize);
    (*bd_).start = (*bd_).c2rust_unnamed.free;
    (*bd).blocks = n as StgWord32;
    setup_tail(bd_);
    setup_tail(bd);
    freeGroup(bd_);

    return bd;
}

unsafe fn split_block_high_no_free(mut bd: *mut bdescr, mut n: W_) -> *mut bdescr {
    if ((*bd).blocks as W_ > n) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 382);
    }

    let mut ret = bd.offset((*bd).blocks as isize).offset(-(n as isize));
    (*ret).blocks = n as StgWord32;
    (*ret).c2rust_unnamed.free = (*bd).start.offset(
        ((*bd).blocks as W_)
            .wrapping_sub(n)
            .wrapping_mul(BLOCK_SIZE_W as W_) as isize,
    );

    (*ret).start = (*ret).c2rust_unnamed.free;
    (*ret).link = null_mut::<bdescr_>();
    (*bd).blocks = ((*bd).blocks as W_).wrapping_sub(n) as StgWord32 as StgWord32;
    setup_tail(ret);
    setup_tail(bd);

    return ret;
}

unsafe fn allocMBlockAlignedGroupOnNode(mut node: u32, mut n: W_) -> *mut bdescr {
    let mut bd = allocGroupOnNode(node, BLOCKS_PER_MBLOCK);

    if ((*bd).blocks as W_
        == (((1 as u64) << 20 as i32) as W_)
            .wrapping_sub(
                ((0x40 as u64)
                    .wrapping_mul(((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32))
                    as W_)
                    .wrapping_add(((1 as u64) << 12 as i32) as W_)
                    .wrapping_sub(1 as W_)
                    & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
            )
            .wrapping_div(((1 as u64) << 12 as i32) as W_)) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 406);
    }

    bd = split_block_high(
        bd,
        ((*bd).blocks as W_).wrapping_sub(((*bd).blocks as W_).wrapping_rem(n)),
    );

    if (((*bd).blocks as W_).wrapping_rem(n) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 408);
    }

    let mut last = null_mut::<bdescr>();
    let mut chunk = null_mut::<bdescr>();

    while (*bd).blocks as W_ > n {
        chunk = split_block_high_no_free(bd, n);
        (*chunk).link = last as *mut bdescr_;
        last = chunk;
    }

    (*bd).link = chunk as *mut bdescr_;

    return bd;
}

unsafe fn alloc_mega_group_from_free_list(
    mut free_list_head: *mut *mut bdescr,
    mut n: StgWord,
    mut best: *mut *mut bdescr,
) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    *best = null_mut::<bdescr>();
    prev = null_mut::<bdescr>();
    bd = *free_list_head;

    while !bd.is_null() {
        if (*bd).blocks as StgWord == n {
            if !prev.is_null() {
                (*prev).link = (*bd).link;
            } else {
                *free_list_head = (*bd).link as *mut bdescr;
            }

            return bd;
        } else if (*bd).blocks as StgWord > n {
            if (*best).is_null() || (*bd).blocks < (**best).blocks {
                *best = bd;
            }
        }

        prev = bd;
        bd = (*bd).link as *mut bdescr;
    }

    return null_mut::<bdescr>();
}

unsafe fn alloc_mega_group(mut node: u32, mut mblocks: StgWord) -> *mut bdescr {
    let mut best = null_mut::<bdescr>();
    let mut bd = null_mut::<bdescr>();
    let mut n: StgWord = 0;
    n = BLOCKS_PER_MBLOCK.wrapping_add(
        (mblocks as W_)
            .wrapping_sub(1 as W_)
            .wrapping_mul(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_),
    ) as StgWord;

    if defer_mblock_frees {
        bd = alloc_mega_group_from_free_list(
            (&raw mut deferred_free_mblock_list as *mut *mut bdescr).offset(node as isize)
                as *mut *mut bdescr,
            n,
            &raw mut best,
        );

        if !bd.is_null() {
            return bd;
        } else if best.is_null() {
            bd = alloc_mega_group_from_free_list(
                (&raw mut free_mblock_list as *mut *mut bdescr).offset(node as isize)
                    as *mut *mut bdescr,
                n,
                &raw mut best,
            );
        }
    } else {
        bd = alloc_mega_group_from_free_list(
            (&raw mut free_mblock_list as *mut *mut bdescr).offset(node as isize)
                as *mut *mut bdescr,
            n,
            &raw mut best,
        );
    }

    if !bd.is_null() {
        return bd;
    } else if !best.is_null() {
        let mut best_mblocks: StgWord = (1 as StgWord).wrapping_add(
            ((((*best).blocks as W_)
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
                & !MBLOCK_MASK as W_) as *mut c_void as StgWord)
                .wrapping_div(MBLOCK_SIZE as StgWord),
        );

        bd = (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(
            ((best as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_) as *mut c_void
                as *mut StgWord8)
                .offset(
                    best_mblocks
                        .wrapping_sub(mblocks)
                        .wrapping_mul(((1 as u64) << 20 as i32) as StgWord)
                        as isize,
                ) as W_,
        ) as *mut bdescr;

        (*best).blocks = BLOCKS_PER_MBLOCK.wrapping_add(
            (best_mblocks as W_)
                .wrapping_sub(mblocks as W_)
                .wrapping_sub(1 as W_)
                .wrapping_mul(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_),
        ) as StgWord32;

        initMBlock((bd as W_ & !MBLOCK_MASK as W_) as *mut c_void, node);
    } else {
        let mut mblock = null_mut::<c_void>();

        if RtsFlags.GcFlags.numa {
            mblock = getMBlocksOnNode(node, mblocks as u32);
        } else {
            mblock = getMBlocks(mblocks as u32);
        }

        initMBlock(mblock, node);
        bd = (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(mblock as W_)
            as *mut bdescr;
    }

    (*bd).blocks = BLOCKS_PER_MBLOCK.wrapping_add(
        (mblocks as W_)
            .wrapping_sub(1 as W_)
            .wrapping_mul(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_),
    ) as StgWord32;

    return bd;
}

unsafe fn allocGroupOnNode(mut node: u32, mut n: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut rem = null_mut::<bdescr>();
    let mut ln: StgWord = 0;

    if n == 0 {
        barf(c"allocGroup: requested zero blocks".as_ptr());
    }

    if n >= BLOCKS_PER_MBLOCK {
        let mut mblocks: StgWord = 0;
        mblocks = (1 as W_).wrapping_add(
            ((n.wrapping_sub(
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
        ) as StgWord;

        recordAllocatedBlocks(node, mblocks.wrapping_mul(BLOCKS_PER_MBLOCK) as u32);
        bd = alloc_mega_group(node, mblocks);
        initGroup(bd);
    } else {
        recordAllocatedBlocks(node, n as u32);
        ln = log_2_ceil(n) as StgWord;

        while ln < NUM_FREE_LISTS as StgWord && free_list[node as usize][ln as usize].is_null() {
            ln = ln.wrapping_add(1);
        }

        if ln == NUM_FREE_LISTS as StgWord {
            bd = alloc_mega_group(node, 1);
            (*bd).blocks = n as StgWord32;
            initGroup(bd);
            rem = bd.offset(n as isize);
            (*rem).blocks = BLOCKS_PER_MBLOCK.wrapping_sub(n) as StgWord32;
            initGroup(rem);
            recordAllocatedBlocks(node, (*rem).blocks as u32);
            freeGroup(rem);
        } else {
            bd = free_list[node as usize][ln as usize];

            if (*bd).blocks as W_ == n {
                dbl_link_remove(
                    bd,
                    (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
                        as *mut *mut bdescr)
                        .offset(ln as isize) as *mut *mut bdescr,
                );

                initGroup(bd);
            } else if (*bd).blocks as W_ > n {
                bd = split_free_block(bd, node, n, ln as u32);

                if ((*bd).blocks as W_ == n) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 583);
                }

                initGroup(bd);
            } else {
                barf(c"allocGroup: free list corrupted".as_ptr());
            }
        }
    }

    if RtsFlags.DebugFlags.zero_on_gc {
        memset(
            (*bd).start as *mut c_void,
            0xaa,
            ((*bd).blocks as usize).wrapping_mul((1 as usize) << 12 as i32),
        );
    }

    if RtsFlags.DebugFlags.sanity {
        checkFreeListSanity();
    }

    return bd;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocAlignedGroupOnNode(mut node: c_uint, mut n: W_) -> *mut bdescr {
    let mut num_blocks: W_ = (2 as W_).wrapping_mul(n).wrapping_sub(1 as W_);

    if num_blocks >= BLOCKS_PER_MBLOCK {
        barf(
            c"allocAlignedGroupOnNode: allocating megablocks is not supported\n    requested blocks: %llu\n    required for alignment: %llu\n    megablock size (in blocks): %llu"
                .as_ptr(),
            n,
            num_blocks,
            BLOCKS_PER_MBLOCK,
        );
    }

    let mut group_size: W_ = n.wrapping_mul(BLOCK_SIZE as W_);

    let mut max_blocks: W_ = ({
        let mut _a: W_ = (num_blocks as W_).wrapping_mul(3 as W_);
        let mut _b: W_ = (((1 as u64) << 20 as i32) as W_)
            .wrapping_sub(
                ((0x40 as u64)
                    .wrapping_mul(((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32))
                    as W_)
                    .wrapping_add(((1 as u64) << 12 as i32) as W_)
                    .wrapping_sub(1 as W_)
                    & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
            )
            .wrapping_div(((1 as u64) << 12 as i32) as W_)
            .wrapping_sub(1 as W_);

        if _a <= _b { _a } else { _b as W_ }
    });

    let mut bd = allocLargeChunkOnNode(node, num_blocks, max_blocks);
    num_blocks = (*bd).blocks as W_;

    let mut slop_low: W_ = 0;

    if ((*bd).start as usize as W_).wrapping_rem(group_size) != 0 {
        slop_low = group_size.wrapping_sub(((*bd).start as usize as W_).wrapping_rem(group_size));
    }

    let mut slop_high: W_ = num_blocks
        .wrapping_mul(BLOCK_SIZE as W_)
        .wrapping_sub(group_size)
        .wrapping_sub(slop_low);

    if (slop_low.wrapping_rem(((1 as u64) << 12 as i32) as W_) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 650);
    }

    if (slop_high.wrapping_rem(((1 as u64) << 12 as i32) as W_) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 651);
    }

    let mut slop_low_blocks: W_ = slop_low.wrapping_div(BLOCK_SIZE as W_);
    let mut slop_high_blocks: W_ = slop_high.wrapping_div(BLOCK_SIZE as W_);

    if (slop_low_blocks
        .wrapping_add(slop_high_blocks)
        .wrapping_add(n)
        == num_blocks) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 656);
    }

    checkFreeListSanity();

    let mut free_before = countFreeList();

    if slop_low_blocks != 0 {
        bd = split_block_high(bd, num_blocks.wrapping_sub(slop_low_blocks));

        if (countBlocks(bd) == num_blocks.wrapping_sub(slop_low_blocks)) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 665);
        }
    }

    if (countFreeList() == free_before.wrapping_add(slop_low_blocks)) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 669);
    }

    checkFreeListSanity();

    if (((*bd).start as usize as W_).wrapping_rem(group_size) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 674);
    }

    free_before = countFreeList();

    if slop_high_blocks != 0 {
        bd = split_block_low(bd, n);

        if ((*bd).blocks as W_ == n) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 682);
        }
    }

    if (countFreeList() == free_before.wrapping_add(slop_high_blocks)) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 686);
    }

    checkFreeListSanity();

    if (((*bd).start as usize as W_).wrapping_rem(group_size) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 691);
    }

    if (Bdescr((*bd).start) == bd) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 694);
    }

    return bd;
}

unsafe fn nodeWithLeastBlocks() -> u32 {
    let mut node: u32 = 0;
    let mut i: u32 = 0;
    let mut min_blocks: u32 = n_alloc_blocks_by_node[0] as u32;
    i = 1;

    while i < n_numa_nodes {
        if n_alloc_blocks_by_node[i as usize] < min_blocks as W_ {
            min_blocks = n_alloc_blocks_by_node[i as usize] as u32;
            node = i;
        }

        i = i.wrapping_add(1);
    }

    return node;
}

unsafe fn allocGroup(mut n: W_) -> *mut bdescr {
    return allocGroupOnNode(nodeWithLeastBlocks(), n);
}

unsafe fn allocLargeChunkOnNode(mut node: u32, mut min: W_, mut max: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut ln: StgWord = 0;
    let mut lnmax: StgWord = 0;

    if min >= BLOCKS_PER_MBLOCK {
        return allocGroupOnNode(node, max);
    }

    ln = log_2_ceil(min) as StgWord;
    lnmax = log_2_ceil(max) as StgWord;

    while ln < NUM_FREE_LISTS as StgWord
        && ln < lnmax
        && free_list[node as usize][ln as usize].is_null()
    {
        ln = ln.wrapping_add(1);
    }

    if ln == NUM_FREE_LISTS as StgWord || ln == lnmax {
        return allocGroupOnNode(node, max);
    }

    bd = free_list[node as usize][ln as usize];

    if (*bd).blocks as W_ <= max {
        dbl_link_remove(
            bd,
            (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
                as *mut *mut bdescr)
                .offset(ln as isize) as *mut *mut bdescr,
        );

        initGroup(bd);
    } else {
        bd = split_free_block(bd, node, max, ln as u32);

        if ((*bd).blocks as W_ == max) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 763);
        }

        initGroup(bd);
    }

    recordAllocatedBlocks(node, (*bd).blocks as u32);

    if RtsFlags.DebugFlags.zero_on_gc {
        memset(
            (*bd).start as *mut c_void,
            0xaa,
            ((*bd).blocks as usize).wrapping_mul((1 as usize) << 12 as i32),
        );
    }

    if RtsFlags.DebugFlags.sanity {
        checkFreeListSanity();
    }

    return bd;
}

unsafe fn allocLargeChunk(mut min: W_, mut max: W_) -> *mut bdescr {
    return allocLargeChunkOnNode(nodeWithLeastBlocks(), min, max);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocGroup_lock(mut n: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            783,
            __r,
        );
    }

    bd = allocGroup(n);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            785,
        );
    }

    return bd;
}

unsafe fn allocBlock_lock() -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            793,
            __r,
        );
    }

    bd = allocBlock();

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            795,
        );
    }

    return bd;
}

unsafe fn allocGroupOnNode_lock(mut node: u32, mut n: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            803,
            __r,
        );
    }

    bd = allocGroupOnNode(node, n);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            805,
        );
    }

    return bd;
}

unsafe fn allocBlockOnNode_lock(mut node: u32) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            813,
            __r,
        );
    }

    bd = allocBlockOnNode(node);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            815,
        );
    }

    return bd;
}

unsafe fn coalesce_mblocks(mut p: *mut bdescr) -> *mut bdescr {
    let mut q = null_mut::<bdescr>();
    q = (*p).link as *mut bdescr;

    if !q.is_null()
        && (q as W_ & !MBLOCK_MASK as W_) as *mut c_void
            == ((p as W_ & !MBLOCK_MASK as W_) as *mut c_void as *mut StgWord8).offset(
                (1 as W_)
                    .wrapping_add(
                        ((((*p).blocks as W_)
                            .wrapping_sub(
                                (((1 as u64) << 20 as i32) as W_)
                                    .wrapping_sub(
                                        ((0x40 as u64).wrapping_mul(
                                            ((1 as u64) << 20 as i32)
                                                .wrapping_div((1 as u64) << 12 as i32),
                                        ) as W_)
                                            .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                            .wrapping_sub(1 as W_)
                                            & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64)
                                                as W_,
                                    )
                                    .wrapping_div(((1 as u64) << 12 as i32) as W_),
                            )
                            .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                            .wrapping_add(MBLOCK_SIZE as W_)
                            .wrapping_sub(1 as W_)
                            & !MBLOCK_MASK as W_) as *mut c_void as W_)
                            .wrapping_div(MBLOCK_SIZE as W_),
                    )
                    .wrapping_mul(MBLOCK_SIZE as W_) as isize,
            ) as *mut c_void
    {
        (*p).blocks = BLOCKS_PER_MBLOCK.wrapping_add(
            (1 as W_)
                .wrapping_add(
                    ((((*p).blocks as W_)
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
                        .wrapping_add(((1 as u64) << 20 as i32) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                        as *mut c_void as W_)
                        .wrapping_div(((1 as u64) << 20 as i32) as W_),
                )
                .wrapping_add(
                    (1 as W_).wrapping_add(
                        ((((*q).blocks as W_)
                            .wrapping_sub(
                                (((1 as u64) << 20 as i32) as W_)
                                    .wrapping_sub(
                                        ((0x40 as u64).wrapping_mul(
                                            ((1 as u64) << 20 as i32)
                                                .wrapping_div((1 as u64) << 12 as i32),
                                        ) as W_)
                                            .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                            .wrapping_sub(1 as W_)
                                            & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64)
                                                as W_,
                                    )
                                    .wrapping_div(((1 as u64) << 12 as i32) as W_),
                            )
                            .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                            .wrapping_add(((1 as u64) << 20 as i32) as W_)
                            .wrapping_sub(1 as W_)
                            & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                            as *mut c_void as W_)
                            .wrapping_div(((1 as u64) << 20 as i32) as W_),
                    ),
                )
                .wrapping_sub(1 as W_)
                .wrapping_mul(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_),
        ) as StgWord32;

        (*p).link = (*q).link;

        return p;
    }

    return q;
}

unsafe fn free_mega_group(mut mg: *mut bdescr) {
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    let mut node: u32 = 0;
    node = (*mg).node as u32;

    if defer_mblock_frees {
        (*mg).link = deferred_free_mblock_list[node as usize] as *mut bdescr_;
        deferred_free_mblock_list[node as usize] = mg;
    } else {
        prev = null_mut::<bdescr>();
        bd = free_mblock_list[node as usize];

        while !bd.is_null() && (*bd).start < (*mg).start {
            prev = bd;
            bd = (*bd).link as *mut bdescr;
        }

        if !prev.is_null() {
            (*mg).link = (*prev).link;
            (*prev).link = mg as *mut bdescr_;
            mg = coalesce_mblocks(prev);
        } else {
            (*mg).link = free_mblock_list[node as usize] as *mut bdescr_;
            free_mblock_list[node as usize] = mg;
        }

        coalesce_mblocks(mg);

        if RtsFlags.DebugFlags.sanity {
            checkFreeListSanity();
        }
    };
}

unsafe fn free_deferred_mega_groups(mut node: u32) {
    let mut mg = null_mut::<bdescr>();
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    let mut new_head = null_mut::<bdescr>();

    sortDeferredList(
        (&raw mut deferred_free_mblock_list as *mut *mut bdescr).offset(node as isize)
            as *mut *mut bdescr,
    );

    new_head = deferred_free_mblock_list[node as usize];
    deferred_free_mblock_list[node as usize] = null_mut::<bdescr>();
    prev = null_mut::<bdescr>();
    bd = free_mblock_list[node as usize];

    while !new_head.is_null() {
        mg = new_head;
        new_head = (*new_head).link as *mut bdescr;

        while !bd.is_null() && (*bd).start < (*mg).start {
            prev = bd;
            bd = (*bd).link as *mut bdescr;
        }

        if !prev.is_null() {
            (*mg).link = (*prev).link;
            (*prev).link = mg as *mut bdescr_;
            mg = coalesce_mblocks(prev);
        } else {
            (*mg).link = free_mblock_list[node as usize] as *mut bdescr_;
            free_mblock_list[node as usize] = mg;
        }

        coalesce_mblocks(mg);
        prev = mg;
        bd = (*prev).link as *mut bdescr;
    }

    if RtsFlags.DebugFlags.sanity {
        checkFreeListSanity();
    }
}

unsafe fn freeGroup(mut p: *mut bdescr) {
    let mut ln: StgWord = 0;
    let mut node: u32 = 0;

    if ((&raw mut (*p).c2rust_unnamed.free).load(Ordering::Relaxed) != -1 as P_) as i32 as i64 != 0
    {
    } else {
        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 966);
    }

    let mut i: u32 = 0;

    while i < (*p).blocks {
        (*p.offset(i as isize)).flags = 0;
        i = i.wrapping_add(1);
    }

    node = (*p).node as u32;
    (&raw mut (*p).c2rust_unnamed.free).store(-1 as *mut c_void as StgPtr, Ordering::Relaxed);
    (&raw mut (*p).r#gen).store(null_mut::<generation_>(), Ordering::Relaxed);
    (&raw mut (*p).gen_no).store(0, Ordering::Relaxed);

    if RtsFlags.DebugFlags.zero_on_gc {
        memset(
            (*p).start as *mut c_void,
            0xaa,
            ((*p).blocks as W_).wrapping_mul(((1 as u64) << 12 as i32) as W_) as usize,
        );
    }

    if (*p).blocks == 0 {
        barf(c"freeGroup: block size is zero".as_ptr());
    }

    if (*p).blocks as W_ >= BLOCKS_PER_MBLOCK {
        let mut mblocks: StgWord = 0;
        mblocks = (1 as W_).wrapping_add(
            ((((*p).blocks as W_)
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
        ) as StgWord;

        if ((*p).blocks as StgWord
            == (((1 as u64) << 20 as i32) as W_)
                .wrapping_sub(
                    ((0x40 as u64).wrapping_mul(
                        ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                    ) as W_)
                        .wrapping_add(((1 as u64) << 12 as i32) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                )
                .wrapping_div(((1 as u64) << 12 as i32) as W_)
                .wrapping_add((mblocks as W_).wrapping_sub(1 as W_).wrapping_mul(
                    ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32) as W_,
                ))) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 990);
        }

        recordFreedBlocks(node, mblocks.wrapping_mul(BLOCKS_PER_MBLOCK) as u32);
        free_mega_group(p);
        return;
    }

    recordFreedBlocks(node, (*p).blocks as u32);

    let mut next = null_mut::<bdescr>();
    next = p.offset((*p).blocks as isize);

    if next
        <= ((MBLOCK_SIZE.wrapping_sub(BLOCK_SIZE) >> BLOCK_SHIFT - BDESCR_SHIFT) as W_)
            .wrapping_add(
                (p as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_) as *mut c_void
                    as W_,
            ) as *mut bdescr
        && (&raw mut (*next).c2rust_unnamed.free).load(Ordering::Relaxed) == -1 as P_
    {
        (*p).blocks = (*p).blocks.wrapping_add((*next).blocks);
        ln = log_2((*next).blocks as W_) as StgWord;

        dbl_link_remove(
            next,
            (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
                as *mut *mut bdescr)
                .offset(ln as isize) as *mut *mut bdescr,
        );

        if (*p).blocks as W_ == BLOCKS_PER_MBLOCK {
            free_mega_group(p);
            return;
        }

        setup_tail(p);
    }

    if p != (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(
        (p as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_) as *mut c_void as W_,
    ) as *mut bdescr
    {
        let mut prev = null_mut::<bdescr>();
        prev = p.offset(-1);

        if (*prev).blocks == 0 {
            prev = (*prev).link as *mut bdescr;
        }

        if (&raw mut (*prev).c2rust_unnamed.free).load(Ordering::Relaxed) == -1 as P_ {
            ln = log_2((*prev).blocks as W_) as StgWord;

            dbl_link_remove(
                prev,
                (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
                    as *mut *mut bdescr)
                    .offset(ln as isize) as *mut *mut bdescr,
            );

            (*prev).blocks = (*prev).blocks.wrapping_add((*p).blocks);

            if (*prev).blocks as W_ >= BLOCKS_PER_MBLOCK {
                free_mega_group(prev);
                return;
            }

            p = prev;
        }
    }

    setup_tail(p);
    free_list_insert(node, p);

    if RtsFlags.DebugFlags.sanity {
        checkFreeListSanity();
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeGroup_lock(mut p: *mut bdescr) {
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            1054,
            __r,
        );
    }

    freeGroup(p);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            1056,
        );
    }
}

unsafe fn freeChain(mut bd: *mut bdescr) {
    let mut next_bd = null_mut::<bdescr>();

    while !bd.is_null() {
        next_bd = (*bd).link as *mut bdescr;
        freeGroup(bd);
        bd = next_bd;
    }
}

unsafe fn freeChain_lock(mut bd: *mut bdescr) {
    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            1073,
            __r,
        );
    }

    freeChain(bd);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/BlockAlloc.c".as_ptr(),
            1075,
        );
    }
}

unsafe fn initMBlock(mut mblock: *mut c_void, mut node: u32) {
    let mut bd = null_mut::<bdescr>();
    let mut block = null_mut::<StgWord8>();
    block = FIRST_BLOCK_OFF.wrapping_add(mblock as W_) as *mut c_void as *mut StgWord8;
    bd = (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(mblock as W_) as *mut bdescr;

    while block
        <= (MBLOCK_SIZE.wrapping_sub(BLOCK_SIZE) as W_).wrapping_add(mblock as W_) as *mut c_void
            as *mut StgWord8
    {
        (*bd).start = block as *mut c_void as StgPtr;
        (*bd).node = node as StgWord16;
        bd = bd.offset(1);
        block = block.offset(BLOCK_SIZE as isize);
    }
}

unsafe fn splitDeferredList(mut head: *mut bdescr) -> *mut bdescr {
    let mut fast = null_mut::<bdescr>();
    let mut slow = null_mut::<bdescr>();
    let mut second_half = null_mut::<bdescr>();
    slow = head;
    fast = (*slow).link as *mut bdescr;

    while !fast.is_null() {
        fast = (*fast).link as *mut bdescr;

        if !fast.is_null() {
            fast = (*fast).link as *mut bdescr;
            slow = (*slow).link as *mut bdescr;
        }
    }

    second_half = (*slow).link as *mut bdescr;
    (*slow).link = null_mut::<bdescr_>();

    return second_half;
}

unsafe fn sortDeferredList(mut head: *mut *mut bdescr) {
    let mut first_half = null_mut::<bdescr>();
    let mut second_half = null_mut::<bdescr>();
    let mut cur = null_mut::<bdescr>();

    if (*head).is_null() || (**head).link.is_null() {
        return;
    }

    first_half = *head;
    second_half = splitDeferredList(*head);
    sortDeferredList(&raw mut first_half);
    sortDeferredList(&raw mut second_half);

    if (*first_half).start < (*second_half).start {
        *head = first_half;
        first_half = (*first_half).link as *mut bdescr;
    } else {
        *head = second_half;
        second_half = (*second_half).link as *mut bdescr;
    }

    cur = *head;

    while !first_half.is_null() && !second_half.is_null() {
        if (*first_half).start < (*second_half).start {
            (*cur).link = first_half as *mut bdescr_;
            first_half = (*first_half).link as *mut bdescr;
        } else {
            (*cur).link = second_half as *mut bdescr_;
            second_half = (*second_half).link as *mut bdescr;
        }

        cur = (*cur).link as *mut bdescr;
    }

    while !first_half.is_null() {
        (*cur).link = first_half as *mut bdescr_;
        first_half = (*first_half).link as *mut bdescr;
        cur = (*cur).link as *mut bdescr;
    }

    while !second_half.is_null() {
        (*cur).link = second_half as *mut bdescr_;
        second_half = (*second_half).link as *mut bdescr;
        cur = (*cur).link as *mut bdescr;
    }
}

unsafe fn deferMBlockFreeing() {
    if defer_mblock_frees {
        barf(c"MBlock freeing is already deferred".as_ptr());
    }

    defer_mblock_frees = true;
}

unsafe fn commitMBlockFreeing() {
    if !defer_mblock_frees {
        barf(c"MBlock freeing was never deferred".as_ptr());
    }

    defer_mblock_frees = false;

    let mut node: u32 = 0;

    while node < n_numa_nodes {
        free_deferred_mega_groups(node);
        node = node.wrapping_add(1);
    }
}

unsafe fn countBlocks(mut bd: *mut bdescr) -> W_ {
    let mut n: W_ = 0;
    n = 0;

    while !bd.is_null() {
        n = n.wrapping_add((*bd).blocks as W_);
        bd = (*bd).link as *mut bdescr;
    }

    return n;
}

unsafe fn countAllocdBlocks(mut bd: *mut bdescr) -> W_ {
    let mut n: W_ = 0;
    n = 0;

    while !bd.is_null() {
        n = n.wrapping_add((*bd).blocks as W_);

        if (*bd).blocks as W_ > BLOCKS_PER_MBLOCK {
            n = n.wrapping_sub(
                (MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_)
                    .wrapping_sub(BLOCKS_PER_MBLOCK)
                    .wrapping_mul(
                        ((*bd).blocks as u64).wrapping_div(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE))
                            as W_,
                    ),
            );
        }

        bd = (*bd).link as *mut bdescr;
    }

    return n;
}

unsafe fn returnMemoryToOS(mut n: u32) -> u32 {
    let mut bd = null_mut::<bdescr>();
    let mut node: u32 = 0;
    let mut size: StgWord = 0;
    let mut init_n: u32 = 0;
    init_n = n;
    node = 0;

    while n > 0 && node < n_numa_nodes {
        bd = free_mblock_list[node as usize];

        while n > 0 && !bd.is_null() {
            size = (1 as W_).wrapping_add(
                ((((*bd).blocks as W_)
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
            ) as StgWord;

            if size > n as StgWord {
                let mut newSize: StgWord = size.wrapping_sub(n as StgWord);
                let mut freeAddr =
                    ((*bd).start as W_ & !MBLOCK_MASK as W_) as *mut c_void as *mut c_char;
                freeAddr = freeAddr.offset(newSize.wrapping_mul(MBLOCK_SIZE as StgWord) as isize);
                (*bd).blocks = BLOCKS_PER_MBLOCK.wrapping_add(
                    (newSize as W_)
                        .wrapping_sub(1 as W_)
                        .wrapping_mul(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_),
                ) as StgWord32;

                freeMBlocks(freeAddr as *mut c_void, n);
                n = 0;
            } else {
                let mut freeAddr_0 =
                    ((*bd).start as W_ & !MBLOCK_MASK as W_) as *mut c_void as *mut c_char;
                n = (n as StgWord).wrapping_sub(size) as u32 as u32;
                bd = (*bd).link as *mut bdescr;
                freeMBlocks(freeAddr_0 as *mut c_void, size as u32);
            }
        }

        free_mblock_list[node as usize] = bd;
        node = node.wrapping_add(1);
    }

    releaseFreeMemory();

    if RtsFlags.DebugFlags.gc {
        if n != 0 {
            debugBelch(
                c"Wanted to free %d more MBlocks than are freeable\n".as_ptr(),
                n,
            );
        }
    }

    return init_n.wrapping_sub(n);
}

unsafe fn check_tail(mut bd: *mut bdescr) {
    let mut tail = tail_of(bd);

    if tail != bd {
        if ((*tail).blocks == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1296);
        }

        if (*tail).c2rust_unnamed.free.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1297);
        }

        if ((*tail).link == bd) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1298);
        }
    }
}

unsafe fn checkFreeListSanity() {
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    let mut ln: StgWord = 0;
    let mut min: StgWord = 0;
    let mut node: u32 = 0;
    node = 0;

    while node < n_numa_nodes {
        min = 1;
        ln = 0;

        while ln < NUM_FREE_LISTS as StgWord {
            if RtsFlags.DebugFlags.block_alloc {
                debugBelch(c"free block list [%llu]:\n".as_ptr(), ln);
            }

            prev = null_mut::<bdescr>();
            bd = free_list[node as usize][ln as usize];

            while !bd.is_null() {
                if RtsFlags.DebugFlags.block_alloc {
                    debugBelch(
                        c"group at %p, length %ld blocks\n".as_ptr(),
                        (*bd).start,
                        (*bd).blocks as i64,
                    );
                }

                if ((*bd).c2rust_unnamed.free == -1 as P_) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1321);
                }

                if ((*bd).blocks > 0
                    && ((*bd).blocks as W_)
                        < (((1 as u64) << 20 as i32) as W_)
                            .wrapping_sub(
                                ((0x40 as u64).wrapping_mul(
                                    ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                                ) as W_)
                                    .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                    .wrapping_sub(1 as W_)
                                    & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                            )
                            .wrapping_div(((1 as u64) << 12 as i32) as W_))
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1322);
                }

                if ((*bd).blocks as StgWord >= min
                    && (*bd).blocks as StgWord
                        <= min.wrapping_mul(2 as StgWord).wrapping_sub(1 as StgWord))
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1323);
                }

                if ((*bd).link != bd) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1324);
                }

                if ((*bd).node as u32 == node) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1325);
                }

                check_tail(bd);

                if !prev.is_null() {
                    if ((*bd).u.back == prev) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1330);
                    }
                } else if (*bd).u.back.is_null() as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1332);
                }

                let mut next = null_mut::<bdescr>();
                next = bd.offset((*bd).blocks as isize);

                if next
                    <= ((MBLOCK_SIZE.wrapping_sub(BLOCK_SIZE) >> BLOCK_SHIFT - BDESCR_SHIFT) as W_)
                        .wrapping_add(
                            (bd as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                                as *mut c_void as W_,
                        ) as *mut bdescr
                {
                    if ((*next).c2rust_unnamed.free != -1 as P_) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1339);
                    }
                }

                prev = bd;
                bd = (*bd).link as *mut bdescr;
            }

            min = min << 1;
            ln = ln.wrapping_add(1);
        }

        prev = null_mut::<bdescr>();
        bd = free_mblock_list[node as usize];

        while !bd.is_null() {
            if RtsFlags.DebugFlags.block_alloc {
                debugBelch(
                    c"mega group at %p, length %ld blocks\n".as_ptr(),
                    (*bd).start,
                    (*bd).blocks as i64,
                );
            }

            if ((*bd).link != bd) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1353);
            }

            if !(*bd).link.is_null() {
                if ((*bd).start < (*(*bd).link).start) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1358);
                }
            }

            if ((*bd).blocks as W_
                >= (((1 as u64) << 20 as i32) as W_)
                    .wrapping_sub(
                        ((0x40 as u64).wrapping_mul(
                            ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                        ) as W_)
                            .wrapping_add(((1 as u64) << 12 as i32) as W_)
                            .wrapping_sub(1 as W_)
                            & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                    )
                    .wrapping_div(((1 as u64) << 12 as i32) as W_)) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1361);
            }

            if ((((1 as u64) << 20 as i32) as W_)
                .wrapping_sub(
                    ((0x40 as u64).wrapping_mul(
                        ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32),
                    ) as W_)
                        .wrapping_add(((1 as u64) << 12 as i32) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as W_,
                )
                .wrapping_div(((1 as u64) << 12 as i32) as W_)
                .wrapping_add(
                    (1 as W_)
                        .wrapping_add(
                            ((((*bd).blocks as W_)
                                .wrapping_sub(
                                    (((1 as u64) << 20 as i32) as W_)
                                        .wrapping_sub(
                                            ((0x40 as u64).wrapping_mul(
                                                ((1 as u64) << 20 as i32)
                                                    .wrapping_div((1 as u64) << 12 as i32),
                                            ) as W_)
                                                .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                                .wrapping_sub(1 as W_)
                                                & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64)
                                                    as W_,
                                        )
                                        .wrapping_div(((1 as u64) << 12 as i32) as W_),
                                )
                                .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                                .wrapping_add(((1 as u64) << 20 as i32) as W_)
                                .wrapping_sub(1 as W_)
                                & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                                as *mut c_void as W_)
                                .wrapping_div(((1 as u64) << 20 as i32) as W_),
                        )
                        .wrapping_sub(1 as W_)
                        .wrapping_mul(
                            ((1 as u64) << 20 as i32).wrapping_div((1 as u64) << 12 as i32) as W_,
                        ),
                )
                == (*bd).blocks as W_) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1363);
            }

            if !(*bd).link.is_null() {
                if (((*bd).link as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                    as *mut c_void
                    != ((bd as W_ & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                        as *mut c_void as *mut StgWord8)
                        .offset(
                            (1 as W_)
                                .wrapping_add(
                                    ((((*bd).blocks as W_)
                                        .wrapping_sub(
                                            (((1 as u64) << 20 as i32) as W_)
                                                .wrapping_sub(
                                                    ((0x40 as u64).wrapping_mul(
                                                        ((1 as u64) << 20 as i32)
                                                            .wrapping_div((1 as u64) << 12 as i32),
                                                    )
                                                        as W_)
                                                        .wrapping_add(
                                                            ((1 as u64) << 12 as i32) as W_,
                                                        )
                                                        .wrapping_sub(1 as W_)
                                                        & !((1 as u64) << 12 as i32)
                                                            .wrapping_sub(1 as u64)
                                                            as W_,
                                                )
                                                .wrapping_div(((1 as u64) << 12 as i32) as W_),
                                        )
                                        .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                                        .wrapping_add(((1 as u64) << 20 as i32) as W_)
                                        .wrapping_sub(1 as W_)
                                        & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_)
                                        as *mut c_void as W_)
                                        .wrapping_div(((1 as u64) << 20 as i32) as W_),
                                )
                                .wrapping_mul(((1 as u64) << 20 as i32) as W_)
                                as isize,
                        ) as *mut c_void) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/BlockAlloc.c".as_ptr(), 1370);
                }
            }

            prev = bd;
            bd = (*bd).link as *mut bdescr;
        }

        node = node.wrapping_add(1);
    }
}

unsafe fn countFreeList() -> W_ {
    let mut bd = null_mut::<bdescr>();
    let mut total_blocks: W_ = 0;
    let mut ln: StgWord = 0;
    let mut node: u32 = 0;
    node = 0;

    while node < n_numa_nodes {
        ln = 0;

        while ln < NUM_FREE_LISTS as StgWord {
            bd = free_list[node as usize][ln as usize];

            while !bd.is_null() {
                total_blocks = total_blocks.wrapping_add((*bd).blocks as W_);
                bd = (*bd).link as *mut bdescr;
            }

            ln = ln.wrapping_add(1);
        }

        bd = free_mblock_list[node as usize];

        while !bd.is_null() {
            total_blocks = total_blocks.wrapping_add(
                BLOCKS_PER_MBLOCK.wrapping_mul(
                    (1 as W_).wrapping_add(
                        ((((*bd).blocks as W_)
                            .wrapping_sub(
                                (((1 as u64) << 20 as i32) as W_)
                                    .wrapping_sub(
                                        ((0x40 as u64).wrapping_mul(
                                            ((1 as u64) << 20 as i32)
                                                .wrapping_div((1 as u64) << 12 as i32),
                                        ) as W_)
                                            .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                            .wrapping_sub(1 as W_)
                                            & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64)
                                                as W_,
                                    )
                                    .wrapping_div(((1 as u64) << 12 as i32) as W_),
                            )
                            .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                            .wrapping_add(MBLOCK_SIZE as W_)
                            .wrapping_sub(1 as W_)
                            & !MBLOCK_MASK as W_) as *mut c_void as W_)
                            .wrapping_div(MBLOCK_SIZE as W_),
                    ),
                ),
            );

            bd = (*bd).link as *mut bdescr;
        }

        node = node.wrapping_add(1);
    }

    return total_blocks;
}

unsafe fn markBlocks(mut bd: *mut bdescr) {
    while !bd.is_null() {
        (*bd).flags = ((*bd).flags as i32 | BF_KNOWN) as StgWord16;
        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn reportUnmarkedBlocks() {
    let mut mblock = null_mut::<c_void>();
    let mut state = null_mut::<c_void>();
    let mut bd = null_mut::<bdescr>();
    debugBelch(c"Unreachable blocks:\n".as_ptr());
    mblock = getFirstMBlock(&raw mut state);

    while !mblock.is_null() {
        bd = (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(mblock as W_)
            as *mut bdescr;

        while bd
            <= ((MBLOCK_SIZE.wrapping_sub(BLOCK_SIZE) >> BLOCK_SHIFT - BDESCR_SHIFT) as W_)
                .wrapping_add(mblock as W_) as *mut bdescr
        {
            if (*bd).flags as i32 & BF_KNOWN == 0 && (*bd).c2rust_unnamed.free != -1 as P_ {
                debugBelch(c"  %p\n".as_ptr(), bd);
            }

            if (*bd).blocks as W_ >= BLOCKS_PER_MBLOCK {
                mblock = (mblock as *mut StgWord8).offset(
                    (1 as W_)
                        .wrapping_add(
                            ((((*bd).blocks as W_)
                                .wrapping_sub(
                                    (((1 as u64) << 20 as i32) as W_)
                                        .wrapping_sub(
                                            ((0x40 as u64).wrapping_mul(
                                                ((1 as u64) << 20 as i32)
                                                    .wrapping_div((1 as u64) << 12 as i32),
                                            ) as W_)
                                                .wrapping_add(((1 as u64) << 12 as i32) as W_)
                                                .wrapping_sub(1 as W_)
                                                & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64)
                                                    as W_,
                                        )
                                        .wrapping_div(((1 as u64) << 12 as i32) as W_),
                                )
                                .wrapping_mul(((1 as u64) << 12 as i32) as W_)
                                .wrapping_add(MBLOCK_SIZE as W_)
                                .wrapping_sub(1 as W_)
                                & !MBLOCK_MASK as W_) as *mut c_void
                                as W_)
                                .wrapping_div(MBLOCK_SIZE as W_),
                        )
                        .wrapping_sub(1 as W_)
                        .wrapping_mul(MBLOCK_SIZE as W_) as isize,
                ) as *mut c_void;

                break;
            } else {
                bd = bd.offset((*bd).blocks as isize);
            }
        }

        mblock = getNextMBlock(&raw mut state, mblock);
    }
}

unsafe fn clear_free_list() {
    let mut node: u32 = 0;

    while node < n_numa_nodes {
        let mut bd = free_mblock_list[node as usize];

        while !bd.is_null() {
            clear_blocks(bd);
            bd = (*bd).link as *mut bdescr;
        }

        let mut ln = 0;

        while ln < NUM_FREE_LISTS {
            let mut bd_0 = free_list[node as usize][ln as usize];

            while !bd_0.is_null() {
                clear_blocks(bd_0);
                bd_0 = (*bd_0).link as *mut bdescr;
            }

            ln += 1;
        }

        node = node.wrapping_add(1);
    }
}
