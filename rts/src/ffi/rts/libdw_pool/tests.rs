use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_libdwPoolTake() {
    let expected = {
        let result: &LibdwSession = unsafe { transmute(&*sys::libdwPoolTake()) };
        todo!()
    };

    let actual = {
        let result: &LibdwSession = unsafe { &*libdwPoolTake() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_libdwPoolTake() {
    let actual = {
        let result: &LibdwSession = unsafe { &*libdwPoolTake() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_libdwPoolRelease() {
    let expected = {
        let mut sess: sys::LibdwSession = todo!();
        unsafe { sys::libdwPoolRelease(&raw mut sess) };
        todo!()
    };

    let actual = {
        let mut sess: LibdwSession = todo!();
        unsafe { libdwPoolRelease(&raw mut sess) };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_libdwPoolRelease() {
    let actual = {
        let sess: LibdwSession = todo!();
        unsafe { libdwPoolRelease(&raw mut sess) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_libdwPoolClear() {
    let expected = {
        unsafe { sys::libdwPoolClear() };
        todo!()
    };

    let actual = {
        unsafe { libdwPoolClear() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_libdwPoolClear() {
    let actual = {
        unsafe { libdwPoolClear() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
