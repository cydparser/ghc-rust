use crate::capability::getCapability;
use crate::disassembler::disassemble;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::ipe::{InfoProv_, InfoProvEnt_, lookupIPE};
use crate::ffi::rts::messages::{barf, debugBelch};
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::closure_macros::{
    GET_INFO, LOOKS_LIKE_CLOSURE_PTR, STATIC_LINK, UNTAG_CLOSURE, UNTAG_CONST_CLOSURE,
    arr_words_words, closure_sizeW, get_con_itbl, get_fun_itbl, get_itbl, itbl_to_fun_itbl,
    stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgAnnFrame, StgArrBytes, StgAtomicallyFrame, StgBCO, StgCatchFrame,
    StgCatchRetryFrame, StgCatchSTMFrame, StgClosure_, StgCompactNFData, StgContinuation, StgInd,
    StgMVar, StgMutArrPtrs, StgMutVar, StgPAP, StgRetFun, StgSelector, StgSmallMutArrPtrs,
    StgStopFrame, StgTVar, StgThunk, StgUnderflowFrame, StgUpdateFrame, StgWeak,
};
use crate::ffi::rts::storage::gc::{generation, generations, oldest_gen};
use crate::ffi::rts::storage::info_tables::{StgLargeBitmap, stg_arg_bitmaps};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::stg::misc_closures::{
    stg_BCO_info, stg_END_TSO_QUEUE_closure, stg_ap_d_info, stg_ap_f_info, stg_ap_l_info,
    stg_ap_n_info, stg_ap_p_info, stg_ap_pp_info, stg_ap_ppp_info, stg_ap_pppp_info,
    stg_ap_ppppp_info, stg_ap_pppppp_info, stg_ap_v_info, stg_apply_interp_info,
    stg_bh_upd_frame_info, stg_ctoi_D1_info, stg_ctoi_F1_info, stg_ctoi_R1n_info,
    stg_ctoi_R1p_info, stg_ctoi_V_info, stg_ctoi_t0_info, stg_ctoi_t1_info, stg_ctoi_t2_info,
    stg_ctoi_t3_info, stg_ctoi_t4_info, stg_ctoi_t5_info, stg_ctoi_t6_info, stg_ctoi_t7_info,
    stg_ctoi_t8_info, stg_marked_upd_frame_info, stg_prompt_frame_info, stg_restore_cccs_d_info,
    stg_restore_cccs_eval_info, stg_restore_cccs_v16_info, stg_restore_cccs_v32_info,
    stg_restore_cccs_v64_info, stg_ret_d_info, stg_ret_f_info, stg_ret_l_info, stg_ret_n_info,
    stg_ret_p_info, stg_ret_t_info, stg_ret_v_info, stg_upd_frame_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::ffi::stg::{BITS_PER_BYTE, P_, W_};
use crate::hash::{HashTable, lookupHashTable};
use crate::prelude::*;
use crate::profiling::fprintCCS;
use crate::sm::gc_thread::{gc_threads, gen_workspace};
use crate::sm::storage::{STATIC_BITS, static_flag};

unsafe fn printPtr(mut p: StgPtr) {
    let mut raw = null::<c_char>();
    raw = lookupGHCName(p as *mut c_void);

    if !raw.is_null() {
        debugBelch(c"<%s>".as_ptr(), raw);
        debugBelch(c"[%p]".as_ptr(), p);
    } else {
        debugBelch(c"%p".as_ptr(), p);
    };
}

unsafe fn printObj(mut obj: *mut StgClosure) {
    debugBelch(c"Object ".as_ptr());
    printPtr(obj as StgPtr);
    debugBelch(c" = ".as_ptr());
    printClosure(obj);
}

unsafe fn printStdObjHdr(mut obj: *const StgClosure, mut tag: *mut c_char) {
    debugBelch(c"%s(".as_ptr(), tag);
    printPtr((*obj).header.info as StgPtr);
    debugBelch(c", %s".as_ptr(), (*(*(*obj).header.prof.ccs).cc).label);
}

unsafe fn printStdObjPayload(mut obj: *const StgClosure) {
    let mut i: StgWord = 0;
    let mut j: StgWord = 0;
    let mut info = null::<StgInfoTable>();
    info = get_itbl(obj);
    i = 0;

    while i < (*info).layout.payload.ptrs as StgWord {
        debugBelch(c", ".as_ptr());

        printPtr(
            *(&raw const (*obj).payload as *const *mut StgClosure_).offset(i as isize) as StgPtr,
        );

        i = i.wrapping_add(1);
    }

    j = 0;

    while j < (*info).layout.payload.nptrs as StgWord {
        debugBelch(
            c", %pd#".as_ptr(),
            *(&raw const (*obj).payload as *const *mut StgClosure_)
                .offset(i.wrapping_add(j) as isize),
        );

        j = j.wrapping_add(1);
    }

    debugBelch(c")\n".as_ptr());
}

unsafe fn printThunkPayload(mut obj: *mut StgThunk) {
    let mut i: StgWord = 0;
    let mut j: StgWord = 0;
    let mut info = null::<StgInfoTable>();
    info = get_itbl(obj as *mut StgClosure);
    i = 0;

    while i < (*info).layout.payload.ptrs as StgWord {
        debugBelch(c", ".as_ptr());

        printPtr(*(&raw mut (*obj).payload as *mut *mut StgClosure_).offset(i as isize) as StgPtr);

        i = i.wrapping_add(1);
    }

    j = 0;

    while j < (*info).layout.payload.nptrs as StgWord {
        debugBelch(
            c", %pd#".as_ptr(),
            *(&raw mut (*obj).payload as *mut *mut StgClosure_).offset(i.wrapping_add(j) as isize),
        );

        j = j.wrapping_add(1);
    }

    debugBelch(c")\n".as_ptr());
}

unsafe fn printThunkObject(mut obj: *mut StgThunk, mut tag: *mut c_char) {
    printStdObjHdr(obj as *mut StgClosure, tag);
    printThunkPayload(obj);
}

unsafe fn printClosure(mut obj: *const StgClosure) {
    debugBelch(c"%p: ".as_ptr(), obj);
    obj = UNTAG_CONST_CLOSURE(obj);

    let mut info = get_itbl(obj);

    while info as StgWord & 1 != 0 {
        obj = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure;
        debugBelch(c"(forwarding to %p) ".as_ptr(), obj as *mut c_void);
        info = get_itbl(obj);
    }

    match (*info).r#type {
        0 => {
            barf(c"Invalid object".as_ptr());
        }
        1 | 2 | 3 | 5 | 6 | 4 | 7 => {
            let mut i: StgWord = 0;
            let mut j: StgWord = 0;
            let mut con_info = get_con_itbl(obj);

            debugBelch(
                c"%s(".as_ptr(),
                (con_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*con_info).con_desc as StgWord) as *const c_char,
            );

            i = 0;

            while i < (*info).layout.payload.ptrs as StgWord {
                if i != 0 {
                    debugBelch(c", ".as_ptr());
                }

                printPtr(
                    *(&raw const (*obj).payload as *const *mut StgClosure_).offset(i as isize)
                        as StgPtr,
                );

                i = i.wrapping_add(1);
            }

            j = 0;

            while j < (*info).layout.payload.nptrs as StgWord {
                if i != 0 || j != 0 {
                    debugBelch(c", ".as_ptr());
                }

                debugBelch(
                    c"%p#".as_ptr(),
                    *(&raw const (*obj).payload as *const *mut StgClosure_)
                        .offset(i.wrapping_add(j) as isize),
                );

                j = j.wrapping_add(1);
            }

            debugBelch(c")\n".as_ptr());
        }
        8 | 9 | 10 | 12 | 13 | 11 | 14 => {
            debugBelch(
                c"FUN/%d(".as_ptr(),
                (*itbl_to_fun_itbl(info)).f.arity as i32,
            );
            printPtr((*obj).header.info as StgPtr);

            let mut ipe = InfoProvEnt_ {
                info: null::<StgInfoTable>(),
                prov: InfoProv_ {
                    table_name: null::<c_char>(),
                    closure_desc: 0,
                    ty_desc: null::<c_char>(),
                    label: null::<c_char>(),
                    unit_id: null::<c_char>(),
                    module: null::<c_char>(),
                    src_file: null::<c_char>(),
                    src_span: null::<c_char>(),
                },
            };

            if lookupIPE((*obj).header.info, &raw mut ipe) {
                debugBelch(c", %s".as_ptr(), ipe.prov.table_name);
            }

            debugBelch(c", %s".as_ptr(), (*(*(*obj).header.prof.ccs).cc).label);
            printStdObjPayload(obj);
        }
        50 => {
            debugBelch(c"PRIM(".as_ptr());
            printPtr((*obj).header.info as StgPtr);
            printStdObjPayload(obj);
        }
        51 => {
            debugBelch(c"MUT_PRIM(".as_ptr());
            printPtr((*obj).header.info as StgPtr);
            printStdObjPayload(obj);
        }
        15 | 16 | 17 | 19 | 20 | 18 | 21 => {
            printThunkObject(
                obj as *mut StgThunk,
                (info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).prof.closure_desc_off as StgWord)
                    as *mut c_char,
            );
        }
        22 => {
            printStdObjHdr(obj, c"THUNK_SELECTOR".as_ptr());
            debugBelch(c", %p)\n".as_ptr(), (*(obj as *mut StgSelector)).selectee);
        }
        23 => {
            disassemble(obj as *mut StgBCO);
        }
        24 => {
            let mut ap = obj as *mut StgAP;
            let mut i_0: StgWord = 0;
            debugBelch(c"AP(".as_ptr());
            printPtr((*ap).fun as StgPtr);
            i_0 = 0;

            while i_0 < (*ap).n_args as StgWord {
                debugBelch(c", ".as_ptr());

                printPtr(
                    *(&raw mut (*ap).payload as *mut *mut StgClosure).offset(i_0 as isize)
                        as StgPtr,
                );

                i_0 = i_0.wrapping_add(1);
            }

            debugBelch(c")\n".as_ptr());
        }
        25 => {
            let mut pap = obj as *mut StgPAP;
            let mut i_1: StgWord = 0;
            debugBelch(c"PAP/%d(".as_ptr(), (*pap).arity as i32);
            printPtr((*pap).fun as StgPtr);
            i_1 = 0;

            while i_1 < (*pap).n_args as StgWord {
                debugBelch(c", ".as_ptr());

                printPtr(
                    *(&raw mut (*pap).payload as *mut *mut StgClosure).offset(i_1 as isize)
                        as StgPtr,
                );

                i_1 = i_1.wrapping_add(1);
            }

            debugBelch(c")\n".as_ptr());
        }
        26 => {
            let mut ap_0 = obj as *mut StgAP_STACK;
            let mut i_2: StgWord = 0;
            debugBelch(c"AP_STACK(".as_ptr());
            printPtr((*ap_0).fun as StgPtr);
            i_2 = 0;

            while i_2 < (*ap_0).size {
                debugBelch(c", ".as_ptr());

                printPtr(
                    *(&raw mut (*ap_0).payload as *mut *mut StgClosure).offset(i_2 as isize)
                        as StgPtr,
                );

                i_2 = i_2.wrapping_add(1);
            }

            debugBelch(c")\n".as_ptr());
        }
        27 => {
            debugBelch(c"IND(".as_ptr());
            printPtr((*(obj as *mut StgInd)).indirectee as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        28 => {
            debugBelch(c"IND_STATIC(".as_ptr());
            printPtr((*(obj as *mut StgInd)).indirectee as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        38 => {
            debugBelch(c"BLACKHOLE(".as_ptr());
            printPtr((*(obj as *mut StgInd)).indirectee as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        65 => {
            let mut frame = obj as *mut StgAnnFrame;
            debugBelch(c"ANN_FRAME(".as_ptr());
            printPtr(GET_INFO(frame as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame).ann as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        33 => {
            let mut frame_0 = obj as *mut StgUpdateFrame;
            debugBelch(c"%s(".as_ptr(), info_update_frame(obj));
            printPtr(GET_INFO(frame_0 as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_0).updatee as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        34 => {
            let mut frame_1 = obj as *mut StgCatchFrame;
            debugBelch(c"CATCH_FRAME(".as_ptr());
            printPtr(GET_INFO(frame_1 as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_1).handler as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        35 => {
            let mut frame_2 = obj as *mut StgUnderflowFrame;
            debugBelch(c"UNDERFLOW_FRAME(".as_ptr());
            printPtr((*frame_2).next_chunk as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        36 => {
            let mut frame_3 = obj as *mut StgStopFrame;
            debugBelch(c"STOP_FRAME(".as_ptr());
            printPtr(GET_INFO(frame_3 as *mut StgClosure) as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        55 => {
            let mut frame_4 = obj as *mut StgAtomicallyFrame;
            debugBelch(c"ATOMICALLY_FRAME(".as_ptr());
            printPtr(GET_INFO(frame_4 as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_4).code as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_4).result as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        56 => {
            let mut frame_5 = obj as *mut StgCatchRetryFrame;
            debugBelch(c"CATCH_RETRY_FRAME(".as_ptr());
            printPtr(GET_INFO(frame_5 as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_5).first_code as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_5).alt_code as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        57 => {
            let mut frame_6 = obj as *mut StgCatchSTMFrame;
            debugBelch(c"CATCH_STM_FRAME(".as_ptr());
            printPtr(GET_INFO(frame_6 as *mut StgClosure) as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_6).code as StgPtr);
            debugBelch(c",".as_ptr());
            printPtr((*frame_6).handler as StgPtr);
            debugBelch(c")\n".as_ptr());
        }
        42 => {
            let mut i_3: StgWord = 0;
            debugBelch(c"ARR_WORDS(\"".as_ptr());
            i_3 = 0;

            while i_3 < arr_words_words(obj as *mut StgArrBytes) {
                debugBelch(
                    c"%llu".as_ptr(),
                    *(&raw mut (*(obj as *mut StgArrBytes)).payload as *mut StgWord)
                        .offset(i_3 as isize),
                );

                i_3 = i_3.wrapping_add(1);
            }

            debugBelch(c"\")\n".as_ptr());
        }
        43 => {
            debugBelch(
                c"MUT_ARR_PTRS_CLEAN(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgMutArrPtrs)).ptrs,
            );
        }
        44 => {
            debugBelch(
                c"MUT_ARR_PTRS_DIRTY(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgMutArrPtrs)).ptrs,
            );
        }
        46 => {
            debugBelch(
                c"MUT_ARR_PTRS_FROZEN_CLEAN(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgMutArrPtrs)).ptrs,
            );
        }
        45 => {
            debugBelch(
                c"MUT_ARR_PTRS_FROZEN_DIRTY(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgMutArrPtrs)).ptrs,
            );
        }
        59 => {
            debugBelch(
                c"SMALL_MUT_ARR_PTRS_CLEAN(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgSmallMutArrPtrs)).ptrs,
            );
        }
        60 => {
            debugBelch(
                c"SMALL_MUT_ARR_PTRS_DIRTY(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgSmallMutArrPtrs)).ptrs,
            );
        }
        62 => {
            debugBelch(
                c"SMALL_MUT_ARR_PTRS_FROZEN_CLEAN(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgSmallMutArrPtrs)).ptrs,
            );
        }
        61 => {
            debugBelch(
                c"SMALL_MUT_ARR_PTRS_FROZEN_DIRTY(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgSmallMutArrPtrs)).ptrs,
            );
        }
        39 | 40 => {
            let mut mv = obj as *mut StgMVar;
            debugBelch(c"MVAR(head=".as_ptr());

            if (*mv).head as *mut StgClosure == &raw mut stg_END_TSO_QUEUE_closure {
                debugBelch(c"END_TSO_QUEUE".as_ptr());
            } else {
                debugBelch(c"%p".as_ptr(), (*mv).head);
            }

            debugBelch(c", tail=".as_ptr());

            if (*mv).tail as *mut StgClosure == &raw mut stg_END_TSO_QUEUE_closure {
                debugBelch(c"END_TSO_QUEUE".as_ptr());
            } else {
                debugBelch(c"%p".as_ptr(), (*mv).tail);
            }

            debugBelch(c", value=".as_ptr());

            if (*mv).value == &raw mut stg_END_TSO_QUEUE_closure {
                debugBelch(c"END_TSO_QUEUE".as_ptr());
            } else {
                debugBelch(c"%p".as_ptr(), (*mv).value);
            }

            debugBelch(c")\n".as_ptr());
        }
        41 => {
            let mut tv = obj as *mut StgTVar;

            debugBelch(
                c"TVAR(value=%p, wq=%p, num_updates=%llu)\n".as_ptr(),
                (*tv).current_value,
                (*tv).first_watch_queue_entry,
                (*tv).num_updates,
            );
        }
        47 => {
            let mut mv_0 = obj as *mut StgMutVar;
            debugBelch(c"MUT_VAR_CLEAN(var=%p)\n".as_ptr(), (*mv_0).var);
        }
        48 => {
            let mut mv_1 = obj as *mut StgMutVar;
            debugBelch(c"MUT_VAR_DIRTY(var=%p)\n".as_ptr(), (*mv_1).var);
        }
        49 => {
            debugBelch(c"WEAK(".as_ptr());

            debugBelch(
                c"key=%p value=%p finalizer=%p".as_ptr(),
                (*(obj as *mut StgWeak)).key as StgPtr,
                (*(obj as *mut StgWeak)).value as StgPtr,
                (*(obj as *mut StgWeak)).finalizer as StgPtr,
            );

            debugBelch(c")\n".as_ptr());
        }
        52 => {
            debugBelch(c"TSO(".as_ptr());

            debugBelch(
                c"%lu (%p)".as_ptr(),
                (*(obj as *mut StgTSO)).id as u64,
                obj as *mut StgTSO,
            );

            debugBelch(c")\n".as_ptr());
        }
        53 => {
            debugBelch(c"STACK\n".as_ptr());
        }
        63 => {
            debugBelch(
                c"COMPACT_NFDATA(size=%llu)\n".as_ptr(),
                (*(obj as *mut StgCompactNFData))
                    .totalW
                    .wrapping_mul(size_of::<W_>() as W_),
            );
        }
        54 => {
            debugBelch(c"TREC_CHUNK\n".as_ptr());
        }
        64 => {
            let mut u = obj as *mut StgContinuation;
            debugBelch(c"CONTINUATION(apply_mask_frame=".as_ptr());
            printPtr((*u).apply_mask_frame as StgPtr);
            debugBelch(c",stack_size=%llu)\n".as_ptr(), (*u).stack_size);
        }
        _ => {
            debugBelch(
                c"*** printClosure: unknown type %d ****\n".as_ptr(),
                (*get_itbl(obj)).r#type as i32,
            );

            barf(c"printClosure %d".as_ptr(), (*get_itbl(obj)).r#type);
        }
    };
}

unsafe fn printMutableList(mut bd: *mut bdescr) {
    let mut p = null_mut::<StgWord>();
    debugBelch(c"mutable list %p: ".as_ptr(), bd);

    while !bd.is_null() {
        p = (*bd).start;

        while p < (*bd).c2rust_unnamed.free {
            debugBelch(
                c"%p (%s), ".as_ptr(),
                *p as *mut c_void,
                info_type(*p as *mut StgClosure),
            );

            p = p.offset(1);
        }

        bd = (*bd).link as *mut bdescr;
    }

    debugBelch(c"\n".as_ptr());
}

unsafe fn info_update_frame(mut closure: *const StgClosure) -> *const c_char {
    let mut info = (*closure).header.info;

    if info == &raw const stg_upd_frame_info {
        return c"NORMAL_UPDATE_FRAME".as_ptr();
    } else if info == &raw const stg_bh_upd_frame_info {
        return c"BH_UPDATE_FRAME".as_ptr();
    } else if info == &raw const stg_marked_upd_frame_info {
        return c"MARKED_UPDATE_FRAME".as_ptr();
    } else {
        return c"ERROR: Not an update frame!!!".as_ptr();
    };
}

unsafe fn printSmallBitmap(
    mut spBottom: StgPtr,
    mut payload: StgPtr,
    mut bitmap: StgWord,
    mut size: u32,
) {
    let mut i: u32 = 0;
    i = 0;

    while i < size {
        debugBelch(
            c"   stk[%ld] (%p) = ".as_ptr(),
            spBottom.offset_from(payload.offset(i as isize)) as i64,
            payload.offset(i as isize),
        );

        if bitmap & 1 == 0 {
            printPtr(*payload.offset(i as isize) as StgPtr);
            debugBelch(c" -- ".as_ptr());
            printObj(*payload.offset(i as isize) as *mut StgClosure);
        } else {
            debugBelch(c"Word# %llu\n".as_ptr(), *payload.offset(i as isize));
        }

        i = i.wrapping_add(1);
        bitmap >>= 1;
    }
}

unsafe fn printLargeBitmap(
    mut spBottom: StgPtr,
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
            debugBelch(
                c"   stk[%llu] (%p) = ".as_ptr(),
                spBottom.offset_from(payload.offset(i as isize)) as i64 as W_,
                payload.offset(i as isize),
            );

            if bitmap & 1 == 0 {
                printPtr(*payload.offset(i as isize) as StgPtr);
                debugBelch(c" -- ".as_ptr());
                printObj(*payload.offset(i as isize) as *mut StgClosure);
            } else {
                debugBelch(c"Word# %llu\n".as_ptr(), *payload.offset(i as isize));
            }

            j = j.wrapping_add(1);
            i = i.wrapping_add(1);
            bitmap >>= 1;
        }

        bmp = bmp.wrapping_add(1);
    }
}

unsafe fn printStackChunk(mut sp: StgPtr, mut spBottom: StgPtr) {
    let mut info = null::<StgInfoTable>();

    if (sp <= spBottom) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Printer.c".as_ptr(), 597);
    }

    let mut bitmap_0: *mut StgLargeBitmap = null_mut::<StgLargeBitmap>();
    let mut current_block_126: u64;

    while sp < spBottom {
        info = get_itbl(sp as *mut StgClosure);

        match (*info).r#type {
            33 | 34 | 35 | 36 | 55 | 56 | 57 => {
                printClosure(sp as *mut StgClosure);
            }
            30 => {
                let mut c: StgWord = *sp;

                if c == &raw const stg_ap_v_info as StgWord {
                    debugBelch(c"stg_ap_v_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_f_info as StgWord {
                    debugBelch(c"stg_ap_f_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_d_info as StgWord {
                    debugBelch(c"stg_ap_d_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_l_info as StgWord {
                    debugBelch(c"stg_ap_l_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_n_info as StgWord {
                    debugBelch(c"stg_ap_n_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_p_info as StgWord {
                    debugBelch(c"stg_ap_p_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_pp_info as StgWord {
                    debugBelch(c"stg_ap_pp_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_ppp_info as StgWord {
                    debugBelch(c"stg_ap_ppp_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_pppp_info as StgWord {
                    debugBelch(c"stg_ap_pppp_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_ppppp_info as StgWord {
                    debugBelch(c"stg_ap_ppppp_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ap_pppppp_info as StgWord {
                    debugBelch(c"stg_ap_pppppp_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_v_info as StgWord {
                    debugBelch(c"stg_ret_v_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_p_info as StgWord {
                    debugBelch(c"stg_ret_p_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_n_info as StgWord {
                    debugBelch(c"stg_ret_n_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_f_info as StgWord {
                    debugBelch(c"stg_ret_f_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_d_info as StgWord {
                    debugBelch(c"stg_ret_d_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_ret_l_info as StgWord {
                    debugBelch(c"stg_ret_l_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_prompt_frame_info as StgWord {
                    debugBelch(c"stg_prompt_frame_info\n".as_ptr());
                    current_block_126 = 16029476503615101993;
                } else if c == &raw const stg_restore_cccs_d_info as StgWord {
                    debugBelch(c"stg_restore_cccs_d_info\n".as_ptr());
                    fprintCCS(__stderrp, *sp.offset(1) as *mut CostCentreStack);
                    debugBelch(c"\n".as_ptr());
                    current_block_126 = 7095457783677275021;
                } else if c == &raw const stg_restore_cccs_v16_info as StgWord {
                    debugBelch(c"stg_restore_cccs_v16_info\n".as_ptr());
                    fprintCCS(__stderrp, *sp.offset(1) as *mut CostCentreStack);
                    debugBelch(c"\n".as_ptr());
                    current_block_126 = 7095457783677275021;
                } else if c == &raw const stg_restore_cccs_v32_info as StgWord {
                    debugBelch(c"stg_restore_cccs_v32_info\n".as_ptr());
                    fprintCCS(__stderrp, *sp.offset(1) as *mut CostCentreStack);
                    debugBelch(c"\n".as_ptr());
                    current_block_126 = 7095457783677275021;
                } else if c == &raw const stg_restore_cccs_v64_info as StgWord {
                    debugBelch(c"stg_restore_cccs_v64_info\n".as_ptr());
                    fprintCCS(__stderrp, *sp.offset(1) as *mut CostCentreStack);
                    debugBelch(c"\n".as_ptr());
                    current_block_126 = 7095457783677275021;
                } else if c == &raw const stg_restore_cccs_eval_info as StgWord {
                    debugBelch(c"stg_restore_cccs_eval_info\n".as_ptr());
                    fprintCCS(__stderrp, *sp.offset(1) as *mut CostCentreStack);
                    debugBelch(c"\n".as_ptr());
                    current_block_126 = 7095457783677275021;
                } else {
                    debugBelch(c"RET_SMALL (%p)\n".as_ptr(), info);
                    current_block_126 = 16029476503615101993;
                }

                match current_block_126 {
                    7095457783677275021 => {}
                    _ => {
                        let mut bitmap: StgWord = (*info).layout.bitmap;

                        printSmallBitmap(
                            spBottom,
                            sp.offset(1),
                            bitmap >> BITMAP_BITS_SHIFT,
                            (bitmap & BITMAP_SIZE_MASK as StgWord) as u32,
                        );
                    }
                }
            }
            29 => {
                let mut c_0: StgWord = *sp;
                let mut bco = *sp.offset(1) as *mut StgBCO;

                if c_0 == &raw const stg_ctoi_R1p_info as StgWord {
                    debugBelch(c"stg_ctoi_R1p_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_R1n_info as StgWord {
                    debugBelch(c"stg_ctoi_R1n_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_F1_info as StgWord {
                    debugBelch(c"stg_ctoi_F1_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_D1_info as StgWord {
                    debugBelch(c"stg_ctoi_D1_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_V_info as StgWord {
                    debugBelch(c"stg_ctoi_V_info".as_ptr());
                } else if c_0 == &raw const stg_BCO_info as StgWord {
                    debugBelch(c"stg_BCO_info".as_ptr());
                } else if c_0 == &raw const stg_apply_interp_info as StgWord {
                    debugBelch(c"stg_apply_interp_info".as_ptr());
                } else if c_0 == &raw const stg_ret_t_info as StgWord {
                    debugBelch(c"stg_ret_t_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t0_info as StgWord {
                    debugBelch(c"stg_ctoi_t0_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t1_info as StgWord {
                    debugBelch(c"stg_ctoi_t1_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t2_info as StgWord {
                    debugBelch(c"stg_ctoi_t2_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t3_info as StgWord {
                    debugBelch(c"stg_ctoi_t3_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t4_info as StgWord {
                    debugBelch(c"stg_ctoi_t4_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t5_info as StgWord {
                    debugBelch(c"stg_ctoi_t5_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t6_info as StgWord {
                    debugBelch(c"stg_ctoi_t6_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t7_info as StgWord {
                    debugBelch(c"stg_ctoi_t7_info".as_ptr());
                } else if c_0 == &raw const stg_ctoi_t8_info as StgWord {
                    debugBelch(c"stg_ctoi_t8_info".as_ptr());
                } else {
                    debugBelch(c"RET_BCO".as_ptr());
                }

                debugBelch(c" (%p)\n".as_ptr(), sp);

                printLargeBitmap(
                    spBottom,
                    sp.offset(2),
                    &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                    (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size as u32,
                );
            }
            31 => {
                debugBelch(c"RET_BIG (%p)\n".as_ptr(), sp);
                bitmap_0 = (info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap;

                printLargeBitmap(
                    spBottom,
                    &raw mut (*(sp as *mut StgClosure)).payload as *mut *mut StgClosure_ as StgPtr,
                    bitmap_0,
                    (*bitmap_0).size as u32,
                );
            }
            32 => {
                let mut fun_info = null::<StgFunInfoTable>();
                let mut ret_fun = null_mut::<StgRetFun>();
                ret_fun = sp as *mut StgRetFun;
                fun_info = get_fun_itbl(UNTAG_CLOSURE((*ret_fun).fun));

                debugBelch(
                    c"RET_FUN (%p) (type=%d)\n".as_ptr(),
                    (*ret_fun).fun,
                    (*fun_info).f.fun_type as i32,
                );

                match (*fun_info).f.fun_type {
                    0 => {
                        printSmallBitmap(
                            spBottom,
                            &raw mut (*ret_fun).payload as StgPtr,
                            (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT,
                            ((*fun_info).f.b.bitmap & BITMAP_SIZE_MASK as StgWord) as u32,
                        );
                    }
                    1 => {
                        printLargeBitmap(
                            spBottom,
                            &raw mut (*ret_fun).payload as StgPtr,
                            (fun_info.offset(1 as i32 as isize) as StgWord)
                                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                                as *mut StgLargeBitmap,
                            (*((fun_info.offset(1 as i32 as isize) as StgWord)
                                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                                as *mut StgLargeBitmap))
                                .size as u32,
                        );
                    }
                    _ => {
                        printSmallBitmap(
                            spBottom,
                            &raw mut (*ret_fun).payload as StgPtr,
                            *(&raw const stg_arg_bitmaps as *const StgWord)
                                .offset((*fun_info).f.fun_type as isize)
                                >> BITMAP_BITS_SHIFT,
                            (*(&raw const stg_arg_bitmaps as *const StgWord)
                                .offset((*fun_info).f.fun_type as isize)
                                & BITMAP_SIZE_MASK as StgWord) as u32,
                        );
                    }
                }
            }
            _ => {
                debugBelch(c"unknown object %d\n".as_ptr(), (*info).r#type as i32);
                barf(c"printStackChunk".as_ptr());
            }
        }

        sp = sp.offset(stack_frame_sizeW(sp as *mut StgClosure) as isize);
    }
}

unsafe fn printStack(mut stack: *mut StgStack) {
    printStackChunk(
        (*stack).sp,
        (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
    );
}

unsafe fn printTSO(mut tso: *mut StgTSO) {
    printStack((*tso).stackobj as *mut StgStack);
}

unsafe fn printStaticObjects(mut p: *mut StgClosure) {
    while p != static_flag as StgWord as *mut StgClosure {
        p = (p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;
        printClosure(p);

        let mut info = get_itbl(p);
        p = *STATIC_LINK(info, p);
    }
}

unsafe fn printWeakLists() {
    debugBelch(c"======= WEAK LISTS =======\n".as_ptr());

    let mut cap_idx: u32 = 0;

    while cap_idx < getNumCapabilities() as u32 {
        debugBelch(c"Capability %d:\n".as_ptr(), cap_idx);

        let mut cap = getCapability(cap_idx);
        let mut weak = (*cap).weak_ptr_list_hd;

        while !weak.is_null() {
            printClosure(weak as *mut StgClosure);
            weak = (*weak).link as *mut StgWeak;
        }

        cap_idx = cap_idx.wrapping_add(1);
    }

    let mut gen_idx: u32 = 0;

    while gen_idx <= (*oldest_gen).no {
        let mut r#gen: *mut generation = generations.offset(gen_idx as isize) as *mut generation;
        debugBelch(c"Generation %d current weaks:\n".as_ptr(), gen_idx);

        let mut weak_0 = (*r#gen).weak_ptr_list;

        while !weak_0.is_null() {
            printClosure(weak_0 as *mut StgClosure);
            weak_0 = (*weak_0).link as *mut StgWeak;
        }

        debugBelch(c"Generation %d old weaks:\n".as_ptr(), gen_idx);

        let mut weak_1 = (*r#gen).old_weak_ptr_list;

        while !weak_1.is_null() {
            printClosure(weak_1 as *mut StgClosure);
            weak_1 = (*weak_1).link as *mut StgWeak;
        }

        gen_idx = gen_idx.wrapping_add(1);
    }

    debugBelch(c"=========================\n".as_ptr());
}

unsafe fn printLargeAndPinnedObjects() {
    debugBelch(c"====== PINNED OBJECTS ======\n".as_ptr());

    let mut cap_idx: u32 = 0;

    while cap_idx < getNumCapabilities() as u32 {
        let mut cap = getCapability(cap_idx);

        debugBelch(
            c"Capability %d: Current pinned object block: %p\n".as_ptr(),
            cap_idx,
            (*cap).pinned_object_block as *mut c_void,
        );

        let mut bd = (*cap).pinned_object_blocks;

        while !bd.is_null() {
            debugBelch(c"%p\n".as_ptr(), bd as *mut c_void);
            bd = (*bd).link as *mut bdescr;
        }

        cap_idx = cap_idx.wrapping_add(1);
    }

    debugBelch(c"====== LARGE OBJECTS =======\n".as_ptr());

    let mut gen_idx: u32 = 0;

    while gen_idx <= (*oldest_gen).no {
        let mut r#gen: *mut generation = generations.offset(gen_idx as isize) as *mut generation;
        debugBelch(c"Generation %d current large objects:\n".as_ptr(), gen_idx);

        let mut bd_0 = (*r#gen).large_objects;

        while !bd_0.is_null() {
            debugBelch(c"%p: ".as_ptr(), bd_0 as *mut c_void);
            printClosure((*bd_0).start as *mut StgClosure);
            bd_0 = (*bd_0).link as *mut bdescr;
        }

        debugBelch(
            c"Generation %d scavenged large objects:\n".as_ptr(),
            gen_idx,
        );

        let mut bd_1 = (*r#gen).scavenged_large_objects;

        while !bd_1.is_null() {
            debugBelch(c"%p: ".as_ptr(), bd_1 as *mut c_void);
            printClosure((*bd_1).start as *mut StgClosure);
            bd_1 = (*bd_1).link as *mut bdescr;
        }

        gen_idx = gen_idx.wrapping_add(1);
    }

    debugBelch(c"============================\n".as_ptr());
}

static mut add_to_fname_table: *mut HashTable = null_mut::<HashTable>();

unsafe fn lookupGHCName(mut addr: *mut c_void) -> *const c_char {
    if add_to_fname_table.is_null() {
        return null::<c_char>();
    }

    return lookupHashTable(add_to_fname_table, addr as StgWord) as *const c_char;
}

unsafe fn DEBUG_LoadSymbols(mut name: *const c_char) {}

static mut searched: i32 = 0;

unsafe fn findPtrBlocks(
    mut p: StgPtr,
    mut bd: *mut bdescr,
    mut arr: *mut StgPtr,
    mut arr_size: i32,
    mut i: i32,
) -> i32 {
    let mut q = null_mut::<StgWord>();
    let mut r = null_mut::<StgWord>();
    let mut end = null_mut::<StgWord>();

    while !bd.is_null() {
        searched += 1;
        q = (*bd).start;

        while q < (*bd).c2rust_unnamed.free {
            if UNTAG_CONST_CLOSURE(*q as *mut StgClosure) == p as *const StgClosure {
                if i < arr_size {
                    r = (*bd).start;

                    while r < (*bd).c2rust_unnamed.free {
                        while *r == 0 {
                            r = r.offset(1);
                        }

                        if !LOOKS_LIKE_CLOSURE_PTR(r as *const c_void) {
                            debugBelch(c"%p found at %p, no closure at %p\n".as_ptr(), p, q, r);

                            break;
                        } else {
                            end = r.offset(closure_sizeW(r as *mut StgClosure) as isize);

                            if q < end {
                                debugBelch(c"%p = ".as_ptr(), r);
                                printClosure(r as *mut StgClosure);

                                let fresh13 = i;
                                i = i + 1;

                                let ref mut fresh14 = *arr.offset(fresh13 as isize);
                                *fresh14 = r;
                                break;
                            } else {
                                r = end;
                            }
                        }
                    }

                    if r >= (*bd).c2rust_unnamed.free {
                        debugBelch(c"%p found at %p, closure?".as_ptr(), p, q);
                    }
                } else {
                    return i;
                }
            }

            q = q.offset(1);
        }

        bd = (*bd).link as *mut bdescr;
    }

    return i;
}

unsafe fn findPtr(mut p: P_, mut follow: i32) {
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    let arr_size = 1024;
    let vla = arr_size as usize;
    let mut arr: Vec<StgPtr> = ::std::vec::from_elem(null_mut::<StgWord>(), vla);
    let mut i = 0;
    searched = 0;
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        bd = (*generations.offset(g as isize)).blocks;
        i = findPtrBlocks(p as StgPtr, bd, arr.as_mut_ptr(), arr_size, i);
        bd = (*generations.offset(g as isize)).large_objects;
        i = findPtrBlocks(p as StgPtr, bd, arr.as_mut_ptr(), arr_size, i);

        if i >= arr_size {
            return;
        }

        n = 0;

        while n < getNumCapabilities() as u32 {
            i = findPtrBlocks(
                p as StgPtr,
                (*(&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .part_list,
                arr.as_mut_ptr(),
                arr_size,
                i,
            );

            i = findPtrBlocks(
                p as StgPtr,
                (*(&raw mut (**gc_threads.offset(n as isize)).gens as *mut gen_workspace)
                    .offset(g as isize))
                .0
                .todo_bd,
                arr.as_mut_ptr(),
                arr_size,
                i,
            );

            n = n.wrapping_add(1);
        }

        if i >= arr_size {
            return;
        }

        g = g.wrapping_add(1);
    }

    if follow != 0 && i == 1 {
        debugBelch(c"-->\n".as_ptr());
        findPtr(*arr.as_mut_ptr().offset(0), 1);
    }
}

static mut what_next_strs: [*const c_char; 5] = [
    c"(unknown)".as_ptr(),
    c"ThreadRunGHC".as_ptr(),
    c"ThreadInterpret".as_ptr(),
    c"ThreadKilled".as_ptr(),
    c"ThreadComplete".as_ptr(),
];

static mut closure_type_names: [*const c_char; 66] = [
    c"INVALID_OBJECT".as_ptr(),
    c"CONSTR".as_ptr(),
    c"CONSTR_1_0".as_ptr(),
    c"CONSTR_0_1".as_ptr(),
    c"CONSTR_2_0".as_ptr(),
    c"CONSTR_1_1".as_ptr(),
    c"CONSTR_0_2".as_ptr(),
    c"CONSTR_NOCAF".as_ptr(),
    c"FUN".as_ptr(),
    c"FUN_1_0".as_ptr(),
    c"FUN_0_1".as_ptr(),
    c"FUN_2_0".as_ptr(),
    c"FUN_1_1".as_ptr(),
    c"FUN_0_2".as_ptr(),
    c"FUN_STATIC".as_ptr(),
    c"THUNK".as_ptr(),
    c"THUNK_1_0".as_ptr(),
    c"THUNK_0_1".as_ptr(),
    c"THUNK_2_0".as_ptr(),
    c"THUNK_1_1".as_ptr(),
    c"THUNK_0_2".as_ptr(),
    c"THUNK_STATIC".as_ptr(),
    c"THUNK_SELECTOR".as_ptr(),
    c"BCO".as_ptr(),
    c"AP".as_ptr(),
    c"PAP".as_ptr(),
    c"AP_STACK".as_ptr(),
    c"IND".as_ptr(),
    c"IND_STATIC".as_ptr(),
    c"RET_BCO".as_ptr(),
    c"RET_SMALL".as_ptr(),
    c"RET_BIG".as_ptr(),
    c"RET_FUN".as_ptr(),
    c"UPDATE_FRAME".as_ptr(),
    c"CATCH_FRAME".as_ptr(),
    c"UNDERFLOW_FRAME".as_ptr(),
    c"STOP_FRAME".as_ptr(),
    c"BLOCKING_QUEUE".as_ptr(),
    c"BLACKHOLE".as_ptr(),
    c"MVAR_CLEAN".as_ptr(),
    c"MVAR_DIRTY".as_ptr(),
    c"TVAR".as_ptr(),
    c"ARR_WORDS".as_ptr(),
    c"MUT_ARR_PTRS_CLEAN".as_ptr(),
    c"MUT_ARR_PTRS_DIRTY".as_ptr(),
    c"MUT_ARR_PTRS_FROZEN_DIRTY".as_ptr(),
    c"MUT_ARR_PTRS_FROZEN_CLEAN".as_ptr(),
    c"MUT_VAR_CLEAN".as_ptr(),
    c"MUT_VAR_DIRTY".as_ptr(),
    c"WEAK".as_ptr(),
    c"PRIM".as_ptr(),
    c"MUT_PRIM".as_ptr(),
    c"TSO".as_ptr(),
    c"STACK".as_ptr(),
    c"TREC_CHUNK".as_ptr(),
    c"ATOMICALLY_FRAME".as_ptr(),
    c"CATCH_RETRY_FRAME".as_ptr(),
    c"CATCH_STM_FRAME".as_ptr(),
    c"WHITEHOLE".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_CLEAN".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_DIRTY".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_FROZEN_DIRTY".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_FROZEN_CLEAN".as_ptr(),
    c"COMPACT_NFDATA".as_ptr(),
    c"CONTINUATION".as_ptr(),
    c"ANN_FRAME".as_ptr(),
];

unsafe fn info_type(mut closure: *const StgClosure) -> *const c_char {
    return closure_type_names[(*get_itbl(closure)).r#type as usize];
}

unsafe fn info_type_by_ip(mut ip: *const StgInfoTable) -> *const c_char {
    return closure_type_names[(*ip).r#type as usize];
}

unsafe fn info_hdr_type(mut closure: *const StgClosure, mut res: *mut c_char) {
    strcpy(
        res,
        closure_type_names[(*get_itbl(closure)).r#type as usize],
    );
}
