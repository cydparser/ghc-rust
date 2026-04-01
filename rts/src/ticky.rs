use crate::eventlog::event_log::{postTickyCounterDefs, postTickyCounterSamples};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::stg::ticky::{
    ALLOC_CON_ctr, ALLOC_CON_gds, ALLOC_FUN_ctr, ALLOC_FUN_gds, ALLOC_HEAP_ctr, ALLOC_HEAP_tot,
    ALLOC_PAP_adm, ALLOC_PAP_ctr, ALLOC_PAP_gds, ALLOC_PRIM_ctr, ALLOC_PRIM_gds, ALLOC_PRIM_slp,
    ALLOC_RTS_ctr, ALLOC_RTS_tot, ALLOC_SE_THK_ctr, ALLOC_STACK_ctr, ALLOC_STACK_tot,
    ALLOC_THK_gds, ALLOC_TSO_ctr, ALLOC_TSO_tot, ALLOC_UP_THK_ctr, CATCHF_PUSHED_ctr,
    ENT_AP_STACK_ctr, ENT_AP_ctr, ENT_BH_ctr, ENT_CONTINUATION_ctr, ENT_DYN_CON_ctr,
    ENT_DYN_FUN_DIRECT_ctr, ENT_DYN_IND_ctr, ENT_DYN_THK_MANY_ctr, ENT_DYN_THK_SINGLE_ctr,
    ENT_LNE_ctr, ENT_PAP_ctr, ENT_PERM_IND_ctr, ENT_STATIC_CON_ctr, ENT_STATIC_FUN_DIRECT_ctr,
    ENT_STATIC_IND_ctr, ENT_STATIC_THK_MANY_ctr, ENT_STATIC_THK_SINGLE_ctr, ENT_VIA_NODE_ctr,
    GC_FAILED_PROMOTION_ctr, GC_SEL_ABANDONED_ctr, GC_SEL_MAJOR_ctr, GC_SEL_MINOR_ctr,
    HEAP_CHK_ctr, KNOWN_CALL_EXTRA_ARGS_ctr, KNOWN_CALL_TOO_FEW_ARGS_ctr, KNOWN_CALL_ctr,
    MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr, MULTI_CHUNK_SLOW_CALL_ctr, RET_NEW_ctr, RET_NEW_hst,
    RET_OLD_ctr, RET_OLD_hst, RET_UNBOXED_TUP_ctr, RET_UNBOXED_TUP_hst, SLOW_CALL_FUN_CORRECT_ctr,
    SLOW_CALL_FUN_TOO_FEW_ctr, SLOW_CALL_FUN_TOO_MANY_ctr, SLOW_CALL_PAP_CORRECT_ctr,
    SLOW_CALL_PAP_TOO_FEW_ctr, SLOW_CALL_PAP_TOO_MANY_ctr, SLOW_CALL_UNEVALD_ctr, SLOW_CALL_ctr,
    SLOW_CALL_fast_d_ctr, SLOW_CALL_fast_f_ctr, SLOW_CALL_fast_l_ctr, SLOW_CALL_fast_n_ctr,
    SLOW_CALL_fast_p_ctr, SLOW_CALL_fast_pp_ctr, SLOW_CALL_fast_ppp_ctr, SLOW_CALL_fast_pppp_ctr,
    SLOW_CALL_fast_ppppp_ctr, SLOW_CALL_fast_pppppp_ctr, SLOW_CALL_fast_pppv_ctr,
    SLOW_CALL_fast_ppv_ctr, SLOW_CALL_fast_pv_ctr, SLOW_CALL_fast_v_ctr, SLOW_CALL_fast_v16_ctr,
    STK_CHK_ctr, TAG_TAGGED_pred, TAG_UNTAGGED_miss, TAG_UNTAGGED_pred, TICKY_BIN_COUNT,
    UNKNOWN_CALL_ctr, UPD_CAF_BH_SINGLE_ENTRY_ctr, UPD_CAF_BH_UPDATABLE_ctr, UPD_CON_IN_NEW_ctr,
    UPD_CON_IN_PLACE_ctr, UPD_NEW_IND_ctr, UPD_NEW_PERM_IND_ctr, UPD_OLD_IND_ctr,
    UPD_OLD_PERM_IND_ctr, UPD_PAP_IN_NEW_ctr, UPD_PAP_IN_PLACE_ctr, UPD_SQUEEZED_ctr,
    UPDF_OMITTED_ctr, UPDF_PUSHED_ctr, UPDF_RCC_OMITTED_ctr, UPDF_RCC_PUSHED_ctr,
    VERY_SLOW_CALL_ctr,
};
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::ffi::stg::types::{StgInt, StgWord, StgWord64};
use crate::prelude::*;
use crate::ticky::{_StgEntCounter, StgEntCounter};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _StgEntCounter {
    pub(crate) registeredp: StgWord,
    pub(crate) arity: StgInt,
    pub(crate) allocd: StgInt,
    pub(crate) str: *mut c_char,
    pub(crate) arg_kinds: *mut c_char,
    pub(crate) ticky_json: *mut c_char,
    pub(crate) info: *mut StgInfoTable,
    pub(crate) entry_count: StgInt,
    pub(crate) allocs: StgInt,
    pub(crate) link: *mut _StgEntCounter,
}

