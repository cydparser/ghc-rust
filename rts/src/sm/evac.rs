use crate::check_unload::markObjectCode;
use crate::ffi::rts::constants::{MAX_CHARLIKE, MAX_INTLIKE, MIN_INTLIKE};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::block::{
    BF_COMPACT, BF_EVACUATED, BF_LARGE, BF_MARKED, BF_NONMOVING, BF_PINNED, BLOCK_SIZE_W, Bdescr,
    bdescr_, dbl_link_onto, dbl_link_remove,
};
use crate::ffi::rts::storage::closure_macros::{
    CHARLIKE_CLOSURE, GET_CLOSURE_TAG, INFO_PTR_TO_STRUCT, INTLIKE_CLOSURE, SET_INFO,
    SET_INFO_RELAXED, SET_INFO_RELEASE, STATIC_LINK, TAG_CLOSURE, THUNK_SELECTOR_sizeW,
    UNTAG_CLOSURE, ap_sizeW, ap_stack_sizeW, arr_words_sizeW, bco_sizeW, continuation_sizeW,
    get_itbl, mut_arr_ptrs_sizeW, pap_sizeW, sizeW_fromITBL, small_mut_arr_ptrs_sizeW, stack_sizeW,
    thunk_sizeW_fromITBL,
};
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
use crate::ffi::stg::types::{StgChar, StgHalfWord, StgInt, StgPtr, StgWord, StgWord16};
use crate::prelude::*;
use crate::sm::cnf::objectGetCompact;
use crate::sm::compact::{is_marked, mark};
use crate::sm::gc::{deadlock_detect_gc, major_gc, unload_mark_needed};
use crate::sm::gc_thread::{gc_thread, gen_workspace};
use crate::sm::gc_utils::todo_block_full;
use crate::sm::gct_decl::the_gc_thread;
use crate::sm::mark_stack::push_mark_stack;
use crate::sm::non_moving::{isNonmovingClosure, nonmovingGetSegment};
use crate::sm::non_moving_allocate::nonmovingAllocateGC;
use crate::sm::non_moving_mark::markQueuePushClosureGC;
use crate::sm::storage::{STATIC_BITS, move_STACK, prev_static_flag, static_flag};
use crate::trace::{DEBUG_RTS, trace_};

const MAX_THUNK_SELECTOR_DEPTH: c_int = 16 as c_int;

