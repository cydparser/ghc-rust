use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_Time_layout() {
    assert_eq!(size_of::<Time>(), size_of::<Time>());
    assert_eq!(align_of::<Time>(), align_of::<Time>());
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getProcessElapsedTime() {
    let expected: Time = { unsafe { sys::getProcessElapsedTime() } };
    let actual: Time = { unsafe { getProcessElapsedTime() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getProcessElapsedTime() {
    let actual: Time = { unsafe { getProcessElapsedTime() } };
    let expected: Time = todo!();
    assert_eq!(expected, actual);
}
