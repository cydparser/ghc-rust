use crate::ffi::rts::_assertFail;
use crate::ffi::rts::linker::OBJECT_UNLOADED;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::stg::W_;
use crate::ffi::stg::smp::xchg;
use crate::ffi::stg::types::StgWord;
use crate::hash::{HashTable, iterHashTable};
use crate::linker_internals::{
    _ObjectCode, DYNAMIC_OBJECT, NativeCodeRange, ObjectCode, SECTIONKIND_OTHER, freeObjectCode,
    linker_mutex,
};
use crate::prelude::*;
use crate::rts_flags::{NO_HEAP_PROFILING, RtsFlags};
use crate::rts_utils::{stgFree, stgMallocBytes};

/// cbindgen:no-export
struct OCSectionIndices {
    capacity: i32,
    n_sections: i32,
    sorted: bool,
    unloaded: bool,
    indices: *mut OCSectionIndex,
}

/// cbindgen:no-export
struct OCSectionIndex {
    start: W_,
    end: W_,
    oc: *mut ObjectCode,
}

static mut object_code_mark_bit: u8 = 0;

static mut objects: *mut ObjectCode = null_mut::<ObjectCode>();

static mut old_objects: *mut ObjectCode = null_mut::<ObjectCode>();

static mut n_unloaded_objects: i32 = 0;

static mut loaded_objects: *mut ObjectCode = null_mut::<ObjectCode>();

static mut global_s_indices: *mut OCSectionIndices = null_mut::<OCSectionIndices>();

unsafe fn tryToUnload() -> bool {
    if RtsFlags.ProfFlags.doHeapProfile != NO_HEAP_PROFILING as u32 {
        return false;
    }

    return !global_s_indices.is_null();
}

unsafe fn createOCSectionIndices() -> *mut OCSectionIndices {
    let mut s_indices = stgMallocBytes(
        size_of::<OCSectionIndices>() as usize,
        c"OCSectionIndices".as_ptr(),
    ) as *mut OCSectionIndices;

    let mut capacity = 1024;
    (*s_indices).capacity = capacity;
    (*s_indices).n_sections = 0;
    (*s_indices).sorted = true;
    (*s_indices).unloaded = false;

    (*s_indices).indices = stgMallocBytes(
        (capacity as usize).wrapping_mul(size_of::<OCSectionIndex>() as usize),
        c"OCSectionIndices::indices".as_ptr(),
    ) as *mut OCSectionIndex;

    return s_indices;
}

unsafe fn freeOCSectionIndices(mut s_indices: *mut OCSectionIndices) {
    stgFree((*s_indices).indices as *mut c_void);
    stgFree(s_indices as *mut c_void);
}

unsafe fn initUnloadCheck() {
    global_s_indices = createOCSectionIndices();
}

unsafe fn exitUnloadCheck() {
    freeOCSectionIndices(global_s_indices);
    global_s_indices = null_mut::<OCSectionIndices>();
}

unsafe fn cmpSectionIndex(mut indexa: *const c_void, mut indexb: *const c_void) -> i32 {
    let mut s1: W_ = (*(indexa as *mut OCSectionIndex)).start;
    let mut s2: W_ = (*(indexb as *mut OCSectionIndex)).start;

    if s1 < s2 {
        return -1;
    } else if s1 > s2 {
        return 1;
    }

    return 0;
}

unsafe fn reserveOCSectionIndices(mut s_indices: *mut OCSectionIndices, mut len: i32) {
    let mut current_capacity = (*s_indices).capacity;
    let mut current_len = (*s_indices).n_sections;

    if current_capacity - current_len >= len {
        return;
    }

    let mut new_capacity = 1 << ceil(log2((current_len + len) as f64)) as i32;
    let mut old_indices = (*s_indices).indices;

    let mut new_indices = stgMallocBytes(
        (new_capacity as usize).wrapping_mul(size_of::<OCSectionIndex>() as usize),
        c"reserveOCSectionIndices".as_ptr(),
    ) as *mut OCSectionIndex;

    let mut i = 0;

    while i < current_len {
        *new_indices.offset(i as isize) = *old_indices.offset(i as isize);
        i += 1;
    }

    (*s_indices).capacity = new_capacity;
    (*s_indices).indices = new_indices;
    stgFree(old_indices as *mut c_void);
}

