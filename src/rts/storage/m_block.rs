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

static mut peak_mblocks_allocated: W_ = sys::peak_mblocks_allocated;

#[unsafe(no_mangle)]
pub static mut mblocks_allocated: W_ = sys::mblocks_allocated;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn initMBlocks() {
    unsafe { transmute(sys::initMBlocks()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlock() -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getMBlock()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getMBlocks(n: u32) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getMBlocks(n.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlockOnNode(node: u32) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getMBlockOnNode(node.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getMBlocksOnNode(node: u32, n: u32) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getMBlocksOnNode(node.into(), n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn freeMBlocks(addr: *mut ::core::ffi::c_void, n: u32) {
    unsafe { transmute(sys::freeMBlocks(&mut addr.into(), n.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn releaseFreeMemory() {
    unsafe { transmute(sys::releaseFreeMemory()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn freeAllMBlocks() {
    unsafe { transmute(sys::freeAllMBlocks()) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getFirstMBlock(
    state: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::getFirstMBlock(&mut &mut state.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn getNextMBlock(
    state: *mut *mut ::core::ffi::c_void,
    mblock: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        transmute(sys::getNextMBlock(
            &mut &mut state.into(),
            &mut mblock.into(),
        ))
    }
}
