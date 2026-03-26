use crate::ffi::rts::flags::RtsFlags;
use crate::ghcautoconf::RTS_LINKER_USE_MMAP;
use crate::linker::m_map::{
    MEM_READ_EXECUTE, mmapAnonForLinker, mprotectForLinker, munmapForLinker,
};
use crate::linker::m32_alloc::m32_alloc;
use crate::linker::mach_o::ocInit_MachO;
use crate::linker_internals::{ObjectCode, SymbolExtra, USE_CONTIGUOUS_MMAP};
use crate::prelude::*;
use crate::rts_utils::stgReallocBytes;
use crate::sm::os_mem::roundUpToPage;

unsafe fn ocAllocateExtras(
    mut oc: *mut ObjectCode,
    mut count: c_int,
    mut first: c_int,
    mut bssSize: c_int,
) -> c_int {
    let mut oldImage = (*oc).image as *mut c_void;
    let extras_size: size_t = (size_of::<SymbolExtra>() as size_t).wrapping_mul(count as size_t);

    if count > 0 as c_int || bssSize > 0 as c_int {
        if RTS_LINKER_USE_MMAP == 0 {
            let mut aligned = (*oc).fileSize + 3 as c_int & !(3 as c_int);
            let mut misalignment = (*oc).misalignment;
            (*oc).image = (*oc).image.offset(-(misalignment as isize));

            (*oc).image = stgReallocBytes(
                (*oc).image as *mut c_void,
                ((misalignment + aligned) as size_t).wrapping_add(extras_size),
                b"ocAllocateExtras\0" as *const u8 as *const c_char as *mut c_char,
            ) as *mut c_char;

            (*oc).image = (*oc).image.offset(misalignment as isize);
            (*oc).symbol_extras = (*oc).image.offset(aligned as isize) as *mut SymbolExtra;
        } else if USE_CONTIGUOUS_MMAP != 0 || RtsFlags.MiscFlags.linkerAlwaysPic as c_int != 0 {
            let mut n = roundUpToPage((*oc).fileSize as size_t);
            bssSize = roundUpToPage(bssSize as size_t) as c_int;

            let mut allocated_size: size_t =
                n.wrapping_add(bssSize as size_t).wrapping_add(extras_size);

            let mut new = mmapAnonForLinker(allocated_size);

            if !new.is_null() {
                memcpy(new, (*oc).image as *const c_void, (*oc).fileSize as size_t);

                if (*oc).imageMapped != 0 {
                    munmapForLinker(
                        (*oc).image as *mut c_void,
                        n,
                        b"ocAllocateExtras\0" as *const u8 as *const c_char,
                    );
                }

                (*oc).image = new as *mut c_char;
                (*oc).imageMapped = r#true;
                (*oc).fileSize = allocated_size as c_int;
                (*oc).symbol_extras =
                    (*oc).image.offset(n as isize).offset(bssSize as isize) as *mut SymbolExtra;
                (*oc).bssBegin = (*oc).image.offset(n as isize);
                (*oc).bssEnd = (*oc).image.offset(n as isize).offset(bssSize as isize);
            } else {
                (*oc).symbol_extras = null_mut::<SymbolExtra>();

                return 0 as c_int;
            }
        } else {
            (*oc).symbol_extras =
                m32_alloc((*oc).rx_m32, extras_size, 8 as size_t) as *mut SymbolExtra;

            if (*oc).symbol_extras.is_null() {
                return 0 as c_int;
            }
        }
    }

    if !(*oc).symbol_extras.is_null() {
        memset((*oc).symbol_extras as *mut c_void, 0 as c_int, extras_size);
    }

    if (*oc).image != oldImage as *mut c_char {
        ocInit_MachO(oc);
    }

    (*oc).first_symbol_extra = first as c_ulong;
    (*oc).n_symbol_extras = count as c_ulong;

    return 1 as c_int;
}

unsafe fn ocProtectExtras(mut oc: *mut ObjectCode) {
    if (*oc).n_symbol_extras == 0 as c_ulong {
        return;
    }

    if !(RTS_LINKER_USE_MMAP == 0) {
        if USE_CONTIGUOUS_MMAP != 0 || RtsFlags.MiscFlags.linkerAlwaysPic as c_int != 0 {
            mprotectForLinker(
                (*oc).symbol_extras as *mut c_void,
                (size_of::<SymbolExtra>() as size_t).wrapping_mul((*oc).n_symbol_extras as size_t),
                MEM_READ_EXECUTE,
            );
        }
    }
}
