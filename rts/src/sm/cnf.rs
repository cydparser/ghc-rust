use crate::capability::recordClosureMutated;
use crate::ffi::hs_ffi::HS_INT32_MAX;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::storage::block::{
    BF_COMPACT, BF_KNOWN, BF_PINNED, BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK,
    Bdescr, MBLOCK_SIZE, allocGroup, bdescr, bdescr_, dbl_link_onto, dbl_link_remove, freeGroup,
};
use crate::ffi::rts::storage::block::{Bdescr, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, LOOKS_LIKE_CLOSURE_PTR, TAG_CLOSURE, UNTAG_CLOSURE, arr_words_sizeW,
    doingErasProfiling, doingLDVProfiling, doingRetainerProfiling, get_itbl, mut_arr_ptrs_sizeW,
};
use crate::ffi::rts::storage::closures::{
    StgArrBytes, StgClosure_, StgCompactNFData, StgCompactNFData_, StgCompactNFDataBlock,
    StgCompactNFDataBlock_, StgMutArrPtrs, StgSmallMutArrPtrs,
};
use crate::ffi::rts::storage::closures::{StgCompactNFData, StgCompactNFDataBlock};
use crate::ffi::rts::storage::gc::{g0, generation, initBdescr, memcount};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::rts::{_assertFail, EXIT_HEAPOVERFLOW, reportHeapOverflow, stg_exit};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    stg_COMPACT_NFDATA_CLEAN_info, stg_COMPACT_NFDATA_DIRTY_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
use crate::ffi::stg::types::{StgPtr, StgWord16, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::hash::{HashTable, insertHashTable};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::cnf::{objectGetCompact, objectGetCompactBlock};
use crate::sm::should_compact::{
    SHOULDCOMPACT_IN_CNF, SHOULDCOMPACT_NOTIN_CNF, SHOULDCOMPACT_PINNED, SHOULDCOMPACT_STATIC,
};
use crate::sm::storage::sm_mutex;
use crate::trace::{DEBUG_RTS, trace_};

#[inline]
pub(crate) unsafe fn objectGetCompactBlock(
    mut closure: *mut StgClosure,
) -> *mut StgCompactNFDataBlock {
    let mut object_block = null_mut::<bdescr>();
    let mut head_block = null_mut::<bdescr>();
    object_block = Bdescr(closure as StgPtr);

    if ((*object_block).flags as i32 & 512 != 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.h".as_ptr(), 48);
    }

    if (*object_block).blocks == 0 {
        head_block = (*object_block).link as *mut bdescr;
    } else {
        head_block = object_block;
    }

    if ((*head_block).flags as i32 & 512 != 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.h".as_ptr(), 55);
    }

    return (*head_block).start as *mut StgCompactNFDataBlock;
}

#[inline]
pub(crate) unsafe fn objectGetCompact(mut closure: *mut StgClosure) -> *mut StgCompactNFData {
    let mut block = objectGetCompactBlock(closure);

    return (*block).owner as *mut StgCompactNFData;
}

type AllocateOp = u32;

const ALLOCATE_IMPORT_APPEND: AllocateOp = 3;

const ALLOCATE_IMPORT_NEW: AllocateOp = 2;

const ALLOCATE_NEW: AllocateOp = 1;

const ALLOCATE_APPEND: AllocateOp = 0;

unsafe fn compactAllocateBlockInternal(
    mut cap: *mut Capability,
    mut aligned_size: StgWord,
    mut first: *mut StgCompactNFDataBlock,
    mut operation: AllocateOp,
) -> *mut StgCompactNFDataBlock {
    let mut self_0 = null_mut::<StgCompactNFDataBlock>();
    let mut block = null_mut::<bdescr>();
    let mut head = null_mut::<bdescr>();
    let mut n_blocks: u32 = 0;
    let mut g = null_mut::<generation>();
    n_blocks = aligned_size.wrapping_div(BLOCK_SIZE as StgWord) as u32;

    if RtsFlags.GcFlags.maxHeapSize > 0 && n_blocks >= RtsFlags.GcFlags.maxHeapSize
        || n_blocks >= HS_INT32_MAX as u32
    {
        reportHeapOverflow();
        stg_exit(EXIT_HEAPOVERFLOW);
    }

    if !first.is_null() {
        block = Bdescr(first as StgPtr);
        g = (*block).r#gen as *mut generation;
    } else {
        g = g0;
    }

    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/CNF.c".as_ptr(),
            202,
            __r,
        );
    }

    block = allocGroup(n_blocks as W_);

    let mut current_block_38: u64;

    match operation as u32 {
        1 => {
            if first.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 206);
            }

            if (g == g0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 207);
            }

            dbl_link_onto(block, &raw mut (*g0).compact_objects);
            (*g).n_compact_blocks = (*g)
                .n_compact_blocks
                .wrapping_add((*block).blocks as memcount);
            (*g).n_new_large_words = ((*g).n_new_large_words as StgWord)
                .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord))
                as memcount as memcount;
            current_block_38 = 1345366029464561491;
        }
        2 => {
            dbl_link_onto(block, &raw mut (*g0).compact_blocks_in_import);
            current_block_38 = 10029153119333114254;
        }
        3 => {
            current_block_38 = 10029153119333114254;
        }
        0 => {
            (*g).n_compact_blocks = (*g)
                .n_compact_blocks
                .wrapping_add((*block).blocks as memcount);

            if g == g0 {
                (*g).n_new_large_words = ((*g).n_new_large_words as StgWord)
                    .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord))
                    as memcount as memcount;
            }

            current_block_38 = 1345366029464561491;
        }
        _ => {
            if (c"code should not be reached".as_ptr()).is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 230);
            }

            current_block_38 = 1345366029464561491;
        }
    }

    match current_block_38 {
        10029153119333114254 => {
            if first.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 217);
            }

            if (g == g0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 218);
            }

            (*g).n_compact_blocks_in_import = (*g)
                .n_compact_blocks_in_import
                .wrapping_add((*block).blocks as memcount);
            (*g).n_new_large_words = ((*g).n_new_large_words as StgWord)
                .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord))
                as memcount as memcount;
        }
        _ => {}
    }

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/CNF.c".as_ptr(),
            235,
        );
    }

    (*cap).total_allocated = (*cap)
        .total_allocated
        .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord) as u64);
    self_0 = (*block).start as *mut StgCompactNFDataBlock;
    (*self_0).self_0 = self_0 as *mut StgCompactNFDataBlock_;
    (*self_0).next = null_mut::<StgCompactNFDataBlock_>();
    head = block;
    initBdescr(head, g, g);
    (*head).flags = BF_COMPACT as StgWord16;
    block = head.offset(1);
    n_blocks = n_blocks.wrapping_sub(1);

    while n_blocks > 0 {
        initBdescr(block, g, g);
        (*block).link = head as *mut bdescr_;
        (*block).blocks = 0;
        (*block).flags = BF_COMPACT as StgWord16;
        block = block.offset(1);
        n_blocks = n_blocks.wrapping_sub(1);
    }

    return self_0;
}

