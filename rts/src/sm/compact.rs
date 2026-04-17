use crate::capability::{getCapability, markCapabilities};
use crate::ffi::mach_deps::TAG_MASK;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::block::{
    BF_MARKED, BF_PINNED, BLOCK_SIZE_W, Bdescr, bdescr, freeChain,
};
use crate::ffi::rts::storage::closure_macros::{
    FUN_INFO_PTR_TO_STRUCT, GET_CLOSURE_TAG, GET_INFO, INFO_PTR_TO_STRUCT, LOOKS_LIKE_CLOSURE_PTR,
    LOOKS_LIKE_INFO_PTR, STATIC_LINK, THUNK_SELECTOR_sizeW, UNTAG_CLOSURE, arr_words_sizeW,
    bco_sizeW, closure_sizeW_, continuation_sizeW, get_itbl, get_ret_itbl, mut_arr_ptrs_sizeW,
    small_mut_arr_ptrs_sizeW, stack_sizeW,
};
use crate::ffi::rts::storage::closure_types::STACK;
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgClosure_, StgCompactNFData, StgCompactNFData_,
    StgCompactNFDataBlock, StgContinuation, StgInd, StgMVar, StgMutArrPtrs, StgPAP, StgRetFun,
    StgSelector, StgSmallMutArrPtrs, StgTRecChunk, StgThunk, StgWeak, TRecEntry, hashtable,
};
use crate::ffi::rts::storage::gc::{generation, generations, memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::info_tables::{
    StgFunInfoTable, StgLargeBitmap, StgSRTField, stg_arg_bitmaps,
};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgWord};
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::ffi::stg::{BITS_PER_BYTE, P_, W_};
use crate::ffi::stg::{BITS_PER_BYTE, W_};
use crate::hash::{
    HashTable, allocHashTable, freeHashTable, insertHashTable, mapHashTable, mapHashTableKeys,
};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::sm::gc::markCAFs;
use crate::sm::gc_thread::{gc_threads, gen_workspace};
use crate::sm::storage::{STATIC_BITS, move_STACK, static_flag};
use crate::stable_name::threadStableNameTable;
use crate::stable_ptr::threadStablePtrTable;
use crate::task::{InCall, Task, all_tasks};
use crate::trace::{DEBUG_RTS, trace_};

#[inline]
pub(crate) unsafe fn mark(mut p: StgPtr, mut bd: *mut bdescr) {
    let mut offset_within_block: u32 = p.offset_from((*bd).start) as i64 as u32;
    let mut bitmap_word = (*bd).u.bitmap.offset(
        (offset_within_block as usize)
            .wrapping_div((BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize))
            as isize,
    );

    let mut bit_mask: StgWord = 1
        << (offset_within_block as usize
            & (BITS_PER_BYTE as usize)
                .wrapping_mul(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize));
    *bitmap_word |= bit_mask;
}

#[inline]
pub(crate) unsafe fn is_marked(mut p: StgPtr, mut bd: *mut bdescr) -> StgWord {
    let mut offset_within_block: u32 = p.offset_from((*bd).start) as i64 as u32;
    let mut bitmap_word = (*bd).u.bitmap.offset(
        (offset_within_block as usize)
            .wrapping_div((BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize))
            as isize,
    );

    let mut bit_mask: StgWord = 1
        << (offset_within_block as usize
            & (BITS_PER_BYTE as usize)
                .wrapping_mul(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize));

    return *bitmap_word & bit_mask;
}

unsafe fn UNTAG_PTR(mut p: W_) -> W_ {
    return p & !TAG_MASK as W_;
}

unsafe fn GET_PTR_TAG(mut p: W_) -> W_ {
    return p & TAG_MASK as W_;
}

unsafe fn get_iptr_tag(mut iptr: *mut StgInfoTable) -> W_ {
    let mut info: *const StgInfoTable = INFO_PTR_TO_STRUCT(iptr);

    match (*info).r#type {
        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            let mut con_tag: W_ = ((*info).srt + 1) as W_;

            if con_tag > TAG_MASK as W_ {
                return TAG_MASK as W_;
            } else {
                return con_tag;
            }
        }
        8 | 9 | 10 | 11 | 12 | 13 | 14 => {
            let mut fun_itbl: *const StgFunInfoTable = FUN_INFO_PTR_TO_STRUCT(iptr);
            let mut arity: W_ = (*fun_itbl).f.arity as W_;

            if arity <= TAG_MASK as W_ {
                return arity;
            } else {
                return 0;
            }
        }
        _ => return 0,
    };
}

