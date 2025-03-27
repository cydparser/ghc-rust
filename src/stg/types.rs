use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub(crate) const STG_INT8_MIN: i32 = -128;

pub const STG_INT8_MAX: u32 = 127;

pub const STG_WORD8_MAX: u32 = 255;

pub(crate) const FMT_Word8: &[u8; 2] = b"u\0";

pub(crate) const FMT_HexWord8: &[u8; 2] = b"x\0";

pub(crate) const STG_INT16_MIN: i32 = -32768;

pub const STG_INT16_MAX: u32 = 32767;

pub const STG_WORD16_MAX: u32 = 65535;

pub(crate) const FMT_Word16: &[u8; 2] = b"u\0";

pub(crate) const FMT_HexWord16: &[u8; 2] = b"x\0";

pub(crate) const STG_INT32_MIN: i32 = -2147483648;

pub const STG_INT32_MAX: u32 = 2147483647;

pub const STG_WORD32_MAX: u32 = 4294967295;

pub const FMT_Word32: &[u8; 2] = b"u\0";

pub(crate) const FMT_HexWord32: &[u8; 2] = b"x\0";

pub(crate) const FMT_Int32: &[u8; 2] = b"d\0";

pub(crate) const FMT_Word64: &[u8; 3] = b"lu\0";

pub(crate) const FMT_HexWord64: &[u8; 3] = b"lx\0";

pub(crate) const FMT_Int64: &[u8; 3] = b"ld\0";

pub const FMT_Word: &[u8; 3] = b"lu\0";

pub(crate) const FMT_HexWord: &[u8; 3] = b"lx\0";

pub const FMT_Int: &[u8; 3] = b"ld\0";

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

pub type StgAddr = *mut ::core::ffi::c_void;

pub type StgChar = StgWord32;

pub type StgBool = ::core::ffi::c_int;

pub type StgFloat = f32;

pub type StgDouble = f64;

pub type StgPtr = *mut StgWord;

pub(crate) type StgVolatilePtr = *mut StgWord;

pub(crate) type StgOffset = StgWord;

pub(crate) type StgCode = StgWord8;

pub type StgStablePtr = *mut ::core::ffi::c_void;

pub(crate) type StgByteArray = *mut StgWord8;

pub(crate) type StgFunPtr = ::core::option::Option<
    unsafe extern "C" fn() -> ::core::option::Option<
        unsafe extern "C" fn() -> *mut ::core::ffi::c_void,
    >,
>;
pub(crate) type StgFun = ::core::option::Option<unsafe extern "C" fn() -> StgFunPtr>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgClosure_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<StgClosure_> for sys::StgClosure_ {
    fn from(x: StgClosure_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgClosure_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgClosure_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgThunk_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<StgThunk_> for sys::StgThunk_ {
    fn from(x: StgThunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgThunk_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunk_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct Capability_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<Capability_> for sys::Capability_ {
    fn from(x: Capability_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Capability_ {
    fn arbitrary(g: &mut Gen) -> Self {
        Capability_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}
