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
pub(crate) unsafe fn heap_view_closurePtrs(
    cap: *mut Capability,
    closure: *mut StgClosure,
) -> *mut StgMutArrPtrs {
    unsafe {
        transmute(sys::heap_view_closurePtrs(
            cap as *mut sys::Capability,
            closure as *mut sys::StgClosure,
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
        sys::heap_view_closure_ptrs_in_pap_payload(
            ptrs as *mut *mut sys::StgClosure,
            nptrs,
            fun as *mut sys::StgClosure,
            payload as *mut *mut sys::StgClosure,
            size,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_heap_view_closureSize"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn heap_view_closureSize(closure: *mut StgClosure) -> StgWord {
    unsafe { sys::heap_view_closureSize(closure as *mut sys::StgClosure) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_collect_pointers"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn collect_pointers(
    closure: *mut StgClosure,
    ptrs: *mut *mut StgClosure,
) -> StgWord {
    unsafe {
        sys::collect_pointers(
            closure as *mut sys::StgClosure,
            ptrs as *mut *mut sys::StgClosure,
        )
    }
}
