use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{
    Condition, Mutex, closeCondition, closeMutex, initCondition, initMutex, signalCondition,
    waitCondition,
};
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

pub(crate) type alloc_thing_fn = Option<unsafe extern "C" fn() -> *mut c_void>;

pub(crate) type free_thing_fn = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

pub(crate) type Pool = Pool_;

/// cbindgen:no-export
struct Pool_ {
    max_size: u32,
    desired_size: u32,
    current_size: u32,
    cond: Condition,
    alloc_fn: alloc_thing_fn,
    free_fn: free_thing_fn,
    available: *mut PoolEntry,
    taken: *mut PoolEntry,
    mutex: Mutex,
}

type PoolEntry = PoolEntry_;

/// cbindgen:no-export
struct PoolEntry_ {
    next: *mut PoolEntry_,
    thing: *mut c_void,
    flags: StgWord,
}

const FLAG_SHOULD_FREE: i32 = 1 << 0;

unsafe fn poolInit(
    mut max_size: u32,
    mut desired_size: u32,
    mut alloc_fn: alloc_thing_fn,
    mut free_fn: free_thing_fn,
) -> *mut Pool {
    let mut pool = stgMallocBytes(size_of::<Pool>() as usize, c"pool_init".as_ptr()) as *mut Pool;
    (*pool).max_size = if max_size == 0 { -1 as u32 } else { max_size };
    (*pool).desired_size = desired_size;
    (*pool).current_size = 0;
    (*pool).alloc_fn = alloc_fn;
    (*pool).free_fn = free_fn;
    (*pool).available = null_mut::<PoolEntry>();
    (*pool).taken = null_mut::<PoolEntry>();
    initMutex(&raw mut (*pool).mutex);
    initCondition(&raw mut (*pool).cond);

    return pool;
}

unsafe fn poolFree(mut pool: *mut Pool) -> i32 {
    if !(*pool).taken.is_null() {
        return 1;
    }

    poolSetMaxSize(pool, 0);
    closeCondition(&raw mut (*pool).cond);
    closeMutex(&raw mut (*pool).mutex);
    stgFree(pool as *mut c_void);

    return 0;
}

unsafe fn free_available(mut pool: *mut Pool, mut size: u32) {
    while (*pool).current_size > size && !(*pool).available.is_null() {
        let mut ent = (*pool).available;
        (*pool).free_fn.expect("non-null function pointer")((*ent).thing);
        (*pool).available = (*ent).next as *mut PoolEntry;
        stgFree(ent as *mut c_void);
        (*pool).current_size = (*pool).current_size.wrapping_sub(1);
    }
}

unsafe fn poolSetDesiredSize(mut pool: *mut Pool, mut size: u32) {
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            88,
            __r,
        );
    }

    (*pool).desired_size = size;
    free_available(pool, size);

    if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            91,
        );
    }
}

unsafe fn poolSetMaxSize(mut pool: *mut Pool, mut size: u32) {
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            95,
            __r,
        );
    }

    if size == 0 {
        size = -1 as u32;
    }

    (*pool).max_size = size;

    if (*pool).desired_size > (*pool).max_size {
        (*pool).desired_size = size;
        free_available(pool, size);
    }

    if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            103,
        );
    }
}

unsafe fn poolGetMaxSize(mut pool: *mut Pool) -> u32 {
    return (*pool).max_size;
}

unsafe fn poolGetDesiredSize(mut pool: *mut Pool) -> u32 {
    return (*pool).desired_size;
}

unsafe fn poolTryTake_(mut pool: *mut Pool) -> *mut PoolEntry {
    let mut ent = null_mut::<PoolEntry>();

    if !(*pool).available.is_null() {
        ent = (*pool).available;
        (*pool).available = (*ent).next as *mut PoolEntry;
    } else if (*pool).current_size < (*pool).max_size {
        ent = stgMallocBytes(size_of::<PoolEntry>() as usize, c"pool_take".as_ptr())
            as *mut PoolEntry;
        (*ent).flags = 0;
        (*ent).thing = (*pool).alloc_fn.expect("non-null function pointer")();
        (*pool).current_size = (*pool).current_size.wrapping_add(1);
    } else {
        return null_mut::<PoolEntry>();
    }

    (*ent).next = (*pool).taken as *mut PoolEntry_;
    (*pool).taken = ent;

    return ent;
}

unsafe fn poolTryTake(mut pool: *mut Pool) -> *mut c_void {
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            136,
            __r,
        );
    }

    let mut ent = poolTryTake_(pool);

    if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            138,
        );
    }

    return if !ent.is_null() { (*ent).thing } else { NULL };
}

unsafe fn poolTake(mut pool: *mut Pool) -> *mut c_void {
    let mut ent = null_mut::<PoolEntry>();
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            144,
            __r,
        );
    }

    while ent.is_null() {
        ent = poolTryTake_(pool);

        if ent.is_null() {
            waitCondition(&raw mut (*pool).cond, &raw mut (*pool).mutex);
        }
    }

    if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            156,
        );
    }

    return (*ent).thing;
}

unsafe fn poolRelease(mut pool: *mut Pool, mut thing: *mut c_void) {
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            161,
            __r,
        );
    }

    let mut last: *mut *mut PoolEntry = &raw mut (*pool).taken;
    let mut ent = (*pool).taken;

    while !ent.is_null() {
        if (*ent).thing == thing {
            *last = (*ent).next as *mut PoolEntry;

            if (*pool).current_size > (*pool).desired_size
                || (*ent).flags & FLAG_SHOULD_FREE as StgWord != 0
            {
                (*pool).free_fn.expect("non-null function pointer")((*ent).thing);
                stgFree(ent as *mut c_void);
            } else {
                (*ent).next = (*pool).available as *mut PoolEntry_;
                (*pool).available = ent;
                signalCondition(&raw mut (*pool).cond);
            }

            if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Pool.c".as_ptr(),
                    179,
                );
            }

            return;
        }

        last = &raw mut (*ent).next as *mut *mut PoolEntry;
        ent = (*ent).next as *mut PoolEntry;
    }

    barf(c"pool_release: trying to release resource which doesn't belong to pool.".as_ptr());
}

unsafe fn poolFlush(mut pool: *mut Pool) {
    let mut __r = pthread_mutex_lock(&raw mut (*pool).mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            191,
            __r,
        );
    }

    free_available(pool, 0);

    let mut ent = (*pool).taken;

    while !ent.is_null() {
        (*ent).flags |= FLAG_SHOULD_FREE as StgWord;
        ent = (*ent).next as *mut PoolEntry;
    }

    if pthread_mutex_unlock(&raw mut (*pool).mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Pool.c".as_ptr(),
            198,
        );
    }
}
