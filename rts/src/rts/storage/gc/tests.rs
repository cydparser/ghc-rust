use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[test]
fn sys_size_nursery_() {
    assert_eq!(size_of::<sys::nursery_>(), size_of::<nursery_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nursery_"][size_of::<nursery_>() - 16usize];
    ["Alignment of nursery_"][align_of::<nursery_>() - 8usize];
    ["Offset of field: nursery_::blocks"][offset_of!(nursery_, blocks) - 0usize];
    ["Offset of field: nursery_::n_blocks"][offset_of!(nursery_, n_blocks) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_generation_() {
    assert_eq!(size_of::<sys::generation_>(), size_of::<generation_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of generation_"][size_of::<generation_>() - 232usize];
    ["Alignment of generation_"][align_of::<generation_>() - 8usize];
    ["Offset of field: generation_::no"][offset_of!(generation_, no) - 0usize];
    ["Offset of field: generation_::blocks"][offset_of!(generation_, blocks) - 8usize];
    ["Offset of field: generation_::n_blocks"][offset_of!(generation_, n_blocks) - 16usize];
    ["Offset of field: generation_::n_words"][offset_of!(generation_, n_words) - 24usize];
    ["Offset of field: generation_::large_objects"]
        [offset_of!(generation_, large_objects) - 32usize];
    ["Offset of field: generation_::n_large_blocks"]
        [offset_of!(generation_, n_large_blocks) - 40usize];
    ["Offset of field: generation_::n_large_words"]
        [offset_of!(generation_, n_large_words) - 48usize];
    ["Offset of field: generation_::n_new_large_words"]
        [offset_of!(generation_, n_new_large_words) - 56usize];
    ["Offset of field: generation_::compact_objects"]
        [offset_of!(generation_, compact_objects) - 64usize];
    ["Offset of field: generation_::n_compact_blocks"]
        [offset_of!(generation_, n_compact_blocks) - 72usize];
    ["Offset of field: generation_::compact_blocks_in_import"]
        [offset_of!(generation_, compact_blocks_in_import) - 80usize];
    ["Offset of field: generation_::n_compact_blocks_in_import"]
        [offset_of!(generation_, n_compact_blocks_in_import) - 88usize];
    ["Offset of field: generation_::max_blocks"][offset_of!(generation_, max_blocks) - 96usize];
    ["Offset of field: generation_::threads"][offset_of!(generation_, threads) - 104usize];
    ["Offset of field: generation_::weak_ptr_list"]
        [offset_of!(generation_, weak_ptr_list) - 112usize];
    ["Offset of field: generation_::to"][offset_of!(generation_, to) - 120usize];
    ["Offset of field: generation_::collections"][offset_of!(generation_, collections) - 128usize];
    ["Offset of field: generation_::par_collections"]
        [offset_of!(generation_, par_collections) - 132usize];
    ["Offset of field: generation_::failed_promotions"]
        [offset_of!(generation_, failed_promotions) - 136usize];
    ["Offset of field: generation_::mark"][offset_of!(generation_, mark) - 140usize];
    ["Offset of field: generation_::compact"][offset_of!(generation_, compact) - 144usize];
    ["Offset of field: generation_::old_blocks"][offset_of!(generation_, old_blocks) - 152usize];
    ["Offset of field: generation_::n_old_blocks"]
        [offset_of!(generation_, n_old_blocks) - 160usize];
    ["Offset of field: generation_::live_estimate"]
        [offset_of!(generation_, live_estimate) - 168usize];
    ["Offset of field: generation_::scavenged_large_objects"]
        [offset_of!(generation_, scavenged_large_objects) - 176usize];
    ["Offset of field: generation_::n_scavenged_large_blocks"]
        [offset_of!(generation_, n_scavenged_large_blocks) - 184usize];
    ["Offset of field: generation_::live_compact_objects"]
        [offset_of!(generation_, live_compact_objects) - 192usize];
    ["Offset of field: generation_::n_live_compact_blocks"]
        [offset_of!(generation_, n_live_compact_blocks) - 200usize];
    ["Offset of field: generation_::bitmap"][offset_of!(generation_, bitmap) - 208usize];
    ["Offset of field: generation_::old_threads"][offset_of!(generation_, old_threads) - 216usize];
    ["Offset of field: generation_::old_weak_ptr_list"]
        [offset_of!(generation_, old_weak_ptr_list) - 224usize];
};

#[test]
#[ignore]
fn test_listAllBlocks() {
    let cb = todo!();
    let mut user = null_mut();
    unsafe { listAllBlocks(cb, &mut user) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocate(cap: Capability, n: W_) -> bool {
    let expected = unsafe { sys::allocate(&mut cap.into(), n) };
    let actual = unsafe { allocate(&mut cap, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocate() {
    let mut cap = null_mut();
    let n = Default::default();
    unsafe { allocate(&mut cap, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocateMightFail(cap: Capability, n: W_) -> bool {
    let expected = unsafe { sys::allocateMightFail(&mut cap.into(), n) };
    let actual = unsafe { allocateMightFail(&mut cap, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocateMightFail() {
    let mut cap = null_mut();
    let n = Default::default();
    unsafe { allocateMightFail(&mut cap, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocatePinned(cap: Capability, n: W_, alignment: W_, align_off: W_) -> bool {
    let expected = unsafe { sys::allocatePinned(&mut cap.into(), n, alignment, align_off) };
    let actual = unsafe { allocatePinned(&mut cap, n, alignment, align_off) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocatePinned() {
    let mut cap = null_mut();
    let n = Default::default();
    let alignment = Default::default();
    let align_off = Default::default();
    unsafe { allocatePinned(&mut cap, n, alignment, align_off) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_flushExec() {
    let len = Default::default();
    let exec_addr = Default::default();
    unsafe { flushExec(len, exec_addr) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performGC() {
    unsafe { performGC() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performMajorGC() {
    unsafe { performMajorGC() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performBlockingMajorGC() {
    unsafe { performBlockingMajorGC() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { newCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newCAF() {
    let mut reg = null_mut();
    let mut caf = null_mut();
    unsafe { newCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newRetainedCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newRetainedCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { newRetainedCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newRetainedCAF() {
    let mut reg = null_mut();
    let mut caf = null_mut();
    unsafe { newRetainedCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newGCdCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newGCdCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { newGCdCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newGCdCAF() {
    let mut reg = null_mut();
    let mut caf = null_mut();
    unsafe { newGCdCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_revertCAFs() {
    unsafe { revertCAFs() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setKeepCAFs() {
    unsafe { setKeepCAFs() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setHighMemDynamic() {
    unsafe { setHighMemDynamic() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_dirty_MUT_VAR() {
    let mut reg = null_mut();
    let mut mv = null_mut();
    let mut old = null_mut();
    unsafe { dirty_MUT_VAR(&mut reg, &mut mv, &mut old) };
    todo!("assert")
}