#[ffi(compiler)]
pub type StgEntCounter = _StgEntCounter;

pub(crate) static mut top_ct: StgEntCounter = _StgEntCounter {
    registeredp: 0,
    arity: 0,
    allocd: 0,
    str: c"TOP".as_ptr(),
    arg_kinds: c"".as_ptr(),
    ticky_json: c"".as_ptr(),
    info: null_mut::<StgInfoTable>(),
    entry_count: 0,
    allocs: 0,
    link: null_mut::<_StgEntCounter>(),
};

pub(crate) static mut ticky_entry_ctrs: *mut StgEntCounter = null_mut::<StgEntCounter>();

unsafe fn PrintTickyInfo() {
    if RtsFlags.TraceFlags.ticky {
        barf(c"Ticky eventlog output can't be used with +RTS -r<file>".as_ptr());
    }

    let mut i: u64 = 0;
    let mut tot_thk_enters = (ENT_STATIC_THK_MANY_ctr
        + ENT_DYN_THK_MANY_ctr
        + ENT_STATIC_THK_SINGLE_ctr
        + ENT_DYN_THK_SINGLE_ctr) as u64;

    let mut tot_con_enters = (ENT_STATIC_CON_ctr + ENT_DYN_CON_ctr) as u64;
    let mut tot_fun_direct_enters = (ENT_STATIC_FUN_DIRECT_ctr + ENT_DYN_FUN_DIRECT_ctr) as u64;

    let mut tot_ind_enters = (ENT_STATIC_IND_ctr + ENT_DYN_IND_ctr) as u64;
    let mut tot_known_calls =
        (KNOWN_CALL_ctr + KNOWN_CALL_TOO_FEW_ARGS_ctr + KNOWN_CALL_EXTRA_ARGS_ctr) as u64;

    let mut tot_tail_calls = (UNKNOWN_CALL_ctr as u64).wrapping_add(tot_known_calls as u64) as u64;

    let mut tot_enters = (tot_con_enters
        .wrapping_add(tot_fun_direct_enters)
        .wrapping_add(tot_ind_enters) as u64)
        .wrapping_add(ENT_PERM_IND_ctr as u64)
        .wrapping_add(ENT_PAP_ctr as u64)
        .wrapping_add(tot_thk_enters as u64) as u64;

    let mut jump_direct_enters = (tot_enters as u64).wrapping_sub(ENT_VIA_NODE_ctr as u64) as u64;

    let mut tot_returns = (RET_NEW_ctr + RET_OLD_ctr + RET_UNBOXED_TUP_ctr) as u64;
    let mut tot_returns_of_new = RET_NEW_ctr as u64;
    let mut con_updates = (UPD_CON_IN_NEW_ctr + UPD_CON_IN_PLACE_ctr) as u64;
    let mut pap_updates = (UPD_PAP_IN_NEW_ctr + UPD_PAP_IN_PLACE_ctr) as u64;
    let mut tot_updates = (UPD_SQUEEZED_ctr as u64)
        .wrapping_add(pap_updates as u64)
        .wrapping_add(con_updates as u64) as u64;

    let mut tot_new_updates = (UPD_NEW_IND_ctr + UPD_NEW_PERM_IND_ctr) as u64;
    let mut tot_old_updates = (UPD_OLD_IND_ctr + UPD_OLD_PERM_IND_ctr) as u64;
    let mut tot_gengc_updates = tot_new_updates.wrapping_add(tot_old_updates);
    let mut tot_tag_preds = (TAG_UNTAGGED_pred + TAG_TAGGED_pred) as u64;
    let mut tf = RtsFlags.TickyFlags.tickyFile;

    if tf.is_null() {
        tf = __stderrp;
    }

    fprintf(tf, c"\nSTACK USAGE:\n".as_ptr());

    fprintf(
        tf,
        c"\nENTERS: %lu  of which %lu (%.1f%%) direct to the entry code\n\t\t  [the rest indirected via Node's info ptr]\n"
            .as_ptr(),
        tot_enters,
        jump_direct_enters,
        100.0f64

            * (if tot_enters == 0 {
                0.0f64
            } else {
                jump_direct_enters as f64 / tot_enters as f64
            }),
    );

    fprintf(
        tf,
        c"%11lu (%5.1f%%) thunks\n".as_ptr(),
        tot_thk_enters,
        100.0f64
            * (if tot_enters == 0 {
                0.0f64
            } else {
                tot_thk_enters as f64 / tot_enters as f64
            }),
    );

    fprintf(
        tf,
        c"%11lu (%5.1f%%) data values\n".as_ptr(),
        tot_con_enters,
        100.0f64
            * (if tot_enters == 0 {
                0.0f64
            } else {
                tot_con_enters as f64 / tot_enters as f64
            }),
    );

    fprintf(
        tf,
        c"%11lu (%5.1f%%) normal indirections\n".as_ptr(),
        tot_ind_enters,
        100.0f64
            * (if tot_enters == 0 {
                0.0f64
            } else {
                tot_ind_enters as f64 / tot_enters as f64
            }),
    );

    fprintf(
        tf,
        c"%11lld (%5.1f%%) permanent indirections\n".as_ptr(),
        ENT_PERM_IND_ctr,
        100.0f64
            * (if tot_enters == 0 {
                0.0f64
            } else {
                ENT_PERM_IND_ctr as f64 / tot_enters as f64
            }),
    );

    fprintf(
        tf,
        c"\nFUNCTION ENTRIES: %lu\n".as_ptr(),
        tot_fun_direct_enters,
    );

    fprintf(
        tf,
        c"\nTAIL CALLS: %lu, of which %lu (%.lf%%) were to known functions\n".as_ptr(),
        tot_tail_calls,
        tot_known_calls,
        100.0f64
            * (if tot_tail_calls == 0 {
                0.0f64
            } else {
                tot_known_calls as f64 / tot_tail_calls as f64
            }),
    );

    fprintf(
        tf,
        c"\nSLOW APPLICATIONS: %lld evaluated, %lld unevaluated\n".as_ptr(),
        SLOW_CALL_ctr,
        SLOW_CALL_UNEVALD_ctr,
    );

    fprintf(tf, c"\n".as_ptr());
    fprintf(
        tf,
        c"         Too few args   Correct args   Too many args\n".as_ptr(),
    );

    fprintf(
        tf,
        c"   FUN     %8lld       %8lld        %8lld\n".as_ptr(),
        SLOW_CALL_FUN_TOO_FEW_ctr,
        SLOW_CALL_FUN_CORRECT_ctr,
        SLOW_CALL_FUN_TOO_MANY_ctr,
    );

    fprintf(
        tf,
        c"   PAP     %8lld       %8lld        %8lld\n".as_ptr(),
        SLOW_CALL_PAP_TOO_FEW_ctr,
        SLOW_CALL_PAP_CORRECT_ctr,
        SLOW_CALL_PAP_TOO_MANY_ctr,
    );

    fprintf(tf, c"\n".as_ptr());
    fprintf(tf, c"\nRETURNS: %lu\n".as_ptr(), tot_returns);

    fprintf(
        tf,
        c"%11lu (%5.1f%%) from entering a new constructor\n\t\t  [the rest from entering an existing constructor]\n"
            .as_ptr(),
        tot_returns_of_new,
        100.0f64

            * (if tot_returns == 0 {
                0.0f64
            } else {
                tot_returns_of_new as f64 / tot_returns as f64
            }),
    );

    fprintf(tf, c"\nRET_NEW:         %11lld: ".as_ptr(), RET_NEW_ctr);
    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%5.1f%%".as_ptr(),
            100.0f64
                * (if RET_NEW_ctr == 0 {
                    0.0f64
                } else {
                    RET_NEW_hst[i as usize] as f64 / RET_NEW_ctr as f64
                }),
        );

        i = i.wrapping_add(1);
    }

    fprintf(tf, c"\n".as_ptr());
    fprintf(tf, c"RET_OLD:         %11lld: ".as_ptr(), RET_OLD_ctr);
    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%5.1f%%".as_ptr(),
            100.0f64
                * (if RET_OLD_ctr == 0 {
                    0.0f64
                } else {
                    RET_OLD_hst[i as usize] as f64 / RET_OLD_ctr as f64
                }),
        );

        i = i.wrapping_add(1);
    }

    fprintf(tf, c"\n".as_ptr());
    fprintf(
        tf,
        c"RET_UNBOXED_TUP: %11lld: ".as_ptr(),
        RET_UNBOXED_TUP_ctr,
    );
    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%5.1f%%".as_ptr(),
            100.0f64
                * (if RET_UNBOXED_TUP_ctr == 0 {
                    0.0f64
                } else {
                    RET_UNBOXED_TUP_hst[i as usize] as f64 / RET_UNBOXED_TUP_ctr as f64
                }),
        );

        i = i.wrapping_add(1);
    }

    fprintf(tf, c"\n".as_ptr());

    fprintf(
        tf,
        c"\nUPDATE FRAMES: %lld (%lld omitted from thunks)".as_ptr(),
        UPDF_PUSHED_ctr,
        UPDF_OMITTED_ctr,
    );

    fprintf(tf, c"\nCATCH FRAMES:  %lld".as_ptr(), CATCHF_PUSHED_ctr);

    if UPDF_RCC_PUSHED_ctr != 0 {
        fprintf(
            tf,
            c"%11lld restore cost centre frames (%lld omitted)\n".as_ptr(),
            UPDF_RCC_PUSHED_ctr,
            UPDF_RCC_OMITTED_ctr,
        );
    }

    fprintf(tf, c"\nUPDATES: %ld\n".as_ptr(), tot_updates);

    fprintf(
        tf,
        c"%11lu (%5.1f%%) data values\n\t\t  [%lld in place, %lld allocated new space]\n".as_ptr(),
        con_updates,
        100.0f64
            * (if tot_updates == 0 {
                0.0f64
            } else {
                con_updates as f64 / tot_updates as f64
            }),
        UPD_CON_IN_PLACE_ctr,
        UPD_CON_IN_NEW_ctr,
    );

    fprintf(
        tf,
        c"%11lu (%5.1f%%) partial applications\n\t\t  [%lld in place, %lld allocated new space]\n"
            .as_ptr(),
        pap_updates,
        100.0f64
            * (if tot_updates == 0 {
                0.0f64
            } else {
                pap_updates as f64 / tot_updates as f64
            }),
        UPD_PAP_IN_PLACE_ctr,
        UPD_PAP_IN_NEW_ctr,
    );

    fprintf(
        tf,
        c"%11lld (%5.1f%%) updates by squeezing\n".as_ptr(),
        UPD_SQUEEZED_ctr,
        100.0f64
            * (if tot_updates == 0 {
                0.0f64
            } else {
                UPD_SQUEEZED_ctr as f64 / tot_updates as f64
            }),
    );

    if tot_gengc_updates != 0 {
        fprintf(
            tf,
            c"\nNEW GEN UPDATES: %9lu (%5.1f%%)\n".as_ptr(),
            tot_new_updates,
            100.0f64
                * (if tot_gengc_updates == 0 {
                    0.0f64
                } else {
                    tot_new_updates as f64 / tot_gengc_updates as f64
                }),
        );

        fprintf(
            tf,
            c"OLD GEN UPDATES: %9lu (%5.1f%%)\n".as_ptr(),
            tot_old_updates,
            100.0f64
                * (if tot_gengc_updates == 0 {
                    0.0f64
                } else {
                    tot_old_updates as f64 / tot_gengc_updates as f64
                }),
        );
    }

    if tot_tag_preds != 0 {
        fprintf(
            tf,
            c"\nTOTAL TAG PREDICTIONS MADE: %9llu \n".as_ptr(),
            tot_tag_preds as StgWord64,
        );

        fprintf(
            tf,
            c"TAGGED PREDICTIONS HIT:     %9llu \n".as_ptr(),
            TAG_TAGGED_pred as StgWord64,
        );

        fprintf(
            tf,
            c"UNTAGGED PREDICTIONS HIT:   %9llu \n".as_ptr(),
            (TAG_UNTAGGED_pred - TAG_UNTAGGED_miss) as StgWord64,
        );

        fprintf(
            tf,
            c"UNTAGGED PREDICTIONS MISS:  %9llu \n".as_ptr(),
            TAG_UNTAGGED_miss as StgWord64,
        );
    }

    printRegisteredCounterInfo(tf);
    fprintf(
        tf,
        c"\n**************************************************\n".as_ptr(),
    );
    ALLOC_HEAP_ctr += ALLOC_RTS_ctr;
    ALLOC_HEAP_tot += ALLOC_RTS_tot;
    fprintf(tf, c"%11lld ALLOC_HEAP_ctr\n".as_ptr(), ALLOC_HEAP_ctr);
    fprintf(tf, c"%11lld ALLOC_HEAP_tot\n".as_ptr(), ALLOC_HEAP_tot);
    fprintf(tf, c"%11lld HEAP_CHK_ctr\n".as_ptr(), HEAP_CHK_ctr);
    fprintf(tf, c"%11lld STK_CHK_ctr\n".as_ptr(), STK_CHK_ctr);
    fprintf(tf, c"%11lld ALLOC_RTS_ctr\n".as_ptr(), ALLOC_RTS_ctr);
    fprintf(tf, c"%11lld ALLOC_RTS_tot\n".as_ptr(), ALLOC_RTS_tot);
    fprintf(tf, c"%11lld ALLOC_FUN_ctr\n".as_ptr(), ALLOC_FUN_ctr);
    fprintf(tf, c"%11lld ALLOC_FUN_gds\n".as_ptr(), ALLOC_FUN_gds);
    fprintf(tf, c"%11lld ALLOC_PAP_ctr\n".as_ptr(), ALLOC_PAP_ctr);
    fprintf(tf, c"%11lld ALLOC_PAP_adm\n".as_ptr(), ALLOC_PAP_adm);
    fprintf(tf, c"%11lld ALLOC_PAP_gds\n".as_ptr(), ALLOC_PAP_gds);
    fprintf(tf, c"%11lld ALLOC_UP_THK_ctr\n".as_ptr(), ALLOC_UP_THK_ctr);
    fprintf(tf, c"%11lld ALLOC_SE_THK_ctr\n".as_ptr(), ALLOC_SE_THK_ctr);
    fprintf(tf, c"%11lld ALLOC_THK_gds\n".as_ptr(), ALLOC_THK_gds);
    fprintf(tf, c"%11lld ALLOC_CON_ctr\n".as_ptr(), ALLOC_CON_ctr);
    fprintf(tf, c"%11lld ALLOC_CON_gds\n".as_ptr(), ALLOC_CON_gds);
    fprintf(tf, c"%11lld ALLOC_PRIM_ctr\n".as_ptr(), ALLOC_PRIM_ctr);
    fprintf(tf, c"%11lld ALLOC_PRIM_gds\n".as_ptr(), ALLOC_PRIM_gds);
    fprintf(tf, c"%11lld ALLOC_PRIM_slp\n".as_ptr(), ALLOC_PRIM_slp);
    fprintf(tf, c"%11lld ALLOC_TSO_ctr\n".as_ptr(), ALLOC_TSO_ctr);
    fprintf(tf, c"%11lld ALLOC_TSO_tot\n".as_ptr(), ALLOC_TSO_tot);
    fprintf(tf, c"%11lld ALLOC_STACK_ctr\n".as_ptr(), ALLOC_STACK_ctr);
    fprintf(tf, c"%11lld ALLOC_STACK_tot\n".as_ptr(), ALLOC_STACK_tot);
    fprintf(tf, c"%11lld ENT_VIA_NODE_ctr\n".as_ptr(), ENT_VIA_NODE_ctr);
    fprintf(
        tf,
        c"%11lld ENT_STATIC_CON_ctr\n".as_ptr(),
        ENT_STATIC_CON_ctr,
    );
    fprintf(tf, c"%11lld ENT_DYN_CON_ctr\n".as_ptr(), ENT_DYN_CON_ctr);

    fprintf(
        tf,
        c"%11lld ENT_STATIC_FUN_DIRECT_ctr\n".as_ptr(),
        ENT_STATIC_FUN_DIRECT_ctr,
    );

    fprintf(
        tf,
        c"%11lld ENT_DYN_FUN_DIRECT_ctr\n".as_ptr(),
        ENT_DYN_FUN_DIRECT_ctr,
    );
    fprintf(tf, c"%11lld ENT_LNE_ctr\n".as_ptr(), ENT_LNE_ctr);
    fprintf(
        tf,
        c"%11lld ENT_STATIC_IND_ctr\n".as_ptr(),
        ENT_STATIC_IND_ctr,
    );
    fprintf(tf, c"%11lld ENT_DYN_IND_ctr\n".as_ptr(), ENT_DYN_IND_ctr);

    if RtsFlags.GcFlags.squeezeUpdFrames as i32 == 0 {
        fprintf(tf, c"%11lld ENT_PERM_IND_ctr\n".as_ptr(), ENT_PERM_IND_ctr);
    } else {
        fprintf(
            tf,
            c"%11lld ENT_PERM_IND_ctr requires +RTS -Z\n".as_ptr(),
            ENT_PERM_IND_ctr,
        );
    }

    fprintf(tf, c"%11lld ENT_AP_ctr\n".as_ptr(), ENT_AP_ctr);
    fprintf(tf, c"%11lld ENT_PAP_ctr\n".as_ptr(), ENT_PAP_ctr);
    fprintf(tf, c"%11lld ENT_AP_STACK_ctr\n".as_ptr(), ENT_AP_STACK_ctr);
    fprintf(
        tf,
        c"%11lld ENT_CONTINUATION_ctr\n".as_ptr(),
        ENT_CONTINUATION_ctr,
    );
    fprintf(tf, c"%11lld ENT_BH_ctr\n".as_ptr(), ENT_BH_ctr);

    fprintf(
        tf,
        c"%11lld ENT_STATIC_THK_SINGLE_ctr\n".as_ptr(),
        ENT_STATIC_THK_SINGLE_ctr,
    );

    fprintf(
        tf,
        c"%11lld ENT_STATIC_THK_MANY_ctr\n".as_ptr(),
        ENT_STATIC_THK_MANY_ctr,
    );
    fprintf(
        tf,
        c"%11lld ENT_DYN_THK_SINGLE_ctr\n".as_ptr(),
        ENT_DYN_THK_SINGLE_ctr,
    );
    fprintf(
        tf,
        c"%11lld ENT_DYN_THK_MANY_ctr\n".as_ptr(),
        ENT_DYN_THK_MANY_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPD_CAF_BH_UPDATABLE_ctr\n".as_ptr(),
        UPD_CAF_BH_UPDATABLE_ctr,
    );

    fprintf(
        tf,
        c"%11lld UPD_CAF_BH_SINGLE_ENTRY_ctr\n".as_ptr(),
        UPD_CAF_BH_SINGLE_ENTRY_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_v16_ctr\n".as_ptr(),
        SLOW_CALL_fast_v16_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_v_ctr\n".as_ptr(),
        SLOW_CALL_fast_v_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_f_ctr\n".as_ptr(),
        SLOW_CALL_fast_f_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_d_ctr\n".as_ptr(),
        SLOW_CALL_fast_d_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_l_ctr\n".as_ptr(),
        SLOW_CALL_fast_l_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_n_ctr\n".as_ptr(),
        SLOW_CALL_fast_n_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_p_ctr\n".as_ptr(),
        SLOW_CALL_fast_p_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_pv_ctr\n".as_ptr(),
        SLOW_CALL_fast_pv_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_pp_ctr\n".as_ptr(),
        SLOW_CALL_fast_pp_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_ppv_ctr\n".as_ptr(),
        SLOW_CALL_fast_ppv_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_ppp_ctr\n".as_ptr(),
        SLOW_CALL_fast_ppp_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_pppv_ctr\n".as_ptr(),
        SLOW_CALL_fast_pppv_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_pppp_ctr\n".as_ptr(),
        SLOW_CALL_fast_pppp_ctr,
    );
    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_ppppp_ctr\n".as_ptr(),
        SLOW_CALL_fast_ppppp_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_fast_pppppp_ctr\n".as_ptr(),
        SLOW_CALL_fast_pppppp_ctr,
    );

    fprintf(
        tf,
        c"%11lld VERY_SLOW_CALL_ctr\n".as_ptr(),
        VERY_SLOW_CALL_ctr,
    );
    fprintf(tf, c"%11lld UNKNOWN_CALL_ctr\n".as_ptr(), UNKNOWN_CALL_ctr);
    fprintf(tf, c"%11lld KNOWN_CALL_ctr\n".as_ptr(), KNOWN_CALL_ctr);

    fprintf(
        tf,
        c"%11lld KNOWN_CALL_TOO_FEW_ARGS_ctr\n".as_ptr(),
        KNOWN_CALL_TOO_FEW_ARGS_ctr,
    );

    fprintf(
        tf,
        c"%11lld KNOWN_CALL_EXTRA_ARGS_ctr\n".as_ptr(),
        KNOWN_CALL_EXTRA_ARGS_ctr,
    );

    fprintf(
        tf,
        c"%11lld MULTI_CHUNK_SLOW_CALL_ctr\n".as_ptr(),
        MULTI_CHUNK_SLOW_CALL_ctr,
    );

    fprintf(
        tf,
        c"%11lld MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr\n".as_ptr(),
        MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr,
    );

    fprintf(tf, c"%11lld SLOW_CALL_ctr\n".as_ptr(), SLOW_CALL_ctr);

    fprintf(
        tf,
        c"%11lld SLOW_CALL_FUN_TOO_FEW_ctr\n".as_ptr(),
        SLOW_CALL_FUN_TOO_FEW_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_FUN_CORRECT_ctr\n".as_ptr(),
        SLOW_CALL_FUN_CORRECT_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_FUN_TOO_MANY_ctr\n".as_ptr(),
        SLOW_CALL_FUN_TOO_MANY_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_PAP_TOO_FEW_ctr\n".as_ptr(),
        SLOW_CALL_PAP_TOO_FEW_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_PAP_CORRECT_ctr\n".as_ptr(),
        SLOW_CALL_PAP_CORRECT_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_PAP_TOO_MANY_ctr\n".as_ptr(),
        SLOW_CALL_PAP_TOO_MANY_ctr,
    );

    fprintf(
        tf,
        c"%11lld SLOW_CALL_UNEVALD_ctr\n".as_ptr(),
        SLOW_CALL_UNEVALD_ctr,
    );
    fprintf(tf, c"%11lld RET_NEW_ctr\n".as_ptr(), RET_NEW_ctr);
    fprintf(tf, c"%11lld RET_OLD_ctr\n".as_ptr(), RET_OLD_ctr);
    fprintf(
        tf,
        c"%11lld RET_UNBOXED_TUP_ctr\n".as_ptr(),
        RET_UNBOXED_TUP_ctr,
    );
    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%11lld RET_NEW_hst_%lu\n".as_ptr(),
            RET_NEW_hst[i as usize],
            i,
        );
        i = i.wrapping_add(1);
    }

    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%11lld RET_OLD_hst_%lu\n".as_ptr(),
            RET_OLD_hst[i as usize],
            i,
        );
        i = i.wrapping_add(1);
    }

    i = 0;

    while i < TICKY_BIN_COUNT as u64 {
        fprintf(
            tf,
            c"%11lld RET_UNBOXED_TUP_hst_%lu\n".as_ptr(),
            RET_UNBOXED_TUP_hst[i as usize],
            i,
        );

        i = i.wrapping_add(1);
    }

    fprintf(tf, c"%11lld UPDF_OMITTED_ctr\n".as_ptr(), UPDF_OMITTED_ctr);
    fprintf(tf, c"%11lld UPDF_PUSHED_ctr\n".as_ptr(), UPDF_PUSHED_ctr);
    fprintf(
        tf,
        c"%11lld CATCHF_PUSHED_ctr\n".as_ptr(),
        CATCHF_PUSHED_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPDF_RCC_PUSHED_ctr\n".as_ptr(),
        UPDF_RCC_PUSHED_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPDF_RCC_OMITTED_ctr\n".as_ptr(),
        UPDF_RCC_OMITTED_ctr,
    );
    fprintf(tf, c"%11lld UPD_SQUEEZED_ctr\n".as_ptr(), UPD_SQUEEZED_ctr);
    fprintf(
        tf,
        c"%11lld UPD_CON_IN_NEW_ctr\n".as_ptr(),
        UPD_CON_IN_NEW_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPD_CON_IN_PLACE_ctr\n".as_ptr(),
        UPD_CON_IN_PLACE_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPD_PAP_IN_NEW_ctr\n".as_ptr(),
        UPD_PAP_IN_NEW_ctr,
    );
    fprintf(
        tf,
        c"%11lld UPD_PAP_IN_PLACE_ctr\n".as_ptr(),
        UPD_PAP_IN_PLACE_ctr,
    );
    fprintf(tf, c"%11lld UPD_NEW_IND_ctr\n".as_ptr(), UPD_NEW_IND_ctr);

    if RtsFlags.GcFlags.squeezeUpdFrames as i32 == 0 {
        fprintf(
            tf,
            c"%11lld UPD_NEW_PERM_IND_ctr\n".as_ptr(),
            UPD_NEW_PERM_IND_ctr,
        );
    } else {
        fprintf(
            tf,
            c"%11lld UPD_NEW_PERM_IND_ctr requires +RTS -Z\n".as_ptr(),
            UPD_NEW_PERM_IND_ctr,
        );
    }

    fprintf(tf, c"%11lld UPD_OLD_IND_ctr\n".as_ptr(), UPD_OLD_IND_ctr);

    if RtsFlags.GcFlags.squeezeUpdFrames as i32 == 0 {
        fprintf(
            tf,
            c"%11lld UPD_OLD_PERM_IND_ctr\n".as_ptr(),
            UPD_OLD_PERM_IND_ctr,
        );
    } else {
        fprintf(
            tf,
            c"%11lld UPD_OLD_PERM_IND_ctr requires +RTS -Z\n".as_ptr(),
            UPD_OLD_PERM_IND_ctr,
        );
    }

    fprintf(
        tf,
        c"%11lld GC_SEL_ABANDONED_ctr\n".as_ptr(),
        GC_SEL_ABANDONED_ctr,
    );
    fprintf(tf, c"%11lld GC_SEL_MINOR_ctr\n".as_ptr(), GC_SEL_MINOR_ctr);
    fprintf(tf, c"%11lld GC_SEL_MAJOR_ctr\n".as_ptr(), GC_SEL_MAJOR_ctr);
    fprintf(
        tf,
        c"%11lld GC_FAILED_PROMOTION_ctr\n".as_ptr(),
        GC_FAILED_PROMOTION_ctr,
    );
}

