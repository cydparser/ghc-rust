use crate::ffi::stg::types::{StgStablePtr, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert(key: *mut StgWord64, spe_closure: *mut c_void) {
    sys! {
        hs_spt_insert(key, spe_closure)
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert_stableptr(key: *mut StgWord64, entry: *mut StgStablePtr) {
    sys! {
        hs_spt_insert_stableptr(key, entry)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_remove(key: *mut StgWord64) {
    sys! {
        hs_spt_remove(key)
    }
}
