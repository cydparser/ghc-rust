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

// First, platform-dependent definitions of size-specific integers.

pub(crate) const STG_INT8_MIN: StgInt8 = StgInt8::MIN;

#[ffi(testsuite)]
pub const STG_INT8_MAX: StgInt8 = StgInt8::MAX;

#[ffi(testsuite)]
pub const STG_WORD8_MAX: StgWord8 = StgWord8::MAX;

pub(crate) const STG_INT16_MIN: StgInt16 = StgInt16::MIN;

#[ffi(testsuite)]
pub const STG_INT16_MAX: StgInt16 = StgInt16::MAX;

#[ffi(testsuite)]
pub const STG_WORD16_MAX: StgWord16 = StgWord16::MAX;

pub(crate) const STG_INT32_MIN: StgInt32 = StgInt32::MIN;

#[ffi(testsuite)]
pub const STG_INT32_MAX: StgInt32 = StgInt32::MAX;

#[ffi(testsuite)]
pub const STG_WORD32_MAX: StgWord32 = StgWord32::MAX;

pub(crate) const STG_INT64_MIN: StgInt64 = StgInt64::MIN;

#[ffi(testsuite)]
pub const STG_INT64_MAX: StgInt64 = StgInt64::MAX;

#[ffi(testsuite)]
pub const STG_WORD64_MAX: StgWord64 = StgWord64::MAX;

pub(crate) const STG_INT_MIN: StgInt = StgInt::MIN;

#[ffi(testsuite)]
pub const STG_INT_MAX: StgInt = StgInt::MAX;

#[ffi(testsuite)]
pub const STG_WORD_MAX: StgWord = StgWord::MAX;

#[ffi(compiler, testsuite)]
pub type StgInt8 = i8;

#[ffi(compiler, testsuite)]
pub type StgWord8 = u8;

#[ffi(compiler, testsuite)]
pub type StgInt16 = i16;

#[ffi(compiler, testsuite)]
pub type StgWord16 = u16;

#[ffi(compiler, testsuite)]
pub type StgInt32 = i32;

#[ffi(compiler, testsuite)]
pub type StgWord32 = u32;

#[ffi(compiler, testsuite)]
pub type StgInt64 = i64;

#[ffi(compiler, testsuite)]
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
impl From<sys::StgWord128> for StgWord128 {
    fn from(sys::StgWord128 { h, l }: sys::StgWord128) -> Self {
        StgWord128 { h, l }
    }
}

#[cfg(feature = "sys")]
impl From<StgWord128> for sys::StgWord128 {
    fn from(StgWord128 { h, l }: StgWord128) -> Self {
        sys::StgWord128 { h, l }
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
#[derive(Clone, Copy, Debug)]
pub struct StgWord256 {
    h: StgWord128,
    l: StgWord128,
}

#[cfg(feature = "sys")]
impl From<sys::StgWord256> for StgWord256 {
    fn from(sys::StgWord256 { h, l }: sys::StgWord256) -> Self {
        StgWord256 {
            h: h.into(),
            l: l.into(),
        }
    }
}

#[cfg(feature = "sys")]
impl From<StgWord256> for sys::StgWord256 {
    fn from(StgWord256 { h, l }: StgWord256) -> Self {
        sys::StgWord256 {
            h: h.into(),
            l: l.into(),
        }
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

// TODO(rust): The `cfg` attributes are only needed for cbingen---these should be isize/usize.

#[ffi(compiler, testsuite)]
#[cfg(target_pointer_width = "64")]
pub type StgInt = i64;
#[cfg(target_pointer_width = "32")]
pub type StgInt = i32;

#[ffi(compiler, ghc_lib, testsuite)]
#[cfg(target_pointer_width = "64")]
pub type StgWord = u64;
#[cfg(target_pointer_width = "32")]
pub type StgWord = u32;

#[cfg(target_pointer_width = "64")]
pub(crate) type StgHalfInt = i32;
#[cfg(target_pointer_width = "32")]
pub(crate) type StgHalfInt = i16;

#[ffi(compiler)]
pub type StgHalfWord = u32;

// Other commonly-used STG datatypes.

#[ffi(compiler)]
pub type StgAddr = *mut c_void;

#[ffi(compiler, testsuite)]
pub type StgChar = StgWord32;

#[ffi(testsuite)]
pub type StgBool = c_int;

#[ffi(compiler, testsuite)]
pub type StgFloat = f32;

#[ffi(compiler, testsuite)]
pub type StgDouble = f64;

/// A heap or stack pointer.
#[ffi(compiler, ghc_lib, testsuite)]
pub type StgPtr = *mut StgWord;

/// A pointer to a volatile word.
pub(crate) type StgVolatilePtr = *mut StgWord;

/// A byte offset within a closure.
pub(crate) type StgOffset = StgWord;

pub(crate) type StgCode = StgWord8;

/// An adjusted index into stable_ptr_table (see [ref:NULL StgStablePtr])
#[ffi(compiler, testsuite)]
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
