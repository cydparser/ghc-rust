use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::rts::storage::block::{MBLOCK_MASK, MBLOCK_SIZE};
use crate::ffi::rts::{EXIT_HEAPOVERFLOW, stg_exit};
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgWord, StgWord64};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgMallocBytes};
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

type block_rec = block_rec_;

/// cbindgen:no-export
struct block_rec_ {
    base: *mut c_char,
    size: W_,
    next: *mut block_rec_,
}

type alloc_rec = alloc_rec_;

/// cbindgen:no-export
struct alloc_rec_ {
    base: *mut c_char,
    size: W_,
    next: *mut alloc_rec_,
}

static mut allocs: *mut alloc_rec = null_mut::<alloc_rec>();

static mut free_blocks: *mut block_rec = null_mut::<block_rec>();

unsafe fn osMemInit() {
    allocs = null_mut::<alloc_rec>();
    free_blocks = null_mut::<block_rec>();
}

unsafe fn allocNew(mut n: u32) -> *mut alloc_rec {
    let mut rec = null_mut::<alloc_rec>();

    rec = stgMallocBytes(
        size_of::<alloc_rec>() as usize,
        c"getMBlocks: allocNew".as_ptr(),
    ) as *mut alloc_rec;

    (*rec).size = (n as W_)
        .wrapping_add(1 as W_)
        .wrapping_mul(MBLOCK_SIZE as W_);

    (*rec).base = VirtualAlloc(
        NULL,
        (*rec).size as SIZE_T,
        (MEM_RESERVE | MEM_TOP_DOWN) as DWORD,
        PAGE_READWRITE as DWORD,
    ) as *mut c_char;

    if (*rec).base.is_null() {
        stgFree(rec as *mut c_void);
        rec = null_mut::<alloc_rec>();

        if GetLastError() == 8 {
            errorBelch(c"Out of memory\n".as_ptr());
            stg_exit(EXIT_HEAPOVERFLOW);
        } else {
            sysErrorBelch(
                c"getMBlocks: VirtualAlloc MEM_RESERVE %d blocks failed".as_ptr(),
                n,
            );
        }
    } else {
        let mut temp = alloc_rec_ {
            base: null_mut::<c_char>(),
            size: 0,
            next: null_mut::<alloc_rec_>(),
        };

        temp.base = null_mut::<c_char>();
        temp.size = 0;
        temp.next = allocs as *mut alloc_rec_;

        let mut it = null_mut::<alloc_rec>();
        it = &raw mut temp;

        while !(*it).next.is_null() && (*(*it).next).base < (*rec).base {
            it = (*it).next as *mut alloc_rec;
        }

        (*rec).next = (*it).next;
        (*it).next = rec as *mut alloc_rec_;
        allocs = temp.next as *mut alloc_rec;
    }

    return rec;
}

unsafe fn insertFree(mut alloc_base: *mut c_char, mut alloc_size: W_) {
    let mut temp = block_rec_ {
        base: null_mut::<c_char>(),
        size: 0,
        next: null_mut::<block_rec_>(),
    };

    let mut it = null_mut::<block_rec>();
    let mut prev = null_mut::<block_rec>();
    temp.base = null_mut::<c_char>();
    temp.size = 0;
    temp.next = free_blocks as *mut block_rec_;
    it = free_blocks;
    prev = &raw mut temp;

    while !it.is_null() && (*it).base < alloc_base {
        prev = it;
        it = (*it).next as *mut block_rec;
    }

    if !it.is_null() && alloc_base.offset(alloc_size as isize) == (*it).base {
        if (*prev).base.offset((*prev).size as isize) == alloc_base {
            (*prev).size = (*prev)
                .size
                .wrapping_add(alloc_size.wrapping_add((*it).size));
            (*prev).next = (*it).next;
            stgFree(it as *mut c_void);
        } else {
            (*it).base = alloc_base;
            (*it).size = (*it).size.wrapping_add(alloc_size);
        }
    } else if (*prev).base.offset((*prev).size as isize) == alloc_base {
        (*prev).size = (*prev).size.wrapping_add(alloc_size);
    } else {
        let mut rec = null_mut::<block_rec>();

        rec = stgMallocBytes(
            size_of::<block_rec>() as usize,
            c"getMBlocks: insertFree".as_ptr(),
        ) as *mut block_rec;

        (*rec).base = alloc_base;
        (*rec).size = alloc_size;
        (*rec).next = it as *mut block_rec_;
        (*prev).next = rec as *mut block_rec_;
    }

    free_blocks = temp.next as *mut block_rec;
}

