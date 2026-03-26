use crate::capability::n_numa_nodes;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::block::{
    BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, allocGroupOnNode, bdescr, bdescr_, freeChain, freeGroup,
};
use crate::ffi::rts::storage::block::{BLOCK_SIZE_W, bdescr, bdescr_};
use crate::ffi::rts::storage::gc::{generation, initBdescr};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord16, StgWord32};
use crate::prelude::*;
use crate::sm::block_alloc::allocLargeChunkOnNode;
use crate::sm::gc::WORK_UNIT_WORDS;
use crate::sm::gc::WORK_UNIT_WORDS;
use crate::sm::gc_thread::gc_thread;
use crate::sm::gc_thread::{gc_thread, gen_workspace};
use crate::sm::gc_utils::allocBlock_sync;
use crate::sm::gct_decl::the_gc_thread;
use crate::sm::gct_decl::the_gc_thread;
use crate::trace::{DEBUG_RTS, trace_};
use crate::ws_deque::{dequeElements, looksEmptyWSDeque, popWSDeque, pushWSDeque};

#[inline]
pub(crate) unsafe fn allocBlock_sync() -> *mut bdescr {
    return allocGroup_sync(1 as uint32_t);
}

#[inline]
pub(crate) unsafe fn allocBlockOnNode_sync(mut node: uint32_t) -> *mut bdescr {
    return allocGroupOnNode_sync(node, 1 as uint32_t);
}

#[inline]
pub(crate) unsafe fn isPartiallyFull(mut bd: *mut bdescr) -> bool {
    return (*bd).c2rust_unnamed.free.offset(WORK_UNIT_WORDS as isize)
        < (*bd).start.offset(BLOCK_SIZE_W as isize);
}

#[inline]
pub(crate) unsafe fn recordMutableGen_GC(mut p: *mut StgClosure, mut gen_no: uint32_t) {
    let mut bd = null_mut::<bdescr>();
    bd = *(*(&raw mut the_gc_thread as *mut gc_thread))
        .mut_lists
        .offset(gen_no as isize);

    if (*bd).c2rust_unnamed.free >= (*bd).start.offset(BLOCK_SIZE_W as isize) {
        let mut new_bd = null_mut::<bdescr>();
        new_bd = allocBlock_sync();
        (*new_bd).link = bd as *mut bdescr_;
        bd = new_bd;

        let ref mut fresh12 = *(*(&raw mut the_gc_thread as *mut gc_thread))
            .mut_lists
            .offset(gen_no as isize);
        *fresh12 = bd;
    }

    let fresh13 = (*bd).c2rust_unnamed.free;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(1);
    *fresh13 = p as StgWord;
}

unsafe fn allocGroup_sync(mut n: uint32_t) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut node: uint32_t = (*(&raw mut the_gc_thread as *mut gc_thread))
        .thread_index
        .wrapping_rem(n_numa_nodes);
    bd = allocGroupOnNode(node, n as W_);

    return bd;
}

unsafe fn allocGroupOnNode_sync(mut node: uint32_t, mut n: uint32_t) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    bd = allocGroupOnNode(node, n as W_);

    return bd;
}

unsafe fn allocBlocks_sync(mut n: uint32_t, mut hd: *mut *mut bdescr) -> uint32_t {
    let mut bd = null_mut::<bdescr>();
    let mut i: uint32_t = 0;
    let mut node: uint32_t = (*(&raw mut the_gc_thread as *mut gc_thread))
        .thread_index
        .wrapping_rem(n_numa_nodes);
    bd = allocLargeChunkOnNode(node, 1 as W_, n as W_);
    n = (*bd).blocks as uint32_t;
    i = 0 as uint32_t;

    while i < n {
        (*bd.offset(i as isize)).blocks = 1 as StgWord32;

        let ref mut fresh9 = (*bd.offset(i as isize)).link;
        *fresh9 = bd.offset(i.wrapping_add(1 as uint32_t) as isize) as *mut bdescr as *mut bdescr_;

        let ref mut fresh10 = (*bd.offset(i as isize)).c2rust_unnamed.free;
        *fresh10 = (*bd.offset(i as isize)).start;
        i = i.wrapping_add(1);
    }

    let ref mut fresh11 = (*bd.offset(n.wrapping_sub(1 as uint32_t) as isize)).link;
    *fresh11 = null_mut::<bdescr_>();
    *hd = bd;

    return n;
}

unsafe fn freeChain_sync(mut bd: *mut bdescr) {
    freeChain(bd);
}

unsafe fn freeGroup_sync(mut bd: *mut bdescr) {
    freeGroup(bd);
}

