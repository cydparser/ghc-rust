use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_CostCentre__layout() {
    assert_eq!(
        offset_of!(CostCentre_, ccID),
        offset_of!(sys::CostCentre_, ccID)
    );
    assert_eq!(
        offset_of!(CostCentre_, label),
        offset_of!(sys::CostCentre_, label)
    );
    assert_eq!(
        offset_of!(CostCentre_, module),
        offset_of!(sys::CostCentre_, module)
    );
    assert_eq!(
        offset_of!(CostCentre_, srcloc),
        offset_of!(sys::CostCentre_, srcloc)
    );
    assert_eq!(
        offset_of!(CostCentre_, mem_alloc),
        offset_of!(sys::CostCentre_, mem_alloc)
    );
    assert_eq!(
        offset_of!(CostCentre_, time_ticks),
        offset_of!(sys::CostCentre_, time_ticks)
    );
    assert_eq!(
        offset_of!(CostCentre_, is_caf),
        offset_of!(sys::CostCentre_, is_caf)
    );
    assert_eq!(
        size_of::<*mut CostCentre_>(),
        size_of::<*mut sys::CostCentre_>()
    );
    assert_eq!(
        offset_of!(CostCentre_, link),
        offset_of!(sys::CostCentre_, link)
    );
    assert_eq!(size_of::<CostCentre_>(), size_of::<sys::CostCentre_>());
    assert_eq!(align_of::<CostCentre_>(), align_of::<sys::CostCentre_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_CostCentre_layout() {
    assert_eq!(size_of::<CostCentre>(), size_of::<sys::CostCentre>());
    assert_eq!(align_of::<CostCentre>(), align_of::<sys::CostCentre>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_CostCentreStack__layout() {
    assert_eq!(
        offset_of!(CostCentreStack_, ccsID),
        offset_of!(sys::CostCentreStack_, ccsID)
    );
    assert_eq!(
        size_of::<*mut CostCentre>(),
        size_of::<*mut sys::CostCentre>()
    );
    assert_eq!(
        offset_of!(CostCentreStack_, cc),
        offset_of!(sys::CostCentreStack_, cc)
    );
    assert_eq!(
        size_of::<*mut CostCentreStack_>(),
        size_of::<*mut sys::CostCentreStack_>()
    );
    assert_eq!(
        offset_of!(CostCentreStack_, prevStack),
        offset_of!(sys::CostCentreStack_, prevStack)
    );
    assert_eq!(
        size_of::<*mut IndexTable_>(),
        size_of::<*mut sys::IndexTable_>()
    );
    assert_eq!(
        offset_of!(CostCentreStack_, indexTable),
        offset_of!(sys::CostCentreStack_, indexTable)
    );
    assert_eq!(
        size_of::<*mut CostCentreStack_>(),
        size_of::<*mut sys::CostCentreStack_>()
    );
    assert_eq!(
        offset_of!(CostCentreStack_, root),
        offset_of!(sys::CostCentreStack_, root)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, depth),
        offset_of!(sys::CostCentreStack_, depth)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, scc_count),
        offset_of!(sys::CostCentreStack_, scc_count)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, selected),
        offset_of!(sys::CostCentreStack_, selected)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, time_ticks),
        offset_of!(sys::CostCentreStack_, time_ticks)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, mem_alloc),
        offset_of!(sys::CostCentreStack_, mem_alloc)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, inherited_alloc),
        offset_of!(sys::CostCentreStack_, inherited_alloc)
    );
    assert_eq!(
        offset_of!(CostCentreStack_, inherited_ticks),
        offset_of!(sys::CostCentreStack_, inherited_ticks)
    );
    assert_eq!(
        size_of::<CostCentreStack_>(),
        size_of::<sys::CostCentreStack_>()
    );
    assert_eq!(
        align_of::<CostCentreStack_>(),
        align_of::<sys::CostCentreStack_>()
    );
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

#[cfg(feature = "sys")]
#[test]
fn sys_IndexTable__layout() {
    assert_eq!(
        size_of::<*mut CostCentre>(),
        size_of::<*mut sys::CostCentre>()
    );
    assert_eq!(
        offset_of!(IndexTable_, cc),
        offset_of!(sys::IndexTable_, cc)
    );
    assert_eq!(
        size_of::<*mut CostCentreStack>(),
        size_of::<*mut sys::CostCentreStack>()
    );
    assert_eq!(
        offset_of!(IndexTable_, ccs),
        offset_of!(sys::IndexTable_, ccs)
    );
    assert_eq!(
        size_of::<*mut IndexTable_>(),
        size_of::<*mut sys::IndexTable_>()
    );
    assert_eq!(
        offset_of!(IndexTable_, next),
        offset_of!(sys::IndexTable_, next)
    );
    assert_eq!(
        offset_of!(IndexTable_, back_edge),
        offset_of!(sys::IndexTable_, back_edge)
    );
    assert_eq!(size_of::<IndexTable_>(), size_of::<sys::IndexTable_>());
    assert_eq!(align_of::<IndexTable_>(), align_of::<sys::IndexTable_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_IndexTable_layout() {
    assert_eq!(size_of::<IndexTable>(), size_of::<sys::IndexTable>());
    assert_eq!(align_of::<IndexTable>(), align_of::<sys::IndexTable>());
}

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

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_era_layout() {
    assert_eq!(
        size_of_val(unsafe { &era }),
        size_of_val(unsafe { &sys::era })
    );
    assert_eq!(
        align_of_val(unsafe { &era }),
        align_of_val(unsafe { &sys::era })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_user_era_layout() {
    assert_eq!(
        size_of_val(unsafe { &user_era }),
        size_of_val(unsafe { &sys::user_era })
    );
    assert_eq!(
        align_of_val(unsafe { &user_era }),
        align_of_val(unsafe { &sys::user_era })
    );
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
