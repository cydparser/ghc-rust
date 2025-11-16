use crate::ffi::stg::types::StgPtr;
#[cfg(feature = "sys")]
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct spEntry {
    addr: StgPtr,
}

#[cfg(feature = "sys")]
impl From<spEntry> for sys::spEntry {
    fn from(x: spEntry) -> Self {
        unsafe { transmute(x) }
    }
}
