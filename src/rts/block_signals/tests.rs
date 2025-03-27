use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_blockUserSignals() {
    unsafe { super::blockUserSignals() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_unblockUserSignals() {
    unsafe { super::unblockUserSignals() };
    todo!("assert")
}
