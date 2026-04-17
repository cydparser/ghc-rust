use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::storage::block::{BLOCK_SIZE, MBLOCK_MASK, MBLOCK_SIZE};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::m_block::{getFirstMBlock, getNextMBlock};
use crate::ffi::rts::{_assertFail, EXIT_HEAPOVERFLOW, stg_exit};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord, StgWord8, StgWord64};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::sm::os_mem::roundUpToAlign;

#[inline]
pub(crate) unsafe fn roundDownToPage(mut x: usize) -> usize {
    let mut size = getPageSize();

    return x & !size.wrapping_sub(1 as usize);
}

#[inline]
pub(crate) unsafe fn roundUpToAlign(mut size: usize, mut align: usize) -> usize {
    return size.wrapping_add(align).wrapping_sub(1 as usize) & !align.wrapping_sub(1 as usize);
}

#[inline]
pub(crate) unsafe fn roundUpToPage(mut x: usize) -> usize {
    return roundUpToAlign(x, getPageSize());
}

const MEM_RESERVE_AND_COMMIT: C2RustUnnamed_8 = 3;

const MEM_COMMIT: C2RustUnnamed_8 = 2;

const MEM_RESERVE: C2RustUnnamed_8 = 1;

type C2RustUnnamed_8 = u32;

static mut next_request: *mut c_void = null_mut::<c_void>();

unsafe fn osMemInit() {
    next_request = RtsFlags.GcFlags.heapBase as *mut c_void;
}

unsafe fn post_mmap_madvise(mut operation: i32, mut size: W_, mut ret: *mut c_void) {
    if operation & MEM_COMMIT as i32 != 0 {
        madvise(ret, size as usize, MADV_WILLNEED);
    } else {
        madvise(ret, size as usize, MADV_DONTNEED);
    };
}

unsafe fn my_mmap(mut addr: *mut c_void, mut size: W_, mut operation: i32) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    let mut err = 0;
    ret = addr;

    if operation & MEM_RESERVE as i32 != 0 {
        if !addr.is_null() {
            err = vm_allocate(
                mach_task_self_ as vm_map_t,
                &raw mut ret as *mut vm_address_t,
                size as vm_size_t,
                false,
            );
        }

        if addr.is_null() || err != 0 {
            err = vm_allocate(
                mach_task_self_ as vm_map_t,
                &raw mut ret as *mut vm_address_t,
                size as vm_size_t,
                true,
            );
        }
    }

    if err != 0 {
        errorBelch(
            c"memory allocation failed (requested %llu bytes)".as_ptr(),
            size,
        );
        stg_exit(EXIT_FAILURE);
    }

    if operation & MEM_COMMIT as i32 != 0 {
        vm_protect(
            mach_task_self_ as vm_map_t,
            ret as vm_address_t,
            size as vm_size_t,
            false,
            VM_PROT_READ | VM_PROT_WRITE,
        );
    }

    post_mmap_madvise(operation, size, ret);

    return ret;
}

unsafe fn my_mmap_or_barf(mut addr: *mut c_void, mut size: W_, mut operation: i32) -> *mut c_void {
    let mut ret = my_mmap(addr, size, operation);

    if ret.is_null() {
        if *__error() == ENOMEM
            || *__error() == EINVAL && size_of::<*mut c_void>() as usize == 4 && size >= 0xc0000000
        {
            errorBelch(c"out of memory (requested %llu bytes)".as_ptr(), size);
            stg_exit(EXIT_HEAPOVERFLOW);
        } else {
            barf(c"getMBlock: mmap: %s".as_ptr(), strerror(*__error()));
        }
    }

    return ret;
}

