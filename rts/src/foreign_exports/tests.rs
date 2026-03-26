use super::*;

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
