use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[test]
#[ignore]
fn test_hs_spt_insert() {
    let mut key = null_mut();
    let mut spe_closure = null_mut();
    unsafe { hs_spt_insert(&mut key, &mut spe_closure) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_insert_stableptr() {
    let mut key = null_mut();
    let mut entry = null_mut();
    unsafe { hs_spt_insert_stableptr(&mut key, &mut entry) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_remove() {
    let mut key = null_mut();
    unsafe { hs_spt_remove(&mut key) };
    todo!("assert")
}
