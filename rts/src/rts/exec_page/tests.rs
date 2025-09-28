use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_ExecPage() {
    assert_eq!(size_of::<sys::ExecPage>(), size_of::<ExecPage>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ExecPage"][size_of::<ExecPage>() - 1usize];
    ["Alignment of ExecPage"][align_of::<ExecPage>() - 1usize];
    ["Offset of field: ExecPage::contents"][offset_of!(ExecPage, contents) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_allocateExecPage() {
    let expected: &ExecPage = { unsafe { transmute(&*sys::allocateExecPage()) } };
    let actual: &ExecPage = { unsafe { &*allocateExecPage() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_allocateExecPage() {
    let actual: &ExecPage = { unsafe { &*allocateExecPage() } };
    let expected: &ExecPage = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_freezeExecPage(page: ExecPage) -> bool {
    let expected = {
        let mut page = page.clone().into();
        unsafe { sys::freezeExecPage(&raw mut page) };
        todo!()
    };
    let actual = {
        let mut page = page.clone();
        unsafe { freezeExecPage(&raw mut page) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_freezeExecPage() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut page: ExecPage = Arbitrary::arbitrary(g);
        unsafe { freezeExecPage(&raw mut page) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_freeExecPage(page: ExecPage) -> bool {
    let expected = {
        let mut page = page.clone().into();
        unsafe { sys::freeExecPage(&raw mut page) };
        todo!()
    };
    let actual = {
        let mut page = page.clone();
        unsafe { freeExecPage(&raw mut page) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_freeExecPage() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut page: ExecPage = Arbitrary::arbitrary(g);
        unsafe { freeExecPage(&raw mut page) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
