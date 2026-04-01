use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_rtsTimerSignal() {
    let expected: i32 = { unsafe { sys::rtsTimerSignal() } };
    let actual: i32 = { unsafe { rtsTimerSignal() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rtsTimerSignal() {
    let actual: i32 = { unsafe { rtsTimerSignal() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}
