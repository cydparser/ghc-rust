use std::mem::size_of;

use super::{CostCentreStack_, CostCentre_};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_size_of_CostCentre_() {
    assert_eq!(
        size_of::<sys::CostCentre_>(),
        size_of::<super::CostCentre_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CostCentre_"][::core::mem::size_of::<CostCentre_>() - 64usize];
    ["Alignment of CostCentre_"][::core::mem::align_of::<CostCentre_>() - 8usize];
    ["Offset of field: CostCentre_::ccID"][::core::mem::offset_of!(CostCentre_, ccID) - 0usize];
    ["Offset of field: CostCentre_::label"][::core::mem::offset_of!(CostCentre_, label) - 8usize];
    ["Offset of field: CostCentre_::module"]
        [::core::mem::offset_of!(CostCentre_, module) - 16usize];
    ["Offset of field: CostCentre_::srcloc"]
        [::core::mem::offset_of!(CostCentre_, srcloc) - 24usize];
    ["Offset of field: CostCentre_::mem_alloc"]
        [::core::mem::offset_of!(CostCentre_, mem_alloc) - 32usize];
    ["Offset of field: CostCentre_::time_ticks"]
        [::core::mem::offset_of!(CostCentre_, time_ticks) - 40usize];
    ["Offset of field: CostCentre_::is_caf"]
        [::core::mem::offset_of!(CostCentre_, is_caf) - 48usize];
    ["Offset of field: CostCentre_::link"][::core::mem::offset_of!(CostCentre_, link) - 56usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_CostCentreStack_() {
    assert_eq!(
        size_of::<sys::CostCentreStack_>(),
        size_of::<super::CostCentreStack_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CostCentreStack_"][::core::mem::size_of::<CostCentreStack_>() - 96usize];
    ["Alignment of CostCentreStack_"][::core::mem::align_of::<CostCentreStack_>() - 8usize];
    ["Offset of field: CostCentreStack_::ccsID"]
        [::core::mem::offset_of!(CostCentreStack_, ccsID) - 0usize];
    ["Offset of field: CostCentreStack_::cc"]
        [::core::mem::offset_of!(CostCentreStack_, cc) - 8usize];
    ["Offset of field: CostCentreStack_::prevStack"]
        [::core::mem::offset_of!(CostCentreStack_, prevStack) - 16usize];
    ["Offset of field: CostCentreStack_::indexTable"]
        [::core::mem::offset_of!(CostCentreStack_, indexTable) - 24usize];
    ["Offset of field: CostCentreStack_::root"]
        [::core::mem::offset_of!(CostCentreStack_, root) - 32usize];
    ["Offset of field: CostCentreStack_::depth"]
        [::core::mem::offset_of!(CostCentreStack_, depth) - 40usize];
    ["Offset of field: CostCentreStack_::scc_count"]
        [::core::mem::offset_of!(CostCentreStack_, scc_count) - 48usize];
    ["Offset of field: CostCentreStack_::selected"]
        [::core::mem::offset_of!(CostCentreStack_, selected) - 56usize];
    ["Offset of field: CostCentreStack_::time_ticks"]
        [::core::mem::offset_of!(CostCentreStack_, time_ticks) - 64usize];
    ["Offset of field: CostCentreStack_::mem_alloc"]
        [::core::mem::offset_of!(CostCentreStack_, mem_alloc) - 72usize];
    ["Offset of field: CostCentreStack_::inherited_alloc"]
        [::core::mem::offset_of!(CostCentreStack_, inherited_alloc) - 80usize];
    ["Offset of field: CostCentreStack_::inherited_ticks"]
        [::core::mem::offset_of!(CostCentreStack_, inherited_ticks) - 88usize];
};

#[test]
#[ignore]
fn test_stopProfTimer() {
    unsafe { super::stopProfTimer() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_startProfTimer() {
    unsafe { super::startProfTimer() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_IndexTable_() {
    assert_eq!(
        size_of::<sys::IndexTable_>(),
        size_of::<super::IndexTable_>()
    )
}
