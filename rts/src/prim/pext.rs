use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

unsafe fn hs_pext(bit_width: u8, src: StgWord64, mask: StgWord64) -> StgWord64 {
    let mut result: u64 = 0;
    let mut offset = 0;
    let mut bit = 0;

    while bit != bit_width as i32 {
        let src_bit: u64 = src as u64 >> bit & 1;
        let mask_bit: u64 = mask as u64 >> bit & 1;

        if mask_bit != 0 {
            result |= src_bit << offset;
            offset += 1;
        }

        bit += 1;
    }

    return result as StgWord64;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    return hs_pext(64, src, mask);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext32(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(32, src as StgWord64, mask as StgWord64) as StgWord32 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext16(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(16, src as StgWord64, mask as StgWord64) as StgWord16 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext8(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(8, src as StgWord64, mask as StgWord64) as StgWord8 as StgWord;
}
