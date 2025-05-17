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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn closure_sizeW_(p: *const StgClosure, info: *const StgInfoTable) -> u32 {
    unsafe {
        sys::closure_sizeW_(
            p as *const sys::StgClosure,
            info as *const sys::StgInfoTable,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingClosure(p: *mut StgClosure) {
    unsafe { sys::stg_overwritingClosure(p as *mut sys::StgClosure) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingMutableClosureOfs(p: *mut StgClosure, offset: u32) {
    unsafe { sys::stg_overwritingMutableClosureOfs(p as *mut sys::StgClosure, offset) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn stg_overwritingClosureSize(p: *mut StgClosure, size: u32) {
    unsafe { sys::stg_overwritingClosureSize(p as *mut sys::StgClosure, size) }
}
