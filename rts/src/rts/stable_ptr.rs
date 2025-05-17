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

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getStablePtr(p: StgPtr) -> StgStablePtr {
    unsafe { sys::getStablePtr(p) }
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
#[derive(Clone)]
struct spEntryOwned {}
#[cfg(test)]
impl Arbitrary for spEntryOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        spEntryOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct spEntryPointees {
    pub addr: StgPtr,
}

#[cfg(test)]
impl Arbitrary for spEntryPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        spEntryPointees {
            addr: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for spEntry {
    type Owned = spEntryOwned;
    type Pointees = spEntryPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            addr: unsafe { &raw mut (*pointees).addr },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

static mut stable_ptr_table: *mut spEntry = null_mut();
