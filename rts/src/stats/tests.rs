use super::*;

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
    let actual: c_int = { unsafe { getRTSStatsEnabled() } };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getRTSStatsEnabled() {
    let expected: c_int = { unsafe { sys::getRTSStatsEnabled() } };
    let actual: c_int = { unsafe { getRTSStatsEnabled() } };
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
