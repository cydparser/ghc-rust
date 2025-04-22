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
pub unsafe extern "C" fn hs_spt_insert(key: *mut StgWord64, spe_closure: *mut ::core::ffi::c_void) {
    unsafe { sys::hs_spt_insert(key, spe_closure) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_insert_stableptr(key: *mut StgWord64, entry: *mut StgStablePtr) {
    unsafe { sys::hs_spt_insert_stableptr(key, entry) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_remove(key: *mut StgWord64) {
    unsafe { sys::hs_spt_remove(key) }
}
