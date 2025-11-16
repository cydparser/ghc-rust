use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TIME_RESOLUTION() {
    assert_eq!(sys::TIME_RESOLUTION, TIME_RESOLUTION);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TIME_MAX() {
    assert_eq!(sys::TIME_MAX, TIME_MAX);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getProcessElapsedTime() {
    let expected: Time = { unsafe { sys::getProcessElapsedTime() } };
    let actual: Time = { unsafe { getProcessElapsedTime() } };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getProcessElapsedTime() {
    let actual: Time = { unsafe { getProcessElapsedTime() } };
    let expected: Time = todo!();
    assert_eq!(expected, actual);
}
