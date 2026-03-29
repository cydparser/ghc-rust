use crate::ffi::rts::stable_ptr::spEntry;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::P_;
use crate::ffi::stg::types::{StgPtr, StgStablePtr, StgWord};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::gc::evac_fn;

static mut stable_ptr_table: *mut spEntry = null_mut::<spEntry>();

static mut stable_ptr_free: *mut spEntry = null_mut::<spEntry>();

static mut SPT_size: u32 = 0;

const INIT_SPT_SIZE: i32 = 64;

static mut old_SPTs: [*mut spEntry; 64] = [null_mut::<spEntry>(); 64];

static mut n_old_SPTs: u32 = 0;

unsafe fn stablePtrLock() {
    initStablePtrTable();
}

unsafe fn stablePtrUnlock() {}

#[inline]
unsafe fn initSpEntryFreeList(mut table: *mut spEntry, mut n: u32) {
    let mut free = null_mut::<spEntry>();
    let mut p = null_mut::<spEntry>();
    p = table.offset(n as isize).offset(-1);

    while p >= table {
        (*p).addr = free as P_ as StgPtr;
        free = p;
        p = p.offset(-1);
    }

    stable_ptr_free = table;
}

unsafe fn initStablePtrTable() {
    if SPT_size > 0 {
        return;
    }

    SPT_size = INIT_SPT_SIZE as u32;

    stable_ptr_table = stgMallocBytes(
        (SPT_size as usize).wrapping_mul(size_of::<spEntry>() as usize),
        c"initStablePtrTable".as_ptr(),
    ) as *mut spEntry;

    initSpEntryFreeList(stable_ptr_table, INIT_SPT_SIZE as u32);
}

unsafe fn enlargeStablePtrTable() {
    let mut old_SPT_size: u32 = SPT_size as u32;
    let mut new_stable_ptr_table = null_mut::<spEntry>();
    SPT_size = SPT_size.wrapping_mul(2 as u32);

    new_stable_ptr_table = stgMallocBytes(
        (SPT_size as usize).wrapping_mul(size_of::<spEntry>() as usize),
        c"enlargeStablePtrTable".as_ptr(),
    ) as *mut spEntry;

    memcpy(
        new_stable_ptr_table as *mut c_void,
        stable_ptr_table as *const c_void,
        (old_SPT_size as usize).wrapping_mul(size_of::<spEntry>() as usize),
    );

    let fresh6 = n_old_SPTs;
    n_old_SPTs = n_old_SPTs.wrapping_add(1);
    old_SPTs[fresh6 as usize] = stable_ptr_table;
    stable_ptr_table = new_stable_ptr_table;
    initSpEntryFreeList(stable_ptr_table.offset(old_SPT_size as isize), old_SPT_size);
}

unsafe fn freeOldSPTs() {
    let mut i: u32 = 0;
    i = 0;

    while i < n_old_SPTs {
        stgFree(old_SPTs[i as usize] as *mut c_void);
        i = i.wrapping_add(1);
    }

    n_old_SPTs = 0;
}

unsafe fn exitStablePtrTable() {
    if !stable_ptr_table.is_null() {
        stgFree(stable_ptr_table as *mut c_void);
    }

    stable_ptr_table = null_mut::<spEntry>();
    SPT_size = 0;
    freeOldSPTs();
}

#[inline]
unsafe fn freeSpEntry(mut sp: *mut spEntry) {
    (*sp).addr = stable_ptr_free as P_ as StgPtr;
    stable_ptr_free = sp;
}

unsafe fn freeStablePtrUnsafe(mut sp: StgStablePtr) {
    if sp.is_null() {
        return;
    }

    let mut spw: StgWord = (sp as StgWord).wrapping_sub(1 as StgWord);
    freeSpEntry(stable_ptr_table.offset(spw as isize) as *mut spEntry);
}

unsafe fn freeStablePtr(mut sp: StgStablePtr) {
    stablePtrLock();
    freeStablePtrUnsafe(sp);
    stablePtrUnlock();
}

unsafe fn getStablePtr(mut p: StgPtr) -> StgStablePtr {
    stablePtrLock();

    if stable_ptr_free.is_null() {
        enlargeStablePtrTable();
    }

    let mut sp: StgWord = stable_ptr_free.offset_from(stable_ptr_table) as i64 as StgWord;
    stable_ptr_free = (*stable_ptr_free).addr as *mut spEntry;

    let ref mut fresh5 = (*stable_ptr_table.offset(sp as isize)).addr;
    *fresh5 = p;
    stablePtrUnlock();
    sp = sp.wrapping_add(1 as StgWord);

    return sp as StgStablePtr;
}

unsafe fn markStablePtrTable(mut evac: evac_fn, mut user: *mut c_void) {
    freeOldSPTs();

    let mut p = null_mut::<spEntry>();
    let mut __end_ptr: *mut spEntry = stable_ptr_table.offset(SPT_size as isize) as *mut spEntry;
    p = stable_ptr_table;

    while p < __end_ptr {
        if !(*p).addr.is_null()
            && ((*p).addr < stable_ptr_table as P_ || (*p).addr >= __end_ptr as P_)
        {
            evac.expect("non-null function pointer")(
                user,
                &raw mut (*p).addr as *mut *mut StgClosure,
            );
        }

        p = p.offset(1);
    }
}

unsafe fn threadStablePtrTable(mut evac: evac_fn, mut user: *mut c_void) {
    let mut p = null_mut::<spEntry>();
    let mut __end_ptr: *mut spEntry = stable_ptr_table.offset(SPT_size as isize) as *mut spEntry;
    p = stable_ptr_table;

    while p < __end_ptr {
        if !(*p).addr.is_null()
            && ((*p).addr < stable_ptr_table as P_ || (*p).addr >= __end_ptr as P_)
        {
            evac.expect("non-null function pointer")(
                user,
                &raw mut (*p).addr as *mut *mut StgClosure,
            );
        }

        p = p.offset(1);
    }
}
