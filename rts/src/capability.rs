#[cfg(feature = "sys")]
use crate::prelude::*;

/// - GHC_PLACES: {libraries, testsuite}
pub type Capability = Capability_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct Capability_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<Capability_> for sys::Capability_ {
    fn from(x: Capability_) -> Self {
        unsafe { transmute(x) }
    }
}
