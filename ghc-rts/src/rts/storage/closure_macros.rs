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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closure_sizeW_(p: *const StgClosure, info: *const StgInfoTable) -> u32 {
    unsafe { transmute(sys::closure_sizeW_(p, info)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingClosure(p: *mut StgClosure) {
    unsafe { sys::stg_overwritingClosure(p) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingMutableClosureOfs(p: *mut StgClosure, offset: u32) {
    unsafe { sys::stg_overwritingMutableClosureOfs(p, offset) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingClosureSize(p: *mut StgClosure, size: u32) {
    unsafe { sys::stg_overwritingClosureSize(p, size) }
}
