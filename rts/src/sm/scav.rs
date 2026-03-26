use crate::ffi::rts::constants::{BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK, MUT_ARR_PTRS_CARD_BITS};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::{BF_COMPACT, BLOCK_SIZE_W, Bdescr, bdescr, dbl_link_onto};
use crate::ffi::rts::storage::closure_macros::{
    STATIC_LINK, THUNK_SELECTOR_sizeW, UNTAG_CLOSURE, arr_words_sizeW, bco_sizeW, closure_sizeW,
    continuation_sizeW, get_fun_itbl, get_itbl, get_ret_itbl, itbl_to_fun_itbl, itbl_to_thunk_itbl,
    mut_arr_ptrs_sizeW, mutArrPtrsCard, mutArrPtrsCards, small_mut_arr_ptrs_sizeW, stack_sizeW,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgBlockingQueue, StgClosure_, StgCompactNFData,
    StgCompactNFDataBlock, StgContinuation, StgInd, StgMVar, StgMutArrPtrs, StgMutVar, StgPAP,
    StgRetFun, StgSelector, StgSmallMutArrPtrs, StgTRecChunk, StgTVar, StgThunk, StgUpdateFrame,
    StgWeak, TRecEntry, hashtable,
};
use crate::ffi::rts::storage::gc::{generation, generations, memcount, oldest_gen};
use crate::ffi::rts::storage::info_tables::{StgFunInfoTable, StgLargeBitmap, stg_arg_bitmaps};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    stg_BLOCKING_QUEUE_CLEAN_info, stg_BLOCKING_QUEUE_DIRTY_info, stg_COMPACT_NFDATA_CLEAN_info,
    stg_COMPACT_NFDATA_DIRTY_info, stg_MUT_ARR_PTRS_CLEAN_info, stg_MUT_ARR_PTRS_DIRTY_info,
    stg_MUT_ARR_PTRS_FROZEN_CLEAN_info, stg_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_MUT_VAR_CLEAN_info,
    stg_MUT_VAR_DIRTY_info, stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info,
    stg_SMALL_MUT_ARR_PTRS_CLEAN_info, stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
    stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info, stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info,
    stg_TVAR_CLEAN_info, stg_TVAR_DIRTY_info, stg_WHITEHOLE_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::hash::{HashTable, allocHashTable, freeHashTable, insertHashTable, mapHashTable};
use crate::io_manager::scavengeTSOIOManager;
use crate::prelude::*;
use crate::sm::evac::{evacuate, evacuate_BLACKHOLE};
use crate::sm::gc::{N, major_gc, mark_stack_bd};
use crate::sm::gc_thread::{gc_thread, gen_workspace};
use crate::sm::gc_utils::{
    freeChain_sync, grab_local_todo_block, push_scanned_block, recordMutableGen_GC,
};
use crate::sm::gct_decl::the_gc_thread;
use crate::sm::heap_utils::walk_large_bitmap;
use crate::sm::mark_stack::{mark_stack_empty, pop_mark_stack};
use crate::sm::mark_weak::scavengeLiveWeak;
use crate::sm::non_moving::END_NONMOVING_TODO_LIST;
use crate::sm::non_moving_scav::{nonmovingScavengeOne, scavengeNonmovingSegment};
use crate::sm::storage::{STATIC_BITS, static_flag};
use crate::trace::{DEBUG_RTS, trace_};

/// cbindgen:no-export
struct MapHashData {
    saved_gct: *mut gc_thread,
    newHash: *mut HashTable,
}

unsafe fn do_evacuate(mut p: *mut *mut StgClosure, mut user: *mut c_void) {
    evacuate(p);
}

unsafe fn scavengeTSO(mut tso: *mut StgTSO) {
    let mut saved_eager: bool = false;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"scavenging thread %llu\0" as *const u8 as *const c_char as *mut c_char,
            (*tso).id,
        );
    }

    if !(*tso).bound.is_null() {
        evacuate(&raw mut (*(*tso).bound).tso as *mut *mut StgClosure);
    }

    saved_eager = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
    evacuate(&raw mut (*tso).blocked_exceptions as *mut *mut StgClosure);
    evacuate(&raw mut (*tso).bq as *mut *mut StgClosure);
    evacuate(&raw mut (*tso).trec as *mut *mut StgClosure);
    evacuate(&raw mut (*tso).stackobj as *mut *mut StgClosure);
    evacuate(&raw mut (*tso)._link as *mut *mut StgClosure);

    if !(*tso).label.is_null() {
        evacuate(&raw mut (*tso).label as *mut *mut StgClosure);
    }

    match (*tso).why_blocked {
        1 | 14 | 2 | 12 | 0 => {
            evacuate(&raw mut (*tso).block_info.closure);
        }
        3 | 4 | 5 | 7 => {
            scavengeTSOIOManager(tso);
        }
        _ => {}
    }

    (*tso).dirty = (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac as StgWord32;
    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = saved_eager;
}

unsafe fn evacuate_hash_entry(
    mut dat: *mut MapHashData,
    mut key: StgWord,
    mut value: *const c_void,
) {
    let mut p = key as *mut StgClosure;
    evacuate(&raw mut p);
    insertHashTable((*dat).newHash, p as StgWord, value);
}

