use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, sysErrorBelch};
use crate::ffi::stg::misc_closures::stg_upd_frame_info;
use crate::linker::m_map::{
    MEM_READ_EXECUTE, MEM_READ_WRITE, mmapAnonForLinker, mprotectForLinker, munmapForLinker,
};
use crate::linker::m32_alloc::m32_allocator;
use crate::prelude::*;
use crate::report_memory_map::reportMemoryMap;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::os_mem::getPageSize;

pub(crate) type m32_allocator = m32_allocator_t;

/// cbindgen:no-export
struct m32_allocator_t {
    executable: bool,
    unprotected_list: *mut m32_page_t,
    protected_list: *mut m32_page_t,
    pages: [*mut m32_page_t; 32],
}

/// cbindgen:no-export
struct m32_page_t {
    c2rust_unnamed: C2RustUnnamed_6,
    contents: [uint8_t; 0],
}

union C2RustUnnamed_6 {
    filled_page: C2RustUnnamed_8,
    current_size: size_t,
    free_page: C2RustUnnamed_7,
}

/// cbindgen:no-export
struct C2RustUnnamed_7 {
    next: *mut m32_page_t,
}

/// cbindgen:no-export
struct C2RustUnnamed_8 {
    size: uint32_t,
    next: *mut m32_page_t,
}

const M32_MAX_PAGES: c_int = 32 as c_int;

const M32_MAP_PAGES: c_int = 32 as c_int;

const M32_MAX_FREE_PAGE_POOL_SIZE: c_int = 256 as c_int;

unsafe fn is_okay_address(mut p: *mut c_void) -> bool {
    let mut here = &raw const stg_upd_frame_info as *mut c_void as *mut int8_t;
    let mut displacement: ssize_t = (p as *mut int8_t).offset_from(here) as ssize_t;

    return RtsFlags.MiscFlags.linkerAlwaysPic as c_int != 0
        || displacement > -(0x7fffffff as c_int) as ssize_t
            && displacement < 0x7fffffff as c_int as ssize_t;
}

unsafe fn m32_filled_page_set_next(mut page: *mut m32_page_t, mut next: *mut m32_page_t) {
    if !next.is_null() && !is_okay_address(next as *mut c_void) {
        barf(
            b"m32_filled_page_set_next: Page %p not within 4GB of program text\0" as *const u8
                as *const c_char,
            next,
        );
    }

    (*page).c2rust_unnamed.filled_page.next = next;
}

unsafe fn m32_filled_page_get_next(mut page: *mut m32_page_t) -> *mut m32_page_t {
    return (*page).c2rust_unnamed.filled_page.next as uintptr_t as *mut m32_page_t;
}

static mut m32_free_page_pool: *mut m32_page_t = null::<m32_page_t>() as *mut m32_page_t;

static mut m32_free_page_pool_size: c_uint = 0 as c_uint;

unsafe fn m32_release_page(mut page: *mut m32_page_t) {
    let pgsz = getPageSize() as size_t;
    let mut sz: ssize_t = (*page).c2rust_unnamed.filled_page.size as ssize_t;

    while sz > 0 as ssize_t {
        if !(m32_free_page_pool_size < M32_MAX_FREE_PAGE_POOL_SIZE as c_uint) {
            break;
        }

        mprotectForLinker(page as *mut c_void, pgsz, MEM_READ_WRITE);
        (*page).c2rust_unnamed.free_page.next = m32_free_page_pool;
        m32_free_page_pool = page;
        m32_free_page_pool_size = m32_free_page_pool_size.wrapping_add(1);
        page = (page as *mut uint8_t).offset(pgsz as isize) as *mut m32_page_t;
        sz = (sz as size_t).wrapping_sub(pgsz) as ssize_t as ssize_t;
    }

    if sz > 0 as ssize_t {
        munmapForLinker(
            page as *mut c_void,
            (sz as size_t).wrapping_add(pgsz).wrapping_sub(1 as size_t)
                & !pgsz.wrapping_sub(1 as size_t),
            b"m32_release_page\0" as *const u8 as *const c_char,
        );
    }
}

