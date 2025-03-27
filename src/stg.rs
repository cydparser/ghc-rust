use crate::stg::types::{StgInt, StgWord};

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

pub type C_ = types::StgChar;

pub type W_ = StgWord;

pub type P_ = *mut StgWord;

pub type I_ = StgInt;

pub type StgWordArray = [StgWord; 0usize];

pub type F_ = types::StgFunPtr;
