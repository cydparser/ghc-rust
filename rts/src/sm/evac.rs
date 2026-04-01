use crate::check_unload::markObjectCode;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{
    LDV_SHIFT, LDV_STATE_CREATE, MAX_CHARLIKE, MAX_INTLIKE, MIN_INTLIKE,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::era;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::spin_lock::{ACQUIRE_SPIN_LOCK, RELEASE_SPIN_LOCK};
use crate::ffi::rts::storage::block::{
    BF_COMPACT, BF_EVACUATED, BF_LARGE, BF_MARKED, BF_NONMOVING, BF_PINNED, BLOCK_SIZE_W, Bdescr,
    bdescr, bdescr_, dbl_link_onto, dbl_link_remove,
};
use crate::ffi::rts::storage::closure_macros::{
    CHARLIKE_CLOSURE, GET_CLOSURE_TAG, INFO_PTR_TO_STRUCT, INTLIKE_CLOSURE, LOOKS_LIKE_CLOSURE_PTR,
    SET_INFO, SET_INFO_RELAXED, SET_INFO_RELEASE, STATIC_LINK, TAG_CLOSURE, THUNK_SELECTOR_sizeW,
    UNTAG_CLOSURE, ap_sizeW, ap_stack_sizeW, arr_words_sizeW, bco_sizeW, continuation_sizeW,
    doingLDVProfiling, get_itbl, mut_arr_ptrs_sizeW, overwritingClosure, pap_sizeW, sizeW_fromITBL,
    small_mut_arr_ptrs_sizeW, stack_sizeW, thunk_sizeW_fromITBL,
};
use crate::ffi::rts::storage::closure_types::THUNK_SELECTOR;
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgClosure_, StgContinuation, StgInd, StgMutArrPtrs,
    StgPAP, StgSelector, StgSmallMutArrPtrs, StgThunk,
};
use crate::ffi::rts::storage::gc::{generation, generations, initBdescr, memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::info_tables::StgSRTField;
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info, stg_IND_info, stg_TSO_info,
    stg_WHITEHOLE_info, stg_sel_0_upd_info,
};
use crate::ffi::stg::smp::{busy_wait_nop, cas, xchg};
use crate::ffi::stg::types::{
    StgChar, StgHalfWord, StgInt, StgPtr, StgVolatilePtr, StgWord, StgWord16, StgWord64,
};
use crate::prelude::*;
use crate::sm::cnf::objectGetCompact;
use crate::sm::compact::{is_marked, mark};
use crate::sm::gc::{deadlock_detect_gc, major_gc, unload_mark_needed, whitehole_gc_spin};
use crate::sm::gc_thread::gen_workspace;
use crate::sm::gc_utils::todo_block_full;
use crate::sm::gct_decl::gct;
use crate::sm::mark_stack::push_mark_stack;
use crate::sm::non_moving::{isNonmovingClosure, nonmovingGetSegment};
use crate::sm::non_moving_allocate::nonmovingAllocateGC;
use crate::sm::non_moving_mark::markQueuePushClosureGC;
use crate::sm::storage::{STATIC_BITS, move_STACK, prev_static_flag, static_flag};
use crate::trace::{DEBUG_RTS, trace_};

const MAX_THUNK_SELECTOR_DEPTH: i32 = 16;

unsafe fn alloc_in_nonmoving_heap(mut size: u32) -> StgPtr {
    (*gct).copied = (*gct).copied.wrapping_add(size as W_);

    let mut to = nonmovingAllocateGC((*gct).cap, size as StgWord) as StgPtr;
    let mut seg = nonmovingGetSegment(to);

    if (*seg).todo_link.is_null() {
        let mut ws: *mut gen_workspace = (&raw mut (*gct).gens as *mut gen_workspace)
            .offset((*oldest_gen).no as isize)
            as *mut gen_workspace;
        (*seg).todo_link = (*ws).0.todo_seg;
        (*ws).0.todo_seg = seg;

        let mut seg_bd = Bdescr(seg as StgPtr);
        (*seg_bd).u.scan = to;
    }

    if major_gc as i32 != 0 && !deadlock_detect_gc {
        markQueuePushClosureGC(
            &raw mut (*(*gct).cap).upd_rem_set.queue,
            to as *mut StgClosure,
        );
    }

    return to;
}

unsafe fn alloc_in_moving_heap(mut size: u32, mut gen_no: u32) -> StgPtr {
    let mut ws: *mut gen_workspace =
        (&raw mut (*gct).gens as *mut gen_workspace).offset(gen_no as isize) as *mut gen_workspace;

    let mut to = (*ws).0.todo_free;
    (*ws).0.todo_free = (*ws).0.todo_free.offset(size as isize);

    if (*ws).0.todo_free > (*ws).0.todo_lim {
        to = todo_block_full(size, ws);
    }

    if ((*ws).0.todo_free >= (*(*ws).0.todo_bd).c2rust_unnamed.free
        && (*ws).0.todo_free <= (*ws).0.todo_lim) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 116);
    }

    return to;
}

