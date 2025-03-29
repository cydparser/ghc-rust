use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_CHAR_MIN() {
    assert_eq!(sys::HS_CHAR_MIN, super::HS_CHAR_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_CHAR_MAX() {
    assert_eq!(sys::HS_CHAR_MAX, super::HS_CHAR_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_FALSE() {
    assert_eq!(sys::HS_BOOL_FALSE, super::HS_BOOL_FALSE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_TRUE() {
    assert_eq!(sys::HS_BOOL_TRUE, super::HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_MIN() {
    assert_eq!(sys::HS_BOOL_MIN, super::HS_BOOL_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_MAX() {
    assert_eq!(sys::HS_BOOL_MAX, super::HS_BOOL_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT8_MIN() {
    assert_eq!(sys::HS_INT8_MIN, super::HS_INT8_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT8_MAX() {
    assert_eq!(sys::HS_INT8_MAX, super::HS_INT8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT16_MIN() {
    assert_eq!(sys::HS_INT16_MIN, super::HS_INT16_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT16_MAX() {
    assert_eq!(sys::HS_INT16_MAX, super::HS_INT16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT32_MIN() {
    assert_eq!(sys::HS_INT32_MIN, super::HS_INT32_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT32_MAX() {
    assert_eq!(sys::HS_INT32_MAX, super::HS_INT32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD8_MAX() {
    assert_eq!(sys::HS_WORD8_MAX, super::HS_WORD8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD16_MAX() {
    assert_eq!(sys::HS_WORD16_MAX, super::HS_WORD16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD32_MAX() {
    assert_eq!(sys::HS_WORD32_MAX, super::HS_WORD32_MAX);
}

#[test]
#[ignore]
fn test_hs_init() {
    let mut argc = Default::default();
    let mut argv = Default::default();
    unsafe { super::hs_init(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_exit() {
    unsafe { super::hs_exit() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_exit_nowait() {
    unsafe { super::hs_exit_nowait() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_set_argv() {
    let argc = Default::default();
    let mut argv = Default::default();
    unsafe { super::hs_set_argv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_thread_done() {
    unsafe { super::hs_thread_done() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_restoreConsoleCP() {
    unsafe { super::hs_restoreConsoleCP() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_perform_gc() {
    unsafe { super::hs_perform_gc() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_lock_stable_ptr_table() {
    unsafe { super::hs_lock_stable_ptr_table() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_lock_stable_tables() {
    unsafe { super::hs_lock_stable_tables() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_unlock_stable_ptr_table() {
    unsafe { super::hs_unlock_stable_ptr_table() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_unlock_stable_tables() {
    unsafe { super::hs_unlock_stable_tables() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_stable_ptr_unsafe() {
    let sp = Default::default();
    unsafe { super::hs_free_stable_ptr_unsafe(sp) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_stable_ptr() {
    let sp = Default::default();
    unsafe { super::hs_free_stable_ptr(sp) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_fun_ptr() {
    let fp = Default::default();
    unsafe { super::hs_free_fun_ptr(fp) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_spt_lookup(key: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_spt_lookup(&mut key.into())) };
    let actual = unsafe { super::hs_spt_lookup(&mut key) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_spt_lookup() {
    let mut key = Default::default();
    unsafe { super::hs_spt_lookup(&mut key) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_spt_keys(keys: StgPtr, szKeys: ::core::ffi::c_int) -> bool {
    let expected = unsafe { transmute(sys::hs_spt_keys(&mut keys.into(), szKeys.into())) };
    let actual = unsafe { super::hs_spt_keys(&mut keys, szKeys) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_spt_keys() {
    let mut keys = Default::default();
    let szKeys = Default::default();
    unsafe { super::hs_spt_keys(&mut keys, szKeys) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_spt_key_count() -> bool {
    let expected = unsafe { transmute(sys::hs_spt_key_count()) };
    let actual = unsafe { super::hs_spt_key_count() };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_spt_key_count() {
    unsafe { super::hs_spt_key_count() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_try_putmvar() {
    let capability = Default::default();
    let sp = Default::default();
    unsafe { super::hs_try_putmvar(capability, sp) };
    todo!("assert")
}
