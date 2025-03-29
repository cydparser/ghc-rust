use std::mem::transmute;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::stg::types::{StgBool, StgInt, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

#[repr(C)]
pub struct CostCentre_ {
    pub ccID: StgInt,
    pub label: *mut ::core::ffi::c_char,
    pub module: *mut ::core::ffi::c_char,
    pub srcloc: *mut ::core::ffi::c_char,
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

pub type CostCentre = CostCentre_;

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

#[cfg(feature = "sys")]
impl From<CostCentreStack_> for sys::CostCentreStack_ {
    fn from(x: CostCentreStack_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type CostCentreStack = CostCentreStack_;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn stopProfTimer() {
    unsafe { transmute(sys::stopProfTimer()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn startProfTimer() {
    unsafe { transmute(sys::startProfTimer()) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexTable_ {
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
