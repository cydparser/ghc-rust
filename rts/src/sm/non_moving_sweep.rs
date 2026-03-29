use crate::capability::{getCapability, recordMutableCap};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::stable_name::{snEntry, stable_name_table};
use crate::ffi::rts::storage::block::{
    Bdescr, allocBlockOnNode_lock, bdescr, freeChain_lock, freeGroup,
};
use crate::ffi::rts::storage::closure_macros::get_itbl;
use crate::ffi::rts::storage::closures::{
    StgBlockingQueue, StgClosure_, StgCompactNFData, StgCompactNFDataBlock, StgMVar, StgMutVar,
    StgSelector, StgTVar, StgThunk,
};
use crate::ffi::rts::storage::gc::{memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info, stg_MUT_VAR_CLEAN_info,
    stg_MUT_VAR_DIRTY_info, stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info, stg_TVAR_CLEAN_info,
    stg_TVAR_DIRTY_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord16};
use crate::ffi::stg::{P_, W_};
use crate::prelude::*;
use crate::sm::cnf::compactFree;
use crate::sm::non_moving::{
    NONMOVING_SEGMENT_SIZE, NonmovingSegment, nonmoving_block_idx, nonmovingClosureBeingSwept,
    nonmovingGetMark, nonmovingHeap, nonmovingMarkEpoch, nonmovingPushActiveSegment,
    nonmovingPushFilledSegment, nonmovingPushFreeSegment, nonmovingSegmentBlockCount,
    nonmovingSegmentBlockSize, nonmovingSegmentGetBlock, nonmovingSegmentInfo,
};
use crate::sm::non_moving_mark::{
    n_nonmoving_compact_blocks, n_nonmoving_large_blocks, n_nonmoving_marked_compact_blocks,
    n_nonmoving_marked_large_blocks, nonmoving_compact_objects, nonmoving_large_objects,
    nonmoving_marked_compact_objects, nonmoving_marked_large_objects, nonmovingIsAlive,
};
use crate::stable_name::{SNT_size, freeSnEntry, stableNameLock, stableNameUnlock};

type SweepResult = u32;

const SEGMENT_FILLED: SweepResult = 2;

const SEGMENT_PARTIAL: SweepResult = 1;

const SEGMENT_FREE: SweepResult = 0;

unsafe fn nonmovingSweepSegment(mut seg: *mut NonmovingSegment) -> SweepResult {
    let blk_cnt: nonmoving_block_idx = nonmovingSegmentBlockCount(seg) as nonmoving_block_idx;

    let mut found_free = false;
    let mut found_live = false;
    let mut i: nonmoving_block_idx = 0;

    while (i as i32) < blk_cnt as i32 {
        if *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) as i32
            == nonmovingMarkEpoch as i32
        {
            found_live = true;
        } else {
            *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) = 0;

            if !found_free {
                found_free = true;
                (*seg).next_free = i;
                (*nonmovingSegmentInfo(seg)).next_free_snap = i as StgWord16;

                let ref mut fresh5 = (*Bdescr(seg as StgPtr)).u.scan;
                *fresh5 = nonmovingSegmentGetBlock(seg, i) as P_ as StgPtr;
            }
        }

        if found_free as i32 != 0 && found_live as i32 != 0 {
            while (i as u32) < nonmovingSegmentBlockCount(seg) {
                if *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) as i32
                    != nonmovingMarkEpoch as i32
                {
                    *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) = 0;
                }

                i = i.wrapping_add(1);
            }

            return SEGMENT_PARTIAL;
        }

        i = i.wrapping_add(1);
    }

    if found_live {
        return SEGMENT_FILLED;
    } else {
        return SEGMENT_FREE;
    };
}

unsafe fn nonmovingClearSegment(mut seg: *mut NonmovingSegment) {
    let mut end: usize = (seg as usize).wrapping_add(NONMOVING_SEGMENT_SIZE as usize);

    memset(
        &raw mut (*seg).bitmap as *mut c_void,
        0,
        end.wrapping_sub(&raw mut (*seg).bitmap as usize),
    );
}

unsafe fn nonmovingClearSegmentFreeBlocks(mut seg: *mut NonmovingSegment) {
    let mut block_size = nonmovingSegmentBlockSize(seg);
    let mut p_idx = 0;

    while p_idx < nonmovingSegmentBlockCount(seg) {
        if nonmovingGetMark(seg, p_idx as nonmoving_block_idx) as i32 == 0 {
            memset(
                nonmovingSegmentGetBlock(seg, p_idx as nonmoving_block_idx),
                0,
                block_size as usize,
            );
        }

        p_idx = p_idx.wrapping_add(1);
    }
}

