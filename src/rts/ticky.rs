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
pub(crate) struct _StgEntCounter {
    pub registeredp: StgWord,
    pub arity: StgInt,
    pub allocd: StgInt,
    pub str_: *mut ::core::ffi::c_char,
    pub arg_kinds: *mut ::core::ffi::c_char,
    pub ticky_json: *mut ::core::ffi::c_char,
    pub info: *mut StgInfoTable,
    pub entry_count: StgInt,
    pub allocs: StgInt,
    pub link: *mut _StgEntCounter,
}

#[cfg(feature = "sys")]
impl From<_StgEntCounter> for sys::_StgEntCounter {
    fn from(x: _StgEntCounter) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgEntCounter {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgEntCounter {
            registeredp: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            allocd: Arbitrary::arbitrary(g),
            str_: Arbitrary::arbitrary(g),
            arg_kinds: Arbitrary::arbitrary(g),
            ticky_json: Arbitrary::arbitrary(g),
            info: Arbitrary::arbitrary(g),
            entry_count: Arbitrary::arbitrary(g),
            allocs: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgEntCounter = _StgEntCounter;
