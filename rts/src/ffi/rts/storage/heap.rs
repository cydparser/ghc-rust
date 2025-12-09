use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn heap_view_closureSize(closure: *mut StgClosure) -> StgWord {
    sys! {
        heap_view_closureSize(closure as * mut sys::StgClosure)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn collect_pointers(
    closure: *mut StgClosure,
    ptrs: *mut *mut StgClosure,
) -> StgWord {
    sys! {
        collect_pointers(closure as * mut sys::StgClosure, ptrs as * mut * mut
        sys::StgClosure)
    }
}