unsafe fn insertOCSectionIndices(mut oc: *mut ObjectCode) {
    (*global_s_indices).sorted = false;

    if (*oc).r#type as u32 == DYNAMIC_OBJECT as i32 as u32 {
        let mut n_ranges = 0;
        let mut ncr = (*oc).nc_ranges;

        while !ncr.is_null() {
            n_ranges += 1;
            ncr = (*ncr).next as *mut NativeCodeRange;
        }

        reserveOCSectionIndices(global_s_indices, n_ranges);

        let mut s_i = (*global_s_indices).n_sections;
        let mut ncr_0 = (*oc).nc_ranges;

        while !ncr_0.is_null() {
            let mut ent: *mut OCSectionIndex =
                (*global_s_indices).indices.offset(s_i as isize) as *mut OCSectionIndex;
            (*ent).start = (*ncr_0).start as W_;
            (*ent).end = (*ncr_0).end as W_;
            (*ent).oc = oc;
            s_i += 1;
            ncr_0 = (*ncr_0).next as *mut NativeCodeRange;
        }

        (*global_s_indices).n_sections = s_i;
    } else {
        reserveOCSectionIndices(global_s_indices, (*oc).n_sections);

        let mut s_i_0 = (*global_s_indices).n_sections;
        let mut i = 0;

        while i < (*oc).n_sections {
            if (*(*oc).sections.offset(i as isize)).kind as u32 != SECTIONKIND_OTHER as i32 as u32 {
                let mut ent_0: *mut OCSectionIndex =
                    (*global_s_indices).indices.offset(s_i_0 as isize) as *mut OCSectionIndex;
                (*ent_0).start = (*(*oc).sections.offset(i as isize)).start as W_;
                (*ent_0).end = ((*(*oc).sections.offset(i as isize)).start as W_)
                    .wrapping_add((*(*oc).sections.offset(i as isize)).size as W_);
                (*ent_0).oc = oc;
                s_i_0 += 1;
            }

            i += 1;
        }

        (*global_s_indices).n_sections = s_i_0;
    }

    if !objects.is_null() {
        (*objects).prev = oc as *mut _ObjectCode;
    }

    (*oc).next = objects as *mut _ObjectCode;
    objects = oc;
}

unsafe fn removeOCSectionIndices(mut s_indices: *mut OCSectionIndices, mut oc: *mut ObjectCode) {
    (*s_indices).unloaded = true;

    let mut i = 0;

    while i < (*oc).n_sections {
        if (*(*oc).sections.offset(i as isize)).kind as u32 != SECTIONKIND_OTHER as i32 as u32 {
            let mut section_idx =
                findSectionIdx(s_indices, (*(*oc).sections.offset(i as isize)).start);

            if section_idx != -1 {
                let ref mut fresh5 = (*(*s_indices).indices.offset(section_idx as isize)).oc;
                *fresh5 = null_mut::<ObjectCode>();
            }
        }

        i += 1;
    }
}

unsafe fn sortOCSectionIndices(mut s_indices: *mut OCSectionIndices) {
    if (*s_indices).sorted {
        return;
    }

    qsort(
        (*s_indices).indices as *mut c_void,
        (*s_indices).n_sections as usize,
        size_of::<OCSectionIndex>() as usize,
        Some(cmpSectionIndex as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );

    (*s_indices).sorted = true;
}

unsafe fn removeRemovedOCSections(mut s_indices: *mut OCSectionIndices) {
    if !(*s_indices).unloaded {
        return;
    }

    let mut next_free_idx = 0;
    let mut i = 0;

    while i < (*s_indices).n_sections {
        if !(*(*s_indices).indices.offset(i as isize)).oc.is_null() {
            if i == next_free_idx {
                next_free_idx += 1;
            } else {
                *(*s_indices).indices.offset(next_free_idx as isize) =
                    *(*s_indices).indices.offset(i as isize);
                next_free_idx += 1;
            }
        }

        i += 1;
    }

    (*s_indices).n_sections = next_free_idx;
    (*s_indices).unloaded = true;
}

unsafe fn findSectionIdx(mut s_indices: *mut OCSectionIndices, mut addr: *const c_void) -> i32 {
    if (*s_indices).sorted as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/CheckUnload.c".as_ptr(), 358);
    }

    let mut w_addr: W_ = addr as W_;

    if (*s_indices).n_sections <= 0 {
        return -1;
    }

    if w_addr < (*(*s_indices).indices.offset(0)).start {
        return -1;
    }

    let mut left = 0;
    let mut right = (*s_indices).n_sections;

    while (left + 1) < right {
        let mut mid = (left + right) / 2;
        let mut w_mid: W_ = (*(*s_indices).indices.offset(mid as isize)).start;

        if w_mid <= w_addr {
            left = mid;
        } else {
            right = mid;
        }
    }

    if (w_addr >= (*(*s_indices).indices.offset(left as isize)).start) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/CheckUnload.c".as_ptr(), 378);
    }

    if w_addr < (*(*s_indices).indices.offset(left as isize)).end {
        return left;
    }

    return -1;
}

