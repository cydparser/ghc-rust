use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_stg_upd_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_upd_frame_info),
        size_of_val(unsafe { &sys::stg_upd_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_upd_frame_info),
        align_of_val(unsafe { &sys::stg_upd_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_bh_upd_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_bh_upd_frame_info),
        size_of_val(unsafe { &sys::stg_bh_upd_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_bh_upd_frame_info),
        align_of_val(unsafe { &sys::stg_bh_upd_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_catch_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_catch_frame_info),
        size_of_val(unsafe { &sys::stg_catch_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_catch_frame_info),
        align_of_val(unsafe { &sys::stg_catch_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_catch_retry_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_catch_retry_frame_info),
        size_of_val(unsafe { &sys::stg_catch_retry_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_catch_retry_frame_info),
        align_of_val(unsafe { &sys::stg_catch_retry_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_atomically_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_atomically_frame_info),
        size_of_val(unsafe { &sys::stg_atomically_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_atomically_frame_info),
        align_of_val(unsafe { &sys::stg_atomically_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_catch_stm_frame_info_layout() {
    assert_eq!(
        size_of_val(&stg_catch_stm_frame_info),
        size_of_val(unsafe { &sys::stg_catch_stm_frame_info })
    );
    assert_eq!(
        align_of_val(&stg_catch_stm_frame_info),
        align_of_val(unsafe { &sys::stg_catch_stm_frame_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_stack_underflow_frame_d_info_layout() {
    assert_eq!(
        size_of_val(&stg_stack_underflow_frame_d_info),
        size_of_val(unsafe { &sys::stg_stack_underflow_frame_d_info })
    );
    assert_eq!(
        align_of_val(&stg_stack_underflow_frame_d_info),
        align_of_val(unsafe { &sys::stg_stack_underflow_frame_d_info })
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_ctoi_t() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_ctoi_t() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_ctoi_t() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_ctoi_t() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_ctoi_t() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ctoi_t3_info_layout() {
    assert_eq!(
        size_of_val(&stg_ctoi_t3_info),
        size_of_val(unsafe { &sys::stg_ctoi_t3_info })
    );
    assert_eq!(
        align_of_val(&stg_ctoi_t3_info),
        align_of_val(unsafe { &sys::stg_ctoi_t3_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_primcall_info_layout() {
    assert_eq!(
        size_of_val(&stg_primcall_info),
        size_of_val(unsafe { &sys::stg_primcall_info })
    );
    assert_eq!(
        align_of_val(&stg_primcall_info),
        align_of_val(unsafe { &sys::stg_primcall_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_IND_STATIC_info_layout() {
    assert_eq!(
        size_of_val(&stg_IND_STATIC_info),
        size_of_val(unsafe { &sys::stg_IND_STATIC_info })
    );
    assert_eq!(
        align_of_val(&stg_IND_STATIC_info),
        align_of_val(unsafe { &sys::stg_IND_STATIC_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys___stg_EAGER_BLACKHOLE_info_layout() {
    assert_eq!(
        size_of_val(&__stg_EAGER_BLACKHOLE_info),
        size_of_val(unsafe { &sys::__stg_EAGER_BLACKHOLE_info })
    );
    assert_eq!(
        align_of_val(&__stg_EAGER_BLACKHOLE_info),
        align_of_val(unsafe { &sys::__stg_EAGER_BLACKHOLE_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_BCO_info_layout() {
    assert_eq!(
        size_of_val(&stg_BCO_info),
        size_of_val(unsafe { &sys::stg_BCO_info })
    );
    assert_eq!(
        align_of_val(&stg_BCO_info),
        align_of_val(unsafe { &sys::stg_BCO_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_STACK_info_layout() {
    assert_eq!(
        size_of_val(&stg_STACK_info),
        size_of_val(unsafe { &sys::stg_STACK_info })
    );
    assert_eq!(
        align_of_val(&stg_STACK_info),
        align_of_val(unsafe { &sys::stg_STACK_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ARR_WORDS_info_layout() {
    assert_eq!(
        size_of_val(&stg_ARR_WORDS_info),
        size_of_val(unsafe { &sys::stg_ARR_WORDS_info })
    );
    assert_eq!(
        align_of_val(&stg_ARR_WORDS_info),
        align_of_val(unsafe { &sys::stg_ARR_WORDS_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_MUT_ARR_PTRS_FROZEN_CLEAN_info_layout() {
    assert_eq!(
        size_of_val(&stg_MUT_ARR_PTRS_FROZEN_CLEAN_info),
        size_of_val(unsafe { &sys::stg_MUT_ARR_PTRS_FROZEN_CLEAN_info })
    );
    assert_eq!(
        align_of_val(&stg_MUT_ARR_PTRS_FROZEN_CLEAN_info),
        align_of_val(unsafe { &sys::stg_MUT_ARR_PTRS_FROZEN_CLEAN_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_SRT_1_info_layout() {
    assert_eq!(
        size_of_val(&stg_SRT_1_info),
        size_of_val(unsafe { &sys::stg_SRT_1_info })
    );
    assert_eq!(
        align_of_val(&stg_SRT_1_info),
        align_of_val(unsafe { &sys::stg_SRT_1_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_SRT_16_info_layout() {
    assert_eq!(
        size_of_val(&stg_SRT_16_info),
        size_of_val(unsafe { &sys::stg_SRT_16_info })
    );
    assert_eq!(
        align_of_val(&stg_SRT_16_info),
        align_of_val(unsafe { &sys::stg_SRT_16_info })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_stg_INTLIKE_closure_layout() {
    assert_eq!(
        size_of_val(unsafe { &stg_INTLIKE_closure }),
        size_of_val(unsafe { &sys::stg_INTLIKE_closure })
    );
    assert_eq!(
        align_of_val(unsafe { &stg_INTLIKE_closure }),
        align_of_val(unsafe { &sys::stg_INTLIKE_closure })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_unpack_cstring_info_layout() {
    assert_eq!(
        size_of_val(&stg_unpack_cstring_info),
        size_of_val(unsafe { &sys::stg_unpack_cstring_info })
    );
    assert_eq!(
        align_of_val(&stg_unpack_cstring_info),
        align_of_val(unsafe { &sys::stg_unpack_cstring_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_unpack_cstring_utf8_info_layout() {
    assert_eq!(
        size_of_val(&stg_unpack_cstring_utf8_info),
        size_of_val(unsafe { &sys::stg_unpack_cstring_utf8_info })
    );
    assert_eq!(
        align_of_val(&stg_unpack_cstring_utf8_info),
        align_of_val(unsafe { &sys::stg_unpack_cstring_utf8_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ap_pp_info_layout() {
    assert_eq!(
        size_of_val(&stg_ap_pp_info),
        size_of_val(unsafe { &sys::stg_ap_pp_info })
    );
    assert_eq!(
        align_of_val(&stg_ap_pp_info),
        align_of_val(unsafe { &sys::stg_ap_pp_info })
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_ap_n_fast() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_ap_n_fast() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_n_fast() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_ap_n_fast() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_n_fast() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_ap_p_fast() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_ap_p_fast() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_p_fast() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_ap_p_fast() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_p_fast() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_ap_pp_fast() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_ap_pp_fast() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_pp_fast() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_ap_pp_fast() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_pp_fast() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_ap_ppp_fast() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_ap_ppp_fast() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_ppp_fast() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_ap_ppp_fast() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_ap_ppp_fast() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_noregs() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_noregs() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_noregs() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_noregs() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_noregs() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ret_p_info_layout() {
    assert_eq!(
        size_of_val(&stg_ret_p_info),
        size_of_val(unsafe { &sys::stg_ret_p_info })
    );
    assert_eq!(
        align_of_val(&stg_ret_p_info),
        align_of_val(unsafe { &sys::stg_ret_p_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ret_n_info_layout() {
    assert_eq!(
        size_of_val(&stg_ret_n_info),
        size_of_val(unsafe { &sys::stg_ret_n_info })
    );
    assert_eq!(
        align_of_val(&stg_ret_n_info),
        align_of_val(unsafe { &sys::stg_ret_n_info })
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_ret_t_info_layout() {
    assert_eq!(
        size_of_val(&stg_ret_t_info),
        size_of_val(unsafe { &sys::stg_ret_t_info })
    );
    assert_eq!(
        align_of_val(&stg_ret_t_info),
        align_of_val(unsafe { &sys::stg_ret_t_info })
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent___stg_gc_enter_1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::__stg_gc_enter_1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { __stg_gc_enter_1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___stg_gc_enter_1() {
    let actual = {
        let result: StgFunPtr = unsafe { __stg_gc_enter_1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_unpt_r1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_unpt_r1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_unpt_r1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_unpt_r1() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_unpt_r1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_unbx_r1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_unbx_r1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_unbx_r1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_unbx_r1() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_unbx_r1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_f1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_f1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_f1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_f1() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_f1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_d1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_d1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_d1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_d1() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_d1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_l1() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_l1() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_l1() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_l1() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_l1() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_pp() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_pp() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_pp() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_pp() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_pp() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_ppp() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_ppp() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_ppp() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_ppp() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_ppp() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_gc_pppp() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_gc_pppp() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_pppp() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_gc_pppp() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_gc_pppp() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent___stg_gc_fun() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::__stg_gc_fun() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { __stg_gc_fun() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test___stg_gc_fun() {
    let actual = {
        let result: StgFunPtr = unsafe { __stg_gc_fun() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
fn sys_stg_stop_thread_info_layout() {
    assert_eq!(
        size_of_val(&stg_stop_thread_info),
        size_of_val(unsafe { &sys::stg_stop_thread_info })
    );
    assert_eq!(
        align_of_val(&stg_stop_thread_info),
        align_of_val(unsafe { &sys::stg_stop_thread_info })
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_StgReturn() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::StgReturn() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { StgReturn() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_StgReturn() {
    let actual = {
        let result: StgFunPtr = unsafe { StgReturn() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_paniczh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_paniczh() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_paniczh() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_paniczh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_paniczh() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_absentErrorzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_absentErrorzh() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_absentErrorzh() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_absentErrorzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_absentErrorzh() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_getThreadAllocationCounterzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_getThreadAllocationCounterzh() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_getThreadAllocationCounterzh() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_getThreadAllocationCounterzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_getThreadAllocationCounterzh() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_getOtherThreadAllocationCounterzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_getOtherThreadAllocationCounterzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