unsafe fn alloc_for_copy_nonmoving(mut size: u32, mut gen_no: u32) -> StgPtr {
    if deadlock_detect_gc {
        return alloc_in_nonmoving_heap(size);
    }

    if gen_no < (*gct).evac_gen_no {
        if (*gct).eager_promotion {
            gen_no = (*gct).evac_gen_no;
        } else {
            (*gct).failed_to_evac = true;
        }
    }

    if gen_no == (*oldest_gen).no {
        return alloc_in_nonmoving_heap(size);
    } else {
        return alloc_in_moving_heap(size, gen_no);
    };
}

unsafe fn alloc_for_copy(mut size: u32, mut gen_no: u32) -> StgPtr {
    if (gen_no < RtsFlags.GcFlags.generations) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 154);
    }

    if RtsFlags.GcFlags.useNonmoving as i64 != 0 {
        return alloc_for_copy_nonmoving(size, gen_no);
    }

    if gen_no < (*gct).evac_gen_no {
        if (*gct).eager_promotion {
            gen_no = (*gct).evac_gen_no;
        } else {
            (*gct).failed_to_evac = true;
        }
    }

    return alloc_in_moving_heap(size, gen_no);
}

#[inline(always)]
unsafe fn copy_tag(
    mut p: *mut *mut StgClosure,
    mut info: *const StgInfoTable,
    mut src: *mut StgClosure,
    mut size: u32,
    mut gen_no: u32,
    mut tag: StgWord,
) {
    let mut to = null_mut::<StgWord>();
    let mut from = null_mut::<StgWord>();
    let mut i: u32 = 0;
    to = alloc_for_copy(size, gen_no);
    from = src as StgPtr;
    *to.offset(0) = info as W_ as StgWord;
    i = 1;

    while i < size {
        *to.offset(i as isize) = *from.offset(i as isize);
        i = i.wrapping_add(1);
    }

    (*src).header.info = (to as StgWord | 1) as *const StgInfoTable;
    *p = TAG_CLOSURE(tag, to as *mut StgClosure);

    if doingLDVProfiling() {
        (*(from as *mut StgClosure)).header.prof.hp.ldvw = size as StgWord;
    }
}

#[inline(always)]
unsafe fn copyPart(
    mut p: *mut *mut StgClosure,
    mut src: *mut StgClosure,
    mut size_to_reserve: u32,
    mut size_to_copy: u32,
    mut gen_no: u32,
) -> bool {
    let mut to = null_mut::<StgWord>();
    let mut from = null_mut::<StgWord>();
    let mut i: u32 = 0;
    let mut info: StgWord = 0;
    info = (*src).header.info as W_ as StgWord;
    to = alloc_for_copy(size_to_reserve, gen_no);
    from = src as StgPtr;
    *to.offset(0) = info;
    i = 1;

    while i < size_to_copy {
        *to.offset(i as isize) = *from.offset(i as isize);
        i = i.wrapping_add(1);
    }

    (p).store(to as *mut StgClosure, Ordering::Release);
    (&raw mut (*src).header.info).store(
        (to as StgWord | 1) as *const StgInfoTable,
        Ordering::Release,
    );

    if doingLDVProfiling() {
        (*(from as *mut StgClosure)).header.prof.hp.ldvw = size_to_reserve as StgWord;
    }

    if size_to_reserve.wrapping_sub(size_to_copy) > 0 {
        if era > 0 {
            let mut i_0: i32 = 0;
            i_0 = 0;

            while i_0 < size_to_reserve.wrapping_sub(size_to_copy) as i32 {
                *to.offset(size_to_copy as isize).offset(i_0 as isize) = 0;
                i_0 += 1;
            }
        }
    }

    return true;
}

#[inline(always)]
unsafe fn copy(
    mut p: *mut *mut StgClosure,
    mut info: *const StgInfoTable,
    mut src: *mut StgClosure,
    mut size: u32,
    mut gen_no: u32,
) {
    copy_tag(p, info, src, size, gen_no, 0);
}

