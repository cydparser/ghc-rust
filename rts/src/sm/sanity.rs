use crate::arena::{arenaBlocks, checkPtrInArena};
use crate::capability::Capability;
use crate::capability::getCapability;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{
    BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK, BlockedOnBlackHole, BlockedOnMVar, BlockedOnMVarRead,
    BlockedOnMsgThrowTo, NotBlocked, TSO_MARKED,
};
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::storage::block::{
    BF_PINNED, BF_SWEPT, BLOCK_SIZE_W, BLOCKS_PER_MBLOCK, Bdescr, bdescr,
};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, GET_INFO, INFO_PTR_TO_STRUCT, LOOKS_LIKE_CLOSURE_PTR, LOOKS_LIKE_INFO_PTR,
    STATIC_LINK, THUNK_SELECTOR_sizeW, UNTAG_CONST_CLOSURE, ap_sizeW, ap_stack_sizeW,
    arr_words_sizeW, bco_sizeW, closure_sizeW, continuation_sizeW, get_fun_itbl, get_itbl,
    get_ret_itbl, mut_arr_ptrs_sizeW, pap_sizeW, sizeW_fromITBL, small_mut_arr_ptrs_sizeW,
    stack_sizeW, thunk_sizeW_fromITBL,
};
use crate::ffi::rts::storage::closures::{
    MessageBlackHole, StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgBlockingQueue, StgClosure_,
    StgCompactNFData, StgCompactNFDataBlock, StgContinuation, StgInd, StgIndStatic, StgMVar,
    StgMutArrPtrs, StgPAP, StgRetFun, StgSelector, StgSmallMutArrPtrs, StgTRecChunk, StgThunk,
    StgUnderflowFrame, StgUpdateFrame, StgWeak,
};
use crate::ffi::rts::storage::gc::{g0, generation, generations, memcount, nursery, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::info_tables::{StgLargeBitmap, stg_arg_bitmaps};
use crate::ffi::rts::storage::m_block::mblocks_allocated;
use crate::ffi::rts::storage::tso::{STACK_DIRTY, STACK_SANE, StgStack};
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::misc_closures::{
    stg_DEAD_WEAK_info, stg_END_TSO_QUEUE_closure, stg_IND_info, stg_MSG_BLACKHOLE_info,
    stg_MVAR_TSO_QUEUE_info, stg_TSO_info, stg_WEAK_info, stg_WHITEHOLE_info,
    stg_maskAsyncExceptionszh_ret_info, stg_maskUninterruptiblezh_ret_info,
    stg_stack_underflow_frame_d_info, stg_stack_underflow_frame_v16_info,
    stg_stack_underflow_frame_v32_info, stg_stack_underflow_frame_v64_info,
    stg_unmaskAsyncExceptionszh_ret_info,
};
use crate::ffi::stg::types::{StgHalfWord, StgOffset, StgPtr, StgWord, StgWord8, StgWord32};
use crate::ffi::stg::{BITS_PER_BYTE, P_, W_};
use crate::prelude::*;
use crate::printer::info_type;
use crate::profiling::prof_arena;
use crate::retainer_profile::retainerStackBlocks;
use crate::rts_flags::{HEAP_BY_RETAINER, RtsFlags};
use crate::sm::block_alloc::{
    checkFreeListSanity, countAllocdBlocks, countBlocks, countFreeList, markBlocks, n_alloc_blocks,
    reportUnmarkedBlocks,
};
use crate::sm::cnf::{compactMarkKnown, countAllocdCompactBlocks, countCompactBlocks};
use crate::sm::gc_thread::{gc_threads, gen_workspace};
use crate::sm::non_moving::{
    NonmovingAllocator, NonmovingHeap, NonmovingSegment, nonmoving_alloca_cnt, nonmoving_block_idx,
    nonmovingBlockConcurrentMark, nonmovingHeap, nonmovingMarkEpoch, nonmovingSegmentBlockCount,
    nonmovingSegmentGetBlock, nonmovingSegmentInfo, nonmovingUnblockConcurrentMark,
};
use crate::sm::non_moving_mark::{
    current_mark_queue, n_nonmoving_compact_blocks, n_nonmoving_large_blocks,
    n_nonmoving_marked_compact_blocks, n_nonmoving_marked_large_blocks, nonmoving_compact_objects,
    nonmoving_large_objects, nonmoving_marked_compact_objects, nonmoving_marked_large_objects,
    upd_rem_set_block_list,
};
use crate::sm::storage::{STATIC_BITS, exec_block, n_nurseries, nurseries, static_flag};

unsafe fn isHeapAlloced(mut p: StgPtr) -> i32 {
    return (p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end)
        as i32;
}

unsafe fn isNonmovingGen(mut r#gen: *mut generation) -> bool {
    return RtsFlags.GcFlags.useNonmoving as i32 != 0 && r#gen == oldest_gen;
}

unsafe fn checkSmallBitmap(mut payload: StgPtr, mut bitmap: StgWord, mut size: u32) {
    let mut i: u32 = 0;
    i = 0;

    while i < size {
        if bitmap & 1 == 0 {
            checkClosureShallow(*payload.offset(i as isize) as *mut StgClosure);
        }

        i = i.wrapping_add(1);
        bitmap >>= 1;
    }
}

unsafe fn checkLargeBitmap(
    mut payload: StgPtr,
    mut large_bitmap: *mut StgLargeBitmap,
    mut size: u32,
) {
    let mut bmp: StgWord = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    i = 0;
    bmp = 0;

    while i < size {
        let mut bitmap: StgWord =
            *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(bmp as isize);
        j = 0;

        while i < size
            && (j as usize) < (BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize)
        {
            if bitmap & 1 == 0 {
                checkClosureShallow(*payload.offset(i as isize) as *mut StgClosure);
            }

            j = j.wrapping_add(1);
            i = i.wrapping_add(1);
            bitmap >>= 1;
        }

        bmp = bmp.wrapping_add(1);
    }
}

unsafe fn checkClosureShallow(mut p: *const StgClosure) {
    if LOOKS_LIKE_CLOSURE_PTR(UNTAG_CONST_CLOSURE(p) as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 105);
    };
}

