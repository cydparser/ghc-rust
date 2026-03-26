use crate::ffi::stg::types::{StgDouble, StgFloat, StgWord};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_word2float32(mut x: StgWord) -> StgFloat {
    return x as StgFloat;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_word2float64(mut x: StgWord) -> StgDouble {
    return x as StgDouble;
}
