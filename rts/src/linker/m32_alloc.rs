use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch, sysErrorBelch};
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
    c2rust_unnamed: C2RustUnnamed_7,
    r#type: m32_page_type,
    contents: [u8; 0],
}

type m32_page_type = u32;

const FILLED_PAGE: m32_page_type = 2;

const NURSERY_PAGE: m32_page_type = 1;

const FREE_PAGE: m32_page_type = 0;

union C2RustUnnamed_7 {
    filled_page: C2RustUnnamed_9,
    current_size: usize,
    free_page: C2RustUnnamed_8,
}

/// cbindgen:no-export
struct C2RustUnnamed_8 {
    next: *mut m32_page_t,
}

/// cbindgen:no-export
struct C2RustUnnamed_9 {
    size: u32,
    next: *mut m32_page_t,
}

const M32_MAX_PAGES: i32 = 32;

const M32_MAP_PAGES: i32 = 32;

const M32_MAX_FREE_PAGE_POOL_SIZE: i32 = 256;

unsafe fn is_okay_address(mut p: *mut c_void) -> bool {
    let mut here = &raw const stg_upd_frame_info as *mut c_void as *mut i8;
    let mut displacement: isize = (p as *mut i8).offset_from(here) as isize;

    return RtsFlags.MiscFlags.linkerAlwaysPic as i32 != 0
        || displacement > -0x7fffffff as isize && displacement < 0x7fffffff;
}

unsafe fn ASSERT_PAGE_ALIGNED(mut page: *mut c_void) {
    let pgsz = getPageSize() as usize;

    if page as usize & (pgsz as usize).wrapping_sub(1 as i32 as usize) != 0 {
        barf(c"m32: invalid page alignment".as_ptr());
    }
}

unsafe fn ASSERT_VALID_PAGE(mut page: *mut m32_page_t) {
    ASSERT_PAGE_ALIGNED(page as *mut c_void);

    match (*page).r#type as u32 {
        0 | 1 | 2 => {}
        _ => {
            barf(c"m32: invalid page state\n".as_ptr());
        }
    };
}

unsafe fn ASSERT_PAGE_TYPE(mut page: *mut m32_page_t, mut ty: m32_page_type) {
    if (*page).r#type as u32 != ty as u32 {
        barf(c"m32: unexpected page type".as_ptr());
    }
}

unsafe fn ASSERT_PAGE_NOT_FREE(mut page: *mut m32_page_t) {
    if (*page).r#type as u32 == FREE_PAGE as i32 as u32 {
        barf(c"m32: unexpected free page".as_ptr());
    }
}

unsafe fn SET_PAGE_TYPE(mut page: *mut m32_page_t, mut ty: m32_page_type) {
    (*page).r#type = ty;
}

unsafe fn m32_filled_page_set_next(mut page: *mut m32_page_t, mut next: *mut m32_page_t) {
    ASSERT_PAGE_TYPE(page, FILLED_PAGE);

    if !next.is_null() && !is_okay_address(next as *mut c_void) {
        barf(
            c"m32_filled_page_set_next: Page %p not within 4GB of program text".as_ptr(),
            next,
        );
    }

    (*page).c2rust_unnamed.filled_page.next = next;
}

unsafe fn m32_filled_page_get_next(mut page: *mut m32_page_t) -> *mut m32_page_t {
    ASSERT_PAGE_TYPE(page, FILLED_PAGE);

    return (*page).c2rust_unnamed.filled_page.next as usize as *mut m32_page_t;
}

static mut m32_free_page_pool: *mut m32_page_t = null_mut::<m32_page_t>();

static mut m32_free_page_pool_size: u32 = 0;

unsafe fn m32_release_page(mut page: *mut m32_page_t) {
    ASSERT_VALID_PAGE(page);
    ASSERT_PAGE_NOT_FREE(page);

    let pgsz = getPageSize() as usize;
    let mut sz: isize = (*page).c2rust_unnamed.filled_page.size as isize;

    while sz > 0 {
        if !(m32_free_page_pool_size < M32_MAX_FREE_PAGE_POOL_SIZE as u32) {
            break;
        }

        mprotectForLinker(page as *mut c_void, pgsz, MEM_READ_WRITE);

        if RtsFlags.DebugFlags.sanity {
            memset(page as *mut c_void, 0xaa, pgsz);
        }

        SET_PAGE_TYPE(page, FREE_PAGE);
        (*page).c2rust_unnamed.free_page.next = m32_free_page_pool;
        m32_free_page_pool = page;
        m32_free_page_pool_size = m32_free_page_pool_size.wrapping_add(1);
        page = (page as *mut u8).offset(pgsz as isize) as *mut m32_page_t;
        sz = (sz as usize).wrapping_sub(pgsz) as isize as isize;
    }

    if sz > 0 {
        munmapForLinker(
            page as *mut c_void,
            (sz as usize).wrapping_add(pgsz).wrapping_sub(1 as usize)
                & !pgsz.wrapping_sub(1 as usize),
            c"m32_release_page".as_ptr(),
        );
    }
}

