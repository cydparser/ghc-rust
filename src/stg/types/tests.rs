use std::mem::size_of;

use super::{StgWord128, StgWord256, StgWord512};

#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT8_MIN() {
    assert_eq!(sys::STG_INT8_MIN, super::STG_INT8_MIN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT8_MAX() {
    assert_eq!(sys::STG_INT8_MAX, super::STG_INT8_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_WORD8_MAX() {
    assert_eq!(sys::STG_WORD8_MAX, super::STG_WORD8_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Word8() {
    assert_eq!(sys::FMT_Word8, super::FMT_Word8);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexWord8() {
    assert_eq!(sys::FMT_HexWord8, super::FMT_HexWord8);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT16_MIN() {
    assert_eq!(sys::STG_INT16_MIN, super::STG_INT16_MIN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT16_MAX() {
    assert_eq!(sys::STG_INT16_MAX, super::STG_INT16_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_WORD16_MAX() {
    assert_eq!(sys::STG_WORD16_MAX, super::STG_WORD16_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Word16() {
    assert_eq!(sys::FMT_Word16, super::FMT_Word16);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexWord16() {
    assert_eq!(sys::FMT_HexWord16, super::FMT_HexWord16);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT32_MIN() {
    assert_eq!(sys::STG_INT32_MIN, super::STG_INT32_MIN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_INT32_MAX() {
    assert_eq!(sys::STG_INT32_MAX, super::STG_INT32_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_WORD32_MAX() {
    assert_eq!(sys::STG_WORD32_MAX, super::STG_WORD32_MAX.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Word32() {
    assert_eq!(sys::FMT_Word32, super::FMT_Word32);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexWord32() {
    assert_eq!(sys::FMT_HexWord32, super::FMT_HexWord32);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Int32() {
    assert_eq!(sys::FMT_Int32, super::FMT_Int32);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Word64() {
    assert_eq!(sys::FMT_Word64, super::FMT_Word64);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexWord64() {
    assert_eq!(sys::FMT_HexWord64, super::FMT_HexWord64);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Int64() {
    assert_eq!(sys::FMT_Int64, super::FMT_Int64);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Word() {
    assert_eq!(sys::FMT_Word, super::FMT_Word);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_HexWord() {
    assert_eq!(sys::FMT_HexWord, super::FMT_HexWord);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_Int() {
    assert_eq!(sys::FMT_Int, super::FMT_Int);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgWord128() {
    assert_eq!(size_of::<sys::StgWord128>(), size_of::<super::StgWord128>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord128"][::core::mem::size_of::<StgWord128>() - 16usize];
    ["Alignment of StgWord128"][::core::mem::align_of::<StgWord128>() - 8usize];
    ["Offset of field: StgWord128::h"][::core::mem::offset_of!(StgWord128, h) - 0usize];
    ["Offset of field: StgWord128::l"][::core::mem::offset_of!(StgWord128, l) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgWord256() {
    assert_eq!(size_of::<sys::StgWord256>(), size_of::<super::StgWord256>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord256"][::core::mem::size_of::<StgWord256>() - 32usize];
    ["Alignment of StgWord256"][::core::mem::align_of::<StgWord256>() - 8usize];
    ["Offset of field: StgWord256::h"][::core::mem::offset_of!(StgWord256, h) - 0usize];
    ["Offset of field: StgWord256::l"][::core::mem::offset_of!(StgWord256, l) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgWord512() {
    assert_eq!(size_of::<sys::StgWord512>(), size_of::<super::StgWord512>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord512"][::core::mem::size_of::<StgWord512>() - 64usize];
    ["Alignment of StgWord512"][::core::mem::align_of::<StgWord512>() - 8usize];
    ["Offset of field: StgWord512::h"][::core::mem::offset_of!(StgWord512, h) - 0usize];
    ["Offset of field: StgWord512::l"][::core::mem::offset_of!(StgWord512, l) - 32usize];
};
