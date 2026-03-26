use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_CCS_MAIN_layout() {
    assert_eq!(
        size_of_val(unsafe { &CCS_MAIN }),
        size_of_val(unsafe { &sys::CCS_MAIN })
    );
    assert_eq!(
        align_of_val(unsafe { &CCS_MAIN }),
        align_of_val(unsafe { &sys::CCS_MAIN })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_CCS_SYSTEM_layout() {
    assert_eq!(
        size_of_val(unsafe { &CCS_SYSTEM }),
        size_of_val(unsafe { &sys::CCS_SYSTEM })
    );
    assert_eq!(
        align_of_val(unsafe { &CCS_SYSTEM }),
        align_of_val(unsafe { &sys::CCS_SYSTEM })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_CCS_DONT_CARE_layout() {
    assert_eq!(
        size_of_val(unsafe { &CCS_DONT_CARE }),
        size_of_val(unsafe { &sys::CCS_DONT_CARE })
    );
    assert_eq!(
        align_of_val(unsafe { &CCS_DONT_CARE }),
        align_of_val(unsafe { &sys::CCS_DONT_CARE })
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_mkCostCentre() {
    let g = &mut Gen::new(100);

    let actual = {
        let mut label: c_char = Arbitrary::arbitrary(g);
        let mut module: c_char = Arbitrary::arbitrary(g);
        let mut srcloc: c_char = Arbitrary::arbitrary(g);
        let result: &CostCentre =
            unsafe { &*mkCostCentre(&raw mut label, &raw mut module, &raw mut srcloc) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_mkCostCentre(label: c_char, module: c_char, srcloc: c_char) -> bool {
    let expected = {
        let mut label = label;
        let mut module = module;
        let mut srcloc = srcloc;

        let result: &CostCentre = unsafe {
            transmute(&*sys::mkCostCentre(
                &raw mut label,
                &raw mut module,
                &raw mut srcloc,
            ))
        };

        todo!()
    };

    let actual = {
        let mut label = label;
        let mut module = module;
        let mut srcloc = srcloc;
        let result: &CostCentre =
            unsafe { &*mkCostCentre(&raw mut label, &raw mut module, &raw mut srcloc) };
        todo!()
    };

    actual == expected
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
fn test_enterFunCCS() {
    let actual = {
        let reg: StgRegTable = todo!();
        let arg1: CostCentreStack = todo!();
        unsafe { enterFunCCS(&raw mut reg, &raw mut arg1) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_enterFunCCS() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut arg1: sys::CostCentreStack = todo!();
        unsafe { sys::enterFunCCS(&raw mut reg, &raw mut arg1) };
        todo!()
    };

    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut arg1: CostCentreStack = todo!();
        unsafe { enterFunCCS(&raw mut reg, &raw mut arg1) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_pushCostCentre() {
    let actual = {
        let arg1: CostCentreStack = todo!();
        let arg2: CostCentre = todo!();
        let result: &CostCentreStack = unsafe { &*pushCostCentre(&raw mut arg1, &raw mut arg2) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_pushCostCentre() {
    let expected = {
        let mut arg1: sys::CostCentreStack = todo!();
        let mut arg2: sys::CostCentre = todo!();
        let result: &CostCentreStack =
            unsafe { transmute(&*sys::pushCostCentre(&raw mut arg1, &raw mut arg2)) };
        todo!()
    };

    let actual = {
        let mut arg1: CostCentreStack = todo!();
        let mut arg2: CostCentre = todo!();
        let result: &CostCentreStack = unsafe { &*pushCostCentre(&raw mut arg1, &raw mut arg2) };
        todo!()
    };
    assert_eq!(actual, expected);
}
