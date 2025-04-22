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

pub const STG_SIG_DFL: i32 = -1;

pub const STG_SIG_IGN: i32 = -2;

pub const STG_SIG_ERR: i32 = -3;

pub const STG_SIG_HAN: i32 = -4;

pub const STG_SIG_RST: i32 = -5;
