use crate::ffi::rts::_assertFail;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch, sysErrorBelch};
use crate::ffi::rts_api::getProgArgv;
use crate::ffi::stg::types::StgWord;
use crate::get_env::{freeProgEnvv, getProgEnvv};
use crate::linker::m_map::{
    MEM_READ_EXECUTE, MEM_READ_WRITE, mmapAnonForLinker, mmapForLinker, mprotectForLinker,
    munmapForLinker,
};
use crate::linker::mach_o_types::{
    MachODsymtabCommand, MachOHeader, MachOLoadCommand, MachONList, MachORelocationInfo,
    MachOSection, MachOSegmentCommand, MachOSymbol, MachOSymtabCommand, ObjectCodeFormatInfo,
};
use crate::linker::macho::plt::{findStub, freeStubs, makeStub, numberOfStubsForSection};
use crate::linker::macho::plt_aarch64::stubSizeAarch64;
use crate::linker::proddable_blocks::{addProddableBlock, checkProddableBlock};
use crate::linker::symbol_extras::ocAllocateExtras;
use crate::linker_internals::{
    ObjectCode, SECTION_M32, SECTION_MMAP, SECTION_NOMEM, SECTIONKIND_CODE_OR_RODATA,
    SECTIONKIND_FINI_ARRAY, SECTIONKIND_INIT_ARRAY, SECTIONKIND_OTHER, SECTIONKIND_RWDATA,
    SEGMENT_PROT_RWO, SEGMENT_PROT_RX, Section, SectionKind, Segment, Symbol_t, addSection, fini_t,
    ghciInsertSymbolTable, init_t, initSegment, lookupDependentSymbol, symhash,
};
use crate::prelude::*;
use crate::rts_symbols::{STRENGTH_NORMAL, SYM_TYPE_CODE, SymbolName};
use crate::rts_utils::{stgCallocBytes, stgFree, stgMallocBytes};
use crate::sm::os_mem::{roundUpToAlign, roundUpToPage};

unsafe fn ocInit_MachO(mut oc: *mut ObjectCode) {
    ocDeinit_MachO(oc);

    (*oc).info = stgCallocBytes(
        1,
        size_of::<ObjectCodeFormatInfo>() as usize,
        c"ocInit_MachO(ObjectCodeFormatInfo)".as_ptr(),
    ) as *mut ObjectCodeFormatInfo;

    (*(*oc).info).header = (*oc).image as *mut MachOHeader;
    (*(*oc).info).symCmd = null_mut::<MachOSymtabCommand>();
    (*(*oc).info).segCmd = null_mut::<MachOSegmentCommand>();
    (*(*oc).info).dsymCmd = null_mut::<MachODsymtabCommand>();

    let mut lc = (*oc)
        .image
        .offset(size_of::<MachOHeader>() as usize as isize)
        as *mut MachOLoadCommand;

    let mut i: usize = 0;

    while i < (*(*(*oc).info).header).ncmds as usize {
        if (*lc).cmd == LC_SEGMENT as u32 || (*lc).cmd == LC_SEGMENT_64 as u32 {
            (*(*oc).info).segCmd = lc as *mut MachOSegmentCommand;
        } else if (*lc).cmd == LC_SYMTAB as u32 {
            (*(*oc).info).symCmd = lc as *mut MachOSymtabCommand;
        } else if (*lc).cmd == LC_DYSYMTAB as u32 {
            (*(*oc).info).dsymCmd = lc as *mut MachODsymtabCommand;
        }

        lc = (lc as *mut c_char).offset((*lc).cmdsize as isize) as *mut MachOLoadCommand;
        i = i.wrapping_add(1);
    }

    if (*(*oc).info).segCmd.is_null() {
        barf(c"ocGetNames_MachO: no segment load command".as_ptr());
    }

    (*(*oc).info).macho_sections = (*(*oc).info).segCmd.offset(1) as *mut MachOSection;
    (*oc).n_sections = (*(*(*oc).info).segCmd).nsects as i32;

    (*(*oc).info).nlist = if (*(*oc).info).symCmd.is_null() {
        null_mut::<MachONList>()
    } else {
        (*oc).image.offset((*(*(*oc).info).symCmd).symoff as isize) as *mut MachONList
    };

    (*(*oc).info).names = if (*(*oc).info).symCmd.is_null() {
        null_mut::<c_char>()
    } else {
        (*oc).image.offset((*(*(*oc).info).symCmd).stroff as isize)
    };

    (*(*oc).info).n_macho_symbols = 0;
    (*(*oc).info).macho_symbols = null_mut::<MachOSymbol>();

    if !(*(*oc).info).nlist.is_null() {
        (*(*oc).info).n_macho_symbols = (*(*(*oc).info).symCmd).nsyms as usize;

        (*(*oc).info).macho_symbols = stgCallocBytes(
            (*(*(*oc).info).symCmd).nsyms as usize,
            size_of::<MachOSymbol>() as usize,
            c"ocInit_MachO(MachOSymbol)".as_ptr(),
        ) as *mut MachOSymbol;

        let mut i_0: u32 = 0;

        while i_0 < (*(*(*oc).info).symCmd).nsyms {
            let ref mut fresh5 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).name;
            *fresh5 = (*(*oc).info)
                .names
                .offset((*(*(*oc).info).nlist.offset(i_0 as isize)).n_un.n_strx as isize)
                as *mut SymbolName;

            let ref mut fresh6 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).nlist;
            *fresh6 = (*(*oc).info).nlist.offset(i_0 as isize) as *mut MachONList;

            let ref mut fresh7 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).addr;
            *fresh7 = NULL as *mut c_void;

            let ref mut fresh8 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).got_addr;
            *fresh8 = NULL;
            i_0 = i_0.wrapping_add(1);
        }
    }
}

unsafe fn ocDeinit_MachO(mut oc: *mut ObjectCode) {
    if !(*oc).info.is_null() {
        if (*(*oc).info).n_macho_symbols > 0 {
            stgFree((*(*oc).info).macho_symbols as *mut c_void);
        }

        freeGot(oc);

        if !(*oc).sections.is_null() {
            let mut i = 0;

            while i < (*oc).n_sections {
                freeStubs((*oc).sections.offset(i as isize) as *mut Section);
                i += 1;
            }
        }

        stgFree((*oc).info as *mut c_void);
        (*oc).info = null_mut::<ObjectCodeFormatInfo>();
    }
}

unsafe fn ocAllocateExtras_MachO(mut oc: *mut ObjectCode) -> i32 {
    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocAllocateExtras_MachO: start\n".as_ptr());
    }

    if !(*(*oc).info).symCmd.is_null() {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocAllocateExtras_MachO: allocate %d symbols\n".as_ptr(),
                (*(*(*oc).info).symCmd).nsyms,
            );
        }

        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ocAllocateExtras_MachO: done\n".as_ptr());
        }

        return ocAllocateExtras(oc, (*(*(*oc).info).symCmd).nsyms as i32, 0, 0);
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocAllocateExtras_MachO: allocated no symbols\n".as_ptr());
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocAllocateExtras_MachO: done\n".as_ptr());
    }

    return ocAllocateExtras(oc, 0, 0, 0);
}

