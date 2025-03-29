use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_ghc_tsan_atomic64_compare_exchange(
    ptr: __tsan_atomic64,
    expected: __tsan_atomic64,
    new_value: __tsan_atomic64,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::ghc_tsan_atomic64_compare_exchange(
            &mut ptr.into(),
            expected.into(),
            new_value.into(),
            success_memorder.into(),
            failure_memorder.into(),
        ))
    };
    let actual = unsafe {
        super::ghc_tsan_atomic64_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    actual == expected
}

#[test]
#[ignore]
fn test_ghc_tsan_atomic64_compare_exchange() {
    let mut ptr = Default::default();
    let expected = Default::default();
    let new_value = Default::default();
    let success_memorder = Default::default();
    let failure_memorder = Default::default();
    unsafe {
        super::ghc_tsan_atomic64_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_ghc_tsan_atomic32_compare_exchange(
    ptr: __tsan_atomic32,
    expected: __tsan_atomic32,
    new_value: __tsan_atomic32,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::ghc_tsan_atomic32_compare_exchange(
            &mut ptr.into(),
            expected.into(),
            new_value.into(),
            success_memorder.into(),
            failure_memorder.into(),
        ))
    };
    let actual = unsafe {
        super::ghc_tsan_atomic32_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    actual == expected
}

#[test]
#[ignore]
fn test_ghc_tsan_atomic32_compare_exchange() {
    let mut ptr = Default::default();
    let expected = Default::default();
    let new_value = Default::default();
    let success_memorder = Default::default();
    let failure_memorder = Default::default();
    unsafe {
        super::ghc_tsan_atomic32_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_ghc_tsan_atomic16_compare_exchange(
    ptr: __tsan_atomic16,
    expected: __tsan_atomic16,
    new_value: __tsan_atomic16,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::ghc_tsan_atomic16_compare_exchange(
            &mut ptr.into(),
            expected.into(),
            new_value.into(),
            success_memorder.into(),
            failure_memorder.into(),
        ))
    };
    let actual = unsafe {
        super::ghc_tsan_atomic16_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    actual == expected
}

#[test]
#[ignore]
fn test_ghc_tsan_atomic16_compare_exchange() {
    let mut ptr = Default::default();
    let expected = Default::default();
    let new_value = Default::default();
    let success_memorder = Default::default();
    let failure_memorder = Default::default();
    unsafe {
        super::ghc_tsan_atomic16_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_ghc_tsan_atomic8_compare_exchange(
    ptr: __tsan_atomic8,
    expected: __tsan_atomic8,
    new_value: __tsan_atomic8,
    success_memorder: ::core::ffi::c_int,
    failure_memorder: ::core::ffi::c_int,
) -> bool {
    let expected = unsafe {
        transmute(sys::ghc_tsan_atomic8_compare_exchange(
            &mut ptr.into(),
            expected.into(),
            new_value.into(),
            success_memorder.into(),
            failure_memorder.into(),
        ))
    };
    let actual = unsafe {
        super::ghc_tsan_atomic8_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    actual == expected
}

#[test]
#[ignore]
fn test_ghc_tsan_atomic8_compare_exchange() {
    let mut ptr = Default::default();
    let expected = Default::default();
    let new_value = Default::default();
    let success_memorder = Default::default();
    let failure_memorder = Default::default();
    unsafe {
        super::ghc_tsan_atomic8_compare_exchange(
            &mut ptr,
            expected,
            new_value,
            success_memorder,
            failure_memorder,
        )
    };
    todo!("assert")
}