#[inline(never)]
unsafe fn evacuate_large(mut p: StgPtr) {
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();
    let mut new_gen = null_mut::<generation>();
    let mut gen_no: u32 = 0;
    let mut new_gen_no: u32 = 0;
    let mut ws = null_mut::<gen_workspace>();
    bd = Bdescr(p);
    r#gen = (&raw mut (*bd).r#gen).load(Ordering::Relaxed) as *mut generation;
    gen_no = (&raw mut (*bd).gen_no).load(Ordering::Relaxed) as u32;
    ACQUIRE_SPIN_LOCK(&raw mut (*r#gen).sync);

    if (&raw mut (*bd).flags).load(Ordering::Relaxed) as i32 & BF_EVACUATED != 0 {
        if gen_no < (*gct).evac_gen_no {
            (*gct).failed_to_evac = true;
        }

        RELEASE_SPIN_LOCK(&raw mut (*r#gen).sync);
        return;
    }

    dbl_link_remove(bd, &raw mut (*r#gen).large_objects);
    new_gen_no = (*bd).dest_no as u32;

    if deadlock_detect_gc as i64 != 0 {
        new_gen_no = (*oldest_gen).no;
    } else if new_gen_no < (*gct).evac_gen_no {
        if (*gct).eager_promotion {
            new_gen_no = (*gct).evac_gen_no;
        } else {
            (*gct).failed_to_evac = true;
        }
    }

    ws = (&raw mut (*gct).gens as *mut gen_workspace).offset(new_gen_no as isize)
        as *mut gen_workspace;
    new_gen = generations.offset(new_gen_no as isize) as *mut generation;
    (&raw mut (*bd).flags).or(BF_EVACUATED as StgWord16, Ordering::AcqRel);

    if (RtsFlags.GcFlags.useNonmoving as i32 != 0 && new_gen == oldest_gen) as i32 as i64 != 0 {
        (&raw mut (*bd).flags).or(BF_NONMOVING as StgWord16, Ordering::AcqRel);

        if major_gc as i32 != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*gct).cap).upd_rem_set.queue,
                p as *mut StgClosure,
            );
        }
    }

    initBdescr(bd, new_gen, (*new_gen).to as *mut generation);

    if (&raw mut (*bd).flags).load(Ordering::Relaxed) as i32 & BF_PINNED != 0 {
        if ((*get_itbl(p as *mut StgClosure)).r#type == 42) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Evac.c".as_ptr(), 462);
        }

        if new_gen != r#gen {
            ACQUIRE_SPIN_LOCK(&raw mut (*new_gen).sync);
        }

        dbl_link_onto(bd, &raw mut (*new_gen).scavenged_large_objects);
        (*new_gen).n_scavenged_large_blocks = (*new_gen)
            .n_scavenged_large_blocks
            .wrapping_add((*bd).blocks as memcount);

        if new_gen != r#gen {
            RELEASE_SPIN_LOCK(&raw mut (*new_gen).sync);
        }
    } else {
        (*bd).link = (*ws).0.todo_large_objects as *mut bdescr_;
        (*ws).0.todo_large_objects = bd;
    }

    RELEASE_SPIN_LOCK(&raw mut (*r#gen).sync);
}

unsafe fn evacuate_static_object(mut link_field: *mut *mut StgClosure, mut q: *mut StgClosure) {
    if RtsFlags.GcFlags.useNonmoving as i64 != 0 {
        if major_gc as i32 != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, q);
        }

        return;
    }

    let mut link: StgWord = (link_field as *mut StgWord).load(Ordering::Relaxed);

    if link & STATIC_BITS as StgWord | prev_static_flag as StgWord != 3 {
        let mut new_list_head: StgWord = q as StgWord | static_flag as StgWord;
        let mut prev: StgWord = 0;
        prev = cas(
            link_field as StgVolatilePtr,
            link,
            (*gct).static_objects as StgWord,
        );

        if prev == link {
            (*gct).static_objects = new_list_head as *mut StgClosure;
        }
    }
}

