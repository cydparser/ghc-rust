use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_BITS_PER_BYTE() {
    assert_eq!(sys::BITS_PER_BYTE, BITS_PER_BYTE);
}
