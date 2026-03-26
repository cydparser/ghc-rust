use crate::ffi::rts::messages::barf;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

pub(crate) type alloc_thing_fn = Option<unsafe extern "C" fn() -> *mut c_void>;

pub(crate) type free_thing_fn = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

pub(crate) type Pool = Pool_;

/// cbindgen:no-export
struct Pool_ {
    max_size: uint32_t,
    desired_size: uint32_t,
    current_size: uint32_t,
    alloc_fn: alloc_thing_fn,
    free_fn: free_thing_fn,
    available: *mut PoolEntry,
    taken: *mut PoolEntry,
}

type PoolEntry = PoolEntry_;

/// cbindgen:no-export
struct PoolEntry_ {
    next: *mut PoolEntry_,
    thing: *mut c_void,
    flags: StgWord,
}

const FLAG_SHOULD_FREE: c_int = (1 as c_int) << 0 as c_int;

unsafe fn poolInit(
    mut max_size: uint32_t,
    mut desired_size: uint32_t,
    mut alloc_fn: alloc_thing_fn,
    mut free_fn: free_thing_fn,
) -> *mut Pool {
    let mut pool = stgMallocBytes(
        size_of::<Pool>() as size_t,
        b"pool_init\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Pool;

    (*pool).max_size = if max_size == 0 as uint32_t {
        -(1 as c_int) as uint32_t
    } else {
        max_size
    };

    (*pool).desired_size = desired_size;
    (*pool).current_size = 0 as uint32_t;
    (*pool).alloc_fn = alloc_fn;
    (*pool).free_fn = free_fn;
    (*pool).available = null_mut::<PoolEntry>();
    (*pool).taken = null_mut::<PoolEntry>();

    return pool;
}

unsafe fn poolFree(mut pool: *mut Pool) -> c_int {
    if !(*pool).taken.is_null() {
        return 1 as c_int;
    }

    poolSetMaxSize(pool, 0 as uint32_t);
    stgFree(pool as *mut c_void);

    return 0 as c_int;
}

unsafe fn free_available(mut pool: *mut Pool, mut size: uint32_t) {
    while (*pool).current_size > size && !(*pool).available.is_null() {
        let mut ent = (*pool).available;
        (*pool).free_fn.expect("non-null function pointer")((*ent).thing);
        (*pool).available = (*ent).next as *mut PoolEntry;
        stgFree(ent as *mut c_void);
        (*pool).current_size = (*pool).current_size.wrapping_sub(1);
    }
}

unsafe fn poolSetDesiredSize(mut pool: *mut Pool, mut size: uint32_t) {
    (*pool).desired_size = size;
    free_available(pool, size);
}

unsafe fn poolSetMaxSize(mut pool: *mut Pool, mut size: uint32_t) {
    if size == 0 as uint32_t {
        size = -(1 as c_int) as uint32_t;
    }

    (*pool).max_size = size;

    if (*pool).desired_size > (*pool).max_size {
        (*pool).desired_size = size;
        free_available(pool, size);
    }
}

unsafe fn poolGetMaxSize(mut pool: *mut Pool) -> uint32_t {
    return (*pool).max_size;
}

unsafe fn poolGetDesiredSize(mut pool: *mut Pool) -> uint32_t {
    return (*pool).desired_size;
}

unsafe fn poolTryTake_(mut pool: *mut Pool) -> *mut PoolEntry {
    let mut ent = null_mut::<PoolEntry>();

    if !(*pool).available.is_null() {
        ent = (*pool).available;
        (*pool).available = (*ent).next as *mut PoolEntry;
    } else if (*pool).current_size < (*pool).max_size {
        ent = stgMallocBytes(
            size_of::<PoolEntry>() as size_t,
            b"pool_take\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut PoolEntry;

        (*ent).flags = 0 as StgWord;
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
    let mut ent = poolTryTake_(pool);

    return if !ent.is_null() { (*ent).thing } else { NULL };
}

unsafe fn poolTake(mut pool: *mut Pool) -> *mut c_void {
    let mut ent = null_mut::<PoolEntry>();

    while ent.is_null() {
        ent = poolTryTake_(pool);

        if ent.is_null() {
            barf(b"Tried to take from an empty pool\0" as *const u8 as *const c_char);
        }
    }

    return (*ent).thing;
}

unsafe fn poolRelease(mut pool: *mut Pool, mut thing: *mut c_void) {
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
            }

            return;
        }

        last = &raw mut (*ent).next as *mut *mut PoolEntry;
        ent = (*ent).next as *mut PoolEntry;
    }

    barf(
        b"pool_release: trying to release resource which doesn't belong to pool.\0" as *const u8
            as *const c_char,
    );
}

unsafe fn poolFlush(mut pool: *mut Pool) {
    free_available(pool, 0 as uint32_t);

    let mut ent = (*pool).taken;

    while !ent.is_null() {
        (*ent).flags |= FLAG_SHOULD_FREE as StgWord;
        ent = (*ent).next as *mut PoolEntry;
    }
}
