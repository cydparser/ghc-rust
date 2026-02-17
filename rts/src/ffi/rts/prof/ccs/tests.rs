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
        offset_of!(IndexTable_, _address),
        offset_of!(sys::IndexTable_, _address)
    );
    assert_eq!(size_of::<IndexTable_>(), size_of::<sys::IndexTable_>());
    assert_eq!(align_of::<IndexTable_>(), align_of::<sys::IndexTable_>());
}
