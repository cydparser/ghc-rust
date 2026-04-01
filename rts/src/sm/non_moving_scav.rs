use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::Bdescr;
use crate::ffi::rts::storage::closure_macros::{
    LOOKS_LIKE_CLOSURE_PTR, get_itbl, small_mut_arr_ptrs_sizeW,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgBCO, StgBlockingQueue, StgClosure_, StgCompactNFData, StgContinuation,
    StgInd, StgMVar, StgMutArrPtrs, StgMutVar, StgPAP, StgSelector, StgSmallMutArrPtrs,
    StgTRecChunk, StgTVar, StgThunk, StgWeak, TRecEntry,
};
use crate::ffi::rts::storage::gc::oldest_gen;
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::P_;
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info, stg_MUT_ARR_PTRS_CLEAN_info,
    stg_MUT_ARR_PTRS_DIRTY_info, stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    stg_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_MUT_VAR_CLEAN_info, stg_MUT_VAR_DIRTY_info,
    stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_CLEAN_info,
    stg_SMALL_MUT_ARR_PTRS_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_TVAR_CLEAN_info, stg_TVAR_DIRTY_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8};
use crate::prelude::*;
use crate::sm::evac::evacuate;
use crate::sm::gc_utils::recordMutableGen_GC;
use crate::sm::gct_decl::gct;
use crate::sm::non_moving::{
    NonmovingSegment, nonmoving_block_idx, nonmovingGetBlockIdx, nonmovingGetMark,
    nonmovingSegmentBlockSize, nonmovingSegmentGetBlock,
};
use crate::sm::non_moving_mark::markQueuePushClosureGC;
use crate::sm::scav::{
    scavenge_AP, scavenge_PAP, scavenge_compact, scavenge_continuation, scavenge_fun_srt,
    scavenge_mut_arr_ptrs, scavenge_stack, scavenge_thunk_srt, scavengeTSO,
};

