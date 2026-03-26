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

static mut spt: *mut HashTable = null::<HashTable>() as *mut HashTable;

#[inline]
unsafe fn hashFingerprint(mut table: *const HashTable, mut key: StgWord) -> c_int {
    let mut ptr: *const StgWord64 = key as *mut StgWord64;

    return hashWord(table, *ptr.offset(1 as c_int as isize));
}

#[inline]
unsafe fn compareFingerprint(mut a: StgWord, mut b: StgWord) -> c_int {
    let mut ptra: *const StgWord64 = a as *mut StgWord64;
    let mut ptrb: *const StgWord64 = b as *mut StgWord64;

    return (*ptra == *ptrb
        && *ptra.offset(1 as c_int as isize) == *ptrb.offset(1 as c_int as isize))
        as c_int;
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
    }

    insertHashTable_(
        spt,
        key as StgWord,
        entry as *const c_void,
        Some(hashFingerprint as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
    );
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_insert(mut key: *mut StgWord64, mut spe_closure: *mut c_void) {
    let mut entry = stgMallocBytes(
        size_of::<StgStablePtr>() as size_t,
        b"hs_spt_insert: entry\0" as *const u8 as *const c_char as *mut c_char,
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
        let mut entry = removeHashTable_(
            spt,
            key as StgWord,
            null::<c_void>(),
            Some(hashFingerprint as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
            Some(compareFingerprint as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
        ) as *mut StgStablePtr;

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
        let ret = keysHashTable(spt, keys as *mut StgWord, szKeys) as c_int;

        return ret;
    } else {
        return 0 as c_int;
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_spt_key_count() -> c_int {
    return if !spt.is_null() {
        keyCountHashTable(spt)
    } else {
        0 as c_int
    };
}

unsafe fn exitStaticPtrTable() {
    if !spt.is_null() {
        freeHashTable(
            spt,
            Some(freeSptEntry as unsafe extern "C" fn(*mut c_void) -> ()),
        );

        spt = null_mut::<HashTable>();
    }
}
