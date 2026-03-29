use crate::capability::recordClosureMutated;
use crate::ffi::hs_ffi::HS_INT32_MAX;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::storage::block::{
    BF_COMPACT, BF_PINNED, BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, Bdescr,
    allocGroup, bdescr, bdescr_, dbl_link_onto, dbl_link_remove, freeGroup,
};
use crate::ffi::rts::storage::block::{Bdescr, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, TAG_CLOSURE, UNTAG_CLOSURE, arr_words_sizeW, get_itbl, mut_arr_ptrs_sizeW,
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
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, reportHeapOverflow, stg_exit};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    stg_COMPACT_NFDATA_CLEAN_info, stg_COMPACT_NFDATA_DIRTY_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
use crate::ffi::stg::types::{StgPtr, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::hash::{HashTable, insertHashTable};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::cnf::{objectGetCompact, objectGetCompactBlock};
use crate::sm::should_compact::{
    SHOULDCOMPACT_IN_CNF, SHOULDCOMPACT_NOTIN_CNF, SHOULDCOMPACT_PINNED, SHOULDCOMPACT_STATIC,
};
use crate::trace::{DEBUG_RTS, trace_};

#[inline]
pub(crate) unsafe fn objectGetCompactBlock(
    mut closure: *mut StgClosure,
) -> *mut StgCompactNFDataBlock {
    let mut object_block = null_mut::<bdescr>();
    let mut head_block = null_mut::<bdescr>();
    object_block = Bdescr(closure as StgPtr);

    if (*object_block).blocks == 0 {
        head_block = (*object_block).link as *mut bdescr;
    } else {
        head_block = object_block;
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

    block = allocGroup(n_blocks as W_);

    let mut current_block_38: u64;

    match operation as u32 {
        1 => {
            dbl_link_onto(block, &raw mut (*g0).compact_objects);
            (*g).n_compact_blocks = (*g)
                .n_compact_blocks
                .wrapping_add((*block).blocks as memcount);
            (*g).n_new_large_words = ((*g).n_new_large_words as StgWord)
                .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord))
                as memcount as memcount;
            current_block_38 = 9853141518545631134;
        }
        2 => {
            dbl_link_onto(block, &raw mut (*g0).compact_blocks_in_import);
            current_block_38 = 6669252993407410313;
        }
        3 => {
            current_block_38 = 6669252993407410313;
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

            current_block_38 = 9853141518545631134;
        }
        _ => {
            unreachable!();
        }
    }

    match current_block_38 {
        6669252993407410313 => {
            (*g).n_compact_blocks_in_import = (*g)
                .n_compact_blocks_in_import
                .wrapping_add((*block).blocks as memcount);
            (*g).n_new_large_words = ((*g).n_new_large_words as StgWord)
                .wrapping_add(aligned_size.wrapping_div(size_of::<StgWord>() as StgWord))
                as memcount as memcount;
        }
        _ => {}
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
        freeGroup(bd);
        block = next;
    }
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

    let ref mut fresh6 = (*(self_0 as *mut StgClosure)).header.info;
    *fresh6 = &raw const stg_COMPACT_NFDATA_CLEAN_info;
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
    (*(*str).last).next = block as *mut StgCompactNFDataBlock_;
    (*str).last = block;
    bd = Bdescr(block as StgPtr);
    (*bd).c2rust_unnamed.free =
        (block as W_).wrapping_add(size_of::<StgCompactNFDataBlock>() as W_) as StgPtr;
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

#[inline]
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

#[inline]
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

#[inline]
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
        info = get_itbl(p as *mut StgClosure);

        let mut current_block_36: u64;

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

                current_block_36 = 6935415395941392425;
            }
            3 => {
                current_block_36 = 6935415395941392425;
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

                current_block_36 = 15964801498953055232;
            }
            5 => {
                current_block_36 = 15964801498953055232;
            }
            6 => {
                current_block_36 = 13504857111530276372;
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
                current_block_36 = 11743904203796629665;
            }
            42 => {
                p = p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize);
                current_block_36 = 11743904203796629665;
            }
            46 | 45 => {
                fixup_mut_arr_ptrs(fixup_table, count, p as *mut StgMutArrPtrs);
                p = p.offset(mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as isize);
                current_block_36 = 11743904203796629665;
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

                current_block_36 = 11743904203796629665;
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

                    current_block_36 = 11743904203796629665;
                } else {
                    current_block_36 = 7626285455412833715;
                }
            }
            _ => {
                current_block_36 = 7626285455412833715;
            }
        }

        match current_block_36 {
            7626285455412833715 => {
                debugBelch(
                    c"Invalid non-NFData closure (type %d) in Compact\n".as_ptr(),
                    (*info).r#type,
                );

                return false;
            }
            6935415395941392425 => {
                p = p.offset(
                    (size_of::<StgClosure>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_36 = 11743904203796629665;
            }
            15964801498953055232 => {
                if !fixup_one_pointer(
                    fixup_table,
                    count,
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                ) {
                    return false;
                }

                current_block_36 = 13504857111530276372;
            }
            _ => {}
        }

        match current_block_36 {
            13504857111530276372 => {
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
            current_block = 8019040075682676234;
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
    (*g0).n_compact_blocks_in_import = ((*g0).n_compact_blocks_in_import as StgWord)
        .wrapping_sub(total_blocks) as memcount as memcount;
    (*g0).n_compact_blocks =
        ((*g0).n_compact_blocks as StgWord).wrapping_add(total_blocks) as memcount as memcount;
    dbl_link_remove(bd, &raw mut (*g0).compact_blocks_in_import);
    dbl_link_onto(bd, &raw mut (*g0).compact_objects);

    return root as StgPtr;
}
