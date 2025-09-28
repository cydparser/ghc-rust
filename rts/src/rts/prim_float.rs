use crate::prelude::*;
use crate::stg::types::StgDouble;
use crate::stg::{I_, W_};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust___int_encodeDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __int_encodeDouble(j: I_, e: I_) -> StgDouble {
    unsafe { sys::__int_encodeDouble(j, e) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust___word_encodeDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn __word_encodeDouble(j: W_, e: I_) -> StgDouble {
    unsafe { sys::__word_encodeDouble(j, e) }
}