unsafe fn gen_map_mblocks(mut size: W_) -> *mut c_void {
    let mut slop: i32 = 0;
    let mut ret = null_mut::<StgWord8>();
    size = size.wrapping_add(MBLOCK_SIZE as W_);
    ret =
        my_mmap_or_barf(null_mut::<c_void>(), size, MEM_RESERVE_AND_COMMIT as i32) as *mut StgWord8;
    slop = (ret as W_ & MBLOCK_MASK as W_) as i32;

    if munmap(
        ret as *mut c_void,
        (MBLOCK_SIZE as usize).wrapping_sub(slop as usize),
    ) == -1
    {
        barf(c"gen_map_mblocks: munmap failed".as_ptr());
    }

    if slop > 0
        && munmap(
            ret.offset(size as isize).offset(-(slop as isize)) as *mut c_void,
            slop as usize,
        ) == -1
    {
        barf(c"gen_map_mblocks: munmap failed".as_ptr());
    }

    ret = ret.offset(MBLOCK_SIZE.wrapping_sub(slop as u64) as isize);

    return ret as *mut c_void;
}

unsafe fn osGetMBlocks(mut n: u32) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    let mut size: W_ = (MBLOCK_SIZE as W_).wrapping_mul(n as W_);

    if next_request.is_null() {
        ret = gen_map_mblocks(size);
    } else {
        ret = my_mmap_or_barf(next_request, size, MEM_RESERVE_AND_COMMIT as i32);

        if ret as W_ & MBLOCK_MASK as W_ != 0 {
            if munmap(ret, size as usize) == -1 {
                barf(c"getMBlock: munmap failed".as_ptr());
            }

            ret = gen_map_mblocks(size);
        }
    }

    next_request = (ret as *mut c_char).offset(size as isize) as *mut c_void;

    return ret;
}

unsafe fn osBindMBlocksToNode(mut addr: *mut c_void, mut size: StgWord, mut node: u32) {}

unsafe fn osFreeMBlocks(mut addr: *mut c_void, mut n: u32) {
    munmap(addr, (n as usize).wrapping_mul(MBLOCK_SIZE as usize));
}

unsafe fn osReleaseFreeMemory() {}

unsafe fn osFreeAllMBlocks() {
    let mut mblock = null_mut::<c_void>();
    let mut state = null_mut::<c_void>();
    mblock = getFirstMBlock(&raw mut state);

    while !mblock.is_null() {
        munmap(mblock, MBLOCK_SIZE as usize);
        mblock = getNextMBlock(&raw mut state, mblock);
    }
}

unsafe fn getPageSize() -> usize {
    static mut pageSize: usize = 0;

    if pageSize == 0 {
        let mut ret: i64 = 0;
        ret = sysconf(_SC_PAGESIZE);

        if ret == -1 as i64 {
            barf(c"getPageSize: cannot get page size".as_ptr());
        }

        pageSize = ret as usize;
    }

    return pageSize;
}

unsafe fn getPhysicalMemorySize() -> StgWord64 {
    static mut physMemSize: StgWord64 = 0;

    if physMemSize == 0 {
        let mut len: usize = size_of::<StgWord64>() as usize;
        let mut ret = -1;

        ret = sysctlbyname(
            c"hw.memsize".as_ptr(),
            &raw mut physMemSize as *mut c_void,
            &raw mut len,
            NULL,
            0,
        );

        if ret == -1 {
            physMemSize = 0;

            return 0;
        }
    }

    return physMemSize;
}