unsafe fn evacuate_compact(mut p: StgPtr) {
    let mut str = null_mut::<StgCompactNFData>();
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();
    let mut new_gen = null_mut::<generation>();
    let mut gen_no: u32 = 0;
    let mut new_gen_no: u32 = 0;
    str = objectGetCompact(p as *mut StgClosure);

    if ((*get_itbl(str as *mut StgClosure)).r#type == 63) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 537);
    }

    bd = Bdescr(str as StgPtr);
    gen_no = (*bd).gen_no as u32;

    if (&raw mut (*bd).flags).load(Ordering::Relaxed) as i32 & BF_NONMOVING != 0 {
        if major_gc as i32 != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*gct).cap).upd_rem_set.queue,
                str as *mut StgClosure,
            );
        }

        return;
    }

    if (*bd).flags as i32 & BF_EVACUATED != 0 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.compact as i64 != 0 {
            trace_(c"Compact %p already evacuated".as_ptr(), str);
        }

        if gen_no < (*gct).evac_gen_no {
            (*gct).failed_to_evac = true;
        }

        return;
    }

    r#gen = (*bd).r#gen as *mut generation;
    gen_no = (*bd).gen_no as u32;
    ACQUIRE_SPIN_LOCK(&raw mut (*r#gen).sync);

    if (*bd).flags as i32 & BF_EVACUATED != 0 {
        if gen_no < (*gct).evac_gen_no {
            (*gct).failed_to_evac = true;
        }

        RELEASE_SPIN_LOCK(&raw mut (*r#gen).sync);
        return;
    }

    dbl_link_remove(bd, &raw mut (*r#gen).compact_objects);
    new_gen_no = (*bd).dest_no as u32;

    if new_gen_no < (*gct).evac_gen_no {
        if (*gct).eager_promotion {
            new_gen_no = (*gct).evac_gen_no;
        } else {
            (*gct).failed_to_evac = true;
        }
    }

    new_gen = generations.offset(new_gen_no as isize) as *mut generation;
    (*bd).flags = ((*bd).flags as i32 | BF_EVACUATED) as StgWord16;

    if (RtsFlags.GcFlags.useNonmoving as i32 != 0 && new_gen == oldest_gen) as i32 as i64 != 0 {
        (&raw mut (*bd).flags).or(BF_NONMOVING as StgWord16, Ordering::Relaxed);

        if major_gc as i32 != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*gct).cap).upd_rem_set.queue,
                str as *mut StgClosure,
            );
        }
    }

    initBdescr(bd, new_gen, (*new_gen).to as *mut generation);

    if !(*str).hash.is_null() {
        let mut ws: *mut gen_workspace = (&raw mut (*gct).gens as *mut gen_workspace)
            .offset(new_gen_no as isize)
            as *mut gen_workspace;
        (*bd).link = (*ws).0.todo_large_objects as *mut bdescr_;
        (*ws).0.todo_large_objects = bd;
    } else {
        if new_gen != r#gen {
            ACQUIRE_SPIN_LOCK(&raw mut (*new_gen).sync);
        }

        dbl_link_onto(bd, &raw mut (*new_gen).live_compact_objects);
        (*new_gen).n_live_compact_blocks = ((*new_gen).n_live_compact_blocks as StgWord)
            .wrapping_add((*str).totalW.wrapping_div(BLOCK_SIZE_W as StgWord))
            as memcount as memcount;

        if new_gen != r#gen {
            RELEASE_SPIN_LOCK(&raw mut (*new_gen).sync);
        }
    }

    RELEASE_SPIN_LOCK(&raw mut (*r#gen).sync);
}

unsafe fn evacuate1(mut p: *mut *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    let mut gen_no: u32 = 0;
    let mut q = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();
    let mut tag: StgWord = 0;
    q = (p).load(Ordering::Relaxed);

    loop {
        tag = GET_CLOSURE_TAG(q);
        q = UNTAG_CLOSURE(q);

        if LOOKS_LIKE_CLOSURE_PTR(q as *const c_void) as i32 as i64 != 0 {
        } else {
            barf(c"invalid closure, info=%p".as_ptr(), (*q).header.info);
        }

        if !(q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end) {
            if !major_gc {
                return;
            }

            if unload_mark_needed as i64 != 0 {
                markObjectCode(q as *const c_void);
            }

            info = get_itbl(q);

            match (*info).r#type {
                21 => {
                    if (*info).srt != 0 {
                        evacuate_static_object(
                            (&raw mut (*q).payload as *mut *mut StgClosure_).offset(1)
                                as *mut *mut StgClosure,
                            q,
                        );
                    }

                    return;
                }
                14 => {
                    if (*info).srt != 0 || (*info).layout.payload.ptrs != 0 {
                        evacuate_static_object(STATIC_LINK(info, q), q);
                    }

                    return;
                }
                28 => {
                    evacuate_static_object(
                        (&raw mut (*q).payload as *mut *mut StgClosure_).offset(1)
                            as *mut *mut StgClosure,
                        q,
                    );

                    return;
                }
                1 | 2 | 4 | 5 => {
                    evacuate_static_object(STATIC_LINK(info, q), q);
                    return;
                }
                3 | 6 | 7 => return,
                _ => {
                    barf(
                        c"evacuate(static): strange closure type %d".as_ptr(),
                        (*info).r#type as i32,
                    );
                }
            }
        }

        bd = Bdescr(q as StgPtr);

        let mut flags: u16 = (&raw mut (*bd).flags).load(Ordering::Relaxed);

        if flags as i32 & (BF_LARGE | BF_MARKED | BF_EVACUATED | BF_COMPACT | BF_NONMOVING) != 0 {
            if ((&raw mut (*bd).flags).load(Ordering::Relaxed) as i32 & 1024) as i64 != 0 {
                if major_gc as i32 != 0 && !deadlock_detect_gc {
                    markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, q);
                }

                return;
            }

            if flags as i32 & BF_EVACUATED != 0 {
                if ((&raw mut (*bd).gen_no).load(Ordering::Relaxed) as u32) < (*gct).evac_gen_no {
                    (*gct).failed_to_evac = true;
                }

                return;
            }

            if flags as i32 & BF_COMPACT != 0 {
                evacuate_compact(q as StgPtr);
                return;
            }

            if flags as i32 & BF_LARGE != 0 {
                evacuate_large(q as StgPtr);
                return;
            }

            if is_marked(q as StgPtr, bd) == 0 {
                mark(q as StgPtr, bd);
                push_mark_stack(q as StgPtr);
            }

            return;
        }

        gen_no = (*bd).dest_no as u32;
        info = (&raw mut (*q).header.info).load(Ordering::Acquire);

        if info as StgWord & 1 != 0 {
            let mut e = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
            (p).store(TAG_CLOSURE(tag, e), Ordering::Relaxed);

            if gen_no < (*gct).evac_gen_no {
                if ((&raw mut (*(Bdescr as unsafe extern "C" fn(StgPtr) -> *mut bdescr)(
                    e as StgPtr,
                ))
                .gen_no)
                    .load(Ordering::Acquire) as u32)
                    < (*gct).evac_gen_no
                {
                    (*gct).failed_to_evac = true;
                }
            }

            return;
        }

        match (*INFO_PTR_TO_STRUCT(info)).r#type {
            58 => {}
            3 => {
                let mut w: StgWord =
                    *(&raw mut (*q).payload as *mut *mut StgClosure_).offset(0) as StgWord;

                if info == (*ghc_hs_iface).Czh_con_info && w as StgChar <= MAX_CHARLIKE as StgChar {
                    (p).store(
                        TAG_CLOSURE(
                            tag,
                            CHARLIKE_CLOSURE(w as StgChar as i32) as *mut StgClosure,
                        ),
                        Ordering::Relaxed,
                    );
                } else if info == (*ghc_hs_iface).Izh_con_info
                    && w as StgInt >= MIN_INTLIKE as StgInt
                    && w as StgInt <= MAX_INTLIKE as StgInt
                {
                    (p).store(
                        TAG_CLOSURE(tag, INTLIKE_CLOSURE(w as StgInt as i32) as *mut StgClosure),
                        Ordering::Relaxed,
                    );
                } else {
                    copy_tag(
                        p,
                        info,
                        q,
                        (size_of::<StgHeader>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            .wrapping_add(1 as usize) as u32,
                        gen_no,
                        tag,
                    );
                }

                return;
            }
            10 | 9 | 2 => {
                copy_tag(
                    p,
                    info,
                    q,
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as u32,
                    gen_no,
                    tag,
                );

                return;
            }
            16 | 17 => {
                copy(
                    p,
                    info,
                    q,
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as u32,
                    gen_no,
                );

                return;
            }
            19 | 18 | 20 => {
                copy(
                    p,
                    info,
                    q,
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as u32,
                    gen_no,
                );

                return;
            }
            12 | 11 | 13 | 5 | 4 => {
                copy_tag(
                    p,
                    info,
                    q,
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as u32,
                    gen_no,
                    tag,
                );

                return;
            }
            6 => {
                copy_tag(
                    p,
                    info,
                    q,
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as u32,
                    gen_no,
                    tag,
                );

                return;
            }
            15 => {
                copy(
                    p,
                    info,
                    q,
                    thunk_sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as u32,
                    gen_no,
                );

                return;
            }
            8 | 1 | 7 => {
                copy_tag(
                    p,
                    info,
                    q,
                    sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as u32,
                    gen_no,
                    tag,
                );

                return;
            }
            38 => {
                let mut r = null_mut::<StgClosure>();
                let mut i = null::<StgInfoTable>();
                r = (*(q as *mut StgInd)).indirectee;

                if GET_CLOSURE_TAG(r) == 0 {
                    i = (&raw mut (*r).header.info).load(Ordering::Acquire);

                    if i as StgWord & 1 != 0 {
                        r = (i as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
                        i = (&raw mut (*r).header.info).load(Ordering::Acquire);
                    }

                    if i == &raw const stg_TSO_info
                        || i == &raw const stg_WHITEHOLE_info
                        || i == &raw const stg_BLOCKING_QUEUE_CLEAN_info
                        || i == &raw const stg_BLOCKING_QUEUE_DIRTY_info
                    {
                        copy(
                            p,
                            info,
                            q,
                            (size_of::<StgInd>() as usize)
                                .wrapping_add(size_of::<W_>() as usize)
                                .wrapping_sub(1 as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as u32,
                            gen_no,
                        );

                        return;
                    }

                    if (i != &raw const stg_IND_info) as i32 as i64 != 0 {
                    } else {
                        _assertFail(c"rts/sm/Evac.c".as_ptr(), 952);
                    }
                }

                q = r;
                (p).store(r, Ordering::Release);
            }
            47 | 48 | 39 | 40 | 41 | 37 | 49 | 50 | 51 => {
                copy(
                    p,
                    info,
                    q,
                    sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as u32,
                    gen_no,
                );

                return;
            }
            23 => {
                copy(p, info, q, bco_sizeW(q as *mut StgBCO) as u32, gen_no);
                return;
            }
            22 => {
                eval_thunk_selector(p, q as *mut StgSelector, true);
                return;
            }
            27 => {
                q = (&raw mut (*(q as *mut StgInd)).indirectee).load(Ordering::Relaxed);
                (p).store(q, Ordering::Relaxed);
            }
            29 | 30 | 31 | 33 | 35 | 36 | 34 | 57 | 56 | 55 | 65 => {
                barf(c"evacuate: stack frame at %p\n".as_ptr(), q);
            }
            25 => {
                copy(p, info, q, pap_sizeW(q as *mut StgPAP) as u32, gen_no);
                return;
            }
            24 => {
                copy(p, info, q, ap_sizeW(q as *mut StgAP) as u32, gen_no);
                return;
            }
            26 => {
                copy(
                    p,
                    info,
                    q,
                    ap_stack_sizeW(q as *mut StgAP_STACK) as u32,
                    gen_no,
                );
                return;
            }
            42 => {
                copy(
                    p,
                    info,
                    q,
                    arr_words_sizeW(q as *mut StgArrBytes) as u32,
                    gen_no,
                );
                return;
            }
            43 | 44 | 46 | 45 => {
                copy(
                    p,
                    info,
                    q,
                    mut_arr_ptrs_sizeW(q as *mut StgMutArrPtrs) as u32,
                    gen_no,
                );

                return;
            }
            59 | 60 | 62 | 61 => {
                copy(
                    p,
                    info,
                    q,
                    small_mut_arr_ptrs_sizeW(q as *mut StgSmallMutArrPtrs) as u32,
                    gen_no,
                );

                return;
            }
            52 => {
                copy(
                    p,
                    info,
                    q,
                    (size_of::<StgTSO>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as u32,
                    gen_no,
                );

                return;
            }
            53 => {
                let mut stack = q as *mut StgStack;
                let mut new_stack = null_mut::<StgStack>();
                let mut r_0 = null_mut::<StgWord>();
                let mut s = null_mut::<StgWord>();
                let mut mine: bool = false;

                mine = copyPart(
                    p,
                    stack as *mut StgClosure,
                    stack_sizeW(stack) as u32,
                    (size_of::<StgStack>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as u32,
                    gen_no,
                );

                if mine {
                    new_stack = *p as *mut StgStack;
                    move_STACK(stack, new_stack);
                    r_0 = (*stack).sp;
                    s = (*new_stack).sp;

                    while r_0
                        < (&raw mut (*stack).stack as *mut StgWord)
                            .offset((*stack).stack_size as isize)
                    {
                        let fresh5 = r_0;
                        r_0 = r_0.offset(1);

                        let fresh6 = s;
                        s = s.offset(1);
                        *fresh6 = *fresh5;
                    }
                }

                return;
            }
            54 => {
                copy(
                    p,
                    info,
                    q,
                    (size_of::<StgTRecChunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as u32,
                    gen_no,
                );

                return;
            }
            64 => {
                copy(
                    p,
                    info,
                    q,
                    continuation_sizeW(q as *mut StgContinuation) as u32,
                    gen_no,
                );

                return;
            }
            _ => {
                barf(
                    c"evacuate: strange closure type %d".as_ptr(),
                    (*INFO_PTR_TO_STRUCT(info)).r#type as i32,
                );
            }
        }
    }
}

unsafe fn evacuate_BLACKHOLE1(mut p: *mut *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    let mut gen_no: u32 = 0;
    let mut q = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();
    q = *p;

    if (q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end) as i32
        as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 1097);
    }

    if (GET_CLOSURE_TAG(q) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 1098);
    }

    bd = Bdescr(q as StgPtr);

    let flags: u16 = (&raw mut (*bd).flags).load(Ordering::Relaxed);

    if (flags as i32 & 512 == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 1104);
    }

    if ((&raw mut (*bd).flags).load(Ordering::Relaxed) as i32 & 1024) as i64 != 0 {
        if major_gc as i32 != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, q);
        }

        return;
    }

    if flags as i32 & BF_LARGE != 0 {
        evacuate_large(q as StgPtr);
        return;
    }

    if flags as i32 & BF_EVACUATED != 0 {
        if ((*bd).gen_no as u32) < (*gct).evac_gen_no {
            (*gct).failed_to_evac = true;
        }

        return;
    }

    if flags as i32 & BF_MARKED != 0 {
        if is_marked(q as StgPtr, bd) == 0 {
            mark(q as StgPtr, bd);
            push_mark_stack(q as StgPtr);
        }

        return;
    }

    gen_no = (*bd).dest_no as u32;
    info = (&raw mut (*q).header.info).load(Ordering::Acquire);

    if info as StgWord & 1 != 0 {
        let mut e = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
        *p = e;

        if gen_no < (*gct).evac_gen_no {
            if ((&raw mut (*(Bdescr as unsafe extern "C" fn(StgPtr) -> *mut bdescr)(e as StgPtr))
                .gen_no)
                .load(Ordering::Acquire) as u32)
                < (*gct).evac_gen_no
            {
                (*gct).failed_to_evac = true;
            }
        }

        return;
    }

    if ((*INFO_PTR_TO_STRUCT(info)).r#type == 38) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Evac.c".as_ptr(), 1150);
    }

    copy(
        p,
        info,
        q,
        (size_of::<StgInd>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as u32,
        gen_no,
    );
}

