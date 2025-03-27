use crate::rts::flags;
use crate::rts::os_threads;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub(crate) type memcount = StgWord;

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

#[cfg(test)]
impl Arbitrary for nursery_ {
    fn arbitrary(g: &mut Gen) -> Self {
        nursery_ {
            blocks: Arbitrary::arbitrary(g),
            n_blocks: Arbitrary::arbitrary(g),
        }
    }
}

pub type nursery = nursery_;

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
    pub mark: ::core::ffi::c_int,
    pub compact: ::core::ffi::c_int,
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

pub type generation = generation_;

#[unsafe(no_mangle)]
pub static mut generations: *mut generation = sys::generations;

#[unsafe(no_mangle)]
pub static mut g0: *mut generation = sys::g0;

static mut oldest_gen: *mut generation = sys::oldest_gen;

pub(crate) type ListBlocksCb =
    ::core::option::Option<unsafe extern "C" fn(user: *mut ::core::ffi::c_void, arg1: *mut bdescr)>;
#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn listAllBlocks(cb: ListBlocksCb, user: *mut ::core::ffi::c_void) {
    unsafe { transmute(sys::listAllBlocks(cb.into(), &mut user.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { transmute(sys::allocate(&mut cap.into(), n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn allocateMightFail(cap: *mut Capability, n: W_) -> StgPtr {
    unsafe { transmute(sys::allocateMightFail(&mut cap.into(), n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn allocatePinned(
    cap: *mut Capability,
    n: W_,
    alignment: W_,
    align_off: W_,
) -> StgPtr {
    unsafe {
        transmute(sys::allocatePinned(
            &mut cap.into(),
            n.into(),
            alignment.into(),
            align_off.into(),
        ))
    }
}

pub(crate) type AdjustorWritable = *mut ::core::ffi::c_void;

pub(crate) type AdjustorExecutable = *mut ::core::ffi::c_void;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn flushExec(len: W_, exec_addr: AdjustorExecutable) {
    unsafe { transmute(sys::flushExec(len.into(), exec_addr.into())) }
}

static mut large_alloc_lim: W_ = sys::large_alloc_lim;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performGC() {
    unsafe { transmute(sys::performGC()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performMajorGC() {
    unsafe { transmute(sys::performMajorGC()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn performBlockingMajorGC() {
    unsafe { transmute(sys::performBlockingMajorGC()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn newCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newCAF(&mut reg.into(), &mut caf.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newRetainedCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newRetainedCAF(&mut reg.into(), &mut caf.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn newGCdCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    unsafe { transmute(sys::newGCdCAF(&mut reg.into(), &mut caf.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn revertCAFs() {
    unsafe { transmute(sys::revertCAFs()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setKeepCAFs() {
    unsafe { transmute(sys::setKeepCAFs()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setHighMemDynamic() {
    unsafe { transmute(sys::setHighMemDynamic()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn dirty_MUT_VAR(
    reg: *mut StgRegTable,
    mv: *mut StgMutVar,
    old: *mut StgClosure,
) {
    unsafe {
        transmute(sys::dirty_MUT_VAR(
            &mut reg.into(),
            &mut mv.into(),
            &mut old.into(),
        ))
    }
}

#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = sys::keepCAFs;
