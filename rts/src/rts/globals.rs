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

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcSignalSignalHandlerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcSignalSignalHandlerStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsPendingDelaysStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsProddingStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsProddingStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemEventThreadEventManagerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemEventThreadEventManagerStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemEventThreadIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemEventThreadIOManagerThreadStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemTimerThreadEventManagerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemTimerThreadEventManagerStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemTimerThreadIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemTimerThreadIOManagerThreadStore(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetLibHSghcFastStringTable")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetLibHSghcFastStringTable(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetLibHSghcGlobalHasPprDebug")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasPprDebug(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetLibHSghcGlobalHasPprDebug(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetLibHSghcGlobalHasNoDebugOutput")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoDebugOutput(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetLibHSghcGlobalHasNoDebugOutput(ptr) }
}

#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetLibHSghcGlobalHasNoStateHack")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoStateHack(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetLibHSghcGlobalHasNoStateHack(ptr) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ghc_unique_counter64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ghc_unique_counter64: HsWord64 = 0;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ghc_unique_inc"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ghc_unique_inc: HsInt = 0;
