use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord64};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib, testsuite)]
#[repr(C)]
pub struct CostCentre_ {
    pub ccID: StgInt,
    pub label: *mut c_char,
    pub module: *mut c_char,
    pub srcloc: *mut c_char,
    pub mem_alloc: StgWord64,
    pub time_ticks: StgWord,
    pub is_caf: StgBool,
    pub link: *mut CostCentre_,
}

#[ffi(compiler, ghc_lib)]
pub type CostCentre = CostCentre_;

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub struct CostCentreStack_ {
    pub ccsID: StgInt,
    pub cc: *mut CostCentre,
    pub prevStack: *mut CostCentreStack_,
    pub indexTable: *mut IndexTable_,
    pub root: *mut CostCentreStack_,
    pub depth: StgWord,
    pub scc_count: StgWord64,
    pub selected: StgWord,
    pub time_ticks: StgWord,
    pub mem_alloc: StgWord64,
    pub inherited_alloc: StgWord64,
    pub inherited_ticks: StgWord,
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

#[ffi(ghc_lib)]
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
