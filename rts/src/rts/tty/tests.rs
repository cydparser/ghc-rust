use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent___hscore_get_saved_termios(fd: ::core::ffi::c_int) -> bool {
    let expected = unsafe { transmute(sys::__hscore_get_saved_termios(fd.into())) };
    let actual = unsafe { super::__hscore_get_saved_termios(fd) };
    actual == expected
}

#[test]
#[ignore]
fn test___hscore_get_saved_termios() {
    let fd = Default::default();
    unsafe { super::__hscore_get_saved_termios(fd) };
    todo!("assert")
}

#[test]
#[ignore]
fn test___hscore_set_saved_termios() {
    let fd = Default::default();
    let mut ts = Default::default();
    unsafe { super::__hscore_set_saved_termios(fd, &mut ts) };
    todo!("assert")
}
