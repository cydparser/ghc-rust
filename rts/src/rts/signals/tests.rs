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
fn sys_eq_STG_SIG_DFL() {
    assert_eq!(sys::STG_SIG_DFL, STG_SIG_DFL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_SIG_IGN() {
    assert_eq!(sys::STG_SIG_IGN, STG_SIG_IGN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_SIG_ERR() {
    assert_eq!(sys::STG_SIG_ERR, STG_SIG_ERR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_SIG_HAN() {
    assert_eq!(sys::STG_SIG_HAN, STG_SIG_HAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_SIG_RST() {
    assert_eq!(sys::STG_SIG_RST, STG_SIG_RST);
}
