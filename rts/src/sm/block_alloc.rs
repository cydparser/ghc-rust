use crate::capability::n_numa_nodes;
use crate::ffi::rts::constants::{BLOCK_SHIFT, MAX_NUMA_NODES, MBLOCK_SHIFT};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::{
    BDESCR_SHIFT, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, FIRST_BLOCK_OFF, MBLOCK_MASK,
    MBLOCK_SIZE, allocBlock, allocBlockOnNode, bdescr, bdescr_, dbl_link_onto, dbl_link_remove,
};
use crate::ffi::rts::storage::m_block::{
    freeMBlocks, getMBlocks, getMBlocksOnNode, releaseFreeMemory,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8, StgWord16, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::prelude::*;
use crate::sm::storage::clear_blocks;

#[cfg(test)]
mod tests;

const NUM_FREE_LISTS: c_int = MBLOCK_SHIFT - BLOCK_SHIFT;

static mut free_list: [[*mut bdescr; 8]; 16] = [[null::<bdescr>() as *mut bdescr; 8]; 16];

static mut free_mblock_list: [*mut bdescr; 16] = [null::<bdescr>() as *mut bdescr; 16];

static mut defer_mblock_frees: bool = false;

static mut deferred_free_mblock_list: [*mut bdescr; 16] = [null::<bdescr>() as *mut bdescr; 16];

static mut n_alloc_blocks: W_ = 0;

static mut hw_alloc_blocks: W_ = 0;

static mut n_alloc_blocks_by_node: [W_; 16] = [0; 16];

unsafe fn initBlockAllocator() {
    let mut i: uint32_t = 0;
    let mut node: uint32_t = 0;
    node = 0 as uint32_t;

    while node < MAX_NUMA_NODES as uint32_t {
        i = 0 as uint32_t;

        while i < NUM_FREE_LISTS as uint32_t {
            free_list[node as usize][i as usize] = null_mut::<bdescr>();
            i = i.wrapping_add(1);
        }

        free_mblock_list[node as usize] = null_mut::<bdescr>();
        n_alloc_blocks_by_node[node as usize] = 0 as W_;
        node = node.wrapping_add(1);
    }

    n_alloc_blocks = 0 as W_;
    hw_alloc_blocks = 0 as W_;
}

#[inline]
unsafe fn recordAllocatedBlocks(mut node: uint32_t, mut n: uint32_t) {
    n_alloc_blocks = n_alloc_blocks.wrapping_add(n as W_);
    n_alloc_blocks_by_node[node as usize] =
        n_alloc_blocks_by_node[node as usize].wrapping_add(n as W_);

    if n > 0 as uint32_t && n_alloc_blocks > hw_alloc_blocks {
        hw_alloc_blocks = n_alloc_blocks;
    }
}

#[inline]
unsafe fn recordFreedBlocks(mut node: uint32_t, mut n: uint32_t) {
    n_alloc_blocks = n_alloc_blocks.wrapping_sub(n as W_);
    n_alloc_blocks_by_node[node as usize] =
        n_alloc_blocks_by_node[node as usize].wrapping_sub(n as W_);
}

#[inline]
unsafe fn tail_of(mut bd: *mut bdescr) -> *mut bdescr {
    return bd
        .offset((*bd).blocks as isize)
        .offset(-(1 as c_int as isize));
}

#[inline]
unsafe fn initGroup(mut head: *mut bdescr) {
    (*head).c2rust_unnamed.free = (*head).start;
    (*head).link = null_mut::<bdescr_>();

    if (*head).blocks > 1 as StgWord32 && (*head).blocks as W_ <= BLOCKS_PER_MBLOCK {
        let mut last = tail_of(head);
        (*last).blocks = 0 as StgWord32;
        (*last).link = head as *mut bdescr_;
    }
}

#[inline]
unsafe fn log_2(mut n: W_) -> uint32_t {
    return ((n as c_ulong).leading_zeros() as i32 as usize
        ^ (size_of::<StgWord>() as usize)
            .wrapping_mul(8 as usize)
            .wrapping_sub(1 as usize)) as uint32_t;
}

#[inline]
unsafe fn log_2_ceil(mut n: W_) -> uint32_t {
    let mut r = log_2(n);

    return if n & n.wrapping_sub(1 as W_) != 0 {
        r.wrapping_add(1 as uint32_t)
    } else {
        r
    };
}

#[inline]
unsafe fn free_list_insert(mut node: uint32_t, mut bd: *mut bdescr) {
    let mut ln: uint32_t = 0;
    ln = log_2((*bd).blocks as W_);

    dbl_link_onto(
        bd,
        (&raw mut *(&raw mut free_list as *mut [*mut bdescr; 8]).offset(node as isize)
            as *mut *mut bdescr)
            .offset(ln as isize) as *mut *mut bdescr,
    );
}

#[inline]
unsafe fn setup_tail(mut bd: *mut bdescr) {
    let mut tail = null_mut::<bdescr>();
    tail = tail_of(bd);

    if tail != bd {
        (*tail).blocks = 0 as StgWord32;
        (*tail).c2rust_unnamed.free = null_mut::<StgWord>();
        (*tail).link = bd as *mut bdescr_;
    }
}

unsafe fn split_free_block(
    mut bd: *mut bdescr,
    mut node: uint32_t,
    mut n: W_,
    mut ln: uint32_t,
) -> *mut bdescr {
    let mut fg = null_mut::<bdescr>();

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

unsafe fn allocMBlockAlignedGroupOnNode(mut node: uint32_t, mut n: W_) -> *mut bdescr {
    let mut bd = allocGroupOnNode(node, BLOCKS_PER_MBLOCK);

    bd = split_block_high(
        bd,
        ((*bd).blocks as W_).wrapping_sub(((*bd).blocks as W_).wrapping_rem(n)),
    );

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

unsafe fn alloc_mega_group(mut node: uint32_t, mut mblocks: StgWord) -> *mut bdescr {
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
                    (((1 as c_ulong) << 20 as c_int) as W_)
                        .wrapping_sub(
                            ((0x40 as c_ulong).wrapping_mul(
                                ((1 as c_ulong) << 20 as c_int)
                                    .wrapping_div((1 as c_ulong) << 12 as c_int),
                            ) as W_)
                                .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                .wrapping_sub(1 as W_)
                                & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong) as W_,
                        )
                        .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                )
                .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
                .wrapping_add(MBLOCK_SIZE as W_)
                .wrapping_sub(1 as W_)
                & !MBLOCK_MASK as W_) as *mut c_void as StgWord)
                .wrapping_div(MBLOCK_SIZE as StgWord),
        );

        bd = (FIRST_BLOCK_OFF >> BLOCK_SHIFT - BDESCR_SHIFT).wrapping_add(
            ((best as W_ & !((1 as c_ulong) << 20 as c_int).wrapping_sub(1 as c_ulong) as W_)
                as *mut c_void as *mut StgWord8)
                .offset(
                    best_mblocks
                        .wrapping_sub(mblocks)
                        .wrapping_mul(((1 as c_ulong) << 20 as c_int) as StgWord)
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
            mblock = getMBlocksOnNode(node, mblocks as uint32_t);
        } else {
            mblock = getMBlocks(mblocks as uint32_t);
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

unsafe fn allocGroupOnNode(mut node: uint32_t, mut n: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut rem = null_mut::<bdescr>();
    let mut ln: StgWord = 0;

    if n == 0 as W_ {
        barf(b"allocGroup: requested zero blocks\0" as *const u8 as *const c_char);
    }

    if n >= BLOCKS_PER_MBLOCK {
        let mut mblocks: StgWord = 0;

        mblocks = (1 as W_).wrapping_add(
            ((n.wrapping_sub(
                (((1 as c_ulong) << 20 as c_int) as W_)
                    .wrapping_sub(
                        ((0x40 as c_ulong).wrapping_mul(
                            ((1 as c_ulong) << 20 as c_int)
                                .wrapping_div((1 as c_ulong) << 12 as c_int),
                        ) as W_)
                            .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                            .wrapping_sub(1 as W_)
                            & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong) as W_,
                    )
                    .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
            )
            .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
            .wrapping_add(MBLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
                & !MBLOCK_MASK as W_) as *mut c_void as W_)
                .wrapping_div(MBLOCK_SIZE as W_),
        ) as StgWord;

        recordAllocatedBlocks(node, mblocks.wrapping_mul(BLOCKS_PER_MBLOCK) as uint32_t);
        bd = alloc_mega_group(node, mblocks);
        initGroup(bd);
    } else {
        recordAllocatedBlocks(node, n as uint32_t);
        ln = log_2_ceil(n) as StgWord;

        while ln < NUM_FREE_LISTS as StgWord && free_list[node as usize][ln as usize].is_null() {
            ln = ln.wrapping_add(1);
        }

        if ln == NUM_FREE_LISTS as StgWord {
            bd = alloc_mega_group(node, 1 as StgWord);
            (*bd).blocks = n as StgWord32;
            initGroup(bd);
            rem = bd.offset(n as isize);
            (*rem).blocks = BLOCKS_PER_MBLOCK.wrapping_sub(n) as StgWord32;
            initGroup(rem);
            recordAllocatedBlocks(node, (*rem).blocks as uint32_t);
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
                bd = split_free_block(bd, node, n, ln as uint32_t);
                initGroup(bd);
            } else {
                barf(b"allocGroup: free list corrupted\0" as *const u8 as *const c_char);
            }
        }
    }

    return bd;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocAlignedGroupOnNode(mut node: uint32_t, mut n: W_) -> *mut bdescr {
    let mut num_blocks: W_ = (2 as W_).wrapping_mul(n).wrapping_sub(1 as W_);

    if num_blocks >= BLOCKS_PER_MBLOCK {
        barf(
            b"allocAlignedGroupOnNode: allocating megablocks is not supported\n    requested blocks: %llu\n    required for alignment: %llu\n    megablock size (in blocks): %llu\0"
                as *const u8 as *const c_char,
            n,
            num_blocks,
            BLOCKS_PER_MBLOCK,
        );
    }

    let mut group_size: W_ = n.wrapping_mul(BLOCK_SIZE as W_);

    let mut max_blocks: W_ = ({
        let mut _a: W_ = (num_blocks as W_).wrapping_mul(3 as W_);
        let mut _b: W_ = (((1 as c_ulong) << 20 as c_int) as W_)
            .wrapping_sub(
                ((0x40 as c_ulong).wrapping_mul(
                    ((1 as c_ulong) << 20 as c_int).wrapping_div((1 as c_ulong) << 12 as c_int),
                ) as W_)
                    .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                    .wrapping_sub(1 as W_)
                    & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong) as W_,
            )
            .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_)
            .wrapping_sub(1 as W_);

        if _a <= _b { _a } else { _b as W_ }
    });

    let mut bd = allocLargeChunkOnNode(node, num_blocks, max_blocks);
    num_blocks = (*bd).blocks as W_;

    let mut slop_low: W_ = 0 as W_;

    if ((*bd).start as uintptr_t as W_).wrapping_rem(group_size) != 0 as W_ {
        slop_low =
            group_size.wrapping_sub(((*bd).start as uintptr_t as W_).wrapping_rem(group_size));
    }

    let mut slop_high: W_ = num_blocks
        .wrapping_mul(BLOCK_SIZE as W_)
        .wrapping_sub(group_size)
        .wrapping_sub(slop_low);

    let mut slop_low_blocks: W_ = slop_low.wrapping_div(BLOCK_SIZE as W_);
    let mut slop_high_blocks: W_ = slop_high.wrapping_div(BLOCK_SIZE as W_);

    if slop_low_blocks != 0 as W_ {
        bd = split_block_high(bd, num_blocks.wrapping_sub(slop_low_blocks));
    }

    if slop_high_blocks != 0 as W_ {
        bd = split_block_low(bd, n);
    }

    return bd;
}

