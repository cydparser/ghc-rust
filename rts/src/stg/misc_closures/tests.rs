use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ctoi_t() -> bool {
    let expected = unsafe { sys::stg_ctoi_t() };
    let actual = unsafe { stg_ctoi_t() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ctoi_t() {
    unsafe { stg_ctoi_t() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_0_fast() -> bool {
    let expected = unsafe { sys::stg_ap_0_fast() };
    let actual = unsafe { stg_ap_0_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_0_fast() {
    unsafe { stg_ap_0_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v_fast() -> bool {
    let expected = unsafe { sys::stg_ap_v_fast() };
    let actual = unsafe { stg_ap_v_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v_fast() {
    unsafe { stg_ap_v_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_f_fast() -> bool {
    let expected = unsafe { sys::stg_ap_f_fast() };
    let actual = unsafe { stg_ap_f_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_f_fast() {
    unsafe { stg_ap_f_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_d_fast() -> bool {
    let expected = unsafe { sys::stg_ap_d_fast() };
    let actual = unsafe { stg_ap_d_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_d_fast() {
    unsafe { stg_ap_d_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_l_fast() -> bool {
    let expected = unsafe { sys::stg_ap_l_fast() };
    let actual = unsafe { stg_ap_l_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_l_fast() {
    unsafe { stg_ap_l_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v16_fast() -> bool {
    let expected = unsafe { sys::stg_ap_v16_fast() };
    let actual = unsafe { stg_ap_v16_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v16_fast() {
    unsafe { stg_ap_v16_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v32_fast() -> bool {
    let expected = unsafe { sys::stg_ap_v32_fast() };
    let actual = unsafe { stg_ap_v32_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v32_fast() {
    unsafe { stg_ap_v32_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_v64_fast() -> bool {
    let expected = unsafe { sys::stg_ap_v64_fast() };
    let actual = unsafe { stg_ap_v64_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_v64_fast() {
    unsafe { stg_ap_v64_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_n_fast() -> bool {
    let expected = unsafe { sys::stg_ap_n_fast() };
    let actual = unsafe { stg_ap_n_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_n_fast() {
    unsafe { stg_ap_n_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_p_fast() -> bool {
    let expected = unsafe { sys::stg_ap_p_fast() };
    let actual = unsafe { stg_ap_p_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_p_fast() {
    unsafe { stg_ap_p_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pv_fast() -> bool {
    let expected = unsafe { sys::stg_ap_pv_fast() };
    let actual = unsafe { stg_ap_pv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pv_fast() {
    unsafe { stg_ap_pv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pp_fast() -> bool {
    let expected = unsafe { sys::stg_ap_pp_fast() };
    let actual = unsafe { stg_ap_pp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pp_fast() {
    unsafe { stg_ap_pp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppv_fast() -> bool {
    let expected = unsafe { sys::stg_ap_ppv_fast() };
    let actual = unsafe { stg_ap_ppv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppv_fast() {
    unsafe { stg_ap_ppv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppp_fast() -> bool {
    let expected = unsafe { sys::stg_ap_ppp_fast() };
    let actual = unsafe { stg_ap_ppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppp_fast() {
    unsafe { stg_ap_ppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppv_fast() -> bool {
    let expected = unsafe { sys::stg_ap_pppv_fast() };
    let actual = unsafe { stg_ap_pppv_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppv_fast() {
    unsafe { stg_ap_pppv_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppp_fast() -> bool {
    let expected = unsafe { sys::stg_ap_pppp_fast() };
    let actual = unsafe { stg_ap_pppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppp_fast() {
    unsafe { stg_ap_pppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_ppppp_fast() -> bool {
    let expected = unsafe { sys::stg_ap_ppppp_fast() };
    let actual = unsafe { stg_ap_ppppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_ppppp_fast() {
    unsafe { stg_ap_ppppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_pppppp_fast() -> bool {
    let expected = unsafe { sys::stg_ap_pppppp_fast() };
    let actual = unsafe { stg_ap_pppppp_fast() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_pppppp_fast() {
    unsafe { stg_ap_pppppp_fast() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_PAP_apply() -> bool {
    let expected = unsafe { sys::stg_PAP_apply() };
    let actual = unsafe { stg_PAP_apply() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_PAP_apply() {
    unsafe { stg_PAP_apply() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_CONTINUATION_apply() -> bool {
    let expected = unsafe { sys::stg_CONTINUATION_apply() };
    let actual = unsafe { stg_CONTINUATION_apply() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_CONTINUATION_apply() {
    unsafe { stg_CONTINUATION_apply() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v16() -> bool {
    let expected = unsafe { sys::stg_ap_stk_v16() };
    let actual = unsafe { stg_ap_stk_v16() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v16() {
    unsafe { stg_ap_stk_v16() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v32() -> bool {
    let expected = unsafe { sys::stg_ap_stk_v32() };
    let actual = unsafe { stg_ap_stk_v32() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v32() {
    unsafe { stg_ap_stk_v32() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_ap_stk_v64() -> bool {
    let expected = unsafe { sys::stg_ap_stk_v64() };
    let actual = unsafe { stg_ap_stk_v64() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_ap_stk_v64() {
    unsafe { stg_ap_stk_v64() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v16() -> bool {
    let expected = unsafe { sys::stg_stk_save_v16() };
    let actual = unsafe { stg_stk_save_v16() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v16() {
    unsafe { stg_stk_save_v16() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v32() -> bool {
    let expected = unsafe { sys::stg_stk_save_v32() };
    let actual = unsafe { stg_stk_save_v32() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v32() {
    unsafe { stg_stk_save_v32() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_stk_save_v64() -> bool {
    let expected = unsafe { sys::stg_stk_save_v64() };
    let actual = unsafe { stg_stk_save_v64() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_stk_save_v64() {
    unsafe { stg_stk_save_v64() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_noregs() -> bool {
    let expected = unsafe { sys::stg_gc_noregs() };
    let actual = unsafe { stg_gc_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_noregs() {
    unsafe { stg_gc_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim() -> bool {
    let expected = unsafe { sys::stg_gc_prim() };
    let actual = unsafe { stg_gc_prim() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim() {
    unsafe { stg_gc_prim() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_p() -> bool {
    let expected = unsafe { sys::stg_gc_prim_p() };
    let actual = unsafe { stg_gc_prim_p() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_p() {
    unsafe { stg_gc_prim_p() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_pp() -> bool {
    let expected = unsafe { sys::stg_gc_prim_pp() };
    let actual = unsafe { stg_gc_prim_pp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_pp() {
    unsafe { stg_gc_prim_pp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_n() -> bool {
    let expected = unsafe { sys::stg_gc_prim_n() };
    let actual = unsafe { stg_gc_prim_n() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_n() {
    unsafe { stg_gc_prim_n() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_p_ll() -> bool {
    let expected = unsafe { sys::stg_gc_prim_p_ll() };
    let actual = unsafe { stg_gc_prim_p_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_p_ll() {
    unsafe { stg_gc_prim_p_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_prim_pp_ll() -> bool {
    let expected = unsafe { sys::stg_gc_prim_pp_ll() };
    let actual = unsafe { stg_gc_prim_pp_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_prim_pp_ll() {
    unsafe { stg_gc_prim_pp_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___stg_gc_enter_1() -> bool {
    let expected = unsafe { sys::__stg_gc_enter_1() };
    let actual = unsafe { __stg_gc_enter_1() };
    actual == expected
}

#[test]
#[ignore]
fn test___stg_gc_enter_1() {
    unsafe { __stg_gc_enter_1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_unpt_r1() -> bool {
    let expected = unsafe { sys::stg_gc_unpt_r1() };
    let actual = unsafe { stg_gc_unpt_r1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_unpt_r1() {
    unsafe { stg_gc_unpt_r1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_unbx_r1() -> bool {
    let expected = unsafe { sys::stg_gc_unbx_r1() };
    let actual = unsafe { stg_gc_unbx_r1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_unbx_r1() {
    unsafe { stg_gc_unbx_r1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_f1() -> bool {
    let expected = unsafe { sys::stg_gc_f1() };
    let actual = unsafe { stg_gc_f1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_f1() {
    unsafe { stg_gc_f1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_d1() -> bool {
    let expected = unsafe { sys::stg_gc_d1() };
    let actual = unsafe { stg_gc_d1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_d1() {
    unsafe { stg_gc_d1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_l1() -> bool {
    let expected = unsafe { sys::stg_gc_l1() };
    let actual = unsafe { stg_gc_l1() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_l1() {
    unsafe { stg_gc_l1() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_pp() -> bool {
    let expected = unsafe { sys::stg_gc_pp() };
    let actual = unsafe { stg_gc_pp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_pp() {
    unsafe { stg_gc_pp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_ppp() -> bool {
    let expected = unsafe { sys::stg_gc_ppp() };
    let actual = unsafe { stg_gc_ppp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_ppp() {
    unsafe { stg_gc_ppp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_gc_pppp() -> bool {
    let expected = unsafe { sys::stg_gc_pppp() };
    let actual = unsafe { stg_gc_pppp() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_gc_pppp() {
    unsafe { stg_gc_pppp() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___stg_gc_fun() -> bool {
    let expected = unsafe { sys::__stg_gc_fun() };
    let actual = unsafe { __stg_gc_fun() };
    actual == expected
}

#[test]
#[ignore]
fn test___stg_gc_fun() {
    unsafe { __stg_gc_fun() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yield_noregs() -> bool {
    let expected = unsafe { sys::stg_yield_noregs() };
    let actual = unsafe { stg_yield_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yield_noregs() {
    unsafe { stg_yield_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yield_to_interpreter() -> bool {
    let expected = unsafe { sys::stg_yield_to_interpreter() };
    let actual = unsafe { stg_yield_to_interpreter() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yield_to_interpreter() {
    unsafe { stg_yield_to_interpreter() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_noregs() -> bool {
    let expected = unsafe { sys::stg_block_noregs() };
    let actual = unsafe { stg_block_noregs() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_noregs() {
    unsafe { stg_block_noregs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_blackhole() -> bool {
    let expected = unsafe { sys::stg_block_blackhole() };
    let actual = unsafe { stg_block_blackhole() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_blackhole() {
    unsafe { stg_block_blackhole() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_blackhole_finally() -> bool {
    let expected = unsafe { sys::stg_block_blackhole_finally() };
    let actual = unsafe { stg_block_blackhole_finally() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_blackhole_finally() {
    unsafe { stg_block_blackhole_finally() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_takemvar() -> bool {
    let expected = unsafe { sys::stg_block_takemvar() };
    let actual = unsafe { stg_block_takemvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_takemvar() {
    unsafe { stg_block_takemvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_readmvar() -> bool {
    let expected = unsafe { sys::stg_block_readmvar() };
    let actual = unsafe { stg_block_readmvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_readmvar() {
    unsafe { stg_block_readmvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_putmvar() -> bool {
    let expected = unsafe { sys::stg_block_putmvar() };
    let actual = unsafe { stg_block_putmvar() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_putmvar() {
    unsafe { stg_block_putmvar() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_stmwait() -> bool {
    let expected = unsafe { sys::stg_block_stmwait() };
    let actual = unsafe { stg_block_stmwait() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_stmwait() {
    unsafe { stg_block_stmwait() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_block_throwto() -> bool {
    let expected = unsafe { sys::stg_block_throwto() };
    let actual = unsafe { stg_block_throwto() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_block_throwto() {
    unsafe { stg_block_throwto() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readIOPortzh() -> bool {
    let expected = unsafe { sys::stg_readIOPortzh() };
    let actual = unsafe { stg_readIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readIOPortzh() {
    unsafe { stg_readIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_writeIOPortzh() -> bool {
    let expected = unsafe { sys::stg_writeIOPortzh() };
    let actual = unsafe { stg_writeIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_writeIOPortzh() {
    unsafe { stg_writeIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newIOPortzh() -> bool {
    let expected = unsafe { sys::stg_newIOPortzh() };
    let actual = unsafe { stg_newIOPortzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newIOPortzh() {
    unsafe { stg_newIOPortzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToStackTop() -> bool {
    let expected = unsafe { sys::stg_returnToStackTop() };
    let actual = unsafe { stg_returnToStackTop() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToStackTop() {
    unsafe { stg_returnToStackTop() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSched() -> bool {
    let expected = unsafe { sys::stg_returnToSched() };
    let actual = unsafe { stg_returnToSched() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSched() {
    unsafe { stg_returnToSched() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSchedNotPaused() -> bool {
    let expected = unsafe { sys::stg_returnToSchedNotPaused() };
    let actual = unsafe { stg_returnToSchedNotPaused() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSchedNotPaused() {
    unsafe { stg_returnToSchedNotPaused() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_returnToSchedButFirst() -> bool {
    let expected = unsafe { sys::stg_returnToSchedButFirst() };
    let actual = unsafe { stg_returnToSchedButFirst() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_returnToSchedButFirst() {
    unsafe { stg_returnToSchedButFirst() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadFinished() -> bool {
    let expected = unsafe { sys::stg_threadFinished() };
    let actual = unsafe { stg_threadFinished() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadFinished() {
    unsafe { stg_threadFinished() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_StgReturn() -> bool {
    let expected = unsafe { sys::StgReturn() };
    let actual = unsafe { StgReturn() };
    actual == expected
}

#[test]
#[ignore]
fn test_StgReturn() {
    unsafe { StgReturn() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeFloatzuIntzh() -> bool {
    let expected = unsafe { sys::stg_decodeFloatzuIntzh() };
    let actual = unsafe { stg_decodeFloatzuIntzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeFloatzuIntzh() {
    unsafe { stg_decodeFloatzuIntzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeDoublezu2Intzh() -> bool {
    let expected = unsafe { sys::stg_decodeDoublezu2Intzh() };
    let actual = unsafe { stg_decodeDoublezu2Intzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeDoublezu2Intzh() {
    unsafe { stg_decodeDoublezu2Intzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_decodeDoublezuInt64zh() -> bool {
    let expected = unsafe { sys::stg_decodeDoublezuInt64zh() };
    let actual = unsafe { stg_decodeDoublezuInt64zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_decodeDoublezuInt64zh() {
    unsafe { stg_decodeDoublezuInt64zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unsafeThawArrayzh() -> bool {
    let expected = unsafe { sys::stg_unsafeThawArrayzh() };
    let actual = unsafe { stg_unsafeThawArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unsafeThawArrayzh() {
    unsafe { stg_unsafeThawArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casArrayzh() -> bool {
    let expected = unsafe { sys::stg_casArrayzh() };
    let actual = unsafe { stg_casArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casArrayzh() {
    unsafe { stg_casArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newByteArrayzh() -> bool {
    let expected = unsafe { sys::stg_newByteArrayzh() };
    let actual = unsafe { stg_newByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newByteArrayzh() {
    unsafe { stg_newByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newPinnedByteArrayzh() -> bool {
    let expected = unsafe { sys::stg_newPinnedByteArrayzh() };
    let actual = unsafe { stg_newPinnedByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newPinnedByteArrayzh() {
    unsafe { stg_newPinnedByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newAlignedPinnedByteArrayzh() -> bool {
    let expected = unsafe { sys::stg_newAlignedPinnedByteArrayzh() };
    let actual = unsafe { stg_newAlignedPinnedByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newAlignedPinnedByteArrayzh() {
    unsafe { stg_newAlignedPinnedByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isByteArrayPinnedzh() -> bool {
    let expected = unsafe { sys::stg_isByteArrayPinnedzh() };
    let actual = unsafe { stg_isByteArrayPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isByteArrayPinnedzh() {
    unsafe { stg_isByteArrayPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isMutableByteArrayPinnedzh() -> bool {
    let expected = unsafe { sys::stg_isMutableByteArrayPinnedzh() };
    let actual = unsafe { stg_isMutableByteArrayPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isMutableByteArrayPinnedzh() {
    unsafe { stg_isMutableByteArrayPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isByteArrayWeaklyPinnedzh() -> bool {
    let expected = unsafe { sys::stg_isByteArrayWeaklyPinnedzh() };
    let actual = unsafe { stg_isByteArrayWeaklyPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isByteArrayWeaklyPinnedzh() {
    unsafe { stg_isByteArrayWeaklyPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isMutableByteArrayWeaklyPinnedzh() -> bool {
    let expected = unsafe { sys::stg_isMutableByteArrayWeaklyPinnedzh() };
    let actual = unsafe { stg_isMutableByteArrayWeaklyPinnedzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isMutableByteArrayWeaklyPinnedzh() {
    unsafe { stg_isMutableByteArrayWeaklyPinnedzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_shrinkMutableByteArrayzh() -> bool {
    let expected = unsafe { sys::stg_shrinkMutableByteArrayzh() };
    let actual = unsafe { stg_shrinkMutableByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_shrinkMutableByteArrayzh() {
    unsafe { stg_shrinkMutableByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_resizzeMutableByteArrayzh() -> bool {
    let expected = unsafe { sys::stg_resizzeMutableByteArrayzh() };
    let actual = unsafe { stg_resizzeMutableByteArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_resizzeMutableByteArrayzh() {
    unsafe { stg_resizzeMutableByteArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_shrinkSmallMutableArrayzh() -> bool {
    let expected = unsafe { sys::stg_shrinkSmallMutableArrayzh() };
    let actual = unsafe { stg_shrinkSmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_shrinkSmallMutableArrayzh() {
    unsafe { stg_shrinkSmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casIntArrayzh() -> bool {
    let expected = unsafe { sys::stg_casIntArrayzh() };
    let actual = unsafe { stg_casIntArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casIntArrayzh() {
    unsafe { stg_casIntArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt8Arrayzh() -> bool {
    let expected = unsafe { sys::stg_casInt8Arrayzh() };
    let actual = unsafe { stg_casInt8Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt8Arrayzh() {
    unsafe { stg_casInt8Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt16Arrayzh() -> bool {
    let expected = unsafe { sys::stg_casInt16Arrayzh() };
    let actual = unsafe { stg_casInt16Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt16Arrayzh() {
    unsafe { stg_casInt16Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt32Arrayzh() -> bool {
    let expected = unsafe { sys::stg_casInt32Arrayzh() };
    let actual = unsafe { stg_casInt32Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt32Arrayzh() {
    unsafe { stg_casInt32Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casInt64Arrayzh() -> bool {
    let expected = unsafe { sys::stg_casInt64Arrayzh() };
    let actual = unsafe { stg_casInt64Arrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casInt64Arrayzh() {
    unsafe { stg_casInt64Arrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newArrayzh() -> bool {
    let expected = unsafe { sys::stg_newArrayzh() };
    let actual = unsafe { stg_newArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newArrayzh() {
    unsafe { stg_newArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyArrayzh() -> bool {
    let expected = unsafe { sys::stg_copyArrayzh() };
    let actual = unsafe { stg_copyArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyArrayzh() {
    unsafe { stg_copyArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copyMutableArrayzh() -> bool {
    let expected = unsafe { sys::stg_copyMutableArrayzh() };
    let actual = unsafe { stg_copyMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copyMutableArrayzh() {
    unsafe { stg_copyMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneArrayzh() -> bool {
    let expected = unsafe { sys::stg_cloneArrayzh() };
    let actual = unsafe { stg_cloneArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneArrayzh() {
    unsafe { stg_cloneArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneMutableArrayzh() -> bool {
    let expected = unsafe { sys::stg_cloneMutableArrayzh() };
    let actual = unsafe { stg_cloneMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneMutableArrayzh() {
    unsafe { stg_cloneMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_freezzeArrayzh() -> bool {
    let expected = unsafe { sys::stg_freezzeArrayzh() };
    let actual = unsafe { stg_freezzeArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_freezzeArrayzh() {
    unsafe { stg_freezzeArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_thawArrayzh() -> bool {
    let expected = unsafe { sys::stg_thawArrayzh() };
    let actual = unsafe { stg_thawArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_thawArrayzh() {
    unsafe { stg_thawArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_newSmallArrayzh() };
    let actual = unsafe { stg_newSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newSmallArrayzh() {
    unsafe { stg_newSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unsafeThawSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_unsafeThawSmallArrayzh() };
    let actual = unsafe { stg_unsafeThawSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unsafeThawSmallArrayzh() {
    unsafe { stg_unsafeThawSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_cloneSmallArrayzh() };
    let actual = unsafe { stg_cloneSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneSmallArrayzh() {
    unsafe { stg_cloneSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_cloneSmallMutableArrayzh() -> bool {
    let expected = unsafe { sys::stg_cloneSmallMutableArrayzh() };
    let actual = unsafe { stg_cloneSmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_cloneSmallMutableArrayzh() {
    unsafe { stg_cloneSmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_freezzeSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_freezzeSmallArrayzh() };
    let actual = unsafe { stg_freezzeSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_freezzeSmallArrayzh() {
    unsafe { stg_freezzeSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_thawSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_thawSmallArrayzh() };
    let actual = unsafe { stg_thawSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_thawSmallArrayzh() {
    unsafe { stg_thawSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copySmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_copySmallArrayzh() };
    let actual = unsafe { stg_copySmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copySmallArrayzh() {
    unsafe { stg_copySmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_copySmallMutableArrayzh() -> bool {
    let expected = unsafe { sys::stg_copySmallMutableArrayzh() };
    let actual = unsafe { stg_copySmallMutableArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_copySmallMutableArrayzh() {
    unsafe { stg_copySmallMutableArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casSmallArrayzh() -> bool {
    let expected = unsafe { sys::stg_casSmallArrayzh() };
    let actual = unsafe { stg_casSmallArrayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casSmallArrayzh() {
    unsafe { stg_casSmallArrayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newMutVarzh() -> bool {
    let expected = unsafe { sys::stg_newMutVarzh() };
    let actual = unsafe { stg_newMutVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newMutVarzh() {
    unsafe { stg_newMutVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicModifyMutVar2zh() -> bool {
    let expected = unsafe { sys::stg_atomicModifyMutVar2zh() };
    let actual = unsafe { stg_atomicModifyMutVar2zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicModifyMutVar2zh() {
    unsafe { stg_atomicModifyMutVar2zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicModifyMutVarzuzh() -> bool {
    let expected = unsafe { sys::stg_atomicModifyMutVarzuzh() };
    let actual = unsafe { stg_atomicModifyMutVarzuzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicModifyMutVarzuzh() {
    unsafe { stg_atomicModifyMutVarzuzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_casMutVarzh() -> bool {
    let expected = unsafe { sys::stg_casMutVarzh() };
    let actual = unsafe { stg_casMutVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_casMutVarzh() {
    unsafe { stg_casMutVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isEmptyMVarzh() -> bool {
    let expected = unsafe { sys::stg_isEmptyMVarzh() };
    let actual = unsafe { stg_isEmptyMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isEmptyMVarzh() {
    unsafe { stg_isEmptyMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newMVarzh() -> bool {
    let expected = unsafe { sys::stg_newMVarzh() };
    let actual = unsafe { stg_newMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newMVarzh() {
    unsafe { stg_newMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_takeMVarzh() -> bool {
    let expected = unsafe { sys::stg_takeMVarzh() };
    let actual = unsafe { stg_takeMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_takeMVarzh() {
    unsafe { stg_takeMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_putMVarzh() -> bool {
    let expected = unsafe { sys::stg_putMVarzh() };
    let actual = unsafe { stg_putMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_putMVarzh() {
    unsafe { stg_putMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readMVarzh() -> bool {
    let expected = unsafe { sys::stg_readMVarzh() };
    let actual = unsafe { stg_readMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readMVarzh() {
    unsafe { stg_readMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryTakeMVarzh() -> bool {
    let expected = unsafe { sys::stg_tryTakeMVarzh() };
    let actual = unsafe { stg_tryTakeMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryTakeMVarzh() {
    unsafe { stg_tryTakeMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryPutMVarzh() -> bool {
    let expected = unsafe { sys::stg_tryPutMVarzh() };
    let actual = unsafe { stg_tryPutMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryPutMVarzh() {
    unsafe { stg_tryPutMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_tryReadMVarzh() -> bool {
    let expected = unsafe { sys::stg_tryReadMVarzh() };
    let actual = unsafe { stg_tryReadMVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_tryReadMVarzh() {
    unsafe { stg_tryReadMVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_waitReadzh() -> bool {
    let expected = unsafe { sys::stg_waitReadzh() };
    let actual = unsafe { stg_waitReadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_waitReadzh() {
    unsafe { stg_waitReadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_waitWritezh() -> bool {
    let expected = unsafe { sys::stg_waitWritezh() };
    let actual = unsafe { stg_waitWritezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_waitWritezh() {
    unsafe { stg_waitWritezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_delayzh() -> bool {
    let expected = unsafe { sys::stg_delayzh() };
    let actual = unsafe { stg_delayzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_delayzh() {
    unsafe { stg_delayzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchzh() -> bool {
    let expected = unsafe { sys::stg_catchzh() };
    let actual = unsafe { stg_catchzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchzh() {
    unsafe { stg_catchzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raisezh() -> bool {
    let expected = unsafe { sys::stg_raisezh() };
    let actual = unsafe { stg_raisezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raisezh() {
    unsafe { stg_raisezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseDivZZerozh() -> bool {
    let expected = unsafe { sys::stg_raiseDivZZerozh() };
    let actual = unsafe { stg_raiseDivZZerozh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseDivZZerozh() {
    unsafe { stg_raiseDivZZerozh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseUnderflowzh() -> bool {
    let expected = unsafe { sys::stg_raiseUnderflowzh() };
    let actual = unsafe { stg_raiseUnderflowzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseUnderflowzh() {
    unsafe { stg_raiseUnderflowzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseOverflowzh() -> bool {
    let expected = unsafe { sys::stg_raiseOverflowzh() };
    let actual = unsafe { stg_raiseOverflowzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseOverflowzh() {
    unsafe { stg_raiseOverflowzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_raiseIOzh() -> bool {
    let expected = unsafe { sys::stg_raiseIOzh() };
    let actual = unsafe { stg_raiseIOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_raiseIOzh() {
    unsafe { stg_raiseIOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_paniczh() -> bool {
    let expected = unsafe { sys::stg_paniczh() };
    let actual = unsafe { stg_paniczh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_paniczh() {
    unsafe { stg_paniczh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_keepAlivezh() -> bool {
    let expected = unsafe { sys::stg_keepAlivezh() };
    let actual = unsafe { stg_keepAlivezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_keepAlivezh() {
    unsafe { stg_keepAlivezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_absentErrorzh() -> bool {
    let expected = unsafe { sys::stg_absentErrorzh() };
    let actual = unsafe { stg_absentErrorzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_absentErrorzh() {
    unsafe { stg_absentErrorzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newPromptTagzh() -> bool {
    let expected = unsafe { sys::stg_newPromptTagzh() };
    let actual = unsafe { stg_newPromptTagzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newPromptTagzh() {
    unsafe { stg_newPromptTagzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_promptzh() -> bool {
    let expected = unsafe { sys::stg_promptzh() };
    let actual = unsafe { stg_promptzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_promptzh() {
    unsafe { stg_promptzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_control0zh() -> bool {
    let expected = unsafe { sys::stg_control0zh() };
    let actual = unsafe { stg_control0zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_control0zh() {
    unsafe { stg_control0zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_control0zh_ll() -> bool {
    let expected = unsafe { sys::stg_control0zh_ll() };
    let actual = unsafe { stg_control0zh_ll() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_control0zh_ll() {
    unsafe { stg_control0zh_ll() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_makeStableNamezh() -> bool {
    let expected = unsafe { sys::stg_makeStableNamezh() };
    let actual = unsafe { stg_makeStableNamezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_makeStableNamezh() {
    unsafe { stg_makeStableNamezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_makeStablePtrzh() -> bool {
    let expected = unsafe { sys::stg_makeStablePtrzh() };
    let actual = unsafe { stg_makeStablePtrzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_makeStablePtrzh() {
    unsafe { stg_makeStablePtrzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_deRefStablePtrzh() -> bool {
    let expected = unsafe { sys::stg_deRefStablePtrzh() };
    let actual = unsafe { stg_deRefStablePtrzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_deRefStablePtrzh() {
    unsafe { stg_deRefStablePtrzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAddzh() -> bool {
    let expected = unsafe { sys::stg_compactAddzh() };
    let actual = unsafe { stg_compactAddzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAddzh() {
    unsafe { stg_compactAddzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAddWithSharingzh() -> bool {
    let expected = unsafe { sys::stg_compactAddWithSharingzh() };
    let actual = unsafe { stg_compactAddWithSharingzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAddWithSharingzh() {
    unsafe { stg_compactAddWithSharingzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactNewzh() -> bool {
    let expected = unsafe { sys::stg_compactNewzh() };
    let actual = unsafe { stg_compactNewzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactNewzh() {
    unsafe { stg_compactNewzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAppendzh() -> bool {
    let expected = unsafe { sys::stg_compactAppendzh() };
    let actual = unsafe { stg_compactAppendzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAppendzh() {
    unsafe { stg_compactAppendzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactResizzezh() -> bool {
    let expected = unsafe { sys::stg_compactResizzezh() };
    let actual = unsafe { stg_compactResizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactResizzezh() {
    unsafe { stg_compactResizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetRootzh() -> bool {
    let expected = unsafe { sys::stg_compactGetRootzh() };
    let actual = unsafe { stg_compactGetRootzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetRootzh() {
    unsafe { stg_compactGetRootzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactContainszh() -> bool {
    let expected = unsafe { sys::stg_compactContainszh() };
    let actual = unsafe { stg_compactContainszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactContainszh() {
    unsafe { stg_compactContainszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactContainsAnyzh() -> bool {
    let expected = unsafe { sys::stg_compactContainsAnyzh() };
    let actual = unsafe { stg_compactContainsAnyzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactContainsAnyzh() {
    unsafe { stg_compactContainsAnyzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetFirstBlockzh() -> bool {
    let expected = unsafe { sys::stg_compactGetFirstBlockzh() };
    let actual = unsafe { stg_compactGetFirstBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetFirstBlockzh() {
    unsafe { stg_compactGetFirstBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactGetNextBlockzh() -> bool {
    let expected = unsafe { sys::stg_compactGetNextBlockzh() };
    let actual = unsafe { stg_compactGetNextBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactGetNextBlockzh() {
    unsafe { stg_compactGetNextBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactAllocateBlockzh() -> bool {
    let expected = unsafe { sys::stg_compactAllocateBlockzh() };
    let actual = unsafe { stg_compactAllocateBlockzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactAllocateBlockzh() {
    unsafe { stg_compactAllocateBlockzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactFixupPointerszh() -> bool {
    let expected = unsafe { sys::stg_compactFixupPointerszh() };
    let actual = unsafe { stg_compactFixupPointerszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactFixupPointerszh() {
    unsafe { stg_compactFixupPointerszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_compactSizzezh() -> bool {
    let expected = unsafe { sys::stg_compactSizzezh() };
    let actual = unsafe { stg_compactSizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_compactSizzezh() {
    unsafe { stg_compactSizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_forkzh() -> bool {
    let expected = unsafe { sys::stg_forkzh() };
    let actual = unsafe { stg_forkzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_forkzh() {
    unsafe { stg_forkzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_forkOnzh() -> bool {
    let expected = unsafe { sys::stg_forkOnzh() };
    let actual = unsafe { stg_forkOnzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_forkOnzh() {
    unsafe { stg_forkOnzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_yieldzh() -> bool {
    let expected = unsafe { sys::stg_yieldzh() };
    let actual = unsafe { stg_yieldzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_yieldzh() {
    unsafe { stg_yieldzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_killMyself() -> bool {
    let expected = unsafe { sys::stg_killMyself() };
    let actual = unsafe { stg_killMyself() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_killMyself() {
    unsafe { stg_killMyself() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_killThreadzh() -> bool {
    let expected = unsafe { sys::stg_killThreadzh() };
    let actual = unsafe { stg_killThreadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_killThreadzh() {
    unsafe { stg_killThreadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getMaskingStatezh() -> bool {
    let expected = unsafe { sys::stg_getMaskingStatezh() };
    let actual = unsafe { stg_getMaskingStatezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getMaskingStatezh() {
    unsafe { stg_getMaskingStatezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_maskAsyncExceptionszh() -> bool {
    let expected = unsafe { sys::stg_maskAsyncExceptionszh() };
    let actual = unsafe { stg_maskAsyncExceptionszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_maskAsyncExceptionszh() {
    unsafe { stg_maskAsyncExceptionszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_maskUninterruptiblezh() -> bool {
    let expected = unsafe { sys::stg_maskUninterruptiblezh() };
    let actual = unsafe { stg_maskUninterruptiblezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_maskUninterruptiblezh() {
    unsafe { stg_maskUninterruptiblezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unmaskAsyncExceptionszh() -> bool {
    let expected = unsafe { sys::stg_unmaskAsyncExceptionszh() };
    let actual = unsafe { stg_unmaskAsyncExceptionszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unmaskAsyncExceptionszh() {
    unsafe { stg_unmaskAsyncExceptionszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_myThreadIdzh() -> bool {
    let expected = unsafe { sys::stg_myThreadIdzh() };
    let actual = unsafe { stg_myThreadIdzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_myThreadIdzh() {
    unsafe { stg_myThreadIdzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_labelThreadzh() -> bool {
    let expected = unsafe { sys::stg_labelThreadzh() };
    let actual = unsafe { stg_labelThreadzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_labelThreadzh() {
    unsafe { stg_labelThreadzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_isCurrentThreadBoundzh() -> bool {
    let expected = unsafe { sys::stg_isCurrentThreadBoundzh() };
    let actual = unsafe { stg_isCurrentThreadBoundzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_isCurrentThreadBoundzh() {
    unsafe { stg_isCurrentThreadBoundzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadLabelzh() -> bool {
    let expected = unsafe { sys::stg_threadLabelzh() };
    let actual = unsafe { stg_threadLabelzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadLabelzh() {
    unsafe { stg_threadLabelzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_threadStatuszh() -> bool {
    let expected = unsafe { sys::stg_threadStatuszh() };
    let actual = unsafe { stg_threadStatuszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_threadStatuszh() {
    unsafe { stg_threadStatuszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_listThreadszh() -> bool {
    let expected = unsafe { sys::stg_listThreadszh() };
    let actual = unsafe { stg_listThreadszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_listThreadszh() {
    unsafe { stg_listThreadszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakzh() -> bool {
    let expected = unsafe { sys::stg_mkWeakzh() };
    let actual = unsafe { stg_mkWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakzh() {
    unsafe { stg_mkWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakNoFinalizzerzh() -> bool {
    let expected = unsafe { sys::stg_mkWeakNoFinalizzerzh() };
    let actual = unsafe { stg_mkWeakNoFinalizzerzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakNoFinalizzerzh() {
    unsafe { stg_mkWeakNoFinalizzerzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkWeakForeignzh() -> bool {
    let expected = unsafe { sys::stg_mkWeakForeignzh() };
    let actual = unsafe { stg_mkWeakForeignzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkWeakForeignzh() {
    unsafe { stg_mkWeakForeignzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_addCFinalizzerToWeakzh() -> bool {
    let expected = unsafe { sys::stg_addCFinalizzerToWeakzh() };
    let actual = unsafe { stg_addCFinalizzerToWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_addCFinalizzerToWeakzh() {
    unsafe { stg_addCFinalizzerToWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_finalizzeWeakzh() -> bool {
    let expected = unsafe { sys::stg_finalizzeWeakzh() };
    let actual = unsafe { stg_finalizzeWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_finalizzeWeakzh() {
    unsafe { stg_finalizzeWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_deRefWeakzh() -> bool {
    let expected = unsafe { sys::stg_deRefWeakzh() };
    let actual = unsafe { stg_deRefWeakzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_deRefWeakzh() {
    unsafe { stg_deRefWeakzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_runRWzh() -> bool {
    let expected = unsafe { sys::stg_runRWzh() };
    let actual = unsafe { stg_runRWzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_runRWzh() {
    unsafe { stg_runRWzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newBCOzh() -> bool {
    let expected = unsafe { sys::stg_newBCOzh() };
    let actual = unsafe { stg_newBCOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newBCOzh() {
    unsafe { stg_newBCOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_mkApUpd0zh() -> bool {
    let expected = unsafe { sys::stg_mkApUpd0zh() };
    let actual = unsafe { stg_mkApUpd0zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_mkApUpd0zh() {
    unsafe { stg_mkApUpd0zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_retryzh() -> bool {
    let expected = unsafe { sys::stg_retryzh() };
    let actual = unsafe { stg_retryzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_retryzh() {
    unsafe { stg_retryzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchRetryzh() -> bool {
    let expected = unsafe { sys::stg_catchRetryzh() };
    let actual = unsafe { stg_catchRetryzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchRetryzh() {
    unsafe { stg_catchRetryzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_catchSTMzh() -> bool {
    let expected = unsafe { sys::stg_catchSTMzh() };
    let actual = unsafe { stg_catchSTMzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_catchSTMzh() {
    unsafe { stg_catchSTMzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_atomicallyzh() -> bool {
    let expected = unsafe { sys::stg_atomicallyzh() };
    let actual = unsafe { stg_atomicallyzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_atomicallyzh() {
    unsafe { stg_atomicallyzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_newTVarzh() -> bool {
    let expected = unsafe { sys::stg_newTVarzh() };
    let actual = unsafe { stg_newTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_newTVarzh() {
    unsafe { stg_newTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readTVarzh() -> bool {
    let expected = unsafe { sys::stg_readTVarzh() };
    let actual = unsafe { stg_readTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readTVarzh() {
    unsafe { stg_readTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_readTVarIOzh() -> bool {
    let expected = unsafe { sys::stg_readTVarIOzh() };
    let actual = unsafe { stg_readTVarIOzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_readTVarIOzh() {
    unsafe { stg_readTVarIOzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_writeTVarzh() -> bool {
    let expected = unsafe { sys::stg_writeTVarzh() };
    let actual = unsafe { stg_writeTVarzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_writeTVarzh() {
    unsafe { stg_writeTVarzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_unpackClosurezh() -> bool {
    let expected = unsafe { sys::stg_unpackClosurezh() };
    let actual = unsafe { stg_unpackClosurezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_unpackClosurezh() {
    unsafe { stg_unpackClosurezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_closureSizzezh() -> bool {
    let expected = unsafe { sys::stg_closureSizzezh() };
    let actual = unsafe { stg_closureSizzezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_closureSizzezh() {
    unsafe { stg_closureSizzezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_whereFromzh() -> bool {
    let expected = unsafe { sys::stg_whereFromzh() };
    let actual = unsafe { stg_whereFromzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_whereFromzh() {
    unsafe { stg_whereFromzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getApStackValzh() -> bool {
    let expected = unsafe { sys::stg_getApStackValzh() };
    let actual = unsafe { stg_getApStackValzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getApStackValzh() {
    unsafe { stg_getApStackValzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getSparkzh() -> bool {
    let expected = unsafe { sys::stg_getSparkzh() };
    let actual = unsafe { stg_getSparkzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getSparkzh() {
    unsafe { stg_getSparkzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_numSparkszh() -> bool {
    let expected = unsafe { sys::stg_numSparkszh() };
    let actual = unsafe { stg_numSparkszh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_numSparkszh() {
    unsafe { stg_numSparkszh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_noDuplicatezh() -> bool {
    let expected = unsafe { sys::stg_noDuplicatezh() };
    let actual = unsafe { stg_noDuplicatezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_noDuplicatezh() {
    unsafe { stg_noDuplicatezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_clearCCSzh() -> bool {
    let expected = unsafe { sys::stg_clearCCSzh() };
    let actual = unsafe { stg_clearCCSzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_clearCCSzh() {
    unsafe { stg_clearCCSzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceEventzh() -> bool {
    let expected = unsafe { sys::stg_traceEventzh() };
    let actual = unsafe { stg_traceEventzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceEventzh() {
    unsafe { stg_traceEventzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceBinaryEventzh() -> bool {
    let expected = unsafe { sys::stg_traceBinaryEventzh() };
    let actual = unsafe { stg_traceBinaryEventzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceBinaryEventzh() {
    unsafe { stg_traceBinaryEventzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_traceMarkerzh() -> bool {
    let expected = unsafe { sys::stg_traceMarkerzh() };
    let actual = unsafe { stg_traceMarkerzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_traceMarkerzh() {
    unsafe { stg_traceMarkerzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_getThreadAllocationCounterzh() -> bool {
    let expected = unsafe { sys::stg_getThreadAllocationCounterzh() };
    let actual = unsafe { stg_getThreadAllocationCounterzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_getThreadAllocationCounterzh() {
    unsafe { stg_getThreadAllocationCounterzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_setThreadAllocationCounterzh() -> bool {
    let expected = unsafe { sys::stg_setThreadAllocationCounterzh() };
    let actual = unsafe { stg_setThreadAllocationCounterzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_setThreadAllocationCounterzh() {
    unsafe { stg_setThreadAllocationCounterzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castWord64ToDoublezh() -> bool {
    let expected = unsafe { sys::stg_castWord64ToDoublezh() };
    let actual = unsafe { stg_castWord64ToDoublezh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castWord64ToDoublezh() {
    unsafe { stg_castWord64ToDoublezh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castDoubleToWord64zh() -> bool {
    let expected = unsafe { sys::stg_castDoubleToWord64zh() };
    let actual = unsafe { stg_castDoubleToWord64zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castDoubleToWord64zh() {
    unsafe { stg_castDoubleToWord64zh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castWord32ToFloatzh() -> bool {
    let expected = unsafe { sys::stg_castWord32ToFloatzh() };
    let actual = unsafe { stg_castWord32ToFloatzh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castWord32ToFloatzh() {
    unsafe { stg_castWord32ToFloatzh() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_stg_castFloatToWord32zh() -> bool {
    let expected = unsafe { sys::stg_castFloatToWord32zh() };
    let actual = unsafe { stg_castFloatToWord32zh() };
    actual == expected
}

#[test]
#[ignore]
fn test_stg_castFloatToWord32zh() {
    unsafe { stg_castFloatToWord32zh() };
    todo!("assert")
}
