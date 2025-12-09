use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct CostCentre_ {
    ccID: StgInt,
    label: *mut c_char,
    module: *mut c_char,
    srcloc: *mut c_char,
    mem_alloc: StgWord64,
    time_ticks: StgWord,
    is_caf: StgBool,
    link: *mut CostCentre_,
}

#[ffi(compiler, ghc_lib)]
pub type CostCentre = CostCentre_;

/// cbindgen:no-export
#[repr(C)]
pub struct CostCentreStack_ {
    ccsID: StgInt,
    cc: *mut CostCentre,
    prevStack: *mut CostCentreStack_,
    indexTable: *mut IndexTable_,
    root: *mut CostCentreStack_,
    depth: StgWord,
    scc_count: StgWord64,
    selected: StgWord,
    time_ticks: StgWord,
    mem_alloc: StgWord64,
    inherited_alloc: StgWord64,
    inherited_ticks: StgWord,
}

#[ffi(compiler, ghc_lib)]
pub type CostCentreStack = CostCentreStack_;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopProfTimer() {
    sys! {
        stopProfTimer()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startProfTimer() {
    sys! {
        startProfTimer()
    }
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct IndexTable_ {
    _address: u8,
}

#[cfg(test)]
impl Arbitrary for IndexTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        IndexTable_ {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
