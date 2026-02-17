use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setIOManagerControlFd(cap_no: u32, fd: c_int) -> bool {
    let expected = {
        unsafe { sys::setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    let actual = {
        unsafe { setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setIOManagerControlFd() {
    let g = &mut Gen::new(100);
    let actual = {
        let cap_no: u32 = Arbitrary::arbitrary(g);
        let fd: c_int = Arbitrary::arbitrary(g);
        unsafe { setIOManagerControlFd(cap_no, fd) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setTimerManagerControlFd(fd: c_int) -> bool {
    let expected = {
        unsafe { sys::setTimerManagerControlFd(fd) };
        todo!()
    };

    let actual = {
        unsafe { setTimerManagerControlFd(fd) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setTimerManagerControlFd() {
    let g = &mut Gen::new(100);
    let actual = {
        let fd: c_int = Arbitrary::arbitrary(g);
        unsafe { setTimerManagerControlFd(fd) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setIOManagerWakeupFd(fd: c_int) -> bool {
    let expected = {
        unsafe { sys::setIOManagerWakeupFd(fd) };
        todo!()
    };

    let actual = {
        unsafe { setIOManagerWakeupFd(fd) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setIOManagerWakeupFd() {
    let g = &mut Gen::new(100);
    let actual = {
        let fd: c_int = Arbitrary::arbitrary(g);
        unsafe { setIOManagerWakeupFd(fd) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
