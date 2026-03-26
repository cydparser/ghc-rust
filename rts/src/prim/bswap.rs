use crate::ffi::stg::types::{StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap16(mut x: StgWord16) -> StgWord16 {
    return (x as c_int >> 8 as c_int | (x as c_int) << 8 as c_int) as StgWord16;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap32(mut x: StgWord32) -> StgWord32 {
    return x >> 24 as c_int
        | x >> 8 as c_int & 0xff00 as StgWord32
        | x << 24 as c_int
        | (x & 0xff00 as StgWord32) << 8 as c_int;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap64(mut x: StgWord64) -> StgWord64 {
    return x >> 56 as c_int
        | x << 56 as c_int
        | x >> 40 as c_int & 0xff00 as StgWord64
        | (x & 0xff00 as StgWord64) << 40 as c_int
        | x >> 24 as c_int & 0xff0000 as StgWord64
        | (x & 0xff0000 as StgWord64) << 24 as c_int
        | x >> 8 as c_int & 0xff000000 as StgWord64
        | (x & 0xff000000 as StgWord64) << 8 as c_int;
}
