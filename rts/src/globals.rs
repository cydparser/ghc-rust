use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::stg::types::StgStablePtr;
use crate::hs_ffi::{HsInt, HsWord64};
use crate::prelude::*;
use crate::stable_ptr::freeStablePtr;

#[cfg(test)]
mod tests;

type StoreKey = u32;

const MaxStoreKey: StoreKey = 12;

const LibHSghcGlobalHasNoStateHack: StoreKey = 11;

const LibHSghcGlobalHasNoDebugOutput: StoreKey = 10;

const LibHSghcGlobalHasPprDebug: StoreKey = 9;

const LibHSghcFastStringTable: StoreKey = 8;

const SystemTimerThreadIOManagerThreadStore: StoreKey = 7;

const SystemTimerThreadEventManagerStore: StoreKey = 6;

const SystemEventThreadIOManagerThreadStore: StoreKey = 5;

const SystemEventThreadEventManagerStore: StoreKey = 4;

const GHCConcWindowsProddingStore: StoreKey = 3;

const GHCConcWindowsIOManagerThreadStore: StoreKey = 2;

const GHCConcWindowsPendingDelaysStore: StoreKey = 1;

const GHCConcSignalSignalHandlerStore: StoreKey = 0;

static mut globalStoreLock: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut store: [StgStablePtr; 12] = [null_mut::<c_void>(); 12];

unsafe fn initGlobalStore() {
    let mut i: u32 = 0;
    i = 0;

    while i < MaxStoreKey as i32 as u32 {
        store[i as usize] = null_mut::<c_void>();
        i = i.wrapping_add(1);
    }

    initMutex(&raw mut globalStoreLock);
}

unsafe fn exitGlobalStore() {
    let mut i: u32 = 0;
    closeMutex(&raw mut globalStoreLock);
    i = 0;

    while i < MaxStoreKey as i32 as u32 {
        if !store[i as usize].is_null() {
            freeStablePtr(store[i as usize]);
            store[i as usize] = null_mut::<c_void>();
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn getOrSetKey(mut key: StoreKey, mut ptr: StgStablePtr) -> StgStablePtr {
    let mut ret = store[key as usize];

    if ret.is_null() {
        let mut __r = pthread_mutex_lock(&raw mut globalStoreLock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Globals.c".as_ptr(),
                80,
                __r,
            );
        }

        ret = store[key as usize];

        if ret.is_null() {
            ret = ptr;
            store[key as usize] = ret;
        }

        if pthread_mutex_unlock(&raw mut globalStoreLock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Globals.c".as_ptr(),
                87,
            );
        }
    }

    return ret;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcSignalSignalHandlerStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(GHCConcSignalSignalHandlerStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsPendingDelaysStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(GHCConcWindowsPendingDelaysStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsIOManagerThreadStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(GHCConcWindowsIOManagerThreadStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetGHCConcWindowsProddingStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(GHCConcWindowsProddingStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadEventManagerStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(SystemEventThreadEventManagerStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemEventThreadIOManagerThreadStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(SystemEventThreadIOManagerThreadStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadEventManagerStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(SystemTimerThreadEventManagerStore, ptr);
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetSystemTimerThreadIOManagerThreadStore(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(SystemTimerThreadIOManagerThreadStore, ptr);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcFastStringTable(mut ptr: StgStablePtr) -> StgStablePtr {
    return getOrSetKey(LibHSghcFastStringTable, ptr);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasPprDebug(mut ptr: StgStablePtr) -> StgStablePtr {
    return getOrSetKey(LibHSghcGlobalHasPprDebug, ptr);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoDebugOutput(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(LibHSghcGlobalHasNoDebugOutput, ptr);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getOrSetLibHSghcGlobalHasNoStateHack(
    mut ptr: StgStablePtr,
) -> StgStablePtr {
    return getOrSetKey(LibHSghcGlobalHasNoStateHack, ptr);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ghc_unique_counter64: HsWord64 = 0;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut ghc_unique_inc: HsInt = 1;
