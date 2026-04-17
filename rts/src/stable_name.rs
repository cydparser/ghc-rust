use crate::ffi::rts::_assertFail;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::stable_name::snEntry;
use crate::ffi::rts::storage::closure_macros::{GET_CLOSURE_TAG, UNTAG_CLOSURE, get_itbl};
use crate::ffi::rts::storage::closures::StgInd;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::P_;
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::hash::{
    HashTable, allocHashTable, freeHashTable, insertHashTable, keyCountHashTable, lookupHashTable,
    removeHashTable,
};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes, stgReallocBytes};
use crate::sm::gc::{evac_fn, isAlive};
use crate::trace::{DEBUG_RTS, trace_};

static mut stable_name_table: *mut snEntry = null_mut::<snEntry>();

static mut stable_name_free: *mut snEntry = null_mut::<snEntry>();

static mut SNT_size: u32 = 0;

const INIT_SNT_SIZE: i32 = 64;

static mut stable_name_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

static mut addrToStableHash: *mut HashTable = null_mut::<HashTable>();

unsafe fn stableNameLock() {
    initStableNameTable();

    let mut __r = pthread_mutex_lock(&raw mut stable_name_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/StableName.c".as_ptr(),
            45,
            __r,
        );
    }
}

unsafe fn stableNameUnlock() {
    if pthread_mutex_unlock(&raw mut stable_name_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/StableName.c".as_ptr(),
            51,
        );
    }
}

unsafe fn initSnEntryFreeList(mut table: *mut snEntry, mut n: u32, mut free: *mut snEntry) {
    let mut p = null_mut::<snEntry>();
    p = table.offset(n as isize).offset(-1);

    while p >= table {
        (*p).addr = free as P_ as StgPtr;
        (*p).old = null_mut::<StgWord>();
        (*p).sn_obj = null_mut::<StgClosure>();
        free = p;
        p = p.offset(-1);
    }

    stable_name_free = table;
}

unsafe fn initStableNameTable() {
    if SNT_size > 0 {
        return;
    }

    SNT_size = INIT_SNT_SIZE as u32;

    stable_name_table = stgMallocBytes(
        (SNT_size as usize).wrapping_mul(size_of::<snEntry>() as usize),
        c"initStableNameTable".as_ptr(),
    ) as *mut snEntry;

    initSnEntryFreeList(
        stable_name_table.offset(1),
        (INIT_SNT_SIZE - 1) as u32,
        null_mut::<snEntry>(),
    );

    addrToStableHash = allocHashTable();
    initMutex(&raw mut stable_name_mutex);
}

unsafe fn enlargeStableNameTable() {
    let mut old_SNT_size: u32 = SNT_size as u32;
    SNT_size = SNT_size.wrapping_mul(2 as u32);

    stable_name_table = stgReallocBytes(
        stable_name_table as *mut c_void,
        (SNT_size as usize).wrapping_mul(size_of::<snEntry>() as usize),
        c"enlargeStableNameTable".as_ptr(),
    ) as *mut snEntry;

    initSnEntryFreeList(
        stable_name_table.offset(old_SNT_size as isize),
        old_SNT_size,
        null_mut::<snEntry>(),
    );
}

unsafe fn exitStableNameTable() {
    if !addrToStableHash.is_null() {
        freeHashTable(addrToStableHash, None);
    }

    addrToStableHash = null_mut::<HashTable>();

    if !stable_name_table.is_null() {
        stgFree(stable_name_table as *mut c_void);
    }

    stable_name_table = null_mut::<snEntry>();
    SNT_size = 0;
    closeMutex(&raw mut stable_name_mutex);
}

unsafe fn freeSnEntry(mut sn: *mut snEntry) {
    if (*sn).sn_obj.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/StableName.c".as_ptr(), 134);
    }

    removeHashTable(addrToStableHash, (*sn).old as StgWord, null::<c_void>());
    (*sn).addr = stable_name_free as P_ as StgPtr;
    stable_name_free = sn;
}

unsafe fn removeIndirections(mut p: *mut StgClosure) -> *mut StgClosure {
    let mut q = null_mut::<StgClosure>();

    loop {
        q = UNTAG_CLOSURE(p);

        match (*get_itbl(q)).r#type {
            27 | 28 => {
                p = (&raw mut (*(q as *mut StgInd)).indirectee).load(Ordering::Acquire);
                continue;
            }
            38 => {
                p = (&raw mut (*(q as *mut StgInd)).indirectee).load(Ordering::Acquire);

                if GET_CLOSURE_TAG(p) != 0 {
                    continue;
                }
            }
            _ => {}
        }

        return p;
    }
}

