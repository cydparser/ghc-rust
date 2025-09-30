#![allow(unused_imports)]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BITS_PER_BYTE() {
    assert_eq!(sys::BITS_PER_BYTE, BITS_PER_BYTE);
}
