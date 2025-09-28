use crate::prelude::*;
use crate::stg::types::{StgDouble, StgFloat, StgWord, StgWord16, StgWord32, StgWord64};

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_add8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_add8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_add16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_add16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_add32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_add32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_add64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_add64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_sub8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_sub8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_sub16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_sub16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_sub32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_sub32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_sub64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_sub64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_and8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_and8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_and16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_and16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_and32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_and32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_and64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_and64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_nand8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_nand8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_nand16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_nand16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_nand32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_nand32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_nand64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_nand64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_or8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_or8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_or16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_or16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_or32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_or32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_or64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_or64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_xor8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_xor8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_xor16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_xor16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_xor32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_atomic_xor32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomic_xor64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_atomic_xor64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_cmpxchg8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { sys::hs_cmpxchg8(x, old, new_) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_cmpxchg16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { sys::hs_cmpxchg16(x, old, new_) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_cmpxchg32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { sys::hs_cmpxchg32(x, old, new_) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_cmpxchg64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> StgWord64 {
    unsafe { sys::hs_cmpxchg64(x, old, new_) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicread8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicread8(x: StgWord) -> StgWord {
    unsafe { sys::hs_atomicread8(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicread16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicread16(x: StgWord) -> StgWord {
    unsafe { sys::hs_atomicread16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicread32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicread32(x: StgWord) -> StgWord {
    unsafe { sys::hs_atomicread32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicread64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicread64(x: StgWord) -> StgWord64 {
    unsafe { sys::hs_atomicread64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicwrite8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite8(x: StgWord, val: StgWord) {
    unsafe { sys::hs_atomicwrite8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicwrite16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite16(x: StgWord, val: StgWord) {
    unsafe { sys::hs_atomicwrite16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicwrite32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite32(x: StgWord, val: StgWord) {
    unsafe { sys::hs_atomicwrite32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_atomicwrite64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite64(x: StgWord, val: StgWord64) {
    unsafe { sys::hs_atomicwrite64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_xchg8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_xchg8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_xchg8(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_xchg16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_xchg16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_xchg16(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_xchg32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_xchg32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { sys::hs_xchg32(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_xchg64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_xchg64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { sys::hs_xchg64(x, val) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bswap16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bswap16(x: StgWord16) -> StgWord16 {
    unsafe { sys::hs_bswap16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bswap32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bswap32(x: StgWord32) -> StgWord32 {
    unsafe { sys::hs_bswap32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bswap64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bswap64(x: StgWord64) -> StgWord64 {
    unsafe { sys::hs_bswap64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bitrev8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bitrev8(x: StgWord) -> StgWord {
    unsafe { sys::hs_bitrev8(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bitrev16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bitrev16(x: StgWord16) -> StgWord16 {
    unsafe { sys::hs_bitrev16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bitrev32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bitrev32(x: StgWord32) -> StgWord32 {
    unsafe { sys::hs_bitrev32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_bitrev64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_bitrev64(x: StgWord64) -> StgWord64 {
    unsafe { sys::hs_bitrev64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pdep64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pdep64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    unsafe { sys::hs_pdep64(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pdep32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pdep32(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pdep32(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pdep16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pdep16(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pdep16(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pdep8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pdep8(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pdep8(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pext64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pext64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    unsafe { sys::hs_pext64(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pext32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pext32(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pext32(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pext16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pext16(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pext16(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_pext8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_pext8(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { sys::hs_pext8(src, mask) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_popcnt8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_popcnt8(x: StgWord) -> StgWord {
    unsafe { sys::hs_popcnt8(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_popcnt16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_popcnt16(x: StgWord) -> StgWord {
    unsafe { sys::hs_popcnt16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_popcnt32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_popcnt32(x: StgWord) -> StgWord {
    unsafe { sys::hs_popcnt32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_popcnt64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_popcnt64(x: StgWord64) -> StgWord {
    unsafe { sys::hs_popcnt64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_popcnt"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_popcnt(x: StgWord) -> StgWord {
    unsafe { sys::hs_popcnt(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_word2float32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_word2float32(x: StgWord) -> StgFloat {
    unsafe { sys::hs_word2float32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_word2float64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_word2float64(x: StgWord) -> StgDouble {
    unsafe { sys::hs_word2float64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_clz8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_clz8(x: StgWord) -> StgWord {
    unsafe { sys::hs_clz8(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_clz16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_clz16(x: StgWord) -> StgWord {
    unsafe { sys::hs_clz16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_clz32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_clz32(x: StgWord) -> StgWord {
    unsafe { sys::hs_clz32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_clz64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_clz64(x: StgWord64) -> StgWord {
    unsafe { sys::hs_clz64(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_ctz8"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_ctz8(x: StgWord) -> StgWord {
    unsafe { sys::hs_ctz8(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_ctz16"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_ctz16(x: StgWord) -> StgWord {
    unsafe { sys::hs_ctz16(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_ctz32"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_ctz32(x: StgWord) -> StgWord {
    unsafe { sys::hs_ctz32(x) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_ctz64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn hs_ctz64(x: StgWord64) -> StgWord {
    unsafe { sys::hs_ctz64(x) }
}
