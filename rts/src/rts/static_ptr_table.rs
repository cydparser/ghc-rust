use crate::prelude::*;
use crate::stg::types::{StgStablePtr, StgWord64};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert_stableptr(key: *mut StgWord64, entry: *mut StgStablePtr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_spt_insert_stableptr(key, entry)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_spt_insert_stableptr")
}