unsafe fn ocVerifyImage_MachO(mut oc: *mut ObjectCode) -> i32 {
    let mut image = (*oc).image;
    let mut header = image as *mut MachOHeader;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocVerifyImage_MachO: start\n".as_ptr());
    }

    if (*header).magic != MH_MAGIC_64 as u32 {
        errorBelch(
            c"Could not load image %s: bad magic!\n  Expected %08x (64bit), got %08x%s\n".as_ptr(),
            (*oc).fileName,
            MH_MAGIC_64,
            (*header).magic,
            if (*header).magic == MH_MAGIC as u32 {
                c" (32bit).".as_ptr()
            } else {
                c".".as_ptr()
            },
        );

        return 0;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocVerifyImage_MachO: done\n".as_ptr());
    }

    return 1;
}

unsafe fn resolveImports(
    mut oc: *mut ObjectCode,
    mut sect: *mut MachOSection,
    mut indirectSyms: *mut u64,
) -> i32 {
    let mut itemSize: usize = 4;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"resolveImports: start\n".as_ptr());
    }

    let mut i = 0;

    while ((i as usize).wrapping_mul(itemSize) as u64) < (*sect).size {
        let mut indirectSymbolIndex =
            *indirectSyms.offset((*sect).reserved1.wrapping_add(i as u32) as isize);

        let mut symbol: *mut MachOSymbol = (*(*oc).info)
            .macho_symbols
            .offset(indirectSymbolIndex as isize)
            as *mut MachOSymbol;

        let mut addr = NULL as *mut c_void;

        if RtsFlags.DebugFlags.linker {
            debugBelch(c"resolveImports: resolving %s\n".as_ptr(), (*symbol).name);
        }

        if (*(*symbol).nlist).n_type as i32 & N_TYPE == N_UNDF
            && (*(*symbol).nlist).n_type as i32 & N_EXT != 0
            && (*(*symbol).nlist).n_value != 0
        {
            addr = (*(*symbol).nlist).n_value as *mut c_void;

            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"resolveImports: undefined external %s has value %p\n".as_ptr(),
                    (*symbol).name,
                    addr,
                );
            }
        } else {
            addr = lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>());

            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"resolveImports: looking up %s, %p\n".as_ptr(),
                    (*symbol).name,
                    addr,
                );
            }
        }

        if addr.is_null() {
            errorBelch(
                c"\nlookupSymbol failed in resolveImports\n%s: unknown symbol `%s'".as_ptr(),
                (*oc).fileName,
                (*symbol).name,
            );

            return 0;
        }

        checkProddableBlock(
            &raw mut (*oc).proddables,
            ((*oc).image.offset((*sect).offset as isize) as *mut *mut c_void).offset(i as isize)
                as *mut c_void,
            size_of::<*mut c_void>() as usize,
        );

        let ref mut fresh23 =
            *((*oc).image.offset((*sect).offset as isize) as *mut *mut c_void).offset(i as isize);
        *fresh23 = addr as *mut c_void;
        i = i.wrapping_add(1);
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"resolveImports: done\n".as_ptr());
    }

    return 1;
}

unsafe fn signExtend(mut val: u64, mut bits: u8) -> i64 {
    return (val << 64 - bits as i32) as i64 >> 64 - bits as i32;
}

unsafe fn isVectorOp(mut p: *mut u32) -> bool {
    return *p & 0x4800000 == 0x4800000;
}

unsafe fn isLoadStore(mut p: *mut u32) -> bool {
    return *p & 0x3b000000 == 0x39000000;
}

unsafe fn decodeAddend(
    mut oc: *mut ObjectCode,
    mut section: *mut Section,
    mut ri: *mut MachORelocationInfo,
) -> i64 {
    let mut p = ((*section).start as *mut u8).offset((*ri).r_address as isize) as *mut u32;

    checkProddableBlock(
        &raw mut (*oc).proddables,
        p as *mut c_void,
        (1 << (*ri).r_length() as i32) as usize,
    );

    match (*ri).r_type() as i32 {
        0 => match (*ri).r_length() as i32 {
            0 => {
                return signExtend(*(p as *mut u8) as u64, (8 << (*ri).r_length() as i32) as u8);
            }
            1 => {
                return signExtend(
                    *(p as *mut u16) as u64,
                    (8 << (*ri).r_length() as i32) as u8,
                );
            }
            2 => {
                return signExtend(*p as u64, (8 << (*ri).r_length() as i32) as u8);
            }
            3 => {
                return signExtend(*(p as *mut u64), (8 << (*ri).r_length() as i32) as u8);
            }
            _ => {
                barf(
                    c"Unsupported r_length (%d) for UNSIGNED relocation".as_ptr(),
                    (*ri).r_length() as i32,
                );
            }
        },
        1 => match (*ri).r_length() as i32 {
            0 => {
                return signExtend(*(p as *mut u8) as u64, (8 << (*ri).r_length() as i32) as u8);
            }
            1 => {
                return signExtend(
                    *(p as *mut u16) as u64,
                    (8 << (*ri).r_length() as i32) as u8,
                );
            }
            2 => {
                return signExtend(*p as u64, (8 << (*ri).r_length() as i32) as u8);
            }
            3 => {
                return signExtend(*(p as *mut u64), (8 << (*ri).r_length() as i32) as u8);
            }
            _ => {
                barf(
                    c"Unsupported r_length (%d) for SUBTRACTOR relocation".as_ptr(),
                    (*ri).r_length() as i32,
                );
            }
        },
        2 => {
            return signExtend(((*p & 0x3ffffff) << 2) as u64, 28);
        }
        3 | 5 => {
            return signExtend(
                ((*p & 0x60000000) >> 29 | ((*p & 0x1ffffe0) >> 3) << 12) as u64,
                33,
            );
        }
        4 | 6 => {
            let mut a: i64 = ((*p & 0x3ffc00) >> 10) as i64;
            let mut shift = 0;

            if isLoadStore(p) {
                shift = (*p >> 30 & 0x3) as i32;

                if 0 == shift && isVectorOp(p) as i32 != 0 {
                    shift = 4;
                }
            }

            return a << shift;
        }
        _ => {}
    }

    barf(
        c"unsupported relocation type: %d\n".as_ptr(),
        (*ri).r_type() as i32,
    );
}

#[inline]
unsafe fn fitsBits(mut bits: usize, mut value: i64) -> bool {
    if bits == 64 {
        return true;
    }

    if bits > 64 {
        barf(
            c"fits_bits with %zu bits and an 64bit integer!".as_ptr(),
            bits,
        );
    }

    return 0 == value >> bits || -1 as i64 == value >> bits;
}