unsafe fn thread(mut p: *mut *mut StgClosure) {
    let mut q0 = *p;
    let mut q0_tagged = GET_CLOSURE_TAG(q0) != 0;
    let mut q = UNTAG_CLOSURE(q0) as P_;

    if q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end {
        let mut bd = Bdescr(q as StgPtr);

        if (*bd).flags as i32 & BF_MARKED != 0 {
            let mut iptr: W_ = *q;
            *p = iptr as *mut StgClosure;
            *q = (p as W_).wrapping_add(1 as W_).wrapping_add(
                (if q0_tagged as i32 != 0 {
                    1 as i32
                } else {
                    0 as i32
                }) as W_,
            ) as StgWord;
        }
    }
}

unsafe fn thread_root(mut user: *mut c_void, mut p: *mut *mut StgClosure) {
    thread(p);
}

unsafe fn thread_(mut p: *mut c_void) {
    thread(p as *mut *mut StgClosure);
}

unsafe fn unthread(p: P_, mut free: W_, mut tag: W_) {
    let mut q: W_ = *p;

    loop {
        match GET_PTR_TAG(q) {
            0 => {
                *p = q as StgWord;
                return;
            }
            1 => {
                let mut q0 = q.wrapping_sub(1 as W_) as P_;
                let mut r: W_ = *q0;
                *q0 = free as StgWord;
                q = r;
            }
            2 => {
                let mut q0_0 = q.wrapping_sub(2 as W_) as P_;
                let mut r_0: W_ = *q0_0;
                *q0_0 = free.wrapping_add(tag) as StgWord;
                q = r_0;
            }
            _ => {
                barf(c"unthread".as_ptr());
            }
        }
    }
}

unsafe fn get_threaded_info(mut p: P_) -> *mut StgInfoTable {
    let mut q: W_ = GET_INFO(UNTAG_CLOSURE(p as *mut StgClosure)) as W_;

    loop {
        match GET_PTR_TAG(q) {
            0 => {
                if LOOKS_LIKE_INFO_PTR(q as StgWord) as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/Compact.c".as_ptr(), 208);
                }

                return q as *mut StgInfoTable;
            }
            1 | 2 => {
                q = *(UNTAG_PTR(q) as P_) as W_;
            }
            _ => {
                barf(c"get_threaded_info".as_ptr());
            }
        }
    }
}

unsafe fn r#move(mut to: P_, mut from: P_, mut size: W_) {
    while size > 0 {
        let fresh6 = from;
        from = from.offset(1);

        let fresh7 = to;
        to = to.offset(1);
        *fresh7 = *fresh6;
        size = size.wrapping_sub(1);
    }
}

