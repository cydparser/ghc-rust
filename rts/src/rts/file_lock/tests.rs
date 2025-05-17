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
fn equivalent_lockFile(id: StgWord64, dev: StgWord64, ino: StgWord64, for_writing: c_int) -> bool {
    let expected = unsafe { sys::lockFile(id, dev, ino, for_writing) };
    let actual = unsafe { lockFile(id, dev, ino, for_writing) };
    actual == expected
}

#[test]
#[ignore]
fn test_lockFile() {
    let id = Default::default();
    let dev = Default::default();
    let ino = Default::default();
    let for_writing = Default::default();
    unsafe { lockFile(id, dev, ino, for_writing) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unlockFile(id: StgWord64) -> bool {
    let expected = unsafe { sys::unlockFile(id) };
    let actual = unsafe { unlockFile(id) };
    actual == expected
}

#[test]
#[ignore]
fn test_unlockFile() {
    let id = Default::default();
    unsafe { unlockFile(id) };
    todo!("assert")
}
