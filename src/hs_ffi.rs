use std::{
    ffi::{c_char, c_int, c_void},
    mem::transmute,
};

#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::stg::types::{
    StgChar, StgDouble, StgFloat, StgInt, StgInt16, StgInt32, StgInt64, StgInt8, StgPtr, StgWord,
    StgWord16, StgWord32, StgWord64, StgWord8,
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

mod tests;

pub(crate) const HS_CHAR_MIN: u32 = 0;

pub(crate) const HS_CHAR_MAX: u32 = 1114111;

pub(crate) const HS_BOOL_FALSE: u32 = 0;

pub const HS_BOOL_TRUE: u32 = 1;

pub(crate) const HS_BOOL_MIN: u32 = 0;

pub(crate) const HS_BOOL_MAX: u32 = 1;

pub(crate) const HS_INT8_MIN: i32 = -128;

pub(crate) const HS_INT8_MAX: u32 = 127;

pub(crate) const HS_INT16_MIN: i32 = -32768;

pub(crate) const HS_INT16_MAX: u32 = 32767;

pub(crate) const HS_INT32_MIN: i32 = -2147483648;

pub(crate) const HS_INT32_MAX: u32 = 2147483647;

pub(crate) const HS_WORD8_MAX: u32 = 255;

pub(crate) const HS_WORD16_MAX: u32 = 65535;

pub(crate) const HS_WORD32_MAX: u32 = 4294967295;

pub type HsChar = StgChar;

pub type HsInt = StgInt;

pub type HsInt8 = StgInt8;

pub type HsInt16 = StgInt16;

pub type HsInt32 = StgInt32;

pub type HsInt64 = StgInt64;

pub type HsWord = StgWord;

pub type HsWord8 = StgWord8;

pub type HsWord16 = StgWord16;

pub type HsWord32 = StgWord32;

pub type HsWord64 = StgWord64;

pub type HsFloat = StgFloat;

pub type HsDouble = StgDouble;

pub type HsBool = StgInt;

pub(crate) type HsPtr = *mut c_void;

pub(crate) type HsFunPtr = Option<unsafe extern "C" fn()>;

pub type HsStablePtr = *mut c_void;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    unsafe { sys::hs_init(argc, argv) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_exit() {
    unsafe { sys::hs_exit() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_exit_nowait() {
    unsafe { sys::hs_exit_nowait() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_set_argv(argc: c_int, argv: *mut *mut c_char) {
    unsafe { sys::hs_set_argv(argc, argv) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_thread_done() {
    unsafe { sys::hs_thread_done() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_restoreConsoleCP() {
    unsafe { sys::hs_restoreConsoleCP() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_perform_gc() {
    unsafe { sys::hs_perform_gc() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_lock_stable_ptr_table() {
    unsafe { sys::hs_lock_stable_ptr_table() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_lock_stable_tables() {
    unsafe { sys::hs_lock_stable_tables() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_unlock_stable_ptr_table() {
    unsafe { sys::hs_unlock_stable_ptr_table() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_unlock_stable_tables() {
    unsafe { sys::hs_unlock_stable_tables() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_free_stable_ptr_unsafe(sp: HsStablePtr) {
    unsafe { sys::hs_free_stable_ptr_unsafe(sp) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_free_stable_ptr(sp: HsStablePtr) {
    unsafe { sys::hs_free_stable_ptr(sp) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_free_fun_ptr(fp: HsFunPtr) {
    unsafe { sys::hs_free_fun_ptr(fp) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_lookup(key: *mut StgWord64) -> StgPtr {
    unsafe { sys::hs_spt_lookup(key) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_keys(keys: *mut StgPtr, szKeys: c_int) -> c_int {
    unsafe { sys::hs_spt_keys(keys, szKeys) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_key_count() -> c_int {
    unsafe { transmute(sys::hs_spt_key_count()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_try_putmvar(capability: c_int, sp: HsStablePtr) {
    unsafe { sys::hs_try_putmvar(capability, sp) }
}
