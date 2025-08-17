use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_heap_view_closurePtrs() {
    todo!()
}

#[test]
#[ignore]
fn test_heap_view_closurePtrs() {
    let cap = null_mut();
    let closure = null_mut();
    unsafe { heap_view_closurePtrs(cap, closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_heap_view_closure_ptrs_in_pap_payload() {
    todo!()
}

#[test]
#[ignore]
fn test_heap_view_closure_ptrs_in_pap_payload() {
    let ptrs = null_mut();
    let nptrs = null_mut();
    let fun = null_mut();
    let payload = null_mut();
    let size = Default::default();
    unsafe { heap_view_closure_ptrs_in_pap_payload(ptrs, nptrs, fun, payload, size) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_heap_view_closureSize() {
    todo!()
}

#[test]
#[ignore]
fn test_heap_view_closureSize() {
    let closure = null_mut();
    unsafe { heap_view_closureSize(closure) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_collect_pointers() {
    todo!()
}

#[test]
#[ignore]
fn test_collect_pointers() {
    let closure = null_mut();
    let ptrs = null_mut();
    unsafe { collect_pointers(closure, ptrs) };
    todo!("assert")
}
