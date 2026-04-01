use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::stable_ptr::{deRefStablePtr, getStablePtr};
use crate::ffi::stg::types::{StgPtr, StgStablePtr, StgWord, StgWord64};
use crate::hash::{
    HashTable, allocHashTable, freeHashTable, hashWord, insertHashTable_, keyCountHashTable,
    keysHashTable, lookupHashTable_, removeHashTable_,
};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::stable_ptr::freeStablePtr;

#[cfg(test)]
mod tests;

static mut spt: *mut HashTable = null_mut::<HashTable>();

static mut spt_lock: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn hashFingerprint(mut table: *const HashTable, mut key: StgWord) -> i32 {
    let mut ptr: *const StgWord64 = key as *mut StgWord64;

    return hashWord(table, *ptr.offset(1));
}

unsafe fn compareFingerprint(mut a: StgWord, mut b: StgWord) -> i32 {
    let mut ptra: *const StgWord64 = a as *mut StgWord64;
    let mut ptrb: *const StgWord64 = b as *mut StgWord64;

    return (*ptra == *ptrb && *ptra.offset(1) == *ptrb.offset(1)) as i32;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert_stableptr(
    mut key: *mut StgWord64,
    mut entry: *mut StgStablePtr,
) {
    if spt.is_null() {
        spt = allocHashTable();
        initMutex(&raw mut spt_lock);
    }

    let mut __r = pthread_mutex_lock(&raw mut spt_lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/StaticPtrTable.c".as_ptr(),
            47,
            __r,
        );
    }

    insertHashTable_(
        spt,
        key as StgWord,
        entry as *const c_void,
        Some(hashFingerprint as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
    );

    if pthread_mutex_unlock(&raw mut spt_lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/StaticPtrTable.c".as_ptr(),
            49,
        );
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert(mut key: *mut StgWord64, mut spe_closure: *mut c_void) {
    let mut entry = stgMallocBytes(
        size_of::<StgStablePtr>() as usize,
        c"hs_spt_insert: entry".as_ptr(),
    ) as *mut StgStablePtr;

    *entry = getStablePtr(spe_closure as StgPtr);
    hs_spt_insert_stableptr(key, entry);
}

unsafe fn freeSptEntry(mut entry: *mut c_void) {
    freeStablePtr(*(entry as *mut StgStablePtr));
    stgFree(entry);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_remove(mut key: *mut StgWord64) {
    if !spt.is_null() {
        let mut __r = pthread_mutex_lock(&raw mut spt_lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                70,
                __r,
            );
        }

        let mut entry = removeHashTable_(
            spt,
            key as StgWord,
            null::<c_void>(),
            Some(hashFingerprint as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
            Some(compareFingerprint as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
        ) as *mut StgStablePtr;

        if pthread_mutex_unlock(&raw mut spt_lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                73,
            );
        }

        if !entry.is_null() {
            freeSptEntry(entry as *mut c_void);
        }
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_lookup(mut key: *mut StgWord64) -> StgPtr {
    if !spt.is_null() {
        let mut __r = pthread_mutex_lock(&raw mut spt_lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                82,
                __r,
            );
        }

        let mut entry = lookupHashTable_(
            spt,
            key as StgWord,
            Some(hashFingerprint as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
            Some(compareFingerprint as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
        ) as *const StgStablePtr;

        let ret = if !entry.is_null() {
            deRefStablePtr(*entry) as StgPtr
        } else {
            null_mut::<StgWord>()
        };

        if pthread_mutex_unlock(&raw mut spt_lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                86,
            );
        }

        return ret;
    } else {
        return null_mut::<StgWord>();
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_keys(mut keys: *mut StgPtr, mut szKeys: c_int) -> c_int {
    if !spt.is_null() {
        let mut __r = pthread_mutex_lock(&raw mut spt_lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                94,
                __r,
            );
        }

        let ret = keysHashTable(spt, keys as *mut StgWord, szKeys) as i32;

        if pthread_mutex_unlock(&raw mut spt_lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/StaticPtrTable.c".as_ptr(),
                96,
            );
        }

        return ret;
    } else {
        return 0;
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_key_count() -> c_int {
    return if !spt.is_null() {
        keyCountHashTable(spt)
    } else {
        0
    };
}

unsafe fn exitStaticPtrTable() {
    if !spt.is_null() {
        freeHashTable(
            spt,
            Some(freeSptEntry as unsafe extern "C" fn(*mut c_void) -> ()),
        );

        spt = null_mut::<HashTable>();
        closeMutex(&raw mut spt_lock);
    }
}
