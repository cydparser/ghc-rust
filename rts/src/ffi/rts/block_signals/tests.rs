use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_blockUserSignals() {
    let expected = {
        unsafe { sys::blockUserSignals() };
        todo!()
    };
    let actual = {
        unsafe { blockUserSignals() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_blockUserSignals() {
    let actual = {
        unsafe { blockUserSignals() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_unblockUserSignals() {
    let expected = {
        unsafe { sys::unblockUserSignals() };
        todo!()
    };
    let actual = {
        unsafe { unblockUserSignals() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unblockUserSignals() {
    let actual = {
        unsafe { unblockUserSignals() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