unsafe fn checkStackFrame(mut c: StgPtr) -> StgOffset {
    let mut size: u32 = 0;
    let mut info = null::<StgRetInfoTable>();
    info = get_ret_itbl(c as *mut StgClosure);

    match (*info).i.r#type {
        33 => {
            if LOOKS_LIKE_CLOSURE_PTR((*(c as *mut StgUpdateFrame)).updatee as *const c_void) as i32
                as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 121);
            }
        }
        55 | 56 | 57 | 34 | 35 | 36 | 30 | 65 => {}
        29 => {
            let mut bco = null_mut::<StgBCO>();
            let mut size_0: u32 = 0;
            bco = *c.offset(1) as *mut StgBCO;
            size_0 = (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size as u32;

            checkLargeBitmap(
                c.offset(2),
                &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size_0,
            );

            return (2 as u32).wrapping_add(size_0) as StgOffset;
        }
        31 => {
            size = (*(((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                as *mut StgLargeBitmap))
                .size as u32;

            checkLargeBitmap(
                c.offset(1),
                ((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            return (1 as u32).wrapping_add(size) as StgOffset;
        }
        32 => {
            let mut fun_info = null::<StgFunInfoTable>();
            let mut ret_fun = null_mut::<StgRetFun>();
            ret_fun = c as *mut StgRetFun;
            fun_info = get_fun_itbl(UNTAG_CONST_CLOSURE((*ret_fun).fun));
            size = (*ret_fun).size as u32;

            match (*fun_info).f.fun_type {
                0 => {
                    checkSmallBitmap(
                        &raw mut (*ret_fun).payload as *mut *mut StgClosure as StgPtr,
                        (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT,
                        size,
                    );
                }
                1 => {
                    checkLargeBitmap(
                        &raw mut (*ret_fun).payload as *mut *mut StgClosure as StgPtr,
                        (fun_info.offset(1 as i32 as isize) as StgWord)
                            .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                            as *mut StgLargeBitmap,
                        size,
                    );
                }
                _ => {
                    checkSmallBitmap(
                        &raw mut (*ret_fun).payload as *mut *mut StgClosure as StgPtr,
                        *(&raw const stg_arg_bitmaps as *const StgWord)
                            .offset((*fun_info).f.fun_type as isize)
                            >> BITMAP_BITS_SHIFT,
                        size,
                    );
                }
            }

            return (size_of::<StgRetFun>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(size as usize) as StgOffset;
        }
        _ => {
            barf(
                c"checkStackFrame: weird activation record found on stack (%p %d).".as_ptr(),
                c,
                (*info).i.r#type,
            );
        }
    }

    size = ((*info).i.layout.bitmap & BITMAP_SIZE_MASK as StgWord) as u32;
    checkSmallBitmap(
        c.offset(1),
        (*info).i.layout.bitmap >> BITMAP_BITS_SHIFT,
        size,
    );

    return (1 as u32).wrapping_add(size) as StgOffset;
}

unsafe fn checkStackChunk(mut sp: StgPtr, mut stack_end: StgPtr) {
    let mut p = null_mut::<StgWord>();
    p = sp;

    while p < stack_end {
        p = p.offset(checkStackFrame(p) as isize);
    }

    if (p == stack_end) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 192);
    };
}

unsafe fn checkPAP(
    mut tagged_fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut n_args: StgWord,
) {
    let mut fun = null::<StgClosure>();
    let mut fun_info = null::<StgFunInfoTable>();
    fun = UNTAG_CONST_CLOSURE(tagged_fun);

    if LOOKS_LIKE_CLOSURE_PTR(fun as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 202);
    }

    fun_info = get_fun_itbl(fun);

    match (*fun_info).f.fun_type {
        0 => {
            checkSmallBitmap(
                payload as StgPtr,
                (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT,
                n_args as u32,
            );
        }
        1 => {
            checkLargeBitmap(
                payload as StgPtr,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                n_args as u32,
            );
        }
        2 => {
            checkLargeBitmap(
                payload as StgPtr,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                n_args as u32,
            );
        }
        _ => {
            checkSmallBitmap(
                payload as StgPtr,
                *(&raw const stg_arg_bitmaps as *const StgWord)
                    .offset((*fun_info).f.fun_type as isize)
                    >> BITMAP_BITS_SHIFT,
                n_args as u32,
            );
        }
    }

    if (if (*fun_info).f.arity > ((1 << 3) - 1) as StgHalfWord {
        (GET_CLOSURE_TAG(tagged_fun) == 0) as i32
    } else {
        (GET_CLOSURE_TAG(tagged_fun) == (*fun_info).f.arity as StgWord) as i32
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 228);
    };
}

unsafe fn checkClosureProfSanity(mut p: *const StgClosure) {
    let mut prof_hdr = (*p).header.prof;
    let mut ccs = prof_hdr.ccs;

    if ccs as *mut c_void as W_ >= mblock_address_space.0.begin
        && (ccs as *mut c_void as W_) < mblock_address_space.0.end
    {
        checkPtrInArena(ccs as StgPtr, prof_arena);
    }
}

unsafe fn checkGenWeakPtrList(mut g: u32) {
    let mut w = (*generations.offset(g as isize)).weak_ptr_list;

    while !w.is_null() {
        if LOOKS_LIKE_CLOSURE_PTR(w as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 339);
        }

        if ((*w).header.info == &raw const stg_WEAK_info
            || (*w).header.info == &raw const stg_DEAD_WEAK_info) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 340);
        }

        if LOOKS_LIKE_CLOSURE_PTR((*w).key as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 341);
        }

        if LOOKS_LIKE_CLOSURE_PTR((*w).value as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 342);
        }

        if LOOKS_LIKE_CLOSURE_PTR((*w).finalizer as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 343);
        }

        if LOOKS_LIKE_CLOSURE_PTR((*w).cfinalizers as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 344);
        }

        w = (*w).link as *mut StgWeak;
    }
}