unsafe fn grab_local_todo_block(mut ws: *mut gen_workspace) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    bd = (*ws).0.todo_overflow;

    if !bd.is_null() {
        (*ws).0.todo_overflow = (*bd).link as *mut bdescr;
        (*bd).link = null_mut::<bdescr_>();
        (*ws).0.n_todo_overflow = (*ws).0.n_todo_overflow.wrapping_sub(1);

        return bd;
    }

    bd = popWSDeque((*ws).0.todo_q) as *mut bdescr;

    if !bd.is_null() {
        return bd;
    }

    return null_mut::<bdescr>();
}

unsafe fn push_scanned_block(mut bd: *mut bdescr, mut ws: *mut gen_workspace) {
    if (*bd).blocks == 1 as StgWord32
        && (*bd)
            .start
            .offset(BLOCK_SIZE_W as isize)
            .offset_from((*bd).c2rust_unnamed.free) as c_long
            > WORK_UNIT_WORDS as c_long
    {
        (*bd).link = (*ws).0.part_list as *mut bdescr_;
        (*ws).0.part_list = bd;
        (*ws).0.n_part_blocks = (*ws).0.n_part_blocks.wrapping_add((*bd).blocks as StgWord);
        (*ws).0.n_part_words = (*ws)
            .0
            .n_part_words
            .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as StgWord);
    } else {
        (*bd).link = (*ws).0.scavd_list as *mut bdescr_;
        (*ws).0.scavd_list = bd;
        (*ws).0.n_scavd_blocks = (*ws).0.n_scavd_blocks.wrapping_add((*bd).blocks as StgWord);
        (*ws).0.n_scavd_words = (*ws)
            .0
            .n_scavd_words
            .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as StgWord);
    };
}

unsafe fn push_todo_block(mut bd: *mut bdescr, mut ws: *mut gen_workspace) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"push todo block %p (%ld words), step %d, todo_q: %ld\0" as *const u8 as *const c_char
                as *mut c_char,
            (*bd).start,
            (*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as c_long as c_ulong,
            (*(*ws).0.r#gen).no,
            dequeElements((*ws).0.todo_q),
        );
    }

    if !pushWSDeque((*ws).0.todo_q, bd as *mut c_void) {
        (*bd).link = (*ws).0.todo_overflow as *mut bdescr_;
        (*ws).0.todo_overflow = bd;
        (*ws).0.n_todo_overflow = (*ws).0.n_todo_overflow.wrapping_add(1);

        (*(&raw mut the_gc_thread as *mut gc_thread)).max_n_todo_overflow = ({
            let mut _a: W_ =
                (*(&raw mut the_gc_thread as *mut gc_thread)).max_n_todo_overflow as W_;

            let mut _b: W_ = (*ws).0.n_todo_overflow as W_;

            if _a <= _b { _b } else { _a as W_ }
        });
    }
}

unsafe fn todo_block_full(mut size: uint32_t, mut ws: *mut gen_workspace) -> StgPtr {
    let mut urgent_to_push: bool = false;
    let mut can_extend: bool = false;
    let mut p = null_mut::<StgWord>();
    let mut bd = null_mut::<bdescr>();
    (*ws).0.todo_free = (*ws).0.todo_free.offset(-(size as isize));
    bd = (*ws).0.todo_bd;
    urgent_to_push = looksEmptyWSDeque((*ws).0.todo_q) as c_int != 0
        && (*ws).0.todo_free.offset_from((*bd).u.scan) as c_long
            >= (WORK_UNIT_WORDS / 2 as c_int) as c_long;

    can_extend = (*ws).0.todo_free.offset(size as isize)
        <= (*bd)
            .start
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize)
        && (*ws).0.todo_free < (*(*ws).0.todo_bd).start.offset(BLOCK_SIZE_W as isize);

    if !urgent_to_push && can_extend as c_int != 0 {
        (*ws).0.todo_lim = ({
            let mut _a =
                (*bd).start.offset(((*bd).blocks as usize).wrapping_mul(
                    ((1 as usize) << 12 as c_int).wrapping_div(size_of::<W_>() as usize),
                ) as isize);

            let mut _b = (*ws).0.todo_lim.offset(
                ({
                    let mut _a_0 = 128 as c_int;
                    let mut _b_0 = size as c_int;

                    (if _a_0 <= _b_0 {
                        _b_0 as c_int
                    } else {
                        _a_0 as c_int
                    })
                }) as isize,
            );

            if _a <= _b { _a as StgPtr } else { _b as StgPtr }
        });

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
            trace_(
                b"increasing limit for %p to %p\0" as *const u8 as *const c_char as *mut c_char,
                (*bd).start,
                (*ws).0.todo_lim,
            );
        }

        p = (*ws).0.todo_free;
        (*ws).0.todo_free = (*ws).0.todo_free.offset(size as isize);

        return p;
    }

    let ref mut fresh6 = (*(&raw mut the_gc_thread as *mut gc_thread)).copied;
    *fresh6 = (*fresh6)
        .wrapping_add((*ws).0.todo_free.offset_from((*bd).c2rust_unnamed.free) as c_long as W_);
    (*bd).c2rust_unnamed.free = (*ws).0.todo_free;

    if bd != (*(&raw mut the_gc_thread as *mut gc_thread)).scan_bd {
        if (*bd).u.scan == (*bd).c2rust_unnamed.free {
            if (*bd).c2rust_unnamed.free == (*bd).start {
                freeGroup_sync(bd);
            } else {
                push_scanned_block(bd, ws);
            }
        } else {
            push_todo_block(bd, ws);
        }
    }

    (*ws).0.todo_bd = null_mut::<bdescr>();
    (*ws).0.todo_free = null_mut::<StgWord>();
    (*ws).0.todo_lim = null_mut::<StgWord>();
    alloc_todo_block(ws, size);
    p = (*ws).0.todo_free;
    (*ws).0.todo_free = (*ws).0.todo_free.offset(size as isize);

    return p;
}

