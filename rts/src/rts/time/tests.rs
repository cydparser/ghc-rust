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
#[quickcheck]
#[ignore]
fn equivalent_getProcessElapsedTime() -> bool {
    let expected = unsafe { sys::getProcessElapsedTime() };
    let actual = unsafe { getProcessElapsedTime() };
    actual == expected
}

#[test]
#[ignore]
fn test_getProcessElapsedTime() {
    unsafe { getProcessElapsedTime() };
    todo!("assert")
}
