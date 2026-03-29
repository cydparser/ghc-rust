use crate::ffi::rts::storage::block::{
    BLOCK_MASK, BLOCK_SIZE, BLOCK_SIZE_W, allocBlock_lock, allocGroup_lock, bdescr, bdescr_,
    freeGroup_lock,
};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord, StgWord16};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

pub(crate) type Arena = _Arena;

/// cbindgen:no-export
struct _Arena {
    current: *mut bdescr,
    free: *mut StgWord,
    lim: *mut StgWord,
}

static mut arena_blocks: i64 = 0;

unsafe fn newArena() -> *mut Arena {
    let mut arena = null_mut::<Arena>();
    arena = stgMallocBytes(size_of::<Arena>() as usize, c"newArena".as_ptr()) as *mut Arena;
    (*arena).current = allocBlock_lock();
    (*(*arena).current).link = null_mut::<bdescr_>();
    (*arena).free = (*(*arena).current).start as *mut StgWord;
    (*arena).lim = (*(*arena).current).start.offset(BLOCK_SIZE_W as isize) as *mut StgWord;
    arena_blocks += 1;

    return arena;
}

unsafe fn arenaAlloc(mut arena: *mut Arena, mut size: usize) -> *mut c_void {
    let mut p = null_mut::<c_void>();
    let mut size_w: u32 = 0;
    let mut req_blocks: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    size = size.wrapping_add((8 as i32 - 1 as i32) as usize) & !(8 - 1) as usize;
    size_w = size.wrapping_div(size_of::<W_>() as usize) as u32;

    if (*arena).free.offset(size_w as isize) < (*arena).lim {
        p = (*arena).free as *mut c_void;
        (*arena).free = (*arena).free.offset(size_w as isize);

        return p;
    } else {
        req_blocks = ((size as W_)
            .wrapping_add(BLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !BLOCK_MASK as W_)
            .wrapping_div(BLOCK_SIZE as W_) as u32;
        bd = allocGroup_lock(req_blocks as W_);
        arena_blocks += (*bd).blocks as i64;
        (*bd).gen_no = 0;
        (*bd).r#gen = null_mut::<generation_>();
        (*bd).dest_no = 0;
        (*bd).flags = 0;
        (*bd).c2rust_unnamed.free = (*bd).start;
        (*bd).link = (*arena).current as *mut bdescr_;
        (*arena).current = bd;
        (*arena).free = (*bd).c2rust_unnamed.free.offset(size_w as isize) as *mut StgWord;
        (*arena).lim = (*bd)
            .c2rust_unnamed
            .free
            .offset(((*bd).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as isize)
            as *mut StgWord;

        return (*bd).start as *mut c_void;
    };
}

unsafe fn arenaFree(mut arena: *mut Arena) {
    let mut bd = null_mut::<bdescr>();
    let mut next = null_mut::<bdescr>();
    bd = (*arena).current;

    while !bd.is_null() {
        next = (*bd).link as *mut bdescr;
        arena_blocks -= (*bd).blocks as i64;
        freeGroup_lock(bd);
        bd = next;
    }

    stgFree(arena as *mut c_void);
}

unsafe fn arenaBlocks() -> u64 {
    return arena_blocks as u64;
}
