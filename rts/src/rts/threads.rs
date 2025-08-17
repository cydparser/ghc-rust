use crate::hs_ffi::{HsBool, HsStablePtr};
use crate::prelude::*;
use crate::rts::capability::Capability;
use crate::rts::storage::closures::{_StgMutArrPtrs, StgClosure};
use crate::rts::storage::tso::StgTSO;
use crate::rts::storage::tso::StgThreadID;
use crate::rts_api::HaskellObj;
use crate::stg::W_;
use crate::stg::regs::StgRegTable;
use crate::stg::types::StgPtr;

use libc::pid_t;

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn createThread(cap: *mut Capability, stack_size: W_) -> *mut StgTSO {
    unsafe { transmute(sys::createThread(cap as *mut sys::Capability, stack_size)) }
}

#[instrument]
pub(crate) unsafe fn scheduleWaitThread(
    tso: *mut StgTSO,
    ret: *mut HaskellObj,
    cap: *mut *mut Capability,
) {
    unsafe {
        sys::scheduleWaitThread(
            tso as *mut sys::StgTSO,
            ret as *mut sys::HaskellObj,
            cap as *mut *mut sys::Capability,
        )
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_createGenThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn createGenThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createGenThread(
            cap as *mut sys::Capability,
            stack_size,
            closure as *mut sys::StgClosure,
        ))
    }
}

#[instrument]
pub(crate) unsafe fn createIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createIOThread(
            cap as *mut sys::Capability,
            stack_size,
            closure as *mut sys::StgClosure,
        ))
    }
}

#[instrument]
pub(crate) unsafe fn createStrictIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createStrictIOThread(
            cap as *mut sys::Capability,
            stack_size,
            closure as *mut sys::StgClosure,
        ))
    }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_suspendThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn suspendThread(arg1: *mut StgRegTable, interruptible: bool) -> *mut c_void {
    unsafe { sys::suspendThread(arg1 as *mut sys::StgRegTable, interruptible) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_resumeThread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn resumeThread(arg1: *mut c_void) -> *mut StgRegTable {
    unsafe { transmute(sys::resumeThread(arg1)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_eq_thread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    unsafe { sys::eq_thread(tso1, tso2) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_cmp_thread"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn cmp_thread(tso1: StgPtr, tso2: StgPtr) -> c_int {
    unsafe { sys::cmp_thread(tso1, tso2) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rts_getThreadId"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_getThreadId(tso: StgPtr) -> StgThreadID {
    unsafe { sys::rts_getThreadId(tso) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_enableThreadAllocationLimit")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(tso: StgPtr) {
    unsafe { sys::rts_enableThreadAllocationLimit(tso) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_rts_disableThreadAllocationLimit")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(tso: StgPtr) {
    unsafe { sys::rts_disableThreadAllocationLimit(tso) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_listThreads"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn listThreads(cap: *mut Capability) -> *mut _StgMutArrPtrs {
    unsafe { transmute(sys::listThreads(cap as *mut sys::Capability)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_forkProcess"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn forkProcess(entry: *mut HsStablePtr) -> pid_t {
    unsafe { sys::forkProcess(entry) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_rtsSupportsBoundThreads"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    unsafe { sys::rtsSupportsBoundThreads() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_n_capabilities"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut n_capabilities: c_uint = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_enabled_capabilities"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut enabled_capabilities: u32 = 0;

// TODO: MainCapability
// #[cfg_attr(feature = "sys", unsafe(export_name = "rust_MainCapability"))]
// #[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
// pub static mut MainCapability: Capability = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_setNumCapabilities"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn setNumCapabilities(new_: u32) {
    unsafe { sys::setNumCapabilities(new_) }
}
