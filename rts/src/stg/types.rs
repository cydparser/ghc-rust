use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const STG_INT8_MIN: i32 = -128;

/// - GHC_PLACES: {testsuite}
pub const STG_INT8_MAX: u32 = 127;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD8_MAX: u32 = 255;

pub(crate) const STG_INT16_MIN: i32 = -32768;

/// - GHC_PLACES: {testsuite}
pub const STG_INT16_MAX: u32 = 32767;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD16_MAX: u32 = 65535;

pub(crate) const STG_INT32_MIN: i32 = -2147483648;

/// - GHC_PLACES: {testsuite}
pub const STG_INT32_MAX: u32 = 2147483647;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD32_MAX: u32 = 4294967295;

pub(crate) const STG_INT64_MIN: i64 = -9223372036854775808;

/// - GHC_PLACES: {testsuite}
pub const STG_INT64_MAX: u64 = 9223372036854775807;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD64_MAX: i32 = -1;

pub(crate) const STG_INT_MIN: i64 = -9223372036854775808;

/// - GHC_PLACES: {testsuite}
pub const STG_INT_MAX: u64 = 9223372036854775807;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD_MAX: i32 = -1;

/// - GHC_PLACES: {testsuite}
pub type StgInt8 = i8;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgWord8 = u8;

/// - GHC_PLACES: {testsuite}
pub type StgInt16 = i16;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgWord16 = u16;

/// - GHC_PLACES: {testsuite}
pub type StgInt32 = i32;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgWord32 = u32;

/// - GHC_PLACES: {testsuite}
pub type StgInt64 = i64;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgWord64 = u64;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct StgWord128 {
    h: StgWord64,
    l: StgWord64,
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

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct StgWord256 {
    h: StgWord128,
    l: StgWord128,
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

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct StgWord512 {
    h: StgWord256,
    l: StgWord256,
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

/// - GHC_PLACES: {testsuite}
pub type StgInt = i64;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgWord = u64;

pub(crate) type StgHalfInt = i32;

pub(crate) type StgHalfWord = u32;

pub(crate) type StgAddr = *mut c_void;

/// - GHC_PLACES: {testsuite}
pub type StgChar = StgWord32;

/// - GHC_PLACES: {testsuite}
pub type StgBool = c_int;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgFloat = f32;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgDouble = f64;

/// - GHC_PLACES: {libraries, testsuite}
pub type StgPtr = *mut StgWord;

pub(crate) type StgVolatilePtr = *mut StgWord;

pub(crate) type StgOffset = StgWord;

pub(crate) type StgCode = StgWord8;

/// - GHC_PLACES: {testsuite}
pub type StgStablePtr = *mut c_void;

pub(crate) type StgByteArray = *mut StgWord8;

pub(crate) type StgFunPtr =
    Option<unsafe extern "C" fn() -> Option<unsafe extern "C" fn() -> *mut c_void>>;

pub(crate) type StgFun = Option<unsafe extern "C" fn() -> StgFunPtr>;
