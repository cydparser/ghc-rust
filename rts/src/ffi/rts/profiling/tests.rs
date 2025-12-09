use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_registerCcList() {
    let expected = {
        let mut cc_list: sys::CostCentre = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { sys::registerCcList(&raw mut cc_list) };
        todo!()
    };
    let actual = {
        let mut cc_list: CostCentre = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { registerCcList(&raw mut cc_list) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_registerCcList() {
    let actual = {
        let mut cc_list: CostCentre = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { registerCcList(&raw mut cc_list) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_registerCcsList() {
    let expected = {
        let mut cc_list: sys::CostCentreStack = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { sys::registerCcsList(&raw mut cc_list) };
        todo!()
    };
    let actual = {
        let mut cc_list: CostCentreStack = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { registerCcsList(&raw mut cc_list) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_registerCcsList() {
    let actual = {
        let mut cc_list: CostCentreStack = todo!();
        let mut cc_list = &raw mut cc_list;
        unsafe { registerCcsList(&raw mut cc_list) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
