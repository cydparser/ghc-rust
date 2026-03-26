use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_IN_STG_CODE_eq() {
    assert_eq!(IN_STG_CODE, sys::IN_STG_CODE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_IN_STG_CODE_layout() {
    assert_eq!(size_of_val(&IN_STG_CODE), size_of_val(&sys::IN_STG_CODE));
    assert_eq!(align_of_val(&IN_STG_CODE), align_of_val(&sys::IN_STG_CODE));
}

#[cfg(feature = "sys")]
#[test]
fn sys__REENTRANT_eq() {
    assert_eq!(_REENTRANT, sys::_REENTRANT);
}

#[cfg(feature = "sys")]
#[test]
fn sys__REENTRANT_layout() {
    assert_eq!(size_of_val(&_REENTRANT), size_of_val(&sys::_REENTRANT));
    assert_eq!(align_of_val(&_REENTRANT), align_of_val(&sys::_REENTRANT));
}

#[cfg(feature = "sys")]
#[test]
fn sys_EXIT_INTERNAL_ERROR_eq() {
    assert_eq!(EXIT_INTERNAL_ERROR, sys::EXIT_INTERNAL_ERROR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_EXIT_INTERNAL_ERROR_layout() {
    assert_eq!(
        size_of_val(&EXIT_INTERNAL_ERROR),
        size_of_val(&sys::EXIT_INTERNAL_ERROR)
    );
    assert_eq!(
        align_of_val(&EXIT_INTERNAL_ERROR),
        align_of_val(&sys::EXIT_INTERNAL_ERROR)
    );
}