#[inline]
unsafe fn compactGetFirstBlock(mut str: *mut StgCompactNFData) -> *mut StgCompactNFDataBlock {
    return (str as W_).wrapping_sub(size_of::<StgCompactNFDataBlock>() as W_)
        as *mut StgCompactNFDataBlock;
}

#[inline]
unsafe fn firstBlockGetCompact(mut block: *mut StgCompactNFDataBlock) -> *mut StgCompactNFData {
    return (block as W_).wrapping_add(size_of::<StgCompactNFDataBlock>() as W_)
        as *mut StgCompactNFData;
}

unsafe fn compactFree(mut str: *mut StgCompactNFData) {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut next = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();
    block = compactGetFirstBlock(str);

    while !block.is_null() {
        next = (*block).next as *mut StgCompactNFDataBlock;
        bd = Bdescr(block as StgPtr);

        if (RtsFlags.GcFlags.useNonmoving as i32 != 0 || (*bd).flags as i32 & 1 == 0) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/CNF.c".as_ptr(), 279);
        }

        freeGroup(bd);
        block = next;
    }
}

unsafe fn compactMarkKnown(mut str: *mut StgCompactNFData) {
    let mut bd = null_mut::<bdescr>();
    let mut block = null_mut::<StgCompactNFDataBlock>();
    block = compactGetFirstBlock(str);

    while !block.is_null() {
        bd = Bdescr(block as StgPtr);
        (*bd).flags = ((*bd).flags as i32 | BF_KNOWN) as StgWord16;
        block = (*block).next as *mut StgCompactNFDataBlock;
    }
}

unsafe fn countCompactBlocks(mut outer: *mut bdescr) -> StgWord {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut count: W_ = 0;
    count = 0;

    while !outer.is_null() {
        let mut inner = null_mut::<bdescr>();
        block = (*outer).start as *mut StgCompactNFDataBlock;

        loop {
            inner = Bdescr(block as StgPtr);

            if ((*inner).flags as i32 & 512 != 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 314);
            }

            count = count.wrapping_add((*inner).blocks as W_);
            block = (*block).next as *mut StgCompactNFDataBlock;

            if block.is_null() {
                break;
            }
        }

        outer = (*outer).link as *mut bdescr;
    }

    return count as StgWord;
}

unsafe fn countAllocdCompactBlocks(mut outer: *mut bdescr) -> StgWord {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut count: W_ = 0;
    count = 0;

    while !outer.is_null() {
        let mut inner = null_mut::<bdescr>();
        block = (*outer).start as *mut StgCompactNFDataBlock;

        loop {
            inner = Bdescr(block as StgPtr);

            if ((*inner).flags as i32 & 512 != 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/CNF.c".as_ptr(), 341);
            }

            count = count.wrapping_add((*inner).blocks as W_);

            if (*inner).blocks as W_ > BLOCKS_PER_MBLOCK {
                count = count.wrapping_sub(
                    (MBLOCK_SIZE.wrapping_div(BLOCK_SIZE) as W_)
                        .wrapping_sub(BLOCKS_PER_MBLOCK)
                        .wrapping_mul(
                            ((*inner).blocks as u64)
                                .wrapping_div(MBLOCK_SIZE.wrapping_div(BLOCK_SIZE))
                                as W_,
                        ),
                );
            }

            block = (*block).next as *mut StgCompactNFDataBlock;

            if block.is_null() {
                break;
            }
        }

        outer = (*outer).link as *mut bdescr;
    }

    return count as StgWord;
}