unsafe fn encodeAddend(
    mut oc: *mut ObjectCode,
    mut section: *mut Section,
    mut ri: *mut MachORelocationInfo,
    mut addend: i64,
    mut symbol: *mut MachOSymbol,
) {
    let mut p = ((*section).start as *mut u8).offset((*ri).r_address as isize) as *mut u32;

    checkProddableBlock(
        &raw mut (*oc).proddables,
        p as *mut c_void,
        (1 << (*ri).r_length() as i32) as usize,
    );

    let mut symbol_name = if !symbol.is_null() && !(*symbol).name.is_null() {
        (*symbol).name as *mut c_char as *const c_char
    } else {
        c"<unknown>".as_ptr()
    };

    let mut file_name = if !(*oc).fileName.is_null() {
        (*oc).fileName as *mut c_char as *const c_char
    } else {
        c"<unknown>".as_ptr()
    };

    match (*ri).r_type() as i32 {
        0 => {
            if !fitsBits((8 << (*ri).r_length() as i32) as usize, addend) {
                let mut library_info: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    c"Relocation out of range for UNSIGNED in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s"
                        .as_ptr(),
                    file_name,
                    symbol_name,
                    addend as i64,
                    (*ri).r_address as i64,
                    if !library_info.is_null() {
                        library_info as *mut c_char as *const c_char
                    } else {
                        c"<unknown>".as_ptr()
                    },
                );
            }

            match (*ri).r_length() as i32 {
                0 => {
                    *(p as *mut u8) = addend as u8;
                }
                1 => {
                    *(p as *mut u16) = addend as u16;
                }
                2 => {
                    *p = addend as u32;
                }
                3 => {
                    *(p as *mut u64) = addend as u64;
                }
                _ => {
                    barf(
                        c"Unsupported r_length (%d) for UNSIGNED relocation".as_ptr(),
                        (*ri).r_length() as i32,
                    );
                }
            }

            return;
        }
        1 => {
            if !fitsBits((8 << (*ri).r_length() as i32) as usize, addend) {
                let mut library_info_0: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    c"Relocation out of range for SUBTRACTOR in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s"
                        .as_ptr(),
                    file_name,
                    symbol_name,
                    addend as i64,
                    (*ri).r_address as i64,
                    if !library_info_0.is_null() {
                        library_info_0 as *mut c_char as *const c_char
                    } else {
                        c"<unknown>".as_ptr()
                    },
                );
            }

            match (*ri).r_length() as i32 {
                0 => {
                    *(p as *mut u8) = addend as u8;
                }
                1 => {
                    *(p as *mut u16) = addend as u16;
                }
                2 => {
                    *p = addend as u32;
                }
                3 => {
                    *(p as *mut u64) = addend as u64;
                }
                _ => {
                    barf(
                        c"Unsupported r_length (%d) for SUBTRACTOR relocation".as_ptr(),
                        (*ri).r_length() as i32,
                    );
                }
            }

            return;
        }
        2 => {
            if !fitsBits(26, addend >> 2) {
                let mut library_info_1: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    c"Relocation target for BRANCH26 out of range in %s: symbol '%s', addend 0x%llx (0x%llx >> 2), address 0x%llx, library: %s"
                        .as_ptr(),
                    file_name,
                    symbol_name,
                    addend as i64,
                    (addend >> 2) as i64,
                    (*ri).r_address as i64,
                    if !library_info_1.is_null() {
                        library_info_1 as *mut c_char as *const c_char
                    } else {
                        c"<unknown>".as_ptr()
                    },
                );
            }

            *p = *p & 0xfc000000 | (addend >> 2) as u32 & 0x3ffffff;
            return;
        }
        3 | 5 => {
            if !fitsBits(21, addend >> 12) {
                let mut reloc_type = if (*ri).r_type() as i32 == ARM64_RELOC_PAGE21 as i32 {
                    c"PAGE21".as_ptr()
                } else {
                    c"GOT_LOAD_PAGE21".as_ptr()
                };

                let mut library_info_2: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    c"Relocation target for %s out of range in %s: symbol '%s', addend 0x%llx (0x%llx >> 12), address 0x%llx, library: %s"
                        .as_ptr(),
                    reloc_type,
                    file_name,
                    symbol_name,
                    addend as i64,
                    (addend >> 12) as i64,
                    (*ri).r_address as i64,
                    if !library_info_2.is_null() {
                        library_info_2 as *mut c_char as *const c_char
                    } else {
                        c"<unknown>".as_ptr()
                    },
                );
            }

            *p = *p & 0x9f00001f
                | (addend << 17 & 0x60000000) as u32
                | (addend >> 9 & 0xffffe0) as u32;
            return;
        }
        4 | 6 => {
            if !fitsBits(12, addend) {
                let mut library_info_3: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    c"Relocation target for PAGEOFF12 out of range in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s"
                        .as_ptr(),
                    file_name,
                    symbol_name,
                    addend as i64,
                    (*ri).r_address as i64,
                    if !library_info_3.is_null() {
                        library_info_3 as *mut c_char as *const c_char
                    } else {
                        c"<unknown>".as_ptr()
                    },
                );
            }

            let mut shift = 0;

            if isLoadStore(p) {
                shift = (*p >> 30 & 0x3) as i32;

                if 0 == shift && isVectorOp(p) as i32 != 0 {
                    shift = 4;
                }
            }

            *p = *p & 0xffc003ff | (addend >> shift << 10) as u32 & 0x3ffc00;
            return;
        }
        _ => {}
    }

    barf(
        c"unsupported relocation type: %d\n".as_ptr(),
        (*ri).r_type() as i32,
    );
}

unsafe fn findInternalGotRefs(mut oc: *mut ObjectCode) {
    let mut curSection = 0;

    while curSection < (*oc).n_sections {
        let mut sect: *mut Section = (*oc).sections.offset(curSection as isize) as *mut Section;

        if !(*sect).info.is_null() {
            let mut msect = (*(*sect).info).macho_section;
            let mut relocs = (*(*sect).info).relocation_info;
            let mut i: u32 = 0;

            while i < (*msect).nreloc {
                let mut ri: *mut MachORelocationInfo =
                    relocs.offset(i as isize) as *mut MachORelocationInfo;

                if isGotLoad(ri as *mut relocation_info) {
                    let mut symbol: *mut MachOSymbol = (*(*oc).info)
                        .macho_symbols
                        .offset((*ri).r_symbolnum() as isize)
                        as *mut MachOSymbol;
                    (*symbol).needs_got = true;
                }

                i = i.wrapping_add(1);
            }
        }

        curSection += 1;
    }
}

unsafe fn isGotLoad(mut ri: *mut relocation_info) -> bool {
    return (*ri).r_type() as i32 == ARM64_RELOC_GOT_LOAD_PAGE21 as i32
        || (*ri).r_type() as i32 == ARM64_RELOC_GOT_LOAD_PAGEOFF12 as i32;
}

unsafe fn needGotSlot(mut symbol: *mut MachOSymbol) -> bool {
    if (*symbol).needs_got {
        return true;
    }

    return (*(*symbol).nlist).n_type as i32 & N_EXT != 0
        && (N_UNDF == (*(*symbol).nlist).n_type as i32 & N_TYPE
            || NO_SECT != (*(*symbol).nlist).n_sect as i32);
}

unsafe fn makeGot(mut oc: *mut ObjectCode) -> bool {
    let mut got_slots: usize = 0;
    let mut i: usize = 0;

    while i < (*(*oc).info).n_macho_symbols {
        if needGotSlot((*(*oc).info).macho_symbols.offset(i as isize) as *mut MachOSymbol) {
            got_slots = got_slots.wrapping_add(1 as usize);
        }

        i = i.wrapping_add(1);
    }

    if got_slots > 0 {
        (*(*oc).info).got_size = got_slots.wrapping_mul(size_of::<*mut c_void>() as usize);
        (*(*oc).info).got_start = mmapAnonForLinker((*(*oc).info).got_size);

        if (*(*oc).info).got_start.is_null() {
            barf(c"MAP_FAILED. errno=%d".as_ptr(), *__error());
        }

        let mut slot: usize = 0;
        let mut i_0: usize = 0;

        while i_0 < (*(*oc).info).n_macho_symbols {
            if needGotSlot((*(*oc).info).macho_symbols.offset(i_0 as isize) as *mut MachOSymbol) {
                let fresh21 = slot;
                slot = slot.wrapping_add(1);

                let ref mut fresh22 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).got_addr;
                *fresh22 = ((*(*oc).info).got_start as *mut u8)
                    .offset(fresh21.wrapping_mul(size_of::<*mut c_void>() as usize) as isize)
                    as *mut c_void;
            }

            i_0 = i_0.wrapping_add(1);
        }
    }

    return EXIT_SUCCESS != 0;
}

