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

/// cbindgen:no-export
#[repr(C)]
pub struct nursery_ {
    blocks: *mut bdescr,
    n_blocks: memcount,
}

#[ffi(compiler)]
pub type nursery = nursery_;

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
