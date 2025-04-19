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
pub unsafe extern "C" fn __hscore_get_saved_termios(
    fd: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::__hscore_get_saved_termios(fd)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __hscore_set_saved_termios(
    fd: ::core::ffi::c_int,
    ts: *mut ::core::ffi::c_void,
) {
    unsafe { sys::__hscore_set_saved_termios(fd, ts) }
}