unsafe fn checkClosure(mut p: *const StgClosure) -> StgOffset {
    let mut info = null::<StgInfoTable>();

    if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 354);
    }

    p = UNTAG_CONST_CLOSURE(p);
    info = (&raw const (*p).header.info).load(Ordering::Acquire);

    if info as StgWord & 1 != 0 {
        if LOOKS_LIKE_CLOSURE_PTR(info as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 361);
        }

        barf(
            c"checkClosure: found EVACUATED closure %u".as_ptr(),
            (*GET_INFO((info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure)).r#type,
        );
    }

    checkClosureProfSanity(p);
    info = INFO_PTR_TO_STRUCT(info);

    match (*info).r#type {
        39 | 40 => {
            let mut mvar = p as *mut StgMVar;

            if LOOKS_LIKE_CLOSURE_PTR((*mvar).head as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 377);
            }

            if LOOKS_LIKE_CLOSURE_PTR((*mvar).tail as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 378);
            }

            if LOOKS_LIKE_CLOSURE_PTR((*mvar).value as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 379);
            }

            return (size_of::<StgMVar>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgOffset;
        }
        15 | 16 | 17 | 19 | 20 | 18 => {
            let mut i: u32 = 0;
            i = 0;

            while i < (*info).layout.payload.ptrs {
                if LOOKS_LIKE_CLOSURE_PTR(
                    *(&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(i as isize) as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 392);
                }

                i = i.wrapping_add(1);
            }

            return thunk_sizeW_fromITBL(info);
        }

        8 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3 | 5 | 6 | 4 | 38 | 50 | 51 | 47 | 48 | 41
        | 21 | 14 | 63 => {
            let mut i_0: u32 = 0;
            i_0 = 0;

            while i_0 < (*info).layout.payload.ptrs {
                if LOOKS_LIKE_CLOSURE_PTR(
                    *(&raw const (*p).payload as *const *mut StgClosure_).offset(i_0 as isize)
                        as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 422);
                }

                i_0 = i_0.wrapping_add(1);
            }

            return sizeW_fromITBL(info);
        }
        37 => {
            let mut bq = p as *mut StgBlockingQueue;

            if LOOKS_LIKE_CLOSURE_PTR((*bq).bh as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 433);
            }

            if ((*get_itbl((*bq).owner as *mut StgClosure)).r#type == 52) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 435);
            }

            if ((*bq).queue
                == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut MessageBlackHole
                || (*(*bq).queue).header.info == &raw const stg_MSG_BLACKHOLE_info
                || (*(*bq).queue).header.info == &raw const stg_IND_info) as i32
                as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 441);
            }

            if ((*bq).link
                == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                    as *mut StgBlockingQueue
                || (*get_itbl((*bq).link as *mut StgClosure)).r#type == 27
                || (*get_itbl((*bq).link as *mut StgClosure)).r#type == 37) as i32
                as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 444);
            }

            return (size_of::<StgBlockingQueue>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgOffset;
        }
        23 => {
            let mut bco = p as *mut StgBCO;

            if LOOKS_LIKE_CLOSURE_PTR((*bco).instrs as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 451);
            }

            if LOOKS_LIKE_CLOSURE_PTR((*bco).literals as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 452);
            }

            if LOOKS_LIKE_CLOSURE_PTR((*bco).ptrs as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 453);
            }

            return bco_sizeW(bco) as StgOffset;
        }
        28 => {
            if LOOKS_LIKE_CLOSURE_PTR((*(p as *mut StgIndStatic)).indirectee as *const c_void)
                as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 458);
            }

            return sizeW_fromITBL(info);
        }
        49 => {
            let mut w = p as *mut StgWeak;

            if LOOKS_LIKE_CLOSURE_PTR((*w).cfinalizers as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 468);
            }

            return sizeW_fromITBL(info);
        }
        22 => {
            if LOOKS_LIKE_CLOSURE_PTR((*(p as *mut StgSelector)).selectee as *const c_void) as i32
                as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 473);
            }

            return THUNK_SELECTOR_sizeW();
        }
        27 => {
            let mut ind = p as *mut StgInd;

            if LOOKS_LIKE_CLOSURE_PTR((*ind).indirectee as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 482);
            }

            return (size_of::<StgInd>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgOffset;
        }
        29 | 30 | 31 | 33 | 35 | 36 | 34 | 55 | 56 | 57 => {
            barf(c"checkClosure: stack frame".as_ptr());
        }
        24 => {
            let mut ap = p as *mut StgAP;

            checkPAP(
                (*ap).fun,
                &raw mut (*ap).payload as *mut *mut StgClosure,
                (*ap).n_args as StgWord,
            );

            return ap_sizeW(ap);
        }
        25 => {
            let mut pap = p as *mut StgPAP;

            checkPAP(
                (*pap).fun,
                &raw mut (*pap).payload as *mut *mut StgClosure,
                (*pap).n_args as StgWord,
            );

            return pap_sizeW(pap);
        }
        26 => {
            let mut ap_0 = p as *mut StgAP_STACK;

            if LOOKS_LIKE_CLOSURE_PTR((*ap_0).fun as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 515);
            }

            checkStackChunk(
                &raw mut (*ap_0).payload as *mut *mut StgClosure as StgPtr,
                (&raw mut (*ap_0).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap_0).size as isize),
            );

            return ap_stack_sizeW(ap_0);
        }
        42 => return arr_words_sizeW(p as *mut StgArrBytes),
        43 | 44 | 46 | 45 => {
            let mut a = p as *mut StgMutArrPtrs;
            let mut i_1: u32 = 0;
            i_1 = 0;

            while (i_1 as StgWord) < (*a).ptrs {
                if LOOKS_LIKE_CLOSURE_PTR(
                    *(&raw mut (*a).payload as *mut *mut StgClosure).offset(i_1 as isize)
                        as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 531);
                }

                i_1 = i_1.wrapping_add(1);
            }

            return mut_arr_ptrs_sizeW(a);
        }
        59 | 60 | 62 | 61 => {
            let mut a_0 = p as *mut StgSmallMutArrPtrs;
            let mut i_2: u32 = 0;

            while (i_2 as StgWord) < (*a_0).ptrs {
                if LOOKS_LIKE_CLOSURE_PTR(
                    *(&raw mut (*a_0).payload as *mut *mut StgClosure).offset(i_2 as isize)
                        as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 543);
                }

                i_2 = i_2.wrapping_add(1);
            }

            return small_mut_arr_ptrs_sizeW(a_0);
        }
        52 => {
            checkTSO(p as *mut StgTSO);

            return (size_of::<StgTSO>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgOffset;
        }
        53 => {
            checkSTACK(p as *mut StgStack);

            return stack_sizeW(p as *mut StgStack) as StgOffset;
        }
        54 => {
            let mut i_3: u32 = 0;
            let mut tc = p as *mut StgTRecChunk;

            if LOOKS_LIKE_CLOSURE_PTR((*tc).prev_chunk as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 560);
            }

            i_3 = 0;

            while (i_3 as StgWord) < (*tc).next_entry_idx {
                if LOOKS_LIKE_CLOSURE_PTR((*tc).entries[i_3 as usize].tvar as *const c_void) as i32
                    as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 562);
                }

                if LOOKS_LIKE_CLOSURE_PTR(
                    (*tc).entries[i_3 as usize].expected_value as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 563);
                }

                if LOOKS_LIKE_CLOSURE_PTR((*tc).entries[i_3 as usize].new_value as *const c_void)
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 564);
                }

                i_3 = i_3.wrapping_add(1);
            }

            return (size_of::<StgTRecChunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as StgOffset;
        }
        64 => {
            let mut cont = p as *mut StgContinuation;

            if !(*cont).apply_mask_frame.is_null() {
                if ((*cont).apply_mask_frame == &raw const stg_unmaskAsyncExceptionszh_ret_info
                    || (*cont).apply_mask_frame == &raw const stg_maskAsyncExceptionszh_ret_info
                    || (*cont).apply_mask_frame == &raw const stg_maskUninterruptiblezh_ret_info)
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 575);
                }

                if LOOKS_LIKE_CLOSURE_PTR(
                    (&raw mut (*cont).stack as *mut StgWord)
                        .offset((*cont).mask_frame_offset as isize)
                        as *const c_void,
                ) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 576);
                }
            }

            checkStackChunk(
                &raw mut (*cont).stack as StgPtr,
                (&raw mut (*cont).stack as *mut StgWord).offset((*cont).stack_size as isize),
            );

            return continuation_sizeW(cont) as StgOffset;
        }
        _ => {
            barf(c"checkClosure (closure type %d)".as_ptr(), (*info).r#type);
        }
    };
}

