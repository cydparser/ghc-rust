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
#[quickcheck]
fn equivalent___hscore_get_saved_termios(fd: c_int) -> bool {
    let expected = unsafe { sys::__hscore_get_saved_termios(fd) };
    let actual = unsafe { __hscore_get_saved_termios(fd) };
    actual == expected
}

#[test]
#[ignore]
fn test___hscore_get_saved_termios() {
    let fd = Default::default();
    unsafe { __hscore_get_saved_termios(fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test___hscore_set_saved_termios() {
    let fd = Default::default();
    let mut ts = null_mut();
    unsafe { __hscore_set_saved_termios(fd, &mut ts) };
    todo!("assert")
}
