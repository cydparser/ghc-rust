use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::{StgBool, StgInt, StgWord, StgWord64};
use crate::prelude::*;

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

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static mut CCS_MAIN: [CostCentreStack; 0] = [];

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
pub static mut CCS_SYSTEM: [CostCentreStack; 0] = [];

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut CCS_DONT_CARE: [CostCentreStack; 0] = [];

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut era: c_uint = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut user_era: StgWord = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn pushCostCentre(
    arg1: *mut CostCentreStack,
    arg2: *mut CostCentre,
) -> *mut CostCentreStack {
    sys! {
        pushCostCentre(arg1 as * mut sys::CostCentreStack, arg2 as * mut sys::CostCentre)
        .cast()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn enterFunCCS(reg: *mut StgRegTable, arg1: *mut CostCentreStack) {
    sys! {
        enterFunCCS(reg as * mut sys::StgRegTable, arg1 as * mut sys::CostCentreStack)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn mkCostCentre(
    label: *mut c_char,
    module: *mut c_char,
    srcloc: *mut c_char,
) -> *mut CostCentre {
    sys! {
        mkCostCentre(label, module, srcloc).cast()
    }
}
