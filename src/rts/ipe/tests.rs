use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_InfoProv_() {
    assert_eq!(size_of::<sys::InfoProv_>(), size_of::<super::InfoProv_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InfoProv_"][::core::mem::size_of::<InfoProv_>() - 64usize];
    ["Alignment of InfoProv_"][::core::mem::align_of::<InfoProv_>() - 8usize];
    ["Offset of field: InfoProv_::table_name"]
        [::core::mem::offset_of!(InfoProv_, table_name) - 0usize];
    ["Offset of field: InfoProv_::closure_desc"]
        [::core::mem::offset_of!(InfoProv_, closure_desc) - 8usize];
    ["Offset of field: InfoProv_::ty_desc"][::core::mem::offset_of!(InfoProv_, ty_desc) - 16usize];
    ["Offset of field: InfoProv_::label"][::core::mem::offset_of!(InfoProv_, label) - 24usize];
    ["Offset of field: InfoProv_::unit_id"][::core::mem::offset_of!(InfoProv_, unit_id) - 32usize];
    ["Offset of field: InfoProv_::module"][::core::mem::offset_of!(InfoProv_, module) - 40usize];
    ["Offset of field: InfoProv_::src_file"]
        [::core::mem::offset_of!(InfoProv_, src_file) - 48usize];
    ["Offset of field: InfoProv_::src_span"]
        [::core::mem::offset_of!(InfoProv_, src_span) - 56usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_InfoProvEnt_() {
    assert_eq!(
        size_of::<sys::InfoProvEnt_>(),
        size_of::<super::InfoProvEnt_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InfoProvEnt_"][::core::mem::size_of::<InfoProvEnt_>() - 72usize];
    ["Alignment of InfoProvEnt_"][::core::mem::align_of::<InfoProvEnt_>() - 8usize];
    ["Offset of field: InfoProvEnt_::info"][::core::mem::offset_of!(InfoProvEnt_, info) - 0usize];
    ["Offset of field: InfoProvEnt_::prov"][::core::mem::offset_of!(InfoProvEnt_, prov) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_IpeBufferEntry() {
    assert_eq!(
        size_of::<sys::IpeBufferEntry>(),
        size_of::<super::IpeBufferEntry>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IpeBufferEntry"][::core::mem::size_of::<IpeBufferEntry>() - 24usize];
    ["Alignment of IpeBufferEntry"][::core::mem::align_of::<IpeBufferEntry>() - 4usize];
    ["Offset of field: IpeBufferEntry::table_name"]
        [::core::mem::offset_of!(IpeBufferEntry, table_name) - 0usize];
    ["Offset of field: IpeBufferEntry::closure_desc"]
        [::core::mem::offset_of!(IpeBufferEntry, closure_desc) - 4usize];
    ["Offset of field: IpeBufferEntry::ty_desc"]
        [::core::mem::offset_of!(IpeBufferEntry, ty_desc) - 8usize];
    ["Offset of field: IpeBufferEntry::label"]
        [::core::mem::offset_of!(IpeBufferEntry, label) - 12usize];
    ["Offset of field: IpeBufferEntry::src_file"]
        [::core::mem::offset_of!(IpeBufferEntry, src_file) - 16usize];
    ["Offset of field: IpeBufferEntry::src_span"]
        [::core::mem::offset_of!(IpeBufferEntry, src_span) - 20usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_IpeBufferListNode_() {
    assert_eq!(
        size_of::<sys::IpeBufferListNode_>(),
        size_of::<super::IpeBufferListNode_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IpeBufferListNode_"][::core::mem::size_of::<IpeBufferListNode_>() - 72usize];
    ["Alignment of IpeBufferListNode_"][::core::mem::align_of::<IpeBufferListNode_>() - 8usize];
    ["Offset of field: IpeBufferListNode_::next"]
        [::core::mem::offset_of!(IpeBufferListNode_, next) - 0usize];
    ["Offset of field: IpeBufferListNode_::compressed"]
        [::core::mem::offset_of!(IpeBufferListNode_, compressed) - 8usize];
    ["Offset of field: IpeBufferListNode_::count"]
        [::core::mem::offset_of!(IpeBufferListNode_, count) - 16usize];
    ["Offset of field: IpeBufferListNode_::tables"]
        [::core::mem::offset_of!(IpeBufferListNode_, tables) - 24usize];
    ["Offset of field: IpeBufferListNode_::entries"]
        [::core::mem::offset_of!(IpeBufferListNode_, entries) - 32usize];
    ["Offset of field: IpeBufferListNode_::entries_size"]
        [::core::mem::offset_of!(IpeBufferListNode_, entries_size) - 40usize];
    ["Offset of field: IpeBufferListNode_::string_table"]
        [::core::mem::offset_of!(IpeBufferListNode_, string_table) - 48usize];
    ["Offset of field: IpeBufferListNode_::string_table_size"]
        [::core::mem::offset_of!(IpeBufferListNode_, string_table_size) - 56usize];
    ["Offset of field: IpeBufferListNode_::unit_id"]
        [::core::mem::offset_of!(IpeBufferListNode_, unit_id) - 64usize];
    ["Offset of field: IpeBufferListNode_::module_name"]
        [::core::mem::offset_of!(IpeBufferListNode_, module_name) - 68usize];
};

#[test]
#[ignore]
fn test_registerInfoProvList() {
    let mut node = Default::default();
    unsafe { super::registerInfoProvList(&mut node) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_formatClosureDescIpe() {
    let ipe_buf = Default::default();
    let mut str_buf = Default::default();
    unsafe { super::formatClosureDescIpe(&ipe_buf, &mut str_buf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupIPE(info: StgInfoTable, out: InfoProvEnt) -> bool {
    let expected = unsafe { transmute(sys::lookupIPE(&info.into(), &mut out.into())) };
    let actual = unsafe { super::lookupIPE(&info, &mut out) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupIPE() {
    let info = Default::default();
    let mut out = Default::default();
    unsafe { super::lookupIPE(&info, &mut out) };
    todo!("assert")
}
