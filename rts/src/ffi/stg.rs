use crate::prelude::*;
#[cfg(feature = "cbindgen")]
pub use crate::stg::types::{
    STG_INT_MAX, STG_INT8_MAX, STG_INT16_MAX, STG_INT32_MAX, STG_INT64_MAX, STG_WORD_MAX,
    STG_WORD8_MAX, STG_WORD16_MAX, STG_WORD32_MAX, STG_WORD64_MAX, StgAddr, StgBool, StgChar,
    StgDouble, StgFloat, StgFun, StgFunPtr, StgHalfWord, StgInt, StgInt8, StgInt16, StgInt32,
    StgInt64, StgPtr, StgStablePtr, StgWord, StgWord8, StgWord16, StgWord32, StgWord64, StgWord128,
    StgWord256, StgWord512,
};
#[cfg(not(feature = "cbindgen"))]
use crate::stg::types::{StgChar, StgFunPtr, StgInt, StgWord};

pub mod mach_regs_for_host;
pub mod misc_closures;
pub mod prim;
pub mod regs;
pub mod ticky;

#[cfg(test)]
mod tests;

pub(crate) const BITS_PER_BYTE: u32 = 8;

#[ffi(compiler)]
pub type C_ = StgChar;

#[ffi(compiler, docs, ghc_lib, libraries, testsuite)]
pub type W_ = StgWord;

#[ffi(compiler)]
pub type P_ = *mut StgWord;

#[ffi(compiler, ghc_lib)]
pub type I_ = StgInt;

pub(crate) type StgWordArray = [StgWord; 0usize];

#[ffi(compiler)]
pub type F_ = StgFunPtr;
