use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_nursery_() {
    assert_eq!(size_of::<sys::nursery_>(), size_of::<nursery_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nursery_"][size_of::<nursery_>() - 16usize];
    ["Alignment of nursery_"][align_of::<nursery_>() - 8usize];
    ["Offset of field: nursery_::blocks"][offset_of!(nursery_, blocks) - 0usize];
    ["Offset of field: nursery_::n_blocks"][offset_of!(nursery_, n_blocks) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_generation_() {
    assert_eq!(size_of::<sys::generation_>(), size_of::<generation_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of generation_"][size_of::<generation_>() - 232usize];
    ["Alignment of generation_"][align_of::<generation_>() - 8usize];
    ["Offset of field: generation_::no"][offset_of!(generation_, no) - 0usize];
    ["Offset of field: generation_::blocks"][offset_of!(generation_, blocks) - 8usize];
    ["Offset of field: generation_::n_blocks"][offset_of!(generation_, n_blocks) - 16usize];
    ["Offset of field: generation_::n_words"][offset_of!(generation_, n_words) - 24usize];
    ["Offset of field: generation_::large_objects"]
        [offset_of!(generation_, large_objects) - 32usize];
    ["Offset of field: generation_::n_large_blocks"]
        [offset_of!(generation_, n_large_blocks) - 40usize];
    ["Offset of field: generation_::n_large_words"]
        [offset_of!(generation_, n_large_words) - 48usize];
    ["Offset of field: generation_::n_new_large_words"]
        [offset_of!(generation_, n_new_large_words) - 56usize];
    ["Offset of field: generation_::compact_objects"]
        [offset_of!(generation_, compact_objects) - 64usize];
    ["Offset of field: generation_::n_compact_blocks"]
        [offset_of!(generation_, n_compact_blocks) - 72usize];
    ["Offset of field: generation_::compact_blocks_in_import"]
        [offset_of!(generation_, compact_blocks_in_import) - 80usize];
    ["Offset of field: generation_::n_compact_blocks_in_import"]
        [offset_of!(generation_, n_compact_blocks_in_import) - 88usize];
    ["Offset of field: generation_::max_blocks"][offset_of!(generation_, max_blocks) - 96usize];
    ["Offset of field: generation_::threads"][offset_of!(generation_, threads) - 104usize];
    ["Offset of field: generation_::weak_ptr_list"]
        [offset_of!(generation_, weak_ptr_list) - 112usize];
    ["Offset of field: generation_::to"][offset_of!(generation_, to) - 120usize];
    ["Offset of field: generation_::collections"][offset_of!(generation_, collections) - 128usize];
    ["Offset of field: generation_::par_collections"]
        [offset_of!(generation_, par_collections) - 132usize];
    ["Offset of field: generation_::failed_promotions"]
        [offset_of!(generation_, failed_promotions) - 136usize];
    ["Offset of field: generation_::mark"][offset_of!(generation_, mark) - 140usize];
    ["Offset of field: generation_::compact"][offset_of!(generation_, compact) - 144usize];
    ["Offset of field: generation_::old_blocks"][offset_of!(generation_, old_blocks) - 152usize];
    ["Offset of field: generation_::n_old_blocks"]
        [offset_of!(generation_, n_old_blocks) - 160usize];
    ["Offset of field: generation_::live_estimate"]
        [offset_of!(generation_, live_estimate) - 168usize];
    ["Offset of field: generation_::scavenged_large_objects"]
        [offset_of!(generation_, scavenged_large_objects) - 176usize];
    ["Offset of field: generation_::n_scavenged_large_blocks"]
        [offset_of!(generation_, n_scavenged_large_blocks) - 184usize];
    ["Offset of field: generation_::live_compact_objects"]
        [offset_of!(generation_, live_compact_objects) - 192usize];
    ["Offset of field: generation_::n_live_compact_blocks"]
        [offset_of!(generation_, n_live_compact_blocks) - 200usize];
    ["Offset of field: generation_::bitmap"][offset_of!(generation_, bitmap) - 208usize];
    ["Offset of field: generation_::old_threads"][offset_of!(generation_, old_threads) - 216usize];
    ["Offset of field: generation_::old_weak_ptr_list"]
        [offset_of!(generation_, old_weak_ptr_list) - 224usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_allocate(n: W_) -> bool {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let result: StgPtr = unsafe { sys::allocate(&raw mut cap, n) };
        todo!()
    };
    let actual = {
        let mut cap: Capability = todo!();
        let result: StgPtr = unsafe { allocate(&raw mut cap, n) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_allocate() {
    let g = &mut Gen::new(100);
    let actual = {
        let cap: Capability = todo!();
        let n: W_ = Arbitrary::arbitrary(g);
        let result: StgPtr = unsafe { allocate(&raw mut cap, n) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setAllocLimitKill(arg1: bool, arg2: bool) -> bool {
    let expected = {
        unsafe { sys::setAllocLimitKill(arg1, arg2) };
        todo!()
    };
    let actual = {
        unsafe { setAllocLimitKill(arg1, arg2) };
        todo!()
    };
    expected == actual
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setAllocLimitKill() {
    let g = &mut Gen::new(100);
    let actual = {
        let arg1: bool = Arbitrary::arbitrary(g);
        let arg2: bool = Arbitrary::arbitrary(g);
        unsafe { setAllocLimitKill(arg1, arg2) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performGC() {
    let expected = {
        unsafe { sys::performGC() };
        todo!()
    };
    let actual = {
        unsafe { performGC() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performGC() {
    let actual = {
        unsafe { performGC() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performMajorGC() {
    let expected = {
        unsafe { sys::performMajorGC() };
        todo!()
    };
    let actual = {
        unsafe { performMajorGC() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performMajorGC() {
    let actual = {
        unsafe { performMajorGC() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_performBlockingMajorGC() {
    let expected = {
        unsafe { sys::performBlockingMajorGC() };
        todo!()
    };
    let actual = {
        unsafe { performBlockingMajorGC() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_performBlockingMajorGC() {
    let actual = {
        unsafe { performBlockingMajorGC() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_revertCAFs() {
    let expected = {
        unsafe { sys::revertCAFs() };
        todo!()
    };
    let actual = {
        unsafe { revertCAFs() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_revertCAFs() {
    let actual = {
        unsafe { revertCAFs() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setKeepCAFs() {
    let expected = {
        unsafe { sys::setKeepCAFs() };
        todo!()
    };
    let actual = {
        unsafe { setKeepCAFs() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setKeepCAFs() {
    let actual = {
        unsafe { setKeepCAFs() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setHighMemDynamic() {
    let expected = {
        unsafe { sys::setHighMemDynamic() };
        todo!()
    };
    let actual = {
        unsafe { setHighMemDynamic() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setHighMemDynamic() {
    let actual = {
        unsafe { setHighMemDynamic() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