unsafe fn checkHeapChain(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if (*bd).flags as i32 & BF_SWEPT == 0 {
            let mut p = (*bd).start;

            while p < (*bd).c2rust_unnamed.free {
                let mut size: u32 = checkClosure(p as *mut StgClosure) as u32;

                if (size as usize
                    >= (1 as usize).wrapping_add(
                        (size_of::<StgHeader>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize),
                    )) as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 605);
                }

                p = p.offset(size as isize);

                while p < (*bd).c2rust_unnamed.free && (*p < 0x1000 || !LOOKS_LIKE_INFO_PTR(*p)) {
                    p = p.offset(1);
                }
            }
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn checkNonmovingSegments(mut seg: *mut NonmovingSegment) {
    while !seg.is_null() {
        let count: nonmoving_block_idx = nonmovingSegmentBlockCount(seg) as nonmoving_block_idx;

        let mut i: nonmoving_block_idx = 0;

        while (i as i32) < count as i32 {
            if *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) as i32
                == nonmovingMarkEpoch as i32
            {
                let mut p = nonmovingSegmentGetBlock(seg, i) as StgPtr;
                checkClosure(p as *mut StgClosure);
            } else if (i as i32) < (*nonmovingSegmentInfo(seg)).next_free_snap as i32 {
                *(&raw mut (*seg).bitmap as *mut u8).offset(i as isize) = 0;
            }

            i = i.wrapping_add(1);
        }

        seg = (*seg).link;
    }
}

unsafe fn checkNonmovingHeap(mut heap: *const NonmovingHeap) {
    checkLargeObjects(nonmoving_large_objects);
    checkLargeObjects(nonmoving_marked_large_objects);
    checkCompactObjects(nonmoving_compact_objects);

    let mut i = 0;

    while i < nonmoving_alloca_cnt as u32 {
        let mut alloc: *const NonmovingAllocator =
            (*heap).allocators.offset(i as isize) as *mut NonmovingAllocator;
        checkNonmovingSegments((*alloc).filled);
        checkNonmovingSegments((*alloc).saved_filled);
        checkNonmovingSegments((*alloc).active);

        let mut cap_n = 0;

        while cap_n < getNumCapabilities() {
            let mut cap = getCapability(cap_n as u32);
            checkNonmovingSegments(*(*cap).current_segments.offset(i as isize));
            cap_n = cap_n.wrapping_add(1);
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn checkHeapChunk(mut start: StgPtr, mut end: StgPtr) {
    let mut p = null_mut::<StgWord>();
    let mut size: u32 = 0;
    p = start;

    while p < end {
        if LOOKS_LIKE_INFO_PTR(*p) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 663);
        }

        size = checkClosure(p as *mut StgClosure) as u32;

        if (size as usize
            >= (1 as usize).wrapping_add(
                (size_of::<StgHeader>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize),
            )) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 666);
        }

        p = p.offset(size as isize);
    }
}

unsafe fn checkLargeObjects(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if (*bd).flags as i32 & BF_PINNED == 0 {
            checkClosure((*bd).start as *mut StgClosure);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn checkCompactObjects(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if ((*bd).flags as i32 & 512 != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 688);
        }

        let mut block = (*bd).start as *mut StgCompactNFDataBlock;
        let mut str = (*block).owner as *mut StgCompactNFData;

        if (str as W_ == (block as W_).wrapping_add(size_of::<StgCompactNFDataBlock>() as W_))
            as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 692);
        }

        let mut totalW: StgWord = 0;
        let mut last = null_mut::<StgCompactNFDataBlock>();

        while !block.is_null() {
            last = block;

            if ((*block).owner == str) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 698);
            }

            totalW = totalW.wrapping_add(
                ((*Bdescr(block as StgPtr)).blocks as usize).wrapping_mul(BLOCK_SIZE_W) as StgWord,
            );

            let mut start = (*Bdescr(block as StgPtr)).start.offset(
                (size_of::<StgCompactNFDataBlock>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize,
            );

            let mut free = null_mut::<StgWord>();

            if (*Bdescr(block as StgPtr)).start == (*str).nursery as P_ {
                free = (*str).hp;
            } else {
                free = (*Bdescr(block as StgPtr)).c2rust_unnamed.free;
            }

            let mut p = start;

            while p < free {
                let mut c = p as *mut StgClosure;
                checkClosureShallow(c);
                p = p.offset(closure_sizeW(c) as isize);
            }

            block = (*block).next as *mut StgCompactNFDataBlock;
        }

        if ((*str).totalW == totalW) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 722);
        }

        if ((*str).last == last) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 723);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn checkSTACK(mut stack: *mut StgStack) {
    let mut sp = (*stack).sp;
    let mut stack_size: StgOffset = (*stack).stack_size as StgOffset;
    let mut stack_end = (&raw mut (*stack).stack as *mut StgWord).offset(stack_size as isize);

    if (&raw mut (*stack).stack as *mut StgWord <= sp && sp <= stack_end) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 734);
    }

    checkStackChunk(sp, stack_end);
}