#[inline]
unsafe fn nodeWithLeastBlocks() -> uint32_t {
    let mut node: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0;
    let mut min_blocks: uint32_t = n_alloc_blocks_by_node[0 as c_int as usize] as uint32_t;
    i = 1 as uint32_t;

    while i < n_numa_nodes {
        if n_alloc_blocks_by_node[i as usize] < min_blocks as W_ {
            min_blocks = n_alloc_blocks_by_node[i as usize] as uint32_t;
            node = i;
        }

        i = i.wrapping_add(1);
    }

    return node;
}

unsafe fn allocGroup(mut n: W_) -> *mut bdescr {
    return allocGroupOnNode(nodeWithLeastBlocks(), n);
}

unsafe fn allocLargeChunkOnNode(mut node: uint32_t, mut min: W_, mut max: W_) -> *mut bdescr {
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
        bd = split_free_block(bd, node, max, ln as uint32_t);
        initGroup(bd);
    }

    recordAllocatedBlocks(node, (*bd).blocks as uint32_t);

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
    bd = allocGroup(n);

    return bd;
}

unsafe fn allocBlock_lock() -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    bd = allocBlock();

    return bd;
}

unsafe fn allocGroupOnNode_lock(mut node: uint32_t, mut n: W_) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    bd = allocGroupOnNode(node, n);

    return bd;
}

