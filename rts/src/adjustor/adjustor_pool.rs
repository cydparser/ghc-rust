use crate::ffi::rts::_assertFail;
use crate::ffi::rts::exec_page::{allocateExecPage, freezeExecPage};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::os_threads::{Mutex, initMutex};
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::sm::os_mem::getPageSize;

pub(crate) type mk_adjustor_code_fn =
    Option<unsafe extern "C" fn(*mut u8, *const c_void, *mut c_void) -> ()>;

/// cbindgen:no-export
pub(crate) struct AdjustorContext {
    pub(crate) hptr: StgStablePtr,
    pub(crate) wptr: StgFunPtr,
}

/// cbindgen:no-export
pub(crate) struct AdjustorTemplate {
    pub(crate) code_start: *mut u8,
    pub(crate) code_end: *mut u8,
    pub(crate) context_ptr: *mut *const AdjustorContext,
}

/// cbindgen:no-export
struct AdjustorPool {
    make_code: mk_adjustor_code_fn,
    user_data: *mut c_void,
    adjustor_code_size: usize,
    context_size: usize,
    chunk_slots: usize,
    free_list: *mut AdjustorChunk,
    lock: Mutex,
}

/// cbindgen:no-export
struct AdjustorChunk {
    first_free: usize,
    owner: *mut AdjustorPool,
    free_list_next: *mut AdjustorChunk,
    exec_page: *mut AdjustorExecPage,
    contexts: *mut c_void,
    slot_bitmap: [u8; 0],
}

/// cbindgen:no-export
struct AdjustorExecPage {
    magic: u64,
    owner: *mut AdjustorChunk,
    adjustor_code: [u8; 0],
}

const ADJUSTOR_EXEC_PAGE_MAGIC: u64 = 0xddeeffaabbcc0011;

unsafe fn new_adjustor_pool(
    mut context_size: usize,
    mut code_size: usize,
    mut make_code: mk_adjustor_code_fn,
    mut user_data: *mut c_void,
) -> *mut AdjustorPool {
    let mut pool = stgMallocBytes(
        size_of::<AdjustorPool>() as usize,
        c"newAdjustorPool".as_ptr(),
    ) as *mut AdjustorPool;

    let code_alignment: usize = 16;
    (*pool).make_code = make_code;
    (*pool).user_data = user_data;
    (*pool).context_size = context_size;
    (*pool).adjustor_code_size = code_size;

    let mut usable_exec_page_sz: usize = getPageSize().wrapping_sub(
        (size_of::<AdjustorExecPage>() as usize)
            .wrapping_add(code_alignment)
            .wrapping_sub(1 as usize)
            .wrapping_div(code_alignment)
            .wrapping_mul(code_alignment),
    );

    (*pool).chunk_slots = usable_exec_page_sz.wrapping_div(
        code_size
            .wrapping_add(code_alignment)
            .wrapping_sub(1 as usize)
            .wrapping_div(code_alignment)
            .wrapping_mul(code_alignment),
    );

    (*pool).free_list = null_mut::<AdjustorChunk>();
    initMutex(&raw mut (*pool).lock);

    return pool;
}

unsafe fn bitmap_set(mut bitmap: *mut u8, mut idx: usize, mut value: bool) {
    let mut word_n: usize = idx.wrapping_div(8 as usize);
    let mut bit: u8 = (1 << idx.wrapping_rem(8 as usize)) as u8;

    if value {
        let ref mut fresh12 = *bitmap.offset(word_n as isize);
        *fresh12 = (*fresh12 as i32 | bit as i32) as u8;
    } else {
        let ref mut fresh13 = *bitmap.offset(word_n as isize);
        *fresh13 = (*fresh13 as i32 & !(bit as i32)) as u8;
    };
}

unsafe fn bitmap_get(mut bitmap: *mut u8, mut idx: usize) -> bool {
    let mut word_n: usize = idx.wrapping_div(8 as usize);
    let mut bit: u8 = (1 << idx.wrapping_rem(8 as usize)) as u8;

    return *bitmap.offset(word_n as isize) as i32 & bit as i32 != 0;
}

