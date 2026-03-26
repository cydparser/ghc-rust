use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

unsafe fn hs_pext(bit_width: c_uchar, src: StgWord64, mask: StgWord64) -> StgWord64 {
    let mut result: uint64_t = 0 as uint64_t;
    let mut offset = 0 as c_int;
    let mut bit = 0 as c_int;

    while bit != bit_width as c_int {
        let src_bit: uint64_t = src as uint64_t >> bit & 1 as uint64_t;
        let mask_bit: uint64_t = mask as uint64_t >> bit & 1 as uint64_t;

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
    return hs_pext(64 as c_uchar, src, mask);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext32(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(32 as c_uchar, src as StgWord64, mask as StgWord64) as StgWord32 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext16(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(16 as c_uchar, src as StgWord64, mask as StgWord64) as StgWord16 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext8(src: StgWord, mask: StgWord) -> StgWord {
    return hs_pext(8 as c_uchar, src as StgWord64, mask as StgWord64) as StgWord8 as StgWord;
}
