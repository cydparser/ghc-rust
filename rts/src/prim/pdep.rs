use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep64(mut src: StgWord64, mut mask: StgWord64) -> StgWord64 {
    let mut result: uint64_t = 0 as uint64_t;

    loop {
        let lowest: uint64_t = (mask as uint64_t).wrapping_neg() & mask as uint64_t;

        if lowest == 0 as uint64_t {
            break;
        }

        let lsb: uint64_t = ((src << 63 as c_int) as int64_t >> 63 as c_int) as uint64_t;
        result |= lsb & lowest;
        mask &= !lowest;
        src >>= 1 as c_int;
    }

    return result as StgWord64;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep32(mut src: StgWord, mut mask: StgWord) -> StgWord {
    return hs_pdep64(src as StgWord64, mask as StgWord64) as StgWord32 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep16(mut src: StgWord, mut mask: StgWord) -> StgWord {
    return hs_pdep64(src as StgWord64, mask as StgWord64) as StgWord16 as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep8(mut src: StgWord, mut mask: StgWord) -> StgWord {
    return hs_pdep64(src as StgWord64, mask as StgWord64) as StgWord8 as StgWord;
}