unsafe fn bitmap_first_unset(
    mut bitmap: *mut u8,
    mut length_in_bits: usize,
    mut start_idx: usize,
) -> usize {
    let mut i: usize = start_idx;

    while i < length_in_bits {
        if bitmap_get(bitmap, i) as i32 == 0 {
            return i;
        }

        i = i.wrapping_add(1);
    }

    return length_in_bits;
}

unsafe fn get_context(mut chunk: *mut AdjustorChunk, mut slot_idx: usize) -> *mut c_void {
    let mut contexts = (*chunk).contexts as *mut u8;

    return contexts.offset((*(*chunk).owner).context_size.wrapping_mul(slot_idx) as isize)
        as *mut c_void;
}

unsafe fn alloc_adjustor(mut pool: *mut AdjustorPool, mut context: *mut c_void) -> *mut c_void {
    let mut slot_idx: usize = 0;
    let mut chunk = null_mut::<AdjustorChunk>();
    let mut __r = pthread_mutex_lock(&raw mut (*pool).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/adjustor/AdjustorPool.c".as_ptr(),
            217,
            __r,
        );
    }

    if (*pool).free_list.is_null() {
        (*pool).free_list = alloc_adjustor_chunk(pool);
    }

    chunk = (*pool).free_list;
    slot_idx = (*chunk).first_free;

    if (slot_idx < (*pool).chunk_slots) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/adjustor/AdjustorPool.c".as_ptr(), 225);
    }

    if (bitmap_get(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx) as i32 == 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/adjustor/AdjustorPool.c".as_ptr(), 226);
    }

    bitmap_set(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx, 1 != 0);

    (*chunk).first_free = bitmap_first_unset(
        &raw mut (*chunk).slot_bitmap as *mut u8,
        (*pool).chunk_slots,
        slot_idx.wrapping_add(1 as usize),
    );

    if (*chunk).first_free == (*pool).chunk_slots {
        (*pool).free_list = (*chunk).free_list_next;
        (*chunk).free_list_next = null_mut::<AdjustorChunk>();
    }

    if bitmap_get(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/adjustor/AdjustorPool.c".as_ptr(), 239);
    }

    bitmap_set(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx, true);
    memcpy(get_context(chunk, slot_idx), context, (*pool).context_size);

    let mut adjustor = (&raw mut (*(*chunk).exec_page).adjustor_code as *mut u8)
        .offset((*pool).adjustor_code_size.wrapping_mul(slot_idx) as isize)
        as *mut u8 as *mut c_void;

    if pthread_mutex_unlock(&raw mut (*pool).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/adjustor/AdjustorPool.c".as_ptr(),
            245,
        );
    }

    return adjustor;
}

unsafe fn free_adjustor(mut adjustor: *mut c_void, mut context: *mut c_void) {
    let mut exec_page_mask: usize = !(getPageSize() as u64).wrapping_sub(1 as u64) as usize;

    let mut exec_page = (adjustor as usize & exec_page_mask) as *mut AdjustorExecPage;

    if (*exec_page).magic != ADJUSTOR_EXEC_PAGE_MAGIC as u64 {
        barf(c"free_adjustor was passed an invalid adjustor".as_ptr());
    }

    let mut chunk = (*exec_page).owner;
    let mut pool = (*chunk).owner;
    let mut slot_off: usize = (adjustor as *mut u8)
        .offset_from(&raw mut (*exec_page).adjustor_code as *mut u8)
        as i64 as usize;

    let mut slot_idx: usize = slot_off.wrapping_div((*pool).adjustor_code_size);

    if (slot_off.wrapping_rem((*pool).adjustor_code_size) == 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/adjustor/AdjustorPool.c".as_ptr(), 266);
    }

    let mut __r = pthread_mutex_lock(&raw mut (*pool).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/adjustor/AdjustorPool.c".as_ptr(),
            268,
            __r,
        );
    }

    if bitmap_get(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/adjustor/AdjustorPool.c".as_ptr(), 271);
    }

    bitmap_set(&raw mut (*chunk).slot_bitmap as *mut u8, slot_idx, false);

    if (*chunk).first_free == (*pool).chunk_slots {
        (*chunk).free_list_next = (*pool).free_list;
        (*pool).free_list = chunk;
    }

    if (*chunk).first_free > slot_idx {
        (*chunk).first_free = slot_idx;
    }

    memcpy(context, get_context(chunk, slot_idx), (*pool).context_size);
    memset(get_context(chunk, slot_idx), 0, (*pool).context_size);

    if pthread_mutex_unlock(&raw mut (*pool).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/adjustor/AdjustorPool.c".as_ptr(),
            288,
        );
    }
}