unsafe fn nonmovingScavengeOne(mut q: *mut StgClosure) {
    let mut current_block: u64;

    if LOOKS_LIKE_CLOSURE_PTR(q as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingScav.c".as_ptr(), 89);
    }

    let mut p = q as StgPtr;
    let mut info = get_itbl(q);
    let saved_eager_promotion = (*gct).eager_promotion;

    match (*info).r#type {
        39 | 40 => {
            let mut mvar = p as *mut StgMVar;
            (*gct).eager_promotion = false;
            evacuate(&raw mut (*mvar).head as *mut *mut StgClosure);
            evacuate(&raw mut (*mvar).tail as *mut *mut StgClosure);
            evacuate(&raw mut (*mvar).value);
            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*mvar).header.info = &raw const stg_MVAR_DIRTY_info;

                markQueuePushClosureGC(
                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                    (*mvar).head as *mut StgClosure,
                );

                markQueuePushClosureGC(
                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                    (*mvar).tail as *mut StgClosure,
                );

                markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, (*mvar).value);
            } else {
                (*mvar).header.info = &raw const stg_MVAR_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        41 => {
            let mut tvar = p as *mut StgTVar;
            (*gct).eager_promotion = false;
            evacuate(&raw mut (*tvar).current_value);
            evacuate(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);
            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*tvar).header.info = &raw const stg_TVAR_DIRTY_info;

                markQueuePushClosureGC(
                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                    (*tvar).current_value,
                );

                markQueuePushClosureGC(
                    &raw mut (*(*gct).cap).upd_rem_set.queue,
                    (*tvar).first_watch_queue_entry as *mut StgClosure,
                );
            } else {
                (*tvar).header.info = &raw const stg_TVAR_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        11 => {
            scavenge_fun_srt(info);

            evacuate(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                    as *mut *mut StgClosure,
            );

            evacuate(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            current_block = 13665239467142187023;
        }
        18 => {
            scavenge_thunk_srt(info);

            evacuate(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(1)
                    as *mut *mut StgClosure,
            );

            evacuate(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            current_block = 13665239467142187023;
        }
        4 => {
            evacuate(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                    as *mut *mut StgClosure,
            );

            evacuate(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            current_block = 13665239467142187023;
        }
        16 => {
            scavenge_thunk_srt(info);

            evacuate(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            current_block = 13665239467142187023;
        }
        9 => {
            scavenge_fun_srt(info);
            current_block = 7249696149466105880;
        }
        2 => {
            current_block = 7249696149466105880;
        }
        17 => {
            scavenge_thunk_srt(info);
            current_block = 13665239467142187023;
        }
        10 => {
            scavenge_fun_srt(info);
            current_block = 13665239467142187023;
        }
        20 => {
            scavenge_thunk_srt(info);
            current_block = 13665239467142187023;
        }
        13 => {
            scavenge_fun_srt(info);
            current_block = 13665239467142187023;
        }
        19 => {
            scavenge_thunk_srt(info);

            evacuate(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            current_block = 13665239467142187023;
        }
        12 => {
            scavenge_fun_srt(info);
            current_block = 15640826738926244895;
        }
        5 => {
            current_block = 15640826738926244895;
        }
        8 => {
            scavenge_fun_srt(info);
            current_block = 9512719473022792396;
        }
        15 => {
            scavenge_thunk_srt(info);

            let mut end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                .offset((*info).layout.payload.ptrs as isize);
            p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_ as StgPtr;

            while p < end {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            current_block = 13665239467142187023;
        }
        49 => {
            let mut weak = p as *mut StgWeak;
            (*gct).eager_promotion = true;
            evacuate(&raw mut (*weak).key);
            (*gct).eager_promotion = saved_eager_promotion;
            current_block = 9512719473022792396;
        }
        1 | 7 | 50 => {
            current_block = 9512719473022792396;
        }
        23 => {
            let mut bco = p as *mut StgBCO;
            evacuate(&raw mut (*bco).instrs as *mut *mut StgClosure);
            evacuate(&raw mut (*bco).literals as *mut *mut StgClosure);
            evacuate(&raw mut (*bco).ptrs as *mut *mut StgClosure);
            current_block = 13665239467142187023;
        }
        47 | 48 => {
            let mut mv = p as *mut StgMutVar;
            (*gct).eager_promotion = false;
            evacuate(&raw mut (*mv).var);
            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*q).header.info = &raw const stg_MUT_VAR_DIRTY_info;

                markQueuePushClosureGC(&raw mut (*(*gct).cap).upd_rem_set.queue, (*mv).var);
            } else {
                (*q).header.info = &raw const stg_MUT_VAR_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        37 => {
            let mut bq = p as *mut StgBlockingQueue;
            (*gct).eager_promotion = false;
            evacuate(&raw mut (*bq).bh);
            evacuate(&raw mut (*bq).owner as *mut *mut StgClosure);
            evacuate(&raw mut (*bq).queue as *mut *mut StgClosure);
            evacuate(&raw mut (*bq).link as *mut *mut StgClosure);
            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
            } else {
                (*bq).header.info = &raw const stg_BLOCKING_QUEUE_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        22 => {
            let mut s = p as *mut StgSelector;
            evacuate(&raw mut (*s).selectee);
            current_block = 13665239467142187023;
        }
        26 => {
            let mut ap = p as *mut StgAP_STACK;
            evacuate(&raw mut (*ap).fun);

            scavenge_stack(
                &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap).size as isize),
            );

            current_block = 13665239467142187023;
        }
        25 => {
            p = scavenge_PAP(p as *mut StgPAP);
            current_block = 13665239467142187023;
        }
        24 => {
            scavenge_AP(p as *mut StgAP);
            current_block = 13665239467142187023;
        }
        3 | 6 | 42 => {
            current_block = 13665239467142187023;
        }
        43 | 44 => {
            (*gct).eager_promotion = false;
            scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);
            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*q).header.info = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
            } else {
                (*q).header.info = &raw const stg_MUT_ARR_PTRS_CLEAN_info;
            }

            (*gct).failed_to_evac = true;
            current_block = 13665239467142187023;
        }
        46 | 45 => {
            scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

            if (*gct).failed_to_evac {
                (*q).header.info = &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info;
            } else {
                (*q).header.info = &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        59 | 60 => {
            let mut next =
                p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
            (*gct).eager_promotion = false;
            p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                as StgPtr;

            while p < next {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            (*gct).eager_promotion = saved_eager_promotion;

            if (*gct).failed_to_evac {
                (*q).header.info = &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info;
            } else {
                (*q).header.info = &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info;
            }

            (*gct).failed_to_evac = true;
            current_block = 13665239467142187023;
        }
        62 | 61 => {
            let mut next_0 =
                p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
            p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                as StgPtr;

            while p < next_0 {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            if (*gct).failed_to_evac {
                (*q).header.info = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info;
            } else {
                (*q).header.info = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info;
            }

            current_block = 13665239467142187023;
        }
        52 => {
            scavengeTSO(p as *mut StgTSO);
            current_block = 13665239467142187023;
        }
        53 => {
            let mut stack = p as *mut StgStack;
            (*gct).eager_promotion = false;

            scavenge_stack(
                (*stack).sp,
                (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
            );

            (*gct).eager_promotion = saved_eager_promotion;
            (*stack).dirty = (*gct).failed_to_evac as StgWord8;
            current_block = 13665239467142187023;
        }
        51 => {
            let mut end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_
                as P_)
                .offset((*info).layout.payload.ptrs as isize);
            (*gct).eager_promotion = false;
            p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_ as StgPtr;

            while p < end_1 {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            (*gct).eager_promotion = saved_eager_promotion;
            (*gct).failed_to_evac = true;
            current_block = 13665239467142187023;
        }
        54 => {
            let mut i: StgWord = 0;
            let mut tc = p as *mut StgTRecChunk;
            let mut e: *mut TRecEntry =
                (&raw mut (*tc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
            (*gct).eager_promotion = false;
            evacuate(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
            i = 0;

            while i < (*tc).next_entry_idx {
                evacuate(&raw mut (*e).tvar as *mut *mut StgClosure);
                evacuate(&raw mut (*e).expected_value);
                evacuate(&raw mut (*e).new_value);
                i = i.wrapping_add(1);
                e = e.offset(1);
            }

            (*gct).eager_promotion = saved_eager_promotion;
            (*gct).failed_to_evac = true;
            current_block = 13665239467142187023;
        }
        27 | 38 | 28 => {
            evacuate(&raw mut (*(p as *mut StgInd)).indirectee);
            current_block = 13665239467142187023;
        }
        63 => {
            scavenge_compact(p as *mut StgCompactNFData);
            current_block = 13665239467142187023;
        }
        64 => {
            scavenge_continuation(p as *mut StgContinuation);
            current_block = 13665239467142187023;
        }
        _ => {
            barf(
                c"nonmoving scavenge: unimplemented/strange closure type %d @ %p".as_ptr(),
                (*info).r#type,
                p,
            );
        }
    }

    match current_block {
        9512719473022792396 => {
            let mut end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_
                as P_)
                .offset((*info).layout.payload.ptrs as isize);
            p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_ as StgPtr;

            while p < end_0 {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }
        }
        7249696149466105880 => {
            evacuate(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );
        }
        15640826738926244895 => {
            evacuate(
                (&raw mut (*q).payload as *mut *mut StgClosure_).offset(0) as *mut *mut StgClosure
            );
        }
        _ => {}
    }

    if (*gct).failed_to_evac {
        (*gct).failed_to_evac = false;

        if (*oldest_gen).no > 0 {
            recordMutableGen_GC(q, (*oldest_gen).no);
        }
    }
}

unsafe fn scavengeNonmovingSegment(mut seg: *mut NonmovingSegment) {
    let blk_size: StgWord = nonmovingSegmentBlockSize(seg) as StgWord;
    (*gct).evac_gen_no = (*oldest_gen).no;
    (*gct).failed_to_evac = false;

    let mut seg_block = Bdescr(seg as StgPtr);

    if ((*seg_block).u.scan >= nonmovingSegmentGetBlock(seg, 0) as P_) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/NonMovingScav.c".as_ptr(), 460);
    }

    if ((*seg_block).u.scan <= nonmovingSegmentGetBlock(seg, (*seg).next_free) as P_) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/NonMovingScav.c".as_ptr(), 461);
    }

    let mut scan = (*seg_block).u.scan;
    let mut scan_end = nonmovingSegmentGetBlock(seg, (*seg).next_free) as StgPtr;

    if scan == scan_end {
        return;
    }

    (*seg_block).u.scan = scan_end;

    let mut p_idx = nonmovingGetBlockIdx(scan);

    while scan < scan_end {
        let mut p = scan as *mut StgClosure;

        if nonmovingGetMark(seg, p_idx) as i32 == 0 {
            nonmovingScavengeOne(p);
        }

        scan = (scan as *mut u8).offset(blk_size as isize) as StgPtr;
        p_idx = p_idx.wrapping_add(1);
    }
}
