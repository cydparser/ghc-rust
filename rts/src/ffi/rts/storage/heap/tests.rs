use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_heap_view_closureSize() {
    let expected: StgWord = {
        let mut closure: sys::StgClosure = todo!();
        unsafe { sys::heap_view_closureSize(&raw mut closure) }
    };
    let actual: StgWord = {
        let mut closure: StgClosure = todo!();
        unsafe { heap_view_closureSize(&raw mut closure) }
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_heap_view_closureSize() {
    let actual: StgWord = {
        let closure: StgClosure = todo!();
        unsafe { heap_view_closureSize(&raw mut closure) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_collect_pointers() {
    let expected: StgWord = {
        let mut closure: sys::StgClosure = todo!();
        let mut ptrs: sys::StgClosure = todo!();
        let mut ptrs = &raw mut ptrs;
        unsafe { sys::collect_pointers(&raw mut closure, &raw mut ptrs) }
    };
    let actual: StgWord = {
        let mut closure: StgClosure = todo!();
        let mut ptrs: StgClosure = todo!();
        let mut ptrs = &raw mut ptrs;
        unsafe { collect_pointers(&raw mut closure, &raw mut ptrs) }
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_collect_pointers() {
    let actual: StgWord = {
        let closure: StgClosure = todo!();
        let mut ptrs: StgClosure = todo!();
        let mut ptrs = &raw mut ptrs;
        unsafe { collect_pointers(&raw mut closure, &raw mut ptrs) }
    };
    let expected: StgWord = todo!();
    assert_eq!(expected, actual);
}
