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
fn test_setIOManagerControlFd() {
    let cap_no = Default::default();
    let fd = Default::default();
    unsafe { setIOManagerControlFd(cap_no, fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setTimerManagerControlFd() {
    let fd = Default::default();
    unsafe { setTimerManagerControlFd(fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setIOManagerWakeupFd() {
    let fd = Default::default();
    unsafe { setIOManagerWakeupFd(fd) };
    todo!("assert")
}
