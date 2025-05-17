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
fn sys_size__ObjectCode() {
    assert_eq!(size_of::<sys::_ObjectCode>(), size_of::<_ObjectCode>())
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_ForeignExportsList() {
    assert_eq!(
        size_of::<sys::ForeignExportsList>(),
        size_of::<ForeignExportsList>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ForeignExportsList"][size_of::<ForeignExportsList>() - 32usize];
    ["Alignment of ForeignExportsList"][align_of::<ForeignExportsList>() - 8usize];
    ["Offset of field: ForeignExportsList::next"][offset_of!(ForeignExportsList, next) - 0usize];
    ["Offset of field: ForeignExportsList::n_entries"]
        [offset_of!(ForeignExportsList, n_entries) - 8usize];
    ["Offset of field: ForeignExportsList::oc"][offset_of!(ForeignExportsList, oc) - 16usize];
    ["Offset of field: ForeignExportsList::stable_ptrs"]
        [offset_of!(ForeignExportsList, stable_ptrs) - 24usize];
    ["Offset of field: ForeignExportsList::exports"]
        [offset_of!(ForeignExportsList, exports) - 32usize];
};

#[test]
#[ignore]
fn test_registerForeignExports() {
    let mut exports = null_mut();
    unsafe { registerForeignExports(&mut exports) };
    todo!("assert")
}
