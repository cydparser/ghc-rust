use crate::ffi::rts::flags::{NO_HEAP_PROFILING, RtsFlags};
use crate::ffi::stg::W_;
use crate::ffi::stg::smp::xchg;
use crate::ffi::stg::types::StgWord;
use crate::hash::{HashTable, iterHashTable};
use crate::linker_internals::{
    _ObjectCode, DYNAMIC_OBJECT, NativeCodeRange, ObjectCode, SECTIONKIND_OTHER, freeObjectCode,
};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};

/// cbindgen:no-export
struct OCSectionIndices {
    capacity: c_int,
    n_sections: c_int,
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

static mut object_code_mark_bit: uint8_t = 0 as uint8_t;

static mut objects: *mut ObjectCode = null::<ObjectCode>() as *mut ObjectCode;

static mut old_objects: *mut ObjectCode = null::<ObjectCode>() as *mut ObjectCode;

static mut n_unloaded_objects: c_int = 0 as c_int;

static mut loaded_objects: *mut ObjectCode = null::<ObjectCode>() as *mut ObjectCode;

static mut global_s_indices: *mut OCSectionIndices =
    null::<OCSectionIndices>() as *mut OCSectionIndices;

unsafe fn tryToUnload() -> bool {
    if RtsFlags.ProfFlags.doHeapProfile != NO_HEAP_PROFILING as uint32_t {
        return r#false != 0;
    }

    return !global_s_indices.is_null();
}

unsafe fn createOCSectionIndices() -> *mut OCSectionIndices {
    let mut s_indices = stgMallocBytes(
        size_of::<OCSectionIndices>() as size_t,
        b"OCSectionIndices\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut OCSectionIndices;

    let mut capacity = 1024 as c_int;
    (*s_indices).capacity = capacity;
    (*s_indices).n_sections = 0 as c_int;
    (*s_indices).sorted = r#true != 0;
    (*s_indices).unloaded = r#false != 0;

    (*s_indices).indices = stgMallocBytes(
        (capacity as size_t).wrapping_mul(size_of::<OCSectionIndex>() as size_t),
        b"OCSectionIndices::indices\0" as *const u8 as *const c_char as *mut c_char,
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

unsafe fn cmpSectionIndex(mut indexa: *const c_void, mut indexb: *const c_void) -> c_int {
    let mut s1: W_ = (*(indexa as *mut OCSectionIndex)).start;
    let mut s2: W_ = (*(indexb as *mut OCSectionIndex)).start;

    if s1 < s2 {
        return -(1 as c_int);
    } else if s1 > s2 {
        return 1 as c_int;
    }

    return 0 as c_int;
}

unsafe fn reserveOCSectionIndices(mut s_indices: *mut OCSectionIndices, mut len: c_int) {
    let mut current_capacity = (*s_indices).capacity;
    let mut current_len = (*s_indices).n_sections;

    if current_capacity - current_len >= len {
        return;
    }

    let mut new_capacity = (1 as c_int) << ceil(log2((current_len + len) as c_double)) as c_int;
    let mut old_indices = (*s_indices).indices;

    let mut new_indices = stgMallocBytes(
        (new_capacity as size_t).wrapping_mul(size_of::<OCSectionIndex>() as size_t),
        b"reserveOCSectionIndices\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut OCSectionIndex;

    let mut i = 0 as c_int;

    while i < current_len {
        *new_indices.offset(i as isize) = *old_indices.offset(i as isize);
        i += 1;
    }

    (*s_indices).capacity = new_capacity;
    (*s_indices).indices = new_indices;
    stgFree(old_indices as *mut c_void);
}

unsafe fn insertOCSectionIndices(mut oc: *mut ObjectCode) {
    (*global_s_indices).sorted = r#false != 0;

    if (*oc).r#type as c_uint == DYNAMIC_OBJECT as c_int as c_uint {
        let mut n_ranges = 0 as c_int;
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
        let mut i = 0 as c_int;

        while i < (*oc).n_sections {
            if (*(*oc).sections.offset(i as isize)).kind as c_uint
                != SECTIONKIND_OTHER as c_int as c_uint
            {
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
    (*s_indices).unloaded = r#true != 0;

    let mut i = 0 as c_int;

    while i < (*oc).n_sections {
        if (*(*oc).sections.offset(i as isize)).kind as c_uint
            != SECTIONKIND_OTHER as c_int as c_uint
        {
            let mut section_idx =
                findSectionIdx(s_indices, (*(*oc).sections.offset(i as isize)).start);

            if section_idx != -(1 as c_int) {
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
        (*s_indices).n_sections as size_t,
        size_of::<OCSectionIndex>() as size_t,
        Some(cmpSectionIndex as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );

    (*s_indices).sorted = r#true != 0;
}

unsafe fn removeRemovedOCSections(mut s_indices: *mut OCSectionIndices) {
    if !(*s_indices).unloaded {
        return;
    }

    let mut next_free_idx = 0 as c_int;
    let mut i = 0 as c_int;

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
    (*s_indices).unloaded = r#true != 0;
}

unsafe fn findSectionIdx(mut s_indices: *mut OCSectionIndices, mut addr: *const c_void) -> c_int {
    let mut w_addr: W_ = addr as W_;

    if (*s_indices).n_sections <= 0 as c_int {
        return -(1 as c_int);
    }

    if w_addr < (*(*s_indices).indices.offset(0 as c_int as isize)).start {
        return -(1 as c_int);
    }

    let mut left = 0 as c_int;
    let mut right = (*s_indices).n_sections;

    while (left + 1 as c_int) < right {
        let mut mid = (left + right) / 2 as c_int;
        let mut w_mid: W_ = (*(*s_indices).indices.offset(mid as isize)).start;

        if w_mid <= w_addr {
            left = mid;
        } else {
            right = mid;
        }
    }

    if w_addr < (*(*s_indices).indices.offset(left as isize)).end {
        return left;
    }

    return -(1 as c_int);
}

unsafe fn findOC(mut s_indices: *mut OCSectionIndices, mut addr: *const c_void) -> *mut ObjectCode {
    let mut oc_idx = findSectionIdx(s_indices, addr);

    if oc_idx == -(1 as c_int) {
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
        return r#true != 0;
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

    iterHashTable(
        (*oc).dependencies as *mut HashTable,
        NULL,
        Some(markObjectLive as unsafe extern "C" fn(*mut c_void, StgWord, *const c_void) -> bool),
    );

    return r#true != 0;
}

unsafe fn markObjectCode(mut addr: *const c_void) {
    if !tryToUnload() {
        return;
    }

    let mut oc = findOC(global_s_indices, addr);

    if !oc.is_null() {
        markObjectLive(NULL, oc as StgWord, null::<c_void>());
    }
}

unsafe fn prepareUnloadCheck() -> bool {
    if !tryToUnload() {
        return r#false != 0;
    }

    removeRemovedOCSections(global_s_indices);
    sortOCSectionIndices(global_s_indices);
    object_code_mark_bit = !(object_code_mark_bit as c_int) as uint8_t;
    old_objects = objects;
    objects = null_mut::<ObjectCode>();

    return r#true != 0;
}

unsafe fn checkUnload() {
    if tryToUnload() {
        let mut s_indices = global_s_indices;
        let mut oc = loaded_objects;

        while !oc.is_null() {
            markObjectLive(NULL, oc as StgWord, null::<c_void>());
            oc = (*oc).next_loaded_object as *mut ObjectCode;
        }

        let mut next = null_mut::<ObjectCode>();
        let mut oc_0 = old_objects;

        while !oc_0.is_null() {
            next = (*oc_0).next as *mut ObjectCode;

            if (*oc_0).unloadable {
                removeOCSectionIndices(s_indices, oc_0);
                freeObjectCode(oc_0);
                n_unloaded_objects -= 1 as c_int;
            } else {
                (*oc_0).next = objects as *mut _ObjectCode;
                objects = oc_0;
            }

            oc_0 = next;
        }
    }

    old_objects = null_mut::<ObjectCode>();
}
