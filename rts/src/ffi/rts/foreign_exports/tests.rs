use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_ForeignExportsList_layout() {
    assert_eq!(
        size_of::<*mut ForeignExportsList>(),
        size_of::<*mut sys::ForeignExportsList>()
    );
    assert_eq!(
        offset_of!(ForeignExportsList, next),
        offset_of!(sys::ForeignExportsList, next)
    );
    assert_eq!(
        offset_of!(ForeignExportsList, n_entries),
        offset_of!(sys::ForeignExportsList, n_entries)
    );
    assert_eq!(
        size_of::<*mut _ObjectCode>(),
        size_of::<*mut sys::_ObjectCode>()
    );
    assert_eq!(
        offset_of!(ForeignExportsList, oc),
        offset_of!(sys::ForeignExportsList, oc)
    );
    assert_eq!(
        offset_of!(ForeignExportsList, stable_ptrs),
        offset_of!(sys::ForeignExportsList, stable_ptrs)
    );
    assert_eq!(
        offset_of!(ForeignExportsList, exports),
        offset_of!(sys::ForeignExportsList, exports)
    );
    assert_eq!(
        size_of::<ForeignExportsList>(),
        size_of::<sys::ForeignExportsList>()
    );
    assert_eq!(
        align_of::<ForeignExportsList>(),
        align_of::<sys::ForeignExportsList>()
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_registerForeignExports() {
    let expected = {
        let mut exports: sys::ForeignExportsList = todo!();
        unsafe { sys::registerForeignExports(&raw mut exports) };
        todo!()
    };

    let actual = {
        let mut exports: ForeignExportsList = todo!();
        unsafe { registerForeignExports(&raw mut exports) };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_registerForeignExports() {
    let actual = {
        let exports: ForeignExportsList = todo!();
        unsafe { registerForeignExports(&raw mut exports) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
