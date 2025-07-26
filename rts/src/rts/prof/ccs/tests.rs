use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_CostCentre_() {
    assert_eq!(size_of::<sys::CostCentre_>(), size_of::<CostCentre_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CostCentre_"][size_of::<CostCentre_>() - 64usize];
    ["Alignment of CostCentre_"][align_of::<CostCentre_>() - 8usize];
    ["Offset of field: CostCentre_::ccID"][offset_of!(CostCentre_, ccID) - 0usize];
    ["Offset of field: CostCentre_::label"][offset_of!(CostCentre_, label) - 8usize];
    ["Offset of field: CostCentre_::module"][offset_of!(CostCentre_, module) - 16usize];
    ["Offset of field: CostCentre_::srcloc"][offset_of!(CostCentre_, srcloc) - 24usize];
    ["Offset of field: CostCentre_::mem_alloc"][offset_of!(CostCentre_, mem_alloc) - 32usize];
    ["Offset of field: CostCentre_::time_ticks"][offset_of!(CostCentre_, time_ticks) - 40usize];
    ["Offset of field: CostCentre_::is_caf"][offset_of!(CostCentre_, is_caf) - 48usize];
    ["Offset of field: CostCentre_::link"][offset_of!(CostCentre_, link) - 56usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_CostCentreStack_() {
    assert_eq!(
        size_of::<sys::CostCentreStack_>(),
        size_of::<CostCentreStack_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CostCentreStack_"][size_of::<CostCentreStack_>() - 96usize];
    ["Alignment of CostCentreStack_"][align_of::<CostCentreStack_>() - 8usize];
    ["Offset of field: CostCentreStack_::ccsID"][offset_of!(CostCentreStack_, ccsID) - 0usize];
    ["Offset of field: CostCentreStack_::cc"][offset_of!(CostCentreStack_, cc) - 8usize];
    ["Offset of field: CostCentreStack_::prevStack"]
        [offset_of!(CostCentreStack_, prevStack) - 16usize];
    ["Offset of field: CostCentreStack_::indexTable"]
        [offset_of!(CostCentreStack_, indexTable) - 24usize];
    ["Offset of field: CostCentreStack_::root"][offset_of!(CostCentreStack_, root) - 32usize];
    ["Offset of field: CostCentreStack_::depth"][offset_of!(CostCentreStack_, depth) - 40usize];
    ["Offset of field: CostCentreStack_::scc_count"]
        [offset_of!(CostCentreStack_, scc_count) - 48usize];
    ["Offset of field: CostCentreStack_::selected"]
        [offset_of!(CostCentreStack_, selected) - 56usize];
    ["Offset of field: CostCentreStack_::time_ticks"]
        [offset_of!(CostCentreStack_, time_ticks) - 64usize];
    ["Offset of field: CostCentreStack_::mem_alloc"]
        [offset_of!(CostCentreStack_, mem_alloc) - 72usize];
    ["Offset of field: CostCentreStack_::inherited_alloc"]
        [offset_of!(CostCentreStack_, inherited_alloc) - 80usize];
    ["Offset of field: CostCentreStack_::inherited_ticks"]
        [offset_of!(CostCentreStack_, inherited_ticks) - 88usize];
};

#[test]
#[ignore]
fn test_stopProfTimer() {
    unsafe { stopProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startProfTimer() {
    unsafe { startProfTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_IndexTable_() {
    assert_eq!(size_of::<sys::IndexTable_>(), size_of::<IndexTable_>())
}
