use crate::ffi::hs_ffi::{HsInt, HsWord64};
use crate::ffi::stg::types::StgStablePtr;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetGHCConcSignalSignalHandlerStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetGHCConcWindowsPendingDelaysStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetGHCConcWindowsIOManagerThreadStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(ptr: StgStablePtr) -> StgStablePtr {
    sys! {
        getOrSetGHCConcWindowsProddingStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetSystemEventThreadEventManagerStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetSystemEventThreadIOManagerThreadStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetSystemTimerThreadEventManagerStore(ptr)
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    ptr: StgStablePtr,
) -> StgStablePtr {
    sys! {
        getOrSetSystemTimerThreadIOManagerThreadStore(ptr)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(ptr: StgStablePtr) -> StgStablePtr {
    sys! {
        getOrSetLibHSghcFastStringTable(ptr)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasPprDebug(ptr: StgStablePtr) -> StgStablePtr {
    sys! {
        getOrSetLibHSghcGlobalHasPprDebug(ptr)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoDebugOutput(ptr: StgStablePtr) -> StgStablePtr {
    sys! {
        getOrSetLibHSghcGlobalHasNoDebugOutput(ptr)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoStateHack(ptr: StgStablePtr) -> StgStablePtr {
    sys! {
        getOrSetLibHSghcGlobalHasNoStateHack(ptr)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ghc_unique_counter64: HsWord64 = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ghc_unique_inc: HsInt = 0;
