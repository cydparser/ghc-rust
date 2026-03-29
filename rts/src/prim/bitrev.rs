use crate::ffi::stg::types::{StgWord, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev8(mut x: StgWord) -> StgWord {
    x = x >> 1 & 0x55 | (x & 0x55) << 1;
    x = x >> 2 & 0x33 | (x & 0x33) << 2;
    x = x >> 4 & 0xf | (x & 0xf) << 4;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev16(mut x: StgWord16) -> StgWord16 {
    x = (x as i32 >> 1 & 0x5555 | (x as i32 & 0x5555) << 1) as StgWord16;
    x = (x as i32 >> 2 & 0x3333 | (x as i32 & 0x3333) << 2) as StgWord16;
    x = (x as i32 >> 4 & 0xf0f | (x as i32 & 0xf0f) << 4) as StgWord16;
    x = (x as i32 >> 8 & 0xff | (x as i32 & 0xff) << 8) as StgWord16;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev32(mut x: StgWord32) -> StgWord32 {
    x = x >> 1 & 0x55555555 | (x & 0x55555555) << 1;
    x = x >> 2 & 0x33333333 | (x & 0x33333333) << 2;
    x = x >> 4 & 0xf0f0f0f | (x & 0xf0f0f0f) << 4;
    x = x >> 8 & 0xff00ff | (x & 0xff00ff) << 8;
    x = x >> 16 | x << 16;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev64(mut x: StgWord64) -> StgWord64 {
    x = x >> 1 & 0x5555555555555555 | (x & 0x5555555555555555) << 1;
    x = x >> 2 & 0x3333333333333333 | (x & 0x3333333333333333) << 2;
    x = x >> 4 & 0xf0f0f0f0f0f0f0f | (x & 0xf0f0f0f0f0f0f0f) << 4;
    x = x >> 8 & 0xff00ff00ff00ff | (x & 0xff00ff00ff00ff) << 8;
    x = x >> 16 & 0xffff0000ffff | (x & 0xffff0000ffff) << 16;
    x = x >> 32 | x << 32;

    return x;
}
