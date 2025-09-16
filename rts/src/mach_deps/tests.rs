use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSINT() {
    assert_eq!(sys::SIZEOF_HSINT, SIZEOF_HSINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSINT() {
    assert_eq!(sys::ALIGNMENT_HSINT, ALIGNMENT_HSINT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSWORD() {
    assert_eq!(sys::SIZEOF_HSWORD, SIZEOF_HSWORD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSWORD() {
    assert_eq!(sys::ALIGNMENT_HSWORD, ALIGNMENT_HSWORD);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSDOUBLE() {
    assert_eq!(sys::SIZEOF_HSDOUBLE, SIZEOF_HSDOUBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSDOUBLE() {
    assert_eq!(sys::ALIGNMENT_HSDOUBLE, ALIGNMENT_HSDOUBLE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSFLOAT() {
    assert_eq!(sys::SIZEOF_HSFLOAT, SIZEOF_HSFLOAT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSFLOAT() {
    assert_eq!(sys::ALIGNMENT_HSFLOAT, ALIGNMENT_HSFLOAT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSPTR() {
    assert_eq!(sys::SIZEOF_HSPTR, SIZEOF_HSPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSPTR() {
    assert_eq!(sys::ALIGNMENT_HSPTR, ALIGNMENT_HSPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSFUNPTR() {
    assert_eq!(sys::SIZEOF_HSFUNPTR, SIZEOF_HSFUNPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSFUNPTR() {
    assert_eq!(sys::ALIGNMENT_HSFUNPTR, ALIGNMENT_HSFUNPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_HSSTABLEPTR() {
    assert_eq!(sys::SIZEOF_HSSTABLEPTR, SIZEOF_HSSTABLEPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_HSSTABLEPTR() {
    assert_eq!(sys::ALIGNMENT_HSSTABLEPTR, ALIGNMENT_HSSTABLEPTR);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_INT8() {
    assert_eq!(sys::SIZEOF_INT8, SIZEOF_INT8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_INT8() {
    assert_eq!(sys::ALIGNMENT_INT8, ALIGNMENT_INT8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_WORD8() {
    assert_eq!(sys::SIZEOF_WORD8, SIZEOF_WORD8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_WORD8() {
    assert_eq!(sys::ALIGNMENT_WORD8, ALIGNMENT_WORD8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_INT16() {
    assert_eq!(sys::SIZEOF_INT16, SIZEOF_INT16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_INT16() {
    assert_eq!(sys::ALIGNMENT_INT16, ALIGNMENT_INT16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_WORD16() {
    assert_eq!(sys::SIZEOF_WORD16, SIZEOF_WORD16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_WORD16() {
    assert_eq!(sys::ALIGNMENT_WORD16, ALIGNMENT_WORD16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_INT32() {
    assert_eq!(sys::SIZEOF_INT32, SIZEOF_INT32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_INT32() {
    assert_eq!(sys::ALIGNMENT_INT32, ALIGNMENT_INT32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_WORD32() {
    assert_eq!(sys::SIZEOF_WORD32, SIZEOF_WORD32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_WORD32() {
    assert_eq!(sys::ALIGNMENT_WORD32, ALIGNMENT_WORD32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_INT64() {
    assert_eq!(sys::SIZEOF_INT64, SIZEOF_INT64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_INT64() {
    assert_eq!(sys::ALIGNMENT_INT64, ALIGNMENT_INT64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_SIZEOF_WORD64() {
    assert_eq!(sys::SIZEOF_WORD64, SIZEOF_WORD64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_ALIGNMENT_WORD64() {
    assert_eq!(sys::ALIGNMENT_WORD64, ALIGNMENT_WORD64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WORD_SIZE_IN_BITS() {
    assert_eq!(sys::WORD_SIZE_IN_BITS, WORD_SIZE_IN_BITS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_WORD_SIZE_IN_BITS_FLOAT() {
    assert_eq!(sys::WORD_SIZE_IN_BITS_FLOAT, WORD_SIZE_IN_BITS_FLOAT);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TAG_BITS() {
    assert_eq!(sys::TAG_BITS, TAG_BITS);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TAG_MASK() {
    assert_eq!(sys::TAG_MASK, TAG_MASK);
}
