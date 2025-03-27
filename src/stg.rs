use crate::mach_deps;
use crate::rts::tsan_utils;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
pub mod types;

pub mod dll;

pub mod mach_regs_for_host;

pub mod regs;

pub mod ticky;

pub mod misc_closures;

pub mod prim;

pub mod smp;

#[cfg(test)]
mod tests;

pub(crate) const BITS_PER_BYTE: u32 = 8;

pub type C_ = StgChar;

pub type W_ = StgWord;

pub type P_ = *mut StgWord;

pub type I_ = StgInt;

pub type StgWordArray = [StgWord; 0usize];

pub type F_ = StgFunPtr;
