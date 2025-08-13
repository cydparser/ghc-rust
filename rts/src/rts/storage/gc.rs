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

pub type nursery = nursery_;

#[repr(C)]
///cbindgen:no-export
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

pub type generation = generation_;

#[repr(C)]
///cbindgen:no-export
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

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_generations"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut generations: *mut generation = null_mut();

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_g0"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut g0: *mut generation = null_mut();

static mut oldest_gen: *mut generation = null_mut();

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;
#[instrument]
pub(crate) unsafe fn listAllBlocks(cb: ListBlocksCb, user: *mut c_void) {
    unsafe { sys::listAllBlocks(transmute(cb), user) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocate"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { sys::allocate(cap as *mut sys::Capability, n) }
}

#[instrument]
pub(crate) unsafe fn allocateMightFail(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { sys::allocateMightFail(cap as *mut sys::Capability, n) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_allocatePinned"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn allocatePinned(
    cap: *mut Capability,
    n: W_,
    alignment: W_,
    align_off: W_,
) -> StgPtr {
    unsafe { sys::allocatePinned(cap as *mut sys::Capability, n, alignment, align_off) }
}

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_flushExec"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn flushExec(len: W_, exec_addr: AdjustorExecutable) {
    unsafe { sys::flushExec(len, exec_addr) }
}

static mut large_alloc_lim: W_ = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performGC() {
    unsafe { sys::performGC() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performMajorGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    unsafe { sys::performMajorGC() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_performBlockingMajorGC"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    unsafe { sys::performBlockingMajorGC() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_newCAF"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn newCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe {
        transmute(sys::newCAF(
            reg as *mut sys::StgRegTable,
            caf as *mut sys::StgIndStatic,
        ))
    }
}

#[instrument]
pub(crate) unsafe fn newRetainedCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe {
        transmute(sys::newRetainedCAF(
            reg as *mut sys::StgRegTable,
            caf as *mut sys::StgIndStatic,
        ))
    }
}

#[instrument]
pub(crate) unsafe fn newGCdCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe {
        transmute(sys::newGCdCAF(
            reg as *mut sys::StgRegTable,
            caf as *mut sys::StgIndStatic,
        ))
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_revertCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn revertCAFs() {
    unsafe { sys::revertCAFs() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setKeepCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    unsafe { sys::setKeepCAFs() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setHighMemDynamic"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    unsafe { sys::setHighMemDynamic() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_dirty_MUT_VAR"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn dirty_MUT_VAR(
    reg: *mut StgRegTable,
    mv: *mut StgMutVar,
    old: *mut StgClosure,
) {
    unsafe {
        sys::dirty_MUT_VAR(
            reg as *mut sys::StgRegTable,
            mv as *mut sys::StgMutVar,
            old as *mut sys::StgClosure,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_keepCAFs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut keepCAFs: bool = false;
