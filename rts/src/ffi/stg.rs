use crate::ffi::stg::types::{StgChar, StgFunPtr, StgInt, StgWord};
use crate::prelude::*;

pub mod mach_regs_for_host;
pub mod misc_closures;
pub mod prim;
pub mod regs;
pub mod ticky;
pub mod types;

#[cfg(test)]
mod tests;

pub(crate) const BITS_PER_BYTE: u32 = 8;

#[ffi(compiler)]
pub type C_ = StgChar;

#[ffi(compiler, ghc_lib, testsuite)]
pub type W_ = StgWord;

#[ffi(compiler)]
pub type P_ = *mut StgWord;

#[ffi(compiler)]
pub type I_ = StgInt;

pub(crate) type StgWordArray = [StgWord; 0usize];

#[ffi(compiler)]
pub type F_ = StgFunPtr;
