use crate::ffi::rts::exec_page::{allocateExecPage, freezeExecPage};
use crate::ffi::rts::messages::barf;
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::ffi::stg::types::{StgFunPtr, StgStablePtr};
use crate::prelude::*;
use crate::rts_utils::stgMallocBytes;
use crate::sm::os_mem::getPageSize;

pub(crate) type mk_adjustor_code_fn =
    Option<unsafe extern "C" fn(*mut uint8_t, *const c_void, *mut c_void) -> ()>;

/// cbindgen:no-export
pub(crate) struct AdjustorContext {
    pub(crate) hptr: StgStablePtr,
    pub(crate) wptr: StgFunPtr,
}

/// cbindgen:no-export
pub(crate) struct AdjustorTemplate {
    pub(crate) code_start: *mut uint8_t,
    pub(crate) code_end: *mut uint8_t,
    pub(crate) context_ptr: *mut *const AdjustorContext,
}

/// cbindgen:no-export
struct AdjustorPool {
    make_code: mk_adjustor_code_fn,
    user_data: *mut c_void,
    adjustor_code_size: size_t,
    context_size: size_t,
    chunk_slots: size_t,
    free_list: *mut AdjustorChunk,
}

/// cbindgen:no-export
struct AdjustorChunk {
    first_free: size_t,
    owner: *mut AdjustorPool,
    free_list_next: *mut AdjustorChunk,
    exec_page: *mut AdjustorExecPage,
    contexts: *mut c_void,
    slot_bitmap: [uint8_t; 0],
}

/// cbindgen:no-export
struct AdjustorExecPage {
    magic: uint64_t,
    owner: *mut AdjustorChunk,
    adjustor_code: [uint8_t; 0],
}

const ADJUSTOR_EXEC_PAGE_MAGIC: c_ulonglong = 0xddeeffaabbcc0011 as c_ulonglong;

