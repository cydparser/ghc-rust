use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_getMonotonicNSec() {
    let expected: StgWord64 = { unsafe { sys::getMonotonicNSec() } };
    let actual: StgWord64 = { unsafe { getMonotonicNSec() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getMonotonicNSec() {
    let actual: StgWord64 = { unsafe { getMonotonicNSec() } };
    let expected: StgWord64 = todo!();
    assert_eq!(expected, actual);
}
