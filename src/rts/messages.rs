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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn vbarf(s: *const ::core::ffi::c_char, ap: *mut __va_list_tag) -> ! {
    unsafe { transmute(sys::vbarf(s, ap)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn verrorBelch(s: *const ::core::ffi::c_char, ap: *mut __va_list_tag) {
    unsafe { sys::verrorBelch(s, ap) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vsysErrorBelch(s: *const ::core::ffi::c_char, ap: *mut __va_list_tag) {
    unsafe { sys::vsysErrorBelch(s, ap) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vdebugBelch(
    s: *const ::core::ffi::c_char,
    ap: *mut __va_list_tag,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::vdebugBelch(s, ap)) }
}

pub(crate) type RtsMsgFunction = ::core::option::Option<
    unsafe extern "C" fn(arg1: *const ::core::ffi::c_char, arg2: *mut __va_list_tag),
>;
pub(crate) type RtsMsgFunctionRetLen = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::core::ffi::c_char,
        arg2: *mut __va_list_tag,
    ) -> ::core::ffi::c_int,
>;
static mut fatalInternalErrorFn: RtsMsgFunction = unsafe { sys::fatalInternalErrorFn };

static mut debugMsgFn: RtsMsgFunctionRetLen = unsafe { sys::debugMsgFn };

static mut errorMsgFn: RtsMsgFunction = unsafe { sys::errorMsgFn };

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsFatalInternalErrorFn(
    arg1: *const ::core::ffi::c_char,
    arg2: *mut __va_list_tag,
) {
    unsafe { sys::rtsFatalInternalErrorFn(arg1, arg2) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsDebugMsgFn(
    arg1: *const ::core::ffi::c_char,
    arg2: *mut __va_list_tag,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::rtsDebugMsgFn(arg1, arg2)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsErrorMsgFn(arg1: *const ::core::ffi::c_char, arg2: *mut __va_list_tag) {
    unsafe { sys::rtsErrorMsgFn(arg1, arg2) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsSysErrorMsgFn(arg1: *const ::core::ffi::c_char, arg2: *mut __va_list_tag) {
    unsafe { sys::rtsSysErrorMsgFn(arg1, arg2) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsBadAlignmentBarf() -> ! {
    unsafe { transmute(sys::rtsBadAlignmentBarf()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    unsafe { transmute(sys::rtsOutOfBoundsAccess()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    unsafe { transmute(sys::rtsMemcpyRangeOverlap()) }
}
