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
impl Arbitrary for snEntry {
    fn arbitrary(g: &mut Gen) -> Self {
        snEntry {
            addr: Arbitrary::arbitrary(g),
            old: Arbitrary::arbitrary(g),
            sn_obj: Arbitrary::arbitrary(g),
        }
    }
}

static mut stable_name_table: *mut snEntry = unsafe { sys::stable_name_table };
