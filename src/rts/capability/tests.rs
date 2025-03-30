use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use std::mem::size_of;

#[cfg(feature = "sys")]
#[test]
fn test_size_of_Capability_() {
    assert_eq!(size_of::<sys::Capability_>(), size_of::<Capability_>())
}
