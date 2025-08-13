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

#[test]
#[ignore]
fn test_getProcessElapsedTime() {
    unsafe { getProcessElapsedTime() };
    todo!("assert")
}