unsafe fn compactNew(mut cap: *mut Capability, mut size: StgWord) -> *mut StgCompactNFData {
    let mut aligned_size: StgWord = 0;
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut self_0 = null_mut::<StgCompactNFData>();
    let mut bd = null_mut::<bdescr>();
    aligned_size = (size
        .wrapping_add(size_of::<StgCompactNFData>() as StgWord)
        .wrapping_add(size_of::<StgCompactNFDataBlock>() as StgWord)
        .wrapping_add(BLOCK_SIZE as W_)
        .wrapping_sub(1 as W_)
        & !BLOCK_MASK as W_) as StgWord;

    if aligned_size >= (BLOCK_SIZE as W_).wrapping_mul(BLOCKS_PER_MBLOCK) {
        aligned_size = (BLOCK_SIZE as W_).wrapping_mul(BLOCKS_PER_MBLOCK) as StgWord;
    }

    block = compactAllocateBlockInternal(
        cap,
        aligned_size,
        null_mut::<StgCompactNFDataBlock>(),
        ALLOCATE_NEW,
    );

    self_0 = firstBlockGetCompact(block);

    let ref mut fresh13 = (*(self_0 as *mut StgClosure)).header.prof.ccs;
    *fresh13 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(self_0 as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(self_0 as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(self_0 as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*(self_0 as *mut StgClosure)).header.info)
        .store(&raw const stg_COMPACT_NFDATA_CLEAN_info, Ordering::Relaxed);
    (*self_0).autoBlockW = aligned_size.wrapping_div(size_of::<StgWord>() as StgWord);
    (*self_0).nursery = block;
    (*self_0).last = block;
    (*self_0).hash = null_mut::<hashtable>();
    (*self_0).link = null_mut::<StgCompactNFData_>();
    (*block).owner = self_0 as *mut StgCompactNFData_;
    bd = Bdescr(block as StgPtr);
    (*bd).c2rust_unnamed.free =
        (self_0 as W_).wrapping_add(size_of::<StgCompactNFData>() as W_) as StgPtr;
    (*self_0).hp = (*bd).c2rust_unnamed.free;
    (*self_0).hpLim = (*bd)
        .start
        .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize);
    (*self_0).totalW = ((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as StgWord;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.compact as i64 != 0 {
        trace_(c"compactNew: size %llu".as_ptr(), size);
    }

    return self_0;
}

unsafe fn compactAppendBlock(
    mut cap: *mut Capability,
    mut str: *mut StgCompactNFData,
    mut aligned_size: StgWord,
) -> *mut StgCompactNFDataBlock {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();

    block = compactAllocateBlockInternal(
        cap,
        aligned_size,
        compactGetFirstBlock(str),
        ALLOCATE_APPEND,
    );

    (*block).owner = str as *mut StgCompactNFData_;
    (*block).next = null_mut::<StgCompactNFDataBlock_>();

    if (*(*str).last).next.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 414);
    }

    (*(*str).last).next = block as *mut StgCompactNFDataBlock_;
    (*str).last = block;
    bd = Bdescr(block as StgPtr);
    (*bd).c2rust_unnamed.free =
        (block as W_).wrapping_add(size_of::<StgCompactNFDataBlock>() as W_) as StgPtr;

    if ((*bd).c2rust_unnamed.free
        == (block as StgPtr).offset(
            (size_of::<StgCompactNFDataBlock>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as isize,
        )) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 420);
    }

    (*str).totalW = (*str)
        .totalW
        .wrapping_add(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as StgWord);

    return block;
}

unsafe fn compactResize(
    mut cap: *mut Capability,
    mut str: *mut StgCompactNFData,
    mut new_size: StgWord,
) {
    let mut aligned_size: StgWord = 0;
    aligned_size = (new_size
        .wrapping_add(size_of::<StgCompactNFDataBlock>() as StgWord)
        .wrapping_add(BLOCK_SIZE as W_)
        .wrapping_sub(1 as W_)
        & !BLOCK_MASK as W_) as StgWord;

    if aligned_size >= (BLOCK_SIZE as W_).wrapping_mul(BLOCKS_PER_MBLOCK) {
        aligned_size = (BLOCK_SIZE as W_).wrapping_mul(BLOCKS_PER_MBLOCK) as StgWord;
    }

    (*str).autoBlockW = aligned_size.wrapping_div(size_of::<StgWord>() as StgWord);
    compactAppendBlock(cap, str, aligned_size);
}

unsafe fn has_room_for(mut bd: *mut bdescr, mut sizeW: StgWord) -> bool {
    return (*bd).c2rust_unnamed.free
        < (*bd)
            .start
            .offset((BLOCK_SIZE_W as W_).wrapping_mul(BLOCKS_PER_MBLOCK) as isize)
        && (*bd).c2rust_unnamed.free.offset(sizeW as isize)
            <= (*bd)
                .start
                .offset(BLOCK_SIZE_W.wrapping_mul((*bd).blocks as usize) as isize);
}

