use crate::ffi::rts::messages::{barf, errorBelch, sysErrorBelch};
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
        1 as size_t,
        size_of::<ObjectCodeFormatInfo>() as size_t,
        b"ocInit_MachO(ObjectCodeFormatInfo)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut ObjectCodeFormatInfo;

    (*(*oc).info).header = (*oc).image as *mut MachOHeader;
    (*(*oc).info).symCmd = null_mut::<MachOSymtabCommand>();
    (*(*oc).info).segCmd = null_mut::<MachOSegmentCommand>();
    (*(*oc).info).dsymCmd = null_mut::<MachODsymtabCommand>();

    let mut lc = (*oc)
        .image
        .offset(size_of::<MachOHeader>() as usize as isize)
        as *mut MachOLoadCommand;

    let mut i: size_t = 0 as size_t;

    while i < (*(*(*oc).info).header).ncmds as size_t {
        if (*lc).cmd == LC_SEGMENT as uint32_t || (*lc).cmd == LC_SEGMENT_64 as uint32_t {
            (*(*oc).info).segCmd = lc as *mut MachOSegmentCommand;
        } else if (*lc).cmd == LC_SYMTAB as uint32_t {
            (*(*oc).info).symCmd = lc as *mut MachOSymtabCommand;
        } else if (*lc).cmd == LC_DYSYMTAB as uint32_t {
            (*(*oc).info).dsymCmd = lc as *mut MachODsymtabCommand;
        }

        lc = (lc as *mut c_char).offset((*lc).cmdsize as isize) as *mut MachOLoadCommand;
        i = i.wrapping_add(1);
    }

    if (*(*oc).info).segCmd.is_null() {
        barf(b"ocGetNames_MachO: no segment load command\0" as *const u8 as *const c_char);
    }

    (*(*oc).info).macho_sections =
        (*(*oc).info).segCmd.offset(1 as c_int as isize) as *mut MachOSection;
    (*oc).n_sections = (*(*(*oc).info).segCmd).nsects as c_int;

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

    (*(*oc).info).n_macho_symbols = 0 as size_t;
    (*(*oc).info).macho_symbols = null_mut::<MachOSymbol>();

    if !(*(*oc).info).nlist.is_null() {
        (*(*oc).info).n_macho_symbols = (*(*(*oc).info).symCmd).nsyms as size_t;

        (*(*oc).info).macho_symbols = stgCallocBytes(
            (*(*(*oc).info).symCmd).nsyms as size_t,
            size_of::<MachOSymbol>() as size_t,
            b"ocInit_MachO(MachOSymbol)\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut MachOSymbol;

        let mut i_0: uint32_t = 0 as uint32_t;

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
        if (*(*oc).info).n_macho_symbols > 0 as size_t {
            stgFree((*(*oc).info).macho_symbols as *mut c_void);
        }

        freeGot(oc);

        if !(*oc).sections.is_null() {
            let mut i = 0 as c_int;

            while i < (*oc).n_sections {
                freeStubs((*oc).sections.offset(i as isize) as *mut Section);
                i += 1;
            }
        }

        stgFree((*oc).info as *mut c_void);
        (*oc).info = null_mut::<ObjectCodeFormatInfo>();
    }
}

unsafe fn ocAllocateExtras_MachO(mut oc: *mut ObjectCode) -> c_int {
    if !(*(*oc).info).symCmd.is_null() {
        return ocAllocateExtras(
            oc,
            (*(*(*oc).info).symCmd).nsyms as c_int,
            0 as c_int,
            0 as c_int,
        );
    }

    return ocAllocateExtras(oc, 0 as c_int, 0 as c_int, 0 as c_int);
}

unsafe fn ocVerifyImage_MachO(mut oc: *mut ObjectCode) -> c_int {
    let mut image = (*oc).image;
    let mut header = image as *mut MachOHeader;

    if (*header).magic != MH_MAGIC_64 as uint32_t {
        errorBelch(
            b"Could not load image %s: bad magic!\n  Expected %08x (64bit), got %08x%s\n\0"
                as *const u8 as *const c_char,
            (*oc).fileName,
            MH_MAGIC_64,
            (*header).magic,
            if (*header).magic == MH_MAGIC as uint32_t {
                b" (32bit).\0" as *const u8 as *const c_char
            } else {
                b".\0" as *const u8 as *const c_char
            },
        );

        return 0 as c_int;
    }

    return 1 as c_int;
}

unsafe fn resolveImports(
    mut oc: *mut ObjectCode,
    mut sect: *mut MachOSection,
    mut indirectSyms: *mut c_ulong,
) -> c_int {
    let mut itemSize: size_t = 4 as size_t;
    let mut i = 0 as c_uint;

    while ((i as size_t).wrapping_mul(itemSize) as uint64_t) < (*sect).size {
        let mut indirectSymbolIndex =
            *indirectSyms.offset((*sect).reserved1.wrapping_add(i as uint32_t) as isize);

        let mut symbol: *mut MachOSymbol = (*(*oc).info)
            .macho_symbols
            .offset(indirectSymbolIndex as isize)
            as *mut MachOSymbol;

        let mut addr = NULL as *mut c_void;

        if (*(*symbol).nlist).n_type as c_int & N_TYPE == N_UNDF
            && (*(*symbol).nlist).n_type as c_int & N_EXT != 0
            && (*(*symbol).nlist).n_value != 0 as uint64_t
        {
            addr = (*(*symbol).nlist).n_value as *mut c_void;
        } else {
            addr = lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>());
        }

        if addr.is_null() {
            errorBelch(
                b"\nlookupSymbol failed in resolveImports\n%s: unknown symbol `%s'\0" as *const u8
                    as *const c_char,
                (*oc).fileName,
                (*symbol).name,
            );

            return 0 as c_int;
        }

        checkProddableBlock(
            &raw mut (*oc).proddables,
            ((*oc).image.offset((*sect).offset as isize) as *mut *mut c_void).offset(i as isize)
                as *mut c_void,
            size_of::<*mut c_void>() as size_t,
        );

        let ref mut fresh23 =
            *((*oc).image.offset((*sect).offset as isize) as *mut *mut c_void).offset(i as isize);
        *fresh23 = addr as *mut c_void;
        i = i.wrapping_add(1);
    }

    return 1 as c_int;
}