unsafe fn allocBlockOnNode_lock(mut node: uint32_t) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    bd = allocBlockOnNode(node);

    return bd;
}

#[inline]
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
                                (((1 as c_ulong) << 20 as c_int) as W_)
                                    .wrapping_sub(
                                        ((0x40 as c_ulong).wrapping_mul(
                                            ((1 as c_ulong) << 20 as c_int)
                                                .wrapping_div((1 as c_ulong) << 12 as c_int),
                                        ) as W_)
                                            .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                            .wrapping_sub(1 as W_)
                                            & !((1 as c_ulong) << 12 as c_int)
                                                .wrapping_sub(1 as c_ulong)
                                                as W_,
                                    )
                                    .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                            )
                            .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
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
                            (((1 as c_ulong) << 20 as c_int) as W_)
                                .wrapping_sub(
                                    ((0x40 as c_ulong).wrapping_mul(
                                        ((1 as c_ulong) << 20 as c_int)
                                            .wrapping_div((1 as c_ulong) << 12 as c_int),
                                    ) as W_)
                                        .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                        .wrapping_sub(1 as W_)
                                        & !((1 as c_ulong) << 12 as c_int)
                                            .wrapping_sub(1 as c_ulong)
                                            as W_,
                                )
                                .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                        )
                        .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
                        .wrapping_add(((1 as c_ulong) << 20 as c_int) as W_)
                        .wrapping_sub(1 as W_)
                        & !((1 as c_ulong) << 20 as c_int).wrapping_sub(1 as c_ulong) as W_)
                        as *mut c_void as W_)
                        .wrapping_div(((1 as c_ulong) << 20 as c_int) as W_),
                )
                .wrapping_add(
                    (1 as W_).wrapping_add(
                        ((((*q).blocks as W_)
                            .wrapping_sub(
                                (((1 as c_ulong) << 20 as c_int) as W_)
                                    .wrapping_sub(
                                        ((0x40 as c_ulong).wrapping_mul(
                                            ((1 as c_ulong) << 20 as c_int)
                                                .wrapping_div((1 as c_ulong) << 12 as c_int),
                                        ) as W_)
                                            .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                            .wrapping_sub(1 as W_)
                                            & !((1 as c_ulong) << 12 as c_int)
                                                .wrapping_sub(1 as c_ulong)
                                                as W_,
                                    )
                                    .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                            )
                            .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
                            .wrapping_add(((1 as c_ulong) << 20 as c_int) as W_)
                            .wrapping_sub(1 as W_)
                            & !((1 as c_ulong) << 20 as c_int).wrapping_sub(1 as c_ulong) as W_)
                            as *mut c_void as W_)
                            .wrapping_div(((1 as c_ulong) << 20 as c_int) as W_),
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
    let mut node: uint32_t = 0;
    node = (*mg).node as uint32_t;

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
    };
}