unsafe fn m32_alloc_page() -> *mut m32_page_t {
    if m32_free_page_pool_size == 0 {
        let pgsz = getPageSize() as usize;
        let map_sz: usize = pgsz.wrapping_mul(M32_MAP_PAGES as usize);
        let mut chunk = mmapAnonForLinker(map_sz) as *mut u8;

        if !is_okay_address(chunk.offset(map_sz as isize) as *mut c_void) {
            reportMemoryMap();

            barf(
                c"m32_alloc_page: failed to allocate pages within 4GB of program text (got %p)"
                    .as_ptr(),
                chunk,
            );
        }

        if RtsFlags.DebugFlags.sanity {
            memset(chunk as *mut c_void, 0xaa, map_sz);
        }

        let mut i = 0;

        while i < M32_MAP_PAGES {
            let mut page =
                chunk.offset((i as usize).wrapping_mul(pgsz) as isize) as *mut m32_page_t;
            SET_PAGE_TYPE(page, FREE_PAGE);
            (*page).c2rust_unnamed.free_page.next = chunk
                .offset(((i + 1 as i32) as usize).wrapping_mul(pgsz) as isize)
                as *mut m32_page_t;
            i += 1;
        }

        let ref mut fresh5 = (*(chunk
            .offset(((32 as i32 - 1 as i32) as usize).wrapping_mul(pgsz) as isize)
            as *mut m32_page_t))
            .c2rust_unnamed
            .free_page
            .next;
        *fresh5 = m32_free_page_pool;
        m32_free_page_pool = chunk as *mut m32_page_t;
        m32_free_page_pool_size = m32_free_page_pool_size.wrapping_add(M32_MAP_PAGES as u32);
    }

    let mut page_0 = m32_free_page_pool;
    m32_free_page_pool = (*page_0).c2rust_unnamed.free_page.next;
    m32_free_page_pool_size = m32_free_page_pool_size.wrapping_sub(1);
    ASSERT_PAGE_TYPE(page_0, FREE_PAGE);

    return page_0;
}

unsafe fn m32_allocator_new(mut executable: bool) -> *mut m32_allocator {
    let mut alloc = stgMallocBytes(
        size_of::<m32_allocator>() as usize,
        c"m32_new_allocator".as_ptr(),
    ) as *mut m32_allocator;

    memset(
        alloc as *mut c_void,
        0,
        size_of::<m32_allocator_t>() as usize,
    );
    (*alloc).executable = executable;

    return alloc;
}

unsafe fn m32_allocator_unmap_list(mut head: *mut m32_page_t) {
    while !head.is_null() {
        ASSERT_VALID_PAGE(head);

        let mut next = m32_filled_page_get_next(head);
        m32_release_page(head);
        head = next;
    }
}

unsafe fn m32_allocator_free(mut alloc: *mut m32_allocator) {
    m32_allocator_unmap_list((*alloc).unprotected_list);
    m32_allocator_unmap_list((*alloc).protected_list);

    let mut i = 0;

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
    ASSERT_PAGE_TYPE(page, FILLED_PAGE);
    m32_filled_page_set_next(page, *head);
    *head = page;
}