unsafe fn freeGot(mut oc: *mut ObjectCode) {
    if !(*(*oc).info).got_start.is_null() && (*(*oc).info).got_size > 0 {
        munmapForLinker(
            (*(*oc).info).got_start,
            (*(*oc).info).got_size,
            c"freeGot".as_ptr(),
        );
    }

    (*(*oc).info).got_start = NULL;
    (*(*oc).info).got_size = 0;
}

unsafe fn symbol_value(mut oc: *mut ObjectCode, mut symbol: *mut MachOSymbol) -> u64 {
    let mut value: u64 = 0;

    if (*(*symbol).nlist).n_type as i32 & N_EXT != 0 {
        value = lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>()) as u64;

        if value == 0 {
            barf(c"Could not lookup symbol: %s!".as_ptr(), (*symbol).name);
        }
    } else {
        value = (*symbol).addr as u64;
    }

    return value;
}

unsafe fn relocateSectionAarch64(mut oc: *mut ObjectCode, mut section: *mut Section) -> i32 {
    if (*section).size == 0 {
        return 1;
    }

    let mut explicit_addend: i64 = 0;
    let mut nreloc: usize = (*(*(*section).info).macho_section).nreloc as usize;
    let mut i: usize = 0;

    while i < nreloc {
        let mut ri: *mut MachORelocationInfo =
            (*(*section).info).relocation_info.offset(i as isize) as *mut MachORelocationInfo;

        match (*ri).r_type() as i32 {
            0 => {
                let mut symbol: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend = decodeAddend(oc, section, ri);
                let mut value = symbol_value(oc, symbol);

                encodeAddend(
                    oc,
                    section,
                    ri,
                    value.wrapping_add(addend as u64) as i64,
                    symbol,
                );
            }
            1 => {
                if !(i.wrapping_add(1 as usize) < nreloc)
                    || !((*(*(*section).info)
                        .relocation_info
                        .offset(i.wrapping_add(1 as usize) as isize))
                    .r_type() as i32
                        == ARM64_RELOC_UNSIGNED as i32)
                {
                    barf(
                        c"SUBTRACTOR relocation *must* be followed by UNSIGNED relocation."
                            .as_ptr(),
                    );
                }

                let mut symbol1: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut sub_value = symbol_value(oc, symbol1);
                let mut ri2: *mut MachORelocationInfo = (*(*section).info)
                    .relocation_info
                    .offset(i.wrapping_add(1 as usize) as isize)
                    as *mut MachORelocationInfo;

                let mut symbol2: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri2).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut add_value = symbol_value(oc, symbol2);
                let mut addend_0 = decodeAddend(oc, section, ri);

                encodeAddend(
                    oc,
                    section,
                    ri,
                    (addend_0 as u64)
                        .wrapping_sub(sub_value)
                        .wrapping_add(add_value) as i64,
                    symbol1,
                );

                i = i.wrapping_add(1 as usize);
            }
            2 => {
                let mut symbol_0: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_1 = decodeAddend(oc, section, ri);
                let mut pc: u64 = ((*section).start as u64).wrapping_add((*ri).r_address as u64);

                let mut value_0: u64 = 0;

                if (*(*symbol_0).nlist).n_type as i32 & N_EXT != 0 {
                    value_0 =
                        lookupDependentSymbol((*symbol_0).name, oc, null_mut::<SymType>()) as u64;

                    if value_0 == 0 {
                        barf(c"Could not lookup symbol: %s!".as_ptr(), (*symbol_0).name);
                    }
                } else {
                    value_0 = (*symbol_0).addr as u64;
                }

                if value_0.wrapping_sub(pc).wrapping_add(addend_1 as u64) >> 2 + 26 - 1 != 0 {
                    if findStub(section, &raw mut value_0 as *mut *mut c_void, 0) {
                        if makeStub(section, &raw mut value_0 as *mut *mut c_void, 0) {
                            barf(c"could not find or make stub".as_ptr());
                        }
                    }
                }

                encodeAddend(
                    oc,
                    section,
                    ri,
                    value_0.wrapping_sub(pc).wrapping_add(addend_1 as u64) as i64,
                    symbol_0,
                );
            }
            3 | 5 => {
                let mut symbol_1: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_2 = decodeAddend(oc, section, ri);

                if !(explicit_addend == 0 || addend_2 == 0) {
                    barf(c"explicit_addend and addend can't be set at the same time.".as_ptr());
                }

                let mut pc_0: u64 = ((*section).start as u64).wrapping_add((*ri).r_address as u64);

                let mut value_1: u64 = (if isGotLoad(ri as *mut relocation_info) as i32 != 0 {
                    (*symbol_1).got_addr
                } else {
                    (*symbol_1).addr as *mut c_void
                }) as u64;

                if (!isGotLoad(ri as *mut relocation_info) || !(*symbol_1).got_addr.is_null())
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/linker/MachO.c".as_ptr(), 697);
                }

                encodeAddend(
                    oc,
                    section,
                    ri,
                    (value_1
                        .wrapping_add(addend_2 as u64)
                        .wrapping_add(explicit_addend as u64)
                        & -(4096 as i32) as u64)
                        .wrapping_sub(pc_0 & -(4096 as i32) as u64) as i64,
                    symbol_1,
                );

                explicit_addend = 0;
            }
            4 | 6 => {
                let mut symbol_2: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_3 = decodeAddend(oc, section, ri);

                if !(explicit_addend == 0 || addend_3 == 0) {
                    barf(c"explicit_addend and addend can't be set at the same time.".as_ptr());
                }

                let mut value_2: u64 = (if isGotLoad(ri as *mut relocation_info) as i32 != 0 {
                    (*symbol_2).got_addr
                } else {
                    (*symbol_2).addr as *mut c_void
                }) as u64;

                if (!isGotLoad(ri as *mut relocation_info) || !(*symbol_2).got_addr.is_null())
                    as i32 as i64
                    != 0
                {
                } else {
                    _assertFail(c"rts/linker/MachO.c".as_ptr(), 711);
                }

                encodeAddend(
                    oc,
                    section,
                    ri,
                    (0xfff
                        & value_2
                            .wrapping_add(addend_3 as u64)
                            .wrapping_add(explicit_addend as u64)) as i64,
                    symbol_2,
                );

                explicit_addend = 0;
            }
            10 => {
                explicit_addend = signExtend((*ri).r_symbolnum() as u64, 24);

                if !(i.wrapping_add(1 as usize) < nreloc)
                    || !((*(*(*section).info)
                        .relocation_info
                        .offset(i.wrapping_add(1 as usize) as isize))
                    .r_type() as i32
                        == ARM64_RELOC_PAGE21 as i32
                        || (*(*(*section).info)
                            .relocation_info
                            .offset(i.wrapping_add(1 as usize) as isize))
                        .r_type() as i32
                            == ARM64_RELOC_PAGEOFF12 as i32)
                {
                    barf(
                        c"ADDEND relocation *must* be followed by PAGE or PAGEOFF relocation"
                            .as_ptr(),
                    );
                }
            }
            _ => {
                barf(
                    c"Relocation of type: %d not (yet) supported!\n".as_ptr(),
                    (*ri).r_type() as i32,
                );
            }
        }

        i = i.wrapping_add(1);
    }

    return 1;
}

