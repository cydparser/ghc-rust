use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_updateRemembSetPushClosure_")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn updateRemembSetPushClosure_(reg: *mut StgRegTable, p: *mut StgClosure_) {
    unsafe {
        sys::updateRemembSetPushClosure_(reg as *mut sys::StgRegTable, p as *mut sys::StgClosure_)
    }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_updateRemembSetPushThunk_")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn updateRemembSetPushThunk_(reg: *mut StgRegTable, p: *mut StgThunk_) {
    unsafe {
        sys::updateRemembSetPushThunk_(reg as *mut sys::StgRegTable, p as *mut sys::StgThunk_)
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stg_copyArray_barrier"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_copyArray_barrier() -> StgFunPtr {
    unsafe { sys::stg_copyArray_barrier() }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_nonmoving_write_barrier_enabled")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut nonmoving_write_barrier_enabled: StgWord = 0;