unsafe fn block_is_full(mut block: *mut StgCompactNFDataBlock) -> bool {
    let mut bd = null_mut::<bdescr>();
    bd = Bdescr(block as StgPtr);

    return !has_room_for(bd, 7);
}

unsafe fn allocateForCompact(
    mut cap: *mut Capability,
    mut str: *mut StgCompactNFData,
    mut sizeW: StgWord,
) -> *mut c_void {
    let mut to = null_mut::<StgWord>();
    let mut next_size: StgWord = 0;
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();

    if !(*str).nursery.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 477);
    }

    if ((*str).hp > (*Bdescr((*str).nursery as StgPtr)).start) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 478);
    }

    if ((*str).hp
        <= (*Bdescr((*str).nursery as StgPtr)).start.offset(
            ((*Bdescr((*str).nursery as StgPtr)).blocks as usize)
                .wrapping_mul(((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize))
                as isize,
        )) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 480);
    }

    loop {
        if (*str).hp.offset(sizeW as isize) < (*str).hpLim {
            to = (*str).hp;
            (*str).hp = (*str).hp.offset(sizeW as isize);

            return to as *mut c_void;
        }

        bd = Bdescr((*str).nursery as StgPtr);
        (*bd).c2rust_unnamed.free = (*str).hp;

        if !block_is_full((*str).nursery) {
            break;
        }

        loop {
            (*str).nursery = (*(*str).nursery).next as *mut StgCompactNFDataBlock;

            if !(!(*str).nursery.is_null() && block_is_full((*str).nursery) as i32 != 0) {
                break;
            }
        }

        if (*str).nursery.is_null() {
            (*str).nursery = compactAppendBlock(
                cap,
                str,
                (*str).autoBlockW.wrapping_mul(size_of::<W_>() as StgWord),
            );
        }

        bd = Bdescr((*str).nursery as StgPtr);
        (*str).hp = (*bd).c2rust_unnamed.free;
        (*str).hpLim = (*bd)
            .start
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize);
    }

    block = (*(*str).nursery).next as *mut StgCompactNFDataBlock;

    while !block.is_null() {
        bd = Bdescr(block as StgPtr);

        if has_room_for(bd, sizeW) {
            to = (*bd).c2rust_unnamed.free;
            (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(sizeW as isize);

            return to as *mut c_void;
        }

        block = (*block).next as *mut StgCompactNFDataBlock;
    }

    next_size = ({
        let mut _a: StgWord =
            ((*str).autoBlockW as StgWord).wrapping_mul(size_of::<StgWord>() as StgWord);

        let mut _b: StgWord = sizeW
            .wrapping_mul(size_of::<StgWord>() as StgWord)
            .wrapping_add(size_of::<StgCompactNFDataBlock>() as StgWord)
            .wrapping_add(((1 as u64) << 12 as i32) as StgWord)
            .wrapping_sub(1 as StgWord)
            & !((1 as u64) << 12 as i32).wrapping_sub(1 as u64) as StgWord;

        if _a <= _b { _b } else { _a as StgWord }
    });

    block = compactAppendBlock(cap, str, next_size);
    bd = Bdescr(block as StgPtr);
    to = (*bd).c2rust_unnamed.free;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(sizeW as isize);

    return to as *mut c_void;
}

unsafe fn insertCompactHash(
    mut cap: *mut Capability,
    mut str: *mut StgCompactNFData,
    mut p: *mut StgClosure,
    mut to: *mut StgClosure,
) {
    insertHashTable(
        (*str).hash as *mut HashTable,
        p as StgWord,
        to as *const c_void,
    );

    let mut strinfo: *mut *const StgInfoTable = &raw mut (*str).header.info;

    if *strinfo == &raw const stg_COMPACT_NFDATA_CLEAN_info {
        *strinfo = &raw const stg_COMPACT_NFDATA_DIRTY_info;
        recordClosureMutated(cap, str as *mut StgClosure);
    }
}

unsafe fn compactContains(mut str: *mut StgCompactNFData, mut what: StgPtr) -> StgWord {
    let mut bd = null_mut::<bdescr>();

    if !(what as W_ >= mblock_address_space.0.begin && (what as W_) < mblock_address_space.0.end) {
        return 0;
    }

    bd = Bdescr(what);

    return ((*bd).flags as i32 & BF_COMPACT != 0
        && (str.is_null() || objectGetCompact(what as *mut StgClosure) == str)) as i32
        as StgWord;
}

unsafe fn compactAllocateBlock(
    mut cap: *mut Capability,
    mut size: StgWord,
    mut previous: *mut StgCompactNFDataBlock,
) -> *mut StgCompactNFDataBlock {
    let mut aligned_size: StgWord = 0;
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();
    aligned_size =
        (size.wrapping_add(BLOCK_SIZE as W_).wrapping_sub(1 as W_) & !BLOCK_MASK as W_) as StgWord;

    block = compactAllocateBlockInternal(
        cap,
        aligned_size,
        null_mut::<StgCompactNFDataBlock>(),
        (if !previous.is_null() {
            ALLOCATE_IMPORT_APPEND as i32
        } else {
            ALLOCATE_IMPORT_NEW as i32
        }) as AllocateOp,
    );

    if !previous.is_null() {
        (*previous).next = block as *mut StgCompactNFDataBlock_;
    }

    bd = Bdescr(block as StgPtr);
    (*bd).c2rust_unnamed.free = ((*bd).start as W_).wrapping_add(size as W_) as P_ as StgPtr;

    return block;
}