unsafe fn checkTSO(mut tso: *mut StgTSO) {
    let mut info = (*(&raw mut (*tso)._link).load(Ordering::Acquire))
        .header
        .info;

    if ((*tso)._link == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
        || info == &raw const stg_MVAR_TSO_QUEUE_info
        || info == &raw const stg_TSO_info
        || info == &raw const stg_WHITEHOLE_info) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 779);
    }

    if (*tso).why_blocked == BlockedOnMVar as StgWord32
        || (*tso).why_blocked == BlockedOnMVarRead as StgWord32
        || (*tso).why_blocked == BlockedOnBlackHole as StgWord32
        || (*tso).why_blocked == BlockedOnMsgThrowTo as StgWord32
        || (*tso).why_blocked == NotBlocked as StgWord32
    {
        if LOOKS_LIKE_CLOSURE_PTR((*tso).block_info.closure as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 788);
        }
    }

    if LOOKS_LIKE_CLOSURE_PTR((*tso).bq as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 791);
    }

    if LOOKS_LIKE_CLOSURE_PTR((*tso).blocked_exceptions as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 792);
    }

    if LOOKS_LIKE_CLOSURE_PTR((*tso).stackobj as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 793);
    }

    if !(*tso).label.is_null() {
        if LOOKS_LIKE_CLOSURE_PTR((*tso).label as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 802);
        }
    }
}

unsafe fn checkGlobalTSOList(mut checkTSOs: bool) {
    let mut g: u32 = 0;

    while g < RtsFlags.GcFlags.generations {
        let mut tso = (*generations.offset(g as isize)).threads;

        while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            if LOOKS_LIKE_CLOSURE_PTR(tso as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 816);
            }

            if ((*get_itbl(tso as *mut StgClosure)).r#type == 52) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 817);
            }

            if checkTSOs {
                checkTSO(tso);
            }

            if (*tso).dirty != 0 {
                if ((*Bdescr(tso as StgPtr)).gen_no as i32 == 0 || (*tso).flags & 64 != 0) as i32
                    as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 825);
                }

                (*tso).flags &= !TSO_MARKED as StgWord32;
            }

            let mut stack = (*tso).stackobj as *mut StgStack;

            loop {
                if (*stack).dirty as i32 & STACK_DIRTY != 0 {
                    if ((*Bdescr(stack as StgPtr)).gen_no as i32 == 0
                        || (*stack).dirty as i32 & 64 != 0) as i32 as i64
                        != 0
                    {
                    } else {
                        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 832);
                    }

                    (*stack).dirty = ((*stack).dirty as i32 & !STACK_SANE) as StgWord8;
                }

                let mut frame = (&raw mut (*stack).stack as *mut StgWord)
                    .offset((*stack).stack_size as isize)
                    .offset(
                        -((size_of::<StgUnderflowFrame>() as usize)
                            .wrapping_add(size_of::<W_>() as usize)
                            .wrapping_sub(1 as usize)
                            .wrapping_div(size_of::<W_>() as usize)
                            as isize),
                    ) as *mut StgUnderflowFrame;

                if (*frame).next_chunk
                    == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
                        as *mut StgStack
                    || (*frame).info != &raw const stg_stack_underflow_frame_d_info
                        && (*frame).info != &raw const stg_stack_underflow_frame_v16_info
                        && (*frame).info != &raw const stg_stack_underflow_frame_v32_info
                        && (*frame).info != &raw const stg_stack_underflow_frame_v64_info
                {
                    break;
                }

                stack = (*frame).next_chunk as *mut StgStack;
            }

            tso = (*tso).global_link as *mut StgTSO;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn checkMutableList(mut mut_bd: *mut bdescr, mut r#gen: u32) {
    let mut bd = null_mut::<bdescr>();
    let mut q = null_mut::<StgWord>();
    let mut p = null_mut::<StgClosure>();
    bd = mut_bd;

    while !bd.is_null() {
        q = (*bd).start;

        while q < (*bd).c2rust_unnamed.free {
            p = *q as *mut StgClosure;

            if (!(p as W_ >= mblock_address_space.0.begin
                && (p as W_) < mblock_address_space.0.end)
                || (*Bdescr(p as StgPtr)).gen_no as u32 == r#gen) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Sanity.c".as_ptr(), 866);
            }

            checkClosure(p);

            match (*get_itbl(p)).r#type {
                52 => {
                    (*(p as *mut StgTSO)).flags |= TSO_MARKED as StgWord32;
                }
                53 => {
                    let ref mut fresh13 = (*(p as *mut StgStack)).dirty;
                    *fresh13 = (*fresh13 as i32 | STACK_SANE) as StgWord8;
                }
                _ => {}
            }

            q = q.offset(1);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn checkLocalMutableLists(mut cap_no: u32) {
    let mut g: u32 = 0;
    g = 1;

    while g < RtsFlags.GcFlags.generations {
        checkMutableList(*(*getCapability(cap_no)).mut_lists.offset(g as isize), g);
        g = g.wrapping_add(1);
    }
}

unsafe fn checkMutableLists() {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        checkLocalMutableLists(i);
        i = i.wrapping_add(1);
    }
}

unsafe fn checkStaticObjects(mut static_objects: *mut StgClosure) {
    let mut p = static_objects;
    let mut info = null::<StgInfoTable>();

    while p != static_flag as StgWord as *mut StgClosure {
        p = (p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;
        checkClosure(p);
        info = get_itbl(p);

        match (*info).r#type {
            28 => {
                let mut indirectee = null::<StgClosure>();
                indirectee = UNTAG_CONST_CLOSURE((*(p as *mut StgIndStatic)).indirectee);

                if LOOKS_LIKE_CLOSURE_PTR(indirectee as *const c_void) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 918);
                }

                if LOOKS_LIKE_INFO_PTR((*indirectee).header.info as StgWord) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/Sanity.c".as_ptr(), 919);
                }

                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            21 => {
                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            14 => {
                p = *STATIC_LINK(info, p);
            }
            1 | 7 | 2 | 4 | 5 => {
                p = *STATIC_LINK(info, p);
            }
            _ => {
                barf(
                    c"checkStaticObjetcs: strange closure %p (%s)".as_ptr(),
                    p,
                    info_type(p),
                );
            }
        }
    }
}

