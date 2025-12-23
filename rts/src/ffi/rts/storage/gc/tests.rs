use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_nursery__layout() {
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(nursery_, blocks),
        offset_of!(sys::nursery_, blocks)
    );
    assert_eq!(
        offset_of!(nursery_, n_blocks),
        offset_of!(sys::nursery_, n_blocks)
    );
    assert_eq!(size_of::<nursery_>(), size_of::<sys::nursery_>());
    assert_eq!(align_of::<nursery_>(), align_of::<sys::nursery_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_nursery_layout() {
    assert_eq!(size_of::<nursery>(), size_of::<sys::nursery>());
    assert_eq!(align_of::<nursery>(), align_of::<sys::nursery>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_generation__layout() {
    assert_eq!(
        offset_of!(generation_, no),
        offset_of!(sys::generation_, no)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, blocks),
        offset_of!(sys::generation_, blocks)
    );
    assert_eq!(
        offset_of!(generation_, n_blocks),
        offset_of!(sys::generation_, n_blocks)
    );
    assert_eq!(
        offset_of!(generation_, n_words),
        offset_of!(sys::generation_, n_words)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, large_objects),
        offset_of!(sys::generation_, large_objects)
    );
    assert_eq!(
        offset_of!(generation_, n_large_blocks),
        offset_of!(sys::generation_, n_large_blocks)
    );
    assert_eq!(
        offset_of!(generation_, n_large_words),
        offset_of!(sys::generation_, n_large_words)
    );
    assert_eq!(
        offset_of!(generation_, n_new_large_words),
        offset_of!(sys::generation_, n_new_large_words)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, compact_objects),
        offset_of!(sys::generation_, compact_objects)
    );
    assert_eq!(
        offset_of!(generation_, n_compact_blocks),
        offset_of!(sys::generation_, n_compact_blocks)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, compact_blocks_in_import),
        offset_of!(sys::generation_, compact_blocks_in_import)
    );
    assert_eq!(
        offset_of!(generation_, n_compact_blocks_in_import),
        offset_of!(sys::generation_, n_compact_blocks_in_import)
    );
    assert_eq!(
        offset_of!(generation_, max_blocks),
        offset_of!(sys::generation_, max_blocks)
    );
    assert_eq!(size_of::<*mut StgTSO>(), size_of::<*mut sys::StgTSO>());
    assert_eq!(
        offset_of!(generation_, threads),
        offset_of!(sys::generation_, threads)
    );
    assert_eq!(size_of::<*mut StgWeak>(), size_of::<*mut sys::StgWeak>());
    assert_eq!(
        offset_of!(generation_, weak_ptr_list),
        offset_of!(sys::generation_, weak_ptr_list)
    );
    assert_eq!(
        size_of::<*mut generation_>(),
        size_of::<*mut sys::generation_>()
    );
    assert_eq!(
        offset_of!(generation_, to),
        offset_of!(sys::generation_, to)
    );
    assert_eq!(
        offset_of!(generation_, collections),
        offset_of!(sys::generation_, collections)
    );
    assert_eq!(
        offset_of!(generation_, par_collections),
        offset_of!(sys::generation_, par_collections)
    );
    assert_eq!(
        offset_of!(generation_, failed_promotions),
        offset_of!(sys::generation_, failed_promotions)
    );
    assert_eq!(
        offset_of!(generation_, mark),
        offset_of!(sys::generation_, mark)
    );
    assert_eq!(
        offset_of!(generation_, compact),
        offset_of!(sys::generation_, compact)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, old_blocks),
        offset_of!(sys::generation_, old_blocks)
    );
    assert_eq!(
        offset_of!(generation_, n_old_blocks),
        offset_of!(sys::generation_, n_old_blocks)
    );
    assert_eq!(
        offset_of!(generation_, live_estimate),
        offset_of!(sys::generation_, live_estimate)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, scavenged_large_objects),
        offset_of!(sys::generation_, scavenged_large_objects)
    );
    assert_eq!(
        offset_of!(generation_, n_scavenged_large_blocks),
        offset_of!(sys::generation_, n_scavenged_large_blocks)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, live_compact_objects),
        offset_of!(sys::generation_, live_compact_objects)
    );
    assert_eq!(
        offset_of!(generation_, n_live_compact_blocks),
        offset_of!(sys::generation_, n_live_compact_blocks)
    );
    assert_eq!(size_of::<*mut bdescr>(), size_of::<*mut sys::bdescr>());
    assert_eq!(
        offset_of!(generation_, bitmap),
        offset_of!(sys::generation_, bitmap)
    );
    assert_eq!(size_of::<*mut StgTSO>(), size_of::<*mut sys::StgTSO>());
    assert_eq!(
        offset_of!(generation_, old_threads),
        offset_of!(sys::generation_, old_threads)
    );
    assert_eq!(size_of::<*mut StgWeak>(), size_of::<*mut sys::StgWeak>());
    assert_eq!(
        offset_of!(generation_, old_weak_ptr_list),
        offset_of!(sys::generation_, old_weak_ptr_list)
    );
    assert_eq!(size_of::<generation_>(), size_of::<sys::generation_>());
    assert_eq!(align_of::<generation_>(), align_of::<sys::generation_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_generation_layout() {
    assert_eq!(size_of::<generation>(), size_of::<sys::generation>());
    assert_eq!(align_of::<generation>(), align_of::<sys::generation>());
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_generations_layout() {
    assert_eq!(
        size_of_val(unsafe { &generations }),
        size_of_val(unsafe { &sys::generations })
    );
    assert_eq!(
        align_of_val(unsafe { &generations }),
        align_of_val(unsafe { &sys::generations })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_g0_layout() {
    assert_eq!(
        size_of_val(unsafe { &g0 }),
        size_of_val(unsafe { &sys::g0 })
    );
    assert_eq!(
        align_of_val(unsafe { &g0 }),
        align_of_val(unsafe { &sys::g0 })
    );
}

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
    actual == expected
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
    actual == expected
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
fn equivalent_newCAF() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut caf: sys::StgIndStatic = todo!();
        let result: &StgInd = unsafe { transmute(&*sys::newCAF(&raw mut reg, &raw mut caf)) };
        todo!()
    };
    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut caf: StgIndStatic = todo!();
        let result: &StgInd = unsafe { &*newCAF(&raw mut reg, &raw mut caf) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_newCAF() {
    let actual = {
        let reg: StgRegTable = todo!();
        let caf: StgIndStatic = todo!();
        let result: &StgInd = unsafe { &*newCAF(&raw mut reg, &raw mut caf) };
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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

#[cfg(feature = "sys")]
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
    assert_eq!(actual, expected);
}

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

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_dirty_MUT_VAR() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut mv: sys::StgMutVar = todo!();
        let mut old: sys::StgClosure = todo!();
        unsafe { sys::dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };
    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut mv: StgMutVar = todo!();
        let mut old: StgClosure = todo!();
        unsafe { dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_dirty_MUT_VAR() {
    let actual = {
        let reg: StgRegTable = todo!();
        let mv: StgMutVar = todo!();
        let old: StgClosure = todo!();
        unsafe { dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_keepCAFs_layout() {
    assert_eq!(
        size_of_val(unsafe { &keepCAFs }),
        size_of_val(unsafe { &sys::keepCAFs })
    );
    assert_eq!(
        align_of_val(unsafe { &keepCAFs }),
        align_of_val(unsafe { &sys::keepCAFs })
    );
}
