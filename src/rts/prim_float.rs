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
pub unsafe extern "C" fn __int_encodeDouble(j: I_, e: I_) -> StgDouble {
    unsafe { transmute(sys::__int_encodeDouble(j.into(), e.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn __int_encodeFloat(j: I_, e: I_) -> StgFloat {
    unsafe { transmute(sys::__int_encodeFloat(j.into(), e.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn __word_encodeDouble(j: W_, e: I_) -> StgDouble {
    unsafe { transmute(sys::__word_encodeDouble(j.into(), e.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn __word_encodeFloat(j: W_, e: I_) -> StgFloat {
    unsafe { transmute(sys::__word_encodeFloat(j.into(), e.into())) }
}