unsafe fn lookupStableName(mut p: StgPtr) -> StgWord {
    stableNameLock();

    if stable_name_free.is_null() {
        enlargeStableNameTable();
    }

    p = removeIndirections(p as *mut StgClosure) as StgPtr;
    p = UNTAG_CLOSURE(p as *mut StgClosure) as StgPtr;

    let mut sn: StgWord = lookupHashTable(addrToStableHash, p as StgWord) as StgWord;

    if sn != 0 {
        if ((*stable_name_table.offset(sn as isize)).addr == p) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/StableName.c".as_ptr(), 197);
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.stable as i64 != 0 {
            trace_(c"cached stable name %ld at %p".as_ptr(), sn, p);
        }

        stableNameUnlock();

        return sn;
    }

    sn = stable_name_free.offset_from(stable_name_table) as i64 as StgWord;
    stable_name_free = (*stable_name_free).addr as *mut snEntry;

    let ref mut fresh6 = (*stable_name_table.offset(sn as isize)).addr;
    *fresh6 = p;

    let ref mut fresh7 = (*stable_name_table.offset(sn as isize)).sn_obj;
    *fresh7 = null_mut::<StgClosure>();
    insertHashTable(addrToStableHash, p as StgWord, sn as *mut c_void);
    stableNameUnlock();

    return sn;
}

unsafe fn rememberOldStableNameAddresses() {
    let mut p = null_mut::<snEntry>();
    let mut __end_ptr: *mut snEntry = stable_name_table.offset(SNT_size as isize) as *mut snEntry;
    p = stable_name_table.offset(1);

    while p < __end_ptr {
        if (*p).addr < stable_name_table as P_ || (*p).addr >= __end_ptr as P_ {
            (*p).old = (*p).addr;
        }

        p = p.offset(1);
    }
}

unsafe fn threadStableNameTable(mut evac: evac_fn, mut user: *mut c_void) {
    let mut p = null_mut::<snEntry>();
    let mut __end_ptr: *mut snEntry = stable_name_table.offset(SNT_size as isize) as *mut snEntry;
    p = stable_name_table.offset(1);

    while p < __end_ptr {
        if (*p).addr < stable_name_table as P_ || (*p).addr >= __end_ptr as P_ {
            if !(*p).sn_obj.is_null() {
                evac.expect("non-null function pointer")(user, &raw mut (*p).sn_obj);
            }

            if !(*p).addr.is_null() {
                evac.expect("non-null function pointer")(
                    user,
                    &raw mut (*p).addr as *mut *mut StgClosure,
                );
            }
        }

        p = p.offset(1);
    }
}

unsafe fn gcStableNameTable() {
    stableNameLock();

    let mut p = null_mut::<snEntry>();
    let mut __end_ptr: *mut snEntry = stable_name_table.offset(SNT_size as isize) as *mut snEntry;
    p = stable_name_table.offset(1);

    while p < __end_ptr {
        if (*p).addr < stable_name_table as P_ || (*p).addr >= __end_ptr as P_ {
            if !(*p).sn_obj.is_null() {
                (*p).sn_obj = isAlive((*p).sn_obj);

                if (*p).sn_obj.is_null() {
                    if 1 != 0 && RtsFlags.DebugFlags.stable as i64 != 0 {
                        trace_(
                            c"GC'd StableName %ld (addr=%p)".as_ptr(),
                            p.offset_from(stable_name_table) as i64,
                            (*p).addr,
                        );
                    }

                    freeSnEntry(p);
                } else if !(*p).addr.is_null() {
                    (*p).addr = isAlive((*p).addr as *mut StgClosure) as StgPtr;

                    if (*p).addr.is_null() {
                        if 1 != 0 && RtsFlags.DebugFlags.stable as i64 != 0 {
                            trace_(
                                c"GC'd pointee %ld".as_ptr(),
                                p.offset_from(stable_name_table) as i64,
                            );
                        }
                    }
                }
            }
        }

        p = p.offset(1);
    }

    stableNameUnlock();
}

unsafe fn updateStableNameTable(mut full: bool) {
    if full as i32 != 0 && !addrToStableHash.is_null() && 0 != keyCountHashTable(addrToStableHash) {
        freeHashTable(addrToStableHash, None);
        addrToStableHash = allocHashTable();
    }

    if full {
        let mut p = null_mut::<snEntry>();
        let mut __end_ptr: *mut snEntry =
            stable_name_table.offset(SNT_size as isize) as *mut snEntry;
        p = stable_name_table.offset(1);

        while p < __end_ptr {
            if (*p).addr < stable_name_table as P_ || (*p).addr >= __end_ptr as P_ {
                if !(*p).addr.is_null() {
                    insertHashTable(
                        addrToStableHash,
                        (*p).addr as StgWord,
                        p.offset_from(stable_name_table) as i64 as *mut c_void,
                    );
                }
            }

            p = p.offset(1);
        }
    } else {
        let mut p_0 = null_mut::<snEntry>();
        let mut __end_ptr_0: *mut snEntry =
            stable_name_table.offset(SNT_size as isize) as *mut snEntry;
        p_0 = stable_name_table.offset(1);

        while p_0 < __end_ptr_0 {
            if (*p_0).addr < stable_name_table as P_ || (*p_0).addr >= __end_ptr_0 as P_ {
                if (*p_0).addr != (*p_0).old {
                    removeHashTable(addrToStableHash, (*p_0).old as StgWord, null::<c_void>());

                    if !(*p_0).addr.is_null() {
                        insertHashTable(
                            addrToStableHash,
                            (*p_0).addr as StgWord,
                            p_0.offset_from(stable_name_table) as i64 as *mut c_void,
                        );
                    }
                }
            }

            p_0 = p_0.offset(1);
        }
    };
}
