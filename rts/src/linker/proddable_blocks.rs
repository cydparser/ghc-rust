use crate::ffi::rts::messages::barf;
use crate::linker::proddable_blocks::ProddableBlockSet;
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgReallocBytes};

/// cbindgen:no-export
pub(crate) struct ProddableBlockSet {
    pub(crate) size: usize,
    pub(crate) capacity: usize,
    pub(crate) data: *mut _ProddableBlock,
}

/// cbindgen:no-export
struct _ProddableBlock {
    start: usize,
    end: usize,
}

type ProddableBlock = _ProddableBlock;

unsafe fn initProddableBlockSet(mut set: *mut ProddableBlockSet) {
    (*set).data = null_mut::<_ProddableBlock>();
    (*set).capacity = 0;
    (*set).size = 0;
}

unsafe fn freeProddableBlocks(mut set: *mut ProddableBlockSet) {
    stgFree((*set).data as *mut c_void);
    (*set).data = null_mut::<_ProddableBlock>();
    (*set).size = 0;
    (*set).capacity = 0;
}

unsafe fn findLower(mut set: *const ProddableBlockSet, mut value: usize) -> usize {
    let mut l: usize = 0;
    let mut r: usize = (*set).size;

    while l < r {
        let mut mid: usize = l.wrapping_add(r.wrapping_sub(l).wrapping_div(2 as usize));

        if (*(*set).data.offset(mid as isize)).start < value {
            l = mid.wrapping_add(1 as usize);
        } else {
            r = mid;
        }
    }

    return l;
}

unsafe fn containsSpan(
    mut set: *const ProddableBlockSet,
    mut start: usize,
    mut end: usize,
) -> bool {
    let mut i = findLower(set, start.wrapping_add(1 as i32 as usize));

    return i > 0
        && (*(*set).data.offset(i.wrapping_sub(1 as usize) as isize)).start <= start
        && end <= (*(*set).data.offset(i.wrapping_sub(1 as usize) as isize)).end;
}

unsafe fn checkProddableBlock(
    mut set: *const ProddableBlockSet,
    mut addr: *mut c_void,
    mut size: usize,
) {
    if !containsSpan(
        set,
        addr as usize,
        (addr as usize).wrapping_add(size as usize),
    ) {
        barf(
            c"checkProddableBlock: invalid fixup in runtime linker: %p".as_ptr(),
            addr,
        );
    }
}

unsafe fn ensureCapacity(mut set: *mut ProddableBlockSet, mut new_capacity: usize) {
    if new_capacity > (*set).capacity {
        let mut cap: usize = if (*set).capacity != 0 {
            (*set).capacity.wrapping_mul(2 as usize)
        } else {
            4
        };

        if cap < new_capacity {
            cap = new_capacity;
        }

        let mut tmp = stgReallocBytes(
            (*set).data as *mut c_void,
            cap.wrapping_mul(size_of::<ProddableBlock>() as usize),
            c"addProddableBlock".as_ptr(),
        ) as *mut ProddableBlock;

        (*set).data = tmp as *mut _ProddableBlock;
        (*set).capacity = cap;
    }
}

unsafe fn addProddableBlock(
    mut set: *mut ProddableBlockSet,
    mut start_ptr: *mut c_void,
    mut size: usize,
) {
    let start: usize = start_ptr as usize;
    let end: usize = start.wrapping_add(size as usize);
    let mut i = findLower(set, start);

    if i > 0
        && start
            <= (*(*set).data.offset(i.wrapping_sub(1 as usize) as isize))
                .end
                .wrapping_add(1 as i32 as usize)
    {
        i = i.wrapping_sub(1);

        if end > (*(*set).data.offset(i as isize)).end {
            (*(*set).data.offset(i as isize)).end = end;
        }
    } else {
        ensureCapacity(set, (*set).size.wrapping_add(1 as usize));

        memmove(
            (*set).data.offset(i.wrapping_add(1 as usize) as isize) as *mut _ProddableBlock
                as *mut c_void,
            (*set).data.offset(i as isize) as *mut _ProddableBlock as *const c_void,
            (size_of::<ProddableBlock>() as usize).wrapping_mul((*set).size.wrapping_sub(i)),
        );

        (*(*set).data.offset(i as isize)).start = start;
        (*(*set).data.offset(i as isize)).end = end;
        (*set).size = (*set).size.wrapping_add(1);
    }

    let mut j: usize = i;

    while j < (*set).size
        && (*(*set).data.offset(j as isize)).start
            <= (*(*set).data.offset(i as isize))
                .end
                .wrapping_add(1 as i32 as usize)
    {
        (*(*set).data.offset(i as isize)).end = (*(*set).data.offset(j as isize)).end;
        j = j.wrapping_add(1);
    }

    if j != i {
        memmove(
            (*set).data.offset(i.wrapping_add(1 as usize) as isize) as *mut _ProddableBlock
                as *mut c_void,
            (*set).data.offset(j as isize) as *mut _ProddableBlock as *const c_void,
            (size_of::<ProddableBlock>() as usize).wrapping_mul((*set).size.wrapping_sub(j)),
        );

        (*set).size = (*set)
            .size
            .wrapping_sub(j.wrapping_sub(i).wrapping_sub(1 as usize));
    }
}
