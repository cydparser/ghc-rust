use crate::ffi::stg::types::{StgWord, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev8(mut x: StgWord) -> StgWord {
    x = x >> 1 as c_int & 0x55 as StgWord | (x & 0x55 as StgWord) << 1 as c_int;
    x = x >> 2 as c_int & 0x33 as StgWord | (x & 0x33 as StgWord) << 2 as c_int;
    x = x >> 4 as c_int & 0xf as StgWord | (x & 0xf as StgWord) << 4 as c_int;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev16(mut x: StgWord16) -> StgWord16 {
    x = (x as c_int >> 1 as c_int & 0x5555 as c_int | (x as c_int & 0x5555 as c_int) << 1 as c_int)
        as StgWord16;
    x = (x as c_int >> 2 as c_int & 0x3333 as c_int | (x as c_int & 0x3333 as c_int) << 2 as c_int)
        as StgWord16;
    x = (x as c_int >> 4 as c_int & 0xf0f as c_int | (x as c_int & 0xf0f as c_int) << 4 as c_int)
        as StgWord16;
    x = (x as c_int >> 8 as c_int & 0xff as c_int | (x as c_int & 0xff as c_int) << 8 as c_int)
        as StgWord16;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev32(mut x: StgWord32) -> StgWord32 {
    x = x >> 1 as c_int & 0x55555555 as StgWord32 | (x & 0x55555555 as StgWord32) << 1 as c_int;
    x = x >> 2 as c_int & 0x33333333 as StgWord32 | (x & 0x33333333 as StgWord32) << 2 as c_int;
    x = x >> 4 as c_int & 0xf0f0f0f as StgWord32 | (x & 0xf0f0f0f as StgWord32) << 4 as c_int;
    x = x >> 8 as c_int & 0xff00ff as StgWord32 | (x & 0xff00ff as StgWord32) << 8 as c_int;
    x = x >> 16 as c_int | x << 16 as c_int;

    return x;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev64(mut x: StgWord64) -> StgWord64 {
    x = x >> 1 as c_int & 0x5555555555555555 as StgWord64
        | (x & 0x5555555555555555 as StgWord64) << 1 as c_int;
    x = x >> 2 as c_int & 0x3333333333333333 as StgWord64
        | (x & 0x3333333333333333 as StgWord64) << 2 as c_int;
    x = x >> 4 as c_int & 0xf0f0f0f0f0f0f0f as StgWord64
        | (x & 0xf0f0f0f0f0f0f0f as StgWord64) << 4 as c_int;
    x = x >> 8 as c_int & 0xff00ff00ff00ff as StgWord64
        | (x & 0xff00ff00ff00ff as StgWord64) << 8 as c_int;
    x = x >> 16 as c_int & 0xffff0000ffff as StgWord64
        | (x & 0xffff0000ffff as StgWord64) << 16 as c_int;
    x = x >> 32 as c_int | x << 32 as c_int;

    return x;
}
