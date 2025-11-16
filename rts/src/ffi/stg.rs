use crate::ffi::stg::types::{StgChar, StgFunPtr, StgInt, StgWord};

pub mod mach_regs_for_host;
pub mod misc_closures;
pub mod prim;
pub mod regs;
pub mod smp;
pub mod ticky;
pub mod types;

#[cfg(test)]
mod tests;

pub(crate) const BITS_PER_BYTE: u32 = 8;

pub(crate) type C_ = StgChar;

/// - GHC_PLACES: {libraries, testsuite}
pub type W_ = StgWord;

pub(crate) type P_ = *mut StgWord;

pub(crate) type I_ = StgInt;

pub(crate) type StgWordArray = [StgWord; 0usize];

pub(crate) type F_ = StgFunPtr;