unsafe fn m32_alloc_page() -> *mut m32_page_t {
    if m32_free_page_pool_size == 0 as c_uint {
        let pgsz = getPageSize() as size_t;
        let map_sz: size_t = pgsz.wrapping_mul(M32_MAP_PAGES as size_t);
        let mut chunk = mmapAnonForLinker(map_sz) as *mut uint8_t;

        if !is_okay_address(chunk.offset(map_sz as isize) as *mut c_void) {
            reportMemoryMap();

            barf(
                b"m32_alloc_page: failed to allocate pages within 4GB of program text (got %p)\0"
                    as *const u8 as *const c_char,
                chunk,
            );
        }

        let mut i = 0 as c_int;

        while i < M32_MAP_PAGES {
            let mut page =
                chunk.offset((i as size_t).wrapping_mul(pgsz) as isize) as *mut m32_page_t;
            (*page).c2rust_unnamed.free_page.next = chunk
                .offset(((i + 1 as c_int) as size_t).wrapping_mul(pgsz) as isize)
                as *mut m32_page_t;
            i += 1;
        }

        let ref mut fresh5 = (*(chunk
            .offset(((32 as c_int - 1 as c_int) as size_t).wrapping_mul(pgsz) as isize)
            as *mut m32_page_t))
            .c2rust_unnamed
            .free_page
            .next;
        *fresh5 = m32_free_page_pool;
        m32_free_page_pool = chunk as *mut m32_page_t;
        m32_free_page_pool_size = m32_free_page_pool_size.wrapping_add(M32_MAP_PAGES as c_uint);
    }

    let mut page_0 = m32_free_page_pool;
    m32_free_page_pool = (*page_0).c2rust_unnamed.free_page.next;
    m32_free_page_pool_size = m32_free_page_pool_size.wrapping_sub(1);

    return page_0;
}

