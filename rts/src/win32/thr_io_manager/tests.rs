use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getIOManagerEvent() {
    let expected = {
        let result: &c_void = unsafe { &*sys::getIOManagerEvent() };
        todo!()
    };

    let actual = {
        let result: &c_void = unsafe { &*getIOManagerEvent() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getIOManagerEvent() {
    let actual = {
        let result: &c_void = unsafe { &*getIOManagerEvent() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_readIOManagerEvent() {
    let expected: HsWord32 = { unsafe { sys::readIOManagerEvent() } };
    let actual: HsWord32 = { unsafe { readIOManagerEvent() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_readIOManagerEvent() {
    let actual: HsWord32 = { unsafe { readIOManagerEvent() } };
    let expected: HsWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_sendIOManagerEvent(event: HsWord32) -> bool {
    let expected = {
        unsafe { sys::sendIOManagerEvent(event) };
        todo!()
    };

    let actual = {
        unsafe { sendIOManagerEvent(event) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_sendIOManagerEvent() {
    let g = &mut Gen::new(100);

    let actual = {
        let event: HsWord32 = Arbitrary::arbitrary(g);
        unsafe { sendIOManagerEvent(event) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