unsafe fn getSectionKind_MachO(mut section: *mut MachOSection) -> SectionKind {
    let mut s_type: u8 = ((*section).flags & SECTION_TYPE as u32) as u8;

    if s_type as i32 == S_MOD_INIT_FUNC_POINTERS {
        return SECTIONKIND_INIT_ARRAY;
    } else if s_type as i32 == S_MOD_TERM_FUNC_POINTERS {
        return SECTIONKIND_FINI_ARRAY;
    } else if 0
        == strcmp(
            &raw mut (*section).segname as *mut c_char,
            c"__TEXT".as_ptr(),
        )
    {
        return SECTIONKIND_CODE_OR_RODATA;
    } else if 0
        == strcmp(
            &raw mut (*section).segname as *mut c_char,
            c"__DATA".as_ptr(),
        )
    {
        return SECTIONKIND_RWDATA;
    } else {
        return SECTIONKIND_OTHER;
    };
}

unsafe fn ocBuildSegments_MachO(mut oc: *mut ObjectCode) -> i32 {
    let mut n_rxSections = 0;
    let mut size_rxSegment: usize = 0;
    let mut rxSegment = null_mut::<Segment>();
    let mut n_rwSections = 0;
    let mut size_rwSegment: usize = 0;
    let mut rwSegment = null_mut::<Segment>();
    let mut n_gbZerofills = 0;
    let mut size_gbZerofillSegment: usize = 0;
    let mut gbZerofillSegment = null_mut::<Segment>();
    let mut n_activeSegments = 0;
    let mut curSegment = 0;
    let mut size_compound: usize = 0;
    let mut segments = null_mut::<Segment>();
    let mut mem = NULL;
    let mut curMem = NULL;
    let mut i = 0;

    while i < (*oc).n_sections {
        let mut macho: *mut MachOSection =
            (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection;

        if 0 == (*macho).size {
            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"ocBuildSegments_MachO: found a zero length section, skipping\n".as_ptr(),
                );
            }
        } else {
            let mut alignment: usize = (1 << (*macho).align) as usize;

            if S_GB_ZEROFILL as u32 == (*macho).flags & SECTION_TYPE as u32 {
                size_gbZerofillSegment = roundUpToAlign(size_gbZerofillSegment, alignment);

                size_gbZerofillSegment =
                    (size_gbZerofillSegment as u64).wrapping_add((*macho).size) as usize as usize;
                n_gbZerofills += 1;
            } else if getSectionKind_MachO(macho) as u32 == SECTIONKIND_CODE_OR_RODATA as i32 as u32
            {
                size_rxSegment = roundUpToAlign(size_rxSegment, alignment);
                size_rxSegment =
                    (size_rxSegment as u64).wrapping_add((*macho).size) as usize as usize;
                n_rxSections += 1;
            } else {
                size_rwSegment = roundUpToAlign(size_rwSegment, alignment);
                size_rwSegment =
                    (size_rwSegment as u64).wrapping_add((*macho).size) as usize as usize;
                n_rwSections += 1;
            }
        }

        i += 1;
    }

    size_compound = roundUpToPage(size_rxSegment)
        .wrapping_add(roundUpToPage(size_rwSegment))
        .wrapping_add(roundUpToPage(size_gbZerofillSegment));

    if n_rxSections > 0 {
        n_activeSegments += 1;
    }

    if n_rwSections > 0 {
        n_activeSegments += 1;
    }

    if n_gbZerofills > 0 {
        n_activeSegments += 1;
    }

    if 0 == size_compound {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ocBuildSegments_MachO: all segments are empty, skipping\n".as_ptr());
        }

        return 1;
    }

    mem = mmapAnonForLinker(size_compound);

    if mem.is_null() {
        return 0;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"ocBuildSegments: allocating %d segments\n".as_ptr(),
            n_activeSegments,
        );
    }

    segments = stgCallocBytes(
        n_activeSegments as usize,
        size_of::<Segment>() as usize,
        c"ocBuildSegments_MachO(segments)".as_ptr(),
    ) as *mut Segment;

    curMem = mem;

    if n_rxSections > 0 {
        rxSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            rxSegment,
            curMem,
            roundUpToPage(size_rxSegment),
            SEGMENT_PROT_RX,
            n_rxSections,
        );

        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocBuildSegments_MachO: init segment %d (RX) at %p size %zu\n".as_ptr(),
                curSegment,
                (*rxSegment).start,
                (*rxSegment).size,
            );
        }

        curMem = (curMem as *mut c_char).offset((*rxSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    if n_rwSections > 0 {
        rwSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            rwSegment,
            curMem,
            roundUpToPage(size_rwSegment),
            SEGMENT_PROT_RWO,
            n_rwSections,
        );

        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocBuildSegments_MachO: init segment %d (RWO) at %p size %zu\n".as_ptr(),
                curSegment,
                (*rwSegment).start,
                (*rwSegment).size,
            );
        }

        curMem = (curMem as *mut c_char).offset((*rwSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    if n_gbZerofills > 0 {
        gbZerofillSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            gbZerofillSegment,
            curMem,
            roundUpToPage(size_gbZerofillSegment),
            SEGMENT_PROT_RWO,
            n_gbZerofills,
        );

        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocBuildSegments_MachO: init segment %d (GB_ZEROFILL) at %p size %zu\n".as_ptr(),
                curSegment,
                (*gbZerofillSegment).start,
                (*gbZerofillSegment).size,
            );
        }

        curMem = (curMem as *mut c_char).offset((*gbZerofillSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    let mut i_0 = 0;
    let mut rx = 0;
    let mut rw = 0;
    let mut gb = 0;

    while i_0 < (*oc).n_sections {
        let mut macho_0: *mut MachOSection =
            (*(*oc).info).macho_sections.offset(i_0 as isize) as *mut MachOSection;

        if !(0 == (*macho_0).size) {
            if S_GB_ZEROFILL as u32 == (*macho_0).flags & SECTION_TYPE as u32 {
                let fresh9 = gb;
                gb = gb + 1;
                *(*gbZerofillSegment).sections_idx.offset(fresh9 as isize) = i_0;
            } else if getSectionKind_MachO(macho_0) as u32
                == SECTIONKIND_CODE_OR_RODATA as i32 as u32
            {
                let fresh10 = rx;
                rx = rx + 1;
                *(*rxSegment).sections_idx.offset(fresh10 as isize) = i_0;
            } else {
                let fresh11 = rw;
                rw = rw + 1;
                *(*rwSegment).sections_idx.offset(fresh11 as isize) = i_0;
            }
        }

        i_0 += 1;
    }

    (*oc).segments = segments;
    (*oc).n_segments = n_activeSegments;

    return 1;
}

unsafe fn ocGetNames_MachO(mut oc: *mut ObjectCode) -> i32 {
    let mut curSymbol = 0;
    let mut commonSize = 0;
    let mut commonStorage = NULL as *mut c_void;
    let mut commonCounter: u64 = 0;

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"ocGetNames_MachO: %s start\n".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    let mut secArray = null_mut::<Section>();

    secArray = stgCallocBytes(
        (*(*(*oc).info).segCmd).nsects as usize,
        size_of::<Section>() as usize,
        c"ocGetNames_MachO(sections)".as_ptr(),
    ) as *mut Section;

    (*oc).sections = secArray;

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"ocGetNames_MachO: will load %d sections\n".as_ptr(),
            (*oc).n_sections,
        );
    }

    if (ocBuildSegments_MachO(oc) != 0) as i32 as i64 != 0 {
    } else {
        barf(c"ocGetNames_MachO: failed to build segments\n".as_ptr());
    }

    let mut seg_n = 0;

    while seg_n < (*oc).n_segments {
        let mut segment: *mut Segment = (*oc).segments.offset(seg_n as isize) as *mut Segment;

        let mut curMem = (*segment).start;

        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocGetNames_MachO: loading segment %d (address = %p, size = %zu) with %d sections\n"
                    .as_ptr(),
                seg_n,
                (*segment).start,
                (*segment).size,
                (*segment).n_sections,
            );
        }

        let mut sec_n = 0;

        while sec_n < (*segment).n_sections {
            let mut sec_idx = *(*segment).sections_idx.offset(sec_n as isize);
            let mut section: *mut MachOSection =
                (*(*oc).info).macho_sections.offset(sec_idx as isize) as *mut MachOSection;

            let mut alignment: usize = (1 << (*section).align) as usize;
            let mut kind = getSectionKind_MachO(section);
            let mut alloc = SECTION_NOMEM;
            let mut start = NULL;
            let mut mapped_start = NULL;
            let mut mapped_size: StgWord = 0;
            let mut mapped_offset: StgWord = 0;
            let mut size: StgWord = (*section).size as StgWord;
            let mut secMem = roundUpToAlign(curMem as usize, alignment) as *mut c_void;
            start = secMem;

            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"ocGetNames_MachO: loading section %d in segment %d (#%d, %s %s)\n                  skipped %zu bytes due to alignment of %zu\n"
                        .as_ptr(),
                    sec_n,
                    seg_n,
                    sec_idx,
                    &raw mut (*section).segname as *mut c_char,
                    &raw mut (*section).sectname as *mut c_char,
                    (secMem as *mut c_char).offset_from(curMem as *mut c_char) as i64,
                    alignment,
                );
            }

            match (*section).flags & SECTION_TYPE as u32 {
                1 | 12 => {
                    if RtsFlags.DebugFlags.linker {
                        debugBelch(c"ocGetNames_MachO: memset to 0 a ZEROFILL section\n".as_ptr());
                    }

                    memset(secMem, 0, (*section).size as usize);

                    addSection(
                        secArray.offset(sec_idx as isize) as *mut Section,
                        kind,
                        alloc,
                        start,
                        size,
                        mapped_offset,
                        mapped_start,
                        mapped_size,
                    );
                }
                _ => {
                    if RtsFlags.DebugFlags.linker {
                        debugBelch(
                            c"ocGetNames_MachO: copying from %p to %p a block of %llu bytes\n"
                                .as_ptr(),
                            (*oc).image.offset((*section).offset as isize) as *mut c_void,
                            secMem,
                            (*section).size,
                        );
                    }

                    let mut nstubs = numberOfStubsForSection(oc, sec_idx as u32);
                    let mut stub_space = stubSizeAarch64.wrapping_mul(nstubs as usize) as u32;

                    let mut mem = mmapForLinker(
                        (*section).size.wrapping_add(stub_space as u64) as usize,
                        MEM_READ_WRITE,
                        MAP_ANON as u32,
                        -1,
                        0,
                    );

                    if mem == MAP_FAILED {
                        sysErrorBelch(
                            c"failed to mmap allocated memory to load section %d. errno = %d"
                                .as_ptr(),
                            sec_idx,
                            *__error(),
                        );
                    }

                    memcpy(
                        mem,
                        (*oc).image.offset((*section).offset as isize) as *const c_void,
                        size as usize,
                    );

                    alloc = SECTION_MMAP;
                    mapped_offset = 0;

                    mapped_size =
                        roundUpToPage(size.wrapping_add(stub_space as StgWord) as usize) as StgWord;

                    start = mem;
                    mapped_start = mem;

                    addSection(
                        secArray.offset(sec_idx as isize) as *mut Section,
                        kind,
                        alloc,
                        start,
                        size,
                        mapped_offset,
                        mapped_start,
                        mapped_size,
                    );

                    (*(*secArray.offset(sec_idx as isize)).info).nstubs = 0;

                    let ref mut fresh12 = (*(*secArray.offset(sec_idx as isize)).info).stub_offset;
                    *fresh12 = (mem as *mut u8).offset(size as isize) as *mut c_void;
                    (*(*secArray.offset(sec_idx as isize)).info).stub_size = stub_space as usize;

                    let ref mut fresh13 = (*(*secArray.offset(sec_idx as isize)).info).stubs;
                    *fresh13 = null_mut::<Stub>();

                    addProddableBlock(&raw mut (*oc).proddables, start, (*section).size as usize);
                }
            }

            curMem = (secMem as *mut c_char).offset((*section).size as isize) as *mut c_void;

            let ref mut fresh14 = (*(*secArray.offset(sec_idx as isize)).info).macho_section;
            *fresh14 = section;

            let ref mut fresh15 = (*(*secArray.offset(sec_idx as isize)).info).relocation_info;
            *fresh15 = (*oc).image.offset((*section).reloff as isize) as *mut MachORelocationInfo;
            sec_n += 1;
        }

        seg_n += 1;
    }

    let mut i: usize = 0;

    while i < (*(*oc).info).n_macho_symbols {
        let mut s: *mut MachOSymbol =
            (*(*oc).info).macho_symbols.offset(i as isize) as *mut MachOSymbol;

        if N_SECT == (*(*s).nlist).n_type as i32 & N_TYPE {
            if NO_SECT == (*(*s).nlist).n_sect as i32 {
                barf(c"Symbol with N_SECT type, but no section.".as_ptr());
            }

            let mut n: u8 = ((*(*s).nlist).n_sect as i32 - 1) as u8;

            if !(0 == (*(*(*oc).info).macho_sections.offset(n as isize)).size) {
                (*s).addr = ((*(*oc).sections.offset(n as isize)).start as *mut u8)
                    .offset(-((*(*(*oc).info).macho_sections.offset(n as isize)).addr as isize))
                    .offset((*(*s).nlist).n_value as isize)
                    as *mut c_void;

                if (*s).addr.is_null() {
                    barf(
                        c"Failed to compute address for symbol %s".as_ptr(),
                        (*s).name,
                    );
                }
            }
        }

        i = i.wrapping_add(1);
    }

    (*oc).n_symbols = 0;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_0: usize = 0;

        while i_0 < (*(*oc).info).n_macho_symbols {
            if !((*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as i32 & N_STAB != 0) {
                if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as i32 & N_EXT != 0 {
                    if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as i32 & N_TYPE == N_UNDF
                        && (*(*(*oc).info).nlist.offset(i_0 as isize)).n_value != 0
                    {
                        commonSize = (commonSize as u64)
                            .wrapping_add((*(*(*oc).info).nlist.offset(i_0 as isize)).n_value)
                            as u64 as u64;

                        (*oc).n_symbols += 1;
                    } else if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as i32 & N_TYPE
                        == N_SECT
                    {
                        (*oc).n_symbols += 1;
                    }
                }
            }

            i_0 = i_0.wrapping_add(1);
        }
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"ocGetNames_MachO: %d external symbols\n".as_ptr(),
            (*oc).n_symbols,
        );
    }

    (*oc).symbols = stgMallocBytes(
        ((*oc).n_symbols as usize).wrapping_mul(size_of::<Symbol_t>() as usize),
        c"ocGetNames_MachO(oc->symbols)".as_ptr(),
    ) as *mut Symbol_t;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_1: usize = 0;

        while i_1 < (*(*oc).info).n_macho_symbols {
            let mut nm = (*(*(*oc).info).macho_symbols.offset(i_1 as isize)).name;

            if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as i32 & N_STAB != 0 {
                if RtsFlags.DebugFlags.linker_verbose {
                    debugBelch(c"ocGetNames_MachO: Skip STAB: %s\n".as_ptr(), nm);
                }
            } else if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as i32 & N_TYPE == N_SECT {
                if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as i32 & N_EXT != 0 {
                    if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_desc as i32 & N_WEAK_DEF != 0
                        && !lookupDependentSymbol(nm, oc, null_mut::<SymType>()).is_null()
                    {
                        if RtsFlags.DebugFlags.linker_verbose {
                            debugBelch(c"    weak: %s\n".as_ptr(), nm);
                        }
                    } else {
                        if RtsFlags.DebugFlags.linker_verbose {
                            debugBelch(c"ocGetNames_MachO: inserting %s\n".as_ptr(), nm);
                        }

                        let mut addr = (*(*(*oc).info).macho_symbols.offset(i_1 as isize)).addr;

                        let mut sym_type = SYM_TYPE_CODE;

                        ghciInsertSymbolTable(
                            (*oc).fileName,
                            symhash,
                            nm,
                            addr,
                            STRENGTH_NORMAL,
                            sym_type,
                            oc,
                        );

                        let ref mut fresh23 = (*(*oc).symbols.offset(curSymbol as isize)).name;
                        *fresh23 = nm;

                        let ref mut fresh24 = (*(*oc).symbols.offset(curSymbol as isize)).addr;
                        *fresh24 = addr;
                        (*(*oc).symbols.offset(curSymbol as isize)).r#type = sym_type;
                        curSymbol = curSymbol.wrapping_add(1);
                    }
                } else if RtsFlags.DebugFlags.linker_verbose {
                    debugBelch(
                        c"ocGetNames_MachO: \t...not external, skipping %s\n".as_ptr(),
                        nm,
                    );
                }
            } else if RtsFlags.DebugFlags.linker_verbose {
                debugBelch(
                    c"ocGetNames_MachO: \t...not defined in this section, skipping %s\n".as_ptr(),
                    nm,
                );
            }

            i_1 = i_1.wrapping_add(1);
        }
    }

    commonStorage = stgCallocBytes(
        1,
        commonSize as usize,
        c"ocGetNames_MachO(common symbols)".as_ptr(),
    ) as *mut c_void;

    commonCounter = commonStorage as u64;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_2: usize = 0;

        while i_2 < (*(*oc).info).n_macho_symbols {
            let mut nm_0 = (*(*(*oc).info).macho_symbols.offset(i_2 as isize)).name;
            let mut nlist: *mut MachONList =
                (*(*oc).info).nlist.offset(i_2 as isize) as *mut MachONList;

            if (*nlist).n_type as i32 & N_TYPE == N_UNDF
                && (*nlist).n_type as i32 & N_EXT != 0
                && (*nlist).n_value != 0
            {
                let mut sz = (*nlist).n_value as u64;
                (*nlist).n_value = commonCounter as u64;

                let ref mut fresh18 = (*(*(*oc).info).macho_symbols.offset(i_2 as isize)).addr;
                *fresh18 = commonCounter as *mut c_void as *mut c_void;

                let mut sym_type_0 = SYM_TYPE_CODE;

                if RtsFlags.DebugFlags.linker_verbose {
                    debugBelch(
                        c"ocGetNames_MachO: inserting common symbol: %s\n".as_ptr(),
                        nm_0,
                    );
                }

                ghciInsertSymbolTable(
                    (*oc).fileName,
                    symhash,
                    nm_0,
                    commonCounter as *mut c_void,
                    STRENGTH_NORMAL,
                    sym_type_0,
                    oc,
                );

                let ref mut fresh19 = (*(*oc).symbols.offset(curSymbol as isize)).name;
                *fresh19 = nm_0;

                let ref mut fresh20 = (*(*oc).symbols.offset(curSymbol as isize)).addr;
                *fresh20 = (*(*(*oc).info).macho_symbols.offset(i_2 as isize)).addr;
                curSymbol = curSymbol.wrapping_add(1);
                commonCounter = commonCounter.wrapping_add(sz);
            }

            i_2 = i_2.wrapping_add(1);
        }
    }

    findInternalGotRefs(oc);
    makeGot(oc);

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"ocGetNames_MachO: done\n".as_ptr());
    }

    return 1;
}

