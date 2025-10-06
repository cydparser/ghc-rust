use crate::prelude::*;
use crate::stg::types::{
    StgChar, StgDouble, StgFloat, StgInt, StgInt8, StgInt16, StgInt32, StgInt64, StgPtr, StgWord,
    StgWord8, StgWord16, StgWord32, StgWord64,
};

#[cfg(test)]
mod tests;

pub(crate) const HS_CHAR_MIN: u32 = 0;

pub(crate) const HS_CHAR_MAX: u32 = 1114111;

pub(crate) const HS_BOOL_FALSE: u32 = 0;

/// - GHC_PLACES: {libraries}
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

pub(crate) type HsChar = StgChar;

/// - GHC_PLACES: {compiler, libraries, testsuite}
pub type HsInt = StgInt;

/// - GHC_PLACES: {testsuite}
pub type HsInt8 = StgInt8;

/// - GHC_PLACES: {testsuite}
pub type HsInt16 = StgInt16;

/// - GHC_PLACES: {testsuite}
pub type HsInt32 = StgInt32;

/// - GHC_PLACES: {libraries, testsuite}
pub type HsInt64 = StgInt64;

/// - GHC_PLACES: {libraries, testsuite}
pub type HsWord = StgWord;

pub(crate) type HsWord8 = StgWord8;

pub(crate) type HsWord16 = StgWord16;

pub(crate) type HsWord32 = StgWord32;

/// - GHC_PLACES: {compiler, libraries}
pub type HsWord64 = StgWord64;

/// - GHC_PLACES: {libraries, testsuite}
pub type HsFloat = StgFloat;

/// - GHC_PLACES: {libraries}
pub type HsDouble = StgDouble;

/// - GHC_PLACES: {libraries}
pub type HsBool = StgInt;

pub(crate) type HsPtr = *mut c_void;

pub(crate) type HsFunPtr = Option<unsafe extern "C" fn()>;

/// - GHC_PLACES: {testsuite}
pub type HsStablePtr = *mut c_void;

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_init(argc, argv)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_init")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_exit() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_exit()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_exit")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_thread_done() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_thread_done()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_thread_done")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_perform_gc() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_perform_gc()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_perform_gc")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr(sp: HsStablePtr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_free_stable_ptr(sp)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_free_stable_ptr")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_lookup(key: *mut StgWord64) -> StgPtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_spt_lookup(key)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_spt_lookup")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_keys(keys: *mut StgPtr, szKeys: c_int) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_spt_keys(keys, szKeys)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_spt_keys")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_key_count() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_spt_key_count()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_spt_key_count")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_try_putmvar(capability: c_int, sp: HsStablePtr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::hs_try_putmvar(capability, sp)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("hs_try_putmvar")
}
