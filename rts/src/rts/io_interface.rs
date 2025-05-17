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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setIOManagerControlFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setIOManagerControlFd(cap_no: u32, fd: c_int) {
    unsafe { sys::setIOManagerControlFd(cap_no, fd) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setTimerManagerControlFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setTimerManagerControlFd(fd: c_int) {
    unsafe { sys::setTimerManagerControlFd(fd) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setIOManagerWakeupFd"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setIOManagerWakeupFd(fd: c_int) {
    unsafe { sys::setIOManagerWakeupFd(fd) }
}
