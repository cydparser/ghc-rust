use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::storage::block::{BLOCK_SIZE, MBLOCK_MASK, MBLOCK_SIZE};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::m_block::{getFirstMBlock, getNextMBlock};
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, stg_exit};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord, StgWord8, StgWord64};
use crate::prelude::*;
use crate::sm::os_mem::roundUpToAlign;

#[inline]
pub(crate) unsafe fn roundDownToPage(mut x: size_t) -> size_t {
    let mut size = getPageSize();

    return x & !size.wrapping_sub(1 as size_t);
}

#[inline]
pub(crate) unsafe fn roundUpToAlign(mut size: size_t, mut align: size_t) -> size_t {
    return size.wrapping_add(align).wrapping_sub(1 as size_t) & !align.wrapping_sub(1 as size_t);
}

#[inline]
pub(crate) unsafe fn roundUpToPage(mut x: size_t) -> size_t {
    return roundUpToAlign(x, getPageSize());
}

const MEM_RESERVE_AND_COMMIT: C2RustUnnamed_7 = 3;

const MEM_COMMIT: C2RustUnnamed_7 = 2;

const MEM_RESERVE: C2RustUnnamed_7 = 1;

type C2RustUnnamed_7 = c_uint;

static mut next_request: *mut c_void = null::<c_void>() as *mut c_void;

unsafe fn osMemInit() {
    next_request = RtsFlags.GcFlags.heapBase as *mut c_void;
}

unsafe fn post_mmap_madvise(mut operation: c_int, mut size: W_, mut ret: *mut c_void) {
    if operation & MEM_COMMIT as c_int != 0 {
        madvise(ret, size as size_t, MADV_WILLNEED);
    } else {
        madvise(ret, size as size_t, MADV_DONTNEED);
    };
}

unsafe fn my_mmap(mut addr: *mut c_void, mut size: W_, mut operation: c_int) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    let mut err = 0 as kern_return_t;
    ret = addr;

    if operation & MEM_RESERVE as c_int != 0 {
        if !addr.is_null() {
            err = vm_allocate(
                mach_task_self_ as vm_map_t,
                &raw mut ret as *mut vm_address_t,
                size as vm_size_t,
                r#false,
            );
        }

        if addr.is_null() || err != 0 {
            err = vm_allocate(
                mach_task_self_ as vm_map_t,
                &raw mut ret as *mut vm_address_t,
                size as vm_size_t,
                r#true,
            );
        }
    }

    if err != 0 {
        errorBelch(
            b"memory allocation failed (requested %llu bytes)\0" as *const u8 as *const c_char,
            size,
        );

        stg_exit(EXIT_FAILURE);
    }

    if operation & MEM_COMMIT as c_int != 0 {
        vm_protect(
            mach_task_self_ as vm_map_t,
            ret as vm_address_t,
            size as vm_size_t,
            r#false,
            VM_PROT_READ | VM_PROT_WRITE,
        );
    }

    post_mmap_madvise(operation, size, ret);

    return ret;
}

unsafe fn my_mmap_or_barf(
    mut addr: *mut c_void,
    mut size: W_,
    mut operation: c_int,
) -> *mut c_void {
    let mut ret = my_mmap(addr, size, operation);

    if ret.is_null() {
        if *__error() == ENOMEM
            || *__error() == EINVAL
                && size_of::<*mut c_void>() as usize == 4 as usize
                && size >= 0xc0000000 as W_
        {
            errorBelch(
                b"out of memory (requested %llu bytes)\0" as *const u8 as *const c_char,
                size,
            );

            stg_exit(EXIT_HEAPOVERFLOW);
        } else {
            barf(
                b"getMBlock: mmap: %s\0" as *const u8 as *const c_char,
                strerror(*__error()),
            );
        }
    }

    return ret;
}

