use crate::stg::types::{StgChar, StgFunPtr, StgInt, StgWord};

pub mod mach_regs_for_host;
pub mod misc_closures;
// TODO: pub mod prim;
pub mod regs;
// TODO: pub mod smp;
pub mod ticky;
pub mod types;

#[cfg(test)]
mod tests;

pub(crate) const BITS_PER_BYTE: u32 = 8;

pub type C_ = StgChar;

pub type W_ = StgWord;

pub type P_ = *mut StgWord;

pub type I_ = StgInt;

pub type StgWordArray = [StgWord; 0usize];

pub type F_ = StgFunPtr;
