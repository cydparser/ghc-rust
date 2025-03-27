use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
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

#[cfg(test)]
impl Arbitrary for CostCentre_ {
    fn arbitrary(g: &mut Gen) -> Self {
        CostCentre_ {
            ccID: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            module: Arbitrary::arbitrary(g),
            srcloc: Arbitrary::arbitrary(g),
            mem_alloc: Arbitrary::arbitrary(g),
            time_ticks: Arbitrary::arbitrary(g),
            is_caf: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for CostCentreStack_ {
    fn arbitrary(g: &mut Gen) -> Self {
        CostCentreStack_ {
            ccsID: Arbitrary::arbitrary(g),
            cc: Arbitrary::arbitrary(g),
            prevStack: Arbitrary::arbitrary(g),
            indexTable: Arbitrary::arbitrary(g),
            root: Arbitrary::arbitrary(g),
            depth: Arbitrary::arbitrary(g),
            scc_count: Arbitrary::arbitrary(g),
            selected: Arbitrary::arbitrary(g),
            time_ticks: Arbitrary::arbitrary(g),
            mem_alloc: Arbitrary::arbitrary(g),
            inherited_alloc: Arbitrary::arbitrary(g),
            inherited_ticks: Arbitrary::arbitrary(g),
        }
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
