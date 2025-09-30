use crate::capability::Capability;
use crate::hs_ffi::{HsBool, HsStablePtr};
use crate::prelude::*;
use crate::rts::storage::closures::StgClosure;
use crate::rts::storage::tso::{StgTSO, StgThreadID};
use crate::stg::W_;
use crate::stg::regs::StgRegTable;
use crate::stg::types::StgPtr;

use libc::pid_t;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn createThread(cap: *mut Capability, stack_size: W_) -> *mut StgTSO {
    #[cfg(feature = "sys")]
    unsafe {
        sys::createThread(cap as *mut sys::Capability, stack_size) as *mut StgTSO
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("createThread")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createGenThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn createGenThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    #[cfg(feature = "sys")]
    unsafe {
        sys::createGenThread(
            cap as *mut sys::Capability,
            stack_size,
            closure as *mut sys::StgClosure,
        ) as *mut StgTSO
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("createGenThread")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_suspendThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn suspendThread(arg1: *mut StgRegTable, interruptible: bool) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::suspendThread(arg1 as *mut sys::StgRegTable, interruptible)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("suspendThread")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_resumeThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn resumeThread(arg1: *mut c_void) -> *mut StgRegTable {
    #[cfg(feature = "sys")]
    unsafe {
        sys::resumeThread(arg1) as *mut StgRegTable
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("resumeThread")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_eq_thread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::eq_thread(tso1, tso2)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("eq_thread")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_cmp_thread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn cmp_thread(tso1: StgPtr, tso2: StgPtr) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::cmp_thread(tso1, tso2)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("cmp_thread")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getThreadId"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getThreadId(tso: StgPtr) -> StgThreadID {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_getThreadId(tso)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_getThreadId")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_enableThreadAllocationLimit")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(tso: StgPtr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_enableThreadAllocationLimit(tso)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_enableThreadAllocationLimit")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_disableThreadAllocationLimit")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(tso: StgPtr) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rts_disableThreadAllocationLimit(tso)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rts_disableThreadAllocationLimit")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_forkProcess"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn forkProcess(entry: *mut HsStablePtr) -> pid_t {
    #[cfg(feature = "sys")]
    unsafe {
        sys::forkProcess(entry)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("forkProcess")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsSupportsBoundThreads"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rtsSupportsBoundThreads()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rtsSupportsBoundThreads")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_enabled_capabilities"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut enabled_capabilities: u32 = 0;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setNumCapabilities"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setNumCapabilities(new_: u32) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::setNumCapabilities(new_)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("setNumCapabilities")
}
