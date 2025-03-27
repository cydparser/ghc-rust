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
pub unsafe extern "C" fn updateRemembSetPushClosure_(reg: *mut StgRegTable, p: *mut StgClosure_) {
    unsafe {
        transmute(sys::updateRemembSetPushClosure_(
            &mut reg.into(),
            &mut p.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn updateRemembSetPushThunk_(reg: *mut StgRegTable, p: *mut StgThunk_) {
    unsafe {
        transmute(sys::updateRemembSetPushThunk_(
            &mut reg.into(),
            &mut p.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stg_copyArray_barrier() -> StgFunPtr {
    unsafe { transmute(sys::stg_copyArray_barrier()) }
}

#[unsafe(no_mangle)]
pub static mut nonmoving_write_barrier_enabled: StgWord = sys::nonmoving_write_barrier_enabled;