unsafe fn thread_static(mut p: *mut StgClosure) {
    while p != static_flag as StgWord as *mut StgClosure {
        p = (p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;

        let mut info = get_itbl(p);

        match (*info).r#type {
            28 => {
                thread(&raw mut (*(p as *mut StgInd)).indirectee);
                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            21 => {
                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            14 => {
                p = *STATIC_LINK(info, p);
            }
            1 | 7 | 2 | 3 | 4 | 5 | 6 => {
                p = *STATIC_LINK(info, p);
            }
            _ => {
                barf(
                    c"thread_static: strange closure %d".as_ptr(),
                    (*info).r#type as i32,
                );
            }
        }
    }
}

unsafe fn thread_large_bitmap(mut p: P_, mut large_bitmap: *mut StgLargeBitmap, mut size: W_) {
    let mut b: W_ = 0;
    let mut bitmap: W_ =
        *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(b as isize) as W_;

    let mut i: W_ = 0;

    while i < size {
        if bitmap & 1 == 0 {
            thread(p as *mut *mut StgClosure);
        }

        i = i.wrapping_add(1);
        p = p.offset(1);

        if i.wrapping_rem((BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize) as W_)
            == 0
        {
            b = b.wrapping_add(1);
            bitmap = *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(b as isize) as W_;
        } else {
            bitmap = bitmap >> 1;
        }
    }
}

unsafe fn thread_small_bitmap(mut p: P_, mut size: W_, mut bitmap: W_) -> P_ {
    while size > 0 {
        if bitmap & 1 == 0 {
            thread(p as *mut *mut StgClosure);
        }

        p = p.offset(1);
        bitmap = bitmap >> 1;
        size = size.wrapping_sub(1);
    }

    return p;
}

unsafe fn thread_arg_block(
    mut fun_info: *mut StgFunInfoTable,
    mut args: *mut *mut StgClosure,
) -> P_ {
    let mut bitmap: W_ = 0;
    let mut size: W_ = 0;
    let mut p = args as P_;
    let mut current_block_7: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = ((*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT) as W_;
            size = ((*fun_info).f.b.bitmap & BITMAP_SIZE_MASK as StgWord) as W_;
            current_block_7 = 6145864612238493040;
        }
        1 => {
            size = (*((fun_info.offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                as *mut StgLargeBitmap))
                .size as W_;

            thread_large_bitmap(
                p,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_7 = 13183875560443969876;
        }
        _ => {
            bitmap = (*(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT) as W_;
            size = (*(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                & BITMAP_SIZE_MASK as StgWord) as W_;
            current_block_7 = 6145864612238493040;
        }
    }

    match current_block_7 {
        6145864612238493040 => {
            p = thread_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

unsafe fn thread_stack(mut p: P_, mut stack_end: P_) {
    let mut size_1: W_ = 0;

    while p < stack_end {
        let mut info = get_ret_itbl(p as *mut StgClosure);

        match (*info).i.r#type {
            56 | 57 | 55 | 33 | 35 | 36 | 34 | 30 | 65 => {
                let mut bitmap: W_ = (*info).i.layout.bitmap as W_ >> BITMAP_BITS_SHIFT;
                let mut size: W_ = (*info).i.layout.bitmap as W_ & BITMAP_SIZE_MASK as W_;
                p = p.offset(1);
                p = thread_small_bitmap(p, size, bitmap);
            }
            29 => {
                p = p.offset(1);

                let mut bco = *p as *mut StgBCO;
                thread(p as *mut *mut StgClosure);
                p = p.offset(1);

                let mut size_0: W_ =
                    (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size as W_;

                thread_large_bitmap(
                    p,
                    &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                    size_0,
                );

                p = p.offset(size_0 as isize);
            }
            31 => {
                p = p.offset(1);
                size_1 = (*(((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap))
                    .size as W_;

                thread_large_bitmap(
                    p,
                    ((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                        .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                        as *mut StgLargeBitmap,
                    size_1,
                );

                p = p.offset(size_1 as isize);
            }
            32 => {
                let mut ret_fun = p as *mut StgRetFun;

                let mut fun_info = FUN_INFO_PTR_TO_STRUCT(get_threaded_info((*ret_fun).fun as P_));

                thread(&raw mut (*ret_fun).fun);

                p = thread_arg_block(
                    fun_info,
                    &raw mut (*ret_fun).payload as *mut *mut StgClosure,
                );
            }
            _ => {
                barf(
                    c"thread_stack: weird activation record found on stack: %d".as_ptr(),
                    (*info).i.r#type as i32,
                );
            }
        }
    }
}

unsafe fn thread_PAP_payload(
    mut fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut size: W_,
) -> P_ {
    let mut fun_info = FUN_INFO_PTR_TO_STRUCT(get_threaded_info(fun as P_));

    if ((*fun_info).i.r#type != 25) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Compact.c".as_ptr(), 407);
    }

    let mut p = payload as P_;
    let mut bitmap: W_ = 0;
    let mut current_block_9: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = ((*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT) as W_;
            current_block_9 = 307863803066297713;
        }
        1 => {
            thread_large_bitmap(
                p,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_9 = 4166486009154926805;
        }
        2 => {
            thread_large_bitmap(
                payload as P_,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_9 = 4166486009154926805;
        }
        _ => {
            bitmap = (*(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT) as W_;
            current_block_9 = 307863803066297713;
        }
    }

    match current_block_9 {
        307863803066297713 => {
            p = thread_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

unsafe fn thread_PAP(mut pap: *mut StgPAP) -> P_ {
    let mut p = thread_PAP_payload(
        (*pap).fun,
        &raw mut (*pap).payload as *mut *mut StgClosure,
        (*pap).n_args as W_,
    );

    thread(&raw mut (*pap).fun);

    return p;
}

unsafe fn thread_AP(mut ap: *mut StgAP) -> P_ {
    let mut p = thread_PAP_payload(
        (*ap).fun,
        &raw mut (*ap).payload as *mut *mut StgClosure,
        (*ap).n_args as W_,
    );

    thread(&raw mut (*ap).fun);

    return p;
}

unsafe fn thread_AP_STACK(mut ap: *mut StgAP_STACK) -> P_ {
    thread(&raw mut (*ap).fun);

    thread_stack(
        &raw mut (*ap).payload as *mut *mut StgClosure as P_,
        (&raw mut (*ap).payload as *mut *mut StgClosure as P_).offset((*ap).size as isize),
    );

    return (ap as P_)
        .offset(
            (size_of::<StgAP_STACK>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as isize,
        )
        .offset((*ap).size as isize);
}

unsafe fn thread_continuation(mut cont: *mut StgContinuation) -> P_ {
    thread_stack(
        &raw mut (*cont).stack as P_,
        (&raw mut (*cont).stack as *mut StgWord).offset((*cont).stack_size as isize),
    );

    return (cont as P_).offset(continuation_sizeW(cont) as isize);
}

unsafe fn thread_TSO(mut tso: *mut StgTSO) -> P_ {
    thread_(&raw mut (*tso)._link as *mut c_void);
    thread_(&raw mut (*tso).global_link as *mut c_void);

    match (&raw mut (*tso).why_blocked).load(Ordering::Acquire) {
        1 | 14 | 2 | 12 | 0 => {
            thread_(&raw mut (*tso).block_info.closure as *mut c_void);
        }
        _ => {}
    }

    thread_(&raw mut (*tso).blocked_exceptions as *mut c_void);
    thread_(&raw mut (*tso).bq as *mut c_void);
    thread_(&raw mut (*tso).trec as *mut c_void);

    if !(*tso).label.is_null() {
        thread_(&raw mut (*tso).label as *mut *mut StgClosure as *mut c_void);
    }

    thread_(&raw mut (*tso).stackobj as *mut c_void);

    return (tso as P_).offset(
        (size_of::<StgTSO>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize,
    );
}

static mut nfdata_chain: *mut StgCompactNFData = null_mut::<StgCompactNFData>();

unsafe fn thread_nfdata_hash_key(
    mut data: *mut c_void,
    mut key: *mut StgWord,
    mut value: *const c_void,
) {
    thread_(key as *mut c_void);
}

unsafe fn add_hash_entry(mut data: *mut c_void, mut key: StgWord, mut value: *const c_void) {
    let mut new_hash = data as *mut HashTable;
    insertHashTable(new_hash, key, value);
}

unsafe fn rehash_CNFs() {
    while !nfdata_chain.is_null() {
        let mut str = nfdata_chain;
        nfdata_chain = (*str).link as *mut StgCompactNFData;
        (*str).link = null_mut::<StgCompactNFData_>();

        let mut new_hash = allocHashTable();

        mapHashTable(
            (*str).hash as *mut HashTable,
            new_hash as *mut c_void,
            Some(add_hash_entry as unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> ()),
        );

        freeHashTable((*str).hash as *mut HashTable, None);
        (*str).hash = new_hash as *mut hashtable;
    }
}

unsafe fn update_fwd_cnf(mut bd: *mut bdescr) {
    while !bd.is_null() {
        if ((*bd).flags as i32 & 512 != 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Compact.c".as_ptr(), 541);
        }

        let mut str = (*((*bd).start as *mut StgCompactNFDataBlock)).owner as *mut StgCompactNFData;

        if !(*str).hash.is_null() {
            mapHashTableKeys(
                (*str).hash as *mut HashTable,
                NULL,
                Some(
                    thread_nfdata_hash_key
                        as unsafe extern "C" fn(*mut c_void, *mut StgWord, *const c_void) -> (),
                ),
            );

            if (*str).link.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Compact.c".as_ptr(), 548);
            }

            (*str).link = nfdata_chain as *mut StgCompactNFData_;
            nfdata_chain = str;
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn update_fwd_large(mut bd: *mut bdescr) {
    let mut current_block_20: u64;

    while !bd.is_null() {
        if !((*bd).flags as i32 & BF_PINNED != 0) {
            let mut p = (*bd).start as P_;
            let mut info = get_itbl(p as *mut StgClosure);

            match (*info).r#type {
                42 => {}
                38 => {
                    current_block_20 = 17179679302217393232;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                43 | 44 | 46 | 45 => {
                    current_block_20 = 6873731126896040597;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                59 | 60 | 62 | 61 => {
                    current_block_20 = 7651349459974463963;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                53 => {
                    current_block_20 = 17833034027772472439;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                26 => {
                    current_block_20 = 14321068961168081528;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                25 => {
                    current_block_20 = 14423565235181675139;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                54 => {
                    current_block_20 = 18317007320854588510;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                64 => {
                    current_block_20 = 13135462033962617438;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
                _ => {
                    current_block_20 = 4822848011261434769;

                    match current_block_20 {
                        4822848011261434769 => {
                            barf(
                                c"update_fwd_large: unknown/strange object  %d".as_ptr(),
                                (*info).r#type as i32,
                            );
                        }
                        17833034027772472439 => {
                            let mut stack = p as *mut StgStack;

                            thread_stack(
                                (*stack).sp as P_,
                                (&raw mut (*stack).stack as *mut StgWord)
                                    .offset((*stack).stack_size as isize),
                            );
                        }
                        7651349459974463963 => {
                            let mut a_0 = p as *mut StgSmallMutArrPtrs;
                            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a_0).payload as *mut *mut StgClosure)
                                    .offset((*a_0).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        6873731126896040597 => {
                            let mut a = null_mut::<StgMutArrPtrs>();
                            a = p as *mut StgMutArrPtrs;
                            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

                            while p
                                < (&raw mut (*a).payload as *mut *mut StgClosure)
                                    .offset((*a).ptrs as isize)
                                    as *mut *mut StgClosure as P_
                            {
                                thread(p as *mut *mut StgClosure);
                                p = p.offset(1);
                            }
                        }
                        17179679302217393232 => {
                            thread_obj(info, p);
                        }
                        14321068961168081528 => {
                            thread_AP_STACK(p as *mut StgAP_STACK);
                        }
                        14423565235181675139 => {
                            thread_PAP(p as *mut StgPAP);
                        }
                        13135462033962617438 => {
                            thread_continuation(p as *mut StgContinuation);
                        }
                        _ => {
                            let mut tc = p as *mut StgTRecChunk;
                            let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                                .offset(0)
                                as *mut TRecEntry;
                            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

                            let mut i: W_ = 0;

                            while i < (*tc).next_entry_idx {
                                thread_(&raw mut (*e).tvar as *mut c_void);
                                thread(&raw mut (*e).expected_value);
                                thread(&raw mut (*e).new_value);
                                i = i.wrapping_add(1);
                                e = e.offset(1);
                            }
                        }
                    }
                }
            }
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn thread_obj(mut info: *const StgInfoTable, mut p: P_) -> P_ {
    match (*info).r#type {
        17 => {
            return p
                .offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(1);
        }
        10 | 3 => {
            return p
                .offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(1);
        }
        9 | 2 => {
            thread(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(1);
        }
        16 => {
            thread(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(1);
        }
        20 => {
            return p
                .offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        13 | 6 => {
            return p
                .offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        19 => {
            thread(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        12 | 5 => {
            thread(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        18 => {
            thread(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            thread(
                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(1)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        11 | 4 => {
            thread(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                    as *mut *mut StgClosure,
            );

            thread(
                (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                    as *mut *mut StgClosure,
            );

            return p
                .offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                )
                .offset(2);
        }
        23 => {
            let mut bco = p as *mut StgBCO;
            thread_(&raw mut (*bco).instrs as *mut c_void);
            thread_(&raw mut (*bco).literals as *mut c_void);
            thread_(&raw mut (*bco).ptrs as *mut c_void);

            return p.offset(bco_sizeW(bco) as isize);
        }
        15 => {
            let mut end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                .offset((*info).layout.payload.ptrs as isize);
            p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_;

            while p < end {
                thread(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            return p.offset((*info).layout.payload.nptrs as isize);
        }
        8 | 1 | 7 | 50 | 51 | 47 | 48 | 41 | 38 | 37 => {
            let mut end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_
                as P_)
                .offset((*info).layout.payload.ptrs as isize);
            p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_;

            while p < end_0 {
                thread(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            return p.offset((*info).layout.payload.nptrs as isize);
        }
        49 => {
            let mut w = p as *mut StgWeak;
            thread(&raw mut (*w).cfinalizers);
            thread(&raw mut (*w).key);
            thread(&raw mut (*w).value);
            thread(&raw mut (*w).finalizer);

            if !(*w).link.is_null() {
                thread_(&raw mut (*w).link as *mut c_void);
            }

            return p.offset(
                (size_of::<StgWeak>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize,
            );
        }
        39 | 40 => {
            let mut mvar = p as *mut StgMVar;
            thread_(&raw mut (*mvar).head as *mut c_void);
            thread_(&raw mut (*mvar).tail as *mut c_void);
            thread(&raw mut (*mvar).value);

            return p.offset(
                (size_of::<StgMVar>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize,
            );
        }
        27 => {
            thread(&raw mut (*(p as *mut StgInd)).indirectee);

            return p.offset(
                (size_of::<StgInd>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize,
            );
        }
        22 => {
            let mut s = p as *mut StgSelector;
            thread(&raw mut (*s).selectee);

            return p.offset(THUNK_SELECTOR_sizeW() as isize);
        }
        26 => return thread_AP_STACK(p as *mut StgAP_STACK),
        25 => return thread_PAP(p as *mut StgPAP),
        24 => return thread_AP(p as *mut StgAP),
        42 => return p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize),
        43 | 44 | 46 | 45 => {
            let mut a = p as *mut StgMutArrPtrs;
            p = &raw mut (*a).payload as *mut *mut StgClosure as P_;

            while p
                < (&raw mut (*a).payload as *mut *mut StgClosure).offset((*a).ptrs as isize)
                    as *mut *mut StgClosure as P_
            {
                thread(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            return (a as P_).offset(mut_arr_ptrs_sizeW(a) as isize);
        }
        59 | 60 | 62 | 61 => {
            let mut a_0 = p as *mut StgSmallMutArrPtrs;
            p = &raw mut (*a_0).payload as *mut *mut StgClosure as P_;

            while p
                < (&raw mut (*a_0).payload as *mut *mut StgClosure).offset((*a_0).ptrs as isize)
                    as *mut *mut StgClosure as P_
            {
                thread(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            return (a_0 as P_).offset(small_mut_arr_ptrs_sizeW(a_0) as isize);
        }
        52 => return thread_TSO(p as *mut StgTSO),
        53 => {
            let mut stack = p as *mut StgStack;

            thread_stack(
                (*stack).sp as P_,
                (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
            );

            return p.offset(stack_sizeW(stack) as isize);
        }
        54 => {
            let mut tc = p as *mut StgTRecChunk;
            let mut e: *mut TRecEntry =
                (&raw mut (*tc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
            thread_(&raw mut (*tc).prev_chunk as *mut c_void);

            let mut i: W_ = 0;

            while i < (*tc).next_entry_idx {
                thread_(&raw mut (*e).tvar as *mut c_void);
                thread(&raw mut (*e).expected_value);
                thread(&raw mut (*e).new_value);
                i = i.wrapping_add(1);
                e = e.offset(1);
            }

            return p.offset(
                (size_of::<StgTRecChunk>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize,
            );
        }
        64 => return thread_continuation(p as *mut StgContinuation),
        _ => {
            barf(
                c"update_fwd: unknown/strange object  %d".as_ptr(),
                (*info).r#type as i32,
            );
        }
    };
}

unsafe fn update_fwd(mut blocks: *mut bdescr) {
    let mut bd = blocks;

    while !bd.is_null() {
        let mut p = (*bd).start as P_;

        while p < (*bd).c2rust_unnamed.free {
            if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Compact.c".as_ptr(), 848);
            }

            let mut info = get_itbl(p as *mut StgClosure);
            p = thread_obj(info, p);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn update_fwd_compact(mut blocks: *mut bdescr) {
    let mut bd = blocks;
    let mut free_bd = blocks;
    let mut free = (*free_bd).start as P_;

    while !bd.is_null() {
        let mut p = (*bd).start as P_;

        while p < (*bd).c2rust_unnamed.free {
            while p < (*bd).c2rust_unnamed.free && is_marked(p as StgPtr, bd) == 0 {
                p = p.offset(1);
            }

            if p >= (*bd).c2rust_unnamed.free {
                break;
            }

            let mut iptr = get_threaded_info(p);
            let mut info = INFO_PTR_TO_STRUCT(iptr);
            let mut q = p;
            p = thread_obj(info, p);

            let mut size: W_ = p.offset_from(q) as i64 as W_;

            if free.offset(size as isize) > (*free_bd).start.offset(BLOCK_SIZE_W as isize) {
                mark(q.offset(1), bd);
                free_bd = (*free_bd).link as *mut bdescr;
                free = (*free_bd).start as P_;
            } else if (is_marked(q.offset(1), bd) == 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Compact.c".as_ptr(), 902);
            }

            let mut iptr_tag = get_iptr_tag(iptr) as StgWord;
            unthread(q, free as W_, iptr_tag as W_);
            free = free.offset(size as isize);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn update_bkwd_compact(mut r#gen: *mut generation) -> W_ {
    let mut bd = null_mut::<bdescr>();
    let mut free_bd = null_mut::<bdescr>();
    free_bd = (*r#gen).old_blocks;
    bd = free_bd;

    let mut free = (*free_bd).start as P_;
    let mut free_blocks: W_ = 1;

    while !bd.is_null() {
        let mut p = (*bd).start as P_;

        while p < (*bd).c2rust_unnamed.free {
            while p < (*bd).c2rust_unnamed.free && is_marked(p as StgPtr, bd) == 0 {
                p = p.offset(1);
            }

            if p >= (*bd).c2rust_unnamed.free {
                break;
            }

            if is_marked(p.offset(1), bd) != 0 {
                (*free_bd).c2rust_unnamed.free = free as StgPtr;

                if RtsFlags.DebugFlags.zero_on_gc {
                    memset(
                        (*free_bd).c2rust_unnamed.free as *mut c_void,
                        0xaa,
                        (((1 as u64) << 12 as i32) as W_).wrapping_sub(
                            ((*free_bd).c2rust_unnamed.free.offset_from((*free_bd).start) as i64
                                as W_)
                                .wrapping_mul(size_of::<W_>() as W_),
                        ) as usize,
                    );
                }

                free_bd = (*free_bd).link as *mut bdescr;
                free = (*free_bd).start as P_;
                free_blocks = free_blocks.wrapping_add(1);
            }

            let mut iptr = get_threaded_info(p);
            let mut iptr_tag = get_iptr_tag(iptr) as StgWord;
            unthread(p, free as W_, iptr_tag as W_);

            if LOOKS_LIKE_INFO_PTR((*(p as *mut StgClosure)).header.info as StgWord) as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/sm/Compact.c".as_ptr(), 954);
            }

            let mut info = get_itbl(p as *mut StgClosure);
            let mut size: W_ = closure_sizeW_(p as *mut StgClosure, info) as W_;

            if free != p {
                r#move(free, p, size);
            }

            if (*info).r#type == STACK as StgHalfWord {
                move_STACK(p as *mut StgStack, free as *mut StgStack);
            }

            free = free.offset(size as isize);
            p = p.offset(size as isize);
        }

        bd = (*bd).link as *mut bdescr;
    }

    (*free_bd).c2rust_unnamed.free = free as StgPtr;

    if !(*free_bd).link.is_null() {
        freeChain((*free_bd).link as *mut bdescr);
        (*free_bd).link = null_mut::<bdescr_>();
    }

    if RtsFlags.DebugFlags.zero_on_gc {
        let mut block_size_bytes: W_ =
            ((*free_bd).blocks as u64).wrapping_mul((1 as u64) << 12 as i32) as W_;

        let mut block_in_use_bytes: W_ =
            ((*free_bd).c2rust_unnamed.free.offset_from((*free_bd).start) as i64 as usize)
                .wrapping_mul(size_of::<W_>() as usize) as W_;

        let mut block_free_bytes: W_ = block_size_bytes.wrapping_sub(block_in_use_bytes);

        memset(
            (*free_bd).c2rust_unnamed.free as *mut c_void,
            0xaa,
            block_free_bytes as usize,
        );
    }

    return free_blocks;
}

unsafe fn compact(
    mut static_objects: *mut StgClosure,
    mut dead_weak_ptr_list: *mut *mut StgWeak,
    mut resurrected_threads: *mut *mut StgTSO,
) {
    markCapabilities(
        transmute::<Option<unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()>, evac_fn>(
            Some(thread_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ),
        NULL,
    );

    let mut g: W_ = 0;

    while g < RtsFlags.GcFlags.generations as W_ {
        if !(*generations.offset(g as isize)).weak_ptr_list.is_null() {
            thread(
                &raw mut (*generations.offset(g as isize)).weak_ptr_list as *mut c_void
                    as *mut *mut StgClosure,
            );
        }

        g = g.wrapping_add(1);
    }

    if !dead_weak_ptr_list.is_null() {
        thread(dead_weak_ptr_list as *mut c_void as *mut *mut StgClosure);
    }

    let mut g_0: W_ = 1;

    while g_0 < RtsFlags.GcFlags.generations as W_ {
        let mut n: W_ = 0;

        while n < getNumCapabilities() as W_ {
            let mut bd = *(*getCapability(n as u32)).mut_lists.offset(g_0 as isize);

            while !bd.is_null() {
                let mut p = (*bd).start as P_;

                while p < (*bd).c2rust_unnamed.free {
                    thread(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                bd = (*bd).link as *mut bdescr;
            }

            n = n.wrapping_add(1);
        }

        g_0 = g_0.wrapping_add(1);
    }

    let mut g_1: W_ = 0;

    while g_1 < RtsFlags.GcFlags.generations as W_ {
        thread(
            &raw mut (*generations.offset(g_1 as isize)).threads as *mut c_void
                as *mut *mut StgClosure,
        );

        g_1 = g_1.wrapping_add(1);
    }

    thread(resurrected_threads as *mut c_void as *mut *mut StgClosure);

    let mut task = all_tasks;

    while !task.is_null() {
        let mut incall = (*task).incall as *mut InCall;

        while !incall.is_null() {
            if !(*incall).tso.is_null() {
                thread_(&raw mut (*incall).tso as *mut c_void);
            }

            incall = (*incall).prev_stack as *mut InCall;
        }

        task = (*task).all_next as *mut Task;
    }

    thread_static(static_objects);

    threadStablePtrTable(
        transmute::<Option<unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()>, evac_fn>(
            Some(thread_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ),
        NULL,
    );

    threadStableNameTable(
        transmute::<Option<unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()>, evac_fn>(
            Some(thread_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ),
        NULL,
    );

    markCAFs(
        transmute::<Option<unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()>, evac_fn>(
            Some(thread_root as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ),
        NULL,
    );

    let mut g_2: W_ = 0;

    while g_2 < RtsFlags.GcFlags.generations as W_ {
        let mut r#gen: *mut generation = generations.offset(g_2 as isize) as *mut generation;

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(c"update_fwd:  %d".as_ptr(), g_2);
        }

        update_fwd((*r#gen).blocks);

        let mut n_0: W_ = 0;

        while n_0 < getNumCapabilities() as W_ {
            update_fwd(
                (*(&raw mut (**gc_threads.offset(n_0 as isize)).gens as *mut gen_workspace)
                    .offset(g_2 as isize))
                .0
                .todo_bd,
            );

            update_fwd(
                (*(&raw mut (**gc_threads.offset(n_0 as isize)).gens as *mut gen_workspace)
                    .offset(g_2 as isize))
                .0
                .part_list,
            );

            n_0 = n_0.wrapping_add(1);
        }

        update_fwd_large((*r#gen).scavenged_large_objects);
        update_fwd_cnf((*r#gen).live_compact_objects);

        if g_2 == RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) as W_
            && !(*r#gen).old_blocks.is_null()
        {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
                trace_(c"update_fwd:  %d (compact)".as_ptr(), g_2);
            }

            update_fwd_compact((*r#gen).old_blocks);
        }

        g_2 = g_2.wrapping_add(1);
    }

    let mut gen_0 = oldest_gen;

    if !(*gen_0).old_blocks.is_null() {
        let mut blocks = update_bkwd_compact(gen_0);

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
            trace_(
                c"update_bkwd: %d (compact, old: %d blocks, now %d blocks)".as_ptr(),
                (*gen_0).no,
                (*gen_0).n_old_blocks,
                blocks,
            );
        }

        (*gen_0).n_old_blocks = blocks as memcount;
    }

    rehash_CNFs();
}
