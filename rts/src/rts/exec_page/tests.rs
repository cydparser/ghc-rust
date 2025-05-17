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
fn sys_size_ExecPage() {
    assert_eq!(size_of::<sys::ExecPage>(), size_of::<ExecPage>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ExecPage"][size_of::<ExecPage>() - 1usize];
    ["Alignment of ExecPage"][align_of::<ExecPage>() - 1usize];
    ["Offset of field: ExecPage::contents"][offset_of!(ExecPage, contents) - 0usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocateExecPage() -> bool {
    let expected = unsafe { transmute(sys::allocateExecPage()) };
    let actual = unsafe { allocateExecPage() };
    actual == expected
}

#[test]
#[ignore]
fn test_allocateExecPage() {
    unsafe { allocateExecPage() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freezeExecPage() {
    let mut page = null_mut();
    unsafe { freezeExecPage(&mut page) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeExecPage() {
    let mut page = null_mut();
    unsafe { freeExecPage(&mut page) };
    todo!("assert")
}
