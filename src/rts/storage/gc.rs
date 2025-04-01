use std::{
    ffi::{c_int, c_void},
    mem::transmute,
};

#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::{
    rts::{
        capability::Capability,
        storage::{
            block::bdescr,
            closures::{StgInd, StgIndStatic, StgMutVar, StgWeak},
        },
        types::{StgClosure, StgTSO},
    },
    stg::{
        regs::StgRegTable,
        types::{StgPtr, StgWord},
        W_,
    },
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

pub(crate) type memcount = StgWord;

pub type nursery = nursery_;

#[repr(C)]
pub(crate) struct nursery_ {
    pub blocks: *mut bdescr,
    pub n_blocks: memcount,
}

#[cfg(feature = "sys")]
impl From<nursery_> for sys::nursery_ {
    fn from(x: nursery_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type generation = generation_;

#[repr(C)]
pub(crate) struct generation_ {
    pub no: u32,
    pub blocks: *mut bdescr,
    pub n_blocks: memcount,
    pub n_words: memcount,
    pub large_objects: *mut bdescr,
    pub n_large_blocks: memcount,
    pub n_large_words: memcount,
    pub n_new_large_words: memcount,
    pub compact_objects: *mut bdescr,
    pub n_compact_blocks: memcount,
    pub compact_blocks_in_import: *mut bdescr,
    pub n_compact_blocks_in_import: memcount,
    pub max_blocks: memcount,
    pub threads: *mut StgTSO,
    pub weak_ptr_list: *mut StgWeak,
    pub to: *mut generation_,
    pub collections: u32,
    pub par_collections: u32,
    pub failed_promotions: u32,
    pub mark: c_int,
    pub compact: c_int,
    pub old_blocks: *mut bdescr,
    pub n_old_blocks: memcount,
    pub live_estimate: memcount,
    pub scavenged_large_objects: *mut bdescr,
    pub n_scavenged_large_blocks: memcount,
    pub live_compact_objects: *mut bdescr,
    pub n_live_compact_blocks: memcount,
    pub bitmap: *mut bdescr,
    pub old_threads: *mut StgTSO,
    pub old_weak_ptr_list: *mut StgWeak,
}

#[cfg(feature = "sys")]
impl From<generation_> for sys::generation_ {
    fn from(x: generation_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn listAllBlocks(cb: ListBlocksCb, user: *mut c_void) {
    unsafe { sys::listAllBlocks(transmute(cb), user) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { sys::allocate(transmute(cap), transmute(n)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocateMightFail(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { sys::allocateMightFail(transmute(cap), transmute(n)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocatePinned(
    cap: *mut Capability,
    n: W_,
    alignment: W_,
    align_off: W_,
) -> StgPtr {
    unsafe { sys::allocatePinned(transmute(cap), n, alignment, align_off) }
}

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn flushExec(len: W_, exec_addr: AdjustorExecutable) {
    unsafe { sys::flushExec(len, transmute(exec_addr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performGC() {
    unsafe { sys::performGC() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performMajorGC() {
    unsafe { sys::performMajorGC() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performBlockingMajorGC() {
    unsafe { sys::performBlockingMajorGC() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn newCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newCAF(transmute(reg), transmute(caf))) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newRetainedCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newRetainedCAF(transmute(reg), transmute(caf))) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newGCdCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newGCdCAF(transmute(reg), transmute(caf))) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn revertCAFs() {
    unsafe { sys::revertCAFs() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setKeepCAFs() {
    unsafe { sys::setKeepCAFs() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setHighMemDynamic() {
    unsafe { sys::setHighMemDynamic() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn dirty_MUT_VAR(
    reg: *mut StgRegTable,
    mv: *mut StgMutVar,
    old: *mut StgClosure,
) {
    unsafe { sys::dirty_MUT_VAR(transmute(reg), transmute(mv), transmute(old)) }
}
