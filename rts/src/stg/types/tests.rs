use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT8_MIN() {
    assert_eq!(sys::STG_INT8_MIN, STG_INT8_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT8_MAX() {
    assert_eq!(sys::STG_INT8_MAX, STG_INT8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_WORD8_MAX() {
    assert_eq!(sys::STG_WORD8_MAX, STG_WORD8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT16_MIN() {
    assert_eq!(sys::STG_INT16_MIN, STG_INT16_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT16_MAX() {
    assert_eq!(sys::STG_INT16_MAX, STG_INT16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_WORD16_MAX() {
    assert_eq!(sys::STG_WORD16_MAX, STG_WORD16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT32_MIN() {
    assert_eq!(sys::STG_INT32_MIN, STG_INT32_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT32_MAX() {
    assert_eq!(sys::STG_INT32_MAX, STG_INT32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_WORD32_MAX() {
    assert_eq!(sys::STG_WORD32_MAX, STG_WORD32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT64_MIN() {
    assert_eq!(sys::STG_INT64_MIN, STG_INT64_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT64_MAX() {
    assert_eq!(sys::STG_INT64_MAX, STG_INT64_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_WORD64_MAX() {
    assert_eq!(sys::STG_WORD64_MAX as StgWord64, STG_WORD64_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT_MIN() {
    assert_eq!(sys::STG_INT_MIN, STG_INT_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_INT_MAX() {
    assert_eq!(sys::STG_INT_MAX, STG_INT_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STG_WORD_MAX() {
    assert_eq!(sys::STG_WORD_MAX, STG_WORD_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgWord128() {
    assert_eq!(size_of::<sys::StgWord128>(), size_of::<StgWord128>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord128"][size_of::<StgWord128>() - 16usize];
    ["Alignment of StgWord128"][align_of::<StgWord128>() - 8usize];
    ["Offset of field: StgWord128::h"][offset_of!(StgWord128, h) - 0usize];
    ["Offset of field: StgWord128::l"][offset_of!(StgWord128, l) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgWord256() {
    assert_eq!(size_of::<sys::StgWord256>(), size_of::<StgWord256>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord256"][size_of::<StgWord256>() - 32usize];
    ["Alignment of StgWord256"][align_of::<StgWord256>() - 8usize];
    ["Offset of field: StgWord256::h"][offset_of!(StgWord256, h) - 0usize];
    ["Offset of field: StgWord256::l"][offset_of!(StgWord256, l) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgWord512() {
    assert_eq!(size_of::<sys::StgWord512>(), size_of::<StgWord512>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord512"][size_of::<StgWord512>() - 64usize];
    ["Alignment of StgWord512"][align_of::<StgWord512>() - 8usize];
    ["Offset of field: StgWord512::h"][offset_of!(StgWord512, h) - 0usize];
    ["Offset of field: StgWord512::l"][offset_of!(StgWord512, l) - 32usize];
};