unsafe fn findOC(mut s_indices: *mut OCSectionIndices, mut addr: *const c_void) -> *mut ObjectCode {
    let mut oc_idx = findSectionIdx(s_indices, addr);

    if oc_idx == -1 {
        return null_mut::<ObjectCode>();
    }

    return (*(*s_indices).indices.offset(oc_idx as isize)).oc;
}

unsafe fn markObjectLive(
    mut data: *mut c_void,
    mut key: StgWord,
    mut value: *const c_void,
) -> bool {
    let mut oc = key as *mut ObjectCode;

    if xchg(&raw mut (*oc).mark, object_code_mark_bit as StgWord) == object_code_mark_bit as StgWord
    {
        return true;
    }

    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/CheckUnload.c".as_ptr(),
            406,
            __r,
        );
    }

    if !(*oc).prev.is_null() {
        (*(*oc).prev).next = (*oc).next;
    } else {
        old_objects = (*oc).next as *mut ObjectCode;
    }

    if !(*oc).next.is_null() {
        (*(*oc).next).prev = (*oc).prev;
    }

    (*oc).prev = null_mut::<_ObjectCode>();
    (*oc).next = objects as *mut _ObjectCode;

    if !objects.is_null() {
        (*objects).prev = oc as *mut _ObjectCode;
    }

    objects = oc;

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/CheckUnload.c".as_ptr(),
            426,
        );
    }

    iterHashTable(
        (*oc).dependencies as *mut HashTable,
        NULL,
        Some(markObjectLive as unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> bool),
    );

    return true;
}

unsafe fn markObjectCode(mut addr: *const c_void) {
    if !tryToUnload() {
        return;
    }

    if !(addr as W_ >= mblock_address_space.0.begin && (addr as W_) < mblock_address_space.0.end)
        as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/CheckUnload.c".as_ptr(), 441);
    }

    let mut oc = findOC(global_s_indices, addr);

    if !oc.is_null() {
        markObjectLive(NULL, oc as StgWord, null::<c_void>());
    }
}

unsafe fn prepareUnloadCheck() -> bool {
    if !tryToUnload() {
        return false;
    }

    removeRemovedOCSections(global_s_indices);
    sortOCSectionIndices(global_s_indices);

    if old_objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/CheckUnload.c".as_ptr(), 461);
    }

    object_code_mark_bit = !(object_code_mark_bit as i32) as u8;
    old_objects = objects;
    objects = null_mut::<ObjectCode>();

    return true;
}

unsafe fn checkUnload() {
    if tryToUnload() {
        let mut s_indices = global_s_indices;

        if (*s_indices).sorted as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/CheckUnload.c".as_ptr(), 478);
        }

        let mut oc = loaded_objects;

        while !oc.is_null() {
            markObjectLive(NULL, oc as StgWord, null::<c_void>());
            oc = (*oc).next_loaded_object as *mut ObjectCode;
        }

        let mut next = null_mut::<ObjectCode>();
        let mut oc_0 = old_objects;

        while !oc_0.is_null() {
            next = (*oc_0).next as *mut ObjectCode;

            if ((*oc_0).status as u32 == OBJECT_UNLOADED as i32 as u32) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/CheckUnload.c".as_ptr(), 489);
            }

            if (*oc_0).symbols.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/CheckUnload.c".as_ptr(), 496);
            }

            if (*oc_0).unloadable {
                removeOCSectionIndices(s_indices, oc_0);
                freeObjectCode(oc_0);
                n_unloaded_objects -= 1;
            } else {
                (*oc_0).next = objects as *mut _ObjectCode;
                objects = oc_0;
            }

            oc_0 = next;
        }
    }

    old_objects = null_mut::<ObjectCode>();
}
