use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_heap_view_closurePtrs(cap: Capability, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::heap_view_closurePtrs(
            &mut cap.into(),
            &mut closure.into(),
        ))
    };
    let actual = unsafe { heap_view_closurePtrs(&mut cap, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_heap_view_closurePtrs() {
    let mut cap = null_mut();
    let mut closure = null_mut();
    unsafe { heap_view_closurePtrs(&mut cap, &mut closure) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_heap_view_closure_ptrs_in_pap_payload() {
    let mut ptrs = null_mut();
    let mut nptrs = null_mut();
    let mut fun = null_mut();
    let mut payload = null_mut();
    let size = Default::default();
    unsafe {
        heap_view_closure_ptrs_in_pap_payload(
            &mut &mut ptrs,
            &mut nptrs,
            &mut fun,
            &mut &mut payload,
            size,
        )
    };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_heap_view_closureSize(closure: StgClosure) -> bool {
    let expected = unsafe { sys::heap_view_closureSize(&mut closure.into()) };
    let actual = unsafe { heap_view_closureSize(&mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_heap_view_closureSize() {
    let mut closure = null_mut();
    unsafe { heap_view_closureSize(&mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_collect_pointers(closure: StgClosure, ptrs: StgClosure) -> bool {
    let expected = unsafe { sys::collect_pointers(&mut closure.into(), &mut &mut ptrs.into()) };
    let actual = unsafe { collect_pointers(&mut closure, &mut &mut ptrs) };
    actual == expected
}

#[test]
#[ignore]
fn test_collect_pointers() {
    let mut closure = null_mut();
    let mut ptrs = null_mut();
    unsafe { collect_pointers(&mut closure, &mut &mut ptrs) };
    todo!("assert")
}
