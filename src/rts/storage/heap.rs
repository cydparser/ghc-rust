use crate::rts::storage::closures;
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
pub(crate) unsafe fn heap_view_closurePtrs(
    cap: *mut Capability,
    closure: *mut StgClosure,
) -> *mut StgMutArrPtrs {
    unsafe {
        transmute(sys::heap_view_closurePtrs(
            &mut cap.into(),
            &mut closure.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn heap_view_closure_ptrs_in_pap_payload(
    ptrs: *mut *mut StgClosure,
    nptrs: *mut StgWord,
    fun: *mut StgClosure,
    payload: *mut *mut StgClosure,
    size: StgWord,
) {
    unsafe {
        transmute(sys::heap_view_closure_ptrs_in_pap_payload(
            &mut &mut ptrs.into(),
            &mut nptrs.into(),
            &mut fun.into(),
            &mut &mut payload.into(),
            size.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn heap_view_closureSize(closure: *mut StgClosure) -> StgWord {
    unsafe { transmute(sys::heap_view_closureSize(&mut closure.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn collect_pointers(
    closure: *mut StgClosure,
    ptrs: *mut *mut StgClosure,
) -> StgWord {
    unsafe {
        transmute(sys::collect_pointers(
            &mut closure.into(),
            &mut &mut ptrs.into(),
        ))
    }
}
