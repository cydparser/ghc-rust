use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xadd_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xadd_seqcst(x as *mut StgWord16, val as StgWord16)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xadd_seqcst(x as *mut StgWord32, val as StgWord32)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_xadd_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xsub_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xsub_seqcst(x as *mut StgWord16, val as StgWord16)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xsub_seqcst(x as *mut StgWord32, val as StgWord32)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_xsub_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_and_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_and_seqcst(x as *mut StgWord16, val as StgWord16) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_and_seqcst(x as *mut StgWord32, val as StgWord32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_and_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_nand_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_nand_seqcst(x as *mut StgWord16, val as StgWord16)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_nand_seqcst(x as *mut StgWord32, val as StgWord32)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_nand_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_or_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_or_seqcst(x as *mut StgWord16, val as StgWord16) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_or_seqcst(x as *mut StgWord32, val as StgWord32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_or_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xor_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xor_seqcst(x as *mut StgWord16, val as StgWord16) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xor_seqcst(x as *mut StgWord32, val as StgWord32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_xor_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg8(
    mut x: StgWord,
    mut old: StgWord,
    mut new: StgWord,
) -> StgWord {
    let mut expected: StgWord8 = old as StgWord8;

    let fresh0 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        x as *mut StgWord8,
        *&raw mut expected,
        new as StgWord8,
    );

    *&raw mut expected = fresh0.0;
    fresh0.1;

    return expected as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg16(
    mut x: StgWord,
    mut old: StgWord,
    mut new: StgWord,
) -> StgWord {
    let mut expected: StgWord16 = old as StgWord16;

    let fresh1 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        x as *mut StgWord16,
        *&raw mut expected,
        new as StgWord16,
    );

    *&raw mut expected = fresh1.0;
    fresh1.1;

    return expected as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg32(
    mut x: StgWord,
    mut old: StgWord,
    mut new: StgWord,
) -> StgWord {
    let mut expected: StgWord32 = old as StgWord32;

    let fresh2 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        x as *mut StgWord32,
        *&raw mut expected,
        new as StgWord32,
    );

    *&raw mut expected = fresh2.0;
    fresh2.1;

    return expected as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_cmpxchg64(
    mut x: StgWord,
    mut old: StgWord64,
    mut new: StgWord64,
) -> StgWord64 {
    let mut expected: StgWord64 = old;

    let fresh3 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
        x as *mut StgWord64,
        *&raw mut expected,
        new,
    );

    *&raw mut expected = fresh3.0;
    fresh3.1;

    return expected;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xchg_seqcst(x as *mut StgWord8, val as StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xchg_seqcst(x as *mut StgWord16, val as StgWord16)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_xchg_seqcst(x as *mut StgWord32, val as StgWord32)
        as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return ::core::intrinsics::atomic_xchg_seqcst(x as *mut StgWord64, val);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread8(mut x: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_load_seqcst(x as *mut StgWord8) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread16(mut x: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_load_seqcst(x as *mut StgWord16) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread32(mut x: StgWord) -> StgWord {
    return ::core::intrinsics::atomic_load_seqcst(x as *mut StgWord32) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread64(mut x: StgWord) -> StgWord64 {
    return ::core::intrinsics::atomic_load_seqcst(x as *mut StgWord64);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite8(mut x: StgWord, mut val: StgWord) {
    ::core::intrinsics::atomic_store_seqcst(x as *mut StgWord8, val as StgWord8);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite16(mut x: StgWord, mut val: StgWord) {
    ::core::intrinsics::atomic_store_seqcst(x as *mut StgWord16, val as StgWord16);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite32(mut x: StgWord, mut val: StgWord) {
    ::core::intrinsics::atomic_store_seqcst(x as *mut StgWord32, val as StgWord32);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite64(mut x: StgWord, mut val: StgWord64) {
    ::core::intrinsics::atomic_store_seqcst(x as *mut StgWord64, val);
}
