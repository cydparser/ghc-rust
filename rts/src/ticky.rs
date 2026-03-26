use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::rts::types::StgInfoTable;
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::ffi::stg::types::{StgInt, StgWord};
use crate::prelude::*;
use crate::ticky::{_StgEntCounter, StgEntCounter};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct _StgEntCounter {
    pub(crate) registeredp: StgWord,
    pub(crate) arity: StgInt,
    pub(crate) allocd: StgInt,
    pub(crate) str: *mut c_char,
    pub(crate) arg_kinds: *mut c_char,
    pub(crate) ticky_json: *mut c_char,
    pub(crate) info: *mut StgInfoTable,
    pub(crate) entry_count: StgInt,
    pub(crate) allocs: StgInt,
    pub(crate) link: *mut _StgEntCounter,
}

#[ffi(compiler)]
pub type StgEntCounter = _StgEntCounter;

pub(crate) static mut top_ct: StgEntCounter = _StgEntCounter {
    registeredp: 0 as StgWord,
    arity: 0 as StgInt,
    allocd: 0 as StgInt,
    str: b"TOP\0" as *const u8 as *const c_char as *mut c_char,
    arg_kinds: b"\0" as *const u8 as *const c_char as *mut c_char,
    ticky_json: b"\0" as *const u8 as *const c_char as *mut c_char,
    info: null::<StgInfoTable>() as *mut StgInfoTable,
    entry_count: 0 as StgInt,
    allocs: 0 as StgInt,
    link: null::<_StgEntCounter>() as *mut _StgEntCounter,
};

pub(crate) static mut ticky_entry_ctrs: *mut StgEntCounter =
    null::<StgEntCounter>() as *mut StgEntCounter;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn requestTickyCounterSamples() {}
