use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_DFL_eq() {
    assert_eq!(STG_SIG_DFL, sys::STG_SIG_DFL);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_DFL_layout() {
    assert_eq!(size_of_val(&STG_SIG_DFL), size_of_val(&sys::STG_SIG_DFL));
    assert_eq!(align_of_val(&STG_SIG_DFL), align_of_val(&sys::STG_SIG_DFL));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_IGN_eq() {
    assert_eq!(STG_SIG_IGN, sys::STG_SIG_IGN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_IGN_layout() {
    assert_eq!(size_of_val(&STG_SIG_IGN), size_of_val(&sys::STG_SIG_IGN));
    assert_eq!(align_of_val(&STG_SIG_IGN), align_of_val(&sys::STG_SIG_IGN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_ERR_eq() {
    assert_eq!(STG_SIG_ERR, sys::STG_SIG_ERR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_ERR_layout() {
    assert_eq!(size_of_val(&STG_SIG_ERR), size_of_val(&sys::STG_SIG_ERR));
    assert_eq!(align_of_val(&STG_SIG_ERR), align_of_val(&sys::STG_SIG_ERR));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_HAN_eq() {
    assert_eq!(STG_SIG_HAN, sys::STG_SIG_HAN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_HAN_layout() {
    assert_eq!(size_of_val(&STG_SIG_HAN), size_of_val(&sys::STG_SIG_HAN));
    assert_eq!(align_of_val(&STG_SIG_HAN), align_of_val(&sys::STG_SIG_HAN));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_RST_eq() {
    assert_eq!(STG_SIG_RST, sys::STG_SIG_RST);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_SIG_RST_layout() {
    assert_eq!(size_of_val(&STG_SIG_RST), size_of_val(&sys::STG_SIG_RST));
    assert_eq!(align_of_val(&STG_SIG_RST), align_of_val(&sys::STG_SIG_RST));
}
