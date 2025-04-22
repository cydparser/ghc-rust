use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_ExecPage() {
    assert_eq!(size_of::<sys::ExecPage>(), size_of::<super::ExecPage>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ExecPage"][::core::mem::size_of::<ExecPage>() - 1usize];
    ["Alignment of ExecPage"][::core::mem::align_of::<ExecPage>() - 1usize];
    ["Offset of field: ExecPage::contents"][::core::mem::offset_of!(ExecPage, contents) - 0usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocateExecPage() -> bool {
    let expected = unsafe { transmute(sys::allocateExecPage()) };
    let actual = unsafe { super::allocateExecPage() };
    actual == expected
}

#[test]
#[ignore]
fn test_allocateExecPage() {
    unsafe { super::allocateExecPage() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freezeExecPage() {
    let mut page = Default::default();
    unsafe { super::freezeExecPage(&mut page) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeExecPage() {
    let mut page = Default::default();
    unsafe { super::freeExecPage(&mut page) };
    todo!("assert")
}
