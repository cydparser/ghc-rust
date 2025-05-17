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
fn sys_size_InfoProv_() {
    assert_eq!(size_of::<sys::InfoProv_>(), size_of::<InfoProv_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InfoProv_"][size_of::<InfoProv_>() - 64usize];
    ["Alignment of InfoProv_"][align_of::<InfoProv_>() - 8usize];
    ["Offset of field: InfoProv_::table_name"][offset_of!(InfoProv_, table_name) - 0usize];
    ["Offset of field: InfoProv_::closure_desc"][offset_of!(InfoProv_, closure_desc) - 8usize];
    ["Offset of field: InfoProv_::ty_desc"][offset_of!(InfoProv_, ty_desc) - 16usize];
    ["Offset of field: InfoProv_::label"][offset_of!(InfoProv_, label) - 24usize];
    ["Offset of field: InfoProv_::unit_id"][offset_of!(InfoProv_, unit_id) - 32usize];
    ["Offset of field: InfoProv_::module"][offset_of!(InfoProv_, module) - 40usize];
    ["Offset of field: InfoProv_::src_file"][offset_of!(InfoProv_, src_file) - 48usize];
    ["Offset of field: InfoProv_::src_span"][offset_of!(InfoProv_, src_span) - 56usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_InfoProvEnt_() {
    assert_eq!(size_of::<sys::InfoProvEnt_>(), size_of::<InfoProvEnt_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InfoProvEnt_"][size_of::<InfoProvEnt_>() - 72usize];
    ["Alignment of InfoProvEnt_"][align_of::<InfoProvEnt_>() - 8usize];
    ["Offset of field: InfoProvEnt_::info"][offset_of!(InfoProvEnt_, info) - 0usize];
    ["Offset of field: InfoProvEnt_::prov"][offset_of!(InfoProvEnt_, prov) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_IpeBufferEntry() {
    assert_eq!(
        size_of::<sys::IpeBufferEntry>(),
        size_of::<IpeBufferEntry>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IpeBufferEntry"][size_of::<IpeBufferEntry>() - 24usize];
    ["Alignment of IpeBufferEntry"][align_of::<IpeBufferEntry>() - 4usize];
    ["Offset of field: IpeBufferEntry::table_name"]
        [offset_of!(IpeBufferEntry, table_name) - 0usize];
    ["Offset of field: IpeBufferEntry::closure_desc"]
        [offset_of!(IpeBufferEntry, closure_desc) - 4usize];
    ["Offset of field: IpeBufferEntry::ty_desc"][offset_of!(IpeBufferEntry, ty_desc) - 8usize];
    ["Offset of field: IpeBufferEntry::label"][offset_of!(IpeBufferEntry, label) - 12usize];
    ["Offset of field: IpeBufferEntry::src_file"][offset_of!(IpeBufferEntry, src_file) - 16usize];
    ["Offset of field: IpeBufferEntry::src_span"][offset_of!(IpeBufferEntry, src_span) - 20usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_IpeBufferListNode_() {
    assert_eq!(
        size_of::<sys::IpeBufferListNode_>(),
        size_of::<IpeBufferListNode_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IpeBufferListNode_"][size_of::<IpeBufferListNode_>() - 72usize];
    ["Alignment of IpeBufferListNode_"][align_of::<IpeBufferListNode_>() - 8usize];
    ["Offset of field: IpeBufferListNode_::next"][offset_of!(IpeBufferListNode_, next) - 0usize];
    ["Offset of field: IpeBufferListNode_::compressed"]
        [offset_of!(IpeBufferListNode_, compressed) - 8usize];
    ["Offset of field: IpeBufferListNode_::count"][offset_of!(IpeBufferListNode_, count) - 16usize];
    ["Offset of field: IpeBufferListNode_::tables"]
        [offset_of!(IpeBufferListNode_, tables) - 24usize];
    ["Offset of field: IpeBufferListNode_::entries"]
        [offset_of!(IpeBufferListNode_, entries) - 32usize];
    ["Offset of field: IpeBufferListNode_::entries_size"]
        [offset_of!(IpeBufferListNode_, entries_size) - 40usize];
    ["Offset of field: IpeBufferListNode_::string_table"]
        [offset_of!(IpeBufferListNode_, string_table) - 48usize];
    ["Offset of field: IpeBufferListNode_::string_table_size"]
        [offset_of!(IpeBufferListNode_, string_table_size) - 56usize];
    ["Offset of field: IpeBufferListNode_::unit_id"]
        [offset_of!(IpeBufferListNode_, unit_id) - 64usize];
    ["Offset of field: IpeBufferListNode_::module_name"]
        [offset_of!(IpeBufferListNode_, module_name) - 68usize];
};

#[test]
#[ignore]
fn test_registerInfoProvList() {
    let mut node = null_mut();
    unsafe { registerInfoProvList(&mut node) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_formatClosureDescIpe() {
    let ipe_buf = null();
    let mut str_buf = null_mut();
    unsafe { formatClosureDescIpe(&ipe_buf, &mut str_buf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupIPE(info: StgInfoTable, out: InfoProvEnt) -> bool {
    let expected = unsafe { transmute(sys::lookupIPE(&info.into(), &mut out.into())) };
    let actual = unsafe { lookupIPE(&info, &mut out) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupIPE() {
    let info = null();
    let mut out = null_mut();
    unsafe { lookupIPE(&info, &mut out) };
    todo!("assert")
}
