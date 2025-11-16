use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn heap_view_closureSize(closure: *mut StgClosure) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::heap_view_closureSize(closure as *mut sys::StgClosure)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("heap_view_closureSize")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn collect_pointers(
    closure: *mut StgClosure,
    ptrs: *mut *mut StgClosure,
) -> StgWord {
    #[cfg(feature = "sys")]
    unsafe {
        sys::collect_pointers(
            closure as *mut sys::StgClosure,
            ptrs as *mut *mut sys::StgClosure,
        )
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("collect_pointers")
}
