use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_GCDetails_layout() {
    assert_eq!(size_of::<GCDetails>(), size_of::<sys::GCDetails>());
    assert_eq!(align_of::<GCDetails>(), align_of::<sys::GCDetails>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_RTSStats_layout() {
    assert_eq!(size_of::<RTSStats>(), size_of::<sys::RTSStats>());
    assert_eq!(align_of::<RTSStats>(), align_of::<sys::RTSStats>());
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getAllocations() {
    let actual: u64 = { unsafe { getAllocations() } };
    let expected: u64 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getAllocations() {
    let expected: u64 = { unsafe { sys::getAllocations() } };
    let actual: u64 = { unsafe { getAllocations() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getRTSStatsEnabled() {
    let actual: i32 = { unsafe { getRTSStatsEnabled() } };
    let expected: i32 = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getRTSStatsEnabled() {
    let expected: i32 = { unsafe { sys::getRTSStatsEnabled() } };
    let actual: i32 = { unsafe { getRTSStatsEnabled() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getRTSStats() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut s: RTSStats = Arbitrary::arbitrary(g);
        unsafe { getRTSStats(&raw mut s) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_getRTSStats(s: RTSStats) -> bool {
    let expected = {
        let mut s = unsafe { transmute(s.clone()) };
        unsafe { sys::getRTSStats(&raw mut s) };
        todo!()
    };

    let actual = {
        let mut s = s.clone();
        unsafe { getRTSStats(&raw mut s) };
        todo!()
    };

    actual == expected
}
