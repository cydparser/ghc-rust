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

use crate::utils::bindgen;
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct _ObjectCode {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<_ObjectCode> for sys::_ObjectCode {
    fn from(x: _ObjectCode) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _ObjectCode {
    fn arbitrary(g: &mut Gen) -> Self {
        _ObjectCode {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub struct ForeignExportsList {
    pub next: *mut ForeignExportsList,
    pub n_entries: c_int,
    pub oc: *mut _ObjectCode,
    pub stable_ptrs: *mut *mut StgStablePtr,
    pub exports: __IncompleteArrayField<StgPtr>,
}

#[cfg(feature = "sys")]
impl From<ForeignExportsList> for sys::ForeignExportsList {
    fn from(x: ForeignExportsList) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct ForeignExportsListOwned {
    pub n_entries: c_int,
    pub exports: __IncompleteArrayField<StgPtr>,
}

#[cfg(test)]
impl Arbitrary for ForeignExportsListOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        ForeignExportsListOwned {
            n_entries: Arbitrary::arbitrary(g),
            exports: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct ForeignExportsListPointees {
    pub next: ForeignExportsList,
    pub oc: _ObjectCode,
    pub stable_ptrs: *mut StgStablePtr,
}

#[cfg(test)]
impl Arbitrary for ForeignExportsListPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        ForeignExportsListPointees {
            next: Arbitrary::arbitrary(g),
            oc: Arbitrary::arbitrary(g),
            stable_ptrs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for ForeignExportsList {
    type Owned = ForeignExportsListOwned;
    type Pointees = ForeignExportsListPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            n_entries: owned.n_entries,
            exports: owned.exports.clone(),
            next: unsafe { &raw mut (*pointees).next },
            oc: unsafe { &raw mut (*pointees).oc },
            stable_ptrs: unsafe { &raw mut (*pointees).stable_ptrs },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            n_entries: self.n_entries,
            exports: self.exports.clone(),
        }
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_registerForeignExports"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn registerForeignExports(exports: *mut ForeignExportsList) {
    unsafe { sys::registerForeignExports(exports as *mut sys::ForeignExportsList) }
}