unsafe fn gen_map_mblocks(mut size: W_) -> *mut c_void {
    let mut slop: c_int = 0;
    let mut ret = null_mut::<StgWord8>();
    size = size.wrapping_add(MBLOCK_SIZE as W_);
    ret = my_mmap_or_barf(null_mut::<c_void>(), size, MEM_RESERVE_AND_COMMIT as c_int)
        as *mut StgWord8;
    slop = (ret as W_ & MBLOCK_MASK as W_) as c_int;

    if munmap(
        ret as *mut c_void,
        (MBLOCK_SIZE as size_t).wrapping_sub(slop as size_t),
    ) == -(1 as c_int)
    {
        barf(b"gen_map_mblocks: munmap failed\0" as *const u8 as *const c_char);
    }

    if slop > 0 as c_int
        && munmap(
            ret.offset(size as isize).offset(-(slop as isize)) as *mut c_void,
            slop as size_t,
        ) == -(1 as c_int)
    {
        barf(b"gen_map_mblocks: munmap failed\0" as *const u8 as *const c_char);
    }

    ret = ret.offset(MBLOCK_SIZE.wrapping_sub(slop as c_ulong) as isize);

    return ret as *mut c_void;
}

unsafe fn osGetMBlocks(mut n: uint32_t) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    let mut size: W_ = (MBLOCK_SIZE as W_).wrapping_mul(n as W_);

    if next_request.is_null() {
        ret = gen_map_mblocks(size);
    } else {
        ret = my_mmap_or_barf(next_request, size, MEM_RESERVE_AND_COMMIT as c_int);

        if ret as W_ & MBLOCK_MASK as W_ != 0 as W_ {
            if munmap(ret, size as size_t) == -(1 as c_int) {
                barf(b"getMBlock: munmap failed\0" as *const u8 as *const c_char);
            }

            ret = gen_map_mblocks(size);
        }
    }

    next_request = (ret as *mut c_char).offset(size as isize) as *mut c_void;

    return ret;
}

unsafe fn osBindMBlocksToNode(mut addr: *mut c_void, mut size: StgWord, mut node: uint32_t) {}

unsafe fn osFreeMBlocks(mut addr: *mut c_void, mut n: uint32_t) {
    munmap(addr, (n as size_t).wrapping_mul(MBLOCK_SIZE as size_t));
}

unsafe fn osReleaseFreeMemory() {}

unsafe fn osFreeAllMBlocks() {
    let mut mblock = null_mut::<c_void>();
    let mut state = null_mut::<c_void>();
    mblock = getFirstMBlock(&raw mut state);

    while !mblock.is_null() {
        munmap(mblock, MBLOCK_SIZE as size_t);
        mblock = getNextMBlock(&raw mut state, mblock);
    }
}

unsafe fn getPageSize() -> size_t {
    static mut pageSize: size_t = 0 as size_t;

    if pageSize == 0 as size_t {
        let mut ret: c_long = 0;
        ret = sysconf(_SC_PAGESIZE);

        if ret == -(1 as c_int) as c_long {
            barf(b"getPageSize: cannot get page size\0" as *const u8 as *const c_char);
        }

        pageSize = ret as size_t;
    }

    return pageSize;
}

unsafe fn getPhysicalMemorySize() -> StgWord64 {
    static mut physMemSize: StgWord64 = 0 as StgWord64;

    if physMemSize == 0 {
        let mut len: size_t = size_of::<StgWord64>() as size_t;
        let mut ret = -(1 as c_int);

        ret = sysctlbyname(
            b"hw.memsize\0" as *const u8 as *const c_char,
            &raw mut physMemSize as *mut c_void,
            &raw mut len,
            NULL,
            0 as size_t,
        );

        if ret == -(1 as c_int) {
            physMemSize = 0 as StgWord64;

            return 0 as StgWord64;
        }
    }

    return physMemSize;
}

unsafe fn osTryReserveHeapMemory(mut len: W_, mut hint: *mut c_void) -> *mut c_void {
    let mut base = null_mut::<c_void>();
    let mut top = null_mut::<c_void>();
    let mut start = null_mut::<c_void>();
    let mut end = null_mut::<c_void>();

    base = my_mmap(
        hint,
        len.wrapping_add(MBLOCK_SIZE as W_),
        MEM_RESERVE as c_int,
    );

    if base.is_null() {
        return NULL;
    }

    top = (base as W_)
        .wrapping_add(len)
        .wrapping_add(MBLOCK_SIZE as W_) as *mut c_void;

    if base as W_ & MBLOCK_MASK as W_ != 0 as W_ {
        start = ((base as W_)
            .wrapping_add(MBLOCK_SIZE as W_)
            .wrapping_sub(1 as W_)
            & !MBLOCK_MASK as W_) as *mut c_void;
        end = (top as W_ & !MBLOCK_MASK as W_) as *mut c_void;

        if munmap(base, (start as W_).wrapping_sub(base as W_) as size_t) < 0 as c_int {
            sysErrorBelch(b"unable to release slop before heap\0" as *const u8 as *const c_char);
        }

        if munmap(end, (top as W_).wrapping_sub(end as W_) as size_t) < 0 as c_int {
            sysErrorBelch(b"unable to release slop after heap\0" as *const u8 as *const c_char);
        }
    } else {
        start = base;
    }

    return start;
}

