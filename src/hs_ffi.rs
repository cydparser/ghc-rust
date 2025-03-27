use crate::stg::types;
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

pub(crate) type HsPtr = *mut ::core::ffi::c_void;

pub(crate) type HsFunPtr = ::core::option::Option<unsafe extern "C" fn()>;

pub type HsStablePtr = *mut ::core::ffi::c_void;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_init(
    argc: *mut ::core::ffi::c_int,
    argv: *mut *mut *mut ::core::ffi::c_char,
) {
    unsafe { transmute(sys::hs_init(&mut argc.into(), &mut &mut &mut argv.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_exit() {
    unsafe { transmute(sys::hs_exit()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_exit_nowait() {
    unsafe { transmute(sys::hs_exit_nowait()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_set_argv(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) {
    unsafe { transmute(sys::hs_set_argv(argc.into(), &mut &mut argv.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_thread_done() {
    unsafe { transmute(sys::hs_thread_done()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_restoreConsoleCP() {
    unsafe { transmute(sys::hs_restoreConsoleCP()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_perform_gc() {
    unsafe { transmute(sys::hs_perform_gc()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_lock_stable_ptr_table() {
    unsafe { transmute(sys::hs_lock_stable_ptr_table()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_lock_stable_tables() {
    unsafe { transmute(sys::hs_lock_stable_tables()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_unlock_stable_ptr_table() {
    unsafe { transmute(sys::hs_unlock_stable_ptr_table()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_unlock_stable_tables() {
    unsafe { transmute(sys::hs_unlock_stable_tables()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_free_stable_ptr_unsafe(sp: HsStablePtr) {
    unsafe { transmute(sys::hs_free_stable_ptr_unsafe(sp.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_free_stable_ptr(sp: HsStablePtr) {
    unsafe { transmute(sys::hs_free_stable_ptr(sp.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn hs_free_fun_ptr(fp: HsFunPtr) {
    unsafe { transmute(sys::hs_free_fun_ptr(fp.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_lookup(key: *mut StgWord64) -> StgPtr {
    unsafe { transmute(sys::hs_spt_lookup(&mut key.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_keys(
    keys: *mut StgPtr,
    szKeys: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::hs_spt_keys(&mut keys.into(), szKeys.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_key_count() -> ::core::ffi::c_int {
    unsafe { transmute(sys::hs_spt_key_count()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_try_putmvar(capability: ::core::ffi::c_int, sp: HsStablePtr) {
    unsafe { transmute(sys::hs_try_putmvar(capability.into(), sp.into())) }
}
