use std::mem::{size_of, transmute};

use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_size_of_nursery_() {
    assert_eq!(size_of::<sys::nursery_>(), size_of::<nursery_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nursery_"][::core::mem::size_of::<nursery_>() - 16usize];
    ["Alignment of nursery_"][::core::mem::align_of::<nursery_>() - 8usize];
    ["Offset of field: nursery_::blocks"][::core::mem::offset_of!(nursery_, blocks) - 0usize];
    ["Offset of field: nursery_::n_blocks"][::core::mem::offset_of!(nursery_, n_blocks) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_generation_() {
    assert_eq!(size_of::<sys::generation_>(), size_of::<generation_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of generation_"][::core::mem::size_of::<generation_>() - 232usize];
    ["Alignment of generation_"][::core::mem::align_of::<generation_>() - 8usize];
    ["Offset of field: generation_::no"][::core::mem::offset_of!(generation_, no) - 0usize];
    ["Offset of field: generation_::blocks"][::core::mem::offset_of!(generation_, blocks) - 8usize];
    ["Offset of field: generation_::n_blocks"]
        [::core::mem::offset_of!(generation_, n_blocks) - 16usize];
    ["Offset of field: generation_::n_words"]
        [::core::mem::offset_of!(generation_, n_words) - 24usize];
    ["Offset of field: generation_::large_objects"]
        [::core::mem::offset_of!(generation_, large_objects) - 32usize];
    ["Offset of field: generation_::n_large_blocks"]
        [::core::mem::offset_of!(generation_, n_large_blocks) - 40usize];
    ["Offset of field: generation_::n_large_words"]
        [::core::mem::offset_of!(generation_, n_large_words) - 48usize];
    ["Offset of field: generation_::n_new_large_words"]
        [::core::mem::offset_of!(generation_, n_new_large_words) - 56usize];
    ["Offset of field: generation_::compact_objects"]
        [::core::mem::offset_of!(generation_, compact_objects) - 64usize];
    ["Offset of field: generation_::n_compact_blocks"]
        [::core::mem::offset_of!(generation_, n_compact_blocks) - 72usize];
    ["Offset of field: generation_::compact_blocks_in_import"]
        [::core::mem::offset_of!(generation_, compact_blocks_in_import) - 80usize];
    ["Offset of field: generation_::n_compact_blocks_in_import"]
        [::core::mem::offset_of!(generation_, n_compact_blocks_in_import) - 88usize];
    ["Offset of field: generation_::max_blocks"]
        [::core::mem::offset_of!(generation_, max_blocks) - 96usize];
    ["Offset of field: generation_::threads"]
        [::core::mem::offset_of!(generation_, threads) - 104usize];
    ["Offset of field: generation_::weak_ptr_list"]
        [::core::mem::offset_of!(generation_, weak_ptr_list) - 112usize];
    ["Offset of field: generation_::to"][::core::mem::offset_of!(generation_, to) - 120usize];
    ["Offset of field: generation_::collections"]
        [::core::mem::offset_of!(generation_, collections) - 128usize];
    ["Offset of field: generation_::par_collections"]
        [::core::mem::offset_of!(generation_, par_collections) - 132usize];
    ["Offset of field: generation_::failed_promotions"]
        [::core::mem::offset_of!(generation_, failed_promotions) - 136usize];
    ["Offset of field: generation_::mark"][::core::mem::offset_of!(generation_, mark) - 140usize];
    ["Offset of field: generation_::compact"]
        [::core::mem::offset_of!(generation_, compact) - 144usize];
    ["Offset of field: generation_::old_blocks"]
        [::core::mem::offset_of!(generation_, old_blocks) - 152usize];
    ["Offset of field: generation_::n_old_blocks"]
        [::core::mem::offset_of!(generation_, n_old_blocks) - 160usize];
    ["Offset of field: generation_::live_estimate"]
        [::core::mem::offset_of!(generation_, live_estimate) - 168usize];
    ["Offset of field: generation_::scavenged_large_objects"]
        [::core::mem::offset_of!(generation_, scavenged_large_objects) - 176usize];
    ["Offset of field: generation_::n_scavenged_large_blocks"]
        [::core::mem::offset_of!(generation_, n_scavenged_large_blocks) - 184usize];
    ["Offset of field: generation_::live_compact_objects"]
        [::core::mem::offset_of!(generation_, live_compact_objects) - 192usize];
    ["Offset of field: generation_::n_live_compact_blocks"]
        [::core::mem::offset_of!(generation_, n_live_compact_blocks) - 200usize];
    ["Offset of field: generation_::bitmap"]
        [::core::mem::offset_of!(generation_, bitmap) - 208usize];
    ["Offset of field: generation_::old_threads"]
        [::core::mem::offset_of!(generation_, old_threads) - 216usize];
    ["Offset of field: generation_::old_weak_ptr_list"]
        [::core::mem::offset_of!(generation_, old_weak_ptr_list) - 224usize];
};

#[test]
#[ignore]
fn test_listAllBlocks() {
    let cb = todo!();
    let mut user = todo!();
    unsafe { listAllBlocks(cb, &mut user) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_allocate() {
    let mut cap = todo!();
    let n = Default::default();
    unsafe { allocate(&mut cap, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_allocateMightFail() {
    let mut cap = todo!();
    let n = Default::default();
    unsafe { allocateMightFail(&mut cap, n) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_allocatePinned() {
    let mut cap = todo!();
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
    let exec_addr = todo!();
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

#[test]
#[ignore]
fn test_newCAF() {
    let mut reg = todo!();
    let mut caf = todo!();
    unsafe { newCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_newRetainedCAF() {
    let mut reg = todo!();
    let mut caf = todo!();
    unsafe { newRetainedCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_newGCdCAF() {
    let mut reg = todo!();
    let mut caf = todo!();
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
    let mut reg = todo!();
    let mut mv = todo!();
    let mut old = todo!();
    unsafe { dirty_MUT_VAR(&mut reg, &mut mv, &mut old) };
    todo!("assert")
}
