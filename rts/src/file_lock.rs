use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::hash::{
    HashTable, allocHashTable, freeHashTable, hashWord, insertHashTable, insertHashTable_,
    lookupHashTable, lookupHashTable_, removeHashTable, removeHashTable_,
};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
struct Lock {
    device: StgWord64,
    inode: StgWord64,
    readers: i32,
}

static mut obj_hash: *mut HashTable = null_mut::<HashTable>();

static mut key_hash: *mut HashTable = null_mut::<HashTable>();

static mut file_lock_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn cmpLocks(mut w1: StgWord, mut w2: StgWord) -> i32 {
    let mut l1 = w1 as *mut Lock;
    let mut l2 = w2 as *mut Lock;

    return ((*l1).device == (*l2).device && (*l1).inode == (*l2).inode) as i32;
}

unsafe fn hashLock(mut table: *const HashTable, mut w: StgWord) -> i32 {
    let mut l = w as *mut Lock;
    let mut key: StgWord = (*l).inode as StgWord
        ^ (*l).inode as StgWord >> 32
        ^ (*l).device as StgWord
        ^ (*l).device as StgWord >> 32;

    return hashWord(table, key);
}

unsafe fn initFileLocking() {
    obj_hash = allocHashTable();
    key_hash = allocHashTable();
    initMutex(&raw mut file_lock_mutex);
}

unsafe fn freeLock(mut lock: *mut c_void) {
    stgFree(lock);
}

unsafe fn freeFileLocking() {
    freeHashTable(
        obj_hash,
        Some(freeLock as unsafe extern "C" fn(*mut c_void) -> ()),
    );
    freeHashTable(key_hash, None);
    closeMutex(&raw mut file_lock_mutex);
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lockFile(
    mut id: StgWord64,
    mut dev: StgWord64,
    mut ino: StgWord64,
    mut for_writing: c_int,
) -> c_int {
    let mut key = Lock {
        device: 0,
        inode: 0,
        readers: 0,
    };

    let mut lock = null_mut::<Lock>();
    let mut __r = pthread_mutex_lock(&raw mut file_lock_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/FileLock.c".as_ptr(),
            83,
            __r,
        );
    }

    key.device = dev;
    key.inode = ino;

    lock = lookupHashTable_(
        obj_hash,
        &raw mut key as StgWord,
        Some(hashLock as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        Some(cmpLocks as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
    ) as *mut Lock;

    if lock.is_null() {
        lock = stgMallocBytes(size_of::<Lock>() as usize, c"lockFile".as_ptr()) as *mut Lock;
        (*lock).device = dev;
        (*lock).inode = ino;
        (*lock).readers = if for_writing != 0 { -1 } else { 1 };

        insertHashTable_(
            obj_hash,
            lock as StgWord,
            lock as *mut c_void,
            Some(hashLock as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
        );

        insertHashTable(key_hash, id as StgWord, lock as *const c_void);

        if pthread_mutex_unlock(&raw mut file_lock_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/FileLock.c".as_ptr(),
                98,
            );
        }

        return 0;
    } else {
        if for_writing != 0 || (*lock).readers < 0 {
            if pthread_mutex_unlock(&raw mut file_lock_mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/FileLock.c".as_ptr(),
                    105,
                );
            }

            return -1;
        }

        insertHashTable(key_hash, id as StgWord, lock as *const c_void);
        (*lock).readers += 1;

        if pthread_mutex_unlock(&raw mut file_lock_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/FileLock.c".as_ptr(),
                110,
            );
        }

        return 0;
    };
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unlockFile(mut id: StgWord64) -> c_int {
    let mut lock = null_mut::<Lock>();
    let mut __r = pthread_mutex_lock(&raw mut file_lock_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/FileLock.c".as_ptr(),
            120,
            __r,
        );
    }

    lock = lookupHashTable(key_hash, id as StgWord) as *mut Lock;

    if lock.is_null() {
        if pthread_mutex_unlock(&raw mut file_lock_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/FileLock.c".as_ptr(),
                127,
            );
        }

        return 1;
    }

    if (*lock).readers < 0 {
        (*lock).readers += 1;
    } else {
        (*lock).readers -= 1;
    }

    if (*lock).readers == 0 {
        removeHashTable_(
            obj_hash,
            lock as StgWord,
            null::<c_void>(),
            Some(hashLock as unsafe extern "C" fn(*const HashTable, StgWord) -> c_int),
            Some(cmpLocks as unsafe extern "C" fn(StgWord, StgWord) -> c_int),
        );

        stgFree(lock as *mut c_void);
    }

    removeHashTable(key_hash, id as StgWord, null::<c_void>());

    if pthread_mutex_unlock(&raw mut file_lock_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/FileLock.c".as_ptr(),
            143,
        );
    }

    return 0;
}