unsafe fn ocMprotect_MachO(mut oc: *mut ObjectCode) -> bool {
    let mut i = 0;

    while i < (*oc).n_segments {
        let mut segment: *mut Segment = (*oc).segments.offset(i as isize) as *mut Segment;

        if !((*segment).size == 0) {
            if (*segment).prot as u32 == SEGMENT_PROT_RX as i32 as u32 {
                mprotectForLinker((*segment).start, (*segment).size, MEM_READ_EXECUTE);
            }
        }

        i += 1;
    }

    let mut i_0 = 0;

    while i_0 < (*oc).n_sections {
        let mut section: *mut Section = (*oc).sections.offset(i_0 as isize) as *mut Section;

        if !((*section).size == 0) {
            if !((*section).alloc as u32 != SECTION_MMAP as i32 as u32) {
                if !((*section).alloc as u32 == SECTION_M32 as i32 as u32) {
                    match (*section).kind as u32 {
                        0 => {
                            mprotectForLinker(
                                (*section).mapped_start,
                                (*section).mapped_size as usize,
                                MEM_READ_EXECUTE,
                            );
                        }
                        _ => {}
                    }
                }
            }
        }

        i_0 += 1;
    }

    return true;
}

unsafe fn ocResolve_MachO(mut oc: *mut ObjectCode) -> i32 {
    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"ocResolve_MachO: %s start\n".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    if !(*(*oc).info).dsymCmd.is_null() {
        let mut indirectSyms = (*oc)
            .image
            .offset((*(*(*oc).info).dsymCmd).indirectsymoff as isize)
            as *mut u64;

        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ocResolve_MachO: resolving dsymLC\n".as_ptr());
        }

        let mut i = 0;

        while i < (*oc).n_sections {
            let mut sectionName: *const c_char =
                &raw mut (*(*(*oc).info).macho_sections.offset(i as isize)).sectname as *mut c_char;

            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"ocResolve_MachO: section %d/%d: %s\n".as_ptr(),
                    i,
                    (*oc).n_sections,
                    sectionName,
                );
            }

            if strcmp(sectionName, c"__la_symbol_ptr".as_ptr()) == 0
                || strcmp(sectionName, c"__la_sym_ptr2".as_ptr()) == 0
                || strcmp(sectionName, c"__la_sym_ptr3".as_ptr()) == 0
            {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0;
                }
            } else if strcmp(sectionName, c"__nl_symbol_ptr".as_ptr()) == 0
                || strcmp(sectionName, c"__pointers".as_ptr()) == 0
            {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0;
                }
            } else if strcmp(sectionName, c"__jump_table".as_ptr()) == 0 {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0;
                }
            } else if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"ocResolve_MachO: unknown section %d/%d\n".as_ptr(),
                    i,
                    (*oc).n_sections,
                );
            }

            i += 1;
        }
    }

    let mut i_0: usize = 0;

    while i_0 < (*(*oc).info).n_macho_symbols {
        let mut symbol: *mut MachOSymbol =
            (*(*oc).info).macho_symbols.offset(i_0 as isize) as *mut MachOSymbol;

        if needGotSlot(symbol) {
            if N_UNDF == (*(*symbol).nlist).n_type as i32 & N_TYPE {
                if (*symbol).addr.is_null() {
                    (*symbol).addr =
                        lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>());

                    if (*symbol).addr.is_null() {
                        errorBelch(c"Failed to lookup symbol: %s".as_ptr(), (*symbol).name);

                        return 0;
                    }
                }
            }

            if (*symbol).addr.is_null() {
                errorBelch(
                    c"Symbol %s has no address!\n".as_ptr(),
                    (*symbol).name as *mut c_char,
                );

                return 0;
            }

            if (*symbol).got_addr.is_null() {
                errorBelch(
                    c"Symbol %s has no Global Offset Table address!\n".as_ptr(),
                    (*symbol).name as *mut c_char,
                );

                return 0;
            }

            *((*symbol).got_addr as *mut u64) = (*symbol).addr as u64;
        }

        i_0 = i_0.wrapping_add(1);
    }

    let mut i_1 = 0;

    while i_1 < (*oc).n_sections {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"ocResolve_MachO: relocating section %d/%d\n".as_ptr(),
                i_1,
                (*oc).n_sections,
            );
        }

        if relocateSectionAarch64(oc, (*oc).sections.offset(i_1 as isize) as *mut Section) == 0 {
            return 0;
        }

        i_1 += 1;
    }

    if !ocMprotect_MachO(oc) {
        return 0;
    }

    return 1;
}

