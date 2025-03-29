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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);

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
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    pub n_entries: ::core::ffi::c_int,
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
impl Arbitrary for ForeignExportsList {
    fn arbitrary(g: &mut Gen) -> Self {
        ForeignExportsList {
            next: Arbitrary::arbitrary(g),
            n_entries: Arbitrary::arbitrary(g),
            oc: Arbitrary::arbitrary(g),
            stable_ptrs: Arbitrary::arbitrary(g),
            exports: Arbitrary::arbitrary(g),
        }
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn registerForeignExports(exports: *mut ForeignExportsList) {
    unsafe { sys::registerForeignExports(exports) }
}