unsafe fn osReserveHeapMemory(mut startAddressPtr: *mut c_void, mut len: *mut W_) -> *mut c_void {
    let mut attempt: c_int = 0;
    let mut at = null_mut::<c_void>();
    let mut minimumAddress: W_ =
        (8 as c_int as W_).wrapping_mul(((1 as c_int) << 30 as c_int) as W_);

    let mut startAddress: W_ = 0x4200000000 as W_;

    if !startAddressPtr.is_null() {
        startAddress = startAddressPtr as W_;
    }

    if startAddress < minimumAddress {
        errorBelch(
            b"Provided heap start address %p is lower than minimum address %p\0" as *const u8
                as *const c_char,
            startAddress as *mut c_void,
            minimumAddress as *mut c_void,
        );
    }

    let mut asLimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    if getrlimit(RLIMIT_AS, &raw mut asLimit) == 0
        && asLimit.rlim_cur > 0 as rlim_t
        && *len > asLimit.rlim_cur
    {
        let mut threadAttr = _opaque_pthread_attr_t {
            __sig: 0,
            __opaque: [0; 56],
        };

        if pthread_attr_init(&raw mut threadAttr) != 0 {
            sysErrorBelch(
                b"failed to initialize thread attributes\0" as *const u8 as *const c_char,
            );

            stg_exit(EXIT_FAILURE);
        }

        let mut stacksz: size_t = 0 as size_t;

        if pthread_attr_getstacksize(&raw mut threadAttr, &raw mut stacksz) != 0 {
            sysErrorBelch(
                b"failed to read default thread stack size\0" as *const u8 as *const c_char,
            );

            stg_exit(EXIT_FAILURE);
        }

        if pthread_attr_destroy(&raw mut threadAttr) != 0 {
            sysErrorBelch(b"failed to destroy thread attributes\0" as *const u8 as *const c_char);
            stg_exit(EXIT_FAILURE);
        }

        let mut pageSize = getPageSize();
        *len = (asLimit.rlim_cur as c_double * 0.666f64) as W_
            & !pageSize.wrapping_sub(1 as size_t) as W_;

        if asLimit.rlim_cur.wrapping_sub(*len) < stacksz.wrapping_mul(3 as size_t) as W_ {
            let mut needed: size_t = stacksz
                .wrapping_mul(3 as size_t)
                .wrapping_mul(3 as size_t)
                .wrapping_div((1024 as c_int * 1024 as c_int) as size_t);

            errorBelch(
                b"the current resource limit for virtual memory ('ulimit -v' or RLIMIT_AS) is too low.\nPlease make sure that at least %zuMiB of virtual memory are available.\0"
                    as *const u8 as *const c_char,
                needed,
            );

            stg_exit(EXIT_FAILURE);
        }
    }

    let MAX_ATTEMPTS = 256 as c_int;
    let vla = MAX_ATTEMPTS as usize;
    let mut bad_allocs: Vec<*mut c_void> = ::std::vec::from_elem(null_mut::<c_void>(), vla);
    let vla_0 = MAX_ATTEMPTS as usize;
    let mut bad_alloc_lens: Vec<size_t> = ::std::vec::from_elem(0, vla_0);

    memset(
        bad_allocs.as_mut_ptr() as *mut c_void,
        0 as c_int,
        (size_of::<*mut c_void>() as size_t).wrapping_mul(MAX_ATTEMPTS as size_t),
    );

    memset(
        bad_alloc_lens.as_mut_ptr() as *mut c_void,
        0 as c_int,
        (size_of::<size_t>() as size_t).wrapping_mul(MAX_ATTEMPTS as size_t),
    );

    attempt = 0 as c_int;

    while attempt < MAX_ATTEMPTS {
        *len &= !MBLOCK_MASK as W_;

        if *len < MBLOCK_SIZE as W_ {
            barf(
                b"osReserveHeapMemory: Failed to allocate heap storage\0" as *const u8
                    as *const c_char,
            );
        }

        let mut hint = startAddress
            .wrapping_add((attempt as c_ulong).wrapping_mul(BLOCK_SIZE) as W_)
            as *mut c_void;
        at = osTryReserveHeapMemory(*len, hint);

        if at.is_null() {
            *len = (*len).wrapping_sub((*len).wrapping_div(8 as W_));
        } else {
            if at as W_ >= minimumAddress {
                break;
            }

            let mut end: uintptr_t = (at as W_).wrapping_add(*len) as uintptr_t;
            let ref mut fresh5 = *bad_allocs.as_mut_ptr().offset(attempt as isize);
            *fresh5 = at;

            if end as W_ > minimumAddress {
                if munmap(
                    minimumAddress as *mut c_void,
                    (end as W_).wrapping_sub(minimumAddress) as size_t,
                ) < 0 as c_int
                {
                    sysErrorBelch(
                        b"unable to release high portion of low memory reservation\0" as *const u8
                            as *const c_char,
                    );
                }

                *bad_alloc_lens.as_mut_ptr().offset(attempt as isize) =
                    minimumAddress.wrapping_sub(at as W_) as size_t;
            } else {
                *bad_alloc_lens.as_mut_ptr().offset(attempt as isize) = *len as size_t;
            }
        }

        attempt += 1;
    }

    let mut i = 0 as c_int;

    while i < MAX_ATTEMPTS {
        if !(*bad_allocs.as_mut_ptr().offset(i as isize)).is_null()
            && munmap(
                *bad_allocs.as_mut_ptr().offset(i as isize),
                *bad_alloc_lens.as_mut_ptr().offset(i as isize),
            ) < 0 as c_int
        {
            sysErrorBelch(b"unable to release reserved heap\0" as *const u8 as *const c_char);
        }

        i += 1;
    }

    if at.is_null() {
        sysErrorBelch(b"failed to reserve heap memory\0" as *const u8 as *const c_char);
    }

    return at;
}

