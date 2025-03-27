use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_RTS_USER_SIGNALS() {
    assert_eq!(sys::RTS_USER_SIGNALS, super::RTS_USER_SIGNALS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MAX_N_CAPABILITIES() {
    assert_eq!(sys::MAX_N_CAPABILITIES, super::MAX_N_CAPABILITIES.into());
}