unsafe fn osTryReserveHeapMemory(mut len: W_, mut hint: *mut c_void) -> *mut c_void {
    let mut base = null_mut::<c_void>();
    let mut top = null_mut::<c_void>();
    let mut start = null_mut::<c_void>();
    let mut end = null_mut::<c_void>();

    if (len & !((1 as u64) << 20 as i32).wrapping_sub(1 as u64) as W_ == len) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/posix/OSMem.c".as_ptr(), 466);
    }

    base = my_mmap(
        hint,
        len.wrapping_add(MBLOCK_SIZE as W_),
        MEM_RESERVE as i32,
    );

    if base.is_null() {
        return NULL;
    }

    top = (base as W_)
        .wrapping_add(len)
        .wrapping_add(MBLOCK_SIZE as W_) as *mut c_void;

    if base as W_ & MBLOCK_MASK as W_ != 0 {
        start = ((base as W_)
            .wrapping_add(MBLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !MBLOCK_MASK as W_) as *mut c_void;
        end = (top as W_ & !MBLOCK_MASK as W_) as *mut c_void;

        if ((end as W_).wrapping_sub(start as W_) == len) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/posix/OSMem.c".as_ptr(), 481);
        }

        if munmap(base, (start as W_).wrapping_sub(base as W_) as usize) < 0 {
            sysErrorBelch(c"unable to release slop before heap".as_ptr());
        }

        if munmap(end, (top as W_).wrapping_sub(end as W_) as usize) < 0 {
            sysErrorBelch(c"unable to release slop after heap".as_ptr());
        }
    } else {
        start = base;
    }

    return start;
}