unsafe fn shouldCompact(mut str: *mut StgCompactNFData, mut p: *mut StgClosure) -> StgWord {
    let mut bd = null_mut::<bdescr>();

    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return SHOULDCOMPACT_STATIC as StgWord;
    }

    bd = Bdescr(p as StgPtr);

    if (*bd).flags as i32 & BF_PINNED != 0 {
        return SHOULDCOMPACT_PINNED as StgWord;
    }

    if (*bd).flags as i32 & BF_COMPACT != 0 && objectGetCompact(p) == str {
        return SHOULDCOMPACT_IN_CNF as StgWord;
    } else {
        return SHOULDCOMPACT_NOTIN_CNF as StgWord;
    };
}

unsafe fn check_object_in_compact(mut str: *mut StgCompactNFData, mut p: *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();

    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return;
    }

    bd = Bdescr(p as StgPtr);

    if ((*bd).flags as i32 & 512 != 0 && objectGetCompact(p) == str) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 648);
    };
}

unsafe fn verify_mut_arr_ptrs(mut str: *mut StgCompactNFData, mut a: *mut StgMutArrPtrs) {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    p = (&raw mut (*a).payload as *mut *mut StgClosure).offset(0) as *mut *mut StgClosure as StgPtr;
    q = (&raw mut (*a).payload as *mut *mut StgClosure).offset((*a).ptrs as isize)
        as *mut *mut StgClosure as StgPtr;

    while p < q {
        check_object_in_compact(str, UNTAG_CLOSURE(*(p as *mut *mut StgClosure)));
        p = p.offset(1);
    }
}

unsafe fn verify_consistency_block(
    mut str: *mut StgCompactNFData,
    mut block: *mut StgCompactNFDataBlock,
) {
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();
    let mut info = null::<StgInfoTable>();
    let mut q = null_mut::<StgClosure>();
    p = firstBlockGetCompact(block) as P_ as StgPtr;
    bd = Bdescr(block as StgPtr);

    while p < (*bd).c2rust_unnamed.free {
        q = p as *mut StgClosure;

        if LOOKS_LIKE_CLOSURE_PTR(q as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/CNF.c".as_ptr(), 679);
        }

        info = get_itbl(q);

        let mut current_block_30: u64;

        match (*info).r#type {
            2 => {
                check_object_in_compact(
                    str,
                    UNTAG_CLOSURE(*(&raw mut (*q).payload as *mut *mut StgClosure_).offset(0)
                        as *mut StgClosure),
                );

                current_block_30 = 17352930546238167574;
            }
            3 => {
                current_block_30 = 17352930546238167574;
            }
            4 => {
                check_object_in_compact(
                    str,
                    UNTAG_CLOSURE(*(&raw mut (*q).payload as *mut *mut StgClosure_).offset(1)
                        as *mut StgClosure),
                );

                current_block_30 = 7711255570521815756;
            }
            5 => {
                current_block_30 = 7711255570521815756;
            }
            6 => {
                current_block_30 = 16517549058459909004;
            }
            1 | 50 | 7 => {
                let mut i: u32 = 0;
                i = 0;

                while i < (*info).layout.payload.ptrs {
                    check_object_in_compact(
                        str,
                        UNTAG_CLOSURE(
                            *(&raw mut (*q).payload as *mut *mut StgClosure_).offset(i as isize)
                                as *mut StgClosure,
                        ),
                    );

                    i = i.wrapping_add(1);
                }

                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add((*info).layout.payload.ptrs as usize)
                        .wrapping_add((*info).layout.payload.nptrs as usize)
                        as isize,
                );

                current_block_30 = 11385396242402735691;
            }
            42 => {
                p = p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize);
                current_block_30 = 11385396242402735691;
            }
            46 | 45 => {
                verify_mut_arr_ptrs(str, p as *mut StgMutArrPtrs);
                p = p.offset(mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as isize);
                current_block_30 = 11385396242402735691;
            }
            62 | 61 => {
                let mut i_0: u32 = 0;
                let mut arr = p as *mut StgSmallMutArrPtrs;
                i_0 = 0;

                while (i_0 as StgWord) < (*arr).ptrs {
                    check_object_in_compact(
                        str,
                        UNTAG_CLOSURE(
                            *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(i_0 as isize),
                        ),
                    );

                    i_0 = i_0.wrapping_add(1);
                }

                p = p.offset(
                    ((size_of::<StgSmallMutArrPtrs>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as StgWord)
                        .wrapping_add((*arr).ptrs) as isize,
                );

                current_block_30 = 11385396242402735691;
            }
            63 => {
                p = p.offset(
                    (size_of::<StgCompactNFData>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_30 = 11385396242402735691;
            }
            _ => {
                barf(c"verify_consistency_block".as_ptr());
            }
        }

        match current_block_30 {
            17352930546238167574 => {
                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_30 = 11385396242402735691;
            }
            7711255570521815756 => {
                check_object_in_compact(
                    str,
                    UNTAG_CLOSURE(*(&raw mut (*q).payload as *mut *mut StgClosure_).offset(0)
                        as *mut StgClosure),
                );

                current_block_30 = 16517549058459909004;
            }
            _ => {}
        }

        match current_block_30 {
            16517549058459909004 => {
                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );
            }
            _ => {}
        }
    }
}

