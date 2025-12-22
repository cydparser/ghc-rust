use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::stg::types::{
    StgChar, StgDouble, StgFloat, StgInt, StgInt8, StgInt16, StgInt32, StgInt64, StgPtr, StgWord,
    StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::prelude::*;

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

#[ffi(compiler, ghc_lib)]
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

#[ffi(compiler, ghc_lib)]
pub type HsWord8 = StgWord8;

#[ffi(ghc_lib)]
pub type HsWord16 = StgWord16;

#[ffi(ghc_lib)]
pub type HsWord32 = StgWord32;

#[ffi(compiler, ghc_lib)]
pub type HsWord64 = StgWord64;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
pub type HsFloat = StgFloat;

#[ffi(compiler, ghc_lib, libraries)]
pub type HsDouble = StgDouble;

#[ffi(compiler, docs, driver, ghc_lib, testsuite, utils)]
pub type HsBool = StgInt;

pub(crate) type HsPtr = *mut c_void;

pub(crate) type HsFunPtr = Option<unsafe extern "C" fn()>;

#[ffi(docs, ghc_lib, testsuite)]
pub type HsStablePtr = *mut c_void;

#[ffi(docs, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    sys! {
        hs_init(argc, argv)
    }
}

#[ffi(docs, ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit() {
    sys! {
        hs_exit()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit_nowait() {
    sys! {
        hs_exit_nowait()
    }
}

#[ffi(docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_thread_done() {
    sys! {
        hs_thread_done()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_perform_gc() {
    sys! {
        hs_perform_gc()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_ptr_table() {
    sys! {
        hs_lock_stable_ptr_table()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_tables() {
    sys! {
        hs_lock_stable_tables()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_ptr_table() {
    sys! {
        hs_unlock_stable_ptr_table()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_tables() {
    sys! {
        hs_unlock_stable_tables()
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr_unsafe(sp: HsStablePtr) {
    sys! {
        hs_free_stable_ptr_unsafe(sp)
    }
}

#[ffi(docs, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr(sp: HsStablePtr) {
    sys! {
        hs_free_stable_ptr(sp)
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_fun_ptr(fp: HsFunPtr) {
    sys! {
        hs_free_fun_ptr(fp)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_lookup(key: *mut StgWord64) -> StgPtr {
    sys! {
        hs_spt_lookup(key)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_keys(keys: *mut StgPtr, szKeys: c_int) -> c_int {
    sys! {
        hs_spt_keys(keys, szKeys)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_key_count() -> c_int {
    sys! {
        hs_spt_key_count()
    }
}

#[ffi(docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_try_putmvar(capability: c_int, sp: HsStablePtr) {
    sys! {
        hs_try_putmvar(capability, sp)
    }
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_try_putmvar_with_value(
    capability: c_int,
    sp: HsStablePtr,
    value: *mut StgClosure,
) {
    sys! {
        hs_try_putmvar_with_value(capability, sp, value as * mut sys::StgClosure)
    }
}
