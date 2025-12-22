use super::*;

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
    actual == expected
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
