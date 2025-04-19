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
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetGHCConcSignalSignalHandlerStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetGHCConcWindowsProddingStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetSystemEventThreadEventManagerStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetSystemEventThreadIOManagerThreadStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetSystemTimerThreadEventManagerStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetSystemTimerThreadIOManagerThreadStore(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetLibHSghcFastStringTable(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasPprDebug(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetLibHSghcGlobalHasPprDebug(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoDebugOutput(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoStateHack(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { transmute(sys::getOrSetLibHSghcGlobalHasNoStateHack(ptr)) }
}

#[unsafe(no_mangle)]
pub static mut ghc_unique_counter64: HsWord64 = unsafe { sys::ghc_unique_counter64 };

#[unsafe(no_mangle)]
pub static mut ghc_unique_inc: HsInt = unsafe { sys::ghc_unique_inc };
