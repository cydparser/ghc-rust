use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz8(mut x: StgWord) -> StgWord {
    return (if x as uint8_t as c_int != 0 {
        (x as uint8_t as c_uint).leading_zeros() as i32 - 24 as c_int
    } else {
        8 as c_int
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz16(mut x: StgWord) -> StgWord {
    return (if x as uint16_t as c_int != 0 {
        (x as uint16_t as c_uint).leading_zeros() as i32 - 16 as c_int
    } else {
        16 as c_int
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz32(mut x: StgWord) -> StgWord {
    return (if x as uint32_t != 0 {
        (x as uint32_t).leading_zeros() as i32
    } else {
        32 as c_int
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz64(mut x: StgWord64) -> StgWord {
    return (if x != 0 {
        (x as c_ulong).leading_zeros() as i32
    } else {
        64 as c_int
    }) as StgWord;
}