unsafe fn unchain_thunk_selectors(mut p: *mut StgSelector, mut val: *mut StgClosure) {
    while !p.is_null() {
        if ((*p).header.info == &raw const stg_WHITEHOLE_info) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Evac.c".as_ptr(), 1184);
        }

        let mut prev = *(&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
            .offset(0) as *mut StgSelector;

        if p as *mut StgClosure == val {
            ((&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                as *mut *mut StgClosure_)
                .store(val, Ordering::Relaxed);
            SET_INFO_RELEASE(p as *mut StgClosure, &raw const stg_sel_0_upd_info);
        } else {
            (&raw mut (*(p as *mut StgInd)).indirectee).store(val, Ordering::Relaxed);
            SET_INFO_RELEASE(p as *mut StgClosure, &raw const stg_IND_info);
        }

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(p as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        }

        p = prev;
    }
}

unsafe fn eval_thunk_selector(q: *mut *mut StgClosure, mut p: *mut StgSelector, mut evac: bool) {
    let mut val: *mut StgClosure = null_mut::<StgClosure>();
    let mut current_block: u64;
    let mut field: u32 = 0;
    let mut info = null_mut::<StgInfoTable>();
    let mut info_ptr: StgWord = 0;
    let mut selectee = null_mut::<StgClosure>();
    let mut prev_thunk_selector = null_mut::<StgSelector>();
    let mut bd = null_mut::<bdescr>();
    prev_thunk_selector = null_mut::<StgSelector>();

    '_selector_chain: loop {
        bd = Bdescr(p as StgPtr);

        if p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end {
            let flags: u16 = (&raw mut (*bd).flags).load(Ordering::Relaxed);

            if (flags as i32 & 2 == 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Evac.c".as_ptr(), 1264);
            }

            if flags as i32 & (BF_EVACUATED | BF_NONMOVING) != 0 {
                unchain_thunk_selectors(prev_thunk_selector, p as *mut StgClosure);

                if flags as i32 & BF_NONMOVING != 0 {
                    markQueuePushClosureGC(
                        &raw mut (*(*gct).cap).upd_rem_set.queue,
                        p as *mut StgClosure,
                    );
                }

                *q = p as *mut StgClosure;

                if evac as i32 != 0 && ((*bd).gen_no as u32) < (*gct).evac_gen_no {
                    (*gct).failed_to_evac = true;
                }

                return;
            }

            if flags as i32 & BF_MARKED != 0 {
                *q = p as *mut StgClosure;

                if evac {
                    evacuate1(q);
                }

                unchain_thunk_selectors(prev_thunk_selector, p as *mut StgClosure);
                return;
            }
        }

        loop {
            info_ptr = xchg(
                &raw mut (*p).header.info as StgPtr,
                &raw const stg_WHITEHOLE_info as StgWord,
            );

            if info_ptr != &raw const stg_WHITEHOLE_info as W_ {
                break;
            }

            write_volatile(
                &mut whitehole_gc_spin as *mut StgWord64,
                read_volatile::<StgWord64>(&whitehole_gc_spin as *const StgWord64).wrapping_add(1),
            );

            busy_wait_nop();
        }

        if info_ptr & 1 != 0
            || (*INFO_PTR_TO_STRUCT(info_ptr as *mut StgInfoTable)).r#type
                != THUNK_SELECTOR as StgHalfWord
        {
            SET_INFO(p as *mut StgClosure, info_ptr as *const StgInfoTable);
            (q).store(p as *mut StgClosure, Ordering::Release);

            if (*Bdescr(p as StgPtr)).flags as i32 & BF_NONMOVING != 0 {
                markQueuePushClosureGC(
                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                    p as *mut StgClosure,
                );
            }

            if evac {
                evacuate1(q);
            }

            unchain_thunk_selectors(prev_thunk_selector, p as *mut StgClosure);
            return;
        }

        field = (*INFO_PTR_TO_STRUCT(info_ptr as *mut StgInfoTable))
            .layout
            .selector_offset as u32;
        selectee = UNTAG_CLOSURE((*p).selectee);

        loop {
            info = (&raw mut (*selectee).header.info as *mut *mut StgInfoTable)
                .load(Ordering::Relaxed);

            if info as StgWord & 1 != 0 {
                current_block = 11707196495568705917;
                break '_selector_chain;
            } else {
                info = INFO_PTR_TO_STRUCT(info);

                match (*info).r#type {
                    1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                        if (field
                            < (*info)
                                .layout
                                .payload
                                .ptrs
                                .wrapping_add((*info).layout.payload.nptrs))
                            as i32 as i64
                            != 0
                        {
                        } else {
                            _assertFail(c"rts/sm/Evac.c".as_ptr(), 1388);
                        }

                        val = ((&raw mut (*selectee).payload as *mut *mut StgClosure_)
                            .offset(field as isize)
                            as *mut *mut StgClosure_)
                            .load(Ordering::Relaxed);

                        if era > 0 {
                            SET_INFO(p as *mut StgClosure, info_ptr as *mut StgInfoTable);

                            overwritingClosure(p as *mut StgClosure);

                            SET_INFO_RELEASE(p as *mut StgClosure, &raw const stg_WHITEHOLE_info);
                        }

                        break;
                    }
                    27 | 28 => {
                        selectee = UNTAG_CLOSURE(
                            (&raw mut (*(selectee as *mut StgInd)).indirectee)
                                .load(Ordering::Relaxed),
                        );
                    }
                    38 => {
                        let mut r = null_mut::<StgClosure>();
                        let mut i = null::<StgInfoTable>();
                        r = (&raw mut (*(selectee as *mut StgInd)).indirectee)
                            .load(Ordering::Acquire);

                        if GET_CLOSURE_TAG(r) == 0 {
                            i = (&raw mut (*r).header.info).load(Ordering::Acquire);

                            if i as StgWord & 1 != 0 {
                                r = (i as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
                                i = (&raw mut (*r).header.info).load(Ordering::Relaxed);
                            }

                            if i == &raw const stg_TSO_info
                                || i == &raw const stg_WHITEHOLE_info
                                || i == &raw const stg_BLOCKING_QUEUE_CLEAN_info
                                || i == &raw const stg_BLOCKING_QUEUE_DIRTY_info
                            {
                                current_block = 11707196495568705917;
                                break '_selector_chain;
                            }

                            if (i != &raw const stg_IND_info) as i32 as i64 != 0 {
                            } else {
                                _assertFail(c"rts/sm/Evac.c".as_ptr(), 1494);
                            }
                        }

                        selectee = UNTAG_CLOSURE(
                            (&raw mut (*(selectee as *mut StgInd)).indirectee)
                                .load(Ordering::Relaxed),
                        );
                    }
                    22 => {
                        let mut val_0 = null_mut::<StgClosure>();

                        if (*gct).thunk_selector_depth >= MAX_THUNK_SELECTOR_DEPTH as W_ {
                            if isNonmovingClosure(p as *mut StgClosure) {
                                markQueuePushClosureGC(
                                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                                    p as *mut StgClosure,
                                );
                            }

                            current_block = 11707196495568705917;
                            break '_selector_chain;
                        } else {
                            (*gct).thunk_selector_depth =
                                (*gct).thunk_selector_depth.wrapping_add(1);

                            eval_thunk_selector(
                                &raw mut val_0,
                                selectee as *mut StgSelector,
                                false,
                            );

                            (*gct).thunk_selector_depth =
                                (*gct).thunk_selector_depth.wrapping_sub(1);

                            if val_0 == selectee {
                                current_block = 11707196495568705917;
                                break '_selector_chain;
                            }

                            selectee = UNTAG_CLOSURE(val_0);
                        }
                    }
                    58 | 24 | 26 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => {
                        current_block = 11707196495568705917;
                        break '_selector_chain;
                    }
                    _ => {
                        barf(
                            c"eval_thunk_selector: strange selectee %d".as_ptr(),
                            (*info).r#type as i32,
                        );
                    }
                }
            }
        }

        loop {
            info_ptr = (&raw mut (*(UNTAG_CLOSURE
                as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
                val
            ))
            .header
            .info as *mut StgWord)
                .load(Ordering::Acquire);

            if info_ptr & 1 != 0 {
                current_block = 9512719473022792396;
                break '_selector_chain;
            }

            info = INFO_PTR_TO_STRUCT(info_ptr as *mut StgInfoTable);

            match (*info).r#type {
                27 | 28 => {}
                22 => {
                    break;
                }
                _ => {
                    current_block = 9512719473022792396;
                    break '_selector_chain;
                }
            }

            val = (&raw mut (*(val as *mut StgInd)).indirectee).load(Ordering::Relaxed);
        }

        ((&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
            as *mut *mut StgClosure_)
            .store(prev_thunk_selector as *mut StgClosure, Ordering::Relaxed);
        prev_thunk_selector = p;
        p = val as *mut StgSelector;
    }

    match current_block {
        11707196495568705917 => {
            SET_INFO_RELAXED(p as *mut StgClosure, info_ptr as *const StgInfoTable);
            *q = p as *mut StgClosure;

            if evac {
                copy(
                    q,
                    info_ptr as *const StgInfoTable,
                    p as *mut StgClosure,
                    THUNK_SELECTOR_sizeW() as u32,
                    (*bd).dest_no as u32,
                );
            }

            if isNonmovingClosure(*q) {
                markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, *q);
            }

            unchain_thunk_selectors(prev_thunk_selector, *q);
            return;
        }
        _ => {
            ((&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                as *mut *mut StgClosure_)
                .store(prev_thunk_selector as *mut StgClosure, Ordering::Relaxed);
            prev_thunk_selector = p;
            *q = val;
            unchain_thunk_selectors(prev_thunk_selector, val);

            if evac {
                evacuate1(q);
            } else if isNonmovingClosure(*q) {
                markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, *q);
            }

            return;
        }
    };
}
