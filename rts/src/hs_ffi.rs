use crate::ffi::rts::adjustor::freeHaskellFunctionPtr;
use crate::ffi::rts::storage::gc::performBlockingMajorGC;
use crate::ffi::stg::types::{
    STG_INT_MAX, STG_INT_MIN, STG_INT8_MAX, STG_INT8_MIN, STG_INT16_MAX, STG_INT16_MIN,
    STG_INT32_MAX, STG_INT32_MIN, STG_INT64_MAX, STG_INT64_MIN, STG_WORD_MAX, STG_WORD8_MAX,
    STG_WORD16_MAX, STG_WORD32_MAX, STG_WORD64_MAX, StgChar, StgDouble, StgFloat, StgInt, StgInt8,
    StgInt16, StgInt32, StgInt64, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::prelude::*;
use crate::rts_api::setProgArgv;
use crate::stable_ptr::{freeStablePtr, freeStablePtrUnsafe, stablePtrLock, stablePtrUnlock};
use crate::task::freeMyTask;

#[cfg(test)]
mod tests;

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

#[ffi(docs, testsuite)]
pub type HsFunPtr = Option<unsafe extern "C" fn() -> ()>;

#[ffi(docs, ghc_lib, testsuite)]
pub type HsStablePtr = *mut c_void;

pub(crate) const HS_CHAR_MIN: u32 = 0;
pub(crate) const HS_CHAR_MAX: u32 = 0x10FFFF;

#[ffi]
pub const HS_BOOL_FALSE: u32 = 0;
#[ffi(docs, ghc_lib)]
pub const HS_BOOL_TRUE: u32 = 1;

pub(crate) const HS_BOOL_MIN: u32 = 0;
pub(crate) const HS_BOOL_MAX: u32 = 1;

pub(crate) const HS_INT_MIN: StgInt = STG_INT_MIN;
pub(crate) const HS_INT_MAX: StgInt = STG_INT_MAX;

pub(crate) const HS_WORD_MAX: StgWord = STG_WORD_MAX;

pub(crate) const HS_INT8_MIN: StgInt8 = STG_INT8_MIN;
pub(crate) const HS_INT8_MAX: StgInt8 = STG_INT8_MAX;

pub(crate) const HS_INT16_MIN: StgInt16 = STG_INT16_MIN;
pub(crate) const HS_INT16_MAX: StgInt16 = STG_INT16_MAX;

pub(crate) const HS_INT32_MIN: StgInt32 = STG_INT32_MIN;
pub(crate) const HS_INT32_MAX: StgInt32 = STG_INT32_MAX;

pub(crate) const HS_INT64_MIN: StgInt64 = STG_INT64_MIN;
pub(crate) const HS_INT64_MAX: StgInt64 = STG_INT64_MAX;

pub(crate) const HS_WORD8_MAX: StgWord8 = STG_WORD8_MAX;
pub(crate) const HS_WORD16_MAX: StgWord16 = STG_WORD16_MAX;

pub(crate) const HS_WORD32_MAX: StgWord32 = STG_WORD32_MAX;
pub(crate) const HS_WORD64_MAX: StgWord64 = STG_WORD64_MAX;

unsafe fn hs_set_argv(mut argc: i32, mut argv: *mut *mut c_char) {
    setProgArgv(argc, argv);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_perform_gc() {
    performBlockingMajorGC();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_ptr_table() {
    stablePtrLock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_tables() {
    stablePtrLock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_ptr_table() {
    stablePtrUnlock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_tables() {
    stablePtrUnlock();
}

#[ffi(docs, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr(mut sp: HsStablePtr) {
    freeStablePtr(sp);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr_unsafe(mut sp: HsStablePtr) {
    freeStablePtrUnsafe(sp);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_fun_ptr(mut fp: HsFunPtr) {
    freeHaskellFunctionPtr(transmute::<HsFunPtr, *mut c_void>(fp));
}

#[ffi(docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_thread_done() {
    freeMyTask();
}
