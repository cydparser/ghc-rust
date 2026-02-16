use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const MIN_PAYLOAD_SIZE: u32 = 1;

pub(crate) const MAX_SPEC_SELECTEE_SIZE: u32 = 15;

pub(crate) const MAX_SPEC_AP_SIZE: u32 = 7;

pub(crate) const MAX_SPEC_THUNK_SIZE: u32 = 2;

pub(crate) const MAX_SPEC_FUN_SIZE: u32 = 2;

pub(crate) const MAX_SPEC_CONSTR_SIZE: u32 = 2;

pub(crate) const MAX_INTLIKE: u32 = 255;

pub(crate) const MIN_INTLIKE: i32 = -16;

pub(crate) const MAX_CHARLIKE: u32 = 255;

pub(crate) const MIN_CHARLIKE: u32 = 0;

pub(crate) const MUT_ARR_PTRS_CARD_BITS: u32 = 7;

pub(crate) const MAX_VANILLA_REG: u32 = 10;

pub(crate) const MAX_FLOAT_REG: u32 = 6;

pub(crate) const MAX_DOUBLE_REG: u32 = 6;

pub(crate) const MAX_LONG_REG: u32 = 1;

pub(crate) const MAX_XMM_REG: u32 = 6;

pub(crate) const INFO_OTHER_TAG: i32 = -1;

pub(crate) const INFO_IND_TAG: i32 = -2;

pub(crate) const INFO_FIRST_TAG: u32 = 0;

pub(crate) const RESERVED_C_STACK_BYTES: u32 = 16384;

pub(crate) const STG_RUN: &CStr = c"_StgRun";

pub(crate) const STG_RETURN: &CStr = c"_StgReturn";

#[ffi(ghc_lib)]
pub const RESERVED_STACK_WORDS: u32 = 21;

pub(crate) const AP_STACK_SPLIM: u32 = 1024;

pub(crate) const BLOCK_SHIFT: u32 = 12;

pub(crate) const MBLOCK_SHIFT: u32 = 20;

pub(crate) const BITMAP_SIZE_MASK: u32 = 63;

#[ffi(compiler, ghc_lib)]
pub const BITMAP_BITS_SHIFT: u32 = 6;

pub(crate) const LDV_SHIFT: u32 = 30;

pub(crate) const LDV_STATE_MASK: u64 = 1152921504606846976;

pub(crate) const LDV_CREATE_MASK: u64 = 1152921503533105152;

pub(crate) const LDV_LAST_MASK: u32 = 1073741823;

pub(crate) const LDV_STATE_CREATE: u32 = 0;

pub(crate) const LDV_STATE_USE: u64 = 1152921504606846976;

pub(crate) const INVALID_GHC_POINTER: u32 = 0;

#[ffi(ghc_lib)]
pub const ThreadRunGHC: u32 = 1;

#[ffi(ghc_lib)]
pub const ThreadInterpret: u32 = 2;

#[ffi(compiler, ghc_lib)]
pub const ThreadKilled: u32 = 3;

#[ffi(ghc_lib)]
pub const ThreadComplete: u32 = 4;

#[ffi(ghc_lib)]
pub const NotBlocked: u32 = 0;

#[ffi(ghc_lib)]
pub const BlockedOnMVar: u32 = 1;

#[ffi(ghc_lib)]
pub const BlockedOnMVarRead: u32 = 14;

#[ffi(ghc_lib)]
pub const BlockedOnBlackHole: u32 = 2;

#[ffi(ghc_lib)]
pub const BlockedOnRead: u32 = 3;

#[ffi(ghc_lib)]
pub const BlockedOnWrite: u32 = 4;

#[ffi(ghc_lib)]
pub const BlockedOnDelay: u32 = 5;

#[ffi(ghc_lib)]
pub const BlockedOnSTM: u32 = 6;

#[ffi(ghc_lib)]
pub const BlockedOnDoProc: u32 = 7;

#[ffi(ghc_lib)]
pub const BlockedOnCCall: u32 = 10;

#[ffi(ghc_lib)]
pub const BlockedOnCCall_Interruptible: u32 = 11;

#[ffi(ghc_lib)]
pub const BlockedOnMsgThrowTo: u32 = 12;

#[ffi(ghc_lib)]
pub const ThreadMigrating: u32 = 13;

#[ffi(compiler, ghc_lib)]
pub const HeapOverflow: u32 = 1;

#[ffi(compiler, ghc_lib, libraries)]
pub const StackOverflow: u32 = 2;

pub(crate) const ThreadYielding: u32 = 3;

#[ffi(ghc_lib)]
pub const ThreadBlocked: u32 = 4;

#[ffi(ghc_lib)]
pub const ThreadFinished: u32 = 5;

#[ffi(ghc_lib)]
pub const TSO_LOCKED: u32 = 2;

#[ffi(ghc_lib)]
pub const TSO_BLOCKEX: u32 = 4;

#[ffi(ghc_lib)]
pub const TSO_INTERRUPTIBLE: u32 = 8;

#[ffi(ghc_lib)]
pub const TSO_STOPPED_ON_BREAKPOINT: u32 = 16;

#[ffi(ghc_lib)]
pub const TSO_MARKED: u32 = 64;

#[ffi(ghc_lib)]
pub const TSO_SQUEEZED: u32 = 128;

#[ffi(ghc_lib)]
pub const TSO_ALLOC_LIMIT: u32 = 256;

#[ffi(ghc_lib)]
pub const TSO_STOP_NEXT_BREAKPOINT: u32 = 512;

#[ffi(ghc_lib)]
pub const TSO_STOP_AFTER_RETURN: u32 = 1024;

pub(crate) const SPIN_COUNT: u32 = 1000;

pub(crate) const MAX_SPARE_WORKERS: u32 = 6;

pub(crate) const MAX_NUMA_NODES: u32 = 16;

pub(crate) const CLOSURE_DESC_BUFFER_SIZE: u32 = 11;
