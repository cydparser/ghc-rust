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

pub const INVALID_OBJECT: u32 = 0;

pub const CONSTR: u32 = 1;

pub const CONSTR_1_0: u32 = 2;

pub const CONSTR_0_1: u32 = 3;

pub const CONSTR_2_0: u32 = 4;

pub const CONSTR_1_1: u32 = 5;

pub const CONSTR_0_2: u32 = 6;

pub const CONSTR_NOCAF: u32 = 7;

pub const FUN: u32 = 8;

pub const FUN_1_0: u32 = 9;

pub const FUN_0_1: u32 = 10;

pub const FUN_2_0: u32 = 11;

pub const FUN_1_1: u32 = 12;

pub const FUN_0_2: u32 = 13;

pub const FUN_STATIC: u32 = 14;

pub const THUNK: u32 = 15;

pub const THUNK_1_0: u32 = 16;

pub const THUNK_0_1: u32 = 17;

pub const THUNK_2_0: u32 = 18;

pub const THUNK_1_1: u32 = 19;

pub const THUNK_0_2: u32 = 20;

pub const THUNK_STATIC: u32 = 21;

pub const THUNK_SELECTOR: u32 = 22;

pub const BCO: u32 = 23;

pub const AP: u32 = 24;

pub const PAP: u32 = 25;

pub const AP_STACK: u32 = 26;

pub const IND: u32 = 27;

pub const IND_STATIC: u32 = 28;

pub const RET_BCO: u32 = 29;

pub const RET_SMALL: u32 = 30;

pub const RET_BIG: u32 = 31;

pub const RET_FUN: u32 = 32;

pub const UPDATE_FRAME: u32 = 33;

pub const CATCH_FRAME: u32 = 34;

pub const UNDERFLOW_FRAME: u32 = 35;

pub const STOP_FRAME: u32 = 36;

pub const BLOCKING_QUEUE: u32 = 37;

pub const BLACKHOLE: u32 = 38;

pub const MVAR_CLEAN: u32 = 39;

pub const MVAR_DIRTY: u32 = 40;

pub const TVAR: u32 = 41;

pub const ARR_WORDS: u32 = 42;

pub const MUT_ARR_PTRS_CLEAN: u32 = 43;

pub const MUT_ARR_PTRS_DIRTY: u32 = 44;

pub const MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 45;

pub const MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 46;

pub const MUT_VAR_CLEAN: u32 = 47;

pub const MUT_VAR_DIRTY: u32 = 48;

pub const WEAK: u32 = 49;

pub const PRIM: u32 = 50;

pub const MUT_PRIM: u32 = 51;

pub const TSO: u32 = 52;

pub const STACK: u32 = 53;

pub const TREC_CHUNK: u32 = 54;

pub const ATOMICALLY_FRAME: u32 = 55;

pub const CATCH_RETRY_FRAME: u32 = 56;

pub const CATCH_STM_FRAME: u32 = 57;

pub const WHITEHOLE: u32 = 58;

pub const SMALL_MUT_ARR_PTRS_CLEAN: u32 = 59;

pub const SMALL_MUT_ARR_PTRS_DIRTY: u32 = 60;

pub const SMALL_MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 61;

pub const SMALL_MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 62;

pub const COMPACT_NFDATA: u32 = 63;

pub const CONTINUATION: u32 = 64;

pub const N_CLOSURE_TYPES: u32 = 65;
