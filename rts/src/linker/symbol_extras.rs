use crate::ghcautoconf::RTS_LINKER_USE_MMAP;
use crate::linker::m_map::{
    MEM_READ_EXECUTE, mmapAnonForLinker, mprotectForLinker, munmapForLinker,
};
use crate::linker::m32_alloc::m32_alloc;
use crate::linker::mach_o::ocInit_MachO;
use crate::linker_internals::{ObjectCode, SymbolExtra, USE_CONTIGUOUS_MMAP};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::stgReallocBytes;
use crate::sm::os_mem::roundUpToPage;

unsafe fn ocAllocateExtras(
    mut oc: *mut ObjectCode,
    mut count: i32,
    mut first: i32,
    mut bssSize: i32,
) -> i32 {
    let mut oldImage = (*oc).image as *mut c_void;
    let extras_size: usize = (size_of::<SymbolExtra>() as usize).wrapping_mul(count as usize);

    if count > 0 || bssSize > 0 {
        if RTS_LINKER_USE_MMAP == 0 {
            let mut aligned = (*oc).fileSize + 3 & !3;
            let mut misalignment = (*oc).misalignment;
            (*oc).image = (*oc).image.offset(-(misalignment as isize));

            (*oc).image = stgReallocBytes(
                (*oc).image as *mut c_void,
                ((misalignment + aligned) as usize).wrapping_add(extras_size),
                c"ocAllocateExtras".as_ptr(),
            ) as *mut c_char;

            (*oc).image = (*oc).image.offset(misalignment as isize);
            (*oc).symbol_extras = (*oc).image.offset(aligned as isize) as *mut SymbolExtra;
        } else if USE_CONTIGUOUS_MMAP != 0 || RtsFlags.MiscFlags.linkerAlwaysPic as i32 != 0 {
            let mut n = roundUpToPage((*oc).fileSize as usize);
            bssSize = roundUpToPage(bssSize as usize) as i32;

            let mut allocated_size: usize =
                n.wrapping_add(bssSize as usize).wrapping_add(extras_size);

            let mut new = mmapAnonForLinker(allocated_size);

            if !new.is_null() {
                memcpy(new, (*oc).image as *const c_void, (*oc).fileSize as usize);

                if (*oc).imageMapped != 0 {
                    munmapForLinker((*oc).image as *mut c_void, n, c"ocAllocateExtras".as_ptr());
                }

                (*oc).image = new as *mut c_char;
                (*oc).imageMapped = true;
                (*oc).fileSize = allocated_size as i32;
                (*oc).symbol_extras =
                    (*oc).image.offset(n as isize).offset(bssSize as isize) as *mut SymbolExtra;
                (*oc).bssBegin = (*oc).image.offset(n as isize);
                (*oc).bssEnd = (*oc).image.offset(n as isize).offset(bssSize as isize);
            } else {
                (*oc).symbol_extras = null_mut::<SymbolExtra>();

                return 0;
            }
        } else {
            (*oc).symbol_extras = m32_alloc((*oc).rx_m32, extras_size, 8) as *mut SymbolExtra;

            if (*oc).symbol_extras.is_null() {
                return 0;
            }
        }
    }

    if !(*oc).symbol_extras.is_null() {
        memset((*oc).symbol_extras as *mut c_void, 0, extras_size);
    }

    if (*oc).image != oldImage as *mut c_char {
        ocInit_MachO(oc);
    }

    (*oc).first_symbol_extra = first as u64;
    (*oc).n_symbol_extras = count as u64;

    return 1;
}

unsafe fn ocProtectExtras(mut oc: *mut ObjectCode) {
    if (*oc).n_symbol_extras == 0 {
        return;
    }

    if !(RTS_LINKER_USE_MMAP == 0) {
        if USE_CONTIGUOUS_MMAP != 0 || RtsFlags.MiscFlags.linkerAlwaysPic as i32 != 0 {
            mprotectForLinker(
                (*oc).symbol_extras as *mut c_void,
                (size_of::<SymbolExtra>() as usize).wrapping_mul((*oc).n_symbol_extras as usize),
                MEM_READ_EXECUTE,
            );
        }
    }
}