unsafe fn verify_consistency_loop(mut str: *mut StgCompactNFData) {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    block = compactGetFirstBlock(str);

    loop {
        verify_consistency_block(str, block);
        block = (*block).next as *mut StgCompactNFDataBlock;

        if !(!block.is_null() && !(*block).owner.is_null()) {
            break;
        }
    }
}

unsafe fn verifyCompact(mut str: *mut StgCompactNFData) {
    if RtsFlags.DebugFlags.sanity {
        verify_consistency_loop(str);
    }
}

unsafe fn any_needs_fixup(mut block: *mut StgCompactNFDataBlock) -> bool {
    loop {
        if (*block).self_0 != block {
            return true;
        }

        block = (*block).next as *mut StgCompactNFDataBlock;

        if !(!block.is_null() && !(*block).owner.is_null()) {
            break;
        }
    }

    return false;
}

unsafe fn spew_failing_pointer(
    mut fixup_table: *mut StgWord,
    mut count: u32,
    mut address: StgWord,
) {
    let mut i: u32 = 0;
    let mut key: StgWord = 0;
    let mut value: StgWord = 0;
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();
    let mut size: StgWord = 0;
    debugBelch(
        c"Failed to adjust 0x%llx. Block dump follows...\n".as_ptr(),
        address,
    );
    i = 0;

    while i < count {
        key = *fixup_table.offset((2 as u32).wrapping_mul(i) as isize);
        value = *fixup_table.offset((2 as u32).wrapping_mul(i).wrapping_add(1 as u32) as isize);
        block = value as *mut StgCompactNFDataBlock;
        bd = Bdescr(block as StgPtr);
        size = ((*bd).c2rust_unnamed.free as W_).wrapping_sub((*bd).start as W_) as StgWord;

        debugBelch(
            c"%u: was 0x%llx-0x%llx, now 0x%llx-0x%llx\n".as_ptr(),
            i,
            key,
            key.wrapping_add(size),
            value,
            value.wrapping_add(size),
        );

        i = i.wrapping_add(1);
    }
}

unsafe fn find_pointer(
    mut fixup_table: *mut StgWord,
    mut count: u32,
    mut q: *mut StgClosure,
) -> *mut StgCompactNFDataBlock {
    let mut address: StgWord = q as StgWord;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    let mut key: StgWord = 0;
    let mut value: StgWord = 0;
    let mut bd = null_mut::<bdescr>();
    a = 0;
    b = count;

    while a < b.wrapping_sub(1 as u32) {
        c = a.wrapping_add(b).wrapping_div(2 as u32);
        key = *fixup_table.offset(c.wrapping_mul(2 as u32) as isize);
        value = *fixup_table.offset(c.wrapping_mul(2 as u32).wrapping_add(1 as u32) as isize);

        if key > address {
            b = c;
        } else {
            a = c;
        }
    }

    if a < b {
        key = *fixup_table.offset(a.wrapping_mul(2 as u32) as isize);
        value = *fixup_table.offset(a.wrapping_mul(2 as u32).wrapping_add(1 as u32) as isize);

        if !(key > address) {
            bd = Bdescr(value as StgPtr);

            if !(key.wrapping_add(((*bd).blocks as u64).wrapping_mul(BLOCK_SIZE) as StgWord)
                <= address)
            {
                return value as *mut StgCompactNFDataBlock;
            }
        }
    }

    spew_failing_pointer(fixup_table, count, address);

    return null_mut::<StgCompactNFDataBlock>();
}

unsafe fn fixup_one_pointer(
    mut fixup_table: *mut StgWord,
    mut count: u32,
    mut p: *mut *mut StgClosure,
) -> bool {
    let mut tag: StgWord = 0;
    let mut q = null_mut::<StgClosure>();
    let mut block = null_mut::<StgCompactNFDataBlock>();
    q = *p;
    tag = GET_CLOSURE_TAG(q);
    q = UNTAG_CLOSURE(q);

    if !(q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end) {
        return true;
    }

    block = find_pointer(fixup_table, count, q);

    if block.is_null() {
        return false;
    }

    if block == (*block).self_0 {
        return true;
    }

    q = (q as W_)
        .wrapping_sub((*block).self_0 as W_)
        .wrapping_add(block as W_) as *mut StgClosure;
    *p = TAG_CLOSURE(tag, q);

    return true;
}

unsafe fn fixup_mut_arr_ptrs(
    mut fixup_table: *mut StgWord,
    mut count: u32,
    mut a: *mut StgMutArrPtrs,
) -> bool {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    p = (&raw mut (*a).payload as *mut *mut StgClosure).offset(0) as *mut *mut StgClosure as StgPtr;
    q = (&raw mut (*a).payload as *mut *mut StgClosure).offset((*a).ptrs as isize)
        as *mut *mut StgClosure as StgPtr;

    while p < q {
        if !fixup_one_pointer(fixup_table, count, p as *mut *mut StgClosure) {
            return false;
        }

        p = p.offset(1);
    }

    return true;
}

