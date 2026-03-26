use crate::ffi::stg::types::{
    StgChar, StgDouble, StgFloat, StgInt, StgInt8, StgInt16, StgInt32, StgInt64, StgWord, StgWord8,
    StgWord16, StgWord32, StgWord64,
};
pub use crate::hs_ffi::{
    HsFunPtr, HsStablePtr, hs_free_fun_ptr, hs_free_stable_ptr, hs_free_stable_ptr_unsafe,
    hs_lock_stable_ptr_table, hs_lock_stable_tables, hs_perform_gc, hs_thread_done,
    hs_unlock_stable_ptr_table, hs_unlock_stable_tables,
};
use crate::prelude::*;
pub use crate::rts_api::{hs_try_putmvar, hs_try_putmvar_with_value};
pub use crate::rts_startup::{hs_exit, hs_exit_nowait, hs_init};
pub use crate::static_ptr_table::{hs_spt_key_count, hs_spt_keys, hs_spt_lookup};

#[cfg(test)]
mod tests;

pub(crate) const HS_CHAR_MIN: u32 = 0;

pub(crate) const HS_CHAR_MAX: u32 = 1114111;

pub(crate) const HS_BOOL_FALSE: u32 = 0;

#[ffi(docs, ghc_lib)]
pub const HS_BOOL_TRUE: u32 = 1;

pub(crate) const HS_BOOL_MIN: u32 = 0;

pub(crate) const HS_BOOL_MAX: u32 = 1;

pub(crate) const HS_INT_MIN: i64 = -9223372036854775808;

pub(crate) const HS_INT_MAX: u64 = 9223372036854775807;

pub(crate) const HS_WORD_MAX: i32 = -1;

pub(crate) const HS_INT8_MIN: i32 = -128;

pub(crate) const HS_INT8_MAX: u32 = 127;

pub(crate) const HS_INT16_MIN: i32 = -32768;

pub(crate) const HS_INT16_MAX: u32 = 32767;

pub(crate) const HS_INT32_MIN: i32 = -2147483648;

pub(crate) const HS_INT32_MAX: u32 = 2147483647;

pub(crate) const HS_INT64_MIN: i64 = -9223372036854775808;

pub(crate) const HS_INT64_MAX: u64 = 9223372036854775807;

pub(crate) const HS_WORD8_MAX: u32 = 255;

pub(crate) const HS_WORD16_MAX: u32 = 65535;

pub(crate) const HS_WORD32_MAX: u32 = 4294967295;

pub(crate) const HS_WORD64_MAX: i32 = -1;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HsChar = StgChar;

#[ffi(compiler, docs, ghc_lib, libraries, testsuite)]
pub type HsInt = StgInt;

#[ffi(ghc_lib, testsuite)]
pub type HsInt8 = StgInt8;

#[ffi(ghc_lib, testsuite)]
pub type HsInt16 = StgInt16;

#[ffi(ghc_lib, testsuite)]
pub type HsInt32 = StgInt32;

#[ffi(ghc_lib, testsuite)]
pub type HsInt64 = StgInt64;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HsWord = StgWord;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HsWord8 = StgWord8;

#[ffi(ghc_lib, testsuite)]
pub type HsWord16 = StgWord16;

#[ffi(ghc_lib, testsuite)]
pub type HsWord32 = StgWord32;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HsWord64 = StgWord64;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
pub type HsFloat = StgFloat;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
pub type HsDouble = StgDouble;

#[ffi(compiler, docs, driver, ghc_lib, libraries, testsuite, utils)]
pub type HsBool = StgInt;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HsPtr = *mut c_void;
