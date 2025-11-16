use crate::prelude::*;
use crate::stg::types::StgWord64;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getMonotonicNSec() -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getMonotonicNSec()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getMonotonicNSec")
}