unsafe fn osReserveHeapMemory(mut startAddressPtr: *mut c_void, mut len: *mut W_) -> *mut c_void {
    let mut attempt: i32 = 0;
    let mut at = null_mut::<c_void>();
    let mut minimumAddress: W_ = (8 as i32 as W_).wrapping_mul(((1 as i32) << 30 as i32) as W_);

    let mut startAddress: W_ = 0x4200000000;

    if !startAddressPtr.is_null() {
        startAddress = startAddressPtr as W_;
    }

    if startAddress < minimumAddress {
        errorBelch(
            c"Provided heap start address %p is lower than minimum address %p".as_ptr(),
            startAddress as *mut c_void,
            minimumAddress as *mut c_void,
        );
    }

    let mut asLimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    if getrlimit(RLIMIT_AS, &raw mut asLimit) == 0
        && asLimit.rlim_cur > 0
        && *len > asLimit.rlim_cur
    {
        let mut threadAttr = _opaque_pthread_attr_t {
            __sig: 0,
            __opaque: [0; 56],
        };

        if pthread_attr_init(&raw mut threadAttr) != 0 {
            sysErrorBelch(c"failed to initialize thread attributes".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        let mut stacksz: usize = 0;

        if pthread_attr_getstacksize(&raw mut threadAttr, &raw mut stacksz) != 0 {
            sysErrorBelch(c"failed to read default thread stack size".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        if pthread_attr_destroy(&raw mut threadAttr) != 0 {
            sysErrorBelch(c"failed to destroy thread attributes".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        let mut pageSize = getPageSize();
        *len =
            (asLimit.rlim_cur as f64 * 0.666f64) as W_ & !pageSize.wrapping_sub(1 as usize) as W_;

        if asLimit.rlim_cur.wrapping_sub(*len) < stacksz.wrapping_mul(3 as usize) as W_ {
            let mut needed: usize = stacksz
                .wrapping_mul(3 as usize)
                .wrapping_mul(3 as usize)
                .wrapping_div((1024 as i32 * 1024 as i32) as usize);

            errorBelch(
                c"the current resource limit for virtual memory ('ulimit -v' or RLIMIT_AS) is too low.\nPlease make sure that at least %zuMiB of virtual memory are available."
                    .as_ptr(),
                needed,
            );

            stg_exit(EXIT_FAILURE);
        }
    }

    let MAX_ATTEMPTS = 256;
    let vla = MAX_ATTEMPTS as usize;

    let mut bad_allocs: Vec<*mut c_void> = ::std::vec::from_elem(null_mut::<c_void>(), vla);

    let vla_0 = MAX_ATTEMPTS as usize;
    let mut bad_alloc_lens: Vec<usize> = ::std::vec::from_elem(0, vla_0);

    memset(
        bad_allocs.as_mut_ptr() as *mut c_void,
        0,
        (size_of::<*mut c_void>() as usize).wrapping_mul(MAX_ATTEMPTS as usize),
    );

    memset(
        bad_alloc_lens.as_mut_ptr() as *mut c_void,
        0,
        (size_of::<usize>() as usize).wrapping_mul(MAX_ATTEMPTS as usize),
    );

    attempt = 0;

    while attempt < MAX_ATTEMPTS {
        *len &= !MBLOCK_MASK as W_;

        if *len < MBLOCK_SIZE as W_ {
            barf(c"osReserveHeapMemory: Failed to allocate heap storage".as_ptr());
        }

        let mut hint = startAddress.wrapping_add((attempt as u64).wrapping_mul(BLOCK_SIZE) as W_)
            as *mut c_void;
        at = osTryReserveHeapMemory(*len, hint);

        if at.is_null() {
            *len = (*len).wrapping_sub((*len).wrapping_div(8 as W_));
        } else {
            if at as W_ >= minimumAddress {
                break;
            }

            let mut end: usize = (at as W_).wrapping_add(*len) as usize;
            let ref mut fresh5 = *bad_allocs.as_mut_ptr().offset(attempt as isize);
            *fresh5 = at;

            if end as W_ > minimumAddress {
                if munmap(
                    minimumAddress as *mut c_void,
                    (end as W_).wrapping_sub(minimumAddress) as usize,
                ) < 0
                {
                    sysErrorBelch(
                        c"unable to release high portion of low memory reservation".as_ptr(),
                    );
                }

                *bad_alloc_lens.as_mut_ptr().offset(attempt as isize) =
                    minimumAddress.wrapping_sub(at as W_) as usize;
            } else {
                *bad_alloc_lens.as_mut_ptr().offset(attempt as isize) = *len as usize;
            }
        }

        attempt += 1;
    }

    let mut i = 0;

    while i < MAX_ATTEMPTS {
        if !(*bad_allocs.as_mut_ptr().offset(i as isize)).is_null()
            && munmap(
                *bad_allocs.as_mut_ptr().offset(i as isize),
                *bad_alloc_lens.as_mut_ptr().offset(i as isize),
            ) < 0
        {
            sysErrorBelch(c"unable to release reserved heap".as_ptr());
        }

        i += 1;
    }

    if at.is_null() {
        sysErrorBelch(c"failed to reserve heap memory".as_ptr());
    }

    return at;
}

unsafe fn osCommitMemory(mut at: *mut c_void, mut size: W_) {
    let mut r = my_mmap(at, size, MEM_COMMIT as i32);

    if r.is_null() {
        errorBelch(c"Unable to commit %llu bytes of memory".as_ptr(), size);
        errorBelch(c"Exiting. The system might be out of memory.".as_ptr());
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn osDecommitMemory(mut at: *mut c_void, mut size: W_) {
    let mut r: i32 = 0;
    r = mprotect(at, size as usize, PROT_NONE);

    if r < 0 {
        sysErrorBelch(c"unable to make released memory unaccessible".as_ptr());
    }

    if !RtsFlags.MiscFlags.disableDelayedOsMemoryReturn {
        r = madvise(at, size as usize, MADV_FREE);

        if r < 0 {
            if !(*__error() == EINVAL) {
                sysErrorBelch(c"unable to decommit memory".as_ptr());
            }
        } else {
            return;
        }
    }

    r = madvise(at, size as usize, MADV_DONTNEED);

    if r < 0 {
        sysErrorBelch(c"unable to decommit memory".as_ptr());
    }
}

unsafe fn osReleaseHeapMemory() {
    let mut r: i32 = 0;

    r = munmap(
        mblock_address_space.0.begin as *mut c_void,
        mblock_address_space
            .0
            .end
            .wrapping_sub(mblock_address_space.0.begin) as usize,
    );

    if r < 0 {
        sysErrorBelch(c"unable to release address space".as_ptr());
    }
}

unsafe fn osBuiltWithNumaSupport() -> bool {
    return false;
}

unsafe fn osNumaAvailable() -> bool {
    return false;
}

unsafe fn osNumaNodes() -> u32 {
    return 1;
}

unsafe fn osNumaMask() -> u64 {
    return 1;
}
