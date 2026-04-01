use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK, MUT_ARR_PTRS_CARD_BITS};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::spin_lock::{ACQUIRE_SPIN_LOCK, RELEASE_SPIN_LOCK};
use crate::ffi::rts::storage::block::{BF_COMPACT, BLOCK_SIZE_W, Bdescr, bdescr, dbl_link_onto};
use crate::ffi::rts::storage::closure_macros::{
    LOOKS_LIKE_CLOSURE_PTR, STATIC_LINK, THUNK_SELECTOR_sizeW, UNTAG_CLOSURE, arr_words_sizeW,
    bco_sizeW, closure_sizeW, continuation_sizeW, get_fun_itbl, get_itbl, get_ret_itbl,
    itbl_to_fun_itbl, itbl_to_thunk_itbl, mut_arr_ptrs_sizeW, mutArrPtrsCard, mutArrPtrsCards,
    small_mut_arr_ptrs_sizeW, stack_sizeW,
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
    stg_COMPACT_NFDATA_DIRTY_info, stg_END_TSO_QUEUE_closure, stg_MUT_ARR_PTRS_CLEAN_info,
    stg_MUT_ARR_PTRS_DIRTY_info, stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    stg_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_MUT_VAR_CLEAN_info, stg_MUT_VAR_DIRTY_info,
    stg_MVAR_CLEAN_info, stg_MVAR_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_CLEAN_info,
    stg_SMALL_MUT_ARR_PTRS_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info, stg_TREC_HEADER_info, stg_TVAR_CLEAN_info,
    stg_TVAR_DIRTY_info, stg_TVAR_WATCH_QUEUE_info, stg_WHITEHOLE_info,
};
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgWord, StgWord8, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::hash::{HashTable, allocHashTable, freeHashTable, insertHashTable, mapHashTable};
use crate::io_manager::scavengeTSOIOManager;
use crate::prelude::*;
use crate::sm::evac::{evacuate_BLACKHOLE1, evacuate1};
use crate::sm::gc::{
    MutListScavStats, N, addMutListScavStats, deadlock_detect_gc, major_gc, mark_stack_bd,
    mutlist_scav_stats, work_stealing, zeroMutListScavStats,
};
use crate::sm::gc_thread::{gc_thread, gen_workspace};
use crate::sm::gc_utils::{
    freeChain_sync, grab_local_todo_block, push_scanned_block, recordMutableGen_GC,
    steal_todo_block,
};
use crate::sm::gct_decl::gct;
use crate::sm::heap_utils::walk_large_bitmap;
use crate::sm::mark_stack::{mark_stack_empty, pop_mark_stack};
use crate::sm::mark_weak::scavengeLiveWeak;
use crate::sm::non_moving::END_NONMOVING_TODO_LIST;
use crate::sm::non_moving_scav::{nonmovingScavengeOne, scavengeNonmovingSegment};
use crate::sm::sanity::checkStaticObjects;
use crate::sm::storage::{STATIC_BITS, static_flag};
use crate::trace::{DEBUG_RTS, trace_};

/// cbindgen:no-export
struct MapHashData {
    saved_gct: *mut gc_thread,
    newHash: *mut HashTable,
}

unsafe fn do_evacuate(mut p: *mut *mut StgClosure, mut user: *mut c_void) {
    evacuate1(p);
}

unsafe fn scavengeTSO1(mut tso: *mut StgTSO) {
    let mut saved_eager: bool = false;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"scavenging thread %llu".as_ptr(), (*tso).id);
    }

    if !(*tso).bound.is_null() {
        evacuate1(&raw mut (*(*tso).bound).tso as *mut *mut StgClosure);
    }

    saved_eager = (*gct).eager_promotion;
    (*gct).eager_promotion = false;
    evacuate1(&raw mut (*tso).blocked_exceptions as *mut *mut StgClosure);
    evacuate1(&raw mut (*tso).bq as *mut *mut StgClosure);
    evacuate1(&raw mut (*tso).trec as *mut *mut StgClosure);
    evacuate1(&raw mut (*tso).stackobj as *mut *mut StgClosure);
    evacuate1(&raw mut (*tso)._link as *mut *mut StgClosure);

    if !(*tso).label.is_null() {
        evacuate1(&raw mut (*tso).label as *mut *mut StgClosure);
    }

    match (&raw mut (*tso).why_blocked).load(Ordering::Acquire) {
        1 | 14 | 2 | 12 | 0 => {
            evacuate1(&raw mut (*tso).block_info.closure);
        }
        3 | 4 | 5 | 7 => {
            scavengeTSOIOManager(tso);
        }
        _ => {
            (*tso).block_info.closure =
                &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut StgClosure;
        }
    }

    (*tso).dirty = (*gct).failed_to_evac as StgWord32;
    (*gct).eager_promotion = saved_eager;
}

unsafe fn evacuate_hash_entry(
    mut dat: *mut MapHashData,
    mut key: StgWord,
    mut value: *const c_void,
) {
    let mut p = key as *mut StgClosure;
    let mut old_gct = gct;
    gct = (*dat).saved_gct;
    evacuate1(&raw mut p);
    insertHashTable((*dat).newHash, p as StgWord, value);
    gct = old_gct;
}