unsafe fn nonmovingSweep() {
    while !nonmovingHeap.sweep_list.is_null() {
        let mut seg = nonmovingHeap.sweep_list;
        nonmovingHeap.sweep_list = (*seg).link;

        let mut ret = nonmovingSweepSegment(seg);

        match ret as u32 {
            0 => {
                nonmovingPushFreeSegment(seg);
            }
            1 => {
                nonmovingPushActiveSegment(seg);
            }
            2 => {
                nonmovingPushFilledSegment(seg);
            }
            _ => {
                barf(
                    c"nonmovingSweep: weird sweep return: %d\n".as_ptr(),
                    ret as u32,
                );
            }
        }
    }
}

unsafe fn is_closure_clean(mut p: *mut StgClosure) -> bool {
    let mut info = get_itbl(p);

    match (*info).r#type {
        39 | 40 => {
            let mut mvar = p as *mut StgMVar;

            if !((*mvar).head as *mut StgClosure as W_ >= mblock_address_space.0.begin
                && ((*mvar).head as *mut StgClosure as W_) < mblock_address_space.0.end)
                || (*Bdescr((*mvar).head as StgPtr)).r#gen == oldest_gen
            {
                if !((*mvar).tail as *mut StgClosure as W_ >= mblock_address_space.0.begin
                    && ((*mvar).tail as *mut StgClosure as W_) < mblock_address_space.0.end)
                    || (*Bdescr((*mvar).tail as StgPtr)).r#gen == oldest_gen
                {
                    if !((*mvar).value as W_ >= mblock_address_space.0.begin
                        && ((*mvar).value as W_) < mblock_address_space.0.end)
                        || (*Bdescr((*mvar).value as StgPtr)).r#gen == oldest_gen
                    {
                        (*mvar).header.info = &raw const stg_MVAR_CLEAN_info;

                        return true;
                    }
                }
            }

            (*mvar).header.info = &raw const stg_MVAR_DIRTY_info;

            return false;
        }
        41 => {
            let mut tvar = p as *mut StgTVar;

            if !((*tvar).current_value as W_ >= mblock_address_space.0.begin
                && ((*tvar).current_value as W_) < mblock_address_space.0.end)
                || (*Bdescr((*tvar).current_value as StgPtr)).r#gen == oldest_gen
            {
                if !((*tvar).first_watch_queue_entry as *mut StgClosure as W_
                    >= mblock_address_space.0.begin
                    && ((*tvar).first_watch_queue_entry as *mut StgClosure as W_)
                        < mblock_address_space.0.end)
                    || (*Bdescr((*tvar).first_watch_queue_entry as StgPtr)).r#gen == oldest_gen
                {
                    (*tvar).header.info = &raw const stg_TVAR_CLEAN_info;

                    return true;
                }
            }

            (*tvar).header.info = &raw const stg_TVAR_DIRTY_info;

            return false;
        }
        15 | 16 | 17 | 19 | 20 | 18 => {
            let mut end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_
                as StgPtr)
                .offset((*info).layout.payload.ptrs as isize);

            let mut q = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as StgPtr;

            while q < end {
                if !(!(*q as *mut StgClosure as W_ >= mblock_address_space.0.begin
                    && (*q as *mut StgClosure as W_) < mblock_address_space.0.end)
                    || (*Bdescr(*q as StgPtr)).r#gen == oldest_gen)
                {
                    return false;
                }

                q = q.offset(1);
            }

            return true;
        }
        8 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3 | 5 | 6 | 4 | 50 => {
            let mut end_0 = (&raw mut (*p).payload as *mut *mut StgClosure_ as StgPtr)
                .offset((*info).layout.payload.ptrs as isize);

            let mut q_0 = &raw mut (*p).payload as *mut *mut StgClosure_ as StgPtr;

            while q_0 < end_0 {
                if !(!(*q_0 as *mut StgClosure as W_ >= mblock_address_space.0.begin
                    && (*q_0 as *mut StgClosure as W_) < mblock_address_space.0.end)
                    || (*Bdescr(*q_0 as StgPtr)).r#gen == oldest_gen)
                {
                    return false;
                }

                q_0 = q_0.offset(1);
            }

            return true;
        }
        49 => return false,
        47 | 48 => {
            if !(!((*(p as *mut StgMutVar)).var as W_ >= mblock_address_space.0.begin
                && ((*(p as *mut StgMutVar)).var as W_) < mblock_address_space.0.end)
                || (*Bdescr((*(p as *mut StgMutVar)).var as StgPtr)).r#gen == oldest_gen)
            {
                (*p).header.info = &raw const stg_MUT_VAR_DIRTY_info;

                return false;
            } else {
                (*p).header.info = &raw const stg_MUT_VAR_CLEAN_info;

                return true;
            }
        }
        37 => {
            let mut bq = p as *mut StgBlockingQueue;

            if !((*bq).bh as W_ >= mblock_address_space.0.begin
                && ((*bq).bh as W_) < mblock_address_space.0.end)
                || (*Bdescr((*bq).bh as StgPtr)).r#gen == oldest_gen
            {
                if !((*bq).owner as *mut StgClosure as W_ >= mblock_address_space.0.begin
                    && ((*bq).owner as *mut StgClosure as W_) < mblock_address_space.0.end)
                    || (*Bdescr((*bq).owner as StgPtr)).r#gen == oldest_gen
                {
                    if !((*bq).queue as *mut StgClosure as W_ >= mblock_address_space.0.begin
                        && ((*bq).queue as *mut StgClosure as W_) < mblock_address_space.0.end)
                        || (*Bdescr((*bq).queue as StgPtr)).r#gen == oldest_gen
                    {
                        if !((*bq).link as *mut StgClosure as W_ >= mblock_address_space.0.begin
                            && ((*bq).link as *mut StgClosure as W_) < mblock_address_space.0.end)
                            || (*Bdescr((*bq).link as StgPtr)).r#gen == oldest_gen
                        {
                            (*bq).header.info = &raw const stg_BLOCKING_QUEUE_CLEAN_info;

                            return true;
                        }
                    }
                }
            }

            (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;

            return false;
        }
        22 => {
            return !((*(p as *mut StgSelector)).selectee as W_ >= mblock_address_space.0.begin
                && ((*(p as *mut StgSelector)).selectee as W_) < mblock_address_space.0.end)
                || (*Bdescr((*(p as *mut StgSelector)).selectee as StgPtr)).r#gen == oldest_gen;
        }
        42 => return true,
        _ => return false,
    };
}

unsafe fn nonmovingSweepMutLists() {
    let mut n: u32 = 0;

    while n < getNumCapabilities() as u32 {
        let mut cap = getCapability(n);
        let mut old_mut_list = *(*cap).mut_lists.offset((*oldest_gen).no as isize);
        let ref mut fresh6 = *(*cap).mut_lists.offset((*oldest_gen).no as isize);
        *fresh6 = allocBlockOnNode_lock((*cap).node);

        let mut bd = old_mut_list;

        while !bd.is_null() {
            let mut p = (*bd).start;

            while p < (*bd).c2rust_unnamed.free {
                let mut q = p as *mut *mut StgClosure;

                if nonmovingIsAlive(*q) as i32 != 0 && !is_closure_clean(*q) {
                    recordMutableCap(*q, cap, (*oldest_gen).no);
                }

                p = p.offset(1);
            }

            bd = (*bd).link as *mut bdescr;
        }

        freeChain_lock(old_mut_list);
        n = n.wrapping_add(1);
    }
}

unsafe fn freeChain_lock_max(mut bd: *mut bdescr, mut max_dur: i32) {
    let mut next_bd = null_mut::<bdescr>();
    let mut i = 0;

    while !bd.is_null() {
        next_bd = (*bd).link as *mut bdescr;
        freeGroup(bd);
        bd = next_bd;

        if i == max_dur {
            i = 0;
        }

        i += 1;
    }
}

unsafe fn nonmovingSweepLargeObjects() {
    freeChain_lock_max(nonmoving_large_objects, 10000);
    nonmoving_large_objects = nonmoving_marked_large_objects;
    n_nonmoving_large_blocks = n_nonmoving_marked_large_blocks;
    nonmoving_marked_large_objects = null_mut::<bdescr>();
    n_nonmoving_marked_large_blocks = 0;
}

unsafe fn nonmovingSweepCompactObjects() {
    let mut next = null_mut::<bdescr>();
    let mut bd = nonmoving_compact_objects;

    while !bd.is_null() {
        next = (*bd).link as *mut bdescr;

        compactFree((*((*bd).start as *mut StgCompactNFDataBlock)).owner as *mut StgCompactNFData);

        bd = next;
    }

    nonmoving_compact_objects = nonmoving_marked_compact_objects;
    n_nonmoving_compact_blocks = n_nonmoving_marked_compact_blocks;
    nonmoving_marked_compact_objects = null_mut::<bdescr>();
    n_nonmoving_marked_compact_blocks = 0;
}

unsafe fn is_alive(mut p: *mut StgClosure) -> bool {
    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return true;
    }

    if nonmovingClosureBeingSwept(p) {
        return nonmovingIsAlive(p);
    } else {
        return true;
    };
}

unsafe fn nonmovingSweepStableNameTable() {
    stableNameLock();

    let mut p = null_mut::<snEntry>();
    let mut __end_ptr: *mut snEntry = stable_name_table.offset(SNT_size as isize) as *mut snEntry;
    p = stable_name_table.offset(1);

    while p < __end_ptr {
        if (*p).addr < stable_name_table as P_ || (*p).addr >= __end_ptr as P_ {
            if !(*p).sn_obj.is_null() {
                if !is_alive((*p).sn_obj) {
                    (*p).sn_obj = null_mut::<StgClosure>();
                    freeSnEntry(p);
                } else if !(*p).addr.is_null() {
                    if !is_alive((*p).addr as *mut StgClosure) {
                        (*p).addr = null_mut::<StgWord>();
                    }
                }
            }
        }

        p = p.offset(1);
    }

    stableNameUnlock();
}