unsafe fn m32_allocator_new(mut executable: bool) -> *mut m32_allocator {
    let mut alloc = stgMallocBytes(
        size_of::<m32_allocator>() as size_t,
        b"m32_new_allocator\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut m32_allocator;

    memset(
        alloc as *mut c_void,
        0 as c_int,
        size_of::<m32_allocator_t>() as size_t,
    );

    (*alloc).executable = executable;

    return alloc;
}

unsafe fn m32_allocator_unmap_list(mut head: *mut m32_page_t) {
    while !head.is_null() {
        let mut next = m32_filled_page_get_next(head);
        m32_release_page(head);
        head = next;
    }
}

unsafe fn m32_allocator_free(mut alloc: *mut m32_allocator) {
    m32_allocator_unmap_list((*alloc).unprotected_list);
    m32_allocator_unmap_list((*alloc).protected_list);

    let mut i = 0 as c_int;

    while i < M32_MAX_PAGES {
        if !(*alloc).pages[i as usize].is_null() {
            m32_release_page((*alloc).pages[i as usize]);
        }

        i += 1;
    }

    stgFree(alloc as *mut c_void);
}

unsafe fn m32_allocator_push_filled_list(
    mut head: *mut *mut m32_page_t,
    mut page: *mut m32_page_t,
) {
    m32_filled_page_set_next(page, *head);
    *head = page;
}

unsafe fn m32_allocator_flush(mut alloc: *mut m32_allocator) {
    let mut i = 0 as c_int;

    while i < M32_MAX_PAGES {
        if !(*alloc).pages[i as usize].is_null() {
            if (*(*alloc).pages[i as usize]).c2rust_unnamed.current_size
                == size_of::<m32_page_t>() as usize
            {
                m32_release_page((*alloc).pages[i as usize]);
            } else {
                m32_allocator_push_filled_list(
                    &raw mut (*alloc).unprotected_list,
                    (*alloc).pages[i as usize],
                );
            }

            (*alloc).pages[i as usize] = null_mut::<m32_page_t>();
        }

        i += 1;
    }

    if (*alloc).executable {
        let mut page = (*alloc).unprotected_list;

        while !page.is_null() {
            let mut next = m32_filled_page_get_next(page);
            m32_allocator_push_filled_list(&raw mut (*alloc).protected_list, page);

            mprotectForLinker(
                page as *mut c_void,
                (*page).c2rust_unnamed.filled_page.size as size_t,
                MEM_READ_EXECUTE,
            );

            page = next;
        }

        (*alloc).unprotected_list = null_mut::<m32_page_t>();
    }
}

unsafe fn m32_is_large_object(mut size: size_t, mut alignment: size_t) -> bool {
    return size
        >= getPageSize().wrapping_sub(
            (size_of::<m32_page_t>() as size_t)
                .wrapping_add(alignment)
                .wrapping_sub(1 as size_t)
                & !alignment.wrapping_sub(1 as size_t),
        );
}

unsafe fn m32_report_allocation(
    mut alloc: *mut m32_allocator_t,
    mut addr: *mut c_void,
    mut size: size_t,
) {
}

unsafe fn m32_alloc(
    mut alloc: *mut m32_allocator_t,
    mut size: size_t,
    mut alignment: size_t,
) -> *mut c_void {
    let mut pgsz = getPageSize();

    if m32_is_large_object(size, alignment) {
        let mut alsize: size_t = (size_of::<m32_page_t>() as size_t)
            .wrapping_add(alignment)
            .wrapping_sub(1 as size_t)
            & !alignment.wrapping_sub(1 as size_t);

        let mut page = mmapAnonForLinker(alsize.wrapping_add(size)) as *mut m32_page_t;

        if page.is_null() {
            sysErrorBelch(
                b"m32_alloc: Failed to map pages for %zd bytes\0" as *const u8 as *const c_char,
                size,
            );

            return NULL;
        } else if !is_okay_address(page as *mut c_void) {
            reportMemoryMap();

            barf(
                b"m32_alloc: warning: Allocation of %zd bytes resulted in pages above 4GB (%p)\0"
                    as *const u8 as *const c_char,
                size,
                page,
            );
        }

        (*page).c2rust_unnamed.filled_page.size = alsize.wrapping_add(size) as uint32_t;
        m32_allocator_push_filled_list(&raw mut (*alloc).unprotected_list, page);

        let mut res = (page as *mut uint8_t).offset(alsize as isize);
        m32_report_allocation(alloc, res as *mut c_void, size);

        return res as *mut c_void;
    }

    let mut empty = -(1 as c_int);
    let mut most_filled = -(1 as c_int);
    let mut i: c_int = 0;
    i = 0 as c_int;

    while i < M32_MAX_PAGES {
        if (*alloc).pages[i as usize].is_null() {
            empty = if empty == -(1 as c_int) { i } else { empty };
        } else {
            let mut alsize_0: size_t = (*(*alloc).pages[i as usize])
                .c2rust_unnamed
                .current_size
                .wrapping_add(alignment)
                .wrapping_sub(1 as size_t)
                & !alignment.wrapping_sub(1 as size_t);

            if size <= pgsz.wrapping_sub(alsize_0) {
                let mut addr = ((*alloc).pages[i as usize] as *mut c_char).offset(alsize_0 as isize)
                    as *mut c_void;
                (*(*alloc).pages[i as usize]).c2rust_unnamed.current_size =
                    alsize_0.wrapping_add(size);
                m32_report_allocation(alloc, addr, size);

                return addr;
            }

            if most_filled == -(1 as c_int)
                || (*(*alloc).pages[most_filled as usize])
                    .c2rust_unnamed
                    .current_size
                    < (*(*alloc).pages[i as usize]).c2rust_unnamed.current_size
            {
                most_filled = i;
            }
        }

        i += 1;
    }

    if empty == -(1 as c_int) {
        m32_allocator_push_filled_list(
            &raw mut (*alloc).unprotected_list,
            (*alloc).pages[most_filled as usize],
        );

        (*alloc).pages[most_filled as usize] = null_mut::<m32_page_t>();
        empty = most_filled;
    }

    let mut page_0 = m32_alloc_page();

    if page_0.is_null() {
        return NULL;
    }

    (*alloc).pages[empty as usize] = page_0;
    (*(*alloc).pages[empty as usize])
        .c2rust_unnamed
        .current_size = size.wrapping_add(
        (size_of::<m32_page_t>() as size_t)
            .wrapping_add(alignment)
            .wrapping_sub(1 as size_t)
            & !alignment.wrapping_sub(1 as size_t),
    );

    let mut res_0 = (page_0 as *mut uint8_t).offset(
        ((size_of::<m32_page_t>() as usize)
            .wrapping_add(alignment as usize)
            .wrapping_sub(1 as usize)
            & !(alignment as usize).wrapping_sub(1 as usize)) as isize,
    );

    m32_report_allocation(alloc, res_0 as *mut c_void, size);

    return res_0 as *mut c_void;
}
