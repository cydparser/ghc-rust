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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_requestHeapCensus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn requestHeapCensus() {
    unsafe { sys::requestHeapCensus() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn startHeapProfTimer() {
    unsafe { sys::startHeapProfTimer() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stopHeapProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stopHeapProfTimer() {
    unsafe { sys::stopHeapProfTimer() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setUserEra(w: StgWord) {
    unsafe { sys::setUserEra(w) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getUserEra() -> StgWord {
    unsafe { sys::getUserEra() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_incrementUserEra"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn incrementUserEra(w: StgWord) -> StgWord {
    unsafe { sys::incrementUserEra(w) }
}