unsafe fn osCommitMemory(mut at: *mut c_void, mut size: W_) {
    let mut r = my_mmap(at, size, MEM_COMMIT as c_int);

    if r.is_null() {
        errorBelch(
            b"Unable to commit %llu bytes of memory\0" as *const u8 as *const c_char,
            size,
        );

        errorBelch(b"Exiting. The system might be out of memory.\0" as *const u8 as *const c_char);
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn osDecommitMemory(mut at: *mut c_void, mut size: W_) {
    let mut r: c_int = 0;

    if !RtsFlags.MiscFlags.disableDelayedOsMemoryReturn {
        r = madvise(at, size as size_t, MADV_FREE);

        if r < 0 as c_int {
            if !(*__error() == EINVAL) {
                sysErrorBelch(b"unable to decommit memory\0" as *const u8 as *const c_char);
            }
        } else {
            return;
        }
    }

    r = madvise(at, size as size_t, MADV_DONTNEED);

    if r < 0 as c_int {
        sysErrorBelch(b"unable to decommit memory\0" as *const u8 as *const c_char);
    }
}

unsafe fn osReleaseHeapMemory() {
    let mut r: c_int = 0;

    r = munmap(
        mblock_address_space.0.begin as *mut c_void,
        mblock_address_space
            .0
            .end
            .wrapping_sub(mblock_address_space.0.begin) as size_t,
    );

    if r < 0 as c_int {
        sysErrorBelch(b"unable to release address space\0" as *const u8 as *const c_char);
    }
}

unsafe fn osBuiltWithNumaSupport() -> bool {
    return r#false != 0;
}

unsafe fn osNumaAvailable() -> bool {
    return r#false != 0;
}

unsafe fn osNumaNodes() -> uint32_t {
    return 1 as uint32_t;
}

unsafe fn osNumaMask() -> uint64_t {
    return 1 as uint64_t;
}
