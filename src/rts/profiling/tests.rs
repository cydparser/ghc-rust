use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_registerCcList() {
    let mut cc_list = Default::default();
    unsafe { super::registerCcList(&mut &mut cc_list) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_registerCcsList() {
    let mut cc_list = Default::default();
    unsafe { super::registerCcsList(&mut &mut cc_list) };
    todo!("assert")
}
