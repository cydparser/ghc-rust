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
fn equivalent_newSpark(reg: StgRegTable, p: StgClosure) -> bool {
    let expected = unsafe { sys::newSpark(&mut reg.into(), &mut p.into()) };
    let actual = unsafe { newSpark(&mut reg, &mut p) };
    actual == expected
}

#[test]
#[ignore]
fn test_newSpark() {
    let mut reg = null_mut();
    let mut p = null_mut();
    unsafe { newSpark(&mut reg, &mut p) };
    todo!("assert")
}
