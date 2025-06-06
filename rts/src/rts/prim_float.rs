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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust___int_encodeDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __int_encodeDouble(j: I_, e: I_) -> StgDouble {
    unsafe { sys::__int_encodeDouble(j, e) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn __int_encodeFloat(j: I_, e: I_) -> StgFloat {
    unsafe { sys::__int_encodeFloat(j, e) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust___word_encodeDouble"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __word_encodeDouble(j: W_, e: I_) -> StgDouble {
    unsafe { sys::__word_encodeDouble(j, e) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn __word_encodeFloat(j: W_, e: I_) -> StgFloat {
    unsafe { sys::__word_encodeFloat(j, e) }
}
