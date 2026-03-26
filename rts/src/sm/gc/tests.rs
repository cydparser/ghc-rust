use super::*;

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
        offset_of!(generation_, pad),
        offset_of!(sys::generation_, pad)
    );
    assert_eq!(size_of::<SpinLock>(), size_of::<sys::SpinLock>());
    assert_eq!(
        offset_of!(generation_, sync),
        offset_of!(sys::generation_, sync)
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
fn sys_generation_layout() {
    assert_eq!(size_of::<generation>(), size_of::<sys::generation>());
    assert_eq!(align_of::<generation>(), align_of::<sys::generation>());
}
