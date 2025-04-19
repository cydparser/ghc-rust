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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setIOManagerControlFd(cap_no: u32, fd: ::core::ffi::c_int) {
    unsafe { sys::setIOManagerControlFd(cap_no, fd) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setTimerManagerControlFd(fd: ::core::ffi::c_int) {
    unsafe { sys::setTimerManagerControlFd(fd) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setIOManagerWakeupFd(fd: ::core::ffi::c_int) {
    unsafe { sys::setIOManagerWakeupFd(fd) }
}
