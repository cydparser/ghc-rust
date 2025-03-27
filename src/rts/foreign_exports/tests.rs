use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of__ObjectCode() {
    assert_eq!(
        size_of::<sys::_ObjectCode>(),
        size_of::<super::_ObjectCode>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_ForeignExportsList() {
    assert_eq!(
        size_of::<sys::ForeignExportsList>(),
        size_of::<super::ForeignExportsList>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ForeignExportsList"][::core::mem::size_of::<ForeignExportsList>() - 32usize];
    ["Alignment of ForeignExportsList"][::core::mem::align_of::<ForeignExportsList>() - 8usize];
    ["Offset of field: ForeignExportsList::next"]
        [::core::mem::offset_of!(ForeignExportsList, next) - 0usize];
    ["Offset of field: ForeignExportsList::n_entries"]
        [::core::mem::offset_of!(ForeignExportsList, n_entries) - 8usize];
    ["Offset of field: ForeignExportsList::oc"]
        [::core::mem::offset_of!(ForeignExportsList, oc) - 16usize];
    ["Offset of field: ForeignExportsList::stable_ptrs"]
        [::core::mem::offset_of!(ForeignExportsList, stable_ptrs) - 24usize];
    ["Offset of field: ForeignExportsList::exports"]
        [::core::mem::offset_of!(ForeignExportsList, exports) - 32usize];
};

#[test]
#[ignore]
fn test_registerForeignExports() {
    let exports = Default::default();
    unsafe { super::registerForeignExports(&mut exports) };
    todo!("assert")
}
