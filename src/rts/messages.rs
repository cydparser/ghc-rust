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
    unsafe { transmute(sys::vbarf(&s.into(), &mut ap.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn verrorBelch(s: *const ::core::ffi::c_char, ap: *mut __va_list_tag) {
    unsafe { transmute(sys::verrorBelch(&s.into(), &mut ap.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vsysErrorBelch(s: *const ::core::ffi::c_char, ap: *mut __va_list_tag) {
    unsafe { transmute(sys::vsysErrorBelch(&s.into(), &mut ap.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn vdebugBelch(
    s: *const ::core::ffi::c_char,
    ap: *mut __va_list_tag,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::vdebugBelch(&s.into(), &mut ap.into())) }
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
static mut fatalInternalErrorFn: RtsMsgFunction = sys::fatalInternalErrorFn;

static mut debugMsgFn: RtsMsgFunctionRetLen = sys::debugMsgFn;

static mut errorMsgFn: RtsMsgFunction = sys::errorMsgFn;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsFatalInternalErrorFn(
    arg1: *const ::core::ffi::c_char,
    arg2: *mut __va_list_tag,
) {
    unsafe { transmute(sys::rtsFatalInternalErrorFn(&arg1.into(), &mut arg2.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsDebugMsgFn(
    arg1: *const ::core::ffi::c_char,
    arg2: *mut __va_list_tag,
) -> ::core::ffi::c_int {
    unsafe { transmute(sys::rtsDebugMsgFn(&arg1.into(), &mut arg2.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsErrorMsgFn(arg1: *const ::core::ffi::c_char, arg2: *mut __va_list_tag) {
    unsafe { transmute(sys::rtsErrorMsgFn(&arg1.into(), &mut arg2.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn rtsSysErrorMsgFn(arg1: *const ::core::ffi::c_char, arg2: *mut __va_list_tag) {
    unsafe { transmute(sys::rtsSysErrorMsgFn(&arg1.into(), &mut arg2.into())) }
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
