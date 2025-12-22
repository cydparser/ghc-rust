#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSINT_eq() {
    assert_eq!(SIZEOF_HSINT, sys::SIZEOF_HSINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSINT_layout() {
    assert_eq!(size_of_val(&SIZEOF_HSINT), size_of_val(&sys::SIZEOF_HSINT));
    assert_eq!(
        align_of_val(&SIZEOF_HSINT),
        align_of_val(&sys::SIZEOF_HSINT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSINT_eq() {
    assert_eq!(ALIGNMENT_HSINT, sys::ALIGNMENT_HSINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSINT_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSINT),
        size_of_val(&sys::ALIGNMENT_HSINT)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSINT),
        align_of_val(&sys::ALIGNMENT_HSINT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSWORD_eq() {
    assert_eq!(SIZEOF_HSWORD, sys::SIZEOF_HSWORD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSWORD_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_HSWORD),
        size_of_val(&sys::SIZEOF_HSWORD)
    );
    assert_eq!(
        align_of_val(&SIZEOF_HSWORD),
        align_of_val(&sys::SIZEOF_HSWORD)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSWORD_eq() {
    assert_eq!(ALIGNMENT_HSWORD, sys::ALIGNMENT_HSWORD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSWORD_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSWORD),
        size_of_val(&sys::ALIGNMENT_HSWORD)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSWORD),
        align_of_val(&sys::ALIGNMENT_HSWORD)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSDOUBLE_eq() {
    assert_eq!(SIZEOF_HSDOUBLE, sys::SIZEOF_HSDOUBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSDOUBLE_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_HSDOUBLE),
        size_of_val(&sys::SIZEOF_HSDOUBLE)
    );
    assert_eq!(
        align_of_val(&SIZEOF_HSDOUBLE),
        align_of_val(&sys::SIZEOF_HSDOUBLE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSDOUBLE_eq() {
    assert_eq!(ALIGNMENT_HSDOUBLE, sys::ALIGNMENT_HSDOUBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSDOUBLE_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSDOUBLE),
        size_of_val(&sys::ALIGNMENT_HSDOUBLE)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSDOUBLE),
        align_of_val(&sys::ALIGNMENT_HSDOUBLE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSFLOAT_eq() {
    assert_eq!(SIZEOF_HSFLOAT, sys::SIZEOF_HSFLOAT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSFLOAT_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_HSFLOAT),
        size_of_val(&sys::SIZEOF_HSFLOAT)
    );
    assert_eq!(
        align_of_val(&SIZEOF_HSFLOAT),
        align_of_val(&sys::SIZEOF_HSFLOAT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSFLOAT_eq() {
    assert_eq!(ALIGNMENT_HSFLOAT, sys::ALIGNMENT_HSFLOAT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSFLOAT_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSFLOAT),
        size_of_val(&sys::ALIGNMENT_HSFLOAT)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSFLOAT),
        align_of_val(&sys::ALIGNMENT_HSFLOAT)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSPTR_eq() {
    assert_eq!(SIZEOF_HSPTR, sys::SIZEOF_HSPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSPTR_layout() {
    assert_eq!(size_of_val(&SIZEOF_HSPTR), size_of_val(&sys::SIZEOF_HSPTR));
    assert_eq!(
        align_of_val(&SIZEOF_HSPTR),
        align_of_val(&sys::SIZEOF_HSPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSPTR_eq() {
    assert_eq!(ALIGNMENT_HSPTR, sys::ALIGNMENT_HSPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSPTR_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSPTR),
        size_of_val(&sys::ALIGNMENT_HSPTR)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSPTR),
        align_of_val(&sys::ALIGNMENT_HSPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSFUNPTR_eq() {
    assert_eq!(SIZEOF_HSFUNPTR, sys::SIZEOF_HSFUNPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSFUNPTR_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_HSFUNPTR),
        size_of_val(&sys::SIZEOF_HSFUNPTR)
    );
    assert_eq!(
        align_of_val(&SIZEOF_HSFUNPTR),
        align_of_val(&sys::SIZEOF_HSFUNPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSFUNPTR_eq() {
    assert_eq!(ALIGNMENT_HSFUNPTR, sys::ALIGNMENT_HSFUNPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSFUNPTR_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSFUNPTR),
        size_of_val(&sys::ALIGNMENT_HSFUNPTR)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSFUNPTR),
        align_of_val(&sys::ALIGNMENT_HSFUNPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSSTABLEPTR_eq() {
    assert_eq!(SIZEOF_HSSTABLEPTR, sys::SIZEOF_HSSTABLEPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_HSSTABLEPTR_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_HSSTABLEPTR),
        size_of_val(&sys::SIZEOF_HSSTABLEPTR)
    );
    assert_eq!(
        align_of_val(&SIZEOF_HSSTABLEPTR),
        align_of_val(&sys::SIZEOF_HSSTABLEPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSSTABLEPTR_eq() {
    assert_eq!(ALIGNMENT_HSSTABLEPTR, sys::ALIGNMENT_HSSTABLEPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_HSSTABLEPTR_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_HSSTABLEPTR),
        size_of_val(&sys::ALIGNMENT_HSSTABLEPTR)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_HSSTABLEPTR),
        align_of_val(&sys::ALIGNMENT_HSSTABLEPTR)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT8_eq() {
    assert_eq!(SIZEOF_INT8, sys::SIZEOF_INT8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT8_layout() {
    assert_eq!(size_of_val(&SIZEOF_INT8), size_of_val(&sys::SIZEOF_INT8));
    assert_eq!(align_of_val(&SIZEOF_INT8), align_of_val(&sys::SIZEOF_INT8));
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT8_eq() {
    assert_eq!(ALIGNMENT_INT8, sys::ALIGNMENT_INT8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT8_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_INT8),
        size_of_val(&sys::ALIGNMENT_INT8)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_INT8),
        align_of_val(&sys::ALIGNMENT_INT8)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD8_eq() {
    assert_eq!(SIZEOF_WORD8, sys::SIZEOF_WORD8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD8_layout() {
    assert_eq!(size_of_val(&SIZEOF_WORD8), size_of_val(&sys::SIZEOF_WORD8));
    assert_eq!(
        align_of_val(&SIZEOF_WORD8),
        align_of_val(&sys::SIZEOF_WORD8)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD8_eq() {
    assert_eq!(ALIGNMENT_WORD8, sys::ALIGNMENT_WORD8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD8_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_WORD8),
        size_of_val(&sys::ALIGNMENT_WORD8)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_WORD8),
        align_of_val(&sys::ALIGNMENT_WORD8)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT16_eq() {
    assert_eq!(SIZEOF_INT16, sys::SIZEOF_INT16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT16_layout() {
    assert_eq!(size_of_val(&SIZEOF_INT16), size_of_val(&sys::SIZEOF_INT16));
    assert_eq!(
        align_of_val(&SIZEOF_INT16),
        align_of_val(&sys::SIZEOF_INT16)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT16_eq() {
    assert_eq!(ALIGNMENT_INT16, sys::ALIGNMENT_INT16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT16_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_INT16),
        size_of_val(&sys::ALIGNMENT_INT16)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_INT16),
        align_of_val(&sys::ALIGNMENT_INT16)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD16_eq() {
    assert_eq!(SIZEOF_WORD16, sys::SIZEOF_WORD16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD16_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_WORD16),
        size_of_val(&sys::SIZEOF_WORD16)
    );
    assert_eq!(
        align_of_val(&SIZEOF_WORD16),
        align_of_val(&sys::SIZEOF_WORD16)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD16_eq() {
    assert_eq!(ALIGNMENT_WORD16, sys::ALIGNMENT_WORD16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD16_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_WORD16),
        size_of_val(&sys::ALIGNMENT_WORD16)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_WORD16),
        align_of_val(&sys::ALIGNMENT_WORD16)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT32_eq() {
    assert_eq!(SIZEOF_INT32, sys::SIZEOF_INT32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT32_layout() {
    assert_eq!(size_of_val(&SIZEOF_INT32), size_of_val(&sys::SIZEOF_INT32));
    assert_eq!(
        align_of_val(&SIZEOF_INT32),
        align_of_val(&sys::SIZEOF_INT32)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT32_eq() {
    assert_eq!(ALIGNMENT_INT32, sys::ALIGNMENT_INT32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT32_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_INT32),
        size_of_val(&sys::ALIGNMENT_INT32)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_INT32),
        align_of_val(&sys::ALIGNMENT_INT32)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD32_eq() {
    assert_eq!(SIZEOF_WORD32, sys::SIZEOF_WORD32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD32_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_WORD32),
        size_of_val(&sys::SIZEOF_WORD32)
    );
    assert_eq!(
        align_of_val(&SIZEOF_WORD32),
        align_of_val(&sys::SIZEOF_WORD32)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD32_eq() {
    assert_eq!(ALIGNMENT_WORD32, sys::ALIGNMENT_WORD32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD32_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_WORD32),
        size_of_val(&sys::ALIGNMENT_WORD32)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_WORD32),
        align_of_val(&sys::ALIGNMENT_WORD32)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT64_eq() {
    assert_eq!(SIZEOF_INT64, sys::SIZEOF_INT64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_INT64_layout() {
    assert_eq!(size_of_val(&SIZEOF_INT64), size_of_val(&sys::SIZEOF_INT64));
    assert_eq!(
        align_of_val(&SIZEOF_INT64),
        align_of_val(&sys::SIZEOF_INT64)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT64_eq() {
    assert_eq!(ALIGNMENT_INT64, sys::ALIGNMENT_INT64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_INT64_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_INT64),
        size_of_val(&sys::ALIGNMENT_INT64)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_INT64),
        align_of_val(&sys::ALIGNMENT_INT64)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD64_eq() {
    assert_eq!(SIZEOF_WORD64, sys::SIZEOF_WORD64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_SIZEOF_WORD64_layout() {
    assert_eq!(
        size_of_val(&SIZEOF_WORD64),
        size_of_val(&sys::SIZEOF_WORD64)
    );
    assert_eq!(
        align_of_val(&SIZEOF_WORD64),
        align_of_val(&sys::SIZEOF_WORD64)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD64_eq() {
    assert_eq!(ALIGNMENT_WORD64, sys::ALIGNMENT_WORD64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_ALIGNMENT_WORD64_layout() {
    assert_eq!(
        size_of_val(&ALIGNMENT_WORD64),
        size_of_val(&sys::ALIGNMENT_WORD64)
    );
    assert_eq!(
        align_of_val(&ALIGNMENT_WORD64),
        align_of_val(&sys::ALIGNMENT_WORD64)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_WORD_SIZE_IN_BITS_eq() {
    assert_eq!(WORD_SIZE_IN_BITS, sys::WORD_SIZE_IN_BITS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_WORD_SIZE_IN_BITS_layout() {
    assert_eq!(
        size_of_val(&WORD_SIZE_IN_BITS),
        size_of_val(&sys::WORD_SIZE_IN_BITS)
    );
    assert_eq!(
        align_of_val(&WORD_SIZE_IN_BITS),
        align_of_val(&sys::WORD_SIZE_IN_BITS)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_TAG_BITS_eq() {
    assert_eq!(TAG_BITS, sys::TAG_BITS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_TAG_BITS_layout() {
    assert_eq!(size_of_val(&TAG_BITS), size_of_val(&sys::TAG_BITS));
    assert_eq!(align_of_val(&TAG_BITS), align_of_val(&sys::TAG_BITS));
}