unsafe fn signExtend(mut val: uint64_t, mut bits: uint8_t) -> int64_t {
    return (val << 64 as c_int - bits as c_int) as int64_t >> 64 as c_int - bits as c_int;
}

unsafe fn isVectorOp(mut p: *mut uint32_t) -> bool {
    return *p & 0x4800000 as uint32_t == 0x4800000 as uint32_t;
}

unsafe fn isLoadStore(mut p: *mut uint32_t) -> bool {
    return *p & 0x3b000000 as uint32_t == 0x39000000 as uint32_t;
}

unsafe fn decodeAddend(
    mut oc: *mut ObjectCode,
    mut section: *mut Section,
    mut ri: *mut MachORelocationInfo,
) -> int64_t {
    let mut p =
        ((*section).start as *mut uint8_t).offset((*ri).r_address as isize) as *mut uint32_t;

    checkProddableBlock(
        &raw mut (*oc).proddables,
        p as *mut c_void,
        ((1 as c_int) << (*ri).r_length() as c_int) as size_t,
    );

    match (*ri).r_type() as c_int {
        0 => match (*ri).r_length() as c_int {
            0 => {
                return signExtend(
                    *(p as *mut uint8_t) as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            1 => {
                return signExtend(
                    *(p as *mut uint16_t) as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            2 => {
                return signExtend(
                    *p as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            3 => {
                return signExtend(
                    *(p as *mut uint64_t),
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            _ => {
                barf(
                    b"Unsupported r_length (%d) for UNSIGNED relocation\0" as *const u8
                        as *const c_char,
                    (*ri).r_length() as c_int,
                );
            }
        },
        1 => match (*ri).r_length() as c_int {
            0 => {
                return signExtend(
                    *(p as *mut uint8_t) as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            1 => {
                return signExtend(
                    *(p as *mut uint16_t) as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            2 => {
                return signExtend(
                    *p as uint64_t,
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            3 => {
                return signExtend(
                    *(p as *mut uint64_t),
                    ((8 as c_int) << (*ri).r_length() as c_int) as uint8_t,
                );
            }
            _ => {
                barf(
                    b"Unsupported r_length (%d) for SUBTRACTOR relocation\0" as *const u8
                        as *const c_char,
                    (*ri).r_length() as c_int,
                );
            }
        },
        2 => {
            return signExtend(
                ((*p & 0x3ffffff as uint32_t) << 2 as c_int) as uint64_t,
                28 as uint8_t,
            );
        }
        3 | 5 => {
            return signExtend(
                ((*p & 0x60000000 as uint32_t) >> 29 as c_int
                    | ((*p & 0x1ffffe0 as uint32_t) >> 3 as c_int) << 12 as c_int)
                    as uint64_t,
                33 as uint8_t,
            );
        }
        4 | 6 => {
            let mut a: int64_t = ((*p & 0x3ffc00 as uint32_t) >> 10 as c_int) as int64_t;
            let mut shift = 0 as c_int;

            if isLoadStore(p) {
                shift = (*p >> 30 as c_int & 0x3 as uint32_t) as c_int;

                if 0 as c_int == shift && isVectorOp(p) as c_int != 0 {
                    shift = 4 as c_int;
                }
            }

            return a << shift;
        }
        _ => {}
    }

    barf(
        b"unsupported relocation type: %d\n\0" as *const u8 as *const c_char,
        (*ri).r_type() as c_int,
    );
}

#[inline]
unsafe fn fitsBits(mut bits: size_t, mut value: int64_t) -> bool {
    if bits == 64 as size_t {
        return r#true != 0;
    }

    if bits > 64 as size_t {
        barf(
            b"fits_bits with %zu bits and an 64bit integer!\0" as *const u8 as *const c_char,
            bits,
        );
    }

    return 0 as int64_t == value >> bits || -(1 as c_int) as int64_t == value >> bits;
}

unsafe fn encodeAddend(
    mut oc: *mut ObjectCode,
    mut section: *mut Section,
    mut ri: *mut MachORelocationInfo,
    mut addend: int64_t,
    mut symbol: *mut MachOSymbol,
) {
    let mut p =
        ((*section).start as *mut uint8_t).offset((*ri).r_address as isize) as *mut uint32_t;

    checkProddableBlock(
        &raw mut (*oc).proddables,
        p as *mut c_void,
        ((1 as c_int) << (*ri).r_length() as c_int) as size_t,
    );

    let mut symbol_name = if !symbol.is_null() && !(*symbol).name.is_null() {
        (*symbol).name as *mut c_char as *const c_char
    } else {
        b"<unknown>\0" as *const u8 as *const c_char
    };

    let mut file_name = if !(*oc).fileName.is_null() {
        (*oc).fileName as *mut c_char as *const c_char
    } else {
        b"<unknown>\0" as *const u8 as *const c_char
    };

    match (*ri).r_type() as c_int {
        0 => {
            if !fitsBits(
                ((8 as c_int) << (*ri).r_length() as c_int) as size_t,
                addend,
            ) {
                let mut library_info: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    b"Relocation out of range for UNSIGNED in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s\0"
                        as *const u8 as *const c_char,
                    file_name,
                    symbol_name,
                    addend as c_longlong,
                    (*ri).r_address as c_longlong,
                    if !library_info.is_null() {
                        library_info as *mut c_char
                            as *const c_char
                    } else {
                        b"<unknown>\0" as *const u8 as *const c_char
                    },
                );
            }

            match (*ri).r_length() as c_int {
                0 => {
                    *(p as *mut uint8_t) = addend as uint8_t;
                }
                1 => {
                    *(p as *mut uint16_t) = addend as uint16_t;
                }
                2 => {
                    *p = addend as uint32_t;
                }
                3 => {
                    *(p as *mut uint64_t) = addend as uint64_t;
                }
                _ => {
                    barf(
                        b"Unsupported r_length (%d) for UNSIGNED relocation\0" as *const u8
                            as *const c_char,
                        (*ri).r_length() as c_int,
                    );
                }
            }

            return;
        }
        1 => {
            if !fitsBits(
                ((8 as c_int) << (*ri).r_length() as c_int) as size_t,
                addend,
            ) {
                let mut library_info_0: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    b"Relocation out of range for SUBTRACTOR in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s\0"
                        as *const u8 as *const c_char,
                    file_name,
                    symbol_name,
                    addend as c_longlong,
                    (*ri).r_address as c_longlong,
                    if !library_info_0.is_null() {
                        library_info_0 as *mut c_char
                            as *const c_char
                    } else {
                        b"<unknown>\0" as *const u8 as *const c_char
                    },
                );
            }

            match (*ri).r_length() as c_int {
                0 => {
                    *(p as *mut uint8_t) = addend as uint8_t;
                }
                1 => {
                    *(p as *mut uint16_t) = addend as uint16_t;
                }
                2 => {
                    *p = addend as uint32_t;
                }
                3 => {
                    *(p as *mut uint64_t) = addend as uint64_t;
                }
                _ => {
                    barf(
                        b"Unsupported r_length (%d) for SUBTRACTOR relocation\0" as *const u8
                            as *const c_char,
                        (*ri).r_length() as c_int,
                    );
                }
            }

            return;
        }
        2 => {
            if !fitsBits(26 as size_t, addend >> 2 as c_int) {
                let mut library_info_1: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    b"Relocation target for BRANCH26 out of range in %s: symbol '%s', addend 0x%llx (0x%llx >> 2), address 0x%llx, library: %s\0"
                        as *const u8 as *const c_char,
                    file_name,
                    symbol_name,
                    addend as c_longlong,
                    (addend >> 2 as c_int) as c_longlong,
                    (*ri).r_address as c_longlong,
                    if !library_info_1.is_null() {
                        library_info_1 as *mut c_char
                            as *const c_char
                    } else {
                        b"<unknown>\0" as *const u8 as *const c_char
                    },
                );
            }

            *p = *p & 0xfc000000 as uint32_t
                | (addend >> 2 as c_int) as uint32_t & 0x3ffffff as uint32_t;
            return;
        }
        3 | 5 => {
            if !fitsBits(21 as size_t, addend >> 12 as c_int) {
                let mut reloc_type = if (*ri).r_type() as c_int == ARM64_RELOC_PAGE21 as c_int {
                    b"PAGE21\0" as *const u8 as *const c_char
                } else {
                    b"GOT_LOAD_PAGE21\0" as *const u8 as *const c_char
                };

                let mut library_info_2: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    b"Relocation target for %s out of range in %s: symbol '%s', addend 0x%llx (0x%llx >> 12), address 0x%llx, library: %s\0"
                        as *const u8 as *const c_char,
                    reloc_type,
                    file_name,
                    symbol_name,
                    addend as c_longlong,
                    (addend >> 12 as c_int) as c_longlong,
                    (*ri).r_address as c_longlong,
                    if !library_info_2.is_null() {
                        library_info_2 as *mut c_char
                            as *const c_char
                    } else {
                        b"<unknown>\0" as *const u8 as *const c_char
                    },
                );
            }

            *p = *p & 0x9f00001f as uint32_t
                | (addend << 17 as c_int & 0x60000000 as int64_t) as uint32_t
                | (addend >> 9 as c_int & 0xffffe0 as int64_t) as uint32_t;
            return;
        }
        4 | 6 => {
            if !fitsBits(12 as size_t, addend) {
                let mut library_info_3: *const c_char = if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                };

                barf(
                    b"Relocation target for PAGEOFF12 out of range in %s: symbol '%s', addend 0x%llx, address 0x%llx, library: %s\0"
                        as *const u8 as *const c_char,
                    file_name,
                    symbol_name,
                    addend as c_longlong,
                    (*ri).r_address as c_longlong,
                    if !library_info_3.is_null() {
                        library_info_3 as *mut c_char
                            as *const c_char
                    } else {
                        b"<unknown>\0" as *const u8 as *const c_char
                    },
                );
            }

            let mut shift = 0 as c_int;

            if isLoadStore(p) {
                shift = (*p >> 30 as c_int & 0x3 as uint32_t) as c_int;

                if 0 as c_int == shift && isVectorOp(p) as c_int != 0 {
                    shift = 4 as c_int;
                }
            }

            *p = *p & 0xffc003ff as uint32_t
                | (addend >> shift << 10 as c_int) as uint32_t & 0x3ffc00 as uint32_t;
            return;
        }
        _ => {}
    }

    barf(
        b"unsupported relocation type: %d\n\0" as *const u8 as *const c_char,
        (*ri).r_type() as c_int,
    );
}

unsafe fn findInternalGotRefs(mut oc: *mut ObjectCode) {
    let mut curSection = 0 as c_int;

    while curSection < (*oc).n_sections {
        let mut sect: *mut Section = (*oc).sections.offset(curSection as isize) as *mut Section;

        if !(*sect).info.is_null() {
            let mut msect = (*(*sect).info).macho_section;
            let mut relocs = (*(*sect).info).relocation_info;
            let mut i: uint32_t = 0 as uint32_t;

            while i < (*msect).nreloc {
                let mut ri: *mut MachORelocationInfo =
                    relocs.offset(i as isize) as *mut MachORelocationInfo;

                if isGotLoad(ri as *mut relocation_info) {
                    let mut symbol: *mut MachOSymbol = (*(*oc).info)
                        .macho_symbols
                        .offset((*ri).r_symbolnum() as isize)
                        as *mut MachOSymbol;
                    (*symbol).needs_got = r#true != 0;
                }

                i = i.wrapping_add(1);
            }
        }

        curSection += 1;
    }
}

unsafe fn isGotLoad(mut ri: *mut relocation_info) -> bool {
    return (*ri).r_type() as c_int == ARM64_RELOC_GOT_LOAD_PAGE21 as c_int
        || (*ri).r_type() as c_int == ARM64_RELOC_GOT_LOAD_PAGEOFF12 as c_int;
}

unsafe fn needGotSlot(mut symbol: *mut MachOSymbol) -> bool {
    if (*symbol).needs_got {
        return r#true != 0;
    }

    return (*(*symbol).nlist).n_type as c_int & N_EXT != 0
        && (N_UNDF == (*(*symbol).nlist).n_type as c_int & N_TYPE
            || NO_SECT != (*(*symbol).nlist).n_sect as c_int);
}

unsafe fn makeGot(mut oc: *mut ObjectCode) -> bool {
    let mut got_slots: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;

    while i < (*(*oc).info).n_macho_symbols {
        if needGotSlot((*(*oc).info).macho_symbols.offset(i as isize) as *mut MachOSymbol) {
            got_slots = got_slots.wrapping_add(1 as size_t);
        }

        i = i.wrapping_add(1);
    }

    if got_slots > 0 as size_t {
        (*(*oc).info).got_size = got_slots.wrapping_mul(size_of::<*mut c_void>() as size_t);
        (*(*oc).info).got_start = mmapAnonForLinker((*(*oc).info).got_size);

        if (*(*oc).info).got_start.is_null() {
            barf(
                b"MAP_FAILED. errno=%d\0" as *const u8 as *const c_char,
                *__error(),
            );
        }

        let mut slot: size_t = 0 as size_t;
        let mut i_0: size_t = 0 as size_t;

        while i_0 < (*(*oc).info).n_macho_symbols {
            if needGotSlot((*(*oc).info).macho_symbols.offset(i_0 as isize) as *mut MachOSymbol) {
                let fresh21 = slot;
                slot = slot.wrapping_add(1);

                let ref mut fresh22 = (*(*(*oc).info).macho_symbols.offset(i_0 as isize)).got_addr;
                *fresh22 = ((*(*oc).info).got_start as *mut uint8_t)
                    .offset(fresh21.wrapping_mul(size_of::<*mut c_void>() as size_t) as isize)
                    as *mut c_void;
            }

            i_0 = i_0.wrapping_add(1);
        }
    }

    return EXIT_SUCCESS != 0;
}

unsafe fn freeGot(mut oc: *mut ObjectCode) {
    if !(*(*oc).info).got_start.is_null() && (*(*oc).info).got_size > 0 as size_t {
        munmapForLinker(
            (*(*oc).info).got_start,
            (*(*oc).info).got_size,
            b"freeGot\0" as *const u8 as *const c_char,
        );
    }

    (*(*oc).info).got_start = NULL;
    (*(*oc).info).got_size = 0 as size_t;
}

unsafe fn symbol_value(mut oc: *mut ObjectCode, mut symbol: *mut MachOSymbol) -> uint64_t {
    let mut value: uint64_t = 0 as uint64_t;

    if (*(*symbol).nlist).n_type as c_int & N_EXT != 0 {
        value = lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>()) as uint64_t;

        if value == 0 {
            barf(
                b"Could not lookup symbol: %s!\0" as *const u8 as *const c_char,
                (*symbol).name,
            );
        }
    } else {
        value = (*symbol).addr as uint64_t;
    }

    return value;
}

unsafe fn relocateSectionAarch64(mut oc: *mut ObjectCode, mut section: *mut Section) -> c_int {
    if (*section).size == 0 as StgWord {
        return 1 as c_int;
    }

    let mut explicit_addend: int64_t = 0 as int64_t;
    let mut nreloc: size_t = (*(*(*section).info).macho_section).nreloc as size_t;
    let mut i: size_t = 0 as size_t;

    while i < nreloc {
        let mut ri: *mut MachORelocationInfo =
            (*(*section).info).relocation_info.offset(i as isize) as *mut MachORelocationInfo;

        match (*ri).r_type() as c_int {
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
                    value.wrapping_add(addend as uint64_t) as int64_t,
                    symbol,
                );
            }
            1 => {
                if !(i.wrapping_add(1 as size_t) < nreloc)
                    || !((*(*(*section).info)
                        .relocation_info
                        .offset(i.wrapping_add(1 as size_t) as isize))
                    .r_type() as c_int
                        == ARM64_RELOC_UNSIGNED as c_int)
                {
                    barf(
                        b"SUBTRACTOR relocation *must* be followed by UNSIGNED relocation.\0"
                            as *const u8 as *const c_char,
                    );
                }

                let mut symbol1: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut sub_value = symbol_value(oc, symbol1);
                let mut ri2: *mut MachORelocationInfo = (*(*section).info)
                    .relocation_info
                    .offset(i.wrapping_add(1 as size_t) as isize)
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
                    (addend_0 as uint64_t)
                        .wrapping_sub(sub_value)
                        .wrapping_add(add_value) as int64_t,
                    symbol1,
                );

                i = i.wrapping_add(1 as size_t);
            }
            2 => {
                let mut symbol_0: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_1 = decodeAddend(oc, section, ri);
                let mut pc: uint64_t =
                    ((*section).start as uint64_t).wrapping_add((*ri).r_address as uint64_t);

                let mut value_0: uint64_t = 0 as uint64_t;

                if (*(*symbol_0).nlist).n_type as c_int & N_EXT != 0 {
                    value_0 = lookupDependentSymbol((*symbol_0).name, oc, null_mut::<SymType>())
                        as uint64_t;

                    if value_0 == 0 {
                        barf(
                            b"Could not lookup symbol: %s!\0" as *const u8 as *const c_char,
                            (*symbol_0).name,
                        );
                    }
                } else {
                    value_0 = (*symbol_0).addr as uint64_t;
                }

                if value_0.wrapping_sub(pc).wrapping_add(addend_1 as uint64_t)
                    >> 2 as c_int + 26 as c_int - 1 as c_int
                    != 0
                {
                    if findStub(section, &raw mut value_0 as *mut *mut c_void, 0 as uint8_t) {
                        if makeStub(section, &raw mut value_0 as *mut *mut c_void, 0 as uint8_t) {
                            barf(b"could not find or make stub\0" as *const u8 as *const c_char);
                        }
                    }
                }

                encodeAddend(
                    oc,
                    section,
                    ri,
                    value_0.wrapping_sub(pc).wrapping_add(addend_1 as uint64_t) as int64_t,
                    symbol_0,
                );
            }
            3 | 5 => {
                let mut symbol_1: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_2 = decodeAddend(oc, section, ri);

                if !(explicit_addend == 0 as int64_t || addend_2 == 0 as int64_t) {
                    barf(
                        b"explicit_addend and addend can't be set at the same time.\0" as *const u8
                            as *const c_char,
                    );
                }

                let mut pc_0: uint64_t =
                    ((*section).start as uint64_t).wrapping_add((*ri).r_address as uint64_t);

                let mut value_1: uint64_t = (if isGotLoad(ri as *mut relocation_info) as c_int != 0
                {
                    (*symbol_1).got_addr
                } else {
                    (*symbol_1).addr as *mut c_void
                }) as uint64_t;

                encodeAddend(
                    oc,
                    section,
                    ri,
                    (value_1
                        .wrapping_add(addend_2 as uint64_t)
                        .wrapping_add(explicit_addend as uint64_t)
                        & -(4096 as c_int) as uint64_t)
                        .wrapping_sub(pc_0 & -(4096 as c_int) as uint64_t)
                        as int64_t,
                    symbol_1,
                );

                explicit_addend = 0 as int64_t;
            }
            4 | 6 => {
                let mut symbol_2: *mut MachOSymbol = (*(*oc).info)
                    .macho_symbols
                    .offset((*ri).r_symbolnum() as isize)
                    as *mut MachOSymbol;

                let mut addend_3 = decodeAddend(oc, section, ri);

                if !(explicit_addend == 0 as int64_t || addend_3 == 0 as int64_t) {
                    barf(
                        b"explicit_addend and addend can't be set at the same time.\0" as *const u8
                            as *const c_char,
                    );
                }

                let mut value_2: uint64_t = (if isGotLoad(ri as *mut relocation_info) as c_int != 0
                {
                    (*symbol_2).got_addr
                } else {
                    (*symbol_2).addr as *mut c_void
                }) as uint64_t;

                encodeAddend(
                    oc,
                    section,
                    ri,
                    (0xfff as uint64_t
                        & value_2
                            .wrapping_add(addend_3 as uint64_t)
                            .wrapping_add(explicit_addend as uint64_t))
                        as int64_t,
                    symbol_2,
                );

                explicit_addend = 0 as int64_t;
            }
            10 => {
                explicit_addend = signExtend((*ri).r_symbolnum() as uint64_t, 24 as uint8_t);

                if !(i.wrapping_add(1 as size_t) < nreloc)
                    || !((*(*(*section).info)
                        .relocation_info
                        .offset(i.wrapping_add(1 as size_t) as isize))
                    .r_type() as c_int
                        == ARM64_RELOC_PAGE21 as c_int
                        || (*(*(*section).info)
                            .relocation_info
                            .offset(i.wrapping_add(1 as size_t) as isize))
                        .r_type() as c_int
                            == ARM64_RELOC_PAGEOFF12 as c_int)
                {
                    barf(
                        b"ADDEND relocation *must* be followed by PAGE or PAGEOFF relocation\0"
                            as *const u8 as *const c_char,
                    );
                }
            }
            _ => {
                barf(
                    b"Relocation of type: %d not (yet) supported!\n\0" as *const u8
                        as *const c_char,
                    (*ri).r_type() as c_int,
                );
            }
        }

        i = i.wrapping_add(1);
    }

    return 1 as c_int;
}

unsafe fn getSectionKind_MachO(mut section: *mut MachOSection) -> SectionKind {
    let mut s_type: uint8_t = ((*section).flags & SECTION_TYPE as uint32_t) as uint8_t;

    if s_type as c_int == S_MOD_INIT_FUNC_POINTERS {
        return SECTIONKIND_INIT_ARRAY;
    } else if s_type as c_int == S_MOD_TERM_FUNC_POINTERS {
        return SECTIONKIND_FINI_ARRAY;
    } else if 0 as c_int
        == strcmp(
            &raw mut (*section).segname as *mut c_char,
            b"__TEXT\0" as *const u8 as *const c_char,
        )
    {
        return SECTIONKIND_CODE_OR_RODATA;
    } else if 0 as c_int
        == strcmp(
            &raw mut (*section).segname as *mut c_char,
            b"__DATA\0" as *const u8 as *const c_char,
        )
    {
        return SECTIONKIND_RWDATA;
    } else {
        return SECTIONKIND_OTHER;
    };
}

unsafe fn ocBuildSegments_MachO(mut oc: *mut ObjectCode) -> c_int {
    let mut n_rxSections = 0 as c_int;
    let mut size_rxSegment: size_t = 0 as size_t;
    let mut rxSegment = null_mut::<Segment>();
    let mut n_rwSections = 0 as c_int;
    let mut size_rwSegment: size_t = 0 as size_t;
    let mut rwSegment = null_mut::<Segment>();
    let mut n_gbZerofills = 0 as c_int;
    let mut size_gbZerofillSegment: size_t = 0 as size_t;
    let mut gbZerofillSegment = null_mut::<Segment>();
    let mut n_activeSegments = 0 as c_int;
    let mut curSegment = 0 as c_int;
    let mut size_compound: size_t = 0;
    let mut segments = null_mut::<Segment>();
    let mut mem = NULL;
    let mut curMem = NULL;
    let mut i = 0 as c_int;

    while i < (*oc).n_sections {
        let mut macho: *mut MachOSection =
            (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection;

        if !(0 as uint64_t == (*macho).size) {
            let mut alignment: size_t = ((1 as c_int) << (*macho).align) as size_t;

            if S_GB_ZEROFILL as uint32_t == (*macho).flags & SECTION_TYPE as uint32_t {
                size_gbZerofillSegment = roundUpToAlign(size_gbZerofillSegment, alignment);
                size_gbZerofillSegment = (size_gbZerofillSegment as uint64_t)
                    .wrapping_add((*macho).size) as size_t
                    as size_t;
                n_gbZerofills += 1;
            } else if getSectionKind_MachO(macho) as c_uint
                == SECTIONKIND_CODE_OR_RODATA as c_int as c_uint
            {
                size_rxSegment = roundUpToAlign(size_rxSegment, alignment);
                size_rxSegment =
                    (size_rxSegment as uint64_t).wrapping_add((*macho).size) as size_t as size_t;
                n_rxSections += 1;
            } else {
                size_rwSegment = roundUpToAlign(size_rwSegment, alignment);
                size_rwSegment =
                    (size_rwSegment as uint64_t).wrapping_add((*macho).size) as size_t as size_t;
                n_rwSections += 1;
            }
        }

        i += 1;
    }

    size_compound = roundUpToPage(size_rxSegment)
        .wrapping_add(roundUpToPage(size_rwSegment))
        .wrapping_add(roundUpToPage(size_gbZerofillSegment));

    if n_rxSections > 0 as c_int {
        n_activeSegments += 1;
    }

    if n_rwSections > 0 as c_int {
        n_activeSegments += 1;
    }

    if n_gbZerofills > 0 as c_int {
        n_activeSegments += 1;
    }

    if 0 as size_t == size_compound {
        return 1 as c_int;
    }

    mem = mmapAnonForLinker(size_compound);

    if mem.is_null() {
        return 0 as c_int;
    }

    segments = stgCallocBytes(
        n_activeSegments as size_t,
        size_of::<Segment>() as size_t,
        b"ocBuildSegments_MachO(segments)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Segment;

    curMem = mem;

    if n_rxSections > 0 as c_int {
        rxSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            rxSegment,
            curMem,
            roundUpToPage(size_rxSegment),
            SEGMENT_PROT_RX,
            n_rxSections,
        );

        curMem = (curMem as *mut c_char).offset((*rxSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    if n_rwSections > 0 as c_int {
        rwSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            rwSegment,
            curMem,
            roundUpToPage(size_rwSegment),
            SEGMENT_PROT_RWO,
            n_rwSections,
        );

        curMem = (curMem as *mut c_char).offset((*rwSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    if n_gbZerofills > 0 as c_int {
        gbZerofillSegment = segments.offset(curSegment as isize) as *mut Segment;

        initSegment(
            gbZerofillSegment,
            curMem,
            roundUpToPage(size_gbZerofillSegment),
            SEGMENT_PROT_RWO,
            n_gbZerofills,
        );

        curMem = (curMem as *mut c_char).offset((*gbZerofillSegment).size as isize) as *mut c_void;
        curSegment += 1;
    }

    let mut i_0 = 0 as c_int;
    let mut rx = 0 as c_int;
    let mut rw = 0 as c_int;
    let mut gb = 0 as c_int;

    while i_0 < (*oc).n_sections {
        let mut macho_0: *mut MachOSection =
            (*(*oc).info).macho_sections.offset(i_0 as isize) as *mut MachOSection;

        if !(0 as uint64_t == (*macho_0).size) {
            if S_GB_ZEROFILL as uint32_t == (*macho_0).flags & SECTION_TYPE as uint32_t {
                let fresh9 = gb;
                gb = gb + 1;
                *(*gbZerofillSegment).sections_idx.offset(fresh9 as isize) = i_0;
            } else if getSectionKind_MachO(macho_0) as c_uint
                == SECTIONKIND_CODE_OR_RODATA as c_int as c_uint
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

    return 1 as c_int;
}

unsafe fn ocGetNames_MachO(mut oc: *mut ObjectCode) -> c_int {
    let mut curSymbol = 0 as c_uint;
    let mut commonSize = 0 as c_ulong;
    let mut commonStorage = NULL as *mut c_void;
    let mut commonCounter: c_ulong = 0;
    let mut secArray = null_mut::<Section>();

    secArray = stgCallocBytes(
        (*(*(*oc).info).segCmd).nsects as size_t,
        size_of::<Section>() as size_t,
        b"ocGetNames_MachO(sections)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Section;

    (*oc).sections = secArray;

    if (ocBuildSegments_MachO(oc) != 0) as c_int as c_long != 0 {
    } else {
        barf(b"ocGetNames_MachO: failed to build segments\n\0" as *const u8 as *const c_char);
    }

    let mut seg_n = 0 as c_int;

    while seg_n < (*oc).n_segments {
        let mut segment: *mut Segment = (*oc).segments.offset(seg_n as isize) as *mut Segment;
        let mut curMem = (*segment).start;
        let mut sec_n = 0 as c_int;

        while sec_n < (*segment).n_sections {
            let mut sec_idx = *(*segment).sections_idx.offset(sec_n as isize);
            let mut section: *mut MachOSection =
                (*(*oc).info).macho_sections.offset(sec_idx as isize) as *mut MachOSection;

            let mut alignment: size_t = ((1 as c_int) << (*section).align) as size_t;
            let mut kind = getSectionKind_MachO(section);
            let mut alloc = SECTION_NOMEM;
            let mut start = NULL;
            let mut mapped_start = NULL;
            let mut mapped_size: StgWord = 0 as StgWord;
            let mut mapped_offset: StgWord = 0 as StgWord;
            let mut size: StgWord = (*section).size as StgWord;
            let mut secMem = roundUpToAlign(curMem as size_t, alignment) as *mut c_void;
            start = secMem;

            match (*section).flags & SECTION_TYPE as uint32_t {
                1 | 12 => {
                    memset(secMem, 0 as c_int, (*section).size as size_t);

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
                    let mut nstubs = numberOfStubsForSection(oc, sec_idx as c_uint);
                    let mut stub_space = stubSizeAarch64.wrapping_mul(nstubs as size_t) as c_uint;

                    let mut mem = mmapForLinker(
                        (*section).size.wrapping_add(stub_space as uint64_t) as size_t,
                        MEM_READ_WRITE,
                        MAP_ANON as uint32_t,
                        -(1 as c_int),
                        0 as c_int,
                    );

                    if mem == MAP_FAILED {
                        sysErrorBelch(
                            b"failed to mmap allocated memory to load section %d. errno = %d\0"
                                as *const u8 as *const c_char,
                            sec_idx,
                            *__error(),
                        );
                    }

                    memcpy(
                        mem,
                        (*oc).image.offset((*section).offset as isize) as *const c_void,
                        size as size_t,
                    );

                    alloc = SECTION_MMAP;
                    mapped_offset = 0 as StgWord;
                    mapped_size = roundUpToPage(size.wrapping_add(stub_space as StgWord) as size_t)
                        as StgWord;
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

                    (*(*secArray.offset(sec_idx as isize)).info).nstubs = 0 as size_t;

                    let ref mut fresh12 = (*(*secArray.offset(sec_idx as isize)).info).stub_offset;
                    *fresh12 = (mem as *mut uint8_t).offset(size as isize) as *mut c_void;
                    (*(*secArray.offset(sec_idx as isize)).info).stub_size = stub_space as size_t;

                    let ref mut fresh13 = (*(*secArray.offset(sec_idx as isize)).info).stubs;
                    *fresh13 = null_mut::<Stub>();
                    addProddableBlock(&raw mut (*oc).proddables, start, (*section).size as size_t);
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

    let mut i: size_t = 0 as size_t;

    while i < (*(*oc).info).n_macho_symbols {
        let mut s: *mut MachOSymbol =
            (*(*oc).info).macho_symbols.offset(i as isize) as *mut MachOSymbol;

        if N_SECT == (*(*s).nlist).n_type as c_int & N_TYPE {
            if NO_SECT == (*(*s).nlist).n_sect as c_int {
                barf(b"Symbol with N_SECT type, but no section.\0" as *const u8 as *const c_char);
            }

            let mut n: uint8_t = ((*(*s).nlist).n_sect as c_int - 1 as c_int) as uint8_t;

            if !(0 as uint64_t == (*(*(*oc).info).macho_sections.offset(n as isize)).size) {
                (*s).addr = ((*(*oc).sections.offset(n as isize)).start as *mut uint8_t)
                    .offset(-((*(*(*oc).info).macho_sections.offset(n as isize)).addr as isize))
                    .offset((*(*s).nlist).n_value as isize)
                    as *mut c_void;

                if (*s).addr.is_null() {
                    barf(
                        b"Failed to compute address for symbol %s\0" as *const u8 as *const c_char,
                        (*s).name,
                    );
                }
            }
        }

        i = i.wrapping_add(1);
    }

    (*oc).n_symbols = 0 as c_int;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_0: size_t = 0 as size_t;

        while i_0 < (*(*oc).info).n_macho_symbols {
            if !((*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as c_int & N_STAB != 0) {
                if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as c_int & N_EXT != 0 {
                    if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as c_int & N_TYPE
                        == N_UNDF
                        && (*(*(*oc).info).nlist.offset(i_0 as isize)).n_value != 0 as uint64_t
                    {
                        commonSize = (commonSize as uint64_t)
                            .wrapping_add((*(*(*oc).info).nlist.offset(i_0 as isize)).n_value)
                            as c_ulong as c_ulong;
                        (*oc).n_symbols += 1;
                    } else if (*(*(*oc).info).nlist.offset(i_0 as isize)).n_type as c_int & N_TYPE
                        == N_SECT
                    {
                        (*oc).n_symbols += 1;
                    }
                }
            }

            i_0 = i_0.wrapping_add(1);
        }
    }

    (*oc).symbols = stgMallocBytes(
        ((*oc).n_symbols as size_t).wrapping_mul(size_of::<Symbol_t>() as size_t),
        b"ocGetNames_MachO(oc->symbols)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut Symbol_t;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_1: size_t = 0 as size_t;

        while i_1 < (*(*oc).info).n_macho_symbols {
            let mut nm = (*(*(*oc).info).macho_symbols.offset(i_1 as isize)).name;

            if !((*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as c_int & N_STAB != 0) {
                if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as c_int & N_TYPE == N_SECT {
                    if (*(*(*oc).info).nlist.offset(i_1 as isize)).n_type as c_int & N_EXT != 0 {
                        if !((*(*(*oc).info).nlist.offset(i_1 as isize)).n_desc as c_int
                            & N_WEAK_DEF
                            != 0
                            && !lookupDependentSymbol(nm, oc, null_mut::<SymType>()).is_null())
                        {
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

                            let ref mut fresh16 = (*(*oc).symbols.offset(curSymbol as isize)).name;
                            *fresh16 = nm;

                            let ref mut fresh17 = (*(*oc).symbols.offset(curSymbol as isize)).addr;
                            *fresh17 = addr;
                            (*(*oc).symbols.offset(curSymbol as isize)).r#type = sym_type;
                            curSymbol = curSymbol.wrapping_add(1);
                        }
                    }
                }
            }

            i_1 = i_1.wrapping_add(1);
        }
    }

    commonStorage = stgCallocBytes(
        1 as size_t,
        commonSize as size_t,
        b"ocGetNames_MachO(common symbols)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_void;

    commonCounter = commonStorage as c_ulong;

    if !(*(*oc).info).symCmd.is_null() {
        let mut i_2: size_t = 0 as size_t;

        while i_2 < (*(*oc).info).n_macho_symbols {
            let mut nm_0 = (*(*(*oc).info).macho_symbols.offset(i_2 as isize)).name;
            let mut nlist: *mut MachONList =
                (*(*oc).info).nlist.offset(i_2 as isize) as *mut MachONList;

            if (*nlist).n_type as c_int & N_TYPE == N_UNDF
                && (*nlist).n_type as c_int & N_EXT != 0
                && (*nlist).n_value != 0 as uint64_t
            {
                let mut sz = (*nlist).n_value as c_ulong;
                (*nlist).n_value = commonCounter as uint64_t;

                let ref mut fresh18 = (*(*(*oc).info).macho_symbols.offset(i_2 as isize)).addr;
                *fresh18 = commonCounter as *mut c_void as *mut c_void;

                let mut sym_type_0 = SYM_TYPE_CODE;

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

    return 1 as c_int;
}

unsafe fn ocMprotect_MachO(mut oc: *mut ObjectCode) -> bool {
    let mut i = 0 as c_int;

    while i < (*oc).n_segments {
        let mut segment: *mut Segment = (*oc).segments.offset(i as isize) as *mut Segment;

        if !((*segment).size == 0 as size_t) {
            if (*segment).prot as c_uint == SEGMENT_PROT_RX as c_int as c_uint {
                mprotectForLinker((*segment).start, (*segment).size, MEM_READ_EXECUTE);
            }
        }

        i += 1;
    }

    let mut i_0 = 0 as c_int;

    while i_0 < (*oc).n_sections {
        let mut section: *mut Section = (*oc).sections.offset(i_0 as isize) as *mut Section;

        if !((*section).size == 0 as StgWord) {
            if !((*section).alloc as c_uint != SECTION_MMAP as c_int as c_uint) {
                if !((*section).alloc as c_uint == SECTION_M32 as c_int as c_uint) {
                    match (*section).kind as c_uint {
                        0 => {
                            mprotectForLinker(
                                (*section).mapped_start,
                                (*section).mapped_size as size_t,
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

    return r#true != 0;
}

unsafe fn ocResolve_MachO(mut oc: *mut ObjectCode) -> c_int {
    if !(*(*oc).info).dsymCmd.is_null() {
        let mut indirectSyms = (*oc)
            .image
            .offset((*(*(*oc).info).dsymCmd).indirectsymoff as isize)
            as *mut c_ulong;

        let mut i = 0 as c_int;

        while i < (*oc).n_sections {
            let mut sectionName: *const c_char =
                &raw mut (*(*(*oc).info).macho_sections.offset(i as isize)).sectname as *mut c_char;

            if strcmp(
                sectionName,
                b"__la_symbol_ptr\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    sectionName,
                    b"__la_sym_ptr2\0" as *const u8 as *const c_char,
                ) == 0
                || strcmp(
                    sectionName,
                    b"__la_sym_ptr3\0" as *const u8 as *const c_char,
                ) == 0
            {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0 as c_int;
                }
            } else if strcmp(
                sectionName,
                b"__nl_symbol_ptr\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(sectionName, b"__pointers\0" as *const u8 as *const c_char) == 0
            {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0 as c_int;
                }
            } else if strcmp(sectionName, b"__jump_table\0" as *const u8 as *const c_char) == 0 {
                if resolveImports(
                    oc,
                    (*(*oc).info).macho_sections.offset(i as isize) as *mut MachOSection,
                    indirectSyms,
                ) == 0
                {
                    return 0 as c_int;
                }
            }

            i += 1;
        }
    }

    let mut i_0: size_t = 0 as size_t;

    while i_0 < (*(*oc).info).n_macho_symbols {
        let mut symbol: *mut MachOSymbol =
            (*(*oc).info).macho_symbols.offset(i_0 as isize) as *mut MachOSymbol;

        if needGotSlot(symbol) {
            if N_UNDF == (*(*symbol).nlist).n_type as c_int & N_TYPE {
                if (*symbol).addr.is_null() {
                    (*symbol).addr =
                        lookupDependentSymbol((*symbol).name, oc, null_mut::<SymType>());

                    if (*symbol).addr.is_null() {
                        errorBelch(
                            b"Failed to lookup symbol: %s\0" as *const u8 as *const c_char,
                            (*symbol).name,
                        );

                        return 0 as c_int;
                    }
                }
            }

            if (*symbol).addr.is_null() {
                errorBelch(
                    b"Symbol %s has no address!\n\0" as *const u8 as *const c_char,
                    (*symbol).name as *mut c_char,
                );

                return 0 as c_int;
            }

            if (*symbol).got_addr.is_null() {
                errorBelch(
                    b"Symbol %s has no Global Offset Table address!\n\0" as *const u8
                        as *const c_char,
                    (*symbol).name as *mut c_char,
                );

                return 0 as c_int;
            }

            *((*symbol).got_addr as *mut uint64_t) = (*symbol).addr as uint64_t;
        }

        i_0 = i_0.wrapping_add(1);
    }

    let mut i_1 = 0 as c_int;

    while i_1 < (*oc).n_sections {
        if relocateSectionAarch64(oc, (*oc).sections.offset(i_1 as isize) as *mut Section) == 0 {
            return 0 as c_int;
        }

        i_1 += 1;
    }

    if !ocMprotect_MachO(oc) {
        return 0 as c_int;
    }

    return 1 as c_int;
}

unsafe fn ocRunInit_MachO(mut oc: *mut ObjectCode) -> c_int {
    if (*(*oc).info).segCmd.is_null() {
        barf(b"ocRunInit_MachO: no segment load command\0" as *const u8 as *const c_char);
    }

    let mut argc: c_int = 0;
    let mut envc: c_int = 0;
    let mut argv = null_mut::<*mut c_char>();
    let mut envv = null_mut::<*mut c_char>();
    getProgArgv(&raw mut argc, &raw mut argv);
    getProgEnvv(&raw mut envc, &raw mut envv);

    let mut i = 0 as c_int;

    while i < (*oc).n_sections {
        if (*(*oc).sections.offset(i as isize)).kind as c_uint
            == SECTIONKIND_INIT_ARRAY as c_int as c_uint
        {
            let mut init_startC = (*(*oc).sections.offset(i as isize)).start;
            let mut init = init_startC as *mut init_t;
            let mut init_end = (init_startC as *mut uint8_t)
                .offset((*(*(*(*oc).sections.offset(i as isize)).info).macho_section).size as isize)
                as *mut init_t;

            let mut pn = 0 as c_int;

            while init < init_end {
                (*init).expect("non-null function pointer")(argc, argv, envv);
                init = init.offset(1);
                pn += 1;
            }
        }

        i += 1;
    }

    freeProgEnvv(envc, envv as *mut *mut c_char);

    return 1 as c_int;
}

unsafe fn ocRunFini_MachO(mut oc: *mut ObjectCode) -> c_int {
    if (*(*oc).info).segCmd.is_null() {
        barf(b"ocRunInit_MachO: no segment load command\0" as *const u8 as *const c_char);
    }

    let mut i = 0 as c_int;

    while i < (*oc).n_sections {
        if (*(*oc).sections.offset(i as isize)).kind as c_uint
            == SECTIONKIND_FINI_ARRAY as c_int as c_uint
        {
            let mut fini_startC = (*(*oc).sections.offset(i as isize)).start;
            let mut fini = fini_startC as *mut fini_t;
            let mut fini_end = (fini_startC as *mut uint8_t)
                .offset((*(*(*(*oc).sections.offset(i as isize)).info).macho_section).size as isize)
                as *mut fini_t;

            let mut pn = 0 as c_int;

            while fini < fini_end {
                (*fini).expect("non-null function pointer")();
                fini = fini.offset(1);
                pn += 1;
            }
        }

        i += 1;
    }

    return 1 as c_int;
}

unsafe fn machoGetMisalignment(mut f: *mut FILE) -> c_int {
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

    let mut misalignment: c_int = 0;

    let mut n = fread(
        &raw mut header as *mut c_void,
        size_of::<MachOHeader>() as size_t,
        1 as size_t,
        f,
    ) as size_t;

    if n != 1 as size_t {
        barf(b"machoGetMisalignment: can't read the Mach-O header\0" as *const u8 as *const c_char);
    }

    fseek(
        f,
        (size_of::<MachOHeader>() as usize).wrapping_neg() as c_long,
        SEEK_CUR,
    );

    if header.magic != MH_MAGIC_64 as uint32_t {
        barf(
            b"Bad magic. Expected: %08x, got: %08x.\0" as *const u8 as *const c_char,
            MH_MAGIC_64,
            header.magic,
        );
    }

    misalignment = ((header.sizeofcmds as usize).wrapping_add(size_of::<MachOHeader>() as usize)
        & 0xf as usize) as c_int;

    return if misalignment != 0 {
        16 as c_int - misalignment
    } else {
        0 as c_int
    };
}