unsafe fn findFreeBlocks(mut n: u32) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    let mut it = null_mut::<block_rec>();

    let mut temp = block_rec_ {
        base: null_mut::<c_char>(),
        size: 0,
        next: null_mut::<block_rec_>(),
    };

    let mut prev = null_mut::<block_rec>();
    let mut required_size: W_ = 0;
    it = free_blocks;
    required_size = (n as u64).wrapping_mul(MBLOCK_SIZE) as W_;
    temp.next = free_blocks as *mut block_rec_;
    temp.base = null_mut::<c_char>();
    temp.size = 0;
    prev = &raw mut temp;

    while !it.is_null() {
        if !(it.is_null() || (*it).size < required_size) {
            if (*it).base as W_ & MBLOCK_MASK as W_ == 0 {
                ret = (*it).base as *mut c_void;

                if (*it).size == required_size {
                    (*prev).next = (*it).next;
                    stgFree(it as *mut c_void);
                } else {
                    (*it).base = (*it).base.offset(required_size as isize);
                    (*it).size = (*it).size.wrapping_sub(required_size);
                }

                break;
            } else {
                let mut need_base = null_mut::<c_char>();
                let mut next = null_mut::<block_rec>();
                let mut new_size: i32 = 0;
                need_base = (((*it).base as W_ & !MBLOCK_MASK as W_) as *mut c_char)
                    .offset(MBLOCK_SIZE as isize);
                new_size = need_base.offset_from((*it).base) as i64 as i32;

                let mut total_size: W_ = (new_size as W_).wrapping_add(required_size);

                if !(total_size > (*it).size) {
                    next = stgMallocBytes(
                        size_of::<block_rec>() as usize,
                        c"getMBlocks: findFreeBlocks: splitting".as_ptr(),
                    ) as *mut block_rec;

                    (*next).base = need_base.offset(required_size as isize);
                    (*next).size = (*it).size.wrapping_sub(total_size);
                    (*it).size = new_size as W_;
                    (*next).next = (*it).next;
                    (*it).next = next as *mut block_rec_;
                    ret = need_base as *mut c_void;
                    break;
                }
            }
        }

        prev = it;
        it = (*it).next as *mut block_rec;
    }

    free_blocks = temp.next as *mut block_rec;

    return ret;
}

unsafe fn commitBlocks(mut base: *mut c_char, mut size: W_) {
    let mut it = null_mut::<alloc_rec>();
    it = allocs;

    while !it.is_null() && (*it).base.offset((*it).size as isize) <= base {
        it = (*it).next as *mut alloc_rec;
    }

    while !it.is_null() && size > 0 {
        let mut size_delta: W_ = 0;
        let mut temp = null_mut::<c_void>();
        size_delta = (*it)
            .size
            .wrapping_sub(base.offset_from((*it).base) as i64 as W_);

        if size_delta > size {
            size_delta = size;
        }

        temp = VirtualAlloc(
            base as LPVOID,
            size_delta as SIZE_T,
            MEM_COMMIT as DWORD,
            PAGE_READWRITE as DWORD,
        ) as *mut c_void;

        if temp.is_null() {
            sysErrorBelch(c"getMBlocks: VirtualAlloc MEM_COMMIT failed".as_ptr());
            stg_exit(EXIT_HEAPOVERFLOW);
        }

        size = size.wrapping_sub(size_delta);
        base = base.offset(size_delta as isize);
        it = (*it).next as *mut alloc_rec;
    }
}

