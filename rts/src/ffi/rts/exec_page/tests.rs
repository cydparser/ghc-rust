use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_ExecPage_layout() {
    assert_eq!(
        offset_of!(ExecPage, contents),
        offset_of!(sys::ExecPage, contents)
    );
    assert_eq!(size_of::<ExecPage>(), size_of::<sys::ExecPage>());
    assert_eq!(align_of::<ExecPage>(), align_of::<sys::ExecPage>());
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_allocateExecPage() {
    let expected: &ExecPage = { unsafe { transmute(&*sys::allocateExecPage()) } };
    let actual: &ExecPage = { unsafe { &*allocateExecPage() } };
    assert_eq!(actual, expected);
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
        let mut page = unsafe { transmute(page.clone()) };
        unsafe { sys::freezeExecPage(&raw mut page) };
        todo!()
    };

    let actual = {
        let mut page = page.clone();
        unsafe { freezeExecPage(&raw mut page) };
        todo!()
    };

    actual == expected
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
