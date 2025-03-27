use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[test]
#[ignore]
fn test_setIOManagerControlFd() {
    let cap_no = Default::default();
    let fd = Default::default();
    unsafe { super::setIOManagerControlFd(cap_no, fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setTimerManagerControlFd() {
    let fd = Default::default();
    unsafe { super::setTimerManagerControlFd(fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setIOManagerWakeupFd() {
    let fd = Default::default();
    unsafe { super::setIOManagerWakeupFd(fd) };
    todo!("assert")
}