unsafe fn checkNurserySanity(mut nursery: *mut nursery) {
    let mut bd = null_mut::<bdescr>();
    let mut prev = null_mut::<bdescr>();
    let mut blocks: u32 = 0;
    prev = null_mut::<bdescr>();
    bd = (*nursery).blocks;

    while !bd.is_null() {
        if ((*bd).r#gen == g0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 956);
        }

        if ((*bd).u.back == prev) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 957);
        }

        prev = bd;
        blocks = blocks.wrapping_add((*bd).blocks as u32);
        bd = (*bd).link as *mut bdescr;
    }

    if (blocks as memcount == (*nursery).n_blocks) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 962);
    };
}

unsafe fn checkGeneration(mut r#gen: *mut generation, mut after_major_gc: bool) {
    let mut n: u32 = 0;
    let mut ws = null_mut::<gen_workspace>();

    if !isNonmovingGen(r#gen) {
        if (countBlocks((*r#gen).blocks) == (*r#gen).n_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 975);
        }
    }

    if (countBlocks((*r#gen).large_objects) == (*r#gen).n_large_blocks) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 977);
    }

    if !after_major_gc {
        return;
    }

    if isNonmovingGen(r#gen) {
        if (countBlocks(nonmoving_large_objects) == n_nonmoving_large_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 994);
        }

        if (countBlocks(nonmoving_marked_large_objects) == n_nonmoving_marked_large_blocks) as i32
            as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 995);
        }

        let mut counted_cnf_blocks: u32 = 0;
        counted_cnf_blocks = (counted_cnf_blocks as StgWord)
            .wrapping_add(countCompactBlocks(nonmoving_marked_compact_objects))
            as u32 as u32;
        counted_cnf_blocks = (counted_cnf_blocks as StgWord)
            .wrapping_add(countCompactBlocks(nonmoving_compact_objects))
            as u32 as u32;
        counted_cnf_blocks = (counted_cnf_blocks as StgWord)
            .wrapping_add(countCompactBlocks((*oldest_gen).compact_objects))
            as u32 as u32;

        let mut total_cnf_blocks: u32 = 0;
        total_cnf_blocks = (total_cnf_blocks as memcount)
            .wrapping_add(n_nonmoving_compact_blocks.wrapping_add((*oldest_gen).n_compact_blocks))
            as u32 as u32;

        total_cnf_blocks = (total_cnf_blocks as memcount)
            .wrapping_add(n_nonmoving_marked_compact_blocks) as u32
            as u32;

        if (counted_cnf_blocks == total_cnf_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1012);
        }
    }

    checkHeapChain((*r#gen).blocks);
    n = 0;

    while n < getNumCapabilities() as u32 {
        ws = (&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
            .offset((*r#gen).no as isize) as *mut gen_workspace;
        checkHeapChain((*ws).0.todo_bd);
        checkHeapChain((*ws).0.part_list);
        checkHeapChain((*ws).0.scavd_list);
        n = n.wrapping_add(1);
    }

    let mut g: u32 = 0;

    while g < RtsFlags.GcFlags.generations {
        checkGenWeakPtrList(g);
        g = g.wrapping_add(1);
    }

    checkLargeObjects((*r#gen).large_objects);
    checkCompactObjects((*r#gen).compact_objects);
}

unsafe fn checkFullHeap(mut after_major_gc: bool) {
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        checkGeneration(
            generations.offset(g as isize) as *mut generation,
            after_major_gc,
        );

        g = g.wrapping_add(1);
    }

    n = 0;

    while n < getNumCapabilities() as u32 {
        checkNurserySanity(nurseries.offset(n as isize) as *mut nursery);
        n = n.wrapping_add(1);
    }
}

unsafe fn checkSanity(mut after_gc: bool, mut major_gc: bool) {
    checkFullHeap(after_gc as i32 != 0 && major_gc as i32 != 0);
    checkFreeListSanity();

    if after_gc {
        checkMutableLists();
        checkGlobalTSOList(true);
    }
}

unsafe fn markCompactBlocks(mut bd: *mut bdescr) {
    while !bd.is_null() {
        compactMarkKnown(
            (*((*bd).start as *mut StgCompactNFDataBlock)).owner as *mut StgCompactNFData,
        );

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn markNonMovingSegments(mut seg: *mut NonmovingSegment) {
    while !seg.is_null() {
        markBlocks(Bdescr(seg as StgPtr));
        seg = (*seg).link;
    }
}

unsafe fn findMemoryLeak() {
    let mut g: u32 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        i = 0;

        while i < getNumCapabilities() as u32 {
            markBlocks(*(*getCapability(i)).mut_lists.offset(g as isize));

            markBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .part_list,
            );

            markBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .scavd_list,
            );

            markBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_bd,
            );

            i = i.wrapping_add(1);
        }

        markBlocks((*generations.offset(g as isize)).blocks);
        markBlocks((*generations.offset(g as isize)).large_objects);
        markCompactBlocks((*generations.offset(g as isize)).compact_objects);
        g = g.wrapping_add(1);
    }

    i = 0;

    while i < n_nurseries {
        markBlocks((*nurseries.offset(i as isize)).blocks);
        i = i.wrapping_add(1);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        markBlocks((**gc_threads.offset(i as isize)).free_blocks);
        markBlocks((*getCapability(i)).pinned_object_block);
        markBlocks((*getCapability(i)).pinned_object_blocks);
        markBlocks((*getCapability(i)).upd_rem_set.queue.blocks);
        i = i.wrapping_add(1);
    }

    if RtsFlags.GcFlags.useNonmoving {
        markBlocks(upd_rem_set_block_list);
        markBlocks(nonmoving_large_objects);
        markBlocks(nonmoving_marked_large_objects);
        markBlocks(nonmoving_compact_objects);
        markBlocks(nonmoving_marked_compact_objects);
        i = 0;

        while i < nonmoving_alloca_cnt as u32 {
            let mut alloc: *mut NonmovingAllocator =
                nonmovingHeap.allocators.offset(i as isize) as *mut NonmovingAllocator;
            markNonMovingSegments((*alloc).filled);
            markNonMovingSegments((*alloc).saved_filled);
            markNonMovingSegments((*alloc).active);
            j = 0;

            while j < getNumCapabilities() as u32 {
                let mut cap = getCapability(j);
                markNonMovingSegments(*(*cap).current_segments.offset(i as isize));
                j = j.wrapping_add(1);
            }

            i = i.wrapping_add(1);
        }

        markNonMovingSegments(nonmovingHeap.sweep_list);
        markNonMovingSegments(nonmovingHeap.free);

        if !current_mark_queue.is_null() {
            markBlocks((*current_mark_queue).blocks);
        }
    }

    markBlocks(exec_block);
    reportUnmarkedBlocks();
}

unsafe fn checkRunQueue(mut cap: *mut Capability) {
    let mut prev = null_mut::<StgTSO>();
    let mut tso = null_mut::<StgTSO>();
    prev = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;

    let mut n: u32 = 0;
    n = 0;
    tso = (*cap).run_queue_hd;

    while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        if (prev == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            || (*prev)._link == tso) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1159);
        }

        if ((*tso).block_info.prev == prev) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1160);
        }

        prev = tso;
        tso = (*tso)._link as *mut StgTSO;
        n = n.wrapping_add(1);
    }

    if ((*cap).run_queue_tl == prev) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1162);
    }

    if ((*cap).n_run_queue == n) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1163);
    };
}

