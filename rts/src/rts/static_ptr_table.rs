use crate::prelude::*;
use crate::stg::types::{StgStablePtr, StgWord64};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_spt_insert_stableptr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert_stableptr(key: *mut StgWord64, entry: *mut StgStablePtr) {
    unsafe { sys::hs_spt_insert_stableptr(key, entry) }
}
