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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getStablePtr(p: StgPtr) -> StgStablePtr {
    unsafe { transmute(sys::getStablePtr(p)) }
}

#[repr(C)]
pub struct spEntry {
    pub addr: StgPtr,
}

#[cfg(feature = "sys")]
impl From<spEntry> for sys::spEntry {
    fn from(x: spEntry) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for spEntry {
    fn arbitrary(g: &mut Gen) -> Self {
        spEntry {
            addr: Arbitrary::arbitrary(g),
        }
    }
}

static mut stable_ptr_table: *mut spEntry = unsafe { sys::stable_ptr_table };
