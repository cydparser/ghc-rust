use crate::hs_ffi::{HsInt, HsWord64};
use crate::prelude::*;
use crate::stg::types::StgStablePtr;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetGHCConcSignalSignalHandlerStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetGHCConcSignalSignalHandlerStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetGHCConcWindowsPendingDelaysStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetGHCConcWindowsPendingDelaysStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetGHCConcWindowsIOManagerThreadStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetGHCConcWindowsIOManagerThreadStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetGHCConcWindowsProddingStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetGHCConcWindowsProddingStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetSystemEventThreadEventManagerStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetSystemEventThreadEventManagerStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetSystemEventThreadIOManagerThreadStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetSystemEventThreadIOManagerThreadStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetSystemTimerThreadEventManagerStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetSystemTimerThreadEventManagerStore")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetSystemTimerThreadIOManagerThreadStore(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetSystemTimerThreadIOManagerThreadStore")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> StgStablePtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getOrSetLibHSghcFastStringTable(ptr)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getOrSetLibHSghcFastStringTable")
}

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
pub static mut ghc_unique_counter64: HsWord64 = 0;

/// - GHC_PLACES: {compiler}
#[ffi]
#[unsafe(no_mangle)]
pub static mut ghc_unique_inc: HsInt = 0;
