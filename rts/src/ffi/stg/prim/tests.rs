use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_add8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_add8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_add8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_add16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_add16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_add16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_add32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_add32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_add32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_add64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_add64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_add64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_add64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_add64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_sub8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_sub8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_sub8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_sub16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_sub16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_sub16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_sub32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_sub32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_sub32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_sub64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_sub64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_sub64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_sub64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_sub64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_and8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_and8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_and8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_and16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_and16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_and16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_and32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_and32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_and32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_and64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_and64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_and64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_and64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_and64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_nand8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_nand8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_nand8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_nand16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_nand16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_nand16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_nand32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_nand32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_nand32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_nand64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_nand64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_nand64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_nand64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_nand64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_or8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_or8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_or8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_or16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_or16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_or16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_or32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_or32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_or32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_or64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_or64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_or64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_or64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_or64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_xor8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_xor8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_xor8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_xor16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_xor16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_xor16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_xor32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_xor32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_xor32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomic_xor64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_xor64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_xor64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomic_xor64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomic_xor64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg8(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg8(x, old, new_) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_cmpxchg8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let old: StgWord = Arbitrary::arbitrary(g);
        let new_: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_cmpxchg8(x, old, new_) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg16(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg16(x, old, new_) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_cmpxchg16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let old: StgWord = Arbitrary::arbitrary(g);
        let new_: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_cmpxchg16(x, old, new_) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg32(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg32(x, old, new_) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_cmpxchg32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let old: StgWord = Arbitrary::arbitrary(g);
        let new_: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_cmpxchg32(x, old, new_) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_cmpxchg64(x, old, new_) } };
    let actual: StgWord64 = { unsafe { hs_cmpxchg64(x, old, new_) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_cmpxchg64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let old: StgWord64 = Arbitrary::arbitrary(g);
        let new_: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_cmpxchg64(x, old, new_) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomicread8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread8(x) } };
    let actual: StgWord = { unsafe { hs_atomicread8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicread8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicread8(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomicread16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread16(x) } };
    let actual: StgWord = { unsafe { hs_atomicread16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicread16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicread16(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomicread32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread32(x) } };
    let actual: StgWord = { unsafe { hs_atomicread32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicread32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicread32(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_atomicread64(x: StgWord) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomicread64(x) } };
    let actual: StgWord64 = { unsafe { hs_atomicread64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicread64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicread64(x) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_atomicwrite8(x: StgWord, val: StgWord) -> bool {
    let expected = {
        unsafe { sys::hs_atomicwrite8(x, val) };
        todo!()
    };
    let actual = {
        unsafe { hs_atomicwrite8(x, val) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicwrite8() {
    let g = &mut Gen::new(100);
    let actual = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicwrite8(x, val) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_atomicwrite16(x: StgWord, val: StgWord) -> bool {
    let expected = {
        unsafe { sys::hs_atomicwrite16(x, val) };
        todo!()
    };
    let actual = {
        unsafe { hs_atomicwrite16(x, val) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicwrite16() {
    let g = &mut Gen::new(100);
    let actual = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicwrite16(x, val) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_atomicwrite32(x: StgWord, val: StgWord) -> bool {
    let expected = {
        unsafe { sys::hs_atomicwrite32(x, val) };
        todo!()
    };
    let actual = {
        unsafe { hs_atomicwrite32(x, val) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicwrite32() {
    let g = &mut Gen::new(100);
    let actual = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_atomicwrite32(x, val) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_hs_atomicwrite64(x: StgWord, val: StgWord64) -> bool {
    let expected = {
        unsafe { sys::hs_atomicwrite64(x, val) };
        todo!()
    };
    let actual = {
        unsafe { hs_atomicwrite64(x, val) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_atomicwrite64() {
    let g = &mut Gen::new(100);
    let actual = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_atomicwrite64(x, val) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_xchg8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg8(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg8(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_xchg8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_xchg8(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_xchg16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg16(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg16(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_xchg16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_xchg16(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_xchg32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg32(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg32(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_xchg32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_xchg32(x, val) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_xchg64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_xchg64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_xchg64(x, val) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_xchg64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord = Arbitrary::arbitrary(g);
        let val: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_xchg64(x, val) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bswap16(x: StgWord16) -> bool {
    let expected: StgWord16 = { unsafe { sys::hs_bswap16(x) } };
    let actual: StgWord16 = { unsafe { hs_bswap16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bswap16() {
    let g = &mut Gen::new(100);
    let actual: StgWord16 = {
        let x: StgWord16 = Arbitrary::arbitrary(g);
        unsafe { hs_bswap16(x) }
    };
    let expected: StgWord16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bswap32(x: StgWord32) -> bool {
    let expected: StgWord32 = { unsafe { sys::hs_bswap32(x) } };
    let actual: StgWord32 = { unsafe { hs_bswap32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bswap32() {
    let g = &mut Gen::new(100);
    let actual: StgWord32 = {
        let x: StgWord32 = Arbitrary::arbitrary(g);
        unsafe { hs_bswap32(x) }
    };
    let expected: StgWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bswap64(x: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_bswap64(x) } };
    let actual: StgWord64 = { unsafe { hs_bswap64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bswap64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_bswap64(x) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_bitrev8(x) } };
    let actual: StgWord = { unsafe { hs_bitrev8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev8(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev16(x: StgWord16) -> bool {
    let expected: StgWord16 = { unsafe { sys::hs_bitrev16(x) } };
    let actual: StgWord16 = { unsafe { hs_bitrev16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev16() {
    let g = &mut Gen::new(100);
    let actual: StgWord16 = {
        let x: StgWord16 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev16(x) }
    };
    let expected: StgWord16 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev32(x: StgWord32) -> bool {
    let expected: StgWord32 = { unsafe { sys::hs_bitrev32(x) } };
    let actual: StgWord32 = { unsafe { hs_bitrev32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev32() {
    let g = &mut Gen::new(100);
    let actual: StgWord32 = {
        let x: StgWord32 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev32(x) }
    };
    let expected: StgWord32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_bitrev64(x: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_bitrev64(x) } };
    let actual: StgWord64 = { unsafe { hs_bitrev64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_bitrev64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_bitrev64(x) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep64(src: StgWord64, mask: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_pdep64(src, mask) } };
    let actual: StgWord64 = { unsafe { hs_pdep64(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let src: StgWord64 = Arbitrary::arbitrary(g);
        let mask: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_pdep64(src, mask) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep32(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep32(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep32(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep32(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep16(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep16(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep16(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep16(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pdep8(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pdep8(src, mask) } };
    let actual: StgWord = { unsafe { hs_pdep8(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pdep8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pdep8(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext64(src: StgWord64, mask: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_pext64(src, mask) } };
    let actual: StgWord64 = { unsafe { hs_pext64(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext64() {
    let g = &mut Gen::new(100);
    let actual: StgWord64 = {
        let src: StgWord64 = Arbitrary::arbitrary(g);
        let mask: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_pext64(src, mask) }
    };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext32(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext32(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext32(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext32(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext16(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext16(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext16(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext16(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_pext8(src: StgWord, mask: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_pext8(src, mask) } };
    let actual: StgWord = { unsafe { hs_pext8(src, mask) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_pext8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let src: StgWord = Arbitrary::arbitrary(g);
        let mask: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_pext8(src, mask) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt8(x) } };
    let actual: StgWord = { unsafe { hs_popcnt8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt8(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt16(x) } };
    let actual: StgWord = { unsafe { hs_popcnt16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt16(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt32(x) } };
    let actual: StgWord = { unsafe { hs_popcnt32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt32(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt64(x: StgWord64) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt64(x) } };
    let actual: StgWord = { unsafe { hs_popcnt64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt64() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt64(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_popcnt(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_popcnt(x) } };
    let actual: StgWord = { unsafe { hs_popcnt(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_popcnt() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_popcnt(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_word2float32(x: StgWord) -> bool {
    let expected: StgFloat = { unsafe { sys::hs_word2float32(x) } };
    let actual: StgFloat = { unsafe { hs_word2float32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_word2float32() {
    let g = &mut Gen::new(100);
    let actual: StgFloat = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_word2float32(x) }
    };
    let expected: StgFloat = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_word2float64(x: StgWord) -> bool {
    let expected: StgDouble = { unsafe { sys::hs_word2float64(x) } };
    let actual: StgDouble = { unsafe { hs_word2float64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_word2float64() {
    let g = &mut Gen::new(100);
    let actual: StgDouble = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_word2float64(x) }
    };
    let expected: StgDouble = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz8(x) } };
    let actual: StgWord = { unsafe { hs_clz8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz8(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz16(x) } };
    let actual: StgWord = { unsafe { hs_clz16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz16(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz32(x) } };
    let actual: StgWord = { unsafe { hs_clz32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_clz32(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_clz64(x: StgWord64) -> bool {
    let expected: StgWord = { unsafe { sys::hs_clz64(x) } };
    let actual: StgWord = { unsafe { hs_clz64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_clz64() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_clz64(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_ctz8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_ctz8(x) } };
    let actual: StgWord = { unsafe { hs_ctz8(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_ctz8() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_ctz8(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_ctz16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_ctz16(x) } };
    let actual: StgWord = { unsafe { hs_ctz16(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_ctz16() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_ctz16(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_ctz32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_ctz32(x) } };
    let actual: StgWord = { unsafe { hs_ctz32(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_ctz32() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord = Arbitrary::arbitrary(g);
        unsafe { hs_ctz32(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_ctz64(x: StgWord64) -> bool {
    let expected: StgWord = { unsafe { sys::hs_ctz64(x) } };
    let actual: StgWord = { unsafe { hs_ctz64(x) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_ctz64() {
    let g = &mut Gen::new(100);
    let actual: StgWord = {
        let x: StgWord64 = Arbitrary::arbitrary(g);
        unsafe { hs_ctz64(x) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_mulIntMayOflo(a: W_, b: W_) -> bool {
    let expected: W_ = { unsafe { sys::hs_mulIntMayOflo(a, b) } };
    let actual: W_ = { unsafe { hs_mulIntMayOflo(a, b) } };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_hs_mulIntMayOflo() {
    let g = &mut Gen::new(100);
    let actual: W_ = {
        let a: W_ = Arbitrary::arbitrary(g);
        let b: W_ = Arbitrary::arbitrary(g);
        unsafe { hs_mulIntMayOflo(a, b) }
    };
    let expected: W_ = todo!();
    assert_eq!(expected, actual);
}
