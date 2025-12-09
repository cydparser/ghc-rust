use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_startTimer() {
    let expected = {
        unsafe { sys::startTimer() };
        todo!()
    };
    let actual = {
        unsafe { startTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_startTimer() {
    let actual = {
        unsafe { startTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stopTimer() {
    let expected = {
        unsafe { sys::stopTimer() };
        todo!()
    };
    let actual = {
        unsafe { stopTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stopTimer() {
    let actual = {
        unsafe { stopTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rtsTimerSignal() {
    let expected: c_int = { unsafe { sys::rtsTimerSignal() } };
    let actual: c_int = { unsafe { rtsTimerSignal() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rtsTimerSignal() {
    let actual: c_int = { unsafe { rtsTimerSignal() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}
