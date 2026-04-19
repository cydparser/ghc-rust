use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_Time_layout() {
    assert_eq!(size_of::<Time>(), size_of::<sys::Time>());
    assert_eq!(align_of::<Time>(), align_of::<sys::Time>());
}
