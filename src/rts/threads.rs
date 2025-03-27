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
    unsafe { transmute(sys::createThread(&mut cap.into(), stack_size.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn scheduleWaitThread(
    tso: *mut StgTSO,
    ret: *mut HaskellObj,
    cap: *mut *mut Capability,
) {
    unsafe {
        transmute(sys::scheduleWaitThread(
            &mut tso.into(),
            &mut ret.into(),
            &mut &mut cap.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn createGenThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createGenThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createIOThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn createStrictIOThread(
    cap: *mut Capability,
    stack_size: W_,
    closure: *mut StgClosure,
) -> *mut StgTSO {
    unsafe {
        transmute(sys::createStrictIOThread(
            &mut cap.into(),
            stack_size.into(),
            &mut closure.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn suspendThread(
    arg1: *mut StgRegTable,
    interruptible: bool,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::suspendThread(&mut arg1.into(), interruptible.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn resumeThread(arg1: *mut ::core::ffi::c_void) -> *mut StgRegTable {
    unsafe { transmute(sys::resumeThread(&mut arg1.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn eq_thread(tso1: StgPtr, tso2: StgPtr) -> bool {
    unsafe { transmute(sys::eq_thread(tso1.into(), tso2.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn cmp_thread(tso1: StgPtr, tso2: StgPtr) -> ::core::ffi::c_int {
    unsafe { transmute(sys::cmp_thread(tso1.into(), tso2.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_getThreadId(tso: StgPtr) -> StgThreadID {
    unsafe { transmute(sys::rts_getThreadId(tso.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_enableThreadAllocationLimit(tso: StgPtr) {
    unsafe { transmute(sys::rts_enableThreadAllocationLimit(tso.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rts_disableThreadAllocationLimit(tso: StgPtr) {
    unsafe { transmute(sys::rts_disableThreadAllocationLimit(tso.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn listThreads(cap: *mut Capability) -> *mut _StgMutArrPtrs {
    unsafe { transmute(sys::listThreads(&mut cap.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn forkProcess(entry: *mut HsStablePtr) -> pid_t {
    unsafe { transmute(sys::forkProcess(&mut entry.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn rtsSupportsBoundThreads() -> HsBool {
    unsafe { transmute(sys::rtsSupportsBoundThreads()) }
}

#[unsafe(no_mangle)]
pub static mut n_capabilities: ::core::ffi::c_uint = sys::n_capabilities;

#[unsafe(no_mangle)]
pub static mut enabled_capabilities: u32 = sys::enabled_capabilities;

#[unsafe(no_mangle)]
pub static mut MainCapability: Capability = sys::MainCapability;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn setNumCapabilities(new_: u32) {
    unsafe { transmute(sys::setNumCapabilities(new_.into())) }
}
