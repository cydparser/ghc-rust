use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_add8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_add8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_add16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_add16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_add32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_add32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_add64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_add64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_sub8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_sub8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_sub16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_sub16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_sub32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_sub32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_sub64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_sub64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_and8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_and8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_and16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_and16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_and32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_and32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_and64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_and64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_nand8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_nand8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_nand16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_nand16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_nand32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_nand32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_nand64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_nand64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_or8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_or8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_or16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_or16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_or32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_or32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_or64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_or64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_xor8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_xor8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_xor16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_xor16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_xor32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomic_xor32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomic_xor64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_atomic_xor64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_cmpxchg8(x.into(), old.into(), new_.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_cmpxchg16(x.into(), old.into(), new_.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_cmpxchg32(x.into(), old.into(), new_.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_cmpxchg64(x.into(), old.into(), new_.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicread8(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomicread8(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicread16(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomicread16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicread32(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_atomicread32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicread64(x: StgWord) -> StgWord64 {
    unsafe { transmute(sys::hs_atomicread64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicwrite8(x: StgWord, val: StgWord) {
    unsafe { transmute(sys::hs_atomicwrite8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicwrite16(x: StgWord, val: StgWord) {
    unsafe { transmute(sys::hs_atomicwrite16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicwrite32(x: StgWord, val: StgWord) {
    unsafe { transmute(sys::hs_atomicwrite32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_atomicwrite64(x: StgWord, val: StgWord64) {
    unsafe { transmute(sys::hs_atomicwrite64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_xchg8(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_xchg8(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_xchg16(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_xchg16(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_xchg32(x: StgWord, val: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_xchg32(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_xchg64(x: StgWord, val: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_xchg64(x.into(), val.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bswap16(x: StgWord16) -> StgWord16 {
    unsafe { transmute(sys::hs_bswap16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bswap32(x: StgWord32) -> StgWord32 {
    unsafe { transmute(sys::hs_bswap32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bswap64(x: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_bswap64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bitrev8(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_bitrev8(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bitrev16(x: StgWord16) -> StgWord16 {
    unsafe { transmute(sys::hs_bitrev16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bitrev32(x: StgWord32) -> StgWord32 {
    unsafe { transmute(sys::hs_bitrev32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_bitrev64(x: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_bitrev64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pdep64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_pdep64(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pdep32(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pdep32(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pdep16(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pdep16(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pdep8(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pdep8(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pext64(src: StgWord64, mask: StgWord64) -> StgWord64 {
    unsafe { transmute(sys::hs_pext64(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pext32(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pext32(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pext16(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pext16(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_pext8(src: StgWord, mask: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_pext8(src.into(), mask.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_popcnt8(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_popcnt8(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_popcnt16(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_popcnt16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_popcnt32(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_popcnt32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_popcnt64(x: StgWord64) -> StgWord {
    unsafe { transmute(sys::hs_popcnt64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_popcnt(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_popcnt(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_word2float32(x: StgWord) -> StgFloat {
    unsafe { transmute(sys::hs_word2float32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_word2float64(x: StgWord) -> StgDouble {
    unsafe { transmute(sys::hs_word2float64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_clz8(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_clz8(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_clz16(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_clz16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_clz32(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_clz32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_clz64(x: StgWord64) -> StgWord {
    unsafe { transmute(sys::hs_clz64(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_ctz8(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_ctz8(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_ctz16(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_ctz16(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_ctz32(x: StgWord) -> StgWord {
    unsafe { transmute(sys::hs_ctz32(x.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_ctz64(x: StgWord64) -> StgWord {
    unsafe { transmute(sys::hs_ctz64(x.into())) }
}
