use crate::capability::Capability;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::closures::{StgClosure, StgInd, StgIndStatic, StgMutVar, StgWeak};
use crate::ffi::rts::storage::tso::StgTSO;
use crate::ffi::stg::W_;
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) type memcount = StgWord;

#[ffi(compiler)]
#[repr(C)]
pub struct nursery_ {
    pub blocks: *mut bdescr,
    pub n_blocks: memcount,
}

#[ffi(compiler)]
pub type nursery = nursery_;

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub struct generation_ {
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

#[ffi(compiler, ghc_lib)]
pub type generation = generation_;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
pub static mut generations: *mut generation = null_mut();

#[ffi(compiler, ghc_lib, utils)]
#[unsafe(no_mangle)]
pub static mut g0: *mut generation = null_mut();

pub(crate) type ListBlocksCb = Option<unsafe extern "C" fn(user: *mut c_void, arg1: *mut bdescr)>;

#[ffi(compiler, docs, ghc_lib, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn allocate(cap: *mut Capability, n: W_) -> StgPtr {
    sys! {
        allocate(cap as * mut sys::Capability, n)
    }
}

pub(crate) type AdjustorWritable = *mut c_void;

pub(crate) type AdjustorExecutable = *mut c_void;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setAllocLimitKill(arg1: bool, arg2: bool) {
    sys! {
        setAllocLimitKill(arg1, arg2)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performGC() {
    sys! {
        performGC()
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performMajorGC() {
    sys! {
        performMajorGC()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn performBlockingMajorGC() {
    sys! {
        performBlockingMajorGC()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn newCAF(reg: *mut StgRegTable, caf: *mut StgIndStatic) -> *mut StgInd {
    sys! {
        newCAF(reg as * mut sys::StgRegTable, caf as * mut sys::StgIndStatic).cast()
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn revertCAFs() {
    sys! {
        revertCAFs()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setKeepCAFs() {
    sys! {
        setKeepCAFs()
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn setHighMemDynamic() {
    sys! {
        setHighMemDynamic()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn dirty_MUT_VAR(
    reg: *mut StgRegTable,
    mv: *mut StgMutVar,
    old: *mut StgClosure,
) {
    sys! {
        dirty_MUT_VAR(reg as * mut sys::StgRegTable, mv as * mut sys::StgMutVar, old as *
        mut sys::StgClosure)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut keepCAFs: bool = false;