unsafe fn m32_allocator_flush(mut alloc: *mut m32_allocator) {
    let mut i = 0;

    while i < M32_MAX_PAGES {
        if !(*alloc).pages[i as usize].is_null() {
            if (*(*alloc).pages[i as usize]).c2rust_unnamed.current_size
                == size_of::<m32_page_t>() as usize
            {
                m32_release_page((*alloc).pages[i as usize]);
            } else {
                SET_PAGE_TYPE((*alloc).pages[i as usize], FILLED_PAGE);

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
            ASSERT_PAGE_TYPE(page, FILLED_PAGE);

            let mut next = m32_filled_page_get_next(page);
            m32_allocator_push_filled_list(&raw mut (*alloc).protected_list, page);

            mprotectForLinker(
                page as *mut c_void,
                (*page).c2rust_unnamed.filled_page.size as usize,
                MEM_READ_EXECUTE,
            );

            page = next;
        }

        (*alloc).unprotected_list = null_mut::<m32_page_t>();
    }
}

unsafe fn m32_is_large_object(mut size: usize, mut alignment: usize) -> bool {
    return size
        >= getPageSize().wrapping_sub(
            (size_of::<m32_page_t>() as usize)
                .wrapping_add(alignment)
                .wrapping_sub(1 as usize)
                & !alignment.wrapping_sub(1 as usize),
        );
}

unsafe fn m32_report_allocation(
    mut alloc: *mut m32_allocator_t,
    mut addr: *mut c_void,
    mut size: usize,
) {
    if RtsFlags.DebugFlags.linker_verbose {
        debugBelch(
            c"m32_allocated(%p:%s): %p - %p\n".as_ptr(),
            alloc,
            if (*alloc).executable as i32 != 0 {
                c"RX".as_ptr()
            } else {
                c"RW".as_ptr()
            },
            addr,
            (addr as *mut u8).offset(size as isize),
        );
    }
}

unsafe fn m32_alloc(
    mut alloc: *mut m32_allocator_t,
    mut size: usize,
    mut alignment: usize,
) -> *mut c_void {
    let mut pgsz = getPageSize();

    if m32_is_large_object(size, alignment) {
        let mut alsize: usize = (size_of::<m32_page_t>() as usize)
            .wrapping_add(alignment)
            .wrapping_sub(1 as usize)
            & !alignment.wrapping_sub(1 as usize);

        let mut page = mmapAnonForLinker(alsize.wrapping_add(size)) as *mut m32_page_t;

        if page.is_null() {
            sysErrorBelch(
                c"m32_alloc: Failed to map pages for %zd bytes".as_ptr(),
                size,
            );

            return NULL;
        } else if !is_okay_address(page as *mut c_void) {
            reportMemoryMap();

            barf(
                c"m32_alloc: warning: Allocation of %zd bytes resulted in pages above 4GB (%p)"
                    .as_ptr(),
                size,
                page,
            );
        }

        SET_PAGE_TYPE(page, FILLED_PAGE);
        (*page).c2rust_unnamed.filled_page.size = alsize.wrapping_add(size) as u32;
        m32_allocator_push_filled_list(&raw mut (*alloc).unprotected_list, page);

        let mut res = (page as *mut u8).offset(alsize as isize);
        m32_report_allocation(alloc, res as *mut c_void, size);

        return res as *mut c_void;
    }

    let mut empty = -1;
    let mut most_filled = -1;
    let mut i: i32 = 0;
    i = 0;

    while i < M32_MAX_PAGES {
        if (*alloc).pages[i as usize].is_null() {
            empty = if empty == -1 { i } else { empty };
        } else {
            ASSERT_VALID_PAGE((*alloc).pages[i as usize]);
            ASSERT_PAGE_TYPE((*alloc).pages[i as usize], NURSERY_PAGE);

            let mut alsize_0: usize = (*(*alloc).pages[i as usize])
                .c2rust_unnamed
                .current_size
                .wrapping_add(alignment)
                .wrapping_sub(1 as usize)
                & !alignment.wrapping_sub(1 as usize);

            if size <= pgsz.wrapping_sub(alsize_0) {
                let mut addr = ((*alloc).pages[i as usize] as *mut c_char).offset(alsize_0 as isize)
                    as *mut c_void;
                (*(*alloc).pages[i as usize]).c2rust_unnamed.current_size =
                    alsize_0.wrapping_add(size);
                m32_report_allocation(alloc, addr, size);

                return addr;
            }

            if most_filled == -1
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

    if empty == -1 {
        SET_PAGE_TYPE((*alloc).pages[most_filled as usize], FILLED_PAGE);

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

    SET_PAGE_TYPE(page_0, NURSERY_PAGE);
    (*alloc).pages[empty as usize] = page_0;
    (*(*alloc).pages[empty as usize])
        .c2rust_unnamed
        .current_size = size.wrapping_add(
        (size_of::<m32_page_t>() as usize)
            .wrapping_add(alignment)
            .wrapping_sub(1 as usize)
            & !alignment.wrapping_sub(1 as usize),
    );

    let mut res_0 = (page_0 as *mut u8).offset(
        ((size_of::<m32_page_t>() as usize)
            .wrapping_add(alignment as usize)
            .wrapping_sub(1 as usize)
            & !(alignment as usize).wrapping_sub(1 as usize)) as isize,
    );

    m32_report_allocation(alloc, res_0 as *mut c_void, size);

    return res_0 as *mut c_void;
}
