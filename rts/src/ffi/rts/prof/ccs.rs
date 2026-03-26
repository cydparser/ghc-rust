use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord64};
use crate::prelude::*;
pub use crate::prof_heap::{era, user_era};
pub use crate::profiling::{
    CCS_DONT_CARE, CCS_MAIN, CCS_SYSTEM, enterFunCCS, mkCostCentre, pushCostCentre,
};
pub use crate::proftimer::{startProfTimer, stopProfTimer};

#[cfg(test)]
mod tests;

pub(crate) const CC_IS_CAF: u32 = 1;

pub(crate) const CC_NOT_CAF: u32 = 0;

#[ffi(ghc_lib)]
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

#[ffi(compiler, ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct IndexTable_ {
    pub cc: *mut CostCentre,
    pub ccs: *mut CostCentreStack,
    pub next: *mut IndexTable_,
    pub back_edge: bool,
}

#[ffi(ghc_lib)]
pub type IndexTable = IndexTable_;