unsafe fn alloc_todo_block(mut ws: *mut gen_workspace, mut size: uint32_t) -> StgPtr {
    let mut bd = null_mut::<bdescr>();
    bd = (*ws).0.part_list;

    if !bd.is_null()
        && (*bd)
            .start
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize)
            .offset_from((*bd).c2rust_unnamed.free) as c_long
            > size as c_int as c_long
    {
        (*ws).0.part_list = (*bd).link as *mut bdescr;
        (*ws).0.n_part_blocks = (*ws).0.n_part_blocks.wrapping_sub((*bd).blocks as StgWord);
        (*ws).0.n_part_words = (*ws)
            .0
            .n_part_words
            .wrapping_sub((*bd).c2rust_unnamed.free.offset_from((*bd).start) as c_long as StgWord);
    } else {
        if size as usize > BLOCK_SIZE_W {
            bd = allocGroup_sync(
                (((size as usize).wrapping_mul(size_of::<W_>() as usize) as W_)
                    .wrapping_add(BLOCK_SIZE as W_)
                    .wrapping_sub(1 as W_)
                    & !BLOCK_MASK as W_)
                    .wrapping_div(BLOCK_SIZE as W_) as uint32_t,
            );
        } else if !(*(&raw mut the_gc_thread as *mut gc_thread))
            .free_blocks
            .is_null()
        {
            bd = (*(&raw mut the_gc_thread as *mut gc_thread)).free_blocks;

            let ref mut fresh7 = (*(&raw mut the_gc_thread as *mut gc_thread)).free_blocks;
            *fresh7 = (*bd).link as *mut bdescr;
        } else {
            let mut chunk_size: StgWord = 16 as StgWord;

            let mut n_blocks: StgWord = ({
                let mut _a: StgWord = chunk_size as StgWord;
                let mut _b: StgWord =
                    ((1 as c_int) << 20 as c_int - 12 as c_int - 1 as c_int) as StgWord;

                if _a <= _b { _a } else { _b as StgWord }
            });

            allocBlocks_sync(n_blocks as uint32_t, &raw mut bd);

            let ref mut fresh8 = (*(&raw mut the_gc_thread as *mut gc_thread)).free_blocks;
            *fresh8 = (*bd).link as *mut bdescr;
        }

        initBdescr(bd, (*ws).0.r#gen, (*(*ws).0.r#gen).to as *mut generation);
        (*bd).u.scan = (*bd).start;
        (*bd).flags = 1 as StgWord16;
    }

    (*bd).link = null_mut::<bdescr_>();
    (*ws).0.todo_bd = bd;
    (*ws).0.todo_free = (*bd).c2rust_unnamed.free;

    (*ws).0.todo_lim = ({
        let mut _a = (*bd).start.offset(
            ((*bd).blocks as usize)
                .wrapping_mul(((1 as usize) << 12 as c_int).wrapping_div(size_of::<W_>() as usize))
                as isize,
        );

        let mut _b = (*bd).c2rust_unnamed.free.offset(
            ({
                let mut _a_0 = 128 as c_int;
                let mut _b_0 = size as c_int;

                (if _a_0 <= _b_0 {
                    _b_0 as c_int
                } else {
                    _a_0 as c_int
                })
            }) as isize,
        );

        if _a <= _b { _a as StgPtr } else { _b as StgPtr }
    });

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"alloc new todo block %p for gen  %d\0" as *const u8 as *const c_char as *mut c_char,
            (*bd).c2rust_unnamed.free,
            (*(*ws).0.r#gen).no,
        );
    }

    return (*ws).0.todo_free;
}