unsafe fn alloc_in_nonmoving_heap(mut size: uint32_t) -> StgPtr {
    let ref mut fresh7 = (*(&raw mut the_gc_thread as *mut gc_thread)).copied;
    *fresh7 = (*fresh7).wrapping_add(size as W_);

    let mut to = nonmovingAllocateGC(
        (*(&raw mut the_gc_thread as *mut gc_thread)).cap,
        size as StgWord,
    ) as StgPtr;

    let mut seg = nonmovingGetSegment(to);

    if (*seg).todo_link.is_null() {
        let mut ws: *mut gen_workspace =
            (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
                .offset((*oldest_gen).no as isize) as *mut gen_workspace;
        (*seg).todo_link = (*ws).0.todo_seg;
        (*ws).0.todo_seg = seg;

        let mut seg_bd = Bdescr(seg as StgPtr);
        (*seg_bd).u.scan = to;
    }

    if major_gc as c_int != 0 && !deadlock_detect_gc {
        markQueuePushClosureGC(
            &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                .upd_rem_set
                .queue,
            to as *mut StgClosure,
        );
    }

    return to;
}

#[inline]
unsafe fn alloc_in_moving_heap(mut size: uint32_t, mut gen_no: uint32_t) -> StgPtr {
    let mut ws: *mut gen_workspace = (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens
        as *mut gen_workspace)
        .offset(gen_no as isize) as *mut gen_workspace;

    let mut to = (*ws).0.todo_free;
    (*ws).0.todo_free = (*ws).0.todo_free.offset(size as isize);

    if (*ws).0.todo_free > (*ws).0.todo_lim {
        to = todo_block_full(size, ws);
    }

    return to;
}

unsafe fn alloc_for_copy_nonmoving(mut size: uint32_t, mut gen_no: uint32_t) -> StgPtr {
    if deadlock_detect_gc {
        return alloc_in_nonmoving_heap(size);
    }

    if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
        if (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion {
            gen_no = (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no;
        } else {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }
    }

    if gen_no == (*oldest_gen).no {
        return alloc_in_nonmoving_heap(size);
    } else {
        return alloc_in_moving_heap(size, gen_no);
    };
}

#[inline]
unsafe fn alloc_for_copy(mut size: uint32_t, mut gen_no: uint32_t) -> StgPtr {
    if RtsFlags.GcFlags.useNonmoving as c_long != 0 {
        return alloc_for_copy_nonmoving(size, gen_no);
    }

    if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
        if (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion {
            gen_no = (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no;
        } else {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }
    }

    return alloc_in_moving_heap(size, gen_no);
}

#[inline(always)]
unsafe fn copy_tag(
    mut p: *mut *mut StgClosure,
    mut info: *const StgInfoTable,
    mut src: *mut StgClosure,
    mut size: uint32_t,
    mut gen_no: uint32_t,
    mut tag: StgWord,
) {
    let mut to = null_mut::<StgWord>();
    let mut from = null_mut::<StgWord>();
    let mut i: uint32_t = 0;
    to = alloc_for_copy(size, gen_no);
    from = src as StgPtr;
    *to.offset(0 as c_int as isize) = info as W_ as StgWord;
    i = 1 as uint32_t;

    while i < size {
        *to.offset(i as isize) = *from.offset(i as isize);
        i = i.wrapping_add(1);
    }

    (*src).header.info = (to as StgWord | 1 as StgWord) as *const StgInfoTable;
    *p = TAG_CLOSURE(tag, to as *mut StgClosure);
}

#[inline(always)]
unsafe fn copyPart(
    mut p: *mut *mut StgClosure,
    mut src: *mut StgClosure,
    mut size_to_reserve: uint32_t,
    mut size_to_copy: uint32_t,
    mut gen_no: uint32_t,
) -> bool {
    let mut to = null_mut::<StgWord>();
    let mut from = null_mut::<StgWord>();
    let mut i: uint32_t = 0;
    let mut info: StgWord = 0;
    info = (*src).header.info as W_ as StgWord;
    to = alloc_for_copy(size_to_reserve, gen_no);
    from = src as StgPtr;
    *to.offset(0 as c_int as isize) = info;
    i = 1 as uint32_t;

    while i < size_to_copy {
        *to.offset(i as isize) = *from.offset(i as isize);
        i = i.wrapping_add(1);
    }

    *p = to as *mut StgClosure;
    (*src).header.info = (to as StgWord | 1 as StgWord) as *const StgInfoTable;

    return r#true != 0;
}

#[inline(always)]
unsafe fn copy(
    mut p: *mut *mut StgClosure,
    mut info: *const StgInfoTable,
    mut src: *mut StgClosure,
    mut size: uint32_t,
    mut gen_no: uint32_t,
) {
    copy_tag(p, info, src, size, gen_no, 0 as StgWord);
}

#[inline(never)]
unsafe fn evacuate_large(mut p: StgPtr) {
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();
    let mut new_gen = null_mut::<generation>();
    let mut gen_no: uint32_t = 0;
    let mut new_gen_no: uint32_t = 0;
    let mut ws = null_mut::<gen_workspace>();
    bd = Bdescr(p);
    r#gen = (*bd).r#gen as *mut generation;
    gen_no = (*bd).gen_no as uint32_t;

    if (*bd).flags as c_int & BF_EVACUATED != 0 {
        if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }

        return;
    }

    dbl_link_remove(bd, &raw mut (*r#gen).large_objects);
    new_gen_no = (*bd).dest_no as uint32_t;

    if deadlock_detect_gc as c_long != 0 {
        new_gen_no = (*oldest_gen).no;
    } else if new_gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
        if (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion {
            new_gen_no = (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no;
        } else {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }
    }

    ws = (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
        .offset(new_gen_no as isize) as *mut gen_workspace;
    new_gen = generations.offset(new_gen_no as isize) as *mut generation;
    ::core::intrinsics::atomic_or_acqrel(&raw mut (*bd).flags, BF_EVACUATED as StgWord16);

    if (RtsFlags.GcFlags.useNonmoving as c_int != 0 && new_gen == oldest_gen) as c_int as c_long
        != 0
    {
        ::core::intrinsics::atomic_or_acqrel(&raw mut (*bd).flags, BF_NONMOVING as StgWord16);

        if major_gc as c_int != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                    .upd_rem_set
                    .queue,
                p as *mut StgClosure,
            );
        }
    }

    initBdescr(bd, new_gen, (*new_gen).to as *mut generation);

    if (*bd).flags as c_int & BF_PINNED != 0 {
        new_gen != r#gen;
        dbl_link_onto(bd, &raw mut (*new_gen).scavenged_large_objects);
        (*new_gen).n_scavenged_large_blocks = (*new_gen)
            .n_scavenged_large_blocks
            .wrapping_add((*bd).blocks as memcount);
        new_gen != r#gen;
    } else {
        (*bd).link = (*ws).0.todo_large_objects as *mut bdescr_;
        (*ws).0.todo_large_objects = bd;
    };
}

#[inline]
unsafe fn evacuate_static_object(mut link_field: *mut *mut StgClosure, mut q: *mut StgClosure) {
    if RtsFlags.GcFlags.useNonmoving as c_long != 0 {
        if major_gc as c_int != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                    .upd_rem_set
                    .queue,
                q,
            );
        }

        return;
    }

    let mut link: StgWord = *(link_field as *mut StgWord);

    if link & STATIC_BITS as StgWord | prev_static_flag as StgWord != 3 as StgWord {
        let mut new_list_head: StgWord = q as StgWord | static_flag as StgWord;
        *link_field = (*(&raw mut the_gc_thread as *mut gc_thread)).static_objects;

        let ref mut fresh15 = (*(&raw mut the_gc_thread as *mut gc_thread)).static_objects;
        *fresh15 = new_list_head as *mut StgClosure;
    }
}

#[inline]
unsafe fn evacuate_compact(mut p: StgPtr) {
    let mut str = null_mut::<StgCompactNFData>();
    let mut bd = null_mut::<bdescr>();
    let mut r#gen = null_mut::<generation>();
    let mut new_gen = null_mut::<generation>();
    let mut gen_no: uint32_t = 0;
    let mut new_gen_no: uint32_t = 0;
    str = objectGetCompact(p as *mut StgClosure);
    bd = Bdescr(str as StgPtr);
    gen_no = (*bd).gen_no as uint32_t;

    if (*bd).flags as c_int & BF_NONMOVING != 0 {
        if major_gc as c_int != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                    .upd_rem_set
                    .queue,
                str as *mut StgClosure,
            );
        }

        return;
    }

    if (*bd).flags as c_int & BF_EVACUATED != 0 {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.compact as c_long != 0 {
            trace_(
                b"Compact %p already evacuated\0" as *const u8 as *const c_char as *mut c_char,
                str,
            );
        }

        if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }

        return;
    }

    r#gen = (*bd).r#gen as *mut generation;
    gen_no = (*bd).gen_no as uint32_t;

    if (*bd).flags as c_int & BF_EVACUATED != 0 {
        if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }

        return;
    }

    dbl_link_remove(bd, &raw mut (*r#gen).compact_objects);
    new_gen_no = (*bd).dest_no as uint32_t;

    if new_gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
        if (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion {
            new_gen_no = (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no;
        } else {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }
    }

    new_gen = generations.offset(new_gen_no as isize) as *mut generation;
    (*bd).flags = ((*bd).flags as c_int | BF_EVACUATED) as StgWord16;

    if (RtsFlags.GcFlags.useNonmoving as c_int != 0 && new_gen == oldest_gen) as c_int as c_long
        != 0
    {
        ::core::intrinsics::atomic_or_relaxed(&raw mut (*bd).flags, BF_NONMOVING as StgWord16);

        if major_gc as c_int != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                    .upd_rem_set
                    .queue,
                str as *mut StgClosure,
            );
        }
    }

    initBdescr(bd, new_gen, (*new_gen).to as *mut generation);

    if !(*str).hash.is_null() {
        let mut ws: *mut gen_workspace =
            (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
                .offset(new_gen_no as isize) as *mut gen_workspace;
        (*bd).link = (*ws).0.todo_large_objects as *mut bdescr_;
        (*ws).0.todo_large_objects = bd;
    } else {
        new_gen != r#gen;
        dbl_link_onto(bd, &raw mut (*new_gen).live_compact_objects);
        (*new_gen).n_live_compact_blocks = ((*new_gen).n_live_compact_blocks as StgWord)
            .wrapping_add((*str).totalW.wrapping_div(BLOCK_SIZE_W as StgWord))
            as memcount as memcount;
        new_gen != r#gen;
    };
}

unsafe fn evacuate(mut p: *mut *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    let mut gen_no: uint32_t = 0;
    let mut q = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();
    let mut tag: StgWord = 0;
    q = *p;

    loop {
        tag = GET_CLOSURE_TAG(q);
        q = UNTAG_CLOSURE(q);

        if !(q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end) {
            if !major_gc {
                return;
            }

            if unload_mark_needed as c_long != 0 {
                markObjectCode(q as *const c_void);
            }

            info = get_itbl(q);

            match (*info).r#type {
                21 => {
                    if (*info).srt != 0 as StgSRTField {
                        evacuate_static_object(
                            (&raw mut (*q).payload as *mut *mut StgClosure_)
                                .offset(1 as c_int as isize)
                                as *mut *mut StgClosure,
                            q,
                        );
                    }

                    return;
                }
                14 => {
                    if (*info).srt != 0 as StgSRTField
                        || (*info).layout.payload.ptrs != 0 as StgHalfWord
                    {
                        evacuate_static_object(STATIC_LINK(info, q), q);
                    }

                    return;
                }
                28 => {
                    evacuate_static_object(
                        (&raw mut (*q).payload as *mut *mut StgClosure_).offset(1 as c_int as isize)
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
                        b"evacuate(static): strange closure type %d\0" as *const u8
                            as *const c_char,
                        (*info).r#type as c_int,
                    );
                }
            }
        }

        bd = Bdescr(q as StgPtr);

        let mut flags: uint16_t = (*bd).flags;

        if flags as c_int & (BF_LARGE | BF_MARKED | BF_EVACUATED | BF_COMPACT | BF_NONMOVING)
            != 0 as c_int
        {
            if ((*bd).flags as c_int & 1024 as c_int) as c_long != 0 {
                if major_gc as c_int != 0 && !deadlock_detect_gc {
                    markQueuePushClosureGC(
                        &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                            .upd_rem_set
                            .queue,
                        q,
                    );
                }

                return;
            }

            if flags as c_int & BF_EVACUATED != 0 {
                if ((*bd).gen_no as uint32_t)
                    < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no
                {
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                }

                return;
            }

            if flags as c_int & BF_COMPACT != 0 {
                evacuate_compact(q as StgPtr);
                return;
            }

            if flags as c_int & BF_LARGE != 0 {
                evacuate_large(q as StgPtr);
                return;
            }

            if is_marked(q as StgPtr, bd) == 0 {
                mark(q as StgPtr, bd);
                push_mark_stack(q as StgPtr);
            }

            return;
        }

        gen_no = (*bd).dest_no as uint32_t;
        info = (*q).header.info;

        if info as StgWord & 1 as StgWord != 0 as StgWord {
            let mut e = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
            *p = TAG_CLOSURE(tag, e);

            if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
                if ((*Bdescr(e as StgPtr)).gen_no as uint32_t)
                    < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no
                {
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                }
            }

            return;
        }

        match (*INFO_PTR_TO_STRUCT(info)).r#type {
            58 => {}
            3 => {
                let mut w: StgWord = *(&raw mut (*q).payload as *mut *mut StgClosure_)
                    .offset(0 as c_int as isize) as StgWord;

                if info == (*ghc_hs_iface).Czh_con_info && w as StgChar <= MAX_CHARLIKE as StgChar {
                    *p = TAG_CLOSURE(
                        tag,
                        CHARLIKE_CLOSURE(w as StgChar as c_int) as *mut StgClosure,
                    );
                } else if info == (*ghc_hs_iface).Izh_con_info
                    && w as StgInt >= MIN_INTLIKE as StgInt
                    && w as StgInt <= MAX_INTLIKE as StgInt
                {
                    *p = TAG_CLOSURE(
                        tag,
                        INTLIKE_CLOSURE(w as StgInt as c_int) as *mut StgClosure,
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
                            .wrapping_add(1 as usize) as uint32_t,
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
                        .wrapping_add(1 as usize) as uint32_t,
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
                        .wrapping_add(1 as usize) as uint32_t,
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
                        .wrapping_add(2 as usize) as uint32_t,
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
                        .wrapping_add(2 as usize) as uint32_t,
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
                        .wrapping_add(2 as usize) as uint32_t,
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
                    thunk_sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as uint32_t,
                    gen_no,
                );

                return;
            }
            8 | 1 | 7 => {
                copy_tag(
                    p,
                    info,
                    q,
                    sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as uint32_t,
                    gen_no,
                    tag,
                );

                return;
            }
            38 => {
                let mut r = null_mut::<StgClosure>();
                let mut i = null::<StgInfoTable>();
                r = (*(q as *mut StgInd)).indirectee;

                if GET_CLOSURE_TAG(r) == 0 as StgWord {
                    i = (*r).header.info;

                    if i as StgWord & 1 as StgWord != 0 as StgWord {
                        r = (i as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
                        i = (*r).header.info;
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
                                as uint32_t,
                            gen_no,
                        );

                        return;
                    }
                }

                q = r;
                *p = r;
            }
            47 | 48 | 39 | 40 | 41 | 37 | 49 | 50 | 51 => {
                copy(
                    p,
                    info,
                    q,
                    sizeW_fromITBL(INFO_PTR_TO_STRUCT(info)) as uint32_t,
                    gen_no,
                );

                return;
            }
            23 => {
                copy(p, info, q, bco_sizeW(q as *mut StgBCO) as uint32_t, gen_no);
                return;
            }
            22 => {
                eval_thunk_selector(p, q as *mut StgSelector, r#true != 0);
                return;
            }
            27 => {
                q = (*(q as *mut StgInd)).indirectee;
                *p = q;
            }
            29 | 30 | 31 | 33 | 35 | 36 | 34 | 57 | 56 | 55 | 65 => {
                barf(
                    b"evacuate: stack frame at %p\n\0" as *const u8 as *const c_char,
                    q,
                );
            }
            25 => {
                copy(p, info, q, pap_sizeW(q as *mut StgPAP) as uint32_t, gen_no);
                return;
            }
            24 => {
                copy(p, info, q, ap_sizeW(q as *mut StgAP) as uint32_t, gen_no);
                return;
            }
            26 => {
                copy(
                    p,
                    info,
                    q,
                    ap_stack_sizeW(q as *mut StgAP_STACK) as uint32_t,
                    gen_no,
                );

                return;
            }
            42 => {
                copy(
                    p,
                    info,
                    q,
                    arr_words_sizeW(q as *mut StgArrBytes) as uint32_t,
                    gen_no,
                );

                return;
            }
            43 | 44 | 46 | 45 => {
                copy(
                    p,
                    info,
                    q,
                    mut_arr_ptrs_sizeW(q as *mut StgMutArrPtrs) as uint32_t,
                    gen_no,
                );

                return;
            }
            59 | 60 | 62 | 61 => {
                copy(
                    p,
                    info,
                    q,
                    small_mut_arr_ptrs_sizeW(q as *mut StgSmallMutArrPtrs) as uint32_t,
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
                        .wrapping_div(size_of::<W_>() as usize) as uint32_t,
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
                    stack_sizeW(stack) as uint32_t,
                    (size_of::<StgStack>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as uint32_t,
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
                        .wrapping_div(size_of::<W_>() as usize) as uint32_t,
                    gen_no,
                );

                return;
            }
            64 => {
                copy(
                    p,
                    info,
                    q,
                    continuation_sizeW(q as *mut StgContinuation) as uint32_t,
                    gen_no,
                );

                return;
            }
            _ => {
                barf(
                    b"evacuate: strange closure type %d\0" as *const u8 as *const c_char,
                    (*INFO_PTR_TO_STRUCT(info)).r#type as c_int,
                );
            }
        }
    }
}

unsafe fn evacuate_BLACKHOLE(mut p: *mut *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    let mut gen_no: uint32_t = 0;
    let mut q = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();
    q = *p;
    bd = Bdescr(q as StgPtr);

    let flags: uint16_t = (*bd).flags;

    if ((*bd).flags as c_int & 1024 as c_int) as c_long != 0 {
        if major_gc as c_int != 0 && !deadlock_detect_gc {
            markQueuePushClosureGC(
                &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                    .upd_rem_set
                    .queue,
                q,
            );
        }

        return;
    }

    if flags as c_int & BF_LARGE != 0 {
        evacuate_large(q as StgPtr);
        return;
    }

    if flags as c_int & BF_EVACUATED != 0 {
        if ((*bd).gen_no as uint32_t) < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
        }

        return;
    }

    if flags as c_int & BF_MARKED != 0 {
        if is_marked(q as StgPtr, bd) == 0 {
            mark(q as StgPtr, bd);
            push_mark_stack(q as StgPtr);
        }

        return;
    }

    gen_no = (*bd).dest_no as uint32_t;
    info = (*q).header.info;

    if info as StgWord & 1 as StgWord != 0 as StgWord {
        let mut e = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
        *p = e;

        if gen_no < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no {
            if ((*Bdescr(e as StgPtr)).gen_no as uint32_t)
                < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no
            {
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
            }
        }

        return;
    }

    copy(
        p,
        info,
        q,
        (size_of::<StgInd>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as uint32_t,
        gen_no,
    );
}

unsafe fn unchain_thunk_selectors(mut p: *mut StgSelector, mut val: *mut StgClosure) {
    while !p.is_null() {
        let mut prev = *(&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
            .offset(0 as c_int as isize) as *mut StgSelector;

        if p as *mut StgClosure == val {
            let ref mut fresh12 = *(&raw mut (*(p as *mut StgThunk)).payload
                as *mut *mut StgClosure_)
                .offset(0 as c_int as isize);
            *fresh12 = val as *mut StgClosure_;
            SET_INFO_RELEASE(p as *mut StgClosure, &raw const stg_sel_0_upd_info);
        } else {
            let ref mut fresh13 = (*(p as *mut StgInd)).indirectee;
            *fresh13 = val;
            SET_INFO_RELEASE(p as *mut StgClosure, &raw const stg_IND_info);
        }

        p = prev;
    }
}

unsafe fn eval_thunk_selector(q: *mut *mut StgClosure, mut p: *mut StgSelector, mut evac: bool) {
    let mut val: *mut StgClosure = null_mut::<StgClosure>();
    let mut current_block: u64;
    let mut field: uint32_t = 0;
    let mut info = null_mut::<StgInfoTable>();
    let mut info_ptr: StgWord = 0;
    let mut selectee = null_mut::<StgClosure>();
    let mut prev_thunk_selector = null_mut::<StgSelector>();
    let mut bd = null_mut::<bdescr>();
    prev_thunk_selector = null_mut::<StgSelector>();

    '_selector_chain: loop {
        bd = Bdescr(p as StgPtr);

        if p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end {
            let flags: uint16_t = (*bd).flags;

            if flags as c_int & (BF_EVACUATED | BF_NONMOVING) != 0 {
                unchain_thunk_selectors(prev_thunk_selector, p as *mut StgClosure);

                if flags as c_int & BF_NONMOVING != 0 {
                    markQueuePushClosureGC(
                        &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                            .upd_rem_set
                            .queue,
                        p as *mut StgClosure,
                    );
                }

                *q = p as *mut StgClosure;

                if evac as c_int != 0
                    && ((*bd).gen_no as uint32_t)
                        < (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no
                {
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                }

                return;
            }

            if flags as c_int & BF_MARKED != 0 {
                *q = p as *mut StgClosure;

                if evac {
                    evacuate(q);
                }

                unchain_thunk_selectors(prev_thunk_selector, p as *mut StgClosure);
                return;
            }
        }

        info_ptr = (*p).header.info as StgWord;
        SET_INFO(p as *mut StgClosure, &raw const stg_WHITEHOLE_info);
        field = (*INFO_PTR_TO_STRUCT(info_ptr as *mut StgInfoTable))
            .layout
            .selector_offset as uint32_t;
        selectee = UNTAG_CLOSURE((*p).selectee);

        loop {
            info = *(&raw mut (*selectee).header.info as *mut *mut StgInfoTable);

            if info as StgWord & 1 as StgWord != 0 as StgWord {
                current_block = 17090990969405305017;
                break '_selector_chain;
            } else {
                info = INFO_PTR_TO_STRUCT(info);

                match (*info).r#type {
                    1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                        val = *(&raw mut (*selectee).payload as *mut *mut StgClosure_)
                            .offset(field as isize);
                        break;
                    }
                    27 | 28 => {
                        selectee = UNTAG_CLOSURE((*(selectee as *mut StgInd)).indirectee);
                    }
                    38 => {
                        let mut r = null_mut::<StgClosure>();
                        let mut i = null::<StgInfoTable>();
                        r = (*(selectee as *mut StgInd)).indirectee;

                        if GET_CLOSURE_TAG(r) == 0 as StgWord {
                            i = (*r).header.info;

                            if i as StgWord & 1 as StgWord != 0 as StgWord {
                                r = (i as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
                                i = (*r).header.info;
                            }

                            if i == &raw const stg_TSO_info
                                || i == &raw const stg_WHITEHOLE_info
                                || i == &raw const stg_BLOCKING_QUEUE_CLEAN_info
                                || i == &raw const stg_BLOCKING_QUEUE_DIRTY_info
                            {
                                current_block = 17090990969405305017;
                                break '_selector_chain;
                            }
                        }

                        selectee = UNTAG_CLOSURE((*(selectee as *mut StgInd)).indirectee);
                    }
                    22 => {
                        let mut val_0 = null_mut::<StgClosure>();

                        if (*(&raw mut the_gc_thread as *mut gc_thread)).thunk_selector_depth
                            >= MAX_THUNK_SELECTOR_DEPTH as W_
                        {
                            if isNonmovingClosure(p as *mut StgClosure) {
                                markQueuePushClosureGC(
                                    &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                                        .upd_rem_set
                                        .queue,
                                    p as *mut StgClosure,
                                );
                            }

                            current_block = 17090990969405305017;
                            break '_selector_chain;
                        } else {
                            let ref mut fresh10 =
                                (*(&raw mut the_gc_thread as *mut gc_thread)).thunk_selector_depth;
                            *fresh10 = (*fresh10).wrapping_add(1);

                            eval_thunk_selector(
                                &raw mut val_0,
                                selectee as *mut StgSelector,
                                r#false != 0,
                            );

                            let ref mut fresh11 =
                                (*(&raw mut the_gc_thread as *mut gc_thread)).thunk_selector_depth;
                            *fresh11 = (*fresh11).wrapping_sub(1);

                            if val_0 == selectee {
                                current_block = 17090990969405305017;
                                break '_selector_chain;
                            }

                            selectee = UNTAG_CLOSURE(val_0);
                        }
                    }
                    58 | 24 | 26 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => {
                        current_block = 17090990969405305017;
                        break '_selector_chain;
                    }
                    _ => {
                        barf(
                            b"eval_thunk_selector: strange selectee %d\0" as *const u8
                                as *const c_char,
                            (*info).r#type as c_int,
                        );
                    }
                }
            }
        }

        loop {
            info_ptr = *(&raw mut (*(UNTAG_CLOSURE
                as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
                val
            ))
            .header
            .info as *mut StgWord);

            if info_ptr & 1 as StgWord != 0 as StgWord {
                current_block = 7427571413727699167;
                break '_selector_chain;
            }

            info = INFO_PTR_TO_STRUCT(info_ptr as *mut StgInfoTable);

            match (*info).r#type {
                27 | 28 => {}
                22 => {
                    break;
                }
                _ => {
                    current_block = 7427571413727699167;
                    break '_selector_chain;
                }
            }

            val = (*(val as *mut StgInd)).indirectee;
        }

        let ref mut fresh8 = *(&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
            .offset(0 as c_int as isize);
        *fresh8 = prev_thunk_selector as *mut StgClosure as *mut StgClosure_;
        prev_thunk_selector = p;
        p = val as *mut StgSelector;
    }

    match current_block {
        17090990969405305017 => {
            SET_INFO_RELAXED(p as *mut StgClosure, info_ptr as *const StgInfoTable);
            *q = p as *mut StgClosure;

            if evac {
                copy(
                    q,
                    info_ptr as *const StgInfoTable,
                    p as *mut StgClosure,
                    THUNK_SELECTOR_sizeW() as uint32_t,
                    (*bd).dest_no as uint32_t,
                );
            }

            if isNonmovingClosure(*q) {
                markQueuePushClosureGC(
                    &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                        .upd_rem_set
                        .queue,
                    *q,
                );
            }

            unchain_thunk_selectors(prev_thunk_selector, *q);
            return;
        }
        _ => {
            let ref mut fresh9 = *(&raw mut (*(p as *mut StgClosure)).payload
                as *mut *mut StgClosure_)
                .offset(0 as c_int as isize);
            *fresh9 = prev_thunk_selector as *mut StgClosure as *mut StgClosure_;
            prev_thunk_selector = p;
            *q = val;
            unchain_thunk_selectors(prev_thunk_selector, val);

            if evac {
                evacuate(q);
            } else if isNonmovingClosure(*q) {
                markQueuePushClosureGC(
                    &raw mut (*(*(&raw mut the_gc_thread as *mut gc_thread)).cap)
                        .upd_rem_set
                        .queue,
                    *q,
                );
            }

            return;
        }
    };
}