unsafe fn scavenge_compact(mut str: *mut StgCompactNFData) {
    let mut saved_eager: bool = false;
    saved_eager = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;

    if !(*str).hash.is_null() {
        let mut dat = MapHashData {
            saved_gct: null_mut::<gc_thread>(),
            newHash: null_mut::<HashTable>(),
        };

        dat.saved_gct = &raw mut the_gc_thread as *mut gc_thread;

        let mut newHash = allocHashTable();
        dat.newHash = newHash;

        mapHashTable(
            (*str).hash as *mut HashTable,
            &raw mut dat as *mut c_void,
            transmute::<
                Option<unsafe extern "C" fn(*mut MapHashData, StgWord, *const c_void) -> ()>,
                MapHashFn,
            >(Some(
                evacuate_hash_entry
                    as unsafe extern "C" fn(*mut MapHashData, StgWord, *const c_void) -> (),
            )),
        );

        freeHashTable((*str).hash as *mut HashTable, None);
        (*str).hash = newHash as *mut hashtable;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.compact as c_long != 0 {
        trace_(
            b"compact alive @%p, gen %d, %llu bytes\0" as *const u8 as *const c_char as *mut c_char,
            str,
            (*Bdescr(str as StgPtr)).gen_no as c_int,
            (*str).totalW.wrapping_mul(size_of::<W_>() as StgWord),
        );
    }

    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = saved_eager;

    if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
        let ref mut fresh36 = (*(str as *mut StgClosure)).header.info;
        *fresh36 = &raw const stg_COMPACT_NFDATA_DIRTY_info;
    } else {
        let ref mut fresh37 = (*(str as *mut StgClosure)).header.info;
        *fresh37 = &raw const stg_COMPACT_NFDATA_CLEAN_info;
    };
}

unsafe fn scavenge_mut_arr_ptrs(mut a: *mut StgMutArrPtrs) -> StgPtr {
    let mut m: W_ = 0;
    let mut any_failed: bool = false;
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    any_failed = r#false != 0;
    p = (&raw mut (*a).payload as *mut *mut StgClosure).offset(0 as c_int as isize)
        as *mut *mut StgClosure as StgPtr;
    m = 0 as W_;

    while (m as c_int) < mutArrPtrsCards((*a).ptrs as W_) as c_int - 1 as c_int {
        q = p.offset(((1 as c_int) << MUT_ARR_PTRS_CARD_BITS) as isize);

        while p < q {
            evacuate(p as *mut *mut StgClosure);
            p = p.offset(1);
        }

        if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
            any_failed = r#true != 0;
            *mutArrPtrsCard(a, m) = 1 as StgWord8;
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
        } else {
            *mutArrPtrsCard(a, m) = 0 as StgWord8;
        }

        m = m.wrapping_add(1);
    }

    q = (&raw mut (*a).payload as *mut *mut StgClosure).offset((*a).ptrs as isize)
        as *mut *mut StgClosure as StgPtr;

    if p < q {
        while p < q {
            evacuate(p as *mut *mut StgClosure);
            p = p.offset(1);
        }

        if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
            any_failed = r#true != 0;
            *mutArrPtrsCard(a, m) = 1 as StgWord8;
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
        } else {
            *mutArrPtrsCard(a, m) = 0 as StgWord8;
        }
    }

    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = any_failed;

    return (a as StgPtr).offset(mut_arr_ptrs_sizeW(a) as isize);
}

