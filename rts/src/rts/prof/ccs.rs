use crate::prelude::*;
use crate::stg::types::{StgBool, StgInt, StgWord, StgWord64};

#[cfg(test)]
mod tests;

pub type CostCentre = CostCentre_;

#[repr(C)]
///cbindgen:no-export
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

#[cfg(feature = "sys")]
impl From<CostCentre_> for sys::CostCentre_ {
    fn from(x: CostCentre_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type CostCentreStack = CostCentreStack_;

#[repr(C)]
///cbindgen:no-export
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

#[cfg(feature = "sys")]
impl From<CostCentreStack_> for sys::CostCentreStack_ {
    fn from(x: CostCentreStack_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_stopProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stopProfTimer() {
    unsafe { sys::stopProfTimer() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startProfTimer"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn startProfTimer() {
    unsafe { sys::startProfTimer() }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct IndexTable_ {
    pub _address: u8,
}

#[cfg(feature = "sys")]
impl From<IndexTable_> for sys::IndexTable_ {
    fn from(x: IndexTable_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for IndexTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        IndexTable_ {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
