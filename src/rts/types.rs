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

pub type nat = ::core::ffi::c_uint;

pub type StgClosure = StgClosure_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgInfoTable_ {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<StgInfoTable_> for sys::StgInfoTable_ {
    fn from(x: StgInfoTable_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgInfoTable_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgInfoTable_ {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgInfoTable = StgInfoTable_;

pub type StgTSO = StgTSO_;
