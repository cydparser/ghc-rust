#[cfg(feature = "sys")]
use crate::prelude::*;
use crate::stg::types::StgPtr;

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
