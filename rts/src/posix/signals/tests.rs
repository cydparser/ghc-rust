use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_DFL_eq() {
    assert_eq!(STG_SIG_DFL, sys::STG_SIG_DFL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_IGN_eq() {
    assert_eq!(STG_SIG_IGN, sys::STG_SIG_IGN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_ERR_eq() {
    assert_eq!(STG_SIG_ERR, sys::STG_SIG_ERR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_HAN_eq() {
    assert_eq!(STG_SIG_HAN, sys::STG_SIG_HAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_RST_eq() {
    assert_eq!(STG_SIG_RST, sys::STG_SIG_RST);
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
fn test_blockUserSignals() {
    let actual = {
        unsafe { blockUserSignals() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_blockUserSignals() {
    let expected = {
        unsafe { sys::blockUserSignals() };
        todo!()
    };

    let actual = {
        unsafe { blockUserSignals() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unblockUserSignals() {
    let actual = {
        unsafe { unblockUserSignals() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_unblockUserSignals() {
    let expected = {
        unsafe { sys::unblockUserSignals() };
        todo!()
    };

    let actual = {
        unsafe { unblockUserSignals() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_sig_install() {
    let g = &mut Gen::new(100);

    let actual: c_int = {
        let arg1: c_int = Arbitrary::arbitrary(g);
        let arg2: c_int = Arbitrary::arbitrary(g);
        let arg3: c_void = todo!();
        unsafe { stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_sig_install(arg1: c_int, arg2: c_int) -> bool {
    let expected: c_int = {
        let mut arg3: c_void = todo!();
        unsafe { sys::stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

    let actual: c_int = {
        let mut arg3: c_void = todo!();
        unsafe { stg_sig_install(arg1, arg2, &raw mut arg3) }
    };

    actual == expected
}
