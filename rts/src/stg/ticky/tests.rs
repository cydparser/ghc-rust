use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TICKY_BIN_COUNT() {
    assert_eq!(sys::TICKY_BIN_COUNT, TICKY_BIN_COUNT);
}