unsafe fn fixup_block(
    mut block: *mut StgCompactNFDataBlock,
    mut fixup_table: *mut StgWord,
    mut count: u32,
) -> bool {
    let mut info = null::<StgInfoTable>();
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();
    bd = Bdescr(block as StgPtr);
    p = (*bd).start.offset(
        (size_of::<StgCompactNFDataBlock>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize,
    );

    while p < (*bd).c2rust_unnamed.free {
        if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/CNF.c".as_ptr(), 919);
        }

        info = get_itbl(p as *mut StgClosure);

        let mut current_block_38: u64;

        match (*info).r#type {
            2 => {
                if !fixup_one_pointer(
                    fixup_table,
                    count,
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                ) {
                    return false;
                }

                current_block_38 = 10418810333188329779;
            }
            3 => {
                current_block_38 = 10418810333188329779;
            }
            4 => {
                if !fixup_one_pointer(
                    fixup_table,
                    count,
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                ) {
                    return false;
                }

                current_block_38 = 3709678680780759592;
            }
            5 => {
                current_block_38 = 3709678680780759592;
            }
            6 => {
                current_block_38 = 12116617988122820262;
            }
            1 | 50 | 7 => {
                let mut end = null_mut::<StgWord>();
                end = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end {
                    if !fixup_one_pointer(fixup_table, count, p as *mut *mut StgClosure) {
                        return false;
                    }

                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
                current_block_38 = 17784502470059252271;
            }
            42 => {
                p = p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize);
                current_block_38 = 17784502470059252271;
            }
            46 | 45 => {
                fixup_mut_arr_ptrs(fixup_table, count, p as *mut StgMutArrPtrs);
                p = p.offset(mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as isize);
                current_block_38 = 17784502470059252271;
            }
            62 | 61 => {
                let mut i: u32 = 0;
                let mut arr = p as *mut StgSmallMutArrPtrs;
                i = 0;

                while (i as StgWord) < (*arr).ptrs {
                    if !fixup_one_pointer(
                        fixup_table,
                        count,
                        (&raw mut (*arr).payload as *mut *mut StgClosure).offset(i as isize)
                            as *mut *mut StgClosure,
                    ) {
                        return false;
                    }

                    i = i.wrapping_add(1);
                }

                p = p.offset(
                    ((size_of::<StgSmallMutArrPtrs>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as StgWord)
                        .wrapping_add((*arr).ptrs) as isize,
                );

                current_block_38 = 17784502470059252271;
            }
            63 => {
                if p == (*bd).start.offset(
                    (size_of::<StgCompactNFDataBlock>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                ) {
                    p = p.offset(
                        (size_of::<StgCompactNFData>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            as isize,
                    );

                    current_block_38 = 17784502470059252271;
                } else {
                    current_block_38 = 13352897668631705285;
                }
            }
            _ => {
                current_block_38 = 13352897668631705285;
            }
        }

        match current_block_38 {
            13352897668631705285 => {
                debugBelch(
                    c"Invalid non-NFData closure (type %d) in Compact\n".as_ptr(),
                    (*info).r#type,
                );

                return false;
            }
            10418810333188329779 => {
                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_38 = 17784502470059252271;
            }
            3709678680780759592 => {
                if !fixup_one_pointer(
                    fixup_table,
                    count,
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                ) {
                    return false;
                }

                current_block_38 = 12116617988122820262;
            }
            _ => {}
        }

        match current_block_38 {
            12116617988122820262 => {
                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );
            }
            _ => {}
        }
    }

    return true;
}

unsafe fn cmp_fixup_table_item(mut e1: *const c_void, mut e2: *const c_void) -> i32 {
    let mut w1 = e1 as *const StgWord;
    let mut w2 = e2 as *const StgWord;

    if *w1 > *w2 {
        return 1;
    } else if *w1 < *w2 {
        return -1;
    } else {
        return 0;
    };
}

unsafe fn build_fixup_table(
    mut block: *mut StgCompactNFDataBlock,
    mut pcount: *mut u32,
) -> *mut StgWord {
    let mut count: u32 = 0;
    let mut tmp = null_mut::<StgCompactNFDataBlock>();
    let mut table = null_mut::<StgWord>();
    count = 0;
    tmp = block;

    loop {
        count = count.wrapping_add(1);
        tmp = (*tmp).next as *mut StgCompactNFDataBlock;

        if !(!tmp.is_null() && !(*tmp).owner.is_null()) {
            break;
        }
    }

    table = stgMallocBytes(
        (size_of::<StgWord>() as usize)
            .wrapping_mul(2 as usize)
            .wrapping_mul(count as usize),
        c"build_fixup_table".as_ptr(),
    ) as *mut StgWord;

    count = 0;

    loop {
        *table.offset(count.wrapping_mul(2 as u32) as isize) = (*block).self_0 as W_ as StgWord;
        *table.offset(count.wrapping_mul(2 as u32).wrapping_add(1 as u32) as isize) =
            block as W_ as StgWord;
        count = count.wrapping_add(1);
        block = (*block).next as *mut StgCompactNFDataBlock;

        if !(!block.is_null() && !(*block).owner.is_null()) {
            break;
        }
    }

    qsort(
        table as *mut c_void,
        count as usize,
        (size_of::<StgWord>() as usize).wrapping_mul(2 as usize),
        Some(cmp_fixup_table_item as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );

    *pcount = count;

    return table;
}

unsafe fn fixup_loop(
    mut block: *mut StgCompactNFDataBlock,
    mut proot: *mut *mut StgClosure,
) -> bool {
    let mut current_block: u64;
    let mut table = null_mut::<StgWord>();
    let mut ok: bool = false;
    let mut count: u32 = 0;
    table = build_fixup_table(block, &raw mut count);

    loop {
        if !fixup_block(block, table, count) {
            ok = false;
            current_block = 12850912279164075601;
            break;
        } else {
            block = (*block).next as *mut StgCompactNFDataBlock;

            if !(!block.is_null() && !(*block).owner.is_null()) {
                current_block = 6873731126896040597;
                break;
            }
        }
    }

    match current_block {
        6873731126896040597 => {
            ok = fixup_one_pointer(table, count, proot);
        }
        _ => {}
    }

    stgFree(table as *mut c_void);

    return ok;
}

unsafe fn fixup_early(mut str: *mut StgCompactNFData, mut block: *mut StgCompactNFDataBlock) {
    let mut last = null_mut::<StgCompactNFDataBlock>();

    loop {
        last = block;
        block = (*block).next as *mut StgCompactNFDataBlock;

        if block.is_null() {
            break;
        }
    }

    (*str).last = last;
}

unsafe fn fixup_late(mut str: *mut StgCompactNFData, mut block: *mut StgCompactNFDataBlock) {
    let mut nursery = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();
    let mut totalW: StgWord = 0;
    nursery = block;
    totalW = 0;

    loop {
        (*block).self_0 = block as *mut StgCompactNFDataBlock_;
        bd = Bdescr(block as StgPtr);
        totalW = totalW.wrapping_add(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as StgWord);

        if !(*block).owner.is_null() {
            if (*bd).c2rust_unnamed.free != (*bd).start {
                nursery = block;
            }

            (*block).owner = str as *mut StgCompactNFData_;
        }

        block = (*block).next as *mut StgCompactNFDataBlock;

        if block.is_null() {
            break;
        }
    }

    (*str).nursery = nursery;
    bd = Bdescr(nursery as StgPtr);
    (*str).hp = (*bd).c2rust_unnamed.free;
    (*str).hpLim = (*bd)
        .start
        .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize);
    (*str).totalW = totalW;
}

unsafe fn maybe_fixup_internal_pointers(
    mut block: *mut StgCompactNFDataBlock,
    mut root: *mut StgClosure,
) -> *mut StgClosure {
    let mut ok: bool = false;
    let mut proot = null_mut::<*mut StgClosure>();

    if !any_needs_fixup(block) {
        return root;
    }

    if RtsFlags.DebugFlags.compact {
        debugBelch(
            c"Compact imported at the wrong address, will fix up internal pointers\n".as_ptr(),
        );
    }

    proot = &raw mut root;
    ok = fixup_loop(block, proot);

    if !ok {
        *proot = null_mut::<StgClosure>();
    }

    return *proot;
}

unsafe fn compactFixupPointers(
    mut str: *mut StgCompactNFData,
    mut root: *mut StgClosure,
) -> StgPtr {
    let mut block = null_mut::<StgCompactNFDataBlock>();
    let mut bd = null_mut::<bdescr>();
    let mut total_blocks: StgWord = 0;
    block = compactGetFirstBlock(str);
    fixup_early(str, block);
    root = maybe_fixup_internal_pointers(block, root);
    fixup_late(str, block);
    bd = Bdescr(block as StgPtr);
    total_blocks = (*str).totalW.wrapping_div(BLOCK_SIZE_W as StgWord);

    let mut __r = pthread_mutex_lock(&raw mut sm_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/sm/CNF.c".as_ptr(),
            1165,
            __r,
        );
    }

    if ((*bd).r#gen == g0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 1166);
    }

    if ((*g0).n_compact_blocks_in_import >= total_blocks) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/CNF.c".as_ptr(), 1167);
    }

    (*g0).n_compact_blocks_in_import = ((*g0).n_compact_blocks_in_import as StgWord)
        .wrapping_sub(total_blocks) as memcount as memcount;
    (*g0).n_compact_blocks =
        ((*g0).n_compact_blocks as StgWord).wrapping_add(total_blocks) as memcount as memcount;
    dbl_link_remove(bd, &raw mut (*g0).compact_blocks_in_import);
    dbl_link_onto(bd, &raw mut (*g0).compact_objects);

    if pthread_mutex_unlock(&raw mut sm_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/sm/CNF.c".as_ptr(),
            1172,
        );
    }

    if !root.is_null() {
        verify_consistency_loop(str);
    }

    return root as StgPtr;
}
