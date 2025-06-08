use std::ffi::{c_int, c_void};
use std::mem::transmute;

#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

pub(crate) const STG_INT8_MIN: i32 = -128;

pub const STG_INT8_MAX: u32 = 127;

pub const STG_WORD8_MAX: u32 = 255;

pub(crate) const STG_INT16_MIN: i32 = -32768;

pub const STG_INT16_MAX: u32 = 32767;

pub const STG_WORD16_MAX: u32 = 65535;

pub(crate) const STG_INT32_MIN: i32 = -2147483648;

pub const STG_INT32_MAX: u32 = 2147483647;

pub const STG_WORD32_MAX: u32 = 4294967295;

pub type StgInt8 = i8;

pub type StgWord8 = u8;

pub type StgInt16 = i16;

pub type StgWord16 = u16;

pub type StgInt32 = i32;

pub type StgWord32 = u32;

pub type StgInt64 = i64;

pub type StgWord64 = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct StgWord128 {
    pub h: StgWord64,
    pub l: StgWord64,
}

#[cfg(feature = "sys")]
impl From<StgWord128> for sys::StgWord128 {
    fn from(x: StgWord128) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgWord128 {
    fn arbitrary(g: &mut Gen) -> Self {
        StgWord128 {
            h: Arbitrary::arbitrary(g),
            l: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct StgWord256 {
    pub h: StgWord128,
    pub l: StgWord128,
}

#[cfg(feature = "sys")]
impl From<StgWord256> for sys::StgWord256 {
    fn from(x: StgWord256) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgWord256 {
    fn arbitrary(g: &mut Gen) -> Self {
        StgWord256 {
            h: Arbitrary::arbitrary(g),
            l: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct StgWord512 {
    pub h: StgWord256,
    pub l: StgWord256,
}

#[cfg(feature = "sys")]
impl From<StgWord512> for sys::StgWord512 {
    fn from(x: StgWord512) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgWord512 {
    fn arbitrary(g: &mut Gen) -> Self {
        StgWord512 {
            h: Arbitrary::arbitrary(g),
            l: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgInt = i64;

pub type StgWord = u64;

pub(crate) type StgHalfInt = i32;

pub type StgHalfWord = u32;

pub type StgAddr = *mut c_void;

pub type StgChar = StgWord32;

pub type StgBool = c_int;

pub type StgFloat = f32;

pub type StgDouble = f64;

pub type StgPtr = *mut StgWord;

pub(crate) type StgVolatilePtr = *mut StgWord;

pub(crate) type StgOffset = StgWord;

pub(crate) type StgCode = StgWord8;

pub type StgStablePtr = *mut c_void;

pub(crate) type StgByteArray = *mut StgWord8;

pub(crate) type StgFunPtr =
    Option<unsafe extern "C" fn() -> Option<unsafe extern "C" fn() -> *mut c_void>>;

pub(crate) type StgFun = Option<unsafe extern "C" fn() -> StgFunPtr>;

// TODO: Add forward declarations, StgClosure_, StgThunk_, Capability_ for the unregisterised backend.
