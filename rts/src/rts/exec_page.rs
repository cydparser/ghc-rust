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
#[derive(Debug, Copy, Clone)]
pub struct ExecPage {
    pub contents: c_char,
}

#[cfg(feature = "sys")]
impl From<ExecPage> for sys::ExecPage {
    fn from(x: ExecPage) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for ExecPage {
    fn arbitrary(g: &mut Gen) -> Self {
        ExecPage {
            contents: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocateExecPage"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocateExecPage() -> *mut ExecPage {
    unsafe { transmute(sys::allocateExecPage()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freezeExecPage"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freezeExecPage(page: *mut ExecPage) {
    unsafe { sys::freezeExecPage(page as *mut sys::ExecPage) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeExecPage"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeExecPage(page: *mut ExecPage) {
    unsafe { sys::freeExecPage(page as *mut sys::ExecPage) }
}
