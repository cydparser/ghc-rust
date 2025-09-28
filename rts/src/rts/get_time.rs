use crate::prelude::*;
use crate::stg::types::StgWord64;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getMonotonicNSec"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getMonotonicNSec() -> StgWord64 {
    unsafe { sys::getMonotonicNSec() }
}
