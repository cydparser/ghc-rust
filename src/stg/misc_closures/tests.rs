use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ctoi_t() -> bool {
    let expected = unsafe { transmute(sys::stg_ctoi_t()) };
    let actual = unsafe { super::stg_ctoi_t() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ctoi_t() {
    unsafe { super::stg_ctoi_t() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_0_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_0_fast()) };
    let actual = unsafe { super::stg_ap_0_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_0_fast() {
    unsafe { super::stg_ap_0_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_v_fast()) };
    let actual = unsafe { super::stg_ap_v_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v_fast() {
    unsafe { super::stg_ap_v_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_f_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_f_fast()) };
    let actual = unsafe { super::stg_ap_f_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_f_fast() {
    unsafe { super::stg_ap_f_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_d_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_d_fast()) };
    let actual = unsafe { super::stg_ap_d_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_d_fast() {
    unsafe { super::stg_ap_d_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_l_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_l_fast()) };
    let actual = unsafe { super::stg_ap_l_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_l_fast() {
    unsafe { super::stg_ap_l_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v16_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_v16_fast()) };
    let actual = unsafe { super::stg_ap_v16_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v16_fast() {
    unsafe { super::stg_ap_v16_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v32_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_v32_fast()) };
    let actual = unsafe { super::stg_ap_v32_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v32_fast() {
    unsafe { super::stg_ap_v32_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v64_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_v64_fast()) };
    let actual = unsafe { super::stg_ap_v64_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v64_fast() {
    unsafe { super::stg_ap_v64_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_n_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_n_fast()) };
    let actual = unsafe { super::stg_ap_n_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_n_fast() {
    unsafe { super::stg_ap_n_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_p_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_p_fast()) };
    let actual = unsafe { super::stg_ap_p_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_p_fast() {
    unsafe { super::stg_ap_p_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pv_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_pv_fast()) };
    let actual = unsafe { super::stg_ap_pv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pv_fast() {
    unsafe { super::stg_ap_pv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pp_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_pp_fast()) };
    let actual = unsafe { super::stg_ap_pp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pp_fast() {
    unsafe { super::stg_ap_pp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppv_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_ppv_fast()) };
    let actual = unsafe { super::stg_ap_ppv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppv_fast() {
    unsafe { super::stg_ap_ppv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppp_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_ppp_fast()) };
    let actual = unsafe { super::stg_ap_ppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppp_fast() {
    unsafe { super::stg_ap_ppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppv_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_pppv_fast()) };
    let actual = unsafe { super::stg_ap_pppv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppv_fast() {
    unsafe { super::stg_ap_pppv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppp_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_pppp_fast()) };
    let actual = unsafe { super::stg_ap_pppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppp_fast() {
    unsafe { super::stg_ap_pppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppppp_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_ppppp_fast()) };
    let actual = unsafe { super::stg_ap_ppppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppppp_fast() {
    unsafe { super::stg_ap_ppppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppppp_fast() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_pppppp_fast()) };
    let actual = unsafe { super::stg_ap_pppppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppppp_fast() {
    unsafe { super::stg_ap_pppppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_PAP_apply() -> bool {
    let expected = unsafe { transmute(sys::stg_PAP_apply()) };
    let actual = unsafe { super::stg_PAP_apply() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_PAP_apply() {
    unsafe { super::stg_PAP_apply() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_CONTINUATION_apply() -> bool {
    let expected = unsafe { transmute(sys::stg_CONTINUATION_apply()) };
    let actual = unsafe { super::stg_CONTINUATION_apply() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_CONTINUATION_apply() {
    unsafe { super::stg_CONTINUATION_apply() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v16() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_stk_v16()) };
    let actual = unsafe { super::stg_ap_stk_v16() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v16() {
    unsafe { super::stg_ap_stk_v16() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v32() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_stk_v32()) };
    let actual = unsafe { super::stg_ap_stk_v32() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v32() {
    unsafe { super::stg_ap_stk_v32() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v64() -> bool {
    let expected = unsafe { transmute(sys::stg_ap_stk_v64()) };
    let actual = unsafe { super::stg_ap_stk_v64() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v64() {
    unsafe { super::stg_ap_stk_v64() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v16() -> bool {
    let expected = unsafe { transmute(sys::stg_stk_save_v16()) };
    let actual = unsafe { super::stg_stk_save_v16() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v16() {
    unsafe { super::stg_stk_save_v16() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v32() -> bool {
    let expected = unsafe { transmute(sys::stg_stk_save_v32()) };
    let actual = unsafe { super::stg_stk_save_v32() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v32() {
    unsafe { super::stg_stk_save_v32() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v64() -> bool {
    let expected = unsafe { transmute(sys::stg_stk_save_v64()) };
    let actual = unsafe { super::stg_stk_save_v64() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v64() {
    unsafe { super::stg_stk_save_v64() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_noregs() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_noregs()) };
    let actual = unsafe { super::stg_gc_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_noregs() {
    unsafe { super::stg_gc_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim()) };
    let actual = unsafe { super::stg_gc_prim() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim() {
    unsafe { super::stg_gc_prim() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_p() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim_p()) };
    let actual = unsafe { super::stg_gc_prim_p() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_p() {
    unsafe { super::stg_gc_prim_p() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_pp() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim_pp()) };
    let actual = unsafe { super::stg_gc_prim_pp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_pp() {
    unsafe { super::stg_gc_prim_pp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_n() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim_n()) };
    let actual = unsafe { super::stg_gc_prim_n() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_n() {
    unsafe { super::stg_gc_prim_n() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_p_ll() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim_p_ll()) };
    let actual = unsafe { super::stg_gc_prim_p_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_p_ll() {
    unsafe { super::stg_gc_prim_p_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_pp_ll() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_prim_pp_ll()) };
    let actual = unsafe { super::stg_gc_prim_pp_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_pp_ll() {
    unsafe { super::stg_gc_prim_pp_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___stg_gc_enter_1() -> bool {
    let expected = unsafe { transmute(sys::__stg_gc_enter_1()) };
    let actual = unsafe { super::__stg_gc_enter_1() };
    actual == expected
}

#[test]
#[ignore]
fn test___stg_gc_enter_1() {
    unsafe { super::__stg_gc_enter_1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_unpt_r1() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_unpt_r1()) };
    let actual = unsafe { super::stg_gc_unpt_r1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_unpt_r1() {
    unsafe { super::stg_gc_unpt_r1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_unbx_r1() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_unbx_r1()) };
    let actual = unsafe { super::stg_gc_unbx_r1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_unbx_r1() {
    unsafe { super::stg_gc_unbx_r1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_f1() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_f1()) };
    let actual = unsafe { super::stg_gc_f1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_f1() {
    unsafe { super::stg_gc_f1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_d1() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_d1()) };
    let actual = unsafe { super::stg_gc_d1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_d1() {
    unsafe { super::stg_gc_d1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_l1() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_l1()) };
    let actual = unsafe { super::stg_gc_l1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_l1() {
    unsafe { super::stg_gc_l1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_pp() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_pp()) };
    let actual = unsafe { super::stg_gc_pp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_pp() {
    unsafe { super::stg_gc_pp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_ppp() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_ppp()) };
    let actual = unsafe { super::stg_gc_ppp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_ppp() {
    unsafe { super::stg_gc_ppp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_pppp() -> bool {
    let expected = unsafe { transmute(sys::stg_gc_pppp()) };
    let actual = unsafe { super::stg_gc_pppp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_pppp() {
    unsafe { super::stg_gc_pppp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___stg_gc_fun() -> bool {
    let expected = unsafe { transmute(sys::__stg_gc_fun()) };
    let actual = unsafe { super::__stg_gc_fun() };
    actual == expected
}

#[test]
#[ignore]
fn test___stg_gc_fun() {
    unsafe { super::__stg_gc_fun() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yield_noregs() -> bool {
    let expected = unsafe { transmute(sys::stg_yield_noregs()) };
    let actual = unsafe { super::stg_yield_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yield_noregs() {
    unsafe { super::stg_yield_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yield_to_interpreter() -> bool {
    let expected = unsafe { transmute(sys::stg_yield_to_interpreter()) };
    let actual = unsafe { super::stg_yield_to_interpreter() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yield_to_interpreter() {
    unsafe { super::stg_yield_to_interpreter() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_noregs() -> bool {
    let expected = unsafe { transmute(sys::stg_block_noregs()) };
    let actual = unsafe { super::stg_block_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_noregs() {
    unsafe { super::stg_block_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_blackhole() -> bool {
    let expected = unsafe { transmute(sys::stg_block_blackhole()) };
    let actual = unsafe { super::stg_block_blackhole() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_blackhole() {
    unsafe { super::stg_block_blackhole() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_blackhole_finally() -> bool {
    let expected = unsafe { transmute(sys::stg_block_blackhole_finally()) };
    let actual = unsafe { super::stg_block_blackhole_finally() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_blackhole_finally() {
    unsafe { super::stg_block_blackhole_finally() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_takemvar() -> bool {
    let expected = unsafe { transmute(sys::stg_block_takemvar()) };
    let actual = unsafe { super::stg_block_takemvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_takemvar() {
    unsafe { super::stg_block_takemvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_readmvar() -> bool {
    let expected = unsafe { transmute(sys::stg_block_readmvar()) };
    let actual = unsafe { super::stg_block_readmvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_readmvar() {
    unsafe { super::stg_block_readmvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_putmvar() -> bool {
    let expected = unsafe { transmute(sys::stg_block_putmvar()) };
    let actual = unsafe { super::stg_block_putmvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_putmvar() {
    unsafe { super::stg_block_putmvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_stmwait() -> bool {
    let expected = unsafe { transmute(sys::stg_block_stmwait()) };
    let actual = unsafe { super::stg_block_stmwait() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_stmwait() {
    unsafe { super::stg_block_stmwait() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_throwto() -> bool {
    let expected = unsafe { transmute(sys::stg_block_throwto()) };
    let actual = unsafe { super::stg_block_throwto() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_throwto() {
    unsafe { super::stg_block_throwto() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readIOPortzh() -> bool {
    let expected = unsafe { transmute(sys::stg_readIOPortzh()) };
    let actual = unsafe { super::stg_readIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readIOPortzh() {
    unsafe { super::stg_readIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_writeIOPortzh() -> bool {
    let expected = unsafe { transmute(sys::stg_writeIOPortzh()) };
    let actual = unsafe { super::stg_writeIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_writeIOPortzh() {
    unsafe { super::stg_writeIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newIOPortzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newIOPortzh()) };
    let actual = unsafe { super::stg_newIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newIOPortzh() {
    unsafe { super::stg_newIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToStackTop() -> bool {
    let expected = unsafe { transmute(sys::stg_returnToStackTop()) };
    let actual = unsafe { super::stg_returnToStackTop() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToStackTop() {
    unsafe { super::stg_returnToStackTop() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSched() -> bool {
    let expected = unsafe { transmute(sys::stg_returnToSched()) };
    let actual = unsafe { super::stg_returnToSched() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSched() {
    unsafe { super::stg_returnToSched() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSchedNotPaused() -> bool {
    let expected = unsafe { transmute(sys::stg_returnToSchedNotPaused()) };
    let actual = unsafe { super::stg_returnToSchedNotPaused() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSchedNotPaused() {
    unsafe { super::stg_returnToSchedNotPaused() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSchedButFirst() -> bool {
    let expected = unsafe { transmute(sys::stg_returnToSchedButFirst()) };
    let actual = unsafe { super::stg_returnToSchedButFirst() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSchedButFirst() {
    unsafe { super::stg_returnToSchedButFirst() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadFinished() -> bool {
    let expected = unsafe { transmute(sys::stg_threadFinished()) };
    let actual = unsafe { super::stg_threadFinished() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadFinished() {
    unsafe { super::stg_threadFinished() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_StgReturn() -> bool {
    let expected = unsafe { transmute(sys::StgReturn()) };
    let actual = unsafe { super::StgReturn() };
    actual == expected
}

#[test]
#[ignore]
fn test_StgReturn() {
    unsafe { super::StgReturn() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeFloatzuIntzh() -> bool {
    let expected = unsafe { transmute(sys::stg_decodeFloatzuIntzh()) };
    let actual = unsafe { super::stg_decodeFloatzuIntzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeFloatzuIntzh() {
    unsafe { super::stg_decodeFloatzuIntzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeDoublezu2Intzh() -> bool {
    let expected = unsafe { transmute(sys::stg_decodeDoublezu2Intzh()) };
    let actual = unsafe { super::stg_decodeDoublezu2Intzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeDoublezu2Intzh() {
    unsafe { super::stg_decodeDoublezu2Intzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeDoublezuInt64zh() -> bool {
    let expected = unsafe { transmute(sys::stg_decodeDoublezuInt64zh()) };
    let actual = unsafe { super::stg_decodeDoublezuInt64zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeDoublezuInt64zh() {
    unsafe { super::stg_decodeDoublezuInt64zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unsafeThawArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_unsafeThawArrayzh()) };
    let actual = unsafe { super::stg_unsafeThawArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unsafeThawArrayzh() {
    unsafe { super::stg_unsafeThawArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casArrayzh()) };
    let actual = unsafe { super::stg_casArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casArrayzh() {
    unsafe { super::stg_casArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newByteArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newByteArrayzh()) };
    let actual = unsafe { super::stg_newByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newByteArrayzh() {
    unsafe { super::stg_newByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newPinnedByteArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newPinnedByteArrayzh()) };
    let actual = unsafe { super::stg_newPinnedByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newPinnedByteArrayzh() {
    unsafe { super::stg_newPinnedByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newAlignedPinnedByteArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newAlignedPinnedByteArrayzh()) };
    let actual = unsafe { super::stg_newAlignedPinnedByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newAlignedPinnedByteArrayzh() {
    unsafe { super::stg_newAlignedPinnedByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isByteArrayPinnedzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isByteArrayPinnedzh()) };
    let actual = unsafe { super::stg_isByteArrayPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isByteArrayPinnedzh() {
    unsafe { super::stg_isByteArrayPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isMutableByteArrayPinnedzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isMutableByteArrayPinnedzh()) };
    let actual = unsafe { super::stg_isMutableByteArrayPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isMutableByteArrayPinnedzh() {
    unsafe { super::stg_isMutableByteArrayPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isByteArrayWeaklyPinnedzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isByteArrayWeaklyPinnedzh()) };
    let actual = unsafe { super::stg_isByteArrayWeaklyPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isByteArrayWeaklyPinnedzh() {
    unsafe { super::stg_isByteArrayWeaklyPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isMutableByteArrayWeaklyPinnedzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isMutableByteArrayWeaklyPinnedzh()) };
    let actual = unsafe { super::stg_isMutableByteArrayWeaklyPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isMutableByteArrayWeaklyPinnedzh() {
    unsafe { super::stg_isMutableByteArrayWeaklyPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_shrinkMutableByteArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_shrinkMutableByteArrayzh()) };
    let actual = unsafe { super::stg_shrinkMutableByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_shrinkMutableByteArrayzh() {
    unsafe { super::stg_shrinkMutableByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_resizzeMutableByteArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_resizzeMutableByteArrayzh()) };
    let actual = unsafe { super::stg_resizzeMutableByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_resizzeMutableByteArrayzh() {
    unsafe { super::stg_resizzeMutableByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_shrinkSmallMutableArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_shrinkSmallMutableArrayzh()) };
    let actual = unsafe { super::stg_shrinkSmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_shrinkSmallMutableArrayzh() {
    unsafe { super::stg_shrinkSmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casIntArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casIntArrayzh()) };
    let actual = unsafe { super::stg_casIntArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casIntArrayzh() {
    unsafe { super::stg_casIntArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt8Arrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casInt8Arrayzh()) };
    let actual = unsafe { super::stg_casInt8Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt8Arrayzh() {
    unsafe { super::stg_casInt8Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt16Arrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casInt16Arrayzh()) };
    let actual = unsafe { super::stg_casInt16Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt16Arrayzh() {
    unsafe { super::stg_casInt16Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt32Arrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casInt32Arrayzh()) };
    let actual = unsafe { super::stg_casInt32Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt32Arrayzh() {
    unsafe { super::stg_casInt32Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt64Arrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casInt64Arrayzh()) };
    let actual = unsafe { super::stg_casInt64Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt64Arrayzh() {
    unsafe { super::stg_casInt64Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newArrayzh()) };
    let actual = unsafe { super::stg_newArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newArrayzh() {
    unsafe { super::stg_newArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_copyArrayzh()) };
    let actual = unsafe { super::stg_copyArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyArrayzh() {
    unsafe { super::stg_copyArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyMutableArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_copyMutableArrayzh()) };
    let actual = unsafe { super::stg_copyMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyMutableArrayzh() {
    unsafe { super::stg_copyMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_cloneArrayzh()) };
    let actual = unsafe { super::stg_cloneArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneArrayzh() {
    unsafe { super::stg_cloneArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneMutableArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_cloneMutableArrayzh()) };
    let actual = unsafe { super::stg_cloneMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneMutableArrayzh() {
    unsafe { super::stg_cloneMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_freezzeArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_freezzeArrayzh()) };
    let actual = unsafe { super::stg_freezzeArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_freezzeArrayzh() {
    unsafe { super::stg_freezzeArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_thawArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_thawArrayzh()) };
    let actual = unsafe { super::stg_thawArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_thawArrayzh() {
    unsafe { super::stg_thawArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newSmallArrayzh()) };
    let actual = unsafe { super::stg_newSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newSmallArrayzh() {
    unsafe { super::stg_newSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unsafeThawSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_unsafeThawSmallArrayzh()) };
    let actual = unsafe { super::stg_unsafeThawSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unsafeThawSmallArrayzh() {
    unsafe { super::stg_unsafeThawSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_cloneSmallArrayzh()) };
    let actual = unsafe { super::stg_cloneSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneSmallArrayzh() {
    unsafe { super::stg_cloneSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneSmallMutableArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_cloneSmallMutableArrayzh()) };
    let actual = unsafe { super::stg_cloneSmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneSmallMutableArrayzh() {
    unsafe { super::stg_cloneSmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_freezzeSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_freezzeSmallArrayzh()) };
    let actual = unsafe { super::stg_freezzeSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_freezzeSmallArrayzh() {
    unsafe { super::stg_freezzeSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_thawSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_thawSmallArrayzh()) };
    let actual = unsafe { super::stg_thawSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_thawSmallArrayzh() {
    unsafe { super::stg_thawSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copySmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_copySmallArrayzh()) };
    let actual = unsafe { super::stg_copySmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copySmallArrayzh() {
    unsafe { super::stg_copySmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copySmallMutableArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_copySmallMutableArrayzh()) };
    let actual = unsafe { super::stg_copySmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copySmallMutableArrayzh() {
    unsafe { super::stg_copySmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casSmallArrayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casSmallArrayzh()) };
    let actual = unsafe { super::stg_casSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casSmallArrayzh() {
    unsafe { super::stg_casSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newMutVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newMutVarzh()) };
    let actual = unsafe { super::stg_newMutVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newMutVarzh() {
    unsafe { super::stg_newMutVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicModifyMutVar2zh() -> bool {
    let expected = unsafe { transmute(sys::stg_atomicModifyMutVar2zh()) };
    let actual = unsafe { super::stg_atomicModifyMutVar2zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicModifyMutVar2zh() {
    unsafe { super::stg_atomicModifyMutVar2zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicModifyMutVarzuzh() -> bool {
    let expected = unsafe { transmute(sys::stg_atomicModifyMutVarzuzh()) };
    let actual = unsafe { super::stg_atomicModifyMutVarzuzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicModifyMutVarzuzh() {
    unsafe { super::stg_atomicModifyMutVarzuzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casMutVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_casMutVarzh()) };
    let actual = unsafe { super::stg_casMutVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casMutVarzh() {
    unsafe { super::stg_casMutVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isEmptyMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isEmptyMVarzh()) };
    let actual = unsafe { super::stg_isEmptyMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isEmptyMVarzh() {
    unsafe { super::stg_isEmptyMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newMVarzh()) };
    let actual = unsafe { super::stg_newMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newMVarzh() {
    unsafe { super::stg_newMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_takeMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_takeMVarzh()) };
    let actual = unsafe { super::stg_takeMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_takeMVarzh() {
    unsafe { super::stg_takeMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_putMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_putMVarzh()) };
    let actual = unsafe { super::stg_putMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_putMVarzh() {
    unsafe { super::stg_putMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_readMVarzh()) };
    let actual = unsafe { super::stg_readMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readMVarzh() {
    unsafe { super::stg_readMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryTakeMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_tryTakeMVarzh()) };
    let actual = unsafe { super::stg_tryTakeMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryTakeMVarzh() {
    unsafe { super::stg_tryTakeMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryPutMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_tryPutMVarzh()) };
    let actual = unsafe { super::stg_tryPutMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryPutMVarzh() {
    unsafe { super::stg_tryPutMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryReadMVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_tryReadMVarzh()) };
    let actual = unsafe { super::stg_tryReadMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryReadMVarzh() {
    unsafe { super::stg_tryReadMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_waitReadzh() -> bool {
    let expected = unsafe { transmute(sys::stg_waitReadzh()) };
    let actual = unsafe { super::stg_waitReadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_waitReadzh() {
    unsafe { super::stg_waitReadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_waitWritezh() -> bool {
    let expected = unsafe { transmute(sys::stg_waitWritezh()) };
    let actual = unsafe { super::stg_waitWritezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_waitWritezh() {
    unsafe { super::stg_waitWritezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_delayzh() -> bool {
    let expected = unsafe { transmute(sys::stg_delayzh()) };
    let actual = unsafe { super::stg_delayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_delayzh() {
    unsafe { super::stg_delayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchzh() -> bool {
    let expected = unsafe { transmute(sys::stg_catchzh()) };
    let actual = unsafe { super::stg_catchzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchzh() {
    unsafe { super::stg_catchzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raisezh() -> bool {
    let expected = unsafe { transmute(sys::stg_raisezh()) };
    let actual = unsafe { super::stg_raisezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raisezh() {
    unsafe { super::stg_raisezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseDivZZerozh() -> bool {
    let expected = unsafe { transmute(sys::stg_raiseDivZZerozh()) };
    let actual = unsafe { super::stg_raiseDivZZerozh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseDivZZerozh() {
    unsafe { super::stg_raiseDivZZerozh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseUnderflowzh() -> bool {
    let expected = unsafe { transmute(sys::stg_raiseUnderflowzh()) };
    let actual = unsafe { super::stg_raiseUnderflowzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseUnderflowzh() {
    unsafe { super::stg_raiseUnderflowzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseOverflowzh() -> bool {
    let expected = unsafe { transmute(sys::stg_raiseOverflowzh()) };
    let actual = unsafe { super::stg_raiseOverflowzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseOverflowzh() {
    unsafe { super::stg_raiseOverflowzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseIOzh() -> bool {
    let expected = unsafe { transmute(sys::stg_raiseIOzh()) };
    let actual = unsafe { super::stg_raiseIOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseIOzh() {
    unsafe { super::stg_raiseIOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_paniczh() -> bool {
    let expected = unsafe { transmute(sys::stg_paniczh()) };
    let actual = unsafe { super::stg_paniczh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_paniczh() {
    unsafe { super::stg_paniczh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_keepAlivezh() -> bool {
    let expected = unsafe { transmute(sys::stg_keepAlivezh()) };
    let actual = unsafe { super::stg_keepAlivezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_keepAlivezh() {
    unsafe { super::stg_keepAlivezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_absentErrorzh() -> bool {
    let expected = unsafe { transmute(sys::stg_absentErrorzh()) };
    let actual = unsafe { super::stg_absentErrorzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_absentErrorzh() {
    unsafe { super::stg_absentErrorzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newPromptTagzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newPromptTagzh()) };
    let actual = unsafe { super::stg_newPromptTagzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newPromptTagzh() {
    unsafe { super::stg_newPromptTagzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_promptzh() -> bool {
    let expected = unsafe { transmute(sys::stg_promptzh()) };
    let actual = unsafe { super::stg_promptzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_promptzh() {
    unsafe { super::stg_promptzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_control0zh() -> bool {
    let expected = unsafe { transmute(sys::stg_control0zh()) };
    let actual = unsafe { super::stg_control0zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_control0zh() {
    unsafe { super::stg_control0zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_control0zh_ll() -> bool {
    let expected = unsafe { transmute(sys::stg_control0zh_ll()) };
    let actual = unsafe { super::stg_control0zh_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_control0zh_ll() {
    unsafe { super::stg_control0zh_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_makeStableNamezh() -> bool {
    let expected = unsafe { transmute(sys::stg_makeStableNamezh()) };
    let actual = unsafe { super::stg_makeStableNamezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_makeStableNamezh() {
    unsafe { super::stg_makeStableNamezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_makeStablePtrzh() -> bool {
    let expected = unsafe { transmute(sys::stg_makeStablePtrzh()) };
    let actual = unsafe { super::stg_makeStablePtrzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_makeStablePtrzh() {
    unsafe { super::stg_makeStablePtrzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_deRefStablePtrzh() -> bool {
    let expected = unsafe { transmute(sys::stg_deRefStablePtrzh()) };
    let actual = unsafe { super::stg_deRefStablePtrzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_deRefStablePtrzh() {
    unsafe { super::stg_deRefStablePtrzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAddzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactAddzh()) };
    let actual = unsafe { super::stg_compactAddzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAddzh() {
    unsafe { super::stg_compactAddzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAddWithSharingzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactAddWithSharingzh()) };
    let actual = unsafe { super::stg_compactAddWithSharingzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAddWithSharingzh() {
    unsafe { super::stg_compactAddWithSharingzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactNewzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactNewzh()) };
    let actual = unsafe { super::stg_compactNewzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactNewzh() {
    unsafe { super::stg_compactNewzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAppendzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactAppendzh()) };
    let actual = unsafe { super::stg_compactAppendzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAppendzh() {
    unsafe { super::stg_compactAppendzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactResizzezh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactResizzezh()) };
    let actual = unsafe { super::stg_compactResizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactResizzezh() {
    unsafe { super::stg_compactResizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetRootzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactGetRootzh()) };
    let actual = unsafe { super::stg_compactGetRootzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetRootzh() {
    unsafe { super::stg_compactGetRootzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactContainszh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactContainszh()) };
    let actual = unsafe { super::stg_compactContainszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactContainszh() {
    unsafe { super::stg_compactContainszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactContainsAnyzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactContainsAnyzh()) };
    let actual = unsafe { super::stg_compactContainsAnyzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactContainsAnyzh() {
    unsafe { super::stg_compactContainsAnyzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetFirstBlockzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactGetFirstBlockzh()) };
    let actual = unsafe { super::stg_compactGetFirstBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetFirstBlockzh() {
    unsafe { super::stg_compactGetFirstBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetNextBlockzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactGetNextBlockzh()) };
    let actual = unsafe { super::stg_compactGetNextBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetNextBlockzh() {
    unsafe { super::stg_compactGetNextBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAllocateBlockzh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactAllocateBlockzh()) };
    let actual = unsafe { super::stg_compactAllocateBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAllocateBlockzh() {
    unsafe { super::stg_compactAllocateBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactFixupPointerszh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactFixupPointerszh()) };
    let actual = unsafe { super::stg_compactFixupPointerszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactFixupPointerszh() {
    unsafe { super::stg_compactFixupPointerszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactSizzezh() -> bool {
    let expected = unsafe { transmute(sys::stg_compactSizzezh()) };
    let actual = unsafe { super::stg_compactSizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactSizzezh() {
    unsafe { super::stg_compactSizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_forkzh() -> bool {
    let expected = unsafe { transmute(sys::stg_forkzh()) };
    let actual = unsafe { super::stg_forkzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_forkzh() {
    unsafe { super::stg_forkzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_forkOnzh() -> bool {
    let expected = unsafe { transmute(sys::stg_forkOnzh()) };
    let actual = unsafe { super::stg_forkOnzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_forkOnzh() {
    unsafe { super::stg_forkOnzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yieldzh() -> bool {
    let expected = unsafe { transmute(sys::stg_yieldzh()) };
    let actual = unsafe { super::stg_yieldzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yieldzh() {
    unsafe { super::stg_yieldzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_killMyself() -> bool {
    let expected = unsafe { transmute(sys::stg_killMyself()) };
    let actual = unsafe { super::stg_killMyself() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_killMyself() {
    unsafe { super::stg_killMyself() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_killThreadzh() -> bool {
    let expected = unsafe { transmute(sys::stg_killThreadzh()) };
    let actual = unsafe { super::stg_killThreadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_killThreadzh() {
    unsafe { super::stg_killThreadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getMaskingStatezh() -> bool {
    let expected = unsafe { transmute(sys::stg_getMaskingStatezh()) };
    let actual = unsafe { super::stg_getMaskingStatezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getMaskingStatezh() {
    unsafe { super::stg_getMaskingStatezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_maskAsyncExceptionszh() -> bool {
    let expected = unsafe { transmute(sys::stg_maskAsyncExceptionszh()) };
    let actual = unsafe { super::stg_maskAsyncExceptionszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_maskAsyncExceptionszh() {
    unsafe { super::stg_maskAsyncExceptionszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_maskUninterruptiblezh() -> bool {
    let expected = unsafe { transmute(sys::stg_maskUninterruptiblezh()) };
    let actual = unsafe { super::stg_maskUninterruptiblezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_maskUninterruptiblezh() {
    unsafe { super::stg_maskUninterruptiblezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unmaskAsyncExceptionszh() -> bool {
    let expected = unsafe { transmute(sys::stg_unmaskAsyncExceptionszh()) };
    let actual = unsafe { super::stg_unmaskAsyncExceptionszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unmaskAsyncExceptionszh() {
    unsafe { super::stg_unmaskAsyncExceptionszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_myThreadIdzh() -> bool {
    let expected = unsafe { transmute(sys::stg_myThreadIdzh()) };
    let actual = unsafe { super::stg_myThreadIdzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_myThreadIdzh() {
    unsafe { super::stg_myThreadIdzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_labelThreadzh() -> bool {
    let expected = unsafe { transmute(sys::stg_labelThreadzh()) };
    let actual = unsafe { super::stg_labelThreadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_labelThreadzh() {
    unsafe { super::stg_labelThreadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isCurrentThreadBoundzh() -> bool {
    let expected = unsafe { transmute(sys::stg_isCurrentThreadBoundzh()) };
    let actual = unsafe { super::stg_isCurrentThreadBoundzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isCurrentThreadBoundzh() {
    unsafe { super::stg_isCurrentThreadBoundzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadLabelzh() -> bool {
    let expected = unsafe { transmute(sys::stg_threadLabelzh()) };
    let actual = unsafe { super::stg_threadLabelzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadLabelzh() {
    unsafe { super::stg_threadLabelzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadStatuszh() -> bool {
    let expected = unsafe { transmute(sys::stg_threadStatuszh()) };
    let actual = unsafe { super::stg_threadStatuszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadStatuszh() {
    unsafe { super::stg_threadStatuszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_listThreadszh() -> bool {
    let expected = unsafe { transmute(sys::stg_listThreadszh()) };
    let actual = unsafe { super::stg_listThreadszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_listThreadszh() {
    unsafe { super::stg_listThreadszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakzh() -> bool {
    let expected = unsafe { transmute(sys::stg_mkWeakzh()) };
    let actual = unsafe { super::stg_mkWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakzh() {
    unsafe { super::stg_mkWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakNoFinalizzerzh() -> bool {
    let expected = unsafe { transmute(sys::stg_mkWeakNoFinalizzerzh()) };
    let actual = unsafe { super::stg_mkWeakNoFinalizzerzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakNoFinalizzerzh() {
    unsafe { super::stg_mkWeakNoFinalizzerzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakForeignzh() -> bool {
    let expected = unsafe { transmute(sys::stg_mkWeakForeignzh()) };
    let actual = unsafe { super::stg_mkWeakForeignzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakForeignzh() {
    unsafe { super::stg_mkWeakForeignzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_addCFinalizzerToWeakzh() -> bool {
    let expected = unsafe { transmute(sys::stg_addCFinalizzerToWeakzh()) };
    let actual = unsafe { super::stg_addCFinalizzerToWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_addCFinalizzerToWeakzh() {
    unsafe { super::stg_addCFinalizzerToWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_finalizzeWeakzh() -> bool {
    let expected = unsafe { transmute(sys::stg_finalizzeWeakzh()) };
    let actual = unsafe { super::stg_finalizzeWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_finalizzeWeakzh() {
    unsafe { super::stg_finalizzeWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_deRefWeakzh() -> bool {
    let expected = unsafe { transmute(sys::stg_deRefWeakzh()) };
    let actual = unsafe { super::stg_deRefWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_deRefWeakzh() {
    unsafe { super::stg_deRefWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_runRWzh() -> bool {
    let expected = unsafe { transmute(sys::stg_runRWzh()) };
    let actual = unsafe { super::stg_runRWzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_runRWzh() {
    unsafe { super::stg_runRWzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newBCOzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newBCOzh()) };
    let actual = unsafe { super::stg_newBCOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newBCOzh() {
    unsafe { super::stg_newBCOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkApUpd0zh() -> bool {
    let expected = unsafe { transmute(sys::stg_mkApUpd0zh()) };
    let actual = unsafe { super::stg_mkApUpd0zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkApUpd0zh() {
    unsafe { super::stg_mkApUpd0zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_retryzh() -> bool {
    let expected = unsafe { transmute(sys::stg_retryzh()) };
    let actual = unsafe { super::stg_retryzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_retryzh() {
    unsafe { super::stg_retryzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchRetryzh() -> bool {
    let expected = unsafe { transmute(sys::stg_catchRetryzh()) };
    let actual = unsafe { super::stg_catchRetryzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchRetryzh() {
    unsafe { super::stg_catchRetryzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchSTMzh() -> bool {
    let expected = unsafe { transmute(sys::stg_catchSTMzh()) };
    let actual = unsafe { super::stg_catchSTMzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchSTMzh() {
    unsafe { super::stg_catchSTMzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicallyzh() -> bool {
    let expected = unsafe { transmute(sys::stg_atomicallyzh()) };
    let actual = unsafe { super::stg_atomicallyzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicallyzh() {
    unsafe { super::stg_atomicallyzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newTVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_newTVarzh()) };
    let actual = unsafe { super::stg_newTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newTVarzh() {
    unsafe { super::stg_newTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readTVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_readTVarzh()) };
    let actual = unsafe { super::stg_readTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readTVarzh() {
    unsafe { super::stg_readTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readTVarIOzh() -> bool {
    let expected = unsafe { transmute(sys::stg_readTVarIOzh()) };
    let actual = unsafe { super::stg_readTVarIOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readTVarIOzh() {
    unsafe { super::stg_readTVarIOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_writeTVarzh() -> bool {
    let expected = unsafe { transmute(sys::stg_writeTVarzh()) };
    let actual = unsafe { super::stg_writeTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_writeTVarzh() {
    unsafe { super::stg_writeTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unpackClosurezh() -> bool {
    let expected = unsafe { transmute(sys::stg_unpackClosurezh()) };
    let actual = unsafe { super::stg_unpackClosurezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unpackClosurezh() {
    unsafe { super::stg_unpackClosurezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_closureSizzezh() -> bool {
    let expected = unsafe { transmute(sys::stg_closureSizzezh()) };
    let actual = unsafe { super::stg_closureSizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_closureSizzezh() {
    unsafe { super::stg_closureSizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_whereFromzh() -> bool {
    let expected = unsafe { transmute(sys::stg_whereFromzh()) };
    let actual = unsafe { super::stg_whereFromzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_whereFromzh() {
    unsafe { super::stg_whereFromzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getApStackValzh() -> bool {
    let expected = unsafe { transmute(sys::stg_getApStackValzh()) };
    let actual = unsafe { super::stg_getApStackValzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getApStackValzh() {
    unsafe { super::stg_getApStackValzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getSparkzh() -> bool {
    let expected = unsafe { transmute(sys::stg_getSparkzh()) };
    let actual = unsafe { super::stg_getSparkzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getSparkzh() {
    unsafe { super::stg_getSparkzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_numSparkszh() -> bool {
    let expected = unsafe { transmute(sys::stg_numSparkszh()) };
    let actual = unsafe { super::stg_numSparkszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_numSparkszh() {
    unsafe { super::stg_numSparkszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_noDuplicatezh() -> bool {
    let expected = unsafe { transmute(sys::stg_noDuplicatezh()) };
    let actual = unsafe { super::stg_noDuplicatezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_noDuplicatezh() {
    unsafe { super::stg_noDuplicatezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_clearCCSzh() -> bool {
    let expected = unsafe { transmute(sys::stg_clearCCSzh()) };
    let actual = unsafe { super::stg_clearCCSzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_clearCCSzh() {
    unsafe { super::stg_clearCCSzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceEventzh() -> bool {
    let expected = unsafe { transmute(sys::stg_traceEventzh()) };
    let actual = unsafe { super::stg_traceEventzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceEventzh() {
    unsafe { super::stg_traceEventzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceBinaryEventzh() -> bool {
    let expected = unsafe { transmute(sys::stg_traceBinaryEventzh()) };
    let actual = unsafe { super::stg_traceBinaryEventzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceBinaryEventzh() {
    unsafe { super::stg_traceBinaryEventzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceMarkerzh() -> bool {
    let expected = unsafe { transmute(sys::stg_traceMarkerzh()) };
    let actual = unsafe { super::stg_traceMarkerzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceMarkerzh() {
    unsafe { super::stg_traceMarkerzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getThreadAllocationCounterzh() -> bool {
    let expected = unsafe { transmute(sys::stg_getThreadAllocationCounterzh()) };
    let actual = unsafe { super::stg_getThreadAllocationCounterzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getThreadAllocationCounterzh() {
    unsafe { super::stg_getThreadAllocationCounterzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_setThreadAllocationCounterzh() -> bool {
    let expected = unsafe { transmute(sys::stg_setThreadAllocationCounterzh()) };
    let actual = unsafe { super::stg_setThreadAllocationCounterzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_setThreadAllocationCounterzh() {
    unsafe { super::stg_setThreadAllocationCounterzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castWord64ToDoublezh() -> bool {
    let expected = unsafe { transmute(sys::stg_castWord64ToDoublezh()) };
    let actual = unsafe { super::stg_castWord64ToDoublezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castWord64ToDoublezh() {
    unsafe { super::stg_castWord64ToDoublezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castDoubleToWord64zh() -> bool {
    let expected = unsafe { transmute(sys::stg_castDoubleToWord64zh()) };
    let actual = unsafe { super::stg_castDoubleToWord64zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castDoubleToWord64zh() {
    unsafe { super::stg_castDoubleToWord64zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castWord32ToFloatzh() -> bool {
    let expected = unsafe { transmute(sys::stg_castWord32ToFloatzh()) };
    let actual = unsafe { super::stg_castWord32ToFloatzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castWord32ToFloatzh() {
    unsafe { super::stg_castWord32ToFloatzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castFloatToWord32zh() -> bool {
    let expected = unsafe { transmute(sys::stg_castFloatToWord32zh()) };
    let actual = unsafe { super::stg_castFloatToWord32zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castFloatToWord32zh() {
    unsafe { super::stg_castFloatToWord32zh() };
    todo!("assert")
}
