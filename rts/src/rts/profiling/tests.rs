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
fn test_registerCcList() {
    let mut cc_list = null_mut();
    unsafe { registerCcList(&mut &mut cc_list) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_registerCcsList() {
    let mut cc_list = null_mut();
    unsafe { registerCcsList(&mut &mut cc_list) };
    todo!("assert")
}
