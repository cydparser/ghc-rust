use crate::capability::Capability;
use crate::prelude::*;
use crate::rts::storage::block::bdescr;
use crate::rts::storage::closures::StgWeak;
use crate::rts::storage::tso::StgTSO;
use crate::stg::W_;
use crate::stg::types::{StgPtr, StgWord};

#[cfg(test)]
mod tests;

pub(crate) type memcount = StgWord;

/// cbindgen:no-export
#[repr(C)]
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

pub(crate) type nursery = nursery_;

/// cbindgen:no-export
#[repr(C)]
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

/// - GHC_PLACES: {libraries}
pub type generation = generation_;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
pub static mut generations: *mut generation = null_mut();

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
pub static mut g0: *mut generation = null_mut();

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;
/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::allocate(cap as *mut sys::Capability, n)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("allocate")
}

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setAllocLimitKill(arg1: bool, arg2: bool) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setAllocLimitKill(arg1, arg2)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setAllocLimitKill")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performGC() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::performGC()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("performGC")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::performMajorGC()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("performMajorGC")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::performBlockingMajorGC()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("performBlockingMajorGC")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn revertCAFs() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::revertCAFs()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("revertCAFs")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setKeepCAFs()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setKeepCAFs")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setHighMemDynamic()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setHighMemDynamic")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = false;