unsafe fn osGetMBlocks(mut n: u32) -> *mut c_void {
    let mut ret = null_mut::<c_void>();
    ret = findFreeBlocks(n);

    if ret.is_null() {
        let mut alloc = null_mut::<alloc_rec>();
        alloc = allocNew(n);

        if alloc.is_null() {
            stg_exit(EXIT_FAILURE);
        } else {
            insertFree((*alloc).base, (*alloc).size);
            ret = findFreeBlocks(n);
        }
    }

    if !ret.is_null() {
        if ret as W_ & MBLOCK_MASK as W_ != 0 {
            barf(c"getMBlocks: misaligned block returned".as_ptr());
        }

        commitBlocks(
            ret as *mut c_char,
            (MBLOCK_SIZE as W_).wrapping_mul(n as W_),
        );
    }

    return ret;
}

unsafe fn decommitBlocks(mut addr: *mut c_char, mut nBytes: W_) {
    let mut p = null_mut::<alloc_rec>();
    p = allocs;

    while !p.is_null() && addr >= (*p).base.offset((*p).size as isize) {
        p = (*p).next as *mut alloc_rec;
    }

    while nBytes > 0 {
        if p.is_null() || (*p).base > addr {
            errorBelch(c"Memory to be freed isn't allocated\n".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        if (*p).base.offset((*p).size as isize) >= addr.offset(nBytes as isize) {
            if VirtualFree(addr as LPVOID, nBytes as SIZE_T, MEM_DECOMMIT as DWORD) == 0 {
                sysErrorBelch(c"osFreeMBlocks: VirtualFree MEM_DECOMMIT failed".as_ptr());

                stg_exit(EXIT_FAILURE);
            }

            nBytes = 0;
        } else {
            let mut bytesToFree: W_ =
                (*p).base.offset((*p).size as isize).offset_from(addr) as i64 as W_;

            if VirtualFree(addr as LPVOID, bytesToFree as SIZE_T, MEM_DECOMMIT as DWORD) == 0 {
                sysErrorBelch(c"osFreeMBlocks: VirtualFree MEM_DECOMMIT failed".as_ptr());

                stg_exit(EXIT_FAILURE);
            }

            addr = addr.offset(bytesToFree as isize);
            nBytes = nBytes.wrapping_sub(bytesToFree);
            p = (*p).next as *mut alloc_rec;
        }
    }
}

unsafe fn osFreeMBlocks(mut addr: *mut c_void, mut n: u32) {
    let mut nBytes: W_ = (n as W_).wrapping_mul(MBLOCK_SIZE as W_);
    insertFree(addr as *mut c_char, nBytes);
    decommitBlocks(addr as *mut c_char, nBytes);
}

unsafe fn osReleaseFreeMemory() {
    let mut prev_a = null_mut::<alloc_rec>();
    let mut a = null_mut::<alloc_rec>();

    let mut head_a = alloc_rec_ {
        base: null_mut::<c_char>(),
        size: 0,
        next: null_mut::<alloc_rec_>(),
    };

    let mut prev_fb = null_mut::<block_rec>();
    let mut fb = null_mut::<block_rec>();

    let mut head_fb = block_rec_ {
        base: null_mut::<c_char>(),
        size: 0,
        next: null_mut::<block_rec_>(),
    };

    let mut a_end = null_mut::<c_char>();
    let mut fb_end = null_mut::<c_char>();
    head_a.base = null_mut::<c_char>();
    head_a.size = 0;
    head_a.next = allocs as *mut alloc_rec_;
    head_fb.base = null_mut::<c_char>();
    head_fb.size = 0;
    head_fb.next = free_blocks as *mut block_rec_;
    prev_a = &raw mut head_a;
    a = allocs;
    prev_fb = &raw mut head_fb;
    fb = free_blocks;

    while !a.is_null() {
        a_end = (*a).base.offset((*a).size as isize);

        while !fb.is_null() && (*fb).base.offset((*fb).size as isize) < a_end {
            prev_fb = fb;
            fb = (*fb).next as *mut block_rec;
        }

        if fb.is_null() {
            break;
        }

        fb_end = (*fb).base.offset((*fb).size as isize);

        if (*fb).base <= (*a).base {
            if fb_end == a_end {
                if (*fb).base == (*a).base {
                    (*prev_fb).next = (*fb).next;
                    stgFree(fb as *mut c_void);
                    fb = (*prev_fb).next as *mut block_rec;
                } else {
                    (*fb).size = (*a).base.offset_from((*fb).base) as i64 as W_;
                }
            } else {
                if (*fb).base != (*a).base {
                    let mut new_fb = null_mut::<block_rec>();

                    new_fb = stgMallocBytes(
                        size_of::<block_rec>() as usize,
                        c"osReleaseFreeMemory".as_ptr(),
                    ) as *mut block_rec;

                    (*new_fb).base = (*fb).base;
                    (*new_fb).size = (*a).base.offset_from((*fb).base) as i64 as W_;
                    (*new_fb).next = fb as *mut block_rec_;
                    (*prev_fb).next = new_fb as *mut block_rec_;
                }

                (*fb).size = fb_end.offset_from(a_end) as i64 as W_;
                (*fb).base = a_end;
            }

            (*prev_a).next = (*a).next;

            if VirtualFree((*a).base as LPVOID, 0, MEM_RELEASE as DWORD) == 0 {
                sysErrorBelch(c"freeAllMBlocks: VirtualFree MEM_RELEASE failed".as_ptr());

                stg_exit(EXIT_FAILURE);
            }

            stgFree(a as *mut c_void);
            a = (*prev_a).next as *mut alloc_rec;
        } else {
            prev_a = a;
            a = (*a).next as *mut alloc_rec;
        }
    }

    allocs = head_a.next as *mut alloc_rec;
    free_blocks = head_fb.next as *mut block_rec;
}

unsafe fn osFreeAllMBlocks() {
    let mut next = null_mut::<block_rec>();
    let mut it = null_mut::<block_rec>();
    next = null_mut::<block_rec>();
    it = free_blocks;

    while !it.is_null() {
        next = (*it).next as *mut block_rec;
        stgFree(it as *mut c_void);
        it = next;
    }

    let mut next_0 = null_mut::<alloc_rec>();
    let mut it_0 = null_mut::<alloc_rec>();
    next_0 = null_mut::<alloc_rec>();
    it_0 = allocs;

    while !it_0.is_null() {
        if VirtualFree((*it_0).base as LPVOID, 0, MEM_RELEASE as DWORD) == 0 {
            sysErrorBelch(c"freeAllMBlocks: VirtualFree MEM_RELEASE failed".as_ptr());
            stg_exit(EXIT_FAILURE);
        }

        next_0 = (*it_0).next as *mut alloc_rec;
        stgFree(it_0 as *mut c_void);
        it_0 = next_0;
    }
}

unsafe fn getPageSize() -> usize {
    static mut pagesize: usize = 0;

    if pagesize == 0 {
        let mut sSysInfo = _SYSTEM_INFO {
            c2rust_unnamed: C2RustUnnamed_9 { dwOemId: 0 },
            dwPageSize: 0,
            lpMinimumApplicationAddress: null_mut::<c_void>(),
            lpMaximumApplicationAddress: null_mut::<c_void>(),
            dwActiveProcessorMask: 0,
            dwNumberOfProcessors: 0,
            dwProcessorType: 0,
            dwAllocationGranularity: 0,
            wProcessorLevel: 0,
            wProcessorRevision: 0,
        };

        GetSystemInfo(&raw mut sSysInfo);
        pagesize = sSysInfo.dwPageSize as usize;
    }

    return pagesize;
}

unsafe fn getPhysicalMemorySize() -> StgWord64 {
    static mut physMemSize: StgWord64 = 0;

    if physMemSize == 0 {
        let mut status = _MEMORYSTATUSEX {
            dwLength: 0,
            dwMemoryLoad: 0,
            ullTotalPhys: 0,
            ullAvailPhys: 0,
            ullTotalPageFile: 0,
            ullAvailPageFile: 0,
            ullTotalVirtual: 0,
            ullAvailVirtual: 0,
            ullAvailExtendedVirtual: 0,
        };

        status.dwLength = size_of::<MEMORYSTATUSEX>() as DWORD;

        if GlobalMemoryStatusEx(&raw mut status) == 0 {
            errorBelch(c"warning: getPhysicalMemorySize: cannot get physical memory size".as_ptr());

            return 0;
        }

        physMemSize = status.ullTotalPhys as StgWord64;
    }

    return physMemSize;
}

static mut heap_base: *mut c_void = NULL;

unsafe fn osReserveHeapMemory(mut startAddress: *mut c_void, mut len: *mut W_) -> *mut c_void {
    let mut start = null_mut::<c_void>();

    heap_base = VirtualAlloc(
        startAddress as LPVOID,
        (*len).wrapping_add(MBLOCK_SIZE as W_) as SIZE_T,
        (MEM_RESERVE | MEM_TOP_DOWN) as DWORD,
        PAGE_READWRITE as DWORD,
    ) as *mut c_void;

    if heap_base.is_null() {
        if GetLastError() == 8 {
            errorBelch(c"out of memory".as_ptr());
        } else {
            sysErrorBelch(
                c"osReserveHeapMemory: VirtualAlloc MEM_RESERVE %llu bytes                 at address %p bytes failed"
                    .as_ptr(),
                (*len).wrapping_add(MBLOCK_SIZE as W_),
                startAddress,
            );
        }

        stg_exit(EXIT_FAILURE);
    }

    start = ((heap_base as W_)
        .wrapping_add(MBLOCK_SIZE as W_)
        .wrapping_sub(1 as W_)
        & !MBLOCK_MASK as W_) as *mut c_void;

    return start;
}

unsafe fn osCommitMemory(mut at: *mut c_void, mut size: W_) {
    let mut temp = null_mut::<c_void>();

    temp = VirtualAlloc(
        at as LPVOID,
        size as SIZE_T,
        MEM_COMMIT as DWORD,
        PAGE_READWRITE as DWORD,
    ) as *mut c_void;

    if temp.is_null() {
        sysErrorBelch(
            c"osCommitMemory: VirtualAlloc MEM_COMMIT failed to commit %llu bytes of memory  (error code: %lu)"
                .as_ptr(),
            size,
            GetLastError(),
        );

        stg_exit(EXIT_HEAPOVERFLOW);
    }
}

unsafe fn osDecommitMemory(mut at: *mut c_void, mut size: W_) {
    if VirtualFree(at as LPVOID, size as SIZE_T, MEM_DECOMMIT as DWORD) == 0 {
        sysErrorBelch(c"osDecommitMemory: VirtualFree MEM_DECOMMIT failed".as_ptr());
        stg_exit(EXIT_FAILURE);
    }
}

unsafe fn osReleaseHeapMemory() {
    VirtualFree(heap_base as LPVOID, 0, MEM_RELEASE as DWORD);
}

unsafe fn osBuiltWithNumaSupport() -> bool {
    return true;
}

unsafe fn osNumaAvailable() -> bool {
    return osNumaNodes() > 1;
}

unsafe fn osNumaNodes() -> u32 {
    static mut numNumaNodes: ULONG = 0;

    if numNumaNodes == 0 {
        if GetNumaHighestNodeNumber(&raw mut numNumaNodes) != 0 {
            numNumaNodes = numNumaNodes.wrapping_add(1 as ULONG);
        } else {
            numNumaNodes = 1;
        }
    }

    return numNumaNodes as u32;
}

unsafe fn osNumaMask() -> u64 {
    if osNumaNodes() as usize > (size_of::<StgWord>() as usize).wrapping_mul(8 as usize) {
        barf(
            c"osNumaMask: too many NUMA nodes (%d)".as_ptr(),
            osNumaNodes(),
        );
    }

    return ((1 << osNumaNodes()) - 1) as u64;
}

unsafe fn osBindMBlocksToNode(mut addr: *mut c_void, mut size: StgWord, mut node: u32) {
    if osNumaAvailable() {
        let mut temp = null_mut::<c_void>();

        if RtsFlags.GcFlags.numa {
            temp = VirtualAllocExNuma(
                GetCurrentProcess(),
                NULL,
                size as SIZE_T,
                (MEM_RESERVE | MEM_COMMIT) as DWORD,
                PAGE_READWRITE as DWORD,
                node as DWORD,
            ) as *mut c_void;

            if temp.is_null() {
                if GetLastError() == 8 {
                    errorBelch(c"out of memory".as_ptr());
                } else {
                    sysErrorBelch(
                        c"osBindMBlocksToNode: VirtualAllocExNuma MEM_RESERVE %llu bytes at address %p bytes failed"
                            .as_ptr(),
                        size,
                        addr,
                    );
                }

                stg_exit(EXIT_FAILURE);
            }
        }
    }
}
