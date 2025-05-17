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
///cbindgen:no-export
pub(crate) struct _StgEntCounter {
    pub registeredp: StgWord,
    pub arity: StgInt,
    pub allocd: StgInt,
    pub str_: *mut c_char,
    pub arg_kinds: *mut c_char,
    pub ticky_json: *mut c_char,
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
#[derive(Clone)]
struct _StgEntCounterOwned {
    pub registeredp: StgWord,
    pub arity: StgInt,
    pub allocd: StgInt,
    pub entry_count: StgInt,
    pub allocs: StgInt,
}

#[cfg(test)]
impl Arbitrary for _StgEntCounterOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgEntCounterOwned {
            registeredp: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            allocd: Arbitrary::arbitrary(g),
            entry_count: Arbitrary::arbitrary(g),
            allocs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgEntCounterPointees {
    pub str_: c_char,
    pub arg_kinds: c_char,
    pub ticky_json: c_char,
    pub info: StgInfoTable,
    pub link: _StgEntCounter,
}

#[cfg(test)]
impl Arbitrary for _StgEntCounterPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgEntCounterPointees {
            str_: Arbitrary::arbitrary(g),
            arg_kinds: Arbitrary::arbitrary(g),
            ticky_json: Arbitrary::arbitrary(g),
            info: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _StgEntCounter {
    type Owned = _StgEntCounterOwned;
    type Pointees = _StgEntCounterPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            registeredp: owned.registeredp,
            arity: owned.arity,
            allocd: owned.allocd,
            entry_count: owned.entry_count,
            allocs: owned.allocs,
            str_: unsafe { &raw mut (*pointees).str_ },
            arg_kinds: unsafe { &raw mut (*pointees).arg_kinds },
            ticky_json: unsafe { &raw mut (*pointees).ticky_json },
            info: unsafe { &raw mut (*pointees).info },
            link: unsafe { &raw mut (*pointees).link },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            registeredp: self.registeredp,
            arity: self.arity,
            allocd: self.allocd,
            entry_count: self.entry_count,
            allocs: self.allocs,
        }
    }
}

pub type StgEntCounter = _StgEntCounter;
