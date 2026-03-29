use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::stg::misc_closures::stg_upd_frame_info;
use crate::ffi::stg::types::StgWord;
use crate::linker::m_map::{MEM_READ_WRITE_THEN_READ_EXECUTE, MemoryAccess};
use crate::prelude::*;
use crate::report_memory_map::reportMemoryMap;
use crate::sm::os_mem::roundUpToPage;

pub(crate) type MemoryAccess = u32;

pub(crate) const MEM_READ_WRITE_EXECUTE: MemoryAccess = 5;

pub(crate) const MEM_READ_EXECUTE: MemoryAccess = 4;

pub(crate) const MEM_READ_WRITE_THEN_READ_EXECUTE: MemoryAccess = 3;

pub(crate) const MEM_READ_WRITE: MemoryAccess = 2;

pub(crate) const MEM_READ_ONLY: MemoryAccess = 1;

pub(crate) const MEM_NO_ACCESS: MemoryAccess = 0;

/// cbindgen:no-export
struct MemoryRegion {
    start: *mut c_void,
    end: *mut c_void,
    last: *mut c_void,
}

const TRY_MAP_32BIT: i32 = MAP_32BIT;

static mut mmap_32bit_base: *mut c_void = unsafe { &raw const stg_upd_frame_info as *mut c_void };

unsafe fn initLinkerMMap() {
    if RtsFlags.MiscFlags.linkerMemBase != 0 {
        mmap_32bit_base = RtsFlags.MiscFlags.linkerMemBase as *mut c_void;
    }
}

unsafe fn memoryAccessDescription(mut mode: MemoryAccess) -> *const c_char {
    match mode as u32 {
        0 => return c"no-access".as_ptr(),
        1 => return c"read-only".as_ptr(),
        2 => return c"read-write".as_ptr(),
        3 => {
            return c"read-write-then-read-execute".as_ptr();
        }
        4 => return c"read-execute".as_ptr(),
        5 => return c"read-write-execute".as_ptr(),
        _ => {
            barf(c"invalid MemoryAccess".as_ptr());
        }
    };
}

unsafe fn memoryAccessToProt(mut access: MemoryAccess) -> i32 {
    match access as u32 {
        0 => return 0,
        1 => return PROT_READ,
        2 => return PROT_READ | PROT_WRITE,
        3 => return PROT_READ | PROT_WRITE,
        4 => return PROT_READ | PROT_EXEC,
        5 => return PROT_READ | PROT_WRITE | PROT_EXEC,
        _ => {
            barf(c"invalid MemoryAccess".as_ptr());
        }
    };
}

unsafe fn doMmap(
    mut map_addr: *mut c_void,
    mut bytes: usize,
    mut prot: i32,
    mut flags: u32,
    mut fd: i32,
    mut offset: i32,
) -> *mut c_void {
    flags |= MAP_PRIVATE as u32;

    let mut result = mmap(map_addr, bytes, prot, flags as i32, fd, offset as off_t);

    if result == MAP_FAILED {
        sysErrorBelch(c"mmap %zx bytes at %p".as_ptr(), bytes, map_addr);
        reportMemoryMap();
        errorBelch(c"Try specifying an address with +RTS -xm<addr> -RTS".as_ptr());

        return NULL;
    }

    return result;
}

unsafe fn nearImage() -> *mut MemoryRegion {
    static mut region: MemoryRegion = MemoryRegion {
        start: NULL,
        end: NULL,
        last: NULL,
    };

    if region.end.is_null() {
        region.start = mmap_32bit_base;
        region.end = (region.start as *mut u8).offset(0x80000000) as *mut c_void;
        region.last = region.start;
    }

    return &raw mut region;
}

unsafe fn mmapAnywhere(
    mut bytes: usize,
    mut access: MemoryAccess,
    mut flags: u32,
    mut fd: i32,
    mut offset: i32,
) -> *mut c_void {
    let mut prot = memoryAccessToProt(access);

    return doMmap(NULL, bytes, prot, flags, fd, offset);
}

unsafe fn mmapInRegion(
    mut region: *mut MemoryRegion,
    mut bytes: usize,
    mut access: MemoryAccess,
    mut flags: u32,
    mut fd: i32,
    mut offset: i32,
) -> *mut c_void {
    let mut wrapped = false;
    let mut prot = memoryAccessToProt(access);
    let mut p = (*region).last;

    loop {
        let mut result = doMmap(p, bytes, prot, flags, fd, offset);

        if result.is_null() {
            return NULL;
        } else if result >= (*region).start && result < (*region).end {
            (*region).last = (result as *mut u8).offset(bytes as isize) as *mut c_void;

            return result;
        } else if wrapped {
            munmap(result, bytes);
            reportMemoryMap();

            errorBelch(
                c"mmapForLinker: failed to mmap() memory between %p and %p; asked for %zu bytes at %p. Try specifying an address with +RTS -xm<addr> -RTS"
                    .as_ptr(),
                (*region).start,
                (*region).end,
                bytes,
                p,
            );

            return NULL;
        } else if result < (*region).start {
            p = (p as *mut u8).offset(bytes as isize) as *mut c_void;
        } else if result >= (*region).end {
            wrapped = true;
            p = (*region).start;
        }

        munmap(result, bytes);
    }
}

unsafe fn mmapForLinker(
    mut bytes: usize,
    mut access: MemoryAccess,
    mut flags: u32,
    mut fd: i32,
    mut offset: i32,
) -> *mut c_void {
    bytes = roundUpToPage(bytes);

    let mut region = null_mut::<MemoryRegion>();

    if RtsFlags.MiscFlags.linkerAlwaysPic {
        region = null_mut::<MemoryRegion>();
    } else {
        region = nearImage();
    }

    if !region.is_null() && (*region).end <= 0xffffffff {
        flags |= TRY_MAP_32BIT as u32;
    }

    let mut result = null_mut::<c_void>();

    if !region.is_null() {
        result = mmapInRegion(region, bytes, access, flags, fd, offset);
    } else {
        result = mmapAnywhere(bytes, access, flags, fd, offset);
    }

    return result;
}

unsafe fn mmapAnon(mut bytes: usize) -> *mut c_void {
    return mmapAnywhere(
        bytes,
        MEM_READ_WRITE_THEN_READ_EXECUTE,
        MAP_ANONYMOUS as u32,
        -1,
        0,
    );
}

unsafe fn mmapAnonForLinker(mut bytes: usize) -> *mut c_void {
    return mmapForLinker(
        bytes,
        MEM_READ_WRITE_THEN_READ_EXECUTE,
        MAP_ANONYMOUS as u32,
        -1,
        0,
    );
}

unsafe fn munmapForLinker(mut addr: *mut c_void, mut bytes: usize, mut caller: *const c_char) {
    let mut r = munmap(addr, bytes);

    if r == -1 {
        sysErrorBelch(c"munmap: %s".as_ptr(), caller);
    }
}

unsafe fn mprotectForLinker(mut start: *mut c_void, mut len: usize, mut mode: MemoryAccess) {
    if len == 0 {
        return;
    }

    let mut prot = memoryAccessToProt(mode);

    if mprotect(start, len, prot) == -1 {
        sysErrorBelch(
            c"mprotectForLinker: failed to protect %zd bytes at %p as %s".as_ptr(),
            len,
            start,
            memoryAccessDescription(mode),
        );
    }
}
