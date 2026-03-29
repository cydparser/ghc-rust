use crate::ffi::stg::types::{StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap16(mut x: StgWord16) -> StgWord16 {
    return (x as i32 >> 8 | (x as i32) << 8) as StgWord16;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap32(mut x: StgWord32) -> StgWord32 {
    return x >> 24 | x >> 8 & 0xff00 | x << 24 | (x & 0xff00) << 8;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap64(mut x: StgWord64) -> StgWord64 {
    return x >> 56
        | x << 56
        | x >> 40 & 0xff00
        | (x & 0xff00) << 40
        | x >> 24 & 0xff0000
        | (x & 0xff0000) << 24
        | x >> 8 & 0xff000000
        | (x & 0xff000000) << 8;
}
