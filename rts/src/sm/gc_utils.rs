use crate::capability::n_numa_nodes;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::spin_lock::{ACQUIRE_SPIN_LOCK, RELEASE_SPIN_LOCK, SpinLock, SpinLock_};
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
use crate::rts_flags::RtsFlags;
use crate::sm::block_alloc::{allocLargeChunkOnNode, countBlocks};
use crate::sm::gc::WORK_UNIT_WORDS;
use crate::sm::gc::{WORK_UNIT_WORDS, notifyTodoBlock};
use crate::sm::gc_thread::{gc_threads, gen_workspace, n_gc_threads};
use crate::sm::gc_utils::allocBlock_sync;
use crate::sm::gct_decl::gct;
use crate::sm::gct_decl::gct;
use crate::trace::{DEBUG_RTS, trace_};
use crate::ws_deque::{dequeElements, looksEmptyWSDeque, popWSDeque, pushWSDeque, stealWSDeque};

#[inline]
pub(crate) unsafe fn allocBlock_sync() -> *mut bdescr {
    return allocGroup_sync(1);
}

#[inline]
pub(crate) unsafe fn allocBlockOnNode_sync(mut node: u32) -> *mut bdescr {
    return allocGroupOnNode_sync(node, 1);
}

#[inline]
pub(crate) unsafe fn isPartiallyFull(mut bd: *mut bdescr) -> bool {
    return (*bd).c2rust_unnamed.free.offset(WORK_UNIT_WORDS as isize)
        < (*bd).start.offset(BLOCK_SIZE_W as isize);
}

#[inline]
pub(crate) unsafe fn recordMutableGen_GC(mut p: *mut StgClosure, mut gen_no: u32) {
    let mut bd = null_mut::<bdescr>();
    bd = *(*gct).mut_lists.offset(gen_no as isize);

    if (*bd).c2rust_unnamed.free >= (*bd).start.offset(BLOCK_SIZE_W as isize) {
        let mut new_bd = null_mut::<bdescr>();
        new_bd = allocBlock_sync();
        (*new_bd).link = bd as *mut bdescr_;
        bd = new_bd;

        let ref mut fresh12 = *(*gct).mut_lists.offset(gen_no as isize);
        *fresh12 = bd;
    }

    let fresh9 = (*bd).c2rust_unnamed.free;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(1);
    *fresh9 = p as StgWord;
}

static mut gc_alloc_block_sync: SpinLock = SpinLock_ {
    lock: 0,
    spin: 0,
    r#yield: 0,
};

unsafe fn allocGroup_sync(mut n: u32) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    let mut node: u32 = (*gct).thread_index.wrapping_rem(n_numa_nodes);
    ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    bd = allocGroupOnNode(node, n as W_);
    RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);

    return bd;
}

unsafe fn allocGroupOnNode_sync(mut node: u32, mut n: u32) -> *mut bdescr {
    let mut bd = null_mut::<bdescr>();
    ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    bd = allocGroupOnNode(node, n as W_);
    RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);

    return bd;
}

unsafe fn allocBlocks_sync(mut n: u32, mut hd: *mut *mut bdescr) -> u32 {
    let mut bd = null_mut::<bdescr>();
    let mut i: u32 = 0;
    let mut node: u32 = (*gct).thread_index.wrapping_rem(n_numa_nodes);
    ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    bd = allocLargeChunkOnNode(node, 1, n as W_);
    n = (*bd).blocks as u32;
    i = 0;

    while i < n {
        (*bd.offset(i as isize)).blocks = 1;

        let ref mut fresh9 = (*bd.offset(i as isize)).link;
        *fresh9 = bd.offset(i.wrapping_add(1 as u32) as isize) as *mut bdescr as *mut bdescr_;

        let ref mut fresh10 = (*bd.offset(i as isize)).c2rust_unnamed.free;
        *fresh10 = (*bd.offset(i as isize)).start;
        i = i.wrapping_add(1);
    }

    let ref mut fresh11 = (*bd.offset(n.wrapping_sub(1 as u32) as isize)).link;
    *fresh11 = null_mut::<bdescr_>();
    RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    *hd = bd;

    return n;
}

unsafe fn freeChain_sync(mut bd: *mut bdescr) {
    ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    freeChain(bd);
    RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
}

unsafe fn freeGroup_sync(mut bd: *mut bdescr) {
    ACQUIRE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
    freeGroup(bd);
    RELEASE_SPIN_LOCK(&raw mut gc_alloc_block_sync);
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
        if (*bd).link.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 115);
        }

        return bd;
    }

    return null_mut::<bdescr>();
}

