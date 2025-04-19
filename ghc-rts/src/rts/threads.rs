use crate::rts::storage::tso;
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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createThread(cap: *mut Capability, stack_size: W_) -> *mut StgTSO {
    unsafe { transmute(sys::createThread(cap, stack_size)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn scheduleWaitThread(
    tso: *mut StgTSO,
    ret: *mut HaskellObj,
    cap: *mut *mut Capability,
) {
    unsafe { sys::scheduleWaitThread(tso, ret, cap) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createGenThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe { transmute(sys::createGenThread(cap, stack_size, closure)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe { transmute(sys::createIOThread(cap, stack_size, closure)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createStrictIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe { transmute(sys::createStrictIOThread(cap, stack_size, closure)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn suspendThread(
    arg1: *mut StgRegTable,
    interruptible: bool,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::suspendThread(arg1, interruptible)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn resumeThread(arg1: *mut ::core::ffi::c_void) -> *mut StgRegTable {
    unsafe { transmute(sys::resumeThread(arg1)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    unsafe { transmute(sys::eq_thread(tso1, tso2)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn cmp_thread(tso1: StgPtr, tso2: StgPtr) -> ::core::ffi::c_int {
    unsafe { transmute(sys::cmp_thread(tso1, tso2)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getThreadId(tso: StgPtr) -> StgThreadID {
    unsafe { transmute(sys::rts_getThreadId(tso)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(tso: StgPtr) {
    unsafe { sys::rts_enableThreadAllocationLimit(tso) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(tso: StgPtr) {
    unsafe { sys::rts_disableThreadAllocationLimit(tso) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn listThreads(cap: *mut Capability) -> *mut _StgMutArrPtrs {
    unsafe { transmute(sys::listThreads(cap)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn forkProcess(entry: *mut HsStablePtr) -> pid_t {
    unsafe { transmute(sys::forkProcess(entry)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    unsafe { transmute(sys::rtsSupportsBoundThreads()) }
}

#[unsafe(no_mangle)]
pub static mut n_capabilities: ::core::ffi::c_uint = unsafe { sys::n_capabilities };

#[unsafe(no_mangle)]
pub static mut enabled_capabilities: u32 = unsafe { sys::enabled_capabilities };

#[unsafe(no_mangle)]
pub static mut MainCapability: Capability = unsafe { sys::MainCapability };

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setNumCapabilities(new_: u32) {
    unsafe { sys::setNumCapabilities(new_) }
}
