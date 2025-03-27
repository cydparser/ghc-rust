use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_BITS_PER_BYTE() {
    assert_eq!(sys::BITS_PER_BYTE, super::BITS_PER_BYTE.into());
}
