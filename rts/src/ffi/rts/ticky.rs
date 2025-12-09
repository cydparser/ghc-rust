use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _StgEntCounter {
    registeredp: StgWord,
    arity: StgInt,
    allocd: StgInt,
    str_: *mut c_char,
    arg_kinds: *mut c_char,
    ticky_json: *mut c_char,
    info: *mut StgInfoTable,
    entry_count: StgInt,
    allocs: StgInt,
    link: *mut _StgEntCounter,
}

#[ffi(compiler)]
pub type StgEntCounter = _StgEntCounter;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestTickyCounterSamples() {
    sys! {
        requestTickyCounterSamples()
    }
}
