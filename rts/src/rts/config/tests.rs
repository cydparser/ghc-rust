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
#[test]
fn sys_eq_RTS_USER_SIGNALS() {
    assert_eq!(sys::RTS_USER_SIGNALS, RTS_USER_SIGNALS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_N_CAPABILITIES() {
    assert_eq!(sys::MAX_N_CAPABILITIES, MAX_N_CAPABILITIES);
}
