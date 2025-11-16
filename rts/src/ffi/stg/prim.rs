use crate::ffi::stg::types::{StgDouble, StgFloat, StgWord, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_add8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_add8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_add16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_add16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_add32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_add32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_add64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_add64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_sub8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_sub8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_sub16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_sub16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_sub32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_sub32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_sub64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_sub64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_and8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_and8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_and16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_and16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_and32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_and32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_and64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_and64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_nand8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_nand8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_nand16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_nand16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_nand32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_nand32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_nand64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_nand64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_or8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_or8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_or16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_or16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_or32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_or32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_or64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_or64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_xor8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_xor8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_xor16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_xor16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_xor32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_xor32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomic_xor64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomic_xor64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_cmpxchg8(x, old, new_)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_cmpxchg8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_cmpxchg16(x, old, new_)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_cmpxchg16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_cmpxchg32(x, old, new_)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_cmpxchg32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_cmpxchg64(x, old, new_)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_cmpxchg64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread8(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicread8(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicread8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread16(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicread16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicread16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread32(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicread32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicread32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread64(x: StgWord) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicread64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicread64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite8(x: StgWord, val: StgWord) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicwrite8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicwrite8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite16(x: StgWord, val: StgWord) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicwrite16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicwrite16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite32(x: StgWord, val: StgWord) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicwrite32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicwrite32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite64(x: StgWord, val: StgWord64) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_atomicwrite64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_atomicwrite64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg8(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_xchg8(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_xchg8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg16(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_xchg16(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_xchg16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg32(x: StgWord, val: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_xchg32(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_xchg32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg64(x: StgWord, val: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_xchg64(x, val)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_xchg64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap16(x: StgWord16) -> StgWord16 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bswap16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bswap16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap32(x: StgWord32) -> StgWord32 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bswap32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bswap32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bswap64(x: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bswap64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bswap64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev8(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bitrev8(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bitrev8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev16(x: StgWord16) -> StgWord16 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bitrev16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bitrev16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev32(x: StgWord32) -> StgWord32 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bitrev32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bitrev32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_bitrev64(x: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_bitrev64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_bitrev64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pdep64(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pdep64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep32(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pdep32(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pdep32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep16(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pdep16(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pdep16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pdep8(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pdep8(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pdep8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pext64(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pext64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext32(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pext32(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pext32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext16(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pext16(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pext16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_pext8(src: StgWord, mask: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_pext8(src, mask)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_pext8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt8(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_popcnt8(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_popcnt8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt16(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_popcnt16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_popcnt16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt32(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_popcnt32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_popcnt32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt64(x: StgWord64) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_popcnt64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_popcnt64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_popcnt(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_popcnt(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_popcnt")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_word2float32(x: StgWord) -> StgFloat {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_word2float32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_word2float32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_word2float64(x: StgWord) -> StgDouble {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_word2float64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_word2float64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz8(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_clz8(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_clz8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz16(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_clz16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_clz16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz32(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_clz32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_clz32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_clz64(x: StgWord64) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_clz64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_clz64")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_ctz8(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_ctz8(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_ctz8")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_ctz16(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_ctz16(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_ctz16")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_ctz32(x: StgWord) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_ctz32(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_ctz32")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_ctz64(x: StgWord64) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_ctz64(x)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_ctz64")
}
