use crate::prelude::*;
use crate::stg::types::StgDouble;
use crate::stg::{I_, W_};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __int_encodeDouble(j: I_, e: I_) -> StgDouble {
    #[cfg(feature = "sys")]
    unsafe {
        sys::__int_encodeDouble(j, e)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("__int_encodeDouble")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __word_encodeDouble(j: W_, e: I_) -> StgDouble {
    #[cfg(feature = "sys")]
    unsafe {
        sys::__word_encodeDouble(j, e)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("__word_encodeDouble")
}
