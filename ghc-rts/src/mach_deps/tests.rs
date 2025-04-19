#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSINT() {
    assert_eq!(sys::SIZEOF_HSINT, super::SIZEOF_HSINT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSINT() {
    assert_eq!(sys::ALIGNMENT_HSINT, super::ALIGNMENT_HSINT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSWORD() {
    assert_eq!(sys::SIZEOF_HSWORD, super::SIZEOF_HSWORD.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSWORD() {
    assert_eq!(sys::ALIGNMENT_HSWORD, super::ALIGNMENT_HSWORD.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSDOUBLE() {
    assert_eq!(sys::SIZEOF_HSDOUBLE, super::SIZEOF_HSDOUBLE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSDOUBLE() {
    assert_eq!(sys::ALIGNMENT_HSDOUBLE, super::ALIGNMENT_HSDOUBLE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSFLOAT() {
    assert_eq!(sys::SIZEOF_HSFLOAT, super::SIZEOF_HSFLOAT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSFLOAT() {
    assert_eq!(sys::ALIGNMENT_HSFLOAT, super::ALIGNMENT_HSFLOAT.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSPTR() {
    assert_eq!(sys::SIZEOF_HSPTR, super::SIZEOF_HSPTR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSPTR() {
    assert_eq!(sys::ALIGNMENT_HSPTR, super::ALIGNMENT_HSPTR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSFUNPTR() {
    assert_eq!(sys::SIZEOF_HSFUNPTR, super::SIZEOF_HSFUNPTR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSFUNPTR() {
    assert_eq!(sys::ALIGNMENT_HSFUNPTR, super::ALIGNMENT_HSFUNPTR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_HSSTABLEPTR() {
    assert_eq!(sys::SIZEOF_HSSTABLEPTR, super::SIZEOF_HSSTABLEPTR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_HSSTABLEPTR() {
    assert_eq!(
        sys::ALIGNMENT_HSSTABLEPTR,
        super::ALIGNMENT_HSSTABLEPTR.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_INT8() {
    assert_eq!(sys::SIZEOF_INT8, super::SIZEOF_INT8.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_INT8() {
    assert_eq!(sys::ALIGNMENT_INT8, super::ALIGNMENT_INT8.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_WORD8() {
    assert_eq!(sys::SIZEOF_WORD8, super::SIZEOF_WORD8.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_WORD8() {
    assert_eq!(sys::ALIGNMENT_WORD8, super::ALIGNMENT_WORD8.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_INT16() {
    assert_eq!(sys::SIZEOF_INT16, super::SIZEOF_INT16.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_INT16() {
    assert_eq!(sys::ALIGNMENT_INT16, super::ALIGNMENT_INT16.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_WORD16() {
    assert_eq!(sys::SIZEOF_WORD16, super::SIZEOF_WORD16.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_WORD16() {
    assert_eq!(sys::ALIGNMENT_WORD16, super::ALIGNMENT_WORD16.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_INT32() {
    assert_eq!(sys::SIZEOF_INT32, super::SIZEOF_INT32.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_INT32() {
    assert_eq!(sys::ALIGNMENT_INT32, super::ALIGNMENT_INT32.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_WORD32() {
    assert_eq!(sys::SIZEOF_WORD32, super::SIZEOF_WORD32.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_WORD32() {
    assert_eq!(sys::ALIGNMENT_WORD32, super::ALIGNMENT_WORD32.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_INT64() {
    assert_eq!(sys::SIZEOF_INT64, super::SIZEOF_INT64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_INT64() {
    assert_eq!(sys::ALIGNMENT_INT64, super::ALIGNMENT_INT64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_SIZEOF_WORD64() {
    assert_eq!(sys::SIZEOF_WORD64, super::SIZEOF_WORD64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ALIGNMENT_WORD64() {
    assert_eq!(sys::ALIGNMENT_WORD64, super::ALIGNMENT_WORD64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_WORD_SIZE_IN_BITS() {
    assert_eq!(sys::WORD_SIZE_IN_BITS, super::WORD_SIZE_IN_BITS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_WORD_SIZE_IN_BITS_FLOAT() {
    assert_eq!(
        sys::WORD_SIZE_IN_BITS_FLOAT,
        super::WORD_SIZE_IN_BITS_FLOAT.into()
    );
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TAG_BITS() {
    assert_eq!(sys::TAG_BITS, super::TAG_BITS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_TAG_MASK() {
    assert_eq!(sys::TAG_MASK, super::TAG_MASK.into());
}
