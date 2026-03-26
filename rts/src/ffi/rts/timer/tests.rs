use super::*;

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
