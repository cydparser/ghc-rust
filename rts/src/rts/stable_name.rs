use crate::prelude::*;
use crate::rts::storage::closures::StgClosure;
use crate::stg::types::StgPtr;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct snEntry {
    addr: StgPtr,
    old: StgPtr,
    sn_obj: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<snEntry> for sys::snEntry {
    fn from(x: snEntry) -> Self {
        unsafe { transmute(x) }
    }
}
