use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep64(mut src: StgWord64, mut mask: StgWord64) -> StgWord64 {
    let mut result: u64 = 0;

    loop {
        let lowest: u64 = (mask as u64).wrapping_neg() & mask as u64;

        if lowest == 0 {
            break;
        }

        let lsb: u64 = ((src << 63) as i64 >> 63) as u64;
        result |= lsb & lowest;
        mask &= !lowest;
        src >>= 1;
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
