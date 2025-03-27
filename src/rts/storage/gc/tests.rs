use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_nursery_() {
    assert_eq!(size_of::<sys::nursery_>(), size_of::<super::nursery_>())
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
    assert_eq!(
        size_of::<sys::generation_>(),
        size_of::<super::generation_>()
    )
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
    let cb = Default::default();
    let user = Default::default();
    unsafe { super::listAllBlocks(cb, &mut user) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocate(cap: Capability, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocate(&mut cap.into(), n.into())) };
    let actual = unsafe { super::allocate(&mut cap, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocate() {
    let cap = Default::default();
    let n = Default::default();
    unsafe { super::allocate(&mut cap, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocateMightFail(cap: Capability, n: W_) -> bool {
    let expected = unsafe { transmute(sys::allocateMightFail(&mut cap.into(), n.into())) };
    let actual = unsafe { super::allocateMightFail(&mut cap, n) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocateMightFail() {
    let cap = Default::default();
    let n = Default::default();
    unsafe { super::allocateMightFail(&mut cap, n) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_allocatePinned(cap: Capability, n: W_, alignment: W_, align_off: W_) -> bool {
    let expected = unsafe {
        transmute(sys::allocatePinned(
            &mut cap.into(),
            n.into(),
            alignment.into(),
            align_off.into(),
        ))
    };
    let actual = unsafe { super::allocatePinned(&mut cap, n, alignment, align_off) };
    actual == expected
}

#[test]
#[ignore]
fn test_allocatePinned() {
    let cap = Default::default();
    let n = Default::default();
    let alignment = Default::default();
    let align_off = Default::default();
    unsafe { super::allocatePinned(&mut cap, n, alignment, align_off) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_flushExec() {
    let len = Default::default();
    let exec_addr = Default::default();
    unsafe { super::flushExec(len, exec_addr) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performGC() {
    unsafe { super::performGC() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performMajorGC() {
    unsafe { super::performMajorGC() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_performBlockingMajorGC() {
    unsafe { super::performBlockingMajorGC() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { super::newCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newCAF() {
    let reg = Default::default();
    let caf = Default::default();
    unsafe { super::newCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newRetainedCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newRetainedCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { super::newRetainedCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newRetainedCAF() {
    let reg = Default::default();
    let caf = Default::default();
    unsafe { super::newRetainedCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newGCdCAF(reg: StgRegTable, caf: StgIndStatic) -> bool {
    let expected = unsafe { transmute(sys::newGCdCAF(&mut reg.into(), &mut caf.into())) };
    let actual = unsafe { super::newGCdCAF(&mut reg, &mut caf) };
    actual == expected
}

#[test]
#[ignore]
fn test_newGCdCAF() {
    let reg = Default::default();
    let caf = Default::default();
    unsafe { super::newGCdCAF(&mut reg, &mut caf) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_revertCAFs() {
    unsafe { super::revertCAFs() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setKeepCAFs() {
    unsafe { super::setKeepCAFs() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_setHighMemDynamic() {
    unsafe { super::setHighMemDynamic() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_dirty_MUT_VAR() {
    let reg = Default::default();
    let mv = Default::default();
    let old = Default::default();
    unsafe { super::dirty_MUT_VAR(&mut reg, &mut mv, &mut old) };
    todo!("assert")
}
