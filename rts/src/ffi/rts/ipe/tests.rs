use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_InfoProv__layout() {
    assert_eq!(
        offset_of!(InfoProv_, table_name),
        offset_of!(sys::InfoProv_, table_name)
    );
    assert_eq!(
        offset_of!(InfoProv_, closure_desc),
        offset_of!(sys::InfoProv_, closure_desc)
    );
    assert_eq!(
        offset_of!(InfoProv_, ty_desc),
        offset_of!(sys::InfoProv_, ty_desc)
    );
    assert_eq!(
        offset_of!(InfoProv_, label),
        offset_of!(sys::InfoProv_, label)
    );
    assert_eq!(
        offset_of!(InfoProv_, unit_id),
        offset_of!(sys::InfoProv_, unit_id)
    );
    assert_eq!(
        offset_of!(InfoProv_, module),
        offset_of!(sys::InfoProv_, module)
    );
    assert_eq!(
        offset_of!(InfoProv_, src_file),
        offset_of!(sys::InfoProv_, src_file)
    );
    assert_eq!(
        offset_of!(InfoProv_, src_span),
        offset_of!(sys::InfoProv_, src_span)
    );
    assert_eq!(size_of::<InfoProv_>(), size_of::<sys::InfoProv_>());
    assert_eq!(align_of::<InfoProv_>(), align_of::<sys::InfoProv_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_InfoProv_layout() {
    assert_eq!(size_of::<InfoProv>(), size_of::<sys::InfoProv>());
    assert_eq!(align_of::<InfoProv>(), align_of::<sys::InfoProv>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_InfoProvEnt_layout() {
    assert_eq!(size_of::<InfoProvEnt>(), size_of::<sys::InfoProvEnt>());
    assert_eq!(align_of::<InfoProvEnt>(), align_of::<sys::InfoProvEnt>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StringIdx_layout() {
    assert_eq!(size_of::<StringIdx>(), size_of::<StringIdx>());
    assert_eq!(align_of::<StringIdx>(), align_of::<StringIdx>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_IpeBufferEntry_layout() {
    assert_eq!(
        offset_of!(IpeBufferEntry, table_name),
        offset_of!(sys::IpeBufferEntry, table_name)
    );
    assert_eq!(
        offset_of!(IpeBufferEntry, closure_desc),
        offset_of!(sys::IpeBufferEntry, closure_desc)
    );
    assert_eq!(
        offset_of!(IpeBufferEntry, ty_desc),
        offset_of!(sys::IpeBufferEntry, ty_desc)
    );
    assert_eq!(
        offset_of!(IpeBufferEntry, label),
        offset_of!(sys::IpeBufferEntry, label)
    );
    assert_eq!(
        offset_of!(IpeBufferEntry, src_file),
        offset_of!(sys::IpeBufferEntry, src_file)
    );
    assert_eq!(
        offset_of!(IpeBufferEntry, src_span),
        offset_of!(sys::IpeBufferEntry, src_span)
    );
    assert_eq!(
        size_of::<IpeBufferEntry>(),
        size_of::<sys::IpeBufferEntry>()
    );
    assert_eq!(
        align_of::<IpeBufferEntry>(),
        align_of::<sys::IpeBufferEntry>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_IpeBufferListNode__layout() {
    assert_eq!(
        size_of::<*mut IpeBufferListNode_>(),
        size_of::<*mut sys::IpeBufferListNode_>()
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, next),
        offset_of!(sys::IpeBufferListNode_, next)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, compressed),
        offset_of!(sys::IpeBufferListNode_, compressed)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, count),
        offset_of!(sys::IpeBufferListNode_, count)
    );
    assert_eq!(
        size_of::<*mut *const StgInfoTable>(),
        size_of::<*mut *const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, tables),
        offset_of!(sys::IpeBufferListNode_, tables)
    );
    assert_eq!(
        size_of::<*mut IpeBufferEntry>(),
        size_of::<*mut sys::IpeBufferEntry>()
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, entries),
        offset_of!(sys::IpeBufferListNode_, entries)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, entries_size),
        offset_of!(sys::IpeBufferListNode_, entries_size)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, string_table),
        offset_of!(sys::IpeBufferListNode_, string_table)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, string_table_size),
        offset_of!(sys::IpeBufferListNode_, string_table_size)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, unit_id),
        offset_of!(sys::IpeBufferListNode_, unit_id)
    );
    assert_eq!(
        offset_of!(IpeBufferListNode_, module_name),
        offset_of!(sys::IpeBufferListNode_, module_name)
    );
    assert_eq!(
        size_of::<IpeBufferListNode_>(),
        size_of::<sys::IpeBufferListNode_>()
    );
    assert_eq!(
        align_of::<IpeBufferListNode_>(),
        align_of::<sys::IpeBufferListNode_>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_IpeBufferListNode_layout() {
    assert_eq!(
        size_of::<IpeBufferListNode>(),
        size_of::<sys::IpeBufferListNode>()
    );
    assert_eq!(
        align_of::<IpeBufferListNode>(),
        align_of::<sys::IpeBufferListNode>()
    );
}

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

    assert_eq!(actual, expected);
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

    assert_eq!(actual, expected);
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