unsafe fn free_deferred_mega_groups(mut node: uint32_t) {
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
}

unsafe fn freeGroup(mut p: *mut bdescr) {
    let mut ln: StgWord = 0;
    let mut node: uint32_t = 0;
    node = (*p).node as uint32_t;
    (*p).c2rust_unnamed.free = -(1 as c_int) as *mut c_void as StgPtr;
    (*p).r#gen = null_mut::<generation_>();
    (*p).gen_no = 0 as StgWord16;

    if (*p).blocks == 0 as StgWord32 {
        barf(b"freeGroup: block size is zero\0" as *const u8 as *const c_char);
    }

    if (*p).blocks as W_ >= BLOCKS_PER_MBLOCK {
        let mut mblocks: StgWord = 0;

        mblocks = (1 as W_).wrapping_add(
            ((((*p).blocks as W_)
                .wrapping_sub(
                    (((1 as c_ulong) << 20 as c_int) as W_)
                        .wrapping_sub(
                            ((0x40 as c_ulong).wrapping_mul(
                                ((1 as c_ulong) << 20 as c_int)
                                    .wrapping_div((1 as c_ulong) << 12 as c_int),
                            ) as W_)
                                .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                .wrapping_sub(1 as W_)
                                & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong) as W_,
                        )
                        .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                )
                .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
                .wrapping_add(MBLOCK_SIZE as W_)
                .wrapping_sub(1 as W_)
                & !MBLOCK_MASK as W_) as *mut c_void as W_)
                .wrapping_div(MBLOCK_SIZE as W_),
        ) as StgWord;

        recordFreedBlocks(node, mblocks.wrapping_mul(BLOCKS_PER_MBLOCK) as uint32_t);
        free_mega_group(p);
        return;
    }

    recordFreedBlocks(node, (*p).blocks as uint32_t);

    let mut next = null_mut::<bdescr>();
    next = p.offset((*p).blocks as isize);

    if next
        <= ((MBLOCK_SIZE.wrapping_sub(BLOCK_SIZE) >> BLOCK_SHIFT - BDESCR_SHIFT) as W_)
            .wrapping_add(
                (p as W_ & !((1 as c_ulong) << 20 as c_int).wrapping_sub(1 as c_ulong) as W_)
                    as *mut c_void as W_,
            ) as *mut bdescr
        && (*next).c2rust_unnamed.free == -(1 as c_int) as P_
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
        (p as W_ & !((1 as c_ulong) << 20 as c_int).wrapping_sub(1 as c_ulong) as W_) as *mut c_void
            as W_,
    ) as *mut bdescr
    {
        let mut prev = null_mut::<bdescr>();
        prev = p.offset(-(1 as c_int as isize));

        if (*prev).blocks == 0 as StgWord32 {
            prev = (*prev).link as *mut bdescr;
        }

        if (*prev).c2rust_unnamed.free == -(1 as c_int) as P_ {
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
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn freeGroup_lock(mut p: *mut bdescr) {
    freeGroup(p);
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
    freeChain(bd);
}

unsafe fn initMBlock(mut mblock: *mut c_void, mut node: uint32_t) {
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
        bd = bd.offset(1 as c_int as isize);
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
        barf(b"MBlock freeing is already deferred\0" as *const u8 as *const c_char);
    }

    defer_mblock_frees = r#true != 0;
}

unsafe fn commitMBlockFreeing() {
    if !defer_mblock_frees {
        barf(b"MBlock freeing was never deferred\0" as *const u8 as *const c_char);
    }

    defer_mblock_frees = r#false != 0;

    let mut node: uint32_t = 0 as uint32_t;

    while node < n_numa_nodes {
        free_deferred_mega_groups(node);
        node = node.wrapping_add(1);
    }
}

unsafe fn countBlocks(mut bd: *mut bdescr) -> W_ {
    let mut n: W_ = 0;
    n = 0 as W_;

    while !bd.is_null() {
        n = n.wrapping_add((*bd).blocks as W_);
        bd = (*bd).link as *mut bdescr;
    }

    return n;
}

unsafe fn countAllocdBlocks(mut bd: *mut bdescr) -> W_ {
    let mut n: W_ = 0;
    n = 0 as W_;

    while !bd.is_null() {
        n = n.wrapping_add((*bd).blocks as W_);

        if (*bd).blocks as W_ > BLOCKS_PER_MBLOCK {
            n = n.wrapping_sub(
                (MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_)
                    .wrapping_sub(BLOCKS_PER_MBLOCK)
                    .wrapping_mul(
                        ((*bd).blocks as c_ulong).wrapping_div(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE))
                            as W_,
                    ),
            );
        }

        bd = (*bd).link as *mut bdescr;
    }

    return n;
}

