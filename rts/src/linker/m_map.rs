use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
use crate::ffi::stg::misc_closures::stg_upd_frame_info;
use crate::ffi::stg::types::StgWord;
use crate::linker::m_map::{MEM_READ_WRITE_THEN_READ_EXECUTE, MemoryAccess};
use crate::prelude::*;
use crate::report_memory_map::reportMemoryMap;
use crate::sm::os_mem::roundUpToPage;

pub(crate) type MemoryAccess = c_uint;

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

const TRY_MAP_32BIT: c_int = MAP_32BIT;

static mut mmap_32bit_base: *mut c_void = unsafe { &raw const stg_upd_frame_info as *mut c_void };

unsafe fn initLinkerMMap() {
    if RtsFlags.MiscFlags.linkerMemBase != 0 as StgWord {
        mmap_32bit_base = RtsFlags.MiscFlags.linkerMemBase as *mut c_void;
    }
}

unsafe fn memoryAccessDescription(mut mode: MemoryAccess) -> *const c_char {
    match mode as c_uint {
        0 => return b"no-access\0" as *const u8 as *const c_char,
        1 => return b"read-only\0" as *const u8 as *const c_char,
        2 => return b"read-write\0" as *const u8 as *const c_char,
        3 => {
            return b"read-write-then-read-execute\0" as *const u8 as *const c_char;
        }
        4 => return b"read-execute\0" as *const u8 as *const c_char,
        5 => return b"read-write-execute\0" as *const u8 as *const c_char,
        _ => {
            barf(b"invalid MemoryAccess\0" as *const u8 as *const c_char);
        }
    };
}

unsafe fn memoryAccessToProt(mut access: MemoryAccess) -> c_int {
    match access as c_uint {
        0 => return 0 as c_int,
        1 => return PROT_READ,
        2 => return PROT_READ | PROT_WRITE,
        3 => return PROT_READ | PROT_WRITE,
        4 => return PROT_READ | PROT_EXEC,
        5 => return PROT_READ | PROT_WRITE | PROT_EXEC,
        _ => {
            barf(b"invalid MemoryAccess\0" as *const u8 as *const c_char);
        }
    };
}

unsafe fn doMmap(
    mut map_addr: *mut c_void,
    mut bytes: size_t,
    mut prot: c_int,
    mut flags: uint32_t,
    mut fd: c_int,
    mut offset: c_int,
) -> *mut c_void {
    flags |= MAP_PRIVATE as uint32_t;

    let mut result = mmap(map_addr, bytes, prot, flags as c_int, fd, offset as off_t);

    if result == MAP_FAILED {
        sysErrorBelch(
            b"mmap %zx bytes at %p\0" as *const u8 as *const c_char,
            bytes,
            map_addr,
        );

        reportMemoryMap();

        errorBelch(
            b"Try specifying an address with +RTS -xm<addr> -RTS\0" as *const u8 as *const c_char,
        );

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
        region.end =
            (region.start as *mut uint8_t).offset(0x80000000 as c_uint as isize) as *mut c_void;
        region.last = region.start;
    }

    return &raw mut region;
}

unsafe fn mmapAnywhere(
    mut bytes: size_t,
    mut access: MemoryAccess,
    mut flags: uint32_t,
    mut fd: c_int,
    mut offset: c_int,
) -> *mut c_void {
    let mut prot = memoryAccessToProt(access);

    return doMmap(NULL, bytes, prot, flags, fd, offset);
}

unsafe fn mmapInRegion(
    mut region: *mut MemoryRegion,
    mut bytes: size_t,
    mut access: MemoryAccess,
    mut flags: uint32_t,
    mut fd: c_int,
    mut offset: c_int,
) -> *mut c_void {
    let mut wrapped = r#false != 0;
    let mut prot = memoryAccessToProt(access);
    let mut p = (*region).last;

    loop {
        let mut result = doMmap(p, bytes, prot, flags, fd, offset);

        if result.is_null() {
            return NULL;
        } else if result >= (*region).start && result < (*region).end {
            (*region).last = (result as *mut uint8_t).offset(bytes as isize) as *mut c_void;

            return result;
        } else if wrapped {
            munmap(result, bytes);
            reportMemoryMap();

            errorBelch(
                b"mmapForLinker: failed to mmap() memory between %p and %p; asked for %zu bytes at %p. Try specifying an address with +RTS -xm<addr> -RTS\0"
                    as *const u8 as *const c_char,
                (*region).start,
                (*region).end,
                bytes,
                p,
            );

            return NULL;
        } else if result < (*region).start {
            p = (p as *mut uint8_t).offset(bytes as isize) as *mut c_void;
        } else if result >= (*region).end {
            wrapped = r#true != 0;
            p = (*region).start;
        }

        munmap(result, bytes);
    }
}

unsafe fn mmapForLinker(
    mut bytes: size_t,
    mut access: MemoryAccess,
    mut flags: uint32_t,
    mut fd: c_int,
    mut offset: c_int,
) -> *mut c_void {
    bytes = roundUpToPage(bytes);

    let mut region = null_mut::<MemoryRegion>();

    if RtsFlags.MiscFlags.linkerAlwaysPic {
        region = null_mut::<MemoryRegion>();
    } else {
        region = nearImage();
    }

    if !region.is_null() && (*region).end <= 0xffffffff as c_uint as *mut c_void {
        flags |= TRY_MAP_32BIT as uint32_t;
    }

    let mut result = null_mut::<c_void>();

    if !region.is_null() {
        result = mmapInRegion(region, bytes, access, flags, fd, offset);
    } else {
        result = mmapAnywhere(bytes, access, flags, fd, offset);
    }

    return result;
}

unsafe fn mmapAnon(mut bytes: size_t) -> *mut c_void {
    return mmapAnywhere(
        bytes,
        MEM_READ_WRITE_THEN_READ_EXECUTE,
        MAP_ANONYMOUS as uint32_t,
        -(1 as c_int),
        0 as c_int,
    );
}

unsafe fn mmapAnonForLinker(mut bytes: size_t) -> *mut c_void {
    return mmapForLinker(
        bytes,
        MEM_READ_WRITE_THEN_READ_EXECUTE,
        MAP_ANONYMOUS as uint32_t,
        -(1 as c_int),
        0 as c_int,
    );
}

unsafe fn munmapForLinker(mut addr: *mut c_void, mut bytes: size_t, mut caller: *const c_char) {
    let mut r = munmap(addr, bytes);

    if r == -(1 as c_int) {
        sysErrorBelch(b"munmap: %s\0" as *const u8 as *const c_char, caller);
    }
}

unsafe fn mprotectForLinker(mut start: *mut c_void, mut len: size_t, mut mode: MemoryAccess) {
    if len == 0 as size_t {
        return;
    }

    let mut prot = memoryAccessToProt(mode);

    if mprotect(start, len, prot) == -(1 as c_int) {
        sysErrorBelch(
            b"mprotectForLinker: failed to protect %zd bytes at %p as %s\0" as *const u8
                as *const c_char,
            len,
            start,
            memoryAccessDescription(mode),
        );
    }
}
