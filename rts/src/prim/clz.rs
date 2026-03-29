use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz8(mut x: StgWord) -> StgWord {
    return (if x as u8 as i32 != 0 {
        (x as u8 as u32).leading_zeros() as i32 - 24
    } else {
        8
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz16(mut x: StgWord) -> StgWord {
    return (if x as u16 as i32 != 0 {
        (x as u16 as u32).leading_zeros() as i32 - 16
    } else {
        16
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz32(mut x: StgWord) -> StgWord {
    return (if x as u32 != 0 {
        (x as u32).leading_zeros() as i32
    } else {
        32
    }) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz64(mut x: StgWord64) -> StgWord {
    return (if x != 0 {
        (x as u64).leading_zeros() as i32
    } else {
        64
    }) as StgWord;
}
