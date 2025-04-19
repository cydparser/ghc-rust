use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_hs_spt_insert() {
    let mut key = Default::default();
    let mut spe_closure = Default::default();
    unsafe { super::hs_spt_insert(&mut key, &mut spe_closure) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_insert_stableptr() {
    let mut key = Default::default();
    let mut entry = Default::default();
    unsafe { super::hs_spt_insert_stableptr(&mut key, &mut entry) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_remove() {
    let mut key = Default::default();
    unsafe { super::hs_spt_remove(&mut key) };
    todo!("assert")
}
