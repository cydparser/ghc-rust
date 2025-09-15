use crate::capability::Capability;
use crate::prelude::*;
use crate::rts::storage::closures::{StgClosure, StgMutArrPtrs};
use crate::stg::types::StgWord;

#[cfg(test)]
mod tests;

#[instrument]
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

#[instrument]
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
#[instrument]
pub unsafe extern "C" fn heap_view_closureSize(closure: *mut StgClosure) -> StgWord {
    unsafe { sys::heap_view_closureSize(closure as *mut sys::StgClosure) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_collect_pointers"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
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
