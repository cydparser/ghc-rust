use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub const ARG_GEN: u32 = 0;

pub const ARG_GEN_BIG: u32 = 1;

pub const ARG_BCO: u32 = 2;

pub const ARG_NONE: u32 = 3;

pub const ARG_N: u32 = 4;

pub const ARG_P: u32 = 5;

pub const ARG_F: u32 = 6;

pub const ARG_D: u32 = 7;

pub const ARG_L: u32 = 8;

pub const ARG_V16: u32 = 9;

pub const ARG_V32: u32 = 10;

pub const ARG_V64: u32 = 11;

pub const ARG_NN: u32 = 12;

pub const ARG_NP: u32 = 13;

pub const ARG_PN: u32 = 14;

pub const ARG_PP: u32 = 15;

pub const ARG_NNN: u32 = 16;

pub const ARG_NNP: u32 = 17;

pub const ARG_NPN: u32 = 18;

pub const ARG_NPP: u32 = 19;

pub const ARG_PNN: u32 = 20;

pub const ARG_PNP: u32 = 21;

pub const ARG_PPN: u32 = 22;

pub const ARG_PPP: u32 = 23;

pub const ARG_PPPP: u32 = 24;

pub const ARG_PPPPP: u32 = 25;

pub const ARG_PPPPPP: u32 = 26;

pub const ARG_PPPPPPP: u32 = 27;

pub const ARG_PPPPPPPP: u32 = 28;