unsafe fn new_adjustor_pool(
    mut context_size: size_t,
    mut code_size: size_t,
    mut make_code: mk_adjustor_code_fn,
    mut user_data: *mut c_void,
) -> *mut AdjustorPool {
    let mut pool = stgMallocBytes(
        size_of::<AdjustorPool>() as size_t,
        b"newAdjustorPool\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut AdjustorPool;

    let code_alignment: size_t = 16 as size_t;
    (*pool).make_code = make_code;
    (*pool).user_data = user_data;
    (*pool).context_size = context_size;
    (*pool).adjustor_code_size = code_size;

    let mut usable_exec_page_sz: size_t = getPageSize().wrapping_sub(
        (size_of::<AdjustorExecPage>() as size_t)
            .wrapping_add(code_alignment)
            .wrapping_sub(1 as size_t)
            .wrapping_div(code_alignment)
            .wrapping_mul(code_alignment),
    );

    (*pool).chunk_slots = usable_exec_page_sz.wrapping_div(
        code_size
            .wrapping_add(code_alignment)
            .wrapping_sub(1 as size_t)
            .wrapping_div(code_alignment)
            .wrapping_mul(code_alignment),
    );

    (*pool).free_list = null_mut::<AdjustorChunk>();

    return pool;
}

unsafe fn bitmap_set(mut bitmap: *mut uint8_t, mut idx: size_t, mut value: bool) {
    let mut word_n: size_t = idx.wrapping_div(8 as size_t);
    let mut bit: uint8_t = ((1 as c_int) << idx.wrapping_rem(8 as size_t)) as uint8_t;

    if value {
        let ref mut fresh5 = *bitmap.offset(word_n as isize);
        *fresh5 = (*fresh5 as c_int | bit as c_int) as uint8_t;
    } else {
        let ref mut fresh6 = *bitmap.offset(word_n as isize);
        *fresh6 = (*fresh6 as c_int & !(bit as c_int)) as uint8_t;
    };
}

unsafe fn bitmap_get(mut bitmap: *mut uint8_t, mut idx: size_t) -> bool {
    let mut word_n: size_t = idx.wrapping_div(8 as size_t);
    let mut bit: uint8_t = ((1 as c_int) << idx.wrapping_rem(8 as size_t)) as uint8_t;

    return *bitmap.offset(word_n as isize) as c_int & bit as c_int != 0;
}

unsafe fn bitmap_first_unset(
    mut bitmap: *mut uint8_t,
    mut length_in_bits: size_t,
    mut start_idx: size_t,
) -> size_t {
    let mut i: size_t = start_idx;

    while i < length_in_bits {
        if bitmap_get(bitmap, i) as c_int == 0 as c_int {
            return i;
        }

        i = i.wrapping_add(1);
    }

    return length_in_bits;
}

unsafe fn get_context(mut chunk: *mut AdjustorChunk, mut slot_idx: size_t) -> *mut c_void {
    let mut contexts = (*chunk).contexts as *mut uint8_t;

    return contexts.offset((*(*chunk).owner).context_size.wrapping_mul(slot_idx) as isize)
        as *mut c_void;
}

unsafe fn alloc_adjustor(mut pool: *mut AdjustorPool, mut context: *mut c_void) -> *mut c_void {
    let mut slot_idx: size_t = 0;
    let mut chunk = null_mut::<AdjustorChunk>();

    if (*pool).free_list.is_null() {
        (*pool).free_list = alloc_adjustor_chunk(pool);
    }

    chunk = (*pool).free_list;
    slot_idx = (*chunk).first_free;

    bitmap_set(
        &raw mut (*chunk).slot_bitmap as *mut uint8_t,
        slot_idx,
        1 as c_int != 0,
    );

    (*chunk).first_free = bitmap_first_unset(
        &raw mut (*chunk).slot_bitmap as *mut uint8_t,
        (*pool).chunk_slots,
        slot_idx.wrapping_add(1 as size_t),
    );

    if (*chunk).first_free == (*pool).chunk_slots {
        (*pool).free_list = (*chunk).free_list_next;
        (*chunk).free_list_next = null_mut::<AdjustorChunk>();
    }

    bitmap_set(
        &raw mut (*chunk).slot_bitmap as *mut uint8_t,
        slot_idx,
        r#true != 0,
    );

    memcpy(get_context(chunk, slot_idx), context, (*pool).context_size);

    let mut adjustor = (&raw mut (*(*chunk).exec_page).adjustor_code as *mut uint8_t)
        .offset((*pool).adjustor_code_size.wrapping_mul(slot_idx) as isize)
        as *mut uint8_t as *mut c_void;

    return adjustor;
}

unsafe fn free_adjustor(mut adjustor: *mut c_void, mut context: *mut c_void) {
    let mut exec_page_mask: uintptr_t =
        !(getPageSize() as c_ulonglong).wrapping_sub(1 as c_ulonglong) as uintptr_t;

    let mut exec_page = (adjustor as uintptr_t & exec_page_mask) as *mut AdjustorExecPage;

    if (*exec_page).magic != ADJUSTOR_EXEC_PAGE_MAGIC as uint64_t {
        barf(b"free_adjustor was passed an invalid adjustor\0" as *const u8 as *const c_char);
    }

    let mut chunk = (*exec_page).owner;
    let mut pool = (*chunk).owner;
    let mut slot_off: size_t = (adjustor as *mut uint8_t)
        .offset_from(&raw mut (*exec_page).adjustor_code as *mut uint8_t)
        as c_long as size_t;

    let mut slot_idx: size_t = slot_off.wrapping_div((*pool).adjustor_code_size);

    bitmap_set(
        &raw mut (*chunk).slot_bitmap as *mut uint8_t,
        slot_idx,
        r#false != 0,
    );

    if (*chunk).first_free == (*pool).chunk_slots {
        (*chunk).free_list_next = (*pool).free_list;
        (*pool).free_list = chunk;
    }

    if (*chunk).first_free > slot_idx {
        (*chunk).first_free = slot_idx;
    }

    memcpy(context, get_context(chunk, slot_idx), (*pool).context_size);

    memset(
        get_context(chunk, slot_idx),
        0 as c_int,
        (*pool).context_size,
    );
}

unsafe fn alloc_adjustor_chunk(mut owner: *mut AdjustorPool) -> *mut AdjustorChunk {
    let mut exec_page = allocateExecPage();

    if exec_page.is_null() {
        barf(b"alloc_adjustor_chunk: failed to allocate\0" as *const u8 as *const c_char);
    }

    let mut adj_page = exec_page as *mut AdjustorExecPage;
    (*adj_page).magic = ADJUSTOR_EXEC_PAGE_MAGIC as uint64_t;

    let mut bitmap_sz: size_t = (*owner)
        .chunk_slots
        .wrapping_add((8 as size_t).wrapping_mul(size_of::<*mut c_void>() as size_t))
        .wrapping_sub(1 as size_t)
        .wrapping_div((8 as size_t).wrapping_mul(size_of::<*mut c_void>() as size_t))
        .wrapping_mul((8 as size_t).wrapping_mul(size_of::<*mut c_void>() as size_t))
        .wrapping_div(8 as size_t);

    let mut contexts_sz: size_t = (*owner).context_size.wrapping_mul((*owner).chunk_slots);
    let mut alloc_sz: size_t = (size_of::<AdjustorChunk>() as size_t)
        .wrapping_add(bitmap_sz)
        .wrapping_add(contexts_sz);

    let mut chunk = stgMallocBytes(
        alloc_sz,
        b"allocAdjustorChunk\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut AdjustorChunk;

    (*chunk).owner = owner;
    (*chunk).first_free = 0 as size_t;
    (*chunk).contexts = (&raw mut (*chunk).slot_bitmap as *mut uint8_t).offset(bitmap_sz as isize)
        as *mut AdjustorContext as *mut c_void;
    (*chunk).free_list_next = null_mut::<AdjustorChunk>();
    (*chunk).exec_page = adj_page;
    (*(*chunk).exec_page).owner = chunk;

    memset(
        &raw mut (*chunk).slot_bitmap as *mut uint8_t as *mut c_void,
        0 as c_int,
        bitmap_sz,
    );

    memset((*chunk).contexts, 0 as c_int, contexts_sz);

    let mut code_sz: size_t = (*owner).adjustor_code_size;
    let mut i: size_t = 0 as size_t;

    while i < (*owner).chunk_slots {
        (*owner).make_code.expect("non-null function pointer")(
            (&raw mut (*adj_page).adjustor_code as *mut uint8_t)
                .offset(i.wrapping_mul(code_sz) as isize) as *mut uint8_t,
            get_context(chunk, i),
            (*owner).user_data,
        );

        i = i.wrapping_add(1);
    }

    freezeExecPage(exec_page);

    return chunk;
}

unsafe fn mk_adjustor_from_template(
    mut exec_code: *mut uint8_t,
    mut context: *const c_void,
    mut user_data: *mut c_void,
) {
    let mut adjustor_context = context as *const AdjustorContext;
    let mut tmpl = user_data as *mut AdjustorTemplate;

    memcpy(
        exec_code as *mut c_void,
        (*tmpl).code_start as *const c_void,
        (*tmpl).code_end.offset_from((*tmpl).code_start) as c_long as size_t,
    );

    let mut context_off: size_t =
        ((*tmpl).context_ptr as *mut uint8_t).offset_from((*tmpl).code_start) as c_long as size_t;

    let mut slot_context_ptr =
        exec_code.offset(context_off as isize) as *mut *const AdjustorContext;
    *slot_context_ptr = adjustor_context;
}

unsafe fn new_adjustor_pool_from_template(mut tmpl: *const AdjustorTemplate) -> *mut AdjustorPool {
    let mut code_size: size_t =
        (*tmpl).code_end.offset_from((*tmpl).code_start) as c_long as size_t;

    return new_adjustor_pool(
        size_of::<AdjustorContext>() as size_t,
        code_size,
        Some(
            mk_adjustor_from_template
                as unsafe extern "C" fn(*mut uint8_t, *const c_void, *mut c_void) -> (),
        ),
        tmpl as *mut c_void,
    );
}
