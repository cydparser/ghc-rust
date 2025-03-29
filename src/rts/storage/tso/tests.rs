use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_FMT_StgThreadID() {
    assert_eq!(sys::FMT_StgThreadID, super::FMT_StgThreadID);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STACK_DIRTY() {
    assert_eq!(sys::STACK_DIRTY, super::STACK_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STACK_SANE() {
    assert_eq!(sys::STACK_SANE, super::STACK_SANE);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTSOProfInfo() {
    assert_eq!(
        size_of::<sys::StgTSOProfInfo>(),
        size_of::<super::StgTSOProfInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSOProfInfo"][::core::mem::size_of::<StgTSOProfInfo>() - 8usize];
    ["Alignment of StgTSOProfInfo"][::core::mem::align_of::<StgTSOProfInfo>() - 8usize];
    ["Offset of field: StgTSOProfInfo::cccs"]
        [::core::mem::offset_of!(StgTSOProfInfo, cccs) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTSOBlockInfo() {
    assert_eq!(
        size_of::<sys::StgTSOBlockInfo>(),
        size_of::<super::StgTSOBlockInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSOBlockInfo"][::core::mem::size_of::<StgTSOBlockInfo>() - 8usize];
    ["Alignment of StgTSOBlockInfo"][::core::mem::align_of::<StgTSOBlockInfo>() - 8usize];
    ["Offset of field: StgTSOBlockInfo::closure"]
        [::core::mem::offset_of!(StgTSOBlockInfo, closure) - 0usize];
    ["Offset of field: StgTSOBlockInfo::prev"]
        [::core::mem::offset_of!(StgTSOBlockInfo, prev) - 0usize];
    ["Offset of field: StgTSOBlockInfo::bh"][::core::mem::offset_of!(StgTSOBlockInfo, bh) - 0usize];
    ["Offset of field: StgTSOBlockInfo::throwto"]
        [::core::mem::offset_of!(StgTSOBlockInfo, throwto) - 0usize];
    ["Offset of field: StgTSOBlockInfo::wakeup"]
        [::core::mem::offset_of!(StgTSOBlockInfo, wakeup) - 0usize];
    ["Offset of field: StgTSOBlockInfo::fd"][::core::mem::offset_of!(StgTSOBlockInfo, fd) - 0usize];
    ["Offset of field: StgTSOBlockInfo::target"]
        [::core::mem::offset_of!(StgTSOBlockInfo, target) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTSO_() {
    assert_eq!(size_of::<sys::StgTSO_>(), size_of::<super::StgTSO_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSO_"][::core::mem::size_of::<StgTSO_>() - 136usize];
    ["Alignment of StgTSO_"][::core::mem::align_of::<StgTSO_>() - 8usize];
    ["Offset of field: StgTSO_::header"][::core::mem::offset_of!(StgTSO_, header) - 0usize];
    ["Offset of field: StgTSO_::_link"][::core::mem::offset_of!(StgTSO_, _link) - 8usize];
    ["Offset of field: StgTSO_::global_link"]
        [::core::mem::offset_of!(StgTSO_, global_link) - 16usize];
    ["Offset of field: StgTSO_::stackobj"][::core::mem::offset_of!(StgTSO_, stackobj) - 24usize];
    ["Offset of field: StgTSO_::what_next"][::core::mem::offset_of!(StgTSO_, what_next) - 32usize];
    ["Offset of field: StgTSO_::flags"][::core::mem::offset_of!(StgTSO_, flags) - 36usize];
    ["Offset of field: StgTSO_::why_blocked"]
        [::core::mem::offset_of!(StgTSO_, why_blocked) - 40usize];
    ["Offset of field: StgTSO_::block_info"]
        [::core::mem::offset_of!(StgTSO_, block_info) - 48usize];
    ["Offset of field: StgTSO_::id"][::core::mem::offset_of!(StgTSO_, id) - 56usize];
    ["Offset of field: StgTSO_::saved_errno"]
        [::core::mem::offset_of!(StgTSO_, saved_errno) - 64usize];
    ["Offset of field: StgTSO_::dirty"][::core::mem::offset_of!(StgTSO_, dirty) - 68usize];
    ["Offset of field: StgTSO_::bound"][::core::mem::offset_of!(StgTSO_, bound) - 72usize];
    ["Offset of field: StgTSO_::cap"][::core::mem::offset_of!(StgTSO_, cap) - 80usize];
    ["Offset of field: StgTSO_::trec"][::core::mem::offset_of!(StgTSO_, trec) - 88usize];
    ["Offset of field: StgTSO_::label"][::core::mem::offset_of!(StgTSO_, label) - 96usize];
    ["Offset of field: StgTSO_::blocked_exceptions"]
        [::core::mem::offset_of!(StgTSO_, blocked_exceptions) - 104usize];
    ["Offset of field: StgTSO_::bq"][::core::mem::offset_of!(StgTSO_, bq) - 112usize];
    ["Offset of field: StgTSO_::alloc_limit"]
        [::core::mem::offset_of!(StgTSO_, alloc_limit) - 120usize];
    ["Offset of field: StgTSO_::tot_stack_size"]
        [::core::mem::offset_of!(StgTSO_, tot_stack_size) - 128usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgStack_() {
    assert_eq!(size_of::<sys::StgStack_>(), size_of::<super::StgStack_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgStack_"][::core::mem::size_of::<StgStack_>() - 24usize];
    ["Alignment of StgStack_"][::core::mem::align_of::<StgStack_>() - 8usize];
    ["Offset of field: StgStack_::header"][::core::mem::offset_of!(StgStack_, header) - 0usize];
    ["Offset of field: StgStack_::stack_size"]
        [::core::mem::offset_of!(StgStack_, stack_size) - 8usize];
    ["Offset of field: StgStack_::dirty"][::core::mem::offset_of!(StgStack_, dirty) - 12usize];
    ["Offset of field: StgStack_::marking"][::core::mem::offset_of!(StgStack_, marking) - 13usize];
    ["Offset of field: StgStack_::sp"][::core::mem::offset_of!(StgStack_, sp) - 16usize];
    ["Offset of field: StgStack_::stack"][::core::mem::offset_of!(StgStack_, stack) - 24usize];
};

#[test]
#[ignore]
fn test_dirty_TSO() {
    let mut cap = Default::default();
    let mut tso = Default::default();
    unsafe { super::dirty_TSO(&mut cap, &mut tso) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setTSOLink() {
    let mut cap = Default::default();
    let mut tso = Default::default();
    let mut target = Default::default();
    unsafe { super::setTSOLink(&mut cap, &mut tso, &mut target) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setTSOPrev() {
    let mut cap = Default::default();
    let mut tso = Default::default();
    let mut target = Default::default();
    unsafe { super::setTSOPrev(&mut cap, &mut tso, &mut target) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_dirty_STACK() {
    let mut cap = Default::default();
    let mut stack = Default::default();
    unsafe { super::dirty_STACK(&mut cap, &mut stack) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_InCall_() {
    assert_eq!(size_of::<sys::InCall_>(), size_of::<super::InCall_>())
}
