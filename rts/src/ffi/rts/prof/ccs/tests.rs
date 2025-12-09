use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_CostCentre_layout() {
    assert_eq!(size_of::<CostCentre>(), size_of::<sys::CostCentre>());
    assert_eq!(align_of::<CostCentre>(), align_of::<sys::CostCentre>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_CostCentreStack_layout() {
    assert_eq!(
        size_of::<CostCentreStack>(),
        size_of::<sys::CostCentreStack>()
    );
    assert_eq!(
        align_of::<CostCentreStack>(),
        align_of::<sys::CostCentreStack>()
    );
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stopProfTimer() {
    let expected = {
        unsafe { sys::stopProfTimer() };
        todo!()
    };
    let actual = {
        unsafe { stopProfTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stopProfTimer() {
    let actual = {
        unsafe { stopProfTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_startProfTimer() {
    let expected = {
        unsafe { sys::startProfTimer() };
        todo!()
    };
    let actual = {
        unsafe { startProfTimer() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_startProfTimer() {
    let actual = {
        unsafe { startProfTimer() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
