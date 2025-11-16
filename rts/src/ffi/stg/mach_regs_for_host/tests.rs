#![allow(unused_imports)]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MACHREGS_NO_REGS() {
    assert_eq!(sys::MACHREGS_NO_REGS, MACHREGS_NO_REGS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MACHREGS_aarch64() {
    assert_eq!(sys::MACHREGS_aarch64, MACHREGS_aarch64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_MACHREGS_darwin() {
    assert_eq!(sys::MACHREGS_darwin, MACHREGS_darwin);
}