unsafe fn steal_todo_block(mut g: u32) -> *mut bdescr {
    let mut n: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    n = 0;

    while n < n_gc_threads {
        if !(n == (*gct).thread_index) {
            bd = stealWSDeque(
                (*(&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_q,
            ) as *mut bdescr;

            if !bd.is_null() {
                return bd;
            }
        }

        n = n.wrapping_add(1);
    }

    return null_mut::<bdescr>();
}

unsafe fn push_scanned_block(mut bd: *mut bdescr, mut ws: *mut gen_workspace) {
    if !bd.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 144);
    }

    if (*bd).link.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 145);
    }

    if ((*bd).r#gen == (*ws).0.r#gen) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 146);
    }

    if ((*bd).u.scan == (*bd).c2rust_unnamed.free) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 147);
    }

    if (*bd).blocks == 1
        && (*bd)
            .start
            .offset(BLOCK_SIZE_W as isize)
            .offset_from((*bd).c2rust_unnamed.free) as i64
            > WORK_UNIT_WORDS as i64
    {
        (*bd).link = (*ws).0.part_list as *mut bdescr_;
        (*ws).0.part_list = bd;
        (*ws).0.n_part_blocks = (*ws).0.n_part_blocks.wrapping_add((*bd).blocks as StgWord);
        (*ws).0.n_part_words = (*ws)
            .0
            .n_part_words
            .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as StgWord);

        if RtsFlags.DebugFlags.sanity {
            if (countBlocks((*ws).0.part_list) == (*ws).0.n_part_blocks) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 159);
            }
        }
    } else {
        (*bd).link = (*ws).0.scavd_list as *mut bdescr_;
        (*ws).0.scavd_list = bd;
        (*ws).0.n_scavd_blocks = (*ws).0.n_scavd_blocks.wrapping_add((*bd).blocks as StgWord);
        (*ws).0.n_scavd_words = (*ws)
            .0
            .n_scavd_words
            .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as StgWord);

        if RtsFlags.DebugFlags.sanity {
            if (countBlocks((*ws).0.scavd_list) == (*ws).0.n_scavd_blocks) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 169);
            }
        }
    };
}

unsafe fn push_todo_block(mut bd: *mut bdescr, mut ws: *mut gen_workspace) {
    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"push todo block %p (%ld words), step %d, todo_q: %ld".as_ptr(),
            (*bd).start,
            (*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as i64 as u64,
            (*(*ws).0.r#gen).no,
            dequeElements((*ws).0.todo_q),
        );
    }

    if (*bd).link.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 180);
    }

    if !pushWSDeque((*ws).0.todo_q, bd as *mut c_void) {
        (*bd).link = (*ws).0.todo_overflow as *mut bdescr_;
        (*ws).0.todo_overflow = bd;
        (*ws).0.n_todo_overflow = (*ws).0.n_todo_overflow.wrapping_add(1);

        (*gct).max_n_todo_overflow = ({
            let mut _a: W_ = (*gct).max_n_todo_overflow as W_;
            let mut _b: W_ = (*ws).0.n_todo_overflow as W_;

            if _a <= _b { _b } else { _a as W_ }
        });
    }

    notifyTodoBlock();
}

