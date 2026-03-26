use crate::ffi::rts::messages::barf;
use crate::linker::proddable_blocks::ProddableBlockSet;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgReallocBytes};

/// cbindgen:no-export
pub(crate) struct ProddableBlockSet {
    pub(crate) size: size_t,
    pub(crate) capacity: size_t,
    pub(crate) data: *mut _ProddableBlock,
}

/// cbindgen:no-export
struct _ProddableBlock {
    start: uintptr_t,
    end: uintptr_t,
}

type ProddableBlock = _ProddableBlock;

unsafe fn initProddableBlockSet(mut set: *mut ProddableBlockSet) {
    (*set).data = null_mut::<_ProddableBlock>();
    (*set).capacity = 0 as size_t;
    (*set).size = 0 as size_t;
}

unsafe fn freeProddableBlocks(mut set: *mut ProddableBlockSet) {
    stgFree((*set).data as *mut c_void);
    (*set).data = null_mut::<_ProddableBlock>();
    (*set).size = 0 as size_t;
    (*set).capacity = 0 as size_t;
}

unsafe fn findLower(mut set: *const ProddableBlockSet, mut value: uintptr_t) -> size_t {
    let mut l: size_t = 0 as size_t;
    let mut r: size_t = (*set).size;

    while l < r {
        let mut mid: size_t = l.wrapping_add(r.wrapping_sub(l).wrapping_div(2 as size_t));

        if (*(*set).data.offset(mid as isize)).start < value {
            l = mid.wrapping_add(1 as size_t);
        } else {
            r = mid;
        }
    }

    return l;
}

unsafe fn containsSpan(
    mut set: *const ProddableBlockSet,
    mut start: uintptr_t,
    mut end: uintptr_t,
) -> bool {
    let mut i = findLower(set, start.wrapping_add(1 as c_int as uintptr_t));

    return i > 0 as size_t
        && (*(*set).data.offset(i.wrapping_sub(1 as size_t) as isize)).start <= start
        && end <= (*(*set).data.offset(i.wrapping_sub(1 as size_t) as isize)).end;
}

unsafe fn checkProddableBlock(
    mut set: *const ProddableBlockSet,
    mut addr: *mut c_void,
    mut size: size_t,
) {
    if !containsSpan(
        set,
        addr as uintptr_t,
        (addr as uintptr_t).wrapping_add(size as uintptr_t),
    ) {
        barf(
            b"checkProddableBlock: invalid fixup in runtime linker: %p\0" as *const u8
                as *const c_char,
            addr,
        );
    }
}

unsafe fn ensureCapacity(mut set: *mut ProddableBlockSet, mut new_capacity: size_t) {
    if new_capacity > (*set).capacity {
        let mut cap: size_t = if (*set).capacity != 0 {
            (*set).capacity.wrapping_mul(2 as size_t)
        } else {
            4 as size_t
        };

        if cap < new_capacity {
            cap = new_capacity;
        }

        let mut tmp = stgReallocBytes(
            (*set).data as *mut c_void,
            cap.wrapping_mul(size_of::<ProddableBlock>() as size_t),
            b"addProddableBlock\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut ProddableBlock;

        (*set).data = tmp as *mut _ProddableBlock;
        (*set).capacity = cap;
    }
}

unsafe fn addProddableBlock(
    mut set: *mut ProddableBlockSet,
    mut start_ptr: *mut c_void,
    mut size: size_t,
) {
    let start: uintptr_t = start_ptr as uintptr_t;
    let end: uintptr_t = start.wrapping_add(size as uintptr_t);
    let mut i = findLower(set, start);

    if i > 0 as size_t
        && start
            <= (*(*set).data.offset(i.wrapping_sub(1 as size_t) as isize))
                .end
                .wrapping_add(1 as c_int as uintptr_t)
    {
        i = i.wrapping_sub(1);

        if end > (*(*set).data.offset(i as isize)).end {
            (*(*set).data.offset(i as isize)).end = end;
        }
    } else {
        ensureCapacity(set, (*set).size.wrapping_add(1 as size_t));

        memmove(
            (*set).data.offset(i.wrapping_add(1 as size_t) as isize) as *mut _ProddableBlock
                as *mut c_void,
            (*set).data.offset(i as isize) as *mut _ProddableBlock as *const c_void,
            (size_of::<ProddableBlock>() as size_t).wrapping_mul((*set).size.wrapping_sub(i)),
        );

        (*(*set).data.offset(i as isize)).start = start;
        (*(*set).data.offset(i as isize)).end = end;
        (*set).size = (*set).size.wrapping_add(1);
    }

    let mut j: size_t = i;

    while j < (*set).size
        && (*(*set).data.offset(j as isize)).start
            <= (*(*set).data.offset(i as isize))
                .end
                .wrapping_add(1 as c_int as uintptr_t)
    {
        (*(*set).data.offset(i as isize)).end = (*(*set).data.offset(j as isize)).end;
        j = j.wrapping_add(1);
    }

    if j != i {
        memmove(
            (*set).data.offset(i.wrapping_add(1 as size_t) as isize) as *mut _ProddableBlock
                as *mut c_void,
            (*set).data.offset(j as isize) as *mut _ProddableBlock as *const c_void,
            (size_of::<ProddableBlock>() as size_t).wrapping_mul((*set).size.wrapping_sub(j)),
        );

        (*set).size = (*set)
            .size
            .wrapping_sub(j.wrapping_sub(i).wrapping_sub(1 as size_t));
    }
}
