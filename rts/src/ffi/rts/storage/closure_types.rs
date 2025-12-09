use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib)]
pub const INVALID_OBJECT: u32 = 0;

#[ffi(compiler, ghc_lib, libraries)]
pub const CONSTR: u32 = 1;

#[ffi(compiler, ghc_lib)]
pub const CONSTR_1_0: u32 = 2;

#[ffi(compiler, ghc_lib, testsuite)]
pub const CONSTR_0_1: u32 = 3;

#[ffi(compiler, ghc_lib)]
pub const CONSTR_2_0: u32 = 4;

#[ffi(compiler, ghc_lib)]
pub const CONSTR_1_1: u32 = 5;

#[ffi(compiler, ghc_lib)]
pub const CONSTR_0_2: u32 = 6;

#[ffi(compiler, ghc_lib)]
pub const CONSTR_NOCAF: u32 = 7;

#[ffi(compiler, ghc_lib)]
pub const FUN: u32 = 8;

#[ffi(compiler, ghc_lib)]
pub const FUN_1_0: u32 = 9;

#[ffi(compiler, ghc_lib)]
pub const FUN_0_1: u32 = 10;

#[ffi(compiler, ghc_lib)]
pub const FUN_2_0: u32 = 11;

#[ffi(compiler, ghc_lib)]
pub const FUN_1_1: u32 = 12;

#[ffi(compiler, ghc_lib)]
pub const FUN_0_2: u32 = 13;

#[ffi(compiler, ghc_lib)]
pub const FUN_STATIC: u32 = 14;

#[ffi(compiler, ghc_lib)]
pub const THUNK: u32 = 15;

#[ffi(compiler, ghc_lib)]
pub const THUNK_1_0: u32 = 16;

#[ffi(compiler, ghc_lib)]
pub const THUNK_0_1: u32 = 17;

#[ffi(compiler, ghc_lib)]
pub const THUNK_2_0: u32 = 18;

#[ffi(compiler, ghc_lib)]
pub const THUNK_1_1: u32 = 19;

#[ffi(compiler, ghc_lib)]
pub const THUNK_0_2: u32 = 20;

#[ffi(compiler, ghc_lib)]
pub const THUNK_STATIC: u32 = 21;

#[ffi(compiler, ghc_lib)]
pub const THUNK_SELECTOR: u32 = 22;

#[ffi(compiler, ghc_lib)]
pub const BCO: u32 = 23;

#[ffi(compiler, ghc_lib)]
pub const AP: u32 = 24;

#[ffi(compiler, ghc_lib)]
pub const PAP: u32 = 25;

#[ffi(compiler, ghc_lib)]
pub const AP_STACK: u32 = 26;

#[ffi(compiler, ghc_lib)]
pub const IND: u32 = 27;

#[ffi(compiler, ghc_lib)]
pub const IND_STATIC: u32 = 28;

#[ffi(compiler, ghc_lib)]
pub const RET_BCO: u32 = 29;

#[ffi(compiler, ghc_lib)]
pub const RET_SMALL: u32 = 30;

#[ffi(compiler, ghc_lib, testsuite)]
pub const RET_BIG: u32 = 31;

#[ffi(compiler, ghc_lib)]
pub const RET_FUN: u32 = 32;

#[ffi(compiler, ghc_lib)]
pub const UPDATE_FRAME: u32 = 33;

#[ffi(compiler, ghc_lib)]
pub const CATCH_FRAME: u32 = 34;

#[ffi(compiler, ghc_lib)]
pub const UNDERFLOW_FRAME: u32 = 35;

#[ffi(compiler, ghc_lib)]
pub const STOP_FRAME: u32 = 36;

#[ffi(compiler, ghc_lib)]
pub const BLOCKING_QUEUE: u32 = 37;

#[ffi(compiler, ghc_lib)]
pub const BLACKHOLE: u32 = 38;

#[ffi(compiler, ghc_lib)]
pub const MVAR_CLEAN: u32 = 39;

#[ffi(compiler, ghc_lib)]
pub const MVAR_DIRTY: u32 = 40;

#[ffi(compiler, ghc_lib)]
pub const TVAR: u32 = 41;

#[ffi(compiler, ghc_lib)]
pub const ARR_WORDS: u32 = 42;

#[ffi(compiler, ghc_lib)]
pub const MUT_ARR_PTRS_CLEAN: u32 = 43;

#[ffi(compiler, ghc_lib)]
pub const MUT_ARR_PTRS_DIRTY: u32 = 44;

#[ffi(compiler, ghc_lib)]
pub const MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 45;

#[ffi(compiler, ghc_lib)]
pub const MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 46;

#[ffi(compiler, ghc_lib)]
pub const MUT_VAR_CLEAN: u32 = 47;

#[ffi(compiler, ghc_lib)]
pub const MUT_VAR_DIRTY: u32 = 48;

#[ffi(compiler, ghc_lib, testsuite)]
pub const WEAK: u32 = 49;

#[ffi(compiler, ghc_lib)]
pub const PRIM: u32 = 50;

#[ffi(compiler, ghc_lib)]
pub const MUT_PRIM: u32 = 51;

#[ffi(compiler, ghc_lib, testsuite)]
pub const TSO: u32 = 52;

#[ffi(compiler, ghc_lib, testsuite)]
pub const STACK: u32 = 53;

#[ffi(compiler, ghc_lib)]
pub const TREC_CHUNK: u32 = 54;

#[ffi(compiler, ghc_lib)]
pub const ATOMICALLY_FRAME: u32 = 55;

#[ffi(compiler, ghc_lib)]
pub const CATCH_RETRY_FRAME: u32 = 56;

#[ffi(compiler, ghc_lib)]
pub const CATCH_STM_FRAME: u32 = 57;

#[ffi(compiler, ghc_lib)]
pub const WHITEHOLE: u32 = 58;

#[ffi(compiler, ghc_lib)]
pub const SMALL_MUT_ARR_PTRS_CLEAN: u32 = 59;

#[ffi(compiler, ghc_lib)]
pub const SMALL_MUT_ARR_PTRS_DIRTY: u32 = 60;

#[ffi(compiler, ghc_lib)]
pub const SMALL_MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 61;

#[ffi(compiler, ghc_lib)]
pub const SMALL_MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 62;

#[ffi(compiler, ghc_lib)]
pub const COMPACT_NFDATA: u32 = 63;

#[ffi(compiler, ghc_lib)]
pub const CONTINUATION: u32 = 64;

#[ffi(compiler, ghc_lib)]
pub const ANN_FRAME: u32 = 65;

#[ffi(compiler, ghc_lib)]
pub const N_CLOSURE_TYPES: u32 = 66;
