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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_vbarf"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn vbarf(s: *const c_char, ap: *mut __va_list_tag) -> ! {
    unsafe { sys::vbarf(s, ap as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn verrorBelch(s: *const c_char, ap: *mut __va_list_tag) {
    unsafe { sys::verrorBelch(s, ap as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vsysErrorBelch(s: *const c_char, ap: *mut __va_list_tag) {
    unsafe { sys::vsysErrorBelch(s, ap as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vdebugBelch(s: *const c_char, ap: *mut __va_list_tag) -> c_int {
    unsafe { sys::vdebugBelch(s, ap as *mut sys::__va_list_tag) }
}

pub(crate) type RtsMsgFunction =
    Option<unsafe extern "C" fn(arg1: *const c_char, arg2: *mut __va_list_tag)>;
pub(crate) type RtsMsgFunctionRetLen =
    Option<unsafe extern "C" fn(arg1: *const c_char, arg2: *mut __va_list_tag) -> c_int>;
static mut fatalInternalErrorFn: RtsMsgFunction = 0;

static mut debugMsgFn: RtsMsgFunctionRetLen = 0;

static mut errorMsgFn: RtsMsgFunction = 0;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsFatalInternalErrorFn(arg1: *const c_char, arg2: *mut __va_list_tag) {
    unsafe { sys::rtsFatalInternalErrorFn(arg1, arg2 as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsDebugMsgFn(arg1: *const c_char, arg2: *mut __va_list_tag) -> c_int {
    unsafe { sys::rtsDebugMsgFn(arg1, arg2 as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsErrorMsgFn(arg1: *const c_char, arg2: *mut __va_list_tag) {
    unsafe { sys::rtsErrorMsgFn(arg1, arg2 as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsSysErrorMsgFn(arg1: *const c_char, arg2: *mut __va_list_tag) {
    unsafe { sys::rtsSysErrorMsgFn(arg1, arg2 as *mut sys::__va_list_tag) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsBadAlignmentBarf() -> ! {
    unsafe { sys::rtsBadAlignmentBarf() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsOutOfBoundsAccess"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    unsafe { sys::rtsOutOfBoundsAccess() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsMemcpyRangeOverlap"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    unsafe { sys::rtsMemcpyRangeOverlap() }
}
