use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_RtsOptsEnabledEnum_layout() {
    assert_eq!(
        size_of::<RtsOptsEnabledEnum>(),
        size_of::<sys::RtsOptsEnabledEnum>()
    );
    assert_eq!(
        align_of::<RtsOptsEnabledEnum>(),
        align_of::<sys::RtsOptsEnabledEnum>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_RtsOptsEnabledEnum_discriminants() {
    assert_eq!(
        RtsOptsEnabledEnum::RtsOptsNone as isize,
        sys::RtsOptsEnabledEnum::RtsOptsNone as isize
    );
    assert_eq!(
        RtsOptsEnabledEnum::RtsOptsIgnore as isize,
        sys::RtsOptsEnabledEnum::RtsOptsIgnore as isize
    );
    assert_eq!(
        RtsOptsEnabledEnum::RtsOptsIgnoreAll as isize,
        sys::RtsOptsEnabledEnum::RtsOptsIgnoreAll as isize
    );
    assert_eq!(
        RtsOptsEnabledEnum::RtsOptsSafeOnly as isize,
        sys::RtsOptsEnabledEnum::RtsOptsSafeOnly as isize
    );
    assert_eq!(
        RtsOptsEnabledEnum::RtsOptsAll as isize,
        sys::RtsOptsEnabledEnum::RtsOptsAll as isize
    )
}

#[cfg(feature = "sys")]
#[test]
fn sys_RtsConfig_layout() {
    assert_eq!(
        size_of::<RtsOptsEnabledEnum>(),
        size_of::<sys::RtsOptsEnabledEnum>()
    );
    assert_eq!(
        offset_of!(RtsConfig, rts_opts_enabled),
        offset_of!(sys::RtsConfig, rts_opts_enabled)
    );
    assert_eq!(
        offset_of!(RtsConfig, rts_opts_suggestions),
        offset_of!(sys::RtsConfig, rts_opts_suggestions)
    );
    assert_eq!(
        offset_of!(RtsConfig, rts_opts),
        offset_of!(sys::RtsConfig, rts_opts)
    );
    assert_eq!(
        offset_of!(RtsConfig, rts_hs_main),
        offset_of!(sys::RtsConfig, rts_hs_main)
    );
    assert_eq!(
        offset_of!(RtsConfig, keep_cafs),
        offset_of!(sys::RtsConfig, keep_cafs)
    );
    assert_eq!(
        size_of::<*const EventLogWriter>(),
        size_of::<*const sys::EventLogWriter>()
    );
    assert_eq!(
        offset_of!(RtsConfig, eventlog_writer),
        offset_of!(sys::RtsConfig, eventlog_writer)
    );
    assert_eq!(
        offset_of!(RtsConfig, defaultsHook),
        offset_of!(sys::RtsConfig, defaultsHook)
    );
    assert_eq!(
        offset_of!(RtsConfig, onExitHook),
        offset_of!(sys::RtsConfig, onExitHook)
    );
    assert_eq!(
        offset_of!(RtsConfig, stackOverflowHook),
        offset_of!(sys::RtsConfig, stackOverflowHook)
    );
    assert_eq!(
        offset_of!(RtsConfig, outOfHeapHook),
        offset_of!(sys::RtsConfig, outOfHeapHook)
    );
    assert_eq!(
        offset_of!(RtsConfig, mallocFailHook),
        offset_of!(sys::RtsConfig, mallocFailHook)
    );
    assert_eq!(
        size_of::<Option<unsafe extern "C" fn(stats: *const GCDetails_)>>(),
        size_of::<Option<unsafe extern "C" fn(stats: *const sys::GCDetails_)>>()
    );
    assert_eq!(
        offset_of!(RtsConfig, gcDoneHook),
        offset_of!(sys::RtsConfig, gcDoneHook)
    );
    assert_eq!(
        offset_of!(RtsConfig, longGCSync),
        offset_of!(sys::RtsConfig, longGCSync)
    );
    assert_eq!(
        offset_of!(RtsConfig, longGCSyncEnd),
        offset_of!(sys::RtsConfig, longGCSyncEnd)
    );
    assert_eq!(size_of::<RtsConfig>(), size_of::<sys::RtsConfig>());
    assert_eq!(align_of::<RtsConfig>(), align_of::<sys::RtsConfig>());
}
