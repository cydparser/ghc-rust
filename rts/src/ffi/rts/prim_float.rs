use crate::ffi::stg::types::StgDouble;
use crate::ffi::stg::{I_, W_};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __int_encodeDouble(j: I_, e: I_) -> StgDouble {
    sys! {
        __int_encodeDouble(j, e)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn __word_encodeDouble(j: W_, e: I_) -> StgDouble {
    sys! {
        __word_encodeDouble(j, e)
    }
}