unsafe fn todo_block_full(mut size: u32, mut ws: *mut gen_workspace) -> StgPtr {
    let mut urgent_to_push: bool = false;
    let mut can_extend: bool = false;
    let mut p = null_mut::<StgWord>();
    let mut bd = null_mut::<bdescr>();
    (*ws).0.todo_free = (*ws).0.todo_free.offset(-(size as isize));
    bd = (*ws).0.todo_bd;

    if !bd.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 247);
    }

    if (*bd).link.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 248);
    }

    if ((*bd).r#gen == (*ws).0.r#gen) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 249);
    }

    urgent_to_push = looksEmptyWSDeque((*ws).0.todo_q) as i32 != 0
        && (*ws).0.todo_free.offset_from((*bd).u.scan) as i64 >= (WORK_UNIT_WORDS / 2) as i64;

    can_extend = (*ws).0.todo_free.offset(size as isize)
        <= (*bd)
            .start
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize)
        && (*ws).0.todo_free < (*(*ws).0.todo_bd).start.offset(BLOCK_SIZE_W as isize);

    if !urgent_to_push && can_extend as i32 != 0 {
        (*ws).0.todo_lim = ({
            let mut _a =
                (*bd).start.offset(((*bd).blocks as usize).wrapping_mul(
                    ((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize),
                ) as isize);

            let mut _b = (*ws).0.todo_lim.offset(
                ({
                    let mut _a_0 = 128;
                    let mut _b_0 = size as i32;
                    (if _a_0 <= _b_0 {
                        _b_0 as i32
                    } else {
                        _a_0 as i32
                    })
                }) as isize,
            );

            if _a <= _b { _a as StgPtr } else { _b as StgPtr }
        });

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"increasing limit for %p to %p".as_ptr(),
                (*bd).start,
                (*ws).0.todo_lim,
            );
        }

        p = (*ws).0.todo_free;
        (*ws).0.todo_free = (*ws).0.todo_free.offset(size as isize);

        return p;
    }

    (*gct).copied = (*gct).copied.wrapping_add(
        (*ws)
            .0
            .todo_free
            .offset_from((&raw mut (*bd).c2rust_unnamed.free).load(Ordering::Relaxed))
            as i64 as W_,
    );

    (&raw mut (*bd).c2rust_unnamed.free).store((*ws).0.todo_free, Ordering::Relaxed);

    if ((*bd).u.scan >= (*bd).start && (*bd).u.scan <= (*bd).c2rust_unnamed.free) as i32 as i64 != 0
    {
    } else {
        _assertFail(c"rts/sm/GCUtils.c".as_ptr(), 289);
    }

    if bd != (*gct).scan_bd {
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

unsafe fn alloc_todo_block(mut ws: *mut gen_workspace, mut size: u32) -> StgPtr {
    let mut bd = null_mut::<bdescr>();
    bd = (*ws).0.part_list;

    if !bd.is_null()
        && (*bd)
            .start
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize)
            .offset_from((*bd).c2rust_unnamed.free) as i64
            > size as i32 as i64
    {
        (*ws).0.part_list = (*bd).link as *mut bdescr;
        (*ws).0.n_part_blocks = (*ws).0.n_part_blocks.wrapping_sub((*bd).blocks as StgWord);
        (*ws).0.n_part_words = (*ws)
            .0
            .n_part_words
            .wrapping_sub((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as StgWord);
    } else {
        if size as usize > BLOCK_SIZE_W {
            bd = allocGroup_sync(
                (((size as usize).wrapping_mul(size_of::<W_>() as usize) as W_)
                    .wrapping_add(BLOCK_SIZE as W_)
                    .wrapping_sub(1 as W_)
                    & !BLOCK_MASK as W_)
                    .wrapping_div(BLOCK_SIZE as W_) as u32,
            );
        } else if !(*gct).free_blocks.is_null() {
            bd = (*gct).free_blocks;
            (*gct).free_blocks = (*bd).link as *mut bdescr;
        } else {
            let mut chunk_size: StgWord = 16;

            let mut n_blocks: StgWord = ({
                let mut _a: StgWord = chunk_size as StgWord;
                let mut _b: StgWord = (1 << 20 - 12 - 1) as StgWord;

                if _a <= _b { _a } else { _b as StgWord }
            });

            allocBlocks_sync(n_blocks as u32, &raw mut bd);
            (*gct).free_blocks = (*bd).link as *mut bdescr;
        }

        initBdescr(bd, (*ws).0.r#gen, (*(*ws).0.r#gen).to as *mut generation);
        (&raw mut (*bd).u.scan).store(
            (&raw mut (*bd).start).load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
        (&raw mut (*bd).flags).store(1, Ordering::Release);
    }

    (*bd).link = null_mut::<bdescr_>();
    (*ws).0.todo_bd = bd;
    (*ws).0.todo_free = (*bd).c2rust_unnamed.free;

    (*ws).0.todo_lim = ({
        let mut _a = (*bd).start.offset(
            ((*bd).blocks as usize)
                .wrapping_mul(((1 as usize) << 12 as i32).wrapping_div(size_of::<W_>() as usize))
                as isize,
        );

        let mut _b = (*bd).c2rust_unnamed.free.offset(
            ({
                let mut _a_0 = 128;
                let mut _b_0 = size as i32;
                (if _a_0 <= _b_0 {
                    _b_0 as i32
                } else {
                    _a_0 as i32
                })
            }) as isize,
        );

        if _a <= _b { _a as StgPtr } else { _b as StgPtr }
    });

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"alloc new todo block %p for gen  %d".as_ptr(),
            (&raw mut (*bd).c2rust_unnamed.free).load(Ordering::Relaxed),
            (*(*ws).0.r#gen).no,
        );
    }

    return (*ws).0.todo_free;
}
