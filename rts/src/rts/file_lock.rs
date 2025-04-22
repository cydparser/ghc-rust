use crate::stg;
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
pub unsafe extern "C" fn lockFile(
    id: StgWord64,
    dev: StgWord64,
    ino: StgWord64,
    for_writing: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::lockFile(id, dev, ino, for_writing)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unlockFile(id: StgWord64) -> ::core::ffi::c_int {
    unsafe { transmute(sys::unlockFile(id)) }
}