unsafe fn printRegisteredCounterInfo(mut tf: *mut FILE) {
    let mut p = null_mut::<StgEntCounter>();

    if !ticky_entry_ctrs.is_null() {
        fprintf(
            tf,
            c"\nThe following table is explained by https://gitlab.haskell.org/ghc/ghc/wikis/debugging/ticky-ticky\nAll allocation numbers are in bytes.\n"
                .as_ptr(),
        );

        fprintf(
            tf,
            c"\n**************************************************\n\n".as_ptr(),
        );
    }

    fprintf(
        tf,
        c"%11s%12s%12s  %-63s %s\n".as_ptr(),
        c"Entries".as_ptr(),
        c"Alloc".as_ptr(),
        c"Alloc'd".as_ptr(),
        c"Non-void Arguments".as_ptr(),
        c"STG Name".as_ptr(),
    );

    fprintf(
        tf,
        c"--------------------------------------------------------------------------------\n"
            .as_ptr(),
    );

    p = ticky_entry_ctrs;

    while !p.is_null() {
        fprintf(
            tf,
            c"%11lld%12lld%12lld %3lu %-60.60s %s".as_ptr(),
            (*p).entry_count,
            (*p).allocs,
            (*p).allocd,
            (*p).arity as u64,
            (*p).arg_kinds,
            (*p).str,
        );

        fprintf(tf, c"\n".as_ptr());
        p = (*p).link as *mut StgEntCounter;
    }
}

unsafe fn emitTickyCounterDefs() {
    postTickyCounterDefs(ticky_entry_ctrs);
}

unsafe fn emitTickyCounterSamples() {
    postTickyCounterSamples(ticky_entry_ctrs);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestTickyCounterSamples() {
    if RtsFlags.TraceFlags.ticky {
        emitTickyCounterSamples();
    }
}
