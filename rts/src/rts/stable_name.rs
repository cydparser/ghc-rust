use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
pub struct snEntry {
    pub addr: StgPtr,
    pub old: StgPtr,
    pub sn_obj: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<snEntry> for sys::snEntry {
    fn from(x: snEntry) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct snEntryOwned {}
#[cfg(test)]
impl Arbitrary for snEntryOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        snEntryOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct snEntryPointees {
    pub addr: StgPtr,
    pub old: StgPtr,
    pub sn_obj: StgClosure,
}

#[cfg(test)]
impl Arbitrary for snEntryPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        snEntryPointees {
            addr: Arbitrary::arbitrary(g),
            old: Arbitrary::arbitrary(g),
            sn_obj: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for snEntry {
    type Owned = snEntryOwned;
    type Pointees = snEntryPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            addr: unsafe { &raw mut (*pointees).addr },
            old: unsafe { &raw mut (*pointees).old },
            sn_obj: unsafe { &raw mut (*pointees).sn_obj },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

static mut stable_name_table: *mut snEntry = null_mut();
