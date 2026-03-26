use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_eq() {
    assert_eq!(HS_BOOL_TRUE, sys::HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_HS_BOOL_TRUE_layout() {
    assert_eq!(size_of_val(&HS_BOOL_TRUE), size_of_val(&sys::HS_BOOL_TRUE));
    assert_eq!(
        align_of_val(&HS_BOOL_TRUE),
        align_of_val(&sys::HS_BOOL_TRUE)
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsChar_layout() {
    assert_eq!(size_of::<HsChar>(), size_of::<HsChar>());
    assert_eq!(align_of::<HsChar>(), align_of::<HsChar>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt_layout() {
    assert_eq!(size_of::<HsInt>(), size_of::<HsInt>());
    assert_eq!(align_of::<HsInt>(), align_of::<HsInt>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt8_layout() {
    assert_eq!(size_of::<HsInt8>(), size_of::<HsInt8>());
    assert_eq!(align_of::<HsInt8>(), align_of::<HsInt8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt16_layout() {
    assert_eq!(size_of::<HsInt16>(), size_of::<HsInt16>());
    assert_eq!(align_of::<HsInt16>(), align_of::<HsInt16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt32_layout() {
    assert_eq!(size_of::<HsInt32>(), size_of::<HsInt32>());
    assert_eq!(align_of::<HsInt32>(), align_of::<HsInt32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsInt64_layout() {
    assert_eq!(size_of::<HsInt64>(), size_of::<HsInt64>());
    assert_eq!(align_of::<HsInt64>(), align_of::<HsInt64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord_layout() {
    assert_eq!(size_of::<HsWord>(), size_of::<HsWord>());
    assert_eq!(align_of::<HsWord>(), align_of::<HsWord>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord8_layout() {
    assert_eq!(size_of::<HsWord8>(), size_of::<HsWord8>());
    assert_eq!(align_of::<HsWord8>(), align_of::<HsWord8>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord16_layout() {
    assert_eq!(size_of::<HsWord16>(), size_of::<HsWord16>());
    assert_eq!(align_of::<HsWord16>(), align_of::<HsWord16>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord32_layout() {
    assert_eq!(size_of::<HsWord32>(), size_of::<HsWord32>());
    assert_eq!(align_of::<HsWord32>(), align_of::<HsWord32>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsWord64_layout() {
    assert_eq!(size_of::<HsWord64>(), size_of::<HsWord64>());
    assert_eq!(align_of::<HsWord64>(), align_of::<HsWord64>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsFloat_layout() {
    assert_eq!(size_of::<HsFloat>(), size_of::<HsFloat>());
    assert_eq!(align_of::<HsFloat>(), align_of::<HsFloat>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsDouble_layout() {
    assert_eq!(size_of::<HsDouble>(), size_of::<HsDouble>());
    assert_eq!(align_of::<HsDouble>(), align_of::<HsDouble>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsBool_layout() {
    assert_eq!(size_of::<HsBool>(), size_of::<HsBool>());
    assert_eq!(align_of::<HsBool>(), align_of::<HsBool>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_HsPtr_layout() {
    assert_eq!(size_of::<HsPtr>(), size_of::<HsPtr>());
    assert_eq!(align_of::<HsPtr>(), align_of::<HsPtr>());
}
