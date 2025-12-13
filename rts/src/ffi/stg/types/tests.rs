use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT8_MAX_eq() {
    assert_eq!(STG_INT8_MAX, sys::STG_INT8_MAX as StgInt8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_WORD8_MAX_eq() {
    assert_eq!(STG_WORD8_MAX, sys::STG_WORD8_MAX as StgWord8);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT16_MAX_eq() {
    assert_eq!(STG_INT16_MAX, sys::STG_INT16_MAX as StgInt16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_WORD16_MAX_eq() {
    assert_eq!(STG_WORD16_MAX, sys::STG_WORD16_MAX as StgWord16);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT32_MAX_eq() {
    assert_eq!(STG_INT32_MAX, sys::STG_INT32_MAX as StgInt32);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_WORD32_MAX_eq() {
    assert_eq!(STG_WORD32_MAX, sys::STG_WORD32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT64_MAX_eq() {
    assert_eq!(STG_INT64_MAX, sys::STG_INT64_MAX as StgInt64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_WORD64_MAX_eq() {
    assert_eq!(STG_WORD64_MAX, sys::STG_WORD64_MAX as StgWord64);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT_MAX_eq() {
    assert_eq!(STG_INT_MAX, sys::STG_INT_MAX as StgInt);
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_INT_MAX_layout() {
    assert_eq!(size_of_val(&STG_INT_MAX), size_of_val(&sys::STG_INT_MAX));
    assert_eq!(align_of_val(&STG_INT_MAX), align_of_val(&sys::STG_INT_MAX));
}

#[cfg(feature = "sys")]
#[test]
fn sys_STG_WORD_MAX_eq() {
    assert_eq!(STG_WORD_MAX, sys::STG_WORD_MAX as StgWord);
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInt8_layout() {
    assert_eq!(size_of::<StgInt8>(), size_of::<StgInt8>());
    assert_eq!(align_of::<StgInt8>(), align_of::<StgInt8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord8_layout() {
    assert_eq!(size_of::<StgWord8>(), size_of::<StgWord8>());
    assert_eq!(align_of::<StgWord8>(), align_of::<StgWord8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInt16_layout() {
    assert_eq!(size_of::<StgInt16>(), size_of::<StgInt16>());
    assert_eq!(align_of::<StgInt16>(), align_of::<StgInt16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord16_layout() {
    assert_eq!(size_of::<StgWord16>(), size_of::<StgWord16>());
    assert_eq!(align_of::<StgWord16>(), align_of::<StgWord16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInt32_layout() {
    assert_eq!(size_of::<StgInt32>(), size_of::<StgInt32>());
    assert_eq!(align_of::<StgInt32>(), align_of::<StgInt32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord32_layout() {
    assert_eq!(size_of::<StgWord32>(), size_of::<StgWord32>());
    assert_eq!(align_of::<StgWord32>(), align_of::<StgWord32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInt64_layout() {
    assert_eq!(size_of::<StgInt64>(), size_of::<StgInt64>());
    assert_eq!(align_of::<StgInt64>(), align_of::<StgInt64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord64_layout() {
    assert_eq!(size_of::<StgWord64>(), size_of::<StgWord64>());
    assert_eq!(align_of::<StgWord64>(), align_of::<StgWord64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord128_layout() {
    assert_eq!(offset_of!(StgWord128, h), offset_of!(sys::StgWord128, h));
    assert_eq!(offset_of!(StgWord128, l), offset_of!(sys::StgWord128, l));
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(align_of::<StgWord128>(), align_of::<sys::StgWord128>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord256_layout() {
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(offset_of!(StgWord256, h), offset_of!(sys::StgWord256, h));
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(offset_of!(StgWord256, l), offset_of!(sys::StgWord256, l));
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(align_of::<StgWord256>(), align_of::<sys::StgWord256>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord512_layout() {
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(offset_of!(StgWord512, h), offset_of!(sys::StgWord512, h));
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(offset_of!(StgWord512, l), offset_of!(sys::StgWord512, l));
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(align_of::<StgWord512>(), align_of::<sys::StgWord512>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInt_layout() {
    assert_eq!(size_of::<StgInt>(), size_of::<StgInt>());
    assert_eq!(align_of::<StgInt>(), align_of::<StgInt>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgWord_layout() {
    assert_eq!(size_of::<StgWord>(), size_of::<StgWord>());
    assert_eq!(align_of::<StgWord>(), align_of::<StgWord>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgHalfWord_layout() {
    assert_eq!(size_of::<StgHalfWord>(), size_of::<StgHalfWord>());
    assert_eq!(align_of::<StgHalfWord>(), align_of::<StgHalfWord>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgAddr_layout() {
    assert_eq!(size_of::<StgAddr>(), size_of::<StgAddr>());
    assert_eq!(align_of::<StgAddr>(), align_of::<StgAddr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgChar_layout() {
    assert_eq!(size_of::<StgChar>(), size_of::<StgChar>());
    assert_eq!(align_of::<StgChar>(), align_of::<StgChar>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgBool_layout() {
    assert_eq!(size_of::<StgBool>(), size_of::<StgBool>());
    assert_eq!(align_of::<StgBool>(), align_of::<StgBool>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgFloat_layout() {
    assert_eq!(size_of::<StgFloat>(), size_of::<StgFloat>());
    assert_eq!(align_of::<StgFloat>(), align_of::<StgFloat>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgDouble_layout() {
    assert_eq!(size_of::<StgDouble>(), size_of::<StgDouble>());
    assert_eq!(align_of::<StgDouble>(), align_of::<StgDouble>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgPtr_layout() {
    assert_eq!(size_of::<StgPtr>(), size_of::<StgPtr>());
    assert_eq!(align_of::<StgPtr>(), align_of::<StgPtr>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgStablePtr_layout() {
    assert_eq!(size_of::<StgStablePtr>(), size_of::<StgStablePtr>());
    assert_eq!(align_of::<StgStablePtr>(), align_of::<StgStablePtr>());
}
