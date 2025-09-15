#[cfg(test)]
mod tests;

/// - GHC_PLACES: {compiler, libraries}
pub const INVALID_OBJECT: u32 = 0;

/// - GHC_PLACES: {compiler, libraries}
pub const CONSTR: u32 = 1;

/// - GHC_PLACES: {compiler}
pub const CONSTR_1_0: u32 = 2;

/// - GHC_PLACES: {compiler, testsuite}
pub const CONSTR_0_1: u32 = 3;

/// - GHC_PLACES: {compiler}
pub const CONSTR_2_0: u32 = 4;

/// - GHC_PLACES: {compiler}
pub const CONSTR_1_1: u32 = 5;

/// - GHC_PLACES: {compiler}
pub const CONSTR_0_2: u32 = 6;

/// - GHC_PLACES: {compiler}
pub const CONSTR_NOCAF: u32 = 7;

/// - GHC_PLACES: {compiler}
pub const FUN: u32 = 8;

/// - GHC_PLACES: {compiler}
pub const FUN_1_0: u32 = 9;

/// - GHC_PLACES: {compiler}
pub const FUN_0_1: u32 = 10;

/// - GHC_PLACES: {compiler}
pub const FUN_2_0: u32 = 11;

/// - GHC_PLACES: {compiler}
pub const FUN_1_1: u32 = 12;

/// - GHC_PLACES: {compiler}
pub const FUN_0_2: u32 = 13;

/// - GHC_PLACES: {compiler}
pub const FUN_STATIC: u32 = 14;

/// - GHC_PLACES: {compiler}
pub const THUNK: u32 = 15;

/// - GHC_PLACES: {compiler}
pub const THUNK_1_0: u32 = 16;

/// - GHC_PLACES: {compiler}
pub const THUNK_0_1: u32 = 17;

/// - GHC_PLACES: {compiler}
pub const THUNK_2_0: u32 = 18;

/// - GHC_PLACES: {compiler}
pub const THUNK_1_1: u32 = 19;

/// - GHC_PLACES: {compiler}
pub const THUNK_0_2: u32 = 20;

/// - GHC_PLACES: {compiler}
pub const THUNK_STATIC: u32 = 21;

/// - GHC_PLACES: {compiler}
pub const THUNK_SELECTOR: u32 = 22;

/// - GHC_PLACES: {compiler}
pub const BCO: u32 = 23;

/// - GHC_PLACES: {compiler}
pub const AP: u32 = 24;

/// - GHC_PLACES: {compiler}
pub const PAP: u32 = 25;

/// - GHC_PLACES: {compiler}
pub const AP_STACK: u32 = 26;

/// - GHC_PLACES: {compiler}
pub const IND: u32 = 27;

/// - GHC_PLACES: {compiler}
pub const IND_STATIC: u32 = 28;

/// - GHC_PLACES: {compiler}
pub const RET_BCO: u32 = 29;

/// - GHC_PLACES: {compiler}
pub const RET_SMALL: u32 = 30;

/// - GHC_PLACES: {compiler, testsuite}
pub const RET_BIG: u32 = 31;

/// - GHC_PLACES: {compiler}
pub const RET_FUN: u32 = 32;

/// - GHC_PLACES: {compiler}
pub const UPDATE_FRAME: u32 = 33;

/// - GHC_PLACES: {compiler}
pub const CATCH_FRAME: u32 = 34;

/// - GHC_PLACES: {compiler, libraries}
pub const UNDERFLOW_FRAME: u32 = 35;

/// - GHC_PLACES: {compiler}
pub const STOP_FRAME: u32 = 36;

/// - GHC_PLACES: {compiler}
pub const BLOCKING_QUEUE: u32 = 37;

/// - GHC_PLACES: {compiler, libraries}
pub const BLACKHOLE: u32 = 38;

/// - GHC_PLACES: {compiler}
pub const MVAR_CLEAN: u32 = 39;

/// - GHC_PLACES: {compiler}
pub const MVAR_DIRTY: u32 = 40;

/// - GHC_PLACES: {compiler}
pub const TVAR: u32 = 41;

/// - GHC_PLACES: {compiler}
pub const ARR_WORDS: u32 = 42;

/// - GHC_PLACES: {compiler}
pub const MUT_ARR_PTRS_CLEAN: u32 = 43;

/// - GHC_PLACES: {compiler}
pub const MUT_ARR_PTRS_DIRTY: u32 = 44;

/// - GHC_PLACES: {compiler}
pub const MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 45;

/// - GHC_PLACES: {compiler}
pub const MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 46;

/// - GHC_PLACES: {compiler}
pub const MUT_VAR_CLEAN: u32 = 47;

/// - GHC_PLACES: {compiler}
pub const MUT_VAR_DIRTY: u32 = 48;

/// - GHC_PLACES: {compiler, testsuite}
pub const WEAK: u32 = 49;

/// - GHC_PLACES: {compiler}
pub const PRIM: u32 = 50;

/// - GHC_PLACES: {compiler}
pub const MUT_PRIM: u32 = 51;

/// - GHC_PLACES: {compiler, libraries, testsuite}
pub const TSO: u32 = 52;

/// - GHC_PLACES: {compiler, libraries, testsuite}
pub const STACK: u32 = 53;

/// - GHC_PLACES: {compiler}
pub const TREC_CHUNK: u32 = 54;

/// - GHC_PLACES: {compiler}
pub const ATOMICALLY_FRAME: u32 = 55;

/// - GHC_PLACES: {compiler}
pub const CATCH_RETRY_FRAME: u32 = 56;

/// - GHC_PLACES: {compiler}
pub const CATCH_STM_FRAME: u32 = 57;

/// - GHC_PLACES: {compiler}
pub const WHITEHOLE: u32 = 58;

/// - GHC_PLACES: {compiler}
pub const SMALL_MUT_ARR_PTRS_CLEAN: u32 = 59;

/// - GHC_PLACES: {compiler}
pub const SMALL_MUT_ARR_PTRS_DIRTY: u32 = 60;

/// - GHC_PLACES: {compiler}
pub const SMALL_MUT_ARR_PTRS_FROZEN_DIRTY: u32 = 61;

/// - GHC_PLACES: {compiler}
pub const SMALL_MUT_ARR_PTRS_FROZEN_CLEAN: u32 = 62;

/// - GHC_PLACES: {compiler}
pub const COMPACT_NFDATA: u32 = 63;

/// - GHC_PLACES: {compiler}
pub const CONTINUATION: u32 = 64;

/// - GHC_PLACES: {compiler}
pub const N_CLOSURE_TYPES: u32 = 65;
