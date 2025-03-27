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
#[derive(Debug, Copy, Clone)]
pub struct ExecPage {
    pub contents: ::core::ffi::c_char,
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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocateExecPage() -> *mut ExecPage {
    unsafe { transmute(sys::allocateExecPage()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freezeExecPage(page: *mut ExecPage) {
    unsafe { transmute(sys::freezeExecPage(&mut page.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeExecPage(page: *mut ExecPage) {
    unsafe { transmute(sys::freeExecPage(&mut page.into())) }
}