unsafe fn scavenge_mut_arr_ptrs_marked(mut a: *mut StgMutArrPtrs) -> StgPtr {
    let mut m: W_ = 0;
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut any_failed: bool = false;
    any_failed = r#false != 0;
    m = 0 as W_;

    while m < mutArrPtrsCards((*a).ptrs as W_) {
        if *mutArrPtrsCard(a, m) as c_int != 0 as c_int {
            p = (&raw mut (*a).payload as *mut *mut StgClosure)
                .offset((m << MUT_ARR_PTRS_CARD_BITS) as isize)
                as *mut *mut StgClosure as StgPtr;

            q = ({
                let mut _a = p.offset(((1 as c_int) << 7 as c_int) as isize);
                let mut _b = (&raw mut (*a).payload as *mut *mut StgClosure)
                    .offset((*a).ptrs as isize) as *mut *mut StgClosure
                    as StgPtr;

                if _a <= _b { _a as StgPtr } else { _b as StgPtr }
            });

            while p < q {
                evacuate(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                any_failed = r#true != 0;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
            } else {
                *mutArrPtrsCard(a, m) = 0 as StgWord8;
            }
        }

        m = m.wrapping_add(1);
    }

    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = any_failed;

    return (a as StgPtr).offset(mut_arr_ptrs_sizeW(a) as isize);
}

#[inline]
unsafe fn scavenge_small_bitmap(mut p: StgPtr, mut size: StgWord, mut bitmap: StgWord) -> StgPtr {
    while size > 0 as StgWord {
        if bitmap & 1 as StgWord == 0 as StgWord {
            evacuate(p as *mut *mut StgClosure);
        }

        p = p.offset(1);
        bitmap = bitmap >> 1 as c_int;
        size = size.wrapping_sub(1);
    }

    return p;
}

#[inline]
unsafe fn scavenge_arg_block(
    mut fun_info: *const StgFunInfoTable,
    mut args: *mut *mut StgClosure,
) -> StgPtr {
    let mut p = null_mut::<StgWord>();
    let mut bitmap: StgWord = 0;
    let mut size: StgWord = 0;
    p = args as StgPtr;

    let mut current_block_8: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            size = (*fun_info).f.b.bitmap & BITMAP_SIZE_MASK as StgWord;
            current_block_8 = 13148051919047804176;
        }
        1 => {
            size = (*((fun_info.offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                as *mut StgLargeBitmap))
                .size;

            scavenge_large_bitmap(
                p,
                (fun_info.offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_8 = 11812396948646013369;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;

            size = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                & BITMAP_SIZE_MASK as StgWord;
            current_block_8 = 13148051919047804176;
        }
    }

    match current_block_8 {
        13148051919047804176 => {
            p = scavenge_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

#[inline]
unsafe fn scavenge_PAP_payload(
    mut fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut size: StgWord,
) -> StgPtr {
    let mut p = null_mut::<StgWord>();
    let mut bitmap: StgWord = 0;
    let mut fun_info = null::<StgFunInfoTable>();
    fun = UNTAG_CLOSURE(fun);
    fun_info = get_fun_itbl(fun);
    p = payload as StgPtr;

    let mut current_block_12: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            current_block_12 = 6251836766594653680;
        }
        1 => {
            scavenge_large_bitmap(
                p,
                (fun_info.offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_12 = 3512920355445576850;
        }
        2 => {
            scavenge_large_bitmap(
                payload as StgPtr,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_12 = 3512920355445576850;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;

            current_block_12 = 6251836766594653680;
        }
    }

    match current_block_12 {
        6251836766594653680 => {
            p = scavenge_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

unsafe fn scavenge_PAP(mut pap: *mut StgPAP) -> StgPtr {
    evacuate(&raw mut (*pap).fun);

    return scavenge_PAP_payload(
        (*pap).fun,
        &raw mut (*pap).payload as *mut *mut StgClosure,
        (*pap).n_args as StgWord,
    );
}

unsafe fn scavenge_AP(mut ap: *mut StgAP) -> StgPtr {
    evacuate(&raw mut (*ap).fun);

    return scavenge_PAP_payload(
        (*ap).fun,
        &raw mut (*ap).payload as *mut *mut StgClosure,
        (*ap).n_args as StgWord,
    );
}

unsafe fn scavenge_continuation(mut cont: *mut StgContinuation) -> StgPtr {
    scavenge_stack(
        &raw mut (*cont).stack as StgPtr,
        (&raw mut (*cont).stack as *mut StgWord).offset((*cont).stack_size as isize),
    );

    return (cont as StgPtr).offset(continuation_sizeW(cont) as isize);
}

unsafe fn scavenge_thunk_srt(mut info: *const StgInfoTable) {
    let mut thunk_info = null_mut::<StgThunkInfoTable>();

    if !major_gc {
        return;
    }

    thunk_info = itbl_to_thunk_itbl(info);

    if (*thunk_info).i.srt != 0 {
        let mut srt = (thunk_info.offset(1 as c_int as isize) as StgWord)
            .wrapping_add((*thunk_info).i.srt as StgWord) as *mut StgClosure;
        evacuate(&raw mut srt);
    }
}

unsafe fn scavenge_fun_srt(mut info: *const StgInfoTable) {
    let mut fun_info = null_mut::<StgFunInfoTable>();

    if !major_gc {
        return;
    }

    fun_info = itbl_to_fun_itbl(info);

    if (*fun_info).i.srt != 0 {
        let mut srt = (fun_info.offset(1 as c_int as isize) as StgWord)
            .wrapping_add((*fun_info).i.srt as StgWord) as *mut StgClosure;
        evacuate(&raw mut srt);
    }
}

unsafe fn scavenge_block(mut bd: *mut bdescr) {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut info = null::<StgInfoTable>();
    let mut saved_eager_promotion: bool = false;
    let mut ws = null_mut::<gen_workspace>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"scavenging block %p (gen %d) @ %p\0" as *const u8 as *const c_char as *mut c_char,
            (*bd).start,
            (*bd).gen_no as c_int,
            (*bd).u.scan,
        );
    }

    let ref mut fresh11 = (*(&raw mut the_gc_thread as *mut gc_thread)).scan_bd;
    *fresh11 = bd;
    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = (*bd).gen_no as uint32_t;
    saved_eager_promotion = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
    ws = (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
        .offset((*bd).gen_no as isize) as *mut gen_workspace;
    p = (*bd).u.scan;

    while p < (*bd).c2rust_unnamed.free || bd == (*ws).0.todo_bd && p < (*ws).0.todo_free {
        info = get_itbl(p as *mut StgClosure);
        q = p;

        let mut current_block_192: u64;

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).value);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*mvar).header.info = &raw const stg_MVAR_DIRTY_info;
                } else {
                    (*mvar).header.info = &raw const stg_MVAR_CLEAN_info;
                }

                p = p.offset(
                    (size_of::<StgMVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tvar).current_value);
                evacuate(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*tvar).header.info = &raw const stg_TVAR_DIRTY_info;
                } else {
                    (*tvar).header.info = &raw const stg_TVAR_CLEAN_info;
                }

                p = p.offset(
                    (size_of::<StgTVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            11 => {
                scavenge_fun_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            18 => {
                scavenge_thunk_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            4 => {
                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            16 => {
                scavenge_thunk_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            9 => {
                scavenge_fun_srt(info);
                current_block_192 = 16508105131140061713;
            }
            2 => {
                current_block_192 = 16508105131140061713;
            }
            17 => {
                scavenge_thunk_srt(info);

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            10 => {
                scavenge_fun_srt(info);
                current_block_192 = 11898771182088686224;
            }
            3 => {
                current_block_192 = 11898771182088686224;
            }
            20 => {
                scavenge_thunk_srt(info);

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            13 => {
                scavenge_fun_srt(info);
                current_block_192 = 11003519446013548369;
            }
            6 => {
                current_block_192 = 11003519446013548369;
            }
            19 => {
                scavenge_thunk_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            12 => {
                scavenge_fun_srt(info);
                current_block_192 = 15810552018466442339;
            }
            5 => {
                current_block_192 = 15810552018466442339;
            }
            8 => {
                scavenge_fun_srt(info);
                current_block_192 = 919954187481050311;
            }
            15 => {
                let mut end = null_mut::<StgWord>();
                scavenge_thunk_srt(info);
                end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
                current_block_192 = 15908231092227701503;
            }
            1 | 7 | 49 | 50 => {
                current_block_192 = 919954187481050311;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                p = p.offset(bco_sizeW(bco) as isize);
                current_block_192 = 15908231092227701503;
            }
            38 => {
                evacuate(&raw mut (*(p as *mut StgInd)).indirectee);

                p = p.offset(
                    (size_of::<StgInd>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            47 | 48 => {
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*(p as *mut StgMutVar)).var);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh12 = (*(q as *mut StgClosure)).header.info;
                    *fresh12 = &raw const stg_MUT_VAR_DIRTY_info;
                } else {
                    let ref mut fresh13 = (*(q as *mut StgClosure)).header.info;
                    *fresh13 = &raw const stg_MUT_VAR_CLEAN_info;
                }

                p = p.offset(
                    (size_of::<StgMutVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*bq).bh);
                evacuate(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).link as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
                } else {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_CLEAN_info;
                }

                p = p.offset(
                    (size_of::<StgBlockingQueue>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate(&raw mut (*s).selectee);
                p = p.offset(THUNK_SELECTOR_sizeW() as isize);
                current_block_192 = 15908231092227701503;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate(&raw mut (*ap).fun);

                scavenge_stack(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                p = (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap).size as isize);
                current_block_192 = 15908231092227701503;
            }
            25 => {
                p = scavenge_PAP(p as *mut StgPAP);
                current_block_192 = 15908231092227701503;
            }
            24 => {
                p = scavenge_AP(p as *mut StgAP);
                current_block_192 = 15908231092227701503;
            }
            42 => {
                p = p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize);
                current_block_192 = 15908231092227701503;
            }
            43 | 44 => {
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                p = scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh14 = (*(q as *mut StgClosure)).header.info;
                    *fresh14 = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh15 = (*(q as *mut StgClosure)).header.info;
                    *fresh15 = &raw const stg_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_192 = 15908231092227701503;
            }
            46 | 45 => {
                p = scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh16 = (*(q as *mut StgClosure)).header.info;
                    *fresh16 = &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh17 = (*(q as *mut StgClosure)).header.info;
                    *fresh17 = &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                current_block_192 = 15908231092227701503;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh18 = (*(q as *mut StgClosure)).header.info;
                    *fresh18 = &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh19 = (*(q as *mut StgClosure)).header.info;
                    *fresh19 = &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_192 = 15908231092227701503;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh20 = (*(q as *mut StgClosure)).header.info;
                    *fresh20 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh21 = (*(q as *mut StgClosure)).header.info;
                    *fresh21 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                current_block_192 = 15908231092227701503;
            }
            52 => {
                scavengeTSO(p as *mut StgTSO);

                p = p.offset(
                    (size_of::<StgTSO>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;

                scavenge_stack(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty =
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac as StgWord8;
                p = p.offset(stack_sizeW(stack) as isize);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                current_block_192 = 15908231092227701503;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_192 = 15908231092227701503;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                    .offset(0 as c_int as isize)
                    as *mut TRecEntry;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0 as StgWord;

                while i < (*tc).next_entry_idx {
                    evacuate(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate(&raw mut (*e).expected_value);
                    evacuate(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;

                p = p.offset(
                    (size_of::<StgTRecChunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 15908231092227701503;
            }
            64 => {
                p = scavenge_continuation(p as *mut StgContinuation);
                current_block_192 = 15908231092227701503;
            }
            _ => {
                barf(
                    b"scavenge: unimplemented/strange closure type %d @ %p\0" as *const u8
                        as *const c_char,
                    (*info).r#type,
                    p,
                );
            }
        }

        match current_block_192 {
            919954187481050311 => {
                let mut end_0 = null_mut::<StgWord>();
                end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_0 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
            }
            15810552018466442339 => {
                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );
            }
            16508105131140061713 => {
                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );
            }
            11898771182088686224 => {
                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );
            }
            11003519446013548369 => {
                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );
            }
            _ => {}
        }

        if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;

            if (*bd).gen_no as c_int > 0 as c_int {
                recordMutableGen_GC(q as *mut StgClosure, (*bd).gen_no as uint32_t);
            }
        }
    }

    if p > (*bd).c2rust_unnamed.free {
        let ref mut fresh22 = (*(&raw mut the_gc_thread as *mut gc_thread)).copied;
        *fresh22 = (*fresh22)
            .wrapping_add((*ws).0.todo_free.offset_from((*bd).c2rust_unnamed.free) as c_long as W_);
        (*bd).c2rust_unnamed.free = p;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(
            b"   scavenged %ld bytes\0" as *const u8 as *const c_char as *mut c_char,
            ((*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as c_long as usize)
                .wrapping_mul(size_of::<W_>() as usize) as c_ulong,
        );
    }

    let ref mut fresh23 = (*(&raw mut the_gc_thread as *mut gc_thread)).scanned;
    *fresh23 = (*fresh23)
        .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as c_long as W_);
    (*bd).u.scan = (*bd).c2rust_unnamed.free;

    if bd != (*ws).0.todo_bd {
        push_scanned_block(bd, ws);
    }

    let ref mut fresh24 = (*(&raw mut the_gc_thread as *mut gc_thread)).scan_bd;
    *fresh24 = null_mut::<bdescr>();
}

unsafe fn scavenge_mark_stack() {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut info = null::<StgInfoTable>();
    let mut saved_eager_promotion: bool = false;
    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = (*oldest_gen).no;
    saved_eager_promotion = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;

    loop {
        p = pop_mark_stack();

        if p.is_null() {
            break;
        }

        info = get_itbl(p as *mut StgClosure);
        q = p;

        let mut current_block_144: u64;

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).value);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*mvar).header.info = &raw const stg_MVAR_DIRTY_info;
                } else {
                    (*mvar).header.info = &raw const stg_MVAR_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tvar).current_value);
                evacuate(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*tvar).header.info = &raw const stg_TVAR_DIRTY_info;
                } else {
                    (*tvar).header.info = &raw const stg_TVAR_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            11 => {
                scavenge_fun_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            18 => {
                scavenge_thunk_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            4 => {
                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(1 as c_int as isize) as *mut *mut StgClosure,
                );

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            9 | 12 => {
                scavenge_fun_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            16 | 19 => {
                scavenge_thunk_srt(info);

                evacuate(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            2 | 5 => {
                evacuate(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_)
                        .offset(0 as c_int as isize) as *mut *mut StgClosure,
                );

                current_block_144 = 4338462691184853296;
            }
            10 | 13 => {
                scavenge_fun_srt(info);
                current_block_144 = 4338462691184853296;
            }
            17 | 20 => {
                scavenge_thunk_srt(info);
                current_block_144 = 4338462691184853296;
            }
            8 => {
                scavenge_fun_srt(info);
                current_block_144 = 3689906465960840878;
            }
            15 => {
                let mut end = null_mut::<StgWord>();
                scavenge_thunk_srt(info);
                end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                current_block_144 = 4338462691184853296;
            }
            1 | 7 | 49 | 50 => {
                current_block_144 = 3689906465960840878;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                current_block_144 = 4338462691184853296;
            }
            27 | 38 => {
                evacuate(&raw mut (*(p as *mut StgInd)).indirectee);
                current_block_144 = 4338462691184853296;
            }
            47 | 48 => {
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*(p as *mut StgMutVar)).var);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh38 = (*(q as *mut StgClosure)).header.info;
                    *fresh38 = &raw const stg_MUT_VAR_DIRTY_info;
                } else {
                    let ref mut fresh39 = (*(q as *mut StgClosure)).header.info;
                    *fresh39 = &raw const stg_MUT_VAR_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*bq).bh);
                evacuate(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).link as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
                } else {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            3 | 6 | 42 => {
                current_block_144 = 4338462691184853296;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate(&raw mut (*s).selectee);
                current_block_144 = 4338462691184853296;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate(&raw mut (*ap).fun);

                scavenge_stack(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                current_block_144 = 4338462691184853296;
            }
            25 => {
                scavenge_PAP(p as *mut StgPAP);
                current_block_144 = 4338462691184853296;
            }
            24 => {
                scavenge_AP(p as *mut StgAP);
                current_block_144 = 4338462691184853296;
            }
            43 | 44 => {
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh40 = (*(q as *mut StgClosure)).header.info;
                    *fresh40 = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh41 = (*(q as *mut StgClosure)).header.info;
                    *fresh41 = &raw const stg_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_144 = 4338462691184853296;
            }
            46 | 45 => {
                let mut q_0 = p;
                scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh42 = (*(q_0 as *mut StgClosure)).header.info;
                    *fresh42 = &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh43 = (*(q_0 as *mut StgClosure)).header.info;
                    *fresh43 = &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                let mut saved_eager: bool = false;
                saved_eager = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = saved_eager;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh44 = (*(q as *mut StgClosure)).header.info;
                    *fresh44 = &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh45 = (*(q as *mut StgClosure)).header.info;
                    *fresh45 = &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_144 = 4338462691184853296;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                let mut q_1 = p;
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh46 = (*(q_1 as *mut StgClosure)).header.info;
                    *fresh46 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh47 = (*(q_1 as *mut StgClosure)).header.info;
                    *fresh47 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                current_block_144 = 4338462691184853296;
            }
            52 => {
                scavengeTSO(p as *mut StgTSO);
                current_block_144 = 4338462691184853296;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;

                scavenge_stack(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty =
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac as StgWord8;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                current_block_144 = 4338462691184853296;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_144 = 4338462691184853296;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                    .offset(0 as c_int as isize)
                    as *mut TRecEntry;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0 as StgWord;

                while i < (*tc).next_entry_idx {
                    evacuate(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate(&raw mut (*e).expected_value);
                    evacuate(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                current_block_144 = 4338462691184853296;
            }
            64 => {
                scavenge_continuation(p as *mut StgContinuation);
                current_block_144 = 4338462691184853296;
            }
            _ => {
                barf(
                    b"scavenge_mark_stack: unimplemented/strange closure type %d @ %p\0"
                        as *const u8 as *const c_char,
                    (*info).r#type,
                    p,
                );
            }
        }

        match current_block_144 {
            3689906465960840878 => {
                let mut end_0 = null_mut::<StgWord>();
                end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_0 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }
            }
            _ => {}
        }

        if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
            (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;

            if (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no != 0 {
                recordMutableGen_GC(
                    q as *mut StgClosure,
                    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no,
                );
            }
        }
    }
}

unsafe fn scavenge_one(mut p: StgPtr) -> bool {
    let mut info = null::<StgInfoTable>();
    let mut no_luck: bool = false;
    let mut saved_eager_promotion: bool = false;

    loop {
        saved_eager_promotion = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
        info = get_itbl(p as *mut StgClosure);

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate(&raw mut (*mvar).value);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*mvar).header.info = &raw const stg_MVAR_DIRTY_info;
                } else {
                    (*mvar).header.info = &raw const stg_MVAR_CLEAN_info;
                }

                break;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tvar).current_value);
                evacuate(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*tvar).header.info = &raw const stg_TVAR_DIRTY_info;
                } else {
                    (*tvar).header.info = &raw const stg_TVAR_CLEAN_info;
                }

                break;
            }
            15 | 16 | 17 | 19 | 20 | 18 => {
                let mut q = null_mut::<StgWord>();
                let mut end = null_mut::<StgWord>();
                end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as StgPtr)
                    .offset((*info).layout.payload.ptrs as isize);
                q = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as StgPtr;

                while q < end {
                    evacuate(q as *mut *mut StgClosure);
                    q = q.offset(1);
                }

                break;
            }
            8 | 9 | 10 | 12 | 13 | 11 | 1 | 7 | 2 | 3 | 5 | 6 | 4 | 50 => {
                let mut q_0 = null_mut::<StgWord>();
                let mut end_0 = null_mut::<StgWord>();
                end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_
                    as StgPtr)
                    .offset((*info).layout.payload.ptrs as isize);
                q_0 = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as StgPtr;

                while q_0 < end_0 {
                    evacuate(q_0 as *mut *mut StgClosure);
                    q_0 = q_0.offset(1);
                }

                break;
            }
            49 => {
                scavengeLiveWeak(p as *mut StgWeak);
                break;
            }
            47 | 48 => {
                let mut q_1 = p;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*(p as *mut StgMutVar)).var);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh26 = (*(q_1 as *mut StgClosure)).header.info;
                    *fresh26 = &raw const stg_MUT_VAR_DIRTY_info;
                } else {
                    let ref mut fresh27 = (*(q_1 as *mut StgClosure)).header.info;
                    *fresh27 = &raw const stg_MUT_VAR_CLEAN_info;
                }

                break;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*bq).bh);
                evacuate(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate(&raw mut (*bq).link as *mut *mut StgClosure);
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_DIRTY_info;
                } else {
                    (*bq).header.info = &raw const stg_BLOCKING_QUEUE_CLEAN_info;
                }

                break;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate(&raw mut (*s).selectee);
                break;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate(&raw mut (*ap).fun);

                scavenge_stack(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                p = (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap).size as isize);
                break;
            }
            25 => {
                p = scavenge_PAP(p as *mut StgPAP);
                break;
            }
            24 => {
                p = scavenge_AP(p as *mut StgAP);
                break;
            }
            42 => {
                break;
            }
            43 | 44 => {
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh28 = (*(p as *mut StgClosure)).header.info;
                    *fresh28 = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh29 = (*(p as *mut StgClosure)).header.info;
                    *fresh29 = &raw const stg_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                break;
            }
            46 | 45 => {
                scavenge_mut_arr_ptrs(p as *mut StgMutArrPtrs);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh30 = (*(p as *mut StgClosure)).header.info;
                    *fresh30 = &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh31 = (*(p as *mut StgClosure)).header.info;
                    *fresh31 = &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                break;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                let mut q_2 = null_mut::<StgWord>();
                let mut saved_eager: bool = false;
                saved_eager = (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                q_2 = p;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = saved_eager;

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh32 = (*(q_2 as *mut StgClosure)).header.info;
                    *fresh32 = &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info;
                } else {
                    let ref mut fresh33 = (*(q_2 as *mut StgClosure)).header.info;
                    *fresh33 = &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info;
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                break;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                let mut q_3 = p;
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);
                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    let ref mut fresh34 = (*(q_3 as *mut StgClosure)).header.info;
                    *fresh34 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info;
                } else {
                    let ref mut fresh35 = (*(q_3 as *mut StgClosure)).header.info;
                    *fresh35 = &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info;
                }

                break;
            }
            52 => {
                scavengeTSO(p as *mut StgTSO);
                break;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;

                scavenge_stack(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty =
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac as StgWord8;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                break;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                break;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry = (&raw mut (*tc).entries as *mut TRecEntry)
                    .offset(0 as c_int as isize)
                    as *mut TRecEntry;
                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                evacuate(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0 as StgWord;

                while i < (*tc).next_entry_idx {
                    evacuate(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate(&raw mut (*e).expected_value);
                    evacuate(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                    saved_eager_promotion;
                (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#true != 0;
                break;
            }
            27 | 38 | 28 => {
                evacuate(&raw mut (*(p as *mut StgInd)).indirectee);
                break;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                break;
            }
            63 => {
                scavenge_compact(p as *mut StgCompactNFData);
                break;
            }
            64 => {
                scavenge_continuation(p as *mut StgContinuation);
                break;
            }
            58 => while get_itbl(p as *mut StgClosure) == &raw const stg_WHITEHOLE_info {},
            _ => {
                barf(
                    b"scavenge_one: strange object %d\0" as *const u8 as *const c_char,
                    (*info).r#type as c_int,
                );
            }
        }
    }

    no_luck = (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac;
    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;

    return no_luck;
}

unsafe fn scavenge_mutable_list(mut bd: *mut bdescr, mut r#gen: *mut generation) {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut gen_no: uint32_t = (*r#gen).no;
    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = gen_no;

    while !bd.is_null() {
        q = (*bd).start;

        while q < (*bd).c2rust_unnamed.free {
            p = *q as StgPtr;

            match (*get_itbl(p as *mut StgClosure)).r#type {
                43 | 59 => {
                    recordMutableGen_GC(p as *mut StgClosure, gen_no);
                }
                44 => {
                    let mut saved_eager_promotion: bool = false;
                    saved_eager_promotion =
                        (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion;
                    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion = r#false != 0;
                    scavenge_mut_arr_ptrs_marked(p as *mut StgMutArrPtrs);

                    if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                        let ref mut fresh52 = (*(p as *mut StgClosure)).header.info;
                        *fresh52 = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
                    } else {
                        let ref mut fresh53 = (*(p as *mut StgClosure)).header.info;
                        *fresh53 = &raw const stg_MUT_ARR_PTRS_CLEAN_info;
                    }

                    (*(&raw mut the_gc_thread as *mut gc_thread)).eager_promotion =
                        saved_eager_promotion;
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
                    recordMutableGen_GC(p as *mut StgClosure, gen_no);
                }
                _ => {
                    if RtsFlags.GcFlags.useNonmoving as c_int != 0
                        && major_gc as c_int != 0
                        && r#gen == oldest_gen
                    {
                        nonmovingScavengeOne(p as *mut StgClosure);
                    } else if scavenge_one(p) {
                        recordMutableGen_GC(p as *mut StgClosure, gen_no);
                    }
                }
            }

            q = q.offset(1);
        }

        bd = (*bd).link as *mut bdescr;
    }
}

unsafe fn scavenge_capability_mut_lists(mut cap: *mut Capability) {
    if RtsFlags.GcFlags.useNonmoving as c_int != 0 && major_gc as c_int != 0 {
        let mut g: uint32_t = (*oldest_gen).no;
        scavenge_mutable_list(*(*cap).saved_mut_lists.offset(g as isize), oldest_gen);
        freeChain_sync(*(*cap).saved_mut_lists.offset(g as isize));

        let ref mut fresh50 = *(*cap).saved_mut_lists.offset(g as isize);
        *fresh50 = null_mut::<bdescr>();
        return;
    }

    let mut g_0: uint32_t = RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t);

    while g_0 > N {
        scavenge_mutable_list(
            *(*cap).saved_mut_lists.offset(g_0 as isize),
            generations.offset(g_0 as isize) as *mut generation,
        );

        freeChain_sync(*(*cap).saved_mut_lists.offset(g_0 as isize));

        let ref mut fresh51 = *(*cap).saved_mut_lists.offset(g_0 as isize);
        *fresh51 = null_mut::<bdescr>();
        g_0 = g_0.wrapping_sub(1);
    }
}

unsafe fn scavenge_static() {
    let mut flagged_p = null_mut::<StgClosure>();
    let mut p = null_mut::<StgClosure>();
    let mut info = null::<StgInfoTable>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as c_long != 0 {
        trace_(b"scavenging static objects\0" as *const u8 as *const c_char as *mut c_char);
    }

    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = (*oldest_gen).no;

    loop {
        flagged_p = (*(&raw mut the_gc_thread as *mut gc_thread)).static_objects;

        if flagged_p == static_flag as StgWord as *mut StgClosure {
            break;
        }

        p = (flagged_p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;
        info = get_itbl(p);

        let mut link = STATIC_LINK(info, p);
        let ref mut fresh48 = (*(&raw mut the_gc_thread as *mut gc_thread)).static_objects;
        *fresh48 = *link;
        *link = (*(&raw mut the_gc_thread as *mut gc_thread)).scavenged_static_objects;

        let ref mut fresh49 =
            (*(&raw mut the_gc_thread as *mut gc_thread)).scavenged_static_objects;
        *fresh49 = flagged_p;

        let mut current_block_27: u64;

        match (*info).r#type {
            28 => {
                let mut ind = p as *mut StgInd;
                evacuate(&raw mut (*ind).indirectee);

                if (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac {
                    (*(&raw mut the_gc_thread as *mut gc_thread)).failed_to_evac = r#false != 0;
                    recordMutableGen_GC(p, (*oldest_gen).no);
                }

                current_block_27 = 4761528863920922185;
            }
            21 => {
                scavenge_thunk_srt(info);
                current_block_27 = 4761528863920922185;
            }
            14 => {
                scavenge_fun_srt(info);
                current_block_27 = 5783071609795492627;
            }
            1 | 7 | 2 | 3 | 4 | 5 | 6 => {
                current_block_27 = 5783071609795492627;
            }
            _ => {
                barf(
                    b"scavenge_static: strange closure %d\0" as *const u8 as *const c_char,
                    (*info).r#type as c_int,
                );
            }
        }

        match current_block_27 {
            5783071609795492627 => {
                let mut q = null_mut::<StgWord>();
                let mut next = null_mut::<StgWord>();
                next = (&raw mut (*p).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                q = &raw mut (*p).payload as *mut *mut StgClosure_ as P_ as StgPtr;

                while q < next {
                    evacuate(q as *mut *mut StgClosure);
                    q = q.offset(1);
                }
            }
            _ => {}
        }
    }
}

unsafe fn scavenge_large_bitmap(
    mut p: StgPtr,
    mut large_bitmap: *mut StgLargeBitmap,
    mut size: StgWord,
) {
    walk_large_bitmap(
        Some(do_evacuate as unsafe extern "C" fn(*mut *mut StgClosure, *mut c_void) -> ()),
        p as *mut *mut StgClosure,
        large_bitmap,
        size,
        NULL,
    );
}

unsafe fn scavenge_stack(mut p: StgPtr, mut stack_end: StgPtr) {
    let mut info = null::<StgRetInfoTable>();
    let mut bitmap: StgWord = 0;
    let mut size: StgWord = 0;

    while p < stack_end {
        info = get_ret_itbl(p as *mut StgClosure);

        match (*info).i.r#type {
            33 => {
                let mut frame = p as *mut StgUpdateFrame;
                evacuate_BLACKHOLE(&raw mut (*frame).updatee);

                p = p.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                continue;
            }
            57 | 56 | 55 | 35 | 36 | 34 | 30 | 65 => {
                bitmap = (*info).i.layout.bitmap >> BITMAP_BITS_SHIFT;
                size = (*info).i.layout.bitmap & BITMAP_SIZE_MASK as StgWord;
                p = p.offset(1);
                p = scavenge_small_bitmap(p, size, bitmap);
            }
            29 => {
                let mut bco = null_mut::<StgBCO>();
                let mut size_0: StgWord = 0;
                p = p.offset(1);
                evacuate(p as *mut *mut StgClosure);
                bco = *p as *mut StgBCO;
                p = p.offset(1);
                size_0 = (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size;

                scavenge_large_bitmap(
                    p,
                    &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                    size_0,
                );

                p = p.offset(size_0 as isize);
                continue;
            }
            31 => {
                let mut size_1: StgWord = 0;
                size_1 = (*(((&raw const (*info).i).offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap))
                    .size;
                p = p.offset(1);

                scavenge_large_bitmap(
                    p,
                    ((&raw const (*info).i).offset(1 as c_int as isize) as StgWord)
                        .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                        as *mut StgLargeBitmap,
                    size_1,
                );

                p = p.offset(size_1 as isize);
            }
            32 => {
                let mut ret_fun = p as *mut StgRetFun;
                let mut fun_info = null::<StgFunInfoTable>();
                evacuate(&raw mut (*ret_fun).fun);
                fun_info = get_fun_itbl(UNTAG_CLOSURE((*ret_fun).fun));

                p = scavenge_arg_block(
                    fun_info,
                    &raw mut (*ret_fun).payload as *mut *mut StgClosure,
                );
            }
            _ => {
                barf(
                    b"scavenge_stack: weird activation record found on stack: %d\0" as *const u8
                        as *const c_char,
                    (*info).i.r#type as c_int,
                );
            }
        }

        if major_gc as c_int != 0 && (*info).i.srt != 0 {
            let mut srt = (info.offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*info).i.srt as StgWord)
                as *mut StgClosure;
            evacuate(&raw mut srt);
        }
    }
}

unsafe fn scavenge_large(mut ws: *mut gen_workspace) {
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();
    (*(&raw mut the_gc_thread as *mut gc_thread)).evac_gen_no = (*(*ws).0.r#gen).no;
    bd = (*ws).0.todo_large_objects;

    while !bd.is_null() {
        (*ws).0.todo_large_objects = (*bd).link as *mut bdescr;

        if (*bd).flags as c_int & BF_COMPACT != 0 {
            dbl_link_onto(bd, &raw mut (*(*ws).0.r#gen).live_compact_objects);

            let mut str =
                (*((*bd).start as *mut StgCompactNFDataBlock)).owner as *mut StgCompactNFData;
            (*(*ws).0.r#gen).n_live_compact_blocks = ((*(*ws).0.r#gen).n_live_compact_blocks
                as StgWord)
                .wrapping_add((*str).totalW.wrapping_div(BLOCK_SIZE_W as StgWord))
                as memcount as memcount;
            p = str as StgPtr;
        } else {
            dbl_link_onto(bd, &raw mut (*(*ws).0.r#gen).scavenged_large_objects);
            (*(*ws).0.r#gen).n_scavenged_large_blocks = (*(*ws).0.r#gen)
                .n_scavenged_large_blocks
                .wrapping_add((*bd).blocks as memcount);
            p = (*bd).start;
        }

        if scavenge_one(p) {
            if (*(*ws).0.r#gen).no > 0 as uint32_t {
                recordMutableGen_GC(p as *mut StgClosure, (*(*ws).0.r#gen).no);
            }
        }

        let ref mut fresh25 = (*(&raw mut the_gc_thread as *mut gc_thread)).scanned;
        *fresh25 = (*fresh25).wrapping_add(closure_sizeW(p as *mut StgClosure) as W_);
        bd = (*ws).0.todo_large_objects;
    }
}

unsafe fn scavenge_find_work() -> bool {
    let mut g: c_int = 0;
    let mut ws = null_mut::<gen_workspace>();
    let mut did_something: bool = false;
    let mut did_anything: bool = false;
    let mut bd = null_mut::<bdescr>();
    let ref mut fresh9 = (*(&raw mut the_gc_thread as *mut gc_thread)).scav_find_work;
    *fresh9 = (*fresh9).wrapping_add(1);
    did_anything = r#false != 0;

    loop {
        did_something = r#false != 0;
        g = RtsFlags.GcFlags.generations.wrapping_sub(1 as uint32_t) as c_int;

        while g >= 0 as c_int {
            ws = (&raw mut (*(&raw mut the_gc_thread as *mut gc_thread)).gens as *mut gen_workspace)
                .offset(g as isize) as *mut gen_workspace;

            if (*ws).0.todo_seg != END_NONMOVING_TODO_LIST {
                let mut seg = (*ws).0.todo_seg;
                (*ws).0.todo_seg = (*seg).todo_link;
                (*seg).todo_link = null_mut::<NonmovingSegment>();
                scavengeNonmovingSegment(seg);
                did_something = r#true != 0;
                break;
            } else {
                let ref mut fresh10 = (*(&raw mut the_gc_thread as *mut gc_thread)).scan_bd;
                *fresh10 = null_mut::<bdescr>();

                if (*(*ws).0.todo_bd).u.scan < (*ws).0.todo_free {
                    scavenge_block((*ws).0.todo_bd);
                    did_something = r#true != 0;
                    break;
                } else if !(*ws).0.todo_large_objects.is_null() {
                    scavenge_large(ws);
                    did_something = r#true != 0;
                    break;
                } else {
                    bd = grab_local_todo_block(ws);

                    if !bd.is_null() {
                        scavenge_block(bd);
                        did_something = r#true != 0;
                        break;
                    } else {
                        g -= 1;
                    }
                }
            }
        }

        if !did_something {
            break;
        }

        did_anything = r#true != 0;
    }

    return did_anything;
}

unsafe fn scavenge_loop() {
    let mut work_to_do: bool = false;

    loop {
        work_to_do = r#false != 0;

        if major_gc as c_int != 0
            && (*(&raw mut the_gc_thread as *mut gc_thread)).static_objects
                != static_flag as StgWord as *mut StgClosure
        {
            scavenge_static();
        }

        if !mark_stack_bd.is_null() && !mark_stack_empty() {
            scavenge_mark_stack();
            work_to_do = r#true != 0;
        }

        if scavenge_find_work() {
            continue;
        }

        if !work_to_do {
            break;
        }
    }
}
