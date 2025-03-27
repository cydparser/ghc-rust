use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_add8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_add8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_add8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_add8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_add8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_add16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_add16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_add16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_add16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_add16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_add32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_add32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_add32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_add32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_add32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_add64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_add64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_add64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_add64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_add64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_sub8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_sub8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_sub8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_sub8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_sub8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_sub16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_sub16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_sub16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_sub16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_sub16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_sub32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_sub32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_sub32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_sub32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_sub32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_sub64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_sub64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_sub64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_sub64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_sub64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_and8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_and8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_and8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_and8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_and8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_and16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_and16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_and16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_and16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_and16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_and32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_and32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_and32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_and32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_and32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_and64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_and64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_and64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_and64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_and64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_nand8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_nand8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_nand8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_nand8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_nand8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_nand16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_nand16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_nand16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_nand16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_nand16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_nand32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_nand32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_nand32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_nand32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_nand32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_nand64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_nand64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_nand64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_nand64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_nand64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_or8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_or8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_or8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_or8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_or8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_or16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_or16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_or16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_or16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_or16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_or32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_or32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_or32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_or32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_or32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_or64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_or64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_or64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_or64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_or64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_xor8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_xor8(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_xor8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_xor8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_xor8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_xor16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_xor16(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_xor16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_xor16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_xor16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_xor32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_xor32(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_xor32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_xor32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_xor32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomic_xor64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_atomic_xor64(x.into(), val.into())) };
    let actual = unsafe { super::hs_atomic_xor64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomic_xor64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomic_xor64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_cmpxchg8(x.into(), old.into(), new_.into())) };
    let actual = unsafe { super::hs_cmpxchg8(x, old, new_) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_cmpxchg8() {
    let x = Default::default();
    let old = Default::default();
    let new_ = Default::default();
    unsafe { super::hs_cmpxchg8(x, old, new_) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_cmpxchg16(x.into(), old.into(), new_.into())) };
    let actual = unsafe { super::hs_cmpxchg16(x, old, new_) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_cmpxchg16() {
    let x = Default::default();
    let old = Default::default();
    let new_ = Default::default();
    unsafe { super::hs_cmpxchg16(x, old, new_) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_cmpxchg32(x.into(), old.into(), new_.into())) };
    let actual = unsafe { super::hs_cmpxchg32(x, old, new_) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_cmpxchg32() {
    let x = Default::default();
    let old = Default::default();
    let new_ = Default::default();
    unsafe { super::hs_cmpxchg32(x, old, new_) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_cmpxchg64(x.into(), old.into(), new_.into())) };
    let actual = unsafe { super::hs_cmpxchg64(x, old, new_) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_cmpxchg64() {
    let x = Default::default();
    let old = Default::default();
    let new_ = Default::default();
    unsafe { super::hs_cmpxchg64(x, old, new_) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomicread8(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomicread8(x.into())) };
    let actual = unsafe { super::hs_atomicread8(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomicread8() {
    let x = Default::default();
    unsafe { super::hs_atomicread8(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomicread16(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomicread16(x.into())) };
    let actual = unsafe { super::hs_atomicread16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomicread16() {
    let x = Default::default();
    unsafe { super::hs_atomicread16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomicread32(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomicread32(x.into())) };
    let actual = unsafe { super::hs_atomicread32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomicread32() {
    let x = Default::default();
    unsafe { super::hs_atomicread32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_atomicread64(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_atomicread64(x.into())) };
    let actual = unsafe { super::hs_atomicread64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_atomicread64() {
    let x = Default::default();
    unsafe { super::hs_atomicread64(x) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_atomicwrite8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomicwrite8(x, val) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_atomicwrite16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomicwrite16(x, val) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_atomicwrite32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomicwrite32(x, val) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_atomicwrite64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_atomicwrite64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_xchg8(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_xchg8(x.into(), val.into())) };
    let actual = unsafe { super::hs_xchg8(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_xchg8() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_xchg8(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_xchg16(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_xchg16(x.into(), val.into())) };
    let actual = unsafe { super::hs_xchg16(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_xchg16() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_xchg16(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_xchg32(x: StgWord, val: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_xchg32(x.into(), val.into())) };
    let actual = unsafe { super::hs_xchg32(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_xchg32() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_xchg32(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_xchg64(x: StgWord, val: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_xchg64(x.into(), val.into())) };
    let actual = unsafe { super::hs_xchg64(x, val) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_xchg64() {
    let x = Default::default();
    let val = Default::default();
    unsafe { super::hs_xchg64(x, val) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bswap16(x: StgWord16) -> bool {
    let expected = unsafe { transmute(sys::hs_bswap16(x.into())) };
    let actual = unsafe { super::hs_bswap16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bswap16() {
    let x = Default::default();
    unsafe { super::hs_bswap16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bswap32(x: StgWord32) -> bool {
    let expected = unsafe { transmute(sys::hs_bswap32(x.into())) };
    let actual = unsafe { super::hs_bswap32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bswap32() {
    let x = Default::default();
    unsafe { super::hs_bswap32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bswap64(x: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_bswap64(x.into())) };
    let actual = unsafe { super::hs_bswap64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bswap64() {
    let x = Default::default();
    unsafe { super::hs_bswap64(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bitrev8(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_bitrev8(x.into())) };
    let actual = unsafe { super::hs_bitrev8(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bitrev8() {
    let x = Default::default();
    unsafe { super::hs_bitrev8(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bitrev16(x: StgWord16) -> bool {
    let expected = unsafe { transmute(sys::hs_bitrev16(x.into())) };
    let actual = unsafe { super::hs_bitrev16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bitrev16() {
    let x = Default::default();
    unsafe { super::hs_bitrev16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bitrev32(x: StgWord32) -> bool {
    let expected = unsafe { transmute(sys::hs_bitrev32(x.into())) };
    let actual = unsafe { super::hs_bitrev32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bitrev32() {
    let x = Default::default();
    unsafe { super::hs_bitrev32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_bitrev64(x: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_bitrev64(x.into())) };
    let actual = unsafe { super::hs_bitrev64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_bitrev64() {
    let x = Default::default();
    unsafe { super::hs_bitrev64(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pdep64(src: StgWord64, mask: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_pdep64(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pdep64(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pdep64() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pdep64(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pdep32(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pdep32(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pdep32(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pdep32() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pdep32(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pdep16(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pdep16(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pdep16(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pdep16() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pdep16(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pdep8(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pdep8(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pdep8(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pdep8() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pdep8(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pext64(src: StgWord64, mask: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_pext64(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pext64(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pext64() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pext64(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pext32(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pext32(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pext32(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pext32() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pext32(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pext16(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pext16(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pext16(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pext16() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pext16(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_pext8(src: StgWord, mask: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_pext8(src.into(), mask.into())) };
    let actual = unsafe { super::hs_pext8(src, mask) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_pext8() {
    let src = Default::default();
    let mask = Default::default();
    unsafe { super::hs_pext8(src, mask) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_popcnt8(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_popcnt8(x.into())) };
    let actual = unsafe { super::hs_popcnt8(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_popcnt8() {
    let x = Default::default();
    unsafe { super::hs_popcnt8(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_popcnt16(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_popcnt16(x.into())) };
    let actual = unsafe { super::hs_popcnt16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_popcnt16() {
    let x = Default::default();
    unsafe { super::hs_popcnt16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_popcnt32(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_popcnt32(x.into())) };
    let actual = unsafe { super::hs_popcnt32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_popcnt32() {
    let x = Default::default();
    unsafe { super::hs_popcnt32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_popcnt64(x: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_popcnt64(x.into())) };
    let actual = unsafe { super::hs_popcnt64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_popcnt64() {
    let x = Default::default();
    unsafe { super::hs_popcnt64(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_popcnt(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_popcnt(x.into())) };
    let actual = unsafe { super::hs_popcnt(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_popcnt() {
    let x = Default::default();
    unsafe { super::hs_popcnt(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_word2float32(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_word2float32(x.into())) };
    let actual = unsafe { super::hs_word2float32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_word2float32() {
    let x = Default::default();
    unsafe { super::hs_word2float32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_word2float64(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_word2float64(x.into())) };
    let actual = unsafe { super::hs_word2float64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_word2float64() {
    let x = Default::default();
    unsafe { super::hs_word2float64(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_clz8(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_clz8(x.into())) };
    let actual = unsafe { super::hs_clz8(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_clz8() {
    let x = Default::default();
    unsafe { super::hs_clz8(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_clz16(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_clz16(x.into())) };
    let actual = unsafe { super::hs_clz16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_clz16() {
    let x = Default::default();
    unsafe { super::hs_clz16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_clz32(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_clz32(x.into())) };
    let actual = unsafe { super::hs_clz32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_clz32() {
    let x = Default::default();
    unsafe { super::hs_clz32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_clz64(x: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_clz64(x.into())) };
    let actual = unsafe { super::hs_clz64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_clz64() {
    let x = Default::default();
    unsafe { super::hs_clz64(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_ctz8(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_ctz8(x.into())) };
    let actual = unsafe { super::hs_ctz8(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_ctz8() {
    let x = Default::default();
    unsafe { super::hs_ctz8(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_ctz16(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_ctz16(x.into())) };
    let actual = unsafe { super::hs_ctz16(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_ctz16() {
    let x = Default::default();
    unsafe { super::hs_ctz16(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_ctz32(x: StgWord) -> bool {
    let expected = unsafe { transmute(sys::hs_ctz32(x.into())) };
    let actual = unsafe { super::hs_ctz32(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_ctz32() {
    let x = Default::default();
    unsafe { super::hs_ctz32(x) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_hs_ctz64(x: StgWord64) -> bool {
    let expected = unsafe { transmute(sys::hs_ctz64(x.into())) };
    let actual = unsafe { super::hs_ctz64(x) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_ctz64() {
    let x = Default::default();
    unsafe { super::hs_ctz64(x) };
    todo!("assert")
}
