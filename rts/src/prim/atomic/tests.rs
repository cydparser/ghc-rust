use super::*;

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
fn equivalent_hs_atomic_add8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add8(x, val) } };
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
fn equivalent_hs_atomic_add16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add16(x, val) } };
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
fn equivalent_hs_atomic_add32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_add32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_add32(x, val) } };
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
fn equivalent_hs_atomic_add64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_add64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_add64(x, val) } };
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
fn equivalent_hs_atomic_sub8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub8(x, val) } };
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
fn equivalent_hs_atomic_sub16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub16(x, val) } };
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
fn equivalent_hs_atomic_sub32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_sub32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_sub32(x, val) } };
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
fn equivalent_hs_atomic_sub64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_sub64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_sub64(x, val) } };
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
fn equivalent_hs_atomic_and8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and8(x, val) } };
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
fn equivalent_hs_atomic_and16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and16(x, val) } };
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
fn equivalent_hs_atomic_and32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_and32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_and32(x, val) } };
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
fn equivalent_hs_atomic_and64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_and64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_and64(x, val) } };
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
fn equivalent_hs_atomic_nand8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand8(x, val) } };
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
fn equivalent_hs_atomic_nand16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand16(x, val) } };
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
fn equivalent_hs_atomic_nand32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_nand32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_nand32(x, val) } };
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
fn equivalent_hs_atomic_nand64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_nand64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_nand64(x, val) } };
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
fn equivalent_hs_atomic_or8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or8(x, val) } };
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
fn equivalent_hs_atomic_or16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or16(x, val) } };
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
fn equivalent_hs_atomic_or32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_or32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_or32(x, val) } };
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
fn equivalent_hs_atomic_or64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_or64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_or64(x, val) } };
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
fn equivalent_hs_atomic_xor8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor8(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor8(x, val) } };
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
fn equivalent_hs_atomic_xor16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor16(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor16(x, val) } };
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
fn equivalent_hs_atomic_xor32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomic_xor32(x, val) } };
    let actual: StgWord = { unsafe { hs_atomic_xor32(x, val) } };
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
fn equivalent_hs_atomic_xor64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomic_xor64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_atomic_xor64(x, val) } };
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
fn equivalent_hs_cmpxchg8(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg8(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg8(x, old, new_) } };
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
fn equivalent_hs_cmpxchg16(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg16(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg16(x, old, new_) } };
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
fn equivalent_hs_cmpxchg32(x: StgWord, old: StgWord, new_: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_cmpxchg32(x, old, new_) } };
    let actual: StgWord = { unsafe { hs_cmpxchg32(x, old, new_) } };
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
fn equivalent_hs_cmpxchg64(x: StgWord, old: StgWord64, new_: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_cmpxchg64(x, old, new_) } };
    let actual: StgWord64 = { unsafe { hs_cmpxchg64(x, old, new_) } };
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
fn equivalent_hs_xchg8(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg8(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg8(x, val) } };
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
fn equivalent_hs_xchg16(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg16(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg16(x, val) } };
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
fn equivalent_hs_xchg32(x: StgWord, val: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_xchg32(x, val) } };
    let actual: StgWord = { unsafe { hs_xchg32(x, val) } };
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
fn equivalent_hs_xchg64(x: StgWord, val: StgWord64) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_xchg64(x, val) } };
    let actual: StgWord64 = { unsafe { hs_xchg64(x, val) } };
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
fn equivalent_hs_atomicread8(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread8(x) } };
    let actual: StgWord = { unsafe { hs_atomicread8(x) } };
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
fn equivalent_hs_atomicread16(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread16(x) } };
    let actual: StgWord = { unsafe { hs_atomicread16(x) } };
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
fn equivalent_hs_atomicread32(x: StgWord) -> bool {
    let expected: StgWord = { unsafe { sys::hs_atomicread32(x) } };
    let actual: StgWord = { unsafe { hs_atomicread32(x) } };
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
fn equivalent_hs_atomicread64(x: StgWord) -> bool {
    let expected: StgWord64 = { unsafe { sys::hs_atomicread64(x) } };
    let actual: StgWord64 = { unsafe { hs_atomicread64(x) } };
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