unsafe fn findSlop(mut bd: *mut bdescr) {
    let mut slop: W_ = 0;

    while !bd.is_null() {
        slop = ((*bd).blocks as usize)
            .wrapping_mul(BLOCK_SIZE_W)
            .wrapping_sub((*bd).c2rust_unnamed.free.offset_from((*bd).start) as i64 as usize)
            as W_;

        if slop > (1024 as usize).wrapping_div(size_of::<W_>() as usize) as W_ {
            debugBelch(
                c"block at %p (bdescr %p) has %lluKB slop\n".as_ptr(),
                (*bd).start,
                bd,
                slop.wrapping_div((1024 as W_).wrapping_div(size_of::<W_>() as W_)),
            );
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn genBlocks(mut r#gen: *mut generation) -> W_ {
    let mut ret: W_ = 0;

    if isNonmovingGen(r#gen) {
        if (countNonMovingHeap(&raw mut nonmovingHeap) == (*r#gen).n_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1195);
        }

        ret = ret.wrapping_add(countAllocdBlocks(nonmoving_large_objects));
        ret = ret.wrapping_add(countAllocdBlocks(nonmoving_marked_large_objects));
        ret = (ret as StgWord).wrapping_add(countAllocdCompactBlocks(nonmoving_compact_objects))
            as W_ as W_;
        ret = (ret as StgWord)
            .wrapping_add(countAllocdCompactBlocks(nonmoving_marked_compact_objects))
            as W_ as W_;
        ret = ret.wrapping_add(countNonMovingHeap(&raw mut nonmovingHeap));

        if !current_mark_queue.is_null() {
            ret = ret.wrapping_add(countBlocks((*current_mark_queue).blocks));
        }
    } else {
        if (countBlocks((*r#gen).blocks) == (*r#gen).n_blocks) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1204);
        }

        if (countCompactBlocks((*r#gen).compact_objects) == (*r#gen).n_compact_blocks) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1205);
        }

        if (countCompactBlocks((*r#gen).compact_blocks_in_import)
            == (*r#gen).n_compact_blocks_in_import) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1206);
        }

        ret = (ret as StgWord).wrapping_add((*r#gen).n_blocks as StgWord) as W_ as W_;
    }

    if (countBlocks((*r#gen).large_objects) == (*r#gen).n_large_blocks) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1210);
    }

    ret = (ret as StgWord).wrapping_add(
        (*r#gen)
            .n_old_blocks
            .wrapping_add(countAllocdBlocks((*r#gen).large_objects) as memcount)
            .wrapping_add(countAllocdCompactBlocks((*r#gen).compact_objects) as memcount)
            .wrapping_add(countAllocdCompactBlocks((*r#gen).compact_blocks_in_import) as memcount)
            as StgWord,
    ) as W_ as W_;

    return ret;
}

unsafe fn countNonMovingSegments(mut segs: *mut NonmovingSegment) -> W_ {
    let mut ret: W_ = 0;

    while !segs.is_null() {
        ret = ret.wrapping_add(countBlocks(Bdescr(segs as StgPtr)));
        segs = (*segs).link;
    }

    return ret;
}

unsafe fn countNonMovingHeap(mut heap: *mut NonmovingHeap) -> W_ {
    let mut ret: W_ = 0;
    let mut alloc_idx = 0;

    while alloc_idx < nonmoving_alloca_cnt as i32 {
        let mut alloc: *mut NonmovingAllocator =
            (*heap).allocators.offset(alloc_idx as isize) as *mut NonmovingAllocator;
        ret = ret.wrapping_add(countNonMovingSegments((*alloc).filled));
        ret = ret.wrapping_add(countNonMovingSegments((*alloc).saved_filled));
        ret = ret.wrapping_add(countNonMovingSegments((*alloc).active));

        let mut c: u32 = 0;

        while c < getNumCapabilities() as u32 {
            let mut cap = getCapability(c);
            ret = ret.wrapping_add(countNonMovingSegments(
                *(*cap).current_segments.offset(alloc_idx as isize),
            ));

            c = c.wrapping_add(1);
        }

        alloc_idx += 1;
    }

    ret = ret.wrapping_add(countNonMovingSegments((*heap).sweep_list));
    ret = ret.wrapping_add(countNonMovingSegments((*heap).free));
    ret = ret.wrapping_add(countNonMovingSegments((*heap).saved_free));

    return ret;
}

unsafe fn memInventory(mut show: bool) {
    let mut g: u32 = 0;
    let mut i: u32 = 0;
    let vla = RtsFlags.GcFlags.generations as usize;
    let mut gen_blocks: Vec<W_> = ::std::vec::from_elem(0, vla);
    let mut nursery_blocks: W_ = 0;
    let mut free_pinned_blocks: W_ = 0;
    let mut retainer_blocks: W_ = 0;
    let mut arena_blocks: W_ = 0;
    let mut exec_blocks: W_ = 0;
    let mut gc_free_blocks: W_ = 0;
    let mut upd_rem_set_blocks: W_ = 0;
    let mut live_blocks: W_ = 0;
    let mut free_blocks: W_ = 0;
    let mut leak: bool = false;

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && !nonmovingBlockConcurrentMark(false) {
        return;
    }

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        *gen_blocks.as_mut_ptr().offset(g as isize) = 0;
        i = 0;

        while i < getNumCapabilities() as u32 {
            let ref mut fresh14 = *gen_blocks.as_mut_ptr().offset(g as isize);
            *fresh14 = (*fresh14).wrapping_add(countBlocks(
                *(*getCapability(i)).mut_lists.offset(g as isize),
            ));

            let ref mut fresh15 = *gen_blocks.as_mut_ptr().offset(g as isize);
            *fresh15 = (*fresh15).wrapping_add(countBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .part_list,
            ));

            let ref mut fresh16 = *gen_blocks.as_mut_ptr().offset(g as isize);
            *fresh16 = (*fresh16).wrapping_add(countBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .scavd_list,
            ));

            let ref mut fresh17 = *gen_blocks.as_mut_ptr().offset(g as isize);
            *fresh17 = (*fresh17).wrapping_add(countBlocks(
                (*(&raw mut (**gc_threads.offset(i as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_bd,
            ));

            i = i.wrapping_add(1);
        }

        let ref mut fresh18 = *gen_blocks.as_mut_ptr().offset(g as isize);
        *fresh18 =
            (*fresh18).wrapping_add(genBlocks(generations.offset(g as isize) as *mut generation));
        g = g.wrapping_add(1);
    }

    i = 0;

    while i < n_nurseries {
        if (countBlocks((*nurseries.offset(i as isize)).blocks)
            == (*nurseries.offset(i as isize)).n_blocks) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1283);
        }

        nursery_blocks = (nursery_blocks as StgWord)
            .wrapping_add((*nurseries.offset(i as isize)).n_blocks as StgWord)
            as W_ as W_;
        i = i.wrapping_add(1);
    }

    i = 0;

    while i < getNumCapabilities() as u32 {
        let mut n = countBlocks((**gc_threads.offset(i as isize)).free_blocks);
        gc_free_blocks = gc_free_blocks.wrapping_add(n);

        if !(*getCapability(i)).pinned_object_block.is_null() {
            nursery_blocks = nursery_blocks
                .wrapping_add((*(*getCapability(i)).pinned_object_block).blocks as W_);
        }

        nursery_blocks =
            nursery_blocks.wrapping_add(countBlocks((*getCapability(i)).pinned_object_blocks));
        free_pinned_blocks =
            free_pinned_blocks.wrapping_add(countBlocks((*getCapability(i)).pinned_object_empty));
        i = i.wrapping_add(1);
    }

    if RtsFlags.ProfFlags.doHeapProfile == HEAP_BY_RETAINER as u32 {
        retainer_blocks = retainerStackBlocks();
    }

    arena_blocks = arenaBlocks() as W_;
    exec_blocks = countAllocdBlocks(exec_block);
    free_blocks = countFreeList();
    i = 0;

    while i < getNumCapabilities() as u32 {
        upd_rem_set_blocks = upd_rem_set_blocks
            .wrapping_add(countBlocks((*getCapability(i)).upd_rem_set.queue.blocks));
        i = i.wrapping_add(1);
    }

    upd_rem_set_blocks = upd_rem_set_blocks.wrapping_add(countBlocks(upd_rem_set_block_list));
    live_blocks = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        live_blocks = live_blocks.wrapping_add(*gen_blocks.as_mut_ptr().offset(g as isize));
        g = g.wrapping_add(1);
    }

    live_blocks = live_blocks.wrapping_add(
        nursery_blocks
            .wrapping_add(retainer_blocks)
            .wrapping_add(arena_blocks)
            .wrapping_add(exec_blocks)
            .wrapping_add(gc_free_blocks)
            .wrapping_add(upd_rem_set_blocks)
            .wrapping_add(free_pinned_blocks),
    );

    leak =
        live_blocks.wrapping_add(free_blocks) != mblocks_allocated.wrapping_mul(BLOCKS_PER_MBLOCK);

    if show as i32 != 0 || leak as i32 != 0 {
        if leak {
            debugBelch(c"Memory leak detected:\n".as_ptr());
        } else {
            debugBelch(c"Memory inventory:\n".as_ptr());
        }

        g = 0;

        while g < RtsFlags.GcFlags.generations {
            debugBelch(
                c"  gen %d blocks : %5llu blocks (%6.1lf MB)\n".as_ptr(),
                g,
                *gen_blocks.as_mut_ptr().offset(g as isize),
                *gen_blocks.as_mut_ptr().offset(g as isize) as f64 * BLOCK_SIZE_W as f64
                    / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                        as f64,
            );

            g = g.wrapping_add(1);
        }

        debugBelch(
            c"  nursery      : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            nursery_blocks,
            nursery_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  empty pinned : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            free_pinned_blocks,
            free_pinned_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  retainer     : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            retainer_blocks,
            retainer_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  arena blocks : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            arena_blocks,
            arena_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  exec         : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            exec_blocks,
            exec_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  GC free pool : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            gc_free_blocks,
            gc_free_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  free         : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            free_blocks,
            free_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  UpdRemSet    : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            upd_rem_set_blocks,
            upd_rem_set_blocks as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        debugBelch(
            c"  total        : %5llu blocks (%6.1lf MB)\n".as_ptr(),
            live_blocks.wrapping_add(free_blocks),
            live_blocks.wrapping_add(free_blocks) as f64 * BLOCK_SIZE_W as f64
                / ((1024 as i32 * 1024 as i32) as usize).wrapping_div(size_of::<W_>() as usize)
                    as f64,
        );

        if leak {
            debugBelch(
                c"\n  in system    : %5llu blocks (%llu MB)\n".as_ptr(),
                mblocks_allocated.wrapping_mul(BLOCKS_PER_MBLOCK),
                mblocks_allocated,
            );
        }
    }

    if leak {
        debugBelch(c"\n".as_ptr());
        findMemoryLeak();
    }

    if (n_alloc_blocks == live_blocks) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1368);
    }

    if !leak as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Sanity.c".as_ptr(), 1369);
    }

    nonmovingUnblockConcurrentMark();
}