unsafe fn ocRunInit_MachO(mut oc: *mut ObjectCode) -> i32 {
    if (*(*oc).info).segCmd.is_null() {
        barf(c"ocRunInit_MachO: no segment load command".as_ptr());
    }

    let mut argc: i32 = 0;
    let mut envc: i32 = 0;
    let mut argv = null_mut::<*mut c_char>();
    let mut envv = null_mut::<*mut c_char>();
    getProgArgv(&raw mut argc, &raw mut argv);
    getProgEnvv(&raw mut envc, &raw mut envv);

    let mut i = 0;

    while i < (*oc).n_sections {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ocRunInit_MachO: checking section %d\n".as_ptr(), i);
        }

        if (*(*oc).sections.offset(i as isize)).kind as u32 == SECTIONKIND_INIT_ARRAY as i32 as u32
        {
            if RtsFlags.DebugFlags.linker {
                debugBelch(c"ocRunInit_MachO:     running mod init functions\n".as_ptr());
            }

            let mut init_startC = (*(*oc).sections.offset(i as isize)).start;
            let mut init = init_startC as *mut init_t;
            let mut init_end = (init_startC as *mut u8)
                .offset((*(*(*(*oc).sections.offset(i as isize)).info).macho_section).size as isize)
                as *mut init_t;

            let mut pn = 0;

            while init < init_end {
                if RtsFlags.DebugFlags.linker {
                    debugBelch(
                        c"ocRunInit_MachO:     function pointer %d at %p to %p\n".as_ptr(),
                        pn,
                        init as *mut c_void,
                        transmute::<init_t, *mut c_void>(*init),
                    );
                }

                (*init).expect("non-null function pointer")(argc, argv, envv);
                init = init.offset(1);
                pn += 1;
            }
        }

        i += 1;
    }

    freeProgEnvv(envc, envv as *mut *mut c_char);

    return 1;
}

