use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_RTS_USER_SIGNALS() {
    assert_eq!(sys::RTS_USER_SIGNALS, RTS_USER_SIGNALS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MAX_N_CAPABILITIES() {
    assert_eq!(sys::MAX_N_CAPABILITIES, MAX_N_CAPABILITIES);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_CACHELINE_SIZE() {
    assert_eq!(sys::CACHELINE_SIZE, CACHELINE_SIZE);
}
