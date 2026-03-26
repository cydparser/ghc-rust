use crate::ffi::stg::types::{StgWord, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).xadd(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).xadd(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).xadd(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_add64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).xadd(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).xsub(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).xsub(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).xsub(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_sub64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).xsub(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).and(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).and(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).and(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_and64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).and(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).nand(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).nand(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).nand(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_nand64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).nand(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).or(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).or(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).or(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_or64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).or(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).xor(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).xor(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).xor(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomic_xor64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).xor(val, Ordering::SeqCst);
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
    let fresh0 = (x as *mut StgWord8).cxchg(
        *&raw mut expected,
        new as StgWord8,
        Ordering::SeqCst,
        Ordering::SeqCst,
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
    let fresh1 = (x as *mut StgWord16).cxchg(
        *&raw mut expected,
        new as StgWord16,
        Ordering::SeqCst,
        Ordering::SeqCst,
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
    let fresh2 = (x as *mut StgWord32).cxchg(
        *&raw mut expected,
        new as StgWord32,
        Ordering::SeqCst,
        Ordering::SeqCst,
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
    let fresh3 =
        (x as *mut StgWord64).cxchg(*&raw mut expected, new, Ordering::SeqCst, Ordering::SeqCst);
    *&raw mut expected = fresh3.0;
    fresh3.1;

    return expected;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg8(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord8).xchg(val as StgWord8, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg16(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord16).xchg(val as StgWord16, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg32(mut x: StgWord, mut val: StgWord) -> StgWord {
    return (x as *mut StgWord32).xchg(val as StgWord32, Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_xchg64(mut x: StgWord, mut val: StgWord64) -> StgWord64 {
    return (x as *mut StgWord64).xchg(val, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread8(mut x: StgWord) -> StgWord {
    return (x as *mut StgWord8).load(Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread16(mut x: StgWord) -> StgWord {
    return (x as *mut StgWord16).load(Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread32(mut x: StgWord) -> StgWord {
    return (x as *mut StgWord32).load(Ordering::SeqCst) as StgWord;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicread64(mut x: StgWord) -> StgWord64 {
    return (x as *mut StgWord64).load(Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite8(mut x: StgWord, mut val: StgWord) {
    (x as *mut StgWord8).store(val as StgWord8, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite16(mut x: StgWord, mut val: StgWord) {
    (x as *mut StgWord16).store(val as StgWord16, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite32(mut x: StgWord, mut val: StgWord) {
    (x as *mut StgWord32).store(val as StgWord32, Ordering::SeqCst);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_atomicwrite64(mut x: StgWord, mut val: StgWord64) {
    (x as *mut StgWord64).store(val, Ordering::SeqCst);
}
