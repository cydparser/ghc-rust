use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

static mut peak_mblocks_allocated: W_ = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_mblocks_allocated"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut mblocks_allocated: W_ = 0;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn initMBlocks() {
    unsafe { sys::initMBlocks() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlock() -> *mut c_void {
    unsafe { sys::getMBlock() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getMBlocks"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getMBlocks(n: u32) -> *mut c_void {
    unsafe { sys::getMBlocks(n) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlockOnNode(node: u32) -> *mut c_void {
    unsafe { sys::getMBlockOnNode(node) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlocksOnNode(node: u32, n: u32) -> *mut c_void {
    unsafe { sys::getMBlocksOnNode(node, n) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_freeMBlocks"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeMBlocks(addr: *mut c_void, n: u32) {
    unsafe { sys::freeMBlocks(addr, n) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_releaseFreeMemory"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn releaseFreeMemory() {
    unsafe { sys::releaseFreeMemory() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeAllMBlocks() {
    unsafe { sys::freeAllMBlocks() }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getFirstMBlock(state: *mut *mut c_void) -> *mut c_void {
    unsafe { sys::getFirstMBlock(state) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getNextMBlock(state: *mut *mut c_void, mblock: *mut c_void) -> *mut c_void {
    unsafe { sys::getNextMBlock(state, mblock) }
}
