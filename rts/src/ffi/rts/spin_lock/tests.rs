#![cfg(feature = "sys")]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_SpinLock__layout() {
    assert_eq!(
        offset_of!(SpinLock_, lock),
        offset_of!(sys::SpinLock_, lock)
    );
    assert_eq!(
        offset_of!(SpinLock_, spin),
        offset_of!(sys::SpinLock_, spin)
    );
    assert_eq!(
        offset_of!(SpinLock_, yield_),
        offset_of!(sys::SpinLock_, yield_)
    );
    assert_eq!(size_of::<SpinLock_>(), size_of::<sys::SpinLock_>());
    assert_eq!(align_of::<SpinLock_>(), align_of::<sys::SpinLock_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_SpinLock_layout() {
    assert_eq!(size_of::<SpinLock>(), size_of::<sys::SpinLock>());
    assert_eq!(align_of::<SpinLock>(), align_of::<sys::SpinLock>());
}
