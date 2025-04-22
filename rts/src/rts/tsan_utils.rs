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

pub(crate) type __tsan_atomic8 = ::core::ffi::c_char;

pub(crate) type __tsan_atomic16 = ::core::ffi::c_short;

pub(crate) type __tsan_atomic32 = ::core::ffi::c_int;

pub(crate) type __tsan_atomic64 = ::core::ffi::c_long;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic64_compare_exchange(
    ptr: *mut __tsan_atomic64,
    expected: __tsan_atomic64,
    new_value: __tsan_atomic64,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> __tsan_atomic64 {
    unsafe {
        transmute(sys::ghc_tsan_atomic64_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic32_compare_exchange(
    ptr: *mut __tsan_atomic32,
    expected: __tsan_atomic32,
    new_value: __tsan_atomic32,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> __tsan_atomic32 {
    unsafe {
        transmute(sys::ghc_tsan_atomic32_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic16_compare_exchange(
    ptr: *mut __tsan_atomic16,
    expected: __tsan_atomic16,
    new_value: __tsan_atomic16,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> __tsan_atomic16 {
    unsafe {
        transmute(sys::ghc_tsan_atomic16_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic8_compare_exchange(
    ptr: *mut __tsan_atomic8,
    expected: __tsan_atomic8,
    new_value: __tsan_atomic8,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> __tsan_atomic8 {
    unsafe {
        transmute(sys::ghc_tsan_atomic8_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        ))
    }
}
