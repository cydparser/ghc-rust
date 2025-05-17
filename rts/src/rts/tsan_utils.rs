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

pub(crate) type __tsan_atomic8 = c_char;

pub(crate) type __tsan_atomic16 = c_short;

pub(crate) type __tsan_atomic32 = c_int;

pub(crate) type __tsan_atomic64 = c_long;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic64_compare_exchange(
    ptr: *mut __tsan_atomic64,
    expected: __tsan_atomic64,
    new_value: __tsan_atomic64,
    success_memorder: c_int,
    failure_memorder: c_int,
) -> __tsan_atomic64 {
    unsafe {
        sys::ghc_tsan_atomic64_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic32_compare_exchange(
    ptr: *mut __tsan_atomic32,
    expected: __tsan_atomic32,
    new_value: __tsan_atomic32,
    success_memorder: c_int,
    failure_memorder: c_int,
) -> __tsan_atomic32 {
    unsafe {
        sys::ghc_tsan_atomic32_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic16_compare_exchange(
    ptr: *mut __tsan_atomic16,
    expected: __tsan_atomic16,
    new_value: __tsan_atomic16,
    success_memorder: c_int,
    failure_memorder: c_int,
) -> __tsan_atomic16 {
    unsafe {
        sys::ghc_tsan_atomic16_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn ghc_tsan_atomic8_compare_exchange(
    ptr: *mut __tsan_atomic8,
    expected: __tsan_atomic8,
    new_value: __tsan_atomic8,
    success_memorder: c_int,
    failure_memorder: c_int,
) -> __tsan_atomic8 {
    unsafe {
        sys::ghc_tsan_atomic8_compare_exchange(
            ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    }
}
