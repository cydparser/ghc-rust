use super::*;

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

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_registerInfoProvList() {
    let expected = {
        let mut node: sys::IpeBufferListNode = todo!();
        unsafe { sys::registerInfoProvList(&raw mut node) };
        todo!()
    };
    let actual = {
        let mut node: IpeBufferListNode = todo!();
        unsafe { registerInfoProvList(&raw mut node) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_registerInfoProvList() {
    let actual = {
        let node: IpeBufferListNode = todo!();
        unsafe { registerInfoProvList(&raw mut node) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_formatClosureDescIpe(str_buf: c_char) -> bool {
    let expected = {
        let mut ipe_buf: sys::InfoProvEnt = todo!();
        let mut str_buf = str_buf;
        unsafe { sys::formatClosureDescIpe(&raw mut ipe_buf, &raw mut str_buf) };
        todo!()
    };
    let actual = {
        let mut ipe_buf: InfoProvEnt = todo!();
        let mut str_buf = str_buf;
        unsafe { formatClosureDescIpe(&raw mut ipe_buf, &raw mut str_buf) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_formatClosureDescIpe() {
    let g = &mut Gen::new(100);
    let actual = {
        let ipe_buf: InfoProvEnt = todo!();
        let mut str_buf: c_char = Arbitrary::arbitrary(g);
        unsafe { formatClosureDescIpe(&raw mut ipe_buf, &raw mut str_buf) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_lookupIPE() {
    let expected: bool = {
        let mut info: sys::StgInfoTable = todo!();
        let mut out: sys::InfoProvEnt = todo!();
        unsafe { sys::lookupIPE(&raw mut info, &raw mut out) }
    };
    let actual: bool = {
        let mut info: StgInfoTable = todo!();
        let mut out: InfoProvEnt = todo!();
        unsafe { lookupIPE(&raw mut info, &raw mut out) }
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_lookupIPE() {
    let actual: bool = {
        let info: StgInfoTable = todo!();
        let out: InfoProvEnt = todo!();
        unsafe { lookupIPE(&raw mut info, &raw mut out) }
    };
    let expected: bool = todo!();
    assert_eq!(expected, actual);
}