unsafe fn alloc_adjustor_chunk(mut owner: *mut AdjustorPool) -> *mut AdjustorChunk {
    let mut exec_page = allocateExecPage();

    if exec_page.is_null() {
        barf(c"alloc_adjustor_chunk: failed to allocate".as_ptr());
    }

    let mut adj_page = exec_page as *mut AdjustorExecPage;
    (*adj_page).magic = ADJUSTOR_EXEC_PAGE_MAGIC as u64;

    let mut bitmap_sz: usize = (*owner)
        .chunk_slots
        .wrapping_add((8 as usize).wrapping_mul(size_of::<*mut c_void>() as usize))
        .wrapping_sub(1 as usize)
        .wrapping_div((8 as usize).wrapping_mul(size_of::<*mut c_void>() as usize))
        .wrapping_mul((8 as usize).wrapping_mul(size_of::<*mut c_void>() as usize))
        .wrapping_div(8 as usize);

    let mut contexts_sz: usize = (*owner).context_size.wrapping_mul((*owner).chunk_slots);

    let mut alloc_sz: usize = (size_of::<AdjustorChunk>() as usize)
        .wrapping_add(bitmap_sz)
        .wrapping_add(contexts_sz);

    let mut chunk = stgMallocBytes(alloc_sz, c"allocAdjustorChunk".as_ptr()) as *mut AdjustorChunk;
    (*chunk).owner = owner;
    (*chunk).first_free = 0;
    (*chunk).contexts = (&raw mut (*chunk).slot_bitmap as *mut u8).offset(bitmap_sz as isize)
        as *mut AdjustorContext as *mut c_void;
    (*chunk).free_list_next = null_mut::<AdjustorChunk>();
    (*chunk).exec_page = adj_page;
    (*(*chunk).exec_page).owner = chunk;
    memset(
        &raw mut (*chunk).slot_bitmap as *mut u8 as *mut c_void,
        0,
        bitmap_sz,
    );
    memset((*chunk).contexts, 0, contexts_sz);

    let mut code_sz: usize = (*owner).adjustor_code_size;
    let mut i: usize = 0;

    while i < (*owner).chunk_slots {
        (*owner).make_code.expect("non-null function pointer")(
            (&raw mut (*adj_page).adjustor_code as *mut u8).offset(i.wrapping_mul(code_sz) as isize)
                as *mut u8,
            get_context(chunk, i),
            (*owner).user_data,
        );

        i = i.wrapping_add(1);
    }

    freezeExecPage(exec_page);

    return chunk;
}

unsafe fn mk_adjustor_from_template(
    mut exec_code: *mut u8,
    mut context: *const c_void,
    mut user_data: *mut c_void,
) {
    let mut adjustor_context = context as *const AdjustorContext;
    let mut tmpl = user_data as *mut AdjustorTemplate;

    memcpy(
        exec_code as *mut c_void,
        (*tmpl).code_start as *const c_void,
        (*tmpl).code_end.offset_from((*tmpl).code_start) as i64 as usize,
    );

    let mut context_off: usize =
        ((*tmpl).context_ptr as *mut u8).offset_from((*tmpl).code_start) as i64 as usize;

    let mut slot_context_ptr =
        exec_code.offset(context_off as isize) as *mut *const AdjustorContext;
    *slot_context_ptr = adjustor_context;
}

unsafe fn new_adjustor_pool_from_template(mut tmpl: *const AdjustorTemplate) -> *mut AdjustorPool {
    let mut code_size: usize = (*tmpl).code_end.offset_from((*tmpl).code_start) as i64 as usize;

    return new_adjustor_pool(
        size_of::<AdjustorContext>() as usize,
        code_size,
        Some(
            mk_adjustor_from_template
                as unsafe extern "C" fn(*mut u8, *const c_void, *mut c_void) -> (),
        ),
        tmpl as *mut c_void,
    );
}