unsafe fn ocRunFini_MachO(mut oc: *mut ObjectCode) -> i32 {
    if (*(*oc).info).segCmd.is_null() {
        barf(c"ocRunInit_MachO: no segment load command".as_ptr());
    }

    let mut i = 0;

    while i < (*oc).n_sections {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ocRunFini_MachO: checking section %d\n".as_ptr(), i);
        }

        if (*(*oc).sections.offset(i as isize)).kind as u32 == SECTIONKIND_FINI_ARRAY as i32 as u32
        {
            if RtsFlags.DebugFlags.linker {
                debugBelch(c"ocRunFini_MachO:     running mod fini functions\n".as_ptr());
            }

            let mut fini_startC = (*(*oc).sections.offset(i as isize)).start;
            let mut fini = fini_startC as *mut fini_t;
            let mut fini_end = (fini_startC as *mut u8)
                .offset((*(*(*(*oc).sections.offset(i as isize)).info).macho_section).size as isize)
                as *mut fini_t;

            let mut pn = 0;

            while fini < fini_end {
                if RtsFlags.DebugFlags.linker {
                    debugBelch(
                        c"ocRunFini_MachO:     function pointer %d at %p to %p\n".as_ptr(),
                        pn,
                        fini as *mut c_void,
                        transmute::<fini_t, *mut c_void>(*fini),
                    );
                }

                (*fini).expect("non-null function pointer")();
                fini = fini.offset(1);
                pn += 1;
            }
        }

        i += 1;
    }

    return 1;
}

unsafe fn machoGetMisalignment(mut f: *mut FILE) -> i32 {
    let mut header = mach_header_64 {
        magic: 0,
        cputype: 0,
        cpusubtype: 0,
        filetype: 0,
        ncmds: 0,
        sizeofcmds: 0,
        flags: 0,
        reserved: 0,
    };

    let mut misalignment: i32 = 0;

    let mut n = fread(
        &raw mut header as *mut c_void,
        size_of::<MachOHeader>() as usize,
        1,
        f,
    ) as usize;

    if n != 1 {
        barf(c"machoGetMisalignment: can't read the Mach-O header".as_ptr());
    }

    fseek(
        f,
        (size_of::<MachOHeader>() as usize).wrapping_neg() as i64,
        SEEK_CUR,
    );

    if header.magic != MH_MAGIC_64 as u32 {
        barf(
            c"Bad magic. Expected: %08x, got: %08x.".as_ptr(),
            MH_MAGIC_64,
            header.magic,
        );
    }

    misalignment =
        ((header.sizeofcmds as usize).wrapping_add(size_of::<MachOHeader>() as usize) & 0xf) as i32;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"mach-o misalignment %d\n".as_ptr(), misalignment);
    }

    return if misalignment != 0 {
        16 - misalignment
    } else {
        0
    };
}
