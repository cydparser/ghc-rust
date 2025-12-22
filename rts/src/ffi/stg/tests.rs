#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_C__layout() {
    assert_eq!(size_of::<C_>(), size_of::<C_>());
    assert_eq!(align_of::<C_>(), align_of::<C_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_W__layout() {
    assert_eq!(size_of::<W_>(), size_of::<W_>());
    assert_eq!(align_of::<W_>(), align_of::<W_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_P__layout() {
    assert_eq!(size_of::<P_>(), size_of::<P_>());
    assert_eq!(align_of::<P_>(), align_of::<P_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_I__layout() {
    assert_eq!(size_of::<I_>(), size_of::<I_>());
    assert_eq!(align_of::<I_>(), align_of::<I_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_F__layout() {
    assert_eq!(size_of::<F_>(), size_of::<F_>());
    assert_eq!(align_of::<F_>(), align_of::<F_>());
}
