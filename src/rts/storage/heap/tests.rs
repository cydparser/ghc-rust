use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_heap_view_closurePtrs(cap: Capability, closure: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::heap_view_closurePtrs(
            &mut cap.into(),
            &mut closure.into(),
        ))
    };
    let actual = unsafe { super::heap_view_closurePtrs(&mut cap, &mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_heap_view_closurePtrs() {
    let mut cap = Default::default();
    let mut closure = Default::default();
    unsafe { super::heap_view_closurePtrs(&mut cap, &mut closure) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_heap_view_closure_ptrs_in_pap_payload() {
    let mut ptrs = Default::default();
    let mut nptrs = Default::default();
    let mut fun = Default::default();
    let mut payload = Default::default();
    let size = Default::default();
    unsafe {
        super::heap_view_closure_ptrs_in_pap_payload(
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
    let expected = unsafe { transmute(sys::heap_view_closureSize(&mut closure.into())) };
    let actual = unsafe { super::heap_view_closureSize(&mut closure) };
    actual == expected
}

#[test]
#[ignore]
fn test_heap_view_closureSize() {
    let mut closure = Default::default();
    unsafe { super::heap_view_closureSize(&mut closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_collect_pointers(closure: StgClosure, ptrs: StgClosure) -> bool {
    let expected = unsafe {
        transmute(sys::collect_pointers(
            &mut closure.into(),
            &mut &mut ptrs.into(),
        ))
    };
    let actual = unsafe { super::collect_pointers(&mut closure, &mut &mut ptrs) };
    actual == expected
}

#[test]
#[ignore]
fn test_collect_pointers() {
    let mut closure = Default::default();
    let mut ptrs = Default::default();
    unsafe { super::collect_pointers(&mut closure, &mut &mut ptrs) };
    todo!("assert")
}
