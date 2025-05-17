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
fn equivalent_getStablePtr(p: StgPtr) -> bool {
    let expected = unsafe { sys::getStablePtr(p) };
    let actual = unsafe { getStablePtr(p) };
    actual == expected
}

#[test]
#[ignore]
fn test_getStablePtr() {
    let p = Default::default();
    unsafe { getStablePtr(p) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_spEntry() {
    assert_eq!(size_of::<sys::spEntry>(), size_of::<spEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of spEntry"][size_of::<spEntry>() - 8usize];
    ["Alignment of spEntry"][align_of::<spEntry>() - 8usize];
    ["Offset of field: spEntry::addr"][offset_of!(spEntry, addr) - 0usize];
};
