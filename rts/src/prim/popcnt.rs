use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut popcount_tab: [u8; 256] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt8(mut x: StgWord) -> StgWord {
    return popcount_tab[x as u8 as usize] as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt16(mut x: StgWord) -> StgWord {
    return (popcount_tab[x as u8 as usize] as i32 + popcount_tab[(x >> 8) as u8 as usize] as i32)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt32(mut x: StgWord) -> StgWord {
    return (popcount_tab[x as u8 as usize] as i32
        + popcount_tab[(x >> 8) as u8 as usize] as i32
        + popcount_tab[(x >> 16) as u8 as usize] as i32
        + popcount_tab[(x >> 24) as u8 as usize] as i32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt64(mut x: StgWord64) -> StgWord {
    return (popcount_tab[x as u8 as usize] as i32
        + popcount_tab[(x >> 8) as u8 as usize] as i32
        + popcount_tab[(x >> 16) as u8 as usize] as i32
        + popcount_tab[(x >> 24) as u8 as usize] as i32
        + popcount_tab[(x >> 32) as u8 as usize] as i32
        + popcount_tab[(x >> 40) as u8 as usize] as i32
        + popcount_tab[(x >> 48) as u8 as usize] as i32
        + popcount_tab[(x >> 56) as u8 as usize] as i32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt(mut x: StgWord) -> StgWord {
    return (popcount_tab[x as u8 as usize] as i32
        + popcount_tab[(x >> 8) as u8 as usize] as i32
        + popcount_tab[(x >> 16) as u8 as usize] as i32
        + popcount_tab[(x >> 24) as u8 as usize] as i32
        + popcount_tab[(x >> 32) as u8 as usize] as i32
        + popcount_tab[(x >> 40) as u8 as usize] as i32
        + popcount_tab[(x >> 48) as u8 as usize] as i32
        + popcount_tab[(x >> 56) as u8 as usize] as i32) as StgWord;
}
