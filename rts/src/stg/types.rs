use crate::prelude::*;

// This module should define types *only*, all beginning with "Stg".
//
// Specifically:
//
//     StgInt8,  16, 32, 64
//     StgWord8, 16, 32, 64
//     StgChar, StgFloat, StgDouble
//
//     ***** All the same size (i.e. sizeof(void *)): *****
//     StgPtr                  Basic pointer type
//     StgWord                 Unit of heap allocation
//     StgInt                  Signed version of StgWord
//     StgAddr                 Generic address type
//
//     StgBool, StgVoid, StgPtr, StgOffset,
//     StgCode, StgStablePtr, StgFunPtr,
//     StgUnion.

#[cfg(test)]
mod tests;

pub(crate) const STG_INT8_MIN: i32 = -128;

// First, platform-dependent definitions of size-specific integers.

/// - GHC_PLACES: {testsuite}
pub const STG_INT8_MAX: u32 = i8::MAX as u32;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD8_MAX: u32 = u8::MAX as u32;

pub(crate) const STG_INT16_MIN: i32 = i16::MIN as i32;

/// - GHC_PLACES: {testsuite}
pub const STG_INT16_MAX: u32 = i16::MAX as u32;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD16_MAX: u32 = u16::MAX as u32;

pub(crate) const STG_INT32_MIN: i32 = i32::MIN;

/// - GHC_PLACES: {testsuite}
pub const STG_INT32_MAX: u32 = i32::MAX as u32;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD32_MAX: u32 = u32::MAX;

pub(crate) const STG_INT64_MIN: i64 = i64::MIN;

/// - GHC_PLACES: {testsuite}
pub const STG_INT64_MAX: u64 = i64::MAX as u64;

/// - GHC_PLACES: {testsuite}
pub const STG_WORD64_MAX: u64 = u64::MAX;

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

// TODO(rust): pub type StgWord128 = u128;
/// cbindgen:no-export
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

// Stg{Int,Word} are defined such that they have the exact same size as a void pointer.

const _: () = {
    if cfg!(not(any(
        target_pointer_width = "64",
        target_pointer_width = "32"
    ))) {
        panic!("GHC untested on this architecture: sizeof(void *) != 4 or 8");
    }
};

/// - GHC_PLACES: {testsuite}
#[cfg(target_pointer_width = "64")]
pub type StgInt = i64;
#[cfg(target_pointer_width = "32")]
pub type StgInt = i32;

/// - GHC_PLACES: {libraries, testsuite}
#[cfg(target_pointer_width = "64")]
pub type StgWord = u64;
#[cfg(target_pointer_width = "32")]
pub type StgWord = u32;

#[cfg(target_pointer_width = "64")]
pub(crate) type StgHalfInt = i32;
#[cfg(target_pointer_width = "32")]
pub(crate) type StgHalfInt = i16;

#[cfg(target_pointer_width = "64")]
pub(crate) type StgHalfWord = u32;
#[cfg(target_pointer_width = "32")]
pub(crate) type StgHalfWord = u16;

// Other commonly-used STG datatypes.

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

// Types for generated C functions when compiling via C.
//
// The C functions take no arguments, and return a pointer to the next
// function to be called use: Ptr to Fun that returns a Ptr to Fun
// which returns Ptr to void
//
// Note: Neither StgFunPtr not StgFun is quite right (that is,
// StgFunPtr != StgFun*).  So, the functions we define all have type
// StgFun but we always have to cast them to StgFunPtr when we assign
// them to something.
// The only way round this would be to write a recursive type but
// C only allows that if you're defining a struct or union.

pub(crate) type StgFunPtr =
    Option<unsafe extern "C" fn() -> Option<unsafe extern "C" fn() -> *mut c_void>>;

pub(crate) type StgFun = Option<unsafe extern "C" fn() -> StgFunPtr>;
