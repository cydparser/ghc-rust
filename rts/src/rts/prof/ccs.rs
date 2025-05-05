use std::{ffi::c_char, mem::transmute};

#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::stg::types::{StgBool, StgInt, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

pub type CostCentre = CostCentre_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct CostCentre_ {
    pub ccID: StgInt,
    pub label: *mut c_char,
    pub module: *mut c_char,
    pub srcloc: *mut c_char,
    pub mem_alloc: StgWord64,
    pub time_ticks: StgWord,
    pub is_caf: StgBool,
    pub link: *mut CostCentre_,
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
pub(crate) struct CostCentreStack_ {
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
