use crate::prelude::*;
use crate::rts::capability::Capability;
use crate::rts::storage::block::bdescr;
use crate::rts::storage::closures::{StgClosure, StgInd, StgIndStatic, StgMutVar, StgWeak};
use crate::rts::storage::tso::StgTSO;
use crate::stg::W_;
use crate::stg::regs::StgRegTable;
use crate::stg::types::{StgPtr, StgWord};

#[cfg(test)]
mod tests;

pub(crate) type memcount = StgWord;

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct nursery_ {
    blocks: *mut bdescr,
    n_blocks: memcount,
}

#[cfg(feature = "sys")]
impl From<nursery_> for sys::nursery_ {
    fn from(x: nursery_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for nursery_ {
    fn arbitrary(g: &mut Gen) -> Self {
        nursery_ {
            blocks: Arbitrary::arbitrary(g),
            n_blocks: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type nursery = nursery_;

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct generation_ {
    no: u32,
    blocks: *mut bdescr,
    n_blocks: memcount,
    n_words: memcount,
    large_objects: *mut bdescr,
    n_large_blocks: memcount,
    n_large_words: memcount,
    n_new_large_words: memcount,
    compact_objects: *mut bdescr,
    n_compact_blocks: memcount,
    compact_blocks_in_import: *mut bdescr,
    n_compact_blocks_in_import: memcount,
    max_blocks: memcount,
    threads: *mut StgTSO,
    weak_ptr_list: *mut StgWeak,
    to: *mut generation_,
    collections: u32,
    par_collections: u32,
    failed_promotions: u32,
    mark: c_int,
    compact: c_int,
    old_blocks: *mut bdescr,
    n_old_blocks: memcount,
    live_estimate: memcount,
    scavenged_large_objects: *mut bdescr,
    n_scavenged_large_blocks: memcount,
    live_compact_objects: *mut bdescr,
    n_live_compact_blocks: memcount,
    bitmap: *mut bdescr,
    old_threads: *mut StgTSO,
    old_weak_ptr_list: *mut StgWeak,
}

#[cfg(feature = "sys")]
impl From<generation_> for sys::generation_ {
    fn from(x: generation_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for generation_ {
    fn arbitrary(g: &mut Gen) -> Self {
        generation_ {
            no: Arbitrary::arbitrary(g),
            blocks: Arbitrary::arbitrary(g),
            n_blocks: Arbitrary::arbitrary(g),
            n_words: Arbitrary::arbitrary(g),
            large_objects: Arbitrary::arbitrary(g),
            n_large_blocks: Arbitrary::arbitrary(g),
            n_large_words: Arbitrary::arbitrary(g),
            n_new_large_words: Arbitrary::arbitrary(g),
            compact_objects: Arbitrary::arbitrary(g),
            n_compact_blocks: Arbitrary::arbitrary(g),
            compact_blocks_in_import: Arbitrary::arbitrary(g),
            n_compact_blocks_in_import: Arbitrary::arbitrary(g),
            max_blocks: Arbitrary::arbitrary(g),
            threads: Arbitrary::arbitrary(g),
            weak_ptr_list: Arbitrary::arbitrary(g),
            to: Arbitrary::arbitrary(g),
            collections: Arbitrary::arbitrary(g),
            par_collections: Arbitrary::arbitrary(g),
            failed_promotions: Arbitrary::arbitrary(g),
            mark: Arbitrary::arbitrary(g),
            compact: Arbitrary::arbitrary(g),
            old_blocks: Arbitrary::arbitrary(g),
            n_old_blocks: Arbitrary::arbitrary(g),
            live_estimate: Arbitrary::arbitrary(g),
            scavenged_large_objects: Arbitrary::arbitrary(g),
            n_scavenged_large_blocks: Arbitrary::arbitrary(g),
            live_compact_objects: Arbitrary::arbitrary(g),
            n_live_compact_blocks: Arbitrary::arbitrary(g),
            bitmap: Arbitrary::arbitrary(g),
            old_threads: Arbitrary::arbitrary(g),
            old_weak_ptr_list: Arbitrary::arbitrary(g),
        }
    }
}

/// - GHC_PLACES: {libraries}
pub type generation = generation_;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_generations"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut generations: *mut generation = null_mut();

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_g0"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut g0: *mut generation = null_mut();

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocate"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { sys::allocate(cap as *mut sys::Capability, n) }
}

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setAllocLimitKill"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setAllocLimitKill(arg1: bool, arg2: bool) {
    unsafe { sys::setAllocLimitKill(arg1, arg2) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performGC() {
    unsafe { sys::performGC() }
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performMajorGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    unsafe { sys::performMajorGC() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performBlockingMajorGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    unsafe { sys::performBlockingMajorGC() }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_revertCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn revertCAFs() {
    unsafe { sys::revertCAFs() }
}

/// - GHC_PLACES: {compiler}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setKeepCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    unsafe { sys::setKeepCAFs() }
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setHighMemDynamic"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    unsafe { sys::setHighMemDynamic() }
}

/// - GHC_PLACES: {compiler}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_keepCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut keepCAFs: bool = false;
