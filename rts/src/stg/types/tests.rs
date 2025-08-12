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
fn sys_size_StgWord128() {
    assert_eq!(size_of::<sys::StgWord128>(), size_of::<StgWord128>())
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
fn sys_size_StgWord256() {
    assert_eq!(size_of::<sys::StgWord256>(), size_of::<StgWord256>())
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
fn sys_size_StgWord512() {
    assert_eq!(size_of::<sys::StgWord512>(), size_of::<StgWord512>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgWord512"][::core::mem::size_of::<StgWord512>() - 64usize];
    ["Alignment of StgWord512"][::core::mem::align_of::<StgWord512>() - 8usize];
    ["Offset of field: StgWord512::h"][::core::mem::offset_of!(StgWord512, h) - 0usize];
    ["Offset of field: StgWord512::l"][::core::mem::offset_of!(StgWord512, l) - 32usize];
};
