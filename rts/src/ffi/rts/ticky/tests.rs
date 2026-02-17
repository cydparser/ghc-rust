use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_StgEntCounter_layout() {
    assert_eq!(size_of::<StgEntCounter>(), size_of::<sys::StgEntCounter>());
    assert_eq!(
        align_of::<StgEntCounter>(),
        align_of::<sys::StgEntCounter>()
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_requestTickyCounterSamples() {
    let expected = {
        unsafe { sys::requestTickyCounterSamples() };
        todo!()
    };

    let actual = {
        unsafe { requestTickyCounterSamples() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_requestTickyCounterSamples() {
    let actual = {
        unsafe { requestTickyCounterSamples() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}
