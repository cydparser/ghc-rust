use crate::hs_ffi::{HsInt, HsWord64};
use crate::prelude::*;
use crate::stg::types::StgStablePtr;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcSignalSignalHandlerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcSignalSignalHandlerStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsPendingDelaysStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetGHCConcWindowsProddingStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetGHCConcWindowsProddingStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemEventThreadEventManagerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemEventThreadEventManagerStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemEventThreadIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemEventThreadIOManagerThreadStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemTimerThreadEventManagerStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemTimerThreadEventManagerStore(ptr) }
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetSystemTimerThreadIOManagerThreadStore")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    unsafe { sys::getOrSetSystemTimerThreadIOManagerThreadStore(ptr) }
}

/// - GHC_PLACES: {compiler}
#[cfg_attr(
    feature = "sys",
    unsafe(export_name = "rust_getOrSetLibHSghcFastStringTable")
)]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> StgStablePtr {
    unsafe { sys::getOrSetLibHSghcFastStringTable(ptr) }
}

/// - GHC_PLACES: {compiler}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ghc_unique_counter64"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ghc_unique_counter64: HsWord64 = 0;

/// - GHC_PLACES: {compiler}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_ghc_unique_inc"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
pub static mut ghc_unique_inc: HsInt = 0;
