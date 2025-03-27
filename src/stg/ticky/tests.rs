use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_TICKY_BIN_COUNT() {
    assert_eq!(sys::TICKY_BIN_COUNT, super::TICKY_BIN_COUNT.into());
}