unsafe fn scavenge_compact1(mut str: *mut StgCompactNFData) {
    let mut saved_eager: bool = false;
    saved_eager = (*gct).eager_promotion;
    (*gct).eager_promotion = false;

    if !(*str).hash.is_null() {
        let mut dat = MapHashData {
            saved_gct: null_mut::<gc_thread>(),
            newHash: null_mut::<HashTable>(),
        };

        dat.saved_gct = gct;

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

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.compact as i64 != 0 {
        trace_(
            c"compact alive @%p, gen %d, %llu bytes".as_ptr(),
            str,
            (*Bdescr(str as StgPtr)).gen_no as i32,
            (*str).totalW.wrapping_mul(size_of::<W_>() as StgWord),
        );
    }

    (*gct).eager_promotion = saved_eager;

    if (*gct).failed_to_evac {
        (&raw mut (*(str as *mut StgClosure)).header.info)
            .store(&raw const stg_COMPACT_NFDATA_DIRTY_info, Ordering::Release);
    } else {
        (&raw mut (*(str as *mut StgClosure)).header.info)
            .store(&raw const stg_COMPACT_NFDATA_CLEAN_info, Ordering::Release);
    };
}

unsafe fn scavenge_mut_arr_ptrs1(mut a: *mut StgMutArrPtrs) -> StgPtr {
    let mut m: W_ = 0;
    let mut any_failed: bool = false;
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    any_failed = false;
    p = (&raw mut (*a).payload as *mut *mut StgClosure).offset(0) as *mut *mut StgClosure as StgPtr;
    m = 0;

    while (m as i32) < mutArrPtrsCards((*a).ptrs as W_) as i32 - 1 {
        q = p.offset((1 << MUT_ARR_PTRS_CARD_BITS) as isize);

        while p < q {
            evacuate1(p as *mut *mut StgClosure);
            p = p.offset(1);
        }

        if (*gct).failed_to_evac {
            any_failed = true;
            *mutArrPtrsCard(a, m) = 1;
            (*gct).failed_to_evac = false;
        } else {
            *mutArrPtrsCard(a, m) = 0;
        }

        m = m.wrapping_add(1);
    }

    q = (&raw mut (*a).payload as *mut *mut StgClosure).offset((*a).ptrs as isize)
        as *mut *mut StgClosure as StgPtr;

    if p < q {
        while p < q {
            evacuate1(p as *mut *mut StgClosure);
            p = p.offset(1);
        }

        if (*gct).failed_to_evac {
            any_failed = true;
            *mutArrPtrsCard(a, m) = 1;
            (*gct).failed_to_evac = false;
        } else {
            *mutArrPtrsCard(a, m) = 0;
        }
    }

    (*gct).failed_to_evac = any_failed;

    return (a as StgPtr).offset(mut_arr_ptrs_sizeW(a) as isize);
}

unsafe fn scavenge_mut_arr_ptrs_marked(mut a: *mut StgMutArrPtrs) -> StgPtr {
    let mut m: W_ = 0;
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut any_failed: bool = false;
    any_failed = false;
    m = 0;

    while m < mutArrPtrsCards((*a).ptrs as W_) {
        if *mutArrPtrsCard(a, m) as i32 != 0 {
            p = (&raw mut (*a).payload as *mut *mut StgClosure)
                .offset((m << MUT_ARR_PTRS_CARD_BITS) as isize)
                as *mut *mut StgClosure as StgPtr;

            q = ({
                let mut _a = p.offset((1 << 7) as isize);
                let mut _b = (&raw mut (*a).payload as *mut *mut StgClosure)
                    .offset((*a).ptrs as isize) as *mut *mut StgClosure
                    as StgPtr;

                if _a <= _b { _a as StgPtr } else { _b as StgPtr }
            });

            while p < q {
                evacuate1(p as *mut *mut StgClosure);
                p = p.offset(1);
            }

            if (*gct).failed_to_evac {
                any_failed = true;
                (*gct).failed_to_evac = false;
            } else {
                *mutArrPtrsCard(a, m) = 0;
            }
        }

        m = m.wrapping_add(1);
    }

    (*gct).failed_to_evac = any_failed;

    return (a as StgPtr).offset(mut_arr_ptrs_sizeW(a) as isize);
}

unsafe fn scavenge_small_bitmap(mut p: StgPtr, mut size: StgWord, mut bitmap: StgWord) -> StgPtr {
    while size > 0 {
        if bitmap & 1 == 0 {
            evacuate1(p as *mut *mut StgClosure);
        }

        p = p.offset(1);
        bitmap = bitmap >> 1;
        size = size.wrapping_sub(1);
    }

    return p;
}

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
            current_block_8 = 7025704749916336511;
        }
        1 => {
            size = (*((fun_info.offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                as *mut StgLargeBitmap))
                .size;

            scavenge_large_bitmap(
                p,
                (fun_info.offset(1 as i32 as isize) as StgWord)
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
            current_block_8 = 7025704749916336511;
        }
    }

    match current_block_8 {
        7025704749916336511 => {
            p = scavenge_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

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

    if ((*fun_info).i.r#type != 25) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/sm/Scav.c".as_ptr(), 360);
    }

    p = payload as StgPtr;

    let mut current_block_12: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            current_block_12 = 15915979825911306861;
        }
        1 => {
            scavenge_large_bitmap(
                p,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_12 = 17407779659766490442;
        }
        2 => {
            scavenge_large_bitmap(
                payload as StgPtr,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_12 = 17407779659766490442;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;
            current_block_12 = 15915979825911306861;
        }
    }

    match current_block_12 {
        15915979825911306861 => {
            p = scavenge_small_bitmap(p, size, bitmap);
        }
        _ => {}
    }

    return p;
}

unsafe fn scavenge_PAP1(mut pap: *mut StgPAP) -> StgPtr {
    evacuate1(&raw mut (*pap).fun);

    return scavenge_PAP_payload(
        (*pap).fun,
        &raw mut (*pap).payload as *mut *mut StgClosure,
        (*pap).n_args as StgWord,
    );
}

unsafe fn scavenge_AP1(mut ap: *mut StgAP) -> StgPtr {
    evacuate1(&raw mut (*ap).fun);

    return scavenge_PAP_payload(
        (*ap).fun,
        &raw mut (*ap).payload as *mut *mut StgClosure,
        (*ap).n_args as StgWord,
    );
}

unsafe fn scavenge_continuation1(mut cont: *mut StgContinuation) -> StgPtr {
    scavenge_stack1(
        &raw mut (*cont).stack as StgPtr,
        (&raw mut (*cont).stack as *mut StgWord).offset((*cont).stack_size as isize),
    );

    return (cont as StgPtr).offset(continuation_sizeW(cont) as isize);
}

unsafe fn scavenge_thunk_srt1(mut info: *const StgInfoTable) {
    let mut thunk_info = null_mut::<StgThunkInfoTable>();

    if !major_gc {
        return;
    }

    thunk_info = itbl_to_thunk_itbl(info);

    if (*thunk_info).i.srt != 0 {
        let mut srt = (thunk_info.offset(1 as i32 as isize) as StgWord)
            .wrapping_add((*thunk_info).i.srt as StgWord) as *mut StgClosure;
        evacuate1(&raw mut srt);
    }
}

unsafe fn scavenge_fun_srt1(mut info: *const StgInfoTable) {
    let mut fun_info = null_mut::<StgFunInfoTable>();

    if !major_gc {
        return;
    }

    fun_info = itbl_to_fun_itbl(info);

    if (*fun_info).i.srt != 0 {
        let mut srt = (fun_info.offset(1 as i32 as isize) as StgWord)
            .wrapping_add((*fun_info).i.srt as StgWord) as *mut StgClosure;
        evacuate1(&raw mut srt);
    }
}

unsafe fn scavenge_block1(mut bd: *mut bdescr) {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut info = null::<StgInfoTable>();
    let mut saved_eager_promotion: bool = false;
    let mut ws = null_mut::<gen_workspace>();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"scavenging block %p (gen %d) @ %p".as_ptr(),
            (*bd).start,
            (*bd).gen_no as i32,
            (*bd).u.scan,
        );
    }

    (*gct).scan_bd = bd;
    (*gct).evac_gen_no = (*bd).gen_no as u32;
    saved_eager_promotion = (*gct).eager_promotion;
    (*gct).failed_to_evac = false;
    ws = (&raw mut (*gct).gens as *mut gen_workspace).offset((*bd).gen_no as isize)
        as *mut gen_workspace;
    p = (*bd).u.scan;

    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && deadlock_detect_gc as i32 != 0 {
        if ((*bd).r#gen == oldest_gen) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 472);
        }
    }

    while p < (*bd).c2rust_unnamed.free || bd == (*ws).0.todo_bd && p < (*ws).0.todo_free {
        if (*bd).link.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 482);
        }

        if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 483);
        }

        info = get_itbl(p as *mut StgClosure);

        if ((*gct).thunk_selector_depth == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 486);
        }

        q = p;

        let mut current_block_192: u64;

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).value);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_CLEAN_info, Ordering::Release);
                }

                p = p.offset(
                    (size_of::<StgMVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tvar).current_value);

                evacuate1(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);

                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_CLEAN_info, Ordering::Release);
                }

                p = p.offset(
                    (size_of::<StgTVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            11 => {
                scavenge_fun_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            18 => {
                scavenge_thunk_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            4 => {
                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            16 => {
                scavenge_thunk_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            9 => {
                scavenge_fun_srt1(info);
                current_block_192 = 9318790868267028162;
            }
            2 => {
                current_block_192 = 9318790868267028162;
            }
            17 => {
                scavenge_thunk_srt1(info);
                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            10 => {
                scavenge_fun_srt1(info);
                current_block_192 = 2239666984073767338;
            }
            3 => {
                current_block_192 = 2239666984073767338;
            }
            20 => {
                scavenge_thunk_srt1(info);
                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            13 => {
                scavenge_fun_srt1(info);
                current_block_192 = 6887952387645651966;
            }
            6 => {
                current_block_192 = 6887952387645651966;
            }
            19 => {
                scavenge_thunk_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgThunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            12 => {
                scavenge_fun_srt1(info);
                current_block_192 = 17256551721821101503;
            }
            5 => {
                current_block_192 = 17256551721821101503;
            }
            8 => {
                scavenge_fun_srt1(info);
                current_block_192 = 9199578309995299736;
            }
            15 => {
                let mut end = null_mut::<StgWord>();
                scavenge_thunk_srt1(info);
                end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
                current_block_192 = 17518666747792551745;
            }
            1 | 7 | 49 | 50 => {
                current_block_192 = 9199578309995299736;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate1(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                p = p.offset(bco_sizeW(bco) as isize);
                current_block_192 = 17518666747792551745;
            }
            38 => {
                evacuate1(&raw mut (*(p as *mut StgInd)).indirectee);
                p = p.offset(
                    (size_of::<StgInd>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            47 | 48 => {
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*(p as *mut StgMutVar)).var);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_CLEAN_info, Ordering::Release);
                }

                p = p.offset(
                    (size_of::<StgMutVar>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*bq).bh);
                evacuate1(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).link as *mut *mut StgClosure);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_CLEAN_info, Ordering::Release);
                }

                p = p.offset(
                    (size_of::<StgBlockingQueue>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate1(&raw mut (*s).selectee);
                p = p.offset(THUNK_SELECTOR_sizeW() as isize);
                current_block_192 = 17518666747792551745;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate1(&raw mut (*ap).fun);

                scavenge_stack1(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                p = (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap).size as isize);
                current_block_192 = 17518666747792551745;
            }
            25 => {
                p = scavenge_PAP1(p as *mut StgPAP);
                current_block_192 = 17518666747792551745;
            }
            24 => {
                p = scavenge_AP1(p as *mut StgAP);
                current_block_192 = 17518666747792551745;
            }
            42 => {
                p = p.offset(arr_words_sizeW(p as *mut StgArrBytes) as isize);
                current_block_192 = 17518666747792551745;
            }
            43 | 44 => {
                (*gct).eager_promotion = false;
                p = scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_CLEAN_info, Ordering::Release);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                current_block_192 = 17518666747792551745;
            }
            46 | 45 => {
                p = scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                current_block_192 = 17518666747792551745;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                (*gct).eager_promotion = false;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info,
                        Ordering::Release,
                    );
                }

                (*gct).failed_to_evac = true;
                current_block_192 = 17518666747792551745;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                current_block_192 = 17518666747792551745;
            }
            52 => {
                scavengeTSO1(p as *mut StgTSO);
                p = p.offset(
                    (size_of::<StgTSO>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*gct).eager_promotion = false;

                scavenge_stack1(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty = (*gct).failed_to_evac as StgWord8;
                p = p.offset(stack_sizeW(stack) as isize);
                (*gct).eager_promotion = saved_eager_promotion;
                current_block_192 = 17518666747792551745;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*gct).eager_promotion = false;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                current_block_192 = 17518666747792551745;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry =
                    (&raw mut (*tc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0;

                while i < (*tc).next_entry_idx {
                    evacuate1(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate1(&raw mut (*e).expected_value);
                    evacuate1(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                p = p.offset(
                    (size_of::<StgTRecChunk>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                current_block_192 = 17518666747792551745;
            }
            64 => {
                p = scavenge_continuation1(p as *mut StgContinuation);
                current_block_192 = 17518666747792551745;
            }
            _ => {
                barf(
                    c"scavenge: unimplemented/strange closure type %d @ %p".as_ptr(),
                    (*info).r#type,
                    p,
                );
            }
        }

        match current_block_192 {
            9199578309995299736 => {
                let mut end_0 = null_mut::<StgWord>();
                end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_0 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                p = p.offset((*info).layout.payload.nptrs as isize);
            }
            17256551721821101503 => {
                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(2 as usize) as isize,
                );
            }
            9318790868267028162 => {
                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );
            }
            2239666984073767338 => {
                p = p.offset(
                    (size_of::<StgHeader>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize)
                        .wrapping_add(1 as usize) as isize,
                );
            }
            6887952387645651966 => {
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

        if (*gct).failed_to_evac {
            (*gct).failed_to_evac = false;

            if (*bd).gen_no as i32 > 0 {
                recordMutableGen_GC(q as *mut StgClosure, (*bd).gen_no as u32);
            }
        }
    }

    if p > (*bd).c2rust_unnamed.free {
        (*gct).copied = (*gct)
            .copied
            .wrapping_add((*ws).0.todo_free.offset_from((*bd).c2rust_unnamed.free) as i64 as W_);

        (&raw mut (*bd).c2rust_unnamed.free).store(p, Ordering::Release);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(
            c"   scavenged %ld bytes".as_ptr(),
            ((*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as i64 as usize)
                .wrapping_mul(size_of::<W_>() as usize) as u64,
        );
    }

    (*gct).scanned = (*gct)
        .scanned
        .wrapping_add((*bd).c2rust_unnamed.free.offset_from((*bd).u.scan) as i64 as W_);
    (*bd).u.scan = (*bd).c2rust_unnamed.free;

    if bd != (*ws).0.todo_bd {
        push_scanned_block(bd, ws);
    }

    (*gct).scan_bd = null_mut::<bdescr>();
}

unsafe fn scavenge_mark_stack() {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();
    let mut info = null::<StgInfoTable>();
    let mut saved_eager_promotion: bool = false;
    (*gct).evac_gen_no = (*oldest_gen).no;
    saved_eager_promotion = (*gct).eager_promotion;

    loop {
        p = pop_mark_stack();

        if p.is_null() {
            break;
        }

        if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 914);
        }

        info = get_itbl(p as *mut StgClosure);
        q = p;

        let mut current_block_144: u64;

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).value);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_CLEAN_info, Ordering::Release);
                }

                current_block_144 = 5250576585193495047;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tvar).current_value);

                evacuate1(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);

                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_CLEAN_info, Ordering::Release);
                }

                current_block_144 = 5250576585193495047;
            }
            11 => {
                scavenge_fun_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            18 => {
                scavenge_thunk_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            4 => {
                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(1)
                        as *mut *mut StgClosure,
                );

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            9 | 12 => {
                scavenge_fun_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            16 | 19 => {
                scavenge_thunk_srt1(info);

                evacuate1(
                    (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            2 | 5 => {
                evacuate1(
                    (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_).offset(0)
                        as *mut *mut StgClosure,
                );

                current_block_144 = 5250576585193495047;
            }
            10 | 13 => {
                scavenge_fun_srt1(info);
                current_block_144 = 5250576585193495047;
            }
            17 | 20 => {
                scavenge_thunk_srt1(info);
                current_block_144 = 5250576585193495047;
            }
            8 => {
                scavenge_fun_srt1(info);
                current_block_144 = 7990025728955927862;
            }
            15 => {
                let mut end = null_mut::<StgWord>();
                scavenge_thunk_srt1(info);
                end = (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                current_block_144 = 5250576585193495047;
            }
            1 | 7 | 49 | 50 => {
                current_block_144 = 7990025728955927862;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate1(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                current_block_144 = 5250576585193495047;
            }
            27 | 38 => {
                evacuate1(&raw mut (*(p as *mut StgInd)).indirectee);
                current_block_144 = 5250576585193495047;
            }
            47 | 48 => {
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*(p as *mut StgMutVar)).var);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_CLEAN_info, Ordering::Release);
                }

                current_block_144 = 5250576585193495047;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*bq).bh);
                evacuate1(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).link as *mut *mut StgClosure);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_CLEAN_info, Ordering::Release);
                }

                current_block_144 = 5250576585193495047;
            }
            3 | 6 | 42 => {
                current_block_144 = 5250576585193495047;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate1(&raw mut (*s).selectee);
                current_block_144 = 5250576585193495047;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate1(&raw mut (*ap).fun);

                scavenge_stack1(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                current_block_144 = 5250576585193495047;
            }
            25 => {
                scavenge_PAP1(p as *mut StgPAP);
                current_block_144 = 5250576585193495047;
            }
            24 => {
                scavenge_AP1(p as *mut StgAP);
                current_block_144 = 5250576585193495047;
            }
            43 | 44 => {
                (*gct).eager_promotion = false;
                scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_CLEAN_info, Ordering::Release);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                current_block_144 = 5250576585193495047;
            }
            46 | 45 => {
                let mut q_0 = p;
                scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(q_0 as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q_0 as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                current_block_144 = 5250576585193495047;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                let mut saved_eager: bool = false;
                saved_eager = (*gct).eager_promotion;
                (*gct).eager_promotion = false;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*gct).eager_promotion = saved_eager;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info,
                        Ordering::Release,
                    );
                }

                (*gct).failed_to_evac = true;
                current_block_144 = 5250576585193495047;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                let mut q_1 = p;
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*gct).failed_to_evac {
                    (&raw mut (*(q_1 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q_1 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                current_block_144 = 5250576585193495047;
            }
            52 => {
                scavengeTSO1(p as *mut StgTSO);
                current_block_144 = 5250576585193495047;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*gct).eager_promotion = false;

                scavenge_stack1(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty = (*gct).failed_to_evac as StgWord8;
                (*gct).eager_promotion = saved_eager_promotion;
                current_block_144 = 5250576585193495047;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*gct).eager_promotion = false;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                current_block_144 = 5250576585193495047;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry =
                    (&raw mut (*tc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0;

                while i < (*tc).next_entry_idx {
                    evacuate1(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate1(&raw mut (*e).expected_value);
                    evacuate1(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                current_block_144 = 5250576585193495047;
            }
            64 => {
                scavenge_continuation1(p as *mut StgContinuation);
                current_block_144 = 5250576585193495047;
            }
            _ => {
                barf(
                    c"scavenge_mark_stack: unimplemented/strange closure type %d @ %p".as_ptr(),
                    (*info).r#type,
                    p,
                );
            }
        }

        match current_block_144 {
            7990025728955927862 => {
                let mut end_0 = null_mut::<StgWord>();
                end_0 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_0 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }
            }
            _ => {}
        }

        if (*gct).failed_to_evac {
            (*gct).failed_to_evac = false;

            if (*gct).evac_gen_no != 0 {
                recordMutableGen_GC(q as *mut StgClosure, (*gct).evac_gen_no);
            }
        }
    }
}

unsafe fn scavenge_one(mut p: StgPtr) -> bool {
    let mut info = null::<StgInfoTable>();
    let mut no_luck: bool = false;
    let mut saved_eager_promotion: bool = false;

    loop {
        saved_eager_promotion = (*gct).eager_promotion;

        if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 1282);
        }

        info = get_itbl(p as *mut StgClosure);

        match (*info).r#type {
            39 | 40 => {
                let mut mvar = p as *mut StgMVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*mvar).head as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).tail as *mut *mut StgClosure);
                evacuate1(&raw mut (*mvar).value);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*mvar).header.info)
                        .store(&raw const stg_MVAR_CLEAN_info, Ordering::Release);
                }

                break;
            }
            41 => {
                let mut tvar = p as *mut StgTVar;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tvar).current_value);

                evacuate1(&raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure);

                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*tvar).header.info)
                        .store(&raw const stg_TVAR_CLEAN_info, Ordering::Release);
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
                    evacuate1(q as *mut *mut StgClosure);
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
                    evacuate1(q_0 as *mut *mut StgClosure);
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
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*(p as *mut StgMutVar)).var);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q_1 as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(q_1 as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_VAR_CLEAN_info, Ordering::Release);
                }

                break;
            }
            37 => {
                let mut bq = p as *mut StgBlockingQueue;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*bq).bh);
                evacuate1(&raw mut (*bq).owner as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).queue as *mut *mut StgClosure);
                evacuate1(&raw mut (*bq).link as *mut *mut StgClosure);
                (*gct).eager_promotion = saved_eager_promotion;

                if (*gct).failed_to_evac {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*bq).header.info)
                        .store(&raw const stg_BLOCKING_QUEUE_CLEAN_info, Ordering::Release);
                }

                break;
            }
            22 => {
                let mut s = p as *mut StgSelector;
                evacuate1(&raw mut (*s).selectee);
                break;
            }
            26 => {
                let mut ap = p as *mut StgAP_STACK;
                evacuate1(&raw mut (*ap).fun);

                scavenge_stack1(
                    &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                    (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                        .offset((*ap).size as isize),
                );

                p = (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                    .offset((*ap).size as isize);
                break;
            }
            25 => {
                p = scavenge_PAP1(p as *mut StgPAP);
                break;
            }
            24 => {
                p = scavenge_AP1(p as *mut StgAP);
                break;
            }
            42 => {
                break;
            }
            43 | 44 => {
                (*gct).eager_promotion = false;
                scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(p as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_DIRTY_info, Ordering::Release);
                } else {
                    (&raw mut (*(p as *mut StgClosure)).header.info)
                        .store(&raw const stg_MUT_ARR_PTRS_CLEAN_info, Ordering::Release);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                break;
            }
            46 | 45 => {
                scavenge_mut_arr_ptrs1(p as *mut StgMutArrPtrs);

                if (*gct).failed_to_evac {
                    (&raw mut (*(p as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(p as *mut StgClosure)).header.info).store(
                        &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                break;
            }
            59 | 60 => {
                let mut next = null_mut::<StgWord>();
                let mut q_2 = null_mut::<StgWord>();
                let mut saved_eager: bool = false;
                saved_eager = (*gct).eager_promotion;
                (*gct).eager_promotion = false;
                q_2 = p;
                next = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*gct).eager_promotion = saved_eager;

                if (*gct).failed_to_evac {
                    (&raw mut (*(q_2 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q_2 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_CLEAN_info,
                        Ordering::Release,
                    );
                }

                (*gct).failed_to_evac = true;
                break;
            }
            62 | 61 => {
                let mut next_0 = null_mut::<StgWord>();
                let mut q_3 = p;
                next_0 = p.offset(small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as isize);

                p = &raw mut (*(p as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure as P_
                    as StgPtr;

                while p < next_0 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                if (*gct).failed_to_evac {
                    (&raw mut (*(q_3 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_DIRTY_info,
                        Ordering::Release,
                    );
                } else {
                    (&raw mut (*(q_3 as *mut StgClosure)).header.info).store(
                        &raw const stg_SMALL_MUT_ARR_PTRS_FROZEN_CLEAN_info,
                        Ordering::Release,
                    );
                }

                break;
            }
            52 => {
                scavengeTSO1(p as *mut StgTSO);
                break;
            }
            53 => {
                let mut stack = p as *mut StgStack;
                (*gct).eager_promotion = false;

                scavenge_stack1(
                    (*stack).sp,
                    (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
                );

                (*stack).dirty = (*gct).failed_to_evac as StgWord8;
                (*gct).eager_promotion = saved_eager_promotion;
                break;
            }
            51 => {
                let mut end_1 = null_mut::<StgWord>();
                (*gct).eager_promotion = false;
                end_1 = (&raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                p = &raw mut (*(p as *mut StgClosure)).payload as *mut *mut StgClosure_ as P_
                    as StgPtr;

                while p < end_1 {
                    evacuate1(p as *mut *mut StgClosure);
                    p = p.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                break;
            }
            54 => {
                let mut i: StgWord = 0;
                let mut tc = p as *mut StgTRecChunk;
                let mut e: *mut TRecEntry =
                    (&raw mut (*tc).entries as *mut TRecEntry).offset(0) as *mut TRecEntry;
                (*gct).eager_promotion = false;
                evacuate1(&raw mut (*tc).prev_chunk as *mut *mut StgClosure);
                i = 0;

                while i < (*tc).next_entry_idx {
                    evacuate1(&raw mut (*e).tvar as *mut *mut StgClosure);
                    evacuate1(&raw mut (*e).expected_value);
                    evacuate1(&raw mut (*e).new_value);
                    i = i.wrapping_add(1);
                    e = e.offset(1);
                }

                (*gct).eager_promotion = saved_eager_promotion;
                (*gct).failed_to_evac = true;
                break;
            }
            27 | 38 | 28 => {
                evacuate1(&raw mut (*(p as *mut StgInd)).indirectee);
                break;
            }
            23 => {
                let mut bco = p as *mut StgBCO;
                evacuate1(&raw mut (*bco).instrs as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).literals as *mut *mut StgClosure);
                evacuate1(&raw mut (*bco).ptrs as *mut *mut StgClosure);
                break;
            }
            63 => {
                scavenge_compact1(p as *mut StgCompactNFData);
                break;
            }
            64 => {
                scavenge_continuation1(p as *mut StgContinuation);
                break;
            }
            58 => while get_itbl(p as *mut StgClosure) == &raw const stg_WHITEHOLE_info {},
            _ => {
                barf(
                    c"scavenge_one: strange object %d".as_ptr(),
                    (*info).r#type as i32,
                );
            }
        }
    }

    no_luck = (*gct).failed_to_evac;
    (*gct).failed_to_evac = false;

    return no_luck;
}

unsafe fn scavenge_mutable_list1(mut bd: *mut bdescr, mut r#gen: *mut generation) {
    let mut p = null_mut::<StgWord>();
    let mut q = null_mut::<StgWord>();

    let mut stats = MutListScavStats {
        n_MUTVAR: 0,
        n_MUTARR: 0,
        n_MVAR: 0,
        n_TVAR: 0,
        n_TREC_CHUNK: 0,
        n_TVAR_WATCH_QUEUE: 0,
        n_TREC_HEADER: 0,
        n_OTHERS: 0,
    };

    zeroMutListScavStats(&raw mut stats);

    let mut gen_no: u32 = (*r#gen).no;
    (*gct).evac_gen_no = gen_no;

    while !bd.is_null() {
        q = (*bd).start;

        while q < (*bd).c2rust_unnamed.free {
            p = *q as StgPtr;

            if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/sm/Scav.c".as_ptr(), 1660);
            }

            let mut pinfo = null::<StgInfoTable>();

            match (*get_itbl(p as *mut StgClosure)).r#type {
                47 | 48 => {
                    stats.n_MUTVAR = stats.n_MUTVAR.wrapping_add(1);
                }
                43 | 44 | 46 | 45 => {
                    stats.n_MUTARR = stats.n_MUTARR.wrapping_add(1);
                }
                39 => {
                    barf(c"MVAR_CLEAN on mutable list".as_ptr());
                }
                40 => {
                    stats.n_MVAR = stats.n_MVAR.wrapping_add(1);
                }
                41 => {
                    stats.n_TVAR = stats.n_TVAR.wrapping_add(1);
                }
                54 => {
                    stats.n_TREC_CHUNK = stats.n_TREC_CHUNK.wrapping_add(1);
                }
                51 => {
                    pinfo = (*(p as *mut StgClosure)).header.info;

                    if pinfo == &raw const stg_TVAR_WATCH_QUEUE_info {
                        stats.n_TVAR_WATCH_QUEUE = stats.n_TVAR_WATCH_QUEUE.wrapping_add(1);
                    } else if pinfo == &raw const stg_TREC_HEADER_info {
                        stats.n_TREC_HEADER = stats.n_TREC_HEADER.wrapping_add(1);
                    } else {
                        stats.n_OTHERS = stats.n_OTHERS.wrapping_add(1);
                    }
                }
                _ => {
                    stats.n_OTHERS = stats.n_OTHERS.wrapping_add(1);
                }
            }

            match (*get_itbl(p as *mut StgClosure)).r#type {
                43 | 59 => {
                    recordMutableGen_GC(p as *mut StgClosure, gen_no);
                }
                44 => {
                    let mut saved_eager_promotion: bool = false;
                    saved_eager_promotion = (*gct).eager_promotion;
                    (*gct).eager_promotion = false;
                    scavenge_mut_arr_ptrs_marked(p as *mut StgMutArrPtrs);

                    if (*gct).failed_to_evac {
                        (&raw mut (*(p as *mut StgClosure)).header.info)
                            .store(&raw const stg_MUT_ARR_PTRS_DIRTY_info, Ordering::Release);
                    } else {
                        (&raw mut (*(p as *mut StgClosure)).header.info)
                            .store(&raw const stg_MUT_ARR_PTRS_CLEAN_info, Ordering::Release);
                    }

                    (*gct).eager_promotion = saved_eager_promotion;
                    (*gct).failed_to_evac = false;
                    recordMutableGen_GC(p as *mut StgClosure, gen_no);
                }
                _ => {
                    if RtsFlags.GcFlags.useNonmoving as i32 != 0
                        && major_gc as i32 != 0
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

    ACQUIRE_SPIN_LOCK(&raw mut (*oldest_gen).sync);
    addMutListScavStats(&raw mut stats, &raw mut mutlist_scav_stats);
    RELEASE_SPIN_LOCK(&raw mut (*oldest_gen).sync);
}

unsafe fn scavenge_capability_mut_Lists1(mut cap: *mut Capability) {
    if RtsFlags.GcFlags.useNonmoving as i32 != 0 && major_gc as i32 != 0 {
        let mut g: u32 = (*oldest_gen).no;
        scavenge_mutable_list1(*(*cap).saved_mut_lists.offset(g as isize), oldest_gen);
        freeChain_sync(*(*cap).saved_mut_lists.offset(g as isize));

        let ref mut fresh50 = *(*cap).saved_mut_lists.offset(g as isize);
        *fresh50 = null_mut::<bdescr>();
        return;
    }

    let mut g_0: u32 = RtsFlags.GcFlags.generations.wrapping_sub(1 as u32);

    while g_0 > N {
        scavenge_mutable_list1(
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

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.gc as i64 != 0 {
        trace_(c"scavenging static objects".as_ptr());
    }

    (*gct).evac_gen_no = (*oldest_gen).no;

    loop {
        flagged_p = (*gct).static_objects;

        if flagged_p == static_flag as StgWord as *mut StgClosure {
            break;
        }

        p = (flagged_p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;

        if LOOKS_LIKE_CLOSURE_PTR(p as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 1810);
        }

        info = get_itbl(p);

        let mut link = STATIC_LINK(info, p);
        (*gct).static_objects = (link).load(Ordering::Relaxed);
        (link).store((*gct).scavenged_static_objects, Ordering::Relaxed);
        (*gct).scavenged_static_objects = flagged_p;

        let mut current_block_27: u64;

        match (*info).r#type {
            28 => {
                let mut ind = p as *mut StgInd;
                evacuate1(&raw mut (*ind).indirectee);

                if (*gct).failed_to_evac {
                    (*gct).failed_to_evac = false;
                    recordMutableGen_GC(p, (*oldest_gen).no);
                }

                current_block_27 = 11636175345244025579;
            }
            21 => {
                scavenge_thunk_srt1(info);
                current_block_27 = 11636175345244025579;
            }
            14 => {
                scavenge_fun_srt1(info);
                current_block_27 = 11307063007268554308;
            }
            1 | 7 | 2 | 3 | 4 | 5 | 6 => {
                current_block_27 = 11307063007268554308;
            }
            _ => {
                barf(
                    c"scavenge_static: strange closure %d".as_ptr(),
                    (*info).r#type as i32,
                );
            }
        }

        match current_block_27 {
            11307063007268554308 => {
                let mut q = null_mut::<StgWord>();
                let mut next = null_mut::<StgWord>();
                next = (&raw mut (*p).payload as *mut *mut StgClosure_ as P_)
                    .offset((*info).layout.payload.ptrs as isize) as StgPtr;
                q = &raw mut (*p).payload as *mut *mut StgClosure_ as P_ as StgPtr;

                while q < next {
                    evacuate1(q as *mut *mut StgClosure);
                    q = q.offset(1);
                }
            }
            _ => {}
        }

        if ((*gct).failed_to_evac as i32 == 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/Scav.c".as_ptr(), 1875);
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

unsafe fn scavenge_stack1(mut p: StgPtr, mut stack_end: StgPtr) {
    let mut info = null::<StgRetInfoTable>();
    let mut bitmap: StgWord = 0;
    let mut size: StgWord = 0;

    while p < stack_end {
        info = get_ret_itbl(p as *mut StgClosure);

        match (*info).i.r#type {
            33 => {
                let mut frame = p as *mut StgUpdateFrame;
                evacuate_BLACKHOLE1(&raw mut (*frame).updatee);
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
                evacuate1(p as *mut *mut StgClosure);
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
                size_1 = (*(((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap))
                    .size;
                p = p.offset(1);

                scavenge_large_bitmap(
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
                let mut fun_info = null::<StgFunInfoTable>();
                evacuate1(&raw mut (*ret_fun).fun);
                fun_info = get_fun_itbl(UNTAG_CLOSURE((*ret_fun).fun));

                p = scavenge_arg_block(
                    fun_info,
                    &raw mut (*ret_fun).payload as *mut *mut StgClosure,
                );
            }
            _ => {
                barf(
                    c"scavenge_stack: weird activation record found on stack: %d".as_ptr(),
                    (*info).i.r#type as i32,
                );
            }
        }

        if major_gc as i32 != 0 && (*info).i.srt != 0 {
            let mut srt = (info.offset(1 as i32 as isize) as StgWord)
                .wrapping_add((*info).i.srt as StgWord)
                as *mut StgClosure;
            evacuate1(&raw mut srt);
        }
    }
}

unsafe fn scavenge_large(mut ws: *mut gen_workspace) {
    let mut bd = null_mut::<bdescr>();
    let mut p = null_mut::<StgWord>();
    (*gct).evac_gen_no = (*(*ws).0.r#gen).no;
    bd = (*ws).0.todo_large_objects;

    while !bd.is_null() {
        (*ws).0.todo_large_objects = (*bd).link as *mut bdescr;
        ACQUIRE_SPIN_LOCK(&raw mut (*(*ws).0.r#gen).sync);

        if (*bd).flags as i32 & BF_COMPACT != 0 {
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

        RELEASE_SPIN_LOCK(&raw mut (*(*ws).0.r#gen).sync);

        if scavenge_one(p) {
            if (*(*ws).0.r#gen).no > 0 {
                recordMutableGen_GC(p as *mut StgClosure, (*(*ws).0.r#gen).no);
            }
        }

        (*gct).scanned = (*gct)
            .scanned
            .wrapping_add(closure_sizeW(p as *mut StgClosure) as W_);
        bd = (*ws).0.todo_large_objects;
    }
}

unsafe fn scavenge_find_work() -> bool {
    let mut g: i32 = 0;
    let mut ws = null_mut::<gen_workspace>();
    let mut did_something: bool = false;
    let mut did_anything: bool = false;
    let mut bd = null_mut::<bdescr>();
    (*gct).scav_find_work = (*gct).scav_find_work.wrapping_add(1);
    did_anything = false;

    loop {
        did_something = false;
        g = RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) as i32;

        while g >= 0 {
            ws = (&raw mut (*gct).gens as *mut gen_workspace).offset(g as isize)
                as *mut gen_workspace;

            if (*ws).0.todo_seg != END_NONMOVING_TODO_LIST {
                let mut seg = (*ws).0.todo_seg;

                if !(*seg).todo_link.is_null() as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/sm/Scav.c".as_ptr(), 2142);
                }

                (*ws).0.todo_seg = (*seg).todo_link;
                (*seg).todo_link = null_mut::<NonmovingSegment>();
                scavengeNonmovingSegment(seg);
                did_something = true;
                break;
            } else {
                (*gct).scan_bd = null_mut::<bdescr>();

                if (*(*ws).0.todo_bd).u.scan < (*ws).0.todo_free {
                    scavenge_block1((*ws).0.todo_bd);
                    did_something = true;
                    break;
                } else if !(*ws).0.todo_large_objects.is_null() {
                    scavenge_large(ws);
                    did_something = true;
                    break;
                } else {
                    bd = grab_local_todo_block(ws);

                    if !bd.is_null() {
                        scavenge_block1(bd);
                        did_something = true;
                        break;
                    } else {
                        g -= 1;
                    }
                }
            }
        }

        if did_something {
            did_anything = true;
        } else {
            if !work_stealing {
                break;
            }

            g = RtsFlags.GcFlags.generations.wrapping_sub(1 as u32) as i32;

            while g >= 0 {
                bd = steal_todo_block(g as u32);

                if !bd.is_null() {
                    scavenge_block1(bd);
                    did_something = true;
                    break;
                } else {
                    g -= 1;
                }
            }

            if !did_something {
                break;
            }

            did_anything = true;
        }
    }

    return did_anything;
}

unsafe fn scavenge_loop1() {
    let mut work_to_do: bool = false;

    loop {
        work_to_do = false;

        if major_gc as i32 != 0
            && (*gct).static_objects != static_flag as StgWord as *mut StgClosure
        {
            if RtsFlags.DebugFlags.sanity {
                checkStaticObjects((*gct).static_objects);
            }

            scavenge_static();
        }

        if !mark_stack_bd.is_null() && !mark_stack_empty() {
            scavenge_mark_stack();
            work_to_do = true;
        }

        if scavenge_find_work() {
            continue;
        }

        if !work_to_do {
            break;
        }
    }
}