unsafe fn returnMemoryToOS(mut n: uint32_t) -> uint32_t {
    let mut bd = null_mut::<bdescr>();
    let mut node: uint32_t = 0;
    let mut size: StgWord = 0;
    let mut init_n: uint32_t = 0;
    init_n = n;
    node = 0 as uint32_t;

    while n > 0 as uint32_t && node < n_numa_nodes {
        bd = free_mblock_list[node as usize];

        while n > 0 as uint32_t && !bd.is_null() {
            size = (1 as W_).wrapping_add(
                ((((*bd).blocks as W_)
                    .wrapping_sub(
                        (((1 as c_ulong) << 20 as c_int) as W_)
                            .wrapping_sub(
                                ((0x40 as c_ulong).wrapping_mul(
                                    ((1 as c_ulong) << 20 as c_int)
                                        .wrapping_div((1 as c_ulong) << 12 as c_int),
                                ) as W_)
                                    .wrapping_add(((1 as c_ulong) << 12 as c_int) as W_)
                                    .wrapping_sub(1 as W_)
                                    & !((1 as c_ulong) << 12 as c_int).wrapping_sub(1 as c_ulong)
                                        as W_,
                            )
                            .wrapping_div(((1 as c_ulong) << 12 as c_int) as W_),
                    )
                    .wrapping_mul(((1 as c_ulong) << 12 as c_int) as W_)
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
                n = 0 as uint32_t;
            } else {
                let mut freeAddr_0 =
                    ((*bd).start as W_ & !MBLOCK_MASK as W_) as *mut c_void as *mut c_char;
                n = (n as StgWord).wrapping_sub(size) as uint32_t as uint32_t;
                bd = (*bd).link as *mut bdescr;
                freeMBlocks(freeAddr_0 as *mut c_void, size as uint32_t);
            }
        }

        free_mblock_list[node as usize] = bd;
        node = node.wrapping_add(1);
    }

    releaseFreeMemory();

    return init_n.wrapping_sub(n);
}

unsafe fn clear_free_list() {
    let mut node: uint32_t = 0 as uint32_t;

    while node < n_numa_nodes {
        let mut bd = free_mblock_list[node as usize];

        while !bd.is_null() {
            clear_blocks(bd);
            bd = (*bd).link as *mut bdescr;
        }

        let mut ln = 0 as c_int;

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
