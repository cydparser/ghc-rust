use crate::check_unload::{
    exitUnloadCheck, initUnloadCheck, insertOCSectionIndices, loaded_objects, n_unloaded_objects,
    object_code_mark_bit, objects,
};
use crate::ffi::hs_ffi::HsInt;
use crate::ffi::hs_ffi::{HS_BOOL_FALSE, HS_BOOL_TRUE, HsBool, HsInt, HsPtr};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::foreign_exports::ForeignExportsList;
use crate::ffi::rts::foreign_exports::ForeignExportsList;
use crate::ffi::rts::linker::{
    OBJECT_DONT_RESOLVE, OBJECT_LOADED, OBJECT_NEEDED, OBJECT_NOT_LOADED, OBJECT_READY,
    OBJECT_RESOLVED, OBJECT_UNLOADED, OStatus, pathchar,
};
use crate::ffi::rts::linker::{OStatus, pathchar};
use crate::ffi::rts::messages::{barf, debugBelch, errorBelch};
use crate::ffi::rts::os_threads::{Mutex, closeMutex, initMutex};
use crate::ffi::rts::storage::closures::{StgInd, StgIndStatic};
use crate::ffi::rts::storage::gc::{newGCdCAF, newRetainedCAF};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{StgStablePtr, StgWord};
use crate::foreign_exports::{foreignExportsFinishedLoadingObject, foreignExportsLoadingObject};
use crate::ghcautoconf::RTS_LINKER_USE_MMAP;
use crate::hash::{HashSet, StrHashTable};
use crate::hash::{
    HashSet, StrHashTable, allocHashSet, allocStrHashTable, freeHashSet, freeStrHashTable,
    insertHashSet, insertStrHashTable, lookupStrHashTable, removeStrHashTable,
};
use crate::linker::load_native_obj_posix::{freeNativeCode_POSIX, loadNativeObj_POSIX};
use crate::linker::m_map::{MEM_READ_WRITE, mmapForLinker, munmapForLinker};
use crate::linker::m32_alloc::m32_allocator;
use crate::linker::m32_alloc::{
    m32_allocator, m32_allocator_flush, m32_allocator_free, m32_allocator_new,
};
use crate::linker::mach_o::{
    ocAllocateExtras_MachO, ocGetNames_MachO, ocInit_MachO, ocResolve_MachO, ocRunFini_MachO,
    ocRunInit_MachO, ocVerifyImage_MachO,
};
use crate::linker::mach_o_types::{ObjectCodeFormatInfo, SectionFormatInfo};
use crate::linker::mach_o_types::{ObjectCodeFormatInfo, SectionFormatInfo};
use crate::linker::proddable_blocks::ProddableBlockSet;
use crate::linker::proddable_blocks::{
    ProddableBlockSet, freeProddableBlocks, initProddableBlockSet,
};
use crate::linker::symbol_extras::ocProtectExtras;
use crate::linker_internals::{
    _ObjectCode, _RtsSymbolInfo, _Section, _Segment, _Symbol, DYNAMIC_OBJECT, NativeCodeRange,
    NativeCodeRange_, ObjectCode, ObjectType, RtsSymbolInfo, STATIC_OBJECT, Section, SectionAlloc,
    SectionKind, Segment, SegmentProt, Symbol_t, SymbolExtra, USE_CONTIGUOUS_MMAP, cxa_finalize_fn,
    isArchive, loadArchive_,
};
use crate::path_utils::{pathdup, pathsize};
use crate::prelude::*;
use crate::profiling::refreshProfilingCCSs;
use crate::rts_symbol_info::{isSymbolImport, isSymbolWeak};
use crate::rts_symbols::{
    RtsSymbolVal, STRENGTH_NORMAL, STRENGTH_STRONG, STRENGTH_WEAK, SYM_TYPE_CODE,
    SYM_TYPE_DUP_DISCARD, SYM_TYPE_HIDDEN, SymStrength, SymType, SymbolName, rtsExtraSyms, rtsSyms,
};
use crate::rts_symbols::{SymStrength, SymType, SymbolName};
use crate::rts_utils::{stgCallocBytes, stgFree, stgMallocBytes};
use crate::stable_ptr::freeStablePtr;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _ObjectCode {
    pub(crate) status: OStatus,
    pub(crate) fileName: *mut pathchar,
    pub(crate) fileSize: i32,
    pub(crate) formatName: *mut c_char,
    pub(crate) r#type: ObjectType,
    pub(crate) archiveMemberName: *mut pathchar,
    pub(crate) symbols: *mut Symbol_t,
    pub(crate) n_symbols: i32,
    pub(crate) image: *mut c_char,
    pub(crate) info: *mut ObjectCodeFormatInfo,
    pub(crate) imageMapped: i32,
    pub(crate) misalignment: i32,
    pub(crate) cxa_finalize: cxa_finalize_fn,
    pub(crate) n_sections: i32,
    pub(crate) sections: *mut Section,
    pub(crate) n_segments: i32,
    pub(crate) segments: *mut Segment,
    pub(crate) next: *mut _ObjectCode,
    pub(crate) prev: *mut _ObjectCode,
    pub(crate) next_loaded_object: *mut _ObjectCode,
    pub(crate) mark: StgWord,
    pub(crate) unloadable: bool,
    pub(crate) dependencies: *mut HashSet,
    pub(crate) proddables: ProddableBlockSet,
    pub(crate) symbol_extras: *mut SymbolExtra,
    pub(crate) first_symbol_extra: u64,
    pub(crate) n_symbol_extras: u64,
    pub(crate) bssBegin: *mut c_char,
    pub(crate) bssEnd: *mut c_char,
    pub(crate) foreign_exports: *mut ForeignExportsList,
    pub(crate) extraInfos: *mut StrHashTable,
    pub(crate) rw_m32: *mut m32_allocator,
    pub(crate) rx_m32: *mut m32_allocator,
    pub(crate) dlopen_handle: *mut c_void,
    pub(crate) nc_ranges: *mut NativeCodeRange,
}

pub(crate) type NativeCodeRange = NativeCodeRange_;

/// cbindgen:no-export
pub(crate) struct NativeCodeRange_ {
    pub(crate) start: *mut c_void,
    pub(crate) end: *mut c_void,
    pub(crate) next: *mut NativeCodeRange_,
}

/// cbindgen:no-export
pub(crate) struct SymbolExtra {}

pub(crate) type Segment = _Segment;

/// cbindgen:no-export
pub(crate) struct _Segment {
    pub(crate) start: *mut c_void,
    pub(crate) size: usize,
    pub(crate) prot: SegmentProt,
    pub(crate) sections_idx: *mut i32,
    pub(crate) n_sections: i32,
}

pub(crate) type SegmentProt = u32;

pub(crate) const SEGMENT_PROT_RWO: SegmentProt = 3;

pub(crate) const SEGMENT_PROT_RX: SegmentProt = 5;

pub(crate) const SEGMENT_PROT_RO: SegmentProt = 1;

pub(crate) type Section = _Section;

/// cbindgen:no-export
pub(crate) struct _Section {
    pub(crate) start: *mut c_void,
    pub(crate) size: StgWord,
    pub(crate) kind: SectionKind,
    pub(crate) alloc: SectionAlloc,
    pub(crate) mapped_offset: StgWord,
    pub(crate) mapped_start: *mut c_void,
    pub(crate) mapped_size: StgWord,
    pub(crate) info: *mut SectionFormatInfo,
}

pub(crate) type SectionAlloc = u32;

pub(crate) const SECTION_MALLOC: SectionAlloc = 3;

pub(crate) const SECTION_MMAP: SectionAlloc = 2;

pub(crate) const SECTION_M32: SectionAlloc = 1;

pub(crate) const SECTION_NOMEM: SectionAlloc = 0;

pub(crate) type SectionKind = u32;

pub(crate) const SECTIONKIND_BFD_IMPORT_LIBRARY: SectionKind = 10;

pub(crate) const SECTIONKIND_BFD_IMPORT_LIBRARY_HEAD: SectionKind = 9;

pub(crate) const SECTIONKIND_IMPORT: SectionKind = 8;

pub(crate) const SECTIONKIND_EXCEPTION_UNWIND: SectionKind = 7;

pub(crate) const SECTIONKIND_EXCEPTION_TABLE: SectionKind = 6;

pub(crate) const SECTIONKIND_DEBUG: SectionKind = 5;

pub(crate) const SECTIONKIND_OTHER: SectionKind = 4;

pub(crate) const SECTIONKIND_FINI_ARRAY: SectionKind = 3;

pub(crate) const SECTIONKIND_INIT_ARRAY: SectionKind = 2;

pub(crate) const SECTIONKIND_RWDATA: SectionKind = 1;

pub(crate) const SECTIONKIND_CODE_OR_RODATA: SectionKind = 0;

pub(crate) type cxa_finalize_fn = Option<unsafe extern "C" fn(*mut c_void) -> ()>;

pub(crate) type Symbol_t = _Symbol;

/// cbindgen:no-export
pub(crate) struct _Symbol {
    pub(crate) name: *mut SymbolName,
    pub(crate) addr: *mut c_void,
    pub(crate) r#type: SymType,
}

pub(crate) type ObjectType = u32;

pub(crate) const DYNAMIC_OBJECT: ObjectType = 1;

pub(crate) const STATIC_OBJECT: ObjectType = 0;

pub(crate) type ObjectCode = _ObjectCode;

pub(crate) type RtsSymbolInfo = _RtsSymbolInfo;

/// cbindgen:no-export
pub(crate) struct _RtsSymbolInfo {
    pub(crate) value: *mut c_void,
    pub(crate) owner: *mut ObjectCode,
    pub(crate) strength: SymStrength,
    pub(crate) r#type: SymType,
}

pub(crate) const USE_CONTIGUOUS_MMAP: i32 = 0;

extern "C" {
    pub(crate) fn isArchive(path: *mut pathchar) -> bool;
    pub(crate) fn loadArchive_(path: *mut pathchar) -> HsInt;
}

#[ffi(testsuite)]
pub type pathchar = c_char;

#[ffi(testsuite)]
#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum OStatus {
    OBJECT_LOADED = 0,
    OBJECT_NEEDED = 1,
    OBJECT_RESOLVED = 2,
    OBJECT_READY = 3,
    OBJECT_UNLOADED = 4,
    OBJECT_DONT_RESOLVE = 5,
    OBJECT_NOT_LOADED = 6,
}

#[cfg(feature = "sys")]
impl From<OStatus> for sys::OStatus {
    fn from(v: OStatus) -> Self {
        use OStatus::*;

        match v {
            OBJECT_LOADED => sys::OStatus::OBJECT_LOADED,
            OBJECT_NEEDED => sys::OStatus::OBJECT_NEEDED,
            OBJECT_RESOLVED => sys::OStatus::OBJECT_RESOLVED,
            OBJECT_READY => sys::OStatus::OBJECT_READY,
            OBJECT_UNLOADED => sys::OStatus::OBJECT_UNLOADED,
            OBJECT_DONT_RESOLVE => sys::OStatus::OBJECT_DONT_RESOLVE,
            OBJECT_NOT_LOADED => sys::OStatus::OBJECT_NOT_LOADED,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::OStatus> for OStatus {
    fn from(v: sys::OStatus) -> Self {
        use OStatus::*;

        match v {
            sys::OStatus::OBJECT_LOADED => OBJECT_LOADED,
            sys::OStatus::OBJECT_NEEDED => OBJECT_NEEDED,
            sys::OStatus::OBJECT_RESOLVED => OBJECT_RESOLVED,
            sys::OStatus::OBJECT_READY => OBJECT_READY,
            sys::OStatus::OBJECT_UNLOADED => OBJECT_UNLOADED,
            sys::OStatus::OBJECT_DONT_RESOLVE => OBJECT_DONT_RESOLVE,
            sys::OStatus::OBJECT_NOT_LOADED => OBJECT_NOT_LOADED,
        }
    }
}

impl TryFrom<u32> for OStatus {
    type Error = ();

    fn try_from(d: u32) -> Result<OStatus, ()> {
        use OStatus::*;

        match d {
            0 => Ok(OBJECT_LOADED),
            1 => Ok(OBJECT_NEEDED),
            2 => Ok(OBJECT_RESOLVED),
            3 => Ok(OBJECT_READY),
            4 => Ok(OBJECT_UNLOADED),
            5 => Ok(OBJECT_DONT_RESOLVE),
            6 => Ok(OBJECT_NOT_LOADED),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for OStatus {
    fn arbitrary(g: &mut Gen) -> Self {
        use OStatus::*;

        match usize::arbitrary(g) % 7 {
            0 => OBJECT_LOADED,
            1 => OBJECT_NEEDED,
            2 => OBJECT_RESOLVED,
            3 => OBJECT_READY,
            4 => OBJECT_UNLOADED,
            5 => OBJECT_DONT_RESOLVE,
            6.. => OBJECT_NOT_LOADED,
        }
    }
}

pub(crate) const OBJECT_NOT_LOADED: OStatus = 6;

pub(crate) const OBJECT_DONT_RESOLVE: OStatus = 5;

pub(crate) const OBJECT_UNLOADED: OStatus = 4;

pub(crate) const OBJECT_READY: OStatus = 3;

pub(crate) const OBJECT_RESOLVED: OStatus = 2;

pub(crate) const OBJECT_NEEDED: OStatus = 1;

pub(crate) const OBJECT_LOADED: OStatus = 0;

static mut symhash: *mut StrHashTable = null_mut::<StrHashTable>();

static mut linker_mutex: Mutex = _opaque_pthread_mutex_t {
    __sig: 0,
    __opaque: [0; 56],
};

unsafe fn ghciRemoveSymbolTable(
    mut table: *mut StrHashTable,
    mut key: *const SymbolName,
    mut owner: *mut ObjectCode,
) {
    let mut pinfo = lookupStrHashTable(table, key as *const c_char) as *mut RtsSymbolInfo;

    if pinfo.is_null() || owner != (*pinfo).owner {
        return;
    }

    removeStrHashTable(table, key as *const c_char, null::<c_void>());

    if isSymbolImport(owner, key as *const c_void) {
        stgFree((*pinfo).value as *mut c_void);
    }

    stgFree(pinfo as *mut c_void);
}

unsafe fn symbolTypeString(mut r#type: SymType) -> *const c_char {
    match r#type as u32 & !(SYM_TYPE_DUP_DISCARD as i32 | SYM_TYPE_HIDDEN as i32) as u32 {
        1 => return c"code".as_ptr(),
        2 => return c"data".as_ptr(),
        4 => return c"indirect-data".as_ptr(),
        _ => {
            barf(
                c"symbolTypeString: unknown symbol type (%d)".as_ptr(),
                r#type as u32,
            );
        }
    };
}

unsafe fn ghciInsertSymbolTable(
    mut obj_name: *mut pathchar,
    mut table: *mut StrHashTable,
    mut key: *const SymbolName,
    mut data: *mut c_void,
    mut strength: SymStrength,
    mut r#type: SymType,
    mut owner: *mut ObjectCode,
) -> i32 {
    let mut pinfo = lookupStrHashTable(table, key as *const c_char) as *mut RtsSymbolInfo;

    if pinfo.is_null() {
        pinfo = stgMallocBytes(
            size_of::<RtsSymbolInfo>() as usize,
            c"ghciInsertToSymbolTable".as_ptr(),
        ) as *mut RtsSymbolInfo;

        (*pinfo).value = data;
        (*pinfo).owner = owner;
        (*pinfo).strength = strength;
        (*pinfo).r#type = r#type;
        insertStrHashTable(table, key as *const c_char, pinfo as *const c_void);

        return 1;
    } else if (*pinfo).r#type as u32 ^ r#type as u32 != 0 {
        if (*pinfo).r#type as u32 & SYM_TYPE_HIDDEN as i32 as u32 != 0 {
            (*pinfo).value = data;
            (*pinfo).owner = owner;
            (*pinfo).strength = strength;
            (*pinfo).r#type = r#type;

            return 1;
        }

        if r#type as u32 & (SYM_TYPE_DUP_DISCARD as i32 | SYM_TYPE_HIDDEN as i32) as u32 == 0 {
            debugBelch(
                c"Symbol type mismatch (existing %d, new %d).\n".as_ptr(),
                (*pinfo).r#type as u32,
                r#type as u32,
            );

            debugBelch(
                c"Symbol %s was defined by %s to be a %s symbol.\n".as_ptr(),
                key,
                obj_name,
                symbolTypeString(r#type),
            );

            debugBelch(
                c"      yet was defined by %s to be a %s symbol.\n".as_ptr(),
                if !(*pinfo).owner.is_null() {
                    (*(*pinfo).owner).fileName as *const pathchar
                } else {
                    b"<builtin>\0" as *const u8 as *const pathchar
                },
                symbolTypeString((*pinfo).r#type),
            );
        }

        return 1;
    } else if (*pinfo).strength as u32 == STRENGTH_STRONG as i32 as u32 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s is already defined as a strong symbol; ignoring redefinition...".as_ptr(),
                key,
            );
        }

        return 1;
    } else if strength as u32 == STRENGTH_WEAK as i32 as u32
        && !data.is_null()
        && (*pinfo).strength as u32 == STRENGTH_WEAK as i32 as u32
        && (*pinfo).value.is_null()
    {
        (*pinfo).value = data;
        (*pinfo).owner = owner;

        return 1;
    } else if strength as u32 == STRENGTH_WEAK as i32 as u32 {
        return 1;
    } else if (*pinfo).strength as u32 == STRENGTH_WEAK as i32 as u32
        && strength as u32 != STRENGTH_WEAK as i32 as u32
    {
        (*pinfo).value = data;
        (*pinfo).owner = owner;
        (*pinfo).strength = strength;

        return 1;
    } else if !(*pinfo).owner.is_null()
        && (*(*pinfo).owner).status as u32 != OBJECT_READY as i32 as u32
        && (*(*pinfo).owner).status as u32 != OBJECT_RESOLVED as i32 as u32
        && (*(*pinfo).owner).status as u32 != OBJECT_NEEDED as i32 as u32
    {
        if !owner.is_null()
            && ((*owner).status as u32 == OBJECT_NEEDED as i32 as u32
                || (*owner).status as u32 == OBJECT_RESOLVED as i32 as u32
                || (*owner).status as u32 == OBJECT_READY as i32 as u32)
        {
            (*pinfo).value = data;
            (*pinfo).owner = owner;
            (*pinfo).strength = strength;
        }

        return 1;
    } else if (*pinfo).owner == owner {
        return 1;
    } else if !owner.is_null() && (*owner).status as u32 == OBJECT_LOADED as i32 as u32 {
        return 1;
    }

    debugBelch(
        c"GHC runtime linker: fatal error: I found a duplicate definition for symbol\n   %s\nwhilst processing object file\n   %s\nThe symbol was previously defined in\n   %s\nThis could be caused by:\n   * Loading two different object files which export the same symbol\n   * Specifying the same object file twice on the GHCi command line\n   * An incorrect `package.conf' entry, causing some object to be\n     loaded twice.\n"
            .as_ptr(),
        key as *mut c_char,
        obj_name,
        if (*pinfo).owner.is_null() {
            c"(GHCi built-in symbols)".as_ptr()
        } else {
            (if !(*(*pinfo).owner).archiveMemberName.is_null() {
                (*(*pinfo).owner).archiveMemberName
            } else {
                (*(*pinfo).owner).fileName
            }) as *const c_char
        },
    );

    return 0;
}

unsafe fn ghciLookupSymbolInfo(
    mut table: *mut StrHashTable,
    mut key: *const SymbolName,
    mut result: *mut *mut RtsSymbolInfo,
) -> HsBool {
    let mut pinfo = lookupStrHashTable(table, key as *const c_char) as *mut RtsSymbolInfo;

    if pinfo.is_null() {
        *result = null_mut::<RtsSymbolInfo>();

        return HS_BOOL_FALSE as HsBool;
    }

    if (*pinfo).strength as u32 == STRENGTH_WEAK as i32 as u32 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"lookupSymbolInfo: promoting %s\n".as_ptr(), key);
        }

        (*pinfo).strength = STRENGTH_NORMAL;
    }

    *result = pinfo;

    return HS_BOOL_TRUE as HsBool;
}

static mut linker_init_done: i32 = 0;

static mut dl_prog_handle: *mut c_void = null_mut::<c_void>();

static mut re_invalid: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: null::<c_char>(),
    re_g: null_mut::<re_guts>(),
};

static mut re_realso: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: null::<c_char>(),
    re_g: null_mut::<re_guts>(),
};

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker() {
    initLinker_(1);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker_(mut retain_cafs: c_int) {
    let mut compileResult: i32 = 0;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"initLinker: start\n".as_ptr());
    }

    if linker_init_done == 1 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"initLinker: idempotent return\n".as_ptr());
        }

        return;
    } else {
        linker_init_done = 1;
    }

    initUnloadCheck();
    initMutex(&raw mut linker_mutex);
    symhash = allocStrHashTable();

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"populating linker symbol table with built-in RTS symbols\n".as_ptr());
    }

    let mut sym: *const RtsSymbolVal = &raw mut rtsSyms as *mut RtsSymbolVal;

    while !(*sym).lbl.is_null() {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"initLinker: inserting rts symbol %s, %p\n".as_ptr(),
                (*sym).lbl,
                (*sym).addr,
            );
        }

        if ghciInsertSymbolTable(
            c"(GHCi built-in symbols)".as_ptr() as *mut pathchar,
            symhash,
            (*sym).lbl,
            (*sym).addr,
            (*sym).strength,
            (*sym).r#type,
            null_mut::<ObjectCode>(),
        ) == 0
        {
            barf(c"ghciInsertSymbolTable failed".as_ptr());
        }

        sym = sym.offset(1);
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"done with built-in RTS symbols\n".as_ptr());
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"populating linker symbol table with extra RTS symbols\n".as_ptr());
    }

    if Some(rtsExtraSyms as unsafe extern "C" fn() -> *mut RtsSymbolVal).is_some()
        && !rtsExtraSyms().is_null()
    {
        let mut sym_0 = rtsExtraSyms();

        while !(*sym_0).lbl.is_null() {
            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"initLinker: inserting extra rts symbol %s, %p\n".as_ptr(),
                    (*sym_0).lbl,
                    (*sym_0).addr,
                );
            }

            if ghciInsertSymbolTable(
                c"(GHCi built-in symbols)".as_ptr() as *mut pathchar,
                symhash,
                (*sym_0).lbl,
                (*sym_0).addr,
                (*sym_0).strength,
                (*sym_0).r#type,
                null_mut::<ObjectCode>(),
            ) == 0
            {
                barf(c"ghciInsertSymbolTable failed".as_ptr());
            }

            sym_0 = sym_0.offset(1);
        }
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"done with extra RTS symbols\n".as_ptr());
    }

    if ghciInsertSymbolTable(
        c"(GHCi built-in symbols)".as_ptr() as *mut pathchar,
        symhash,
        c"_newCAF".as_ptr(),
        transmute::<
            Option<unsafe extern "C" fn(*mut StgRegTable, *mut StgIndStatic) -> *mut StgInd>,
            *mut c_void,
        >(if retain_cafs != 0 {
            Some(
                newRetainedCAF
                    as unsafe extern "C" fn(*mut StgRegTable, *mut StgIndStatic) -> *mut StgInd,
            )
        } else {
            Some(
                newGCdCAF
                    as unsafe extern "C" fn(*mut StgRegTable, *mut StgIndStatic) -> *mut StgInd,
            )
        }),
        STRENGTH_NORMAL,
        SYM_TYPE_CODE,
        null_mut::<ObjectCode>(),
    ) == 0
    {
        barf(c"ghciInsertSymbolTable failed".as_ptr());
    }

    dl_prog_handle = RTLD_DEFAULT;

    compileResult = regcomp(
        &raw mut re_invalid,
        c"(([^ \t()])+\\.so([^ \t:()])*):([ \t])*(invalid ELF header|file too short|invalid file format|Exec format error)"
            .as_ptr(),
        REG_EXTENDED,
    );

    if compileResult != 0 {
        barf(c"Compiling re_invalid failed".as_ptr());
    }

    compileResult = regcomp(
        &raw mut re_realso,
        c"(GROUP|INPUT) *\\( *([^ )]+)".as_ptr(),
        REG_EXTENDED,
    );

    if compileResult != 0 {
        barf(c"Compiling re_realso failed".as_ptr());
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"initLinker: done\n".as_ptr());
    }
}

unsafe fn exitLinker() {
    if linker_init_done == 1 {
        regfree(&raw mut re_invalid);
        regfree(&raw mut re_realso);
    }

    if linker_init_done == 1 {
        freeStrHashTable(
            symhash,
            Some(free as unsafe extern "C" fn(*mut c_void) -> ()),
        );
        exitUnloadCheck();
    }

    closeMutex(&raw mut linker_mutex);
}

unsafe fn internal_dlsym(mut symbol: *const c_char) -> *mut c_void {
    let mut v = null_mut::<c_void>();

    if (pthread_mutex_lock(&raw mut linker_mutex) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 604);
    }

    dlerror();
    v = dlsym(dl_prog_handle, symbol);

    if dlerror().is_null() {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"internal_dlsym: found symbol '%s' in program\n".as_ptr(),
                symbol,
            );
        }

        return v;
    }

    let mut nc = loaded_objects;

    while !nc.is_null() {
        if (*nc).r#type as u32 == DYNAMIC_OBJECT as i32 as u32 {
            v = dlsym((*nc).dlopen_handle, symbol);

            if dlerror().is_null() {
                if RtsFlags.DebugFlags.linker {
                    debugBelch(
                        c"internal_dlsym: found symbol '%s' in shared object\n".as_ptr(),
                        symbol,
                    );
                }

                return v;
            }
        }

        nc = (*nc).next_loaded_object as *mut ObjectCode;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"internal_dlsym: looking for symbol '%s' in special cases\n".as_ptr(),
            symbol,
        );
    }

    return NULL;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    mut handle: *mut c_void,
    mut symbol_name: *const c_char,
) -> *mut c_void {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            668,
            __r,
        );
    }

    if (*symbol_name.offset(0) as i32 == '_' as i32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 673);
    }

    symbol_name = symbol_name.offset(1);

    let mut result = dlsym(handle, symbol_name);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            687,
        );
    }

    return result;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn addDLL(mut dll_name: *mut pathchar) -> *const c_char {
    let mut errmsg = null_mut::<c_char>();

    if !loadNativeObj(dll_name, &raw mut errmsg).is_null() {
        return null::<c_char>();
    } else {
        if !errmsg.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Linker.c".as_ptr(), 697);
        }

        return errmsg;
    };
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn findSystemLibrary(mut dll_name: *mut pathchar) -> *mut pathchar {
    if RtsFlags.DebugFlags.linker {
        debugBelch(c"\nfindSystemLibrary: dll_name = `%s'\n".as_ptr(), dll_name);
    }

    return null_mut::<pathchar>();
}

unsafe fn warnMissingKBLibraryPaths() {
    static mut missing_update_warn: HsBool = HS_BOOL_FALSE as HsBool;

    if missing_update_warn == 0 {
        debugBelch(c"Warning: If linking fails, consider installing KB2533623.\n".as_ptr());

        missing_update_warn = HS_BOOL_TRUE as HsBool;
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn addLibrarySearchPath(mut dll_path: *mut pathchar) -> HsPtr {
    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"\naddLibrarySearchPath: dll_path = `%s'\n".as_ptr(),
            dll_path,
        );
    }

    return NULL;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn removeLibrarySearchPath(mut dll_path_index: HsPtr) -> HsBool {
    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"\nremoveLibrarySearchPath: ptr = `%p'\n".as_ptr(),
            dll_path_index,
        );
    }

    return HS_BOOL_FALSE as HsBool;
}

unsafe fn insertSymbol(
    mut obj_name: *mut pathchar,
    mut key: *mut SymbolName,
    mut data: *mut c_void,
) -> HsInt {
    return ghciInsertSymbolTable(
        obj_name,
        symhash,
        key,
        data,
        STRENGTH_NORMAL,
        SYM_TYPE_CODE,
        null_mut::<ObjectCode>(),
    ) as HsInt;
}

unsafe fn lookupDependentSymbol(
    mut lbl: *mut SymbolName,
    mut dependent: *mut ObjectCode,
    mut r#type: *mut SymType,
) -> *mut c_void {
    if (pthread_mutex_lock(&raw mut linker_mutex) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 801);
    }

    if RtsFlags.DebugFlags.linker_verbose {
        debugBelch(c"lookupSymbol: looking up '%s'\n".as_ptr(), lbl);
    }

    if !symhash.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 804);
    }

    let mut pinfo = null_mut::<RtsSymbolInfo>();

    if strcmp(lbl, c"___dso_handle".as_ptr()) == 0 {
        if !dependent.is_null() {
            return (*dependent).image as *mut c_void;
        } else {
            return transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut SymbolName,
                        *mut ObjectCode,
                        *mut SymType,
                    ) -> *mut c_void,
                >,
                *mut c_void,
            >(Some(
                lookupDependentSymbol
                    as unsafe extern "C" fn(
                        *mut SymbolName,
                        *mut ObjectCode,
                        *mut SymType,
                    ) -> *mut c_void,
            ));
        }
    }

    if strcmp(lbl, c"___cxa_atexit".as_ptr()) == 0 && !dependent.is_null() {
        (*dependent).cxa_finalize =
            transmute::<*mut c_void, cxa_finalize_fn>(lookupDependentSymbol(
                c"___cxa_finalize".as_ptr() as *mut SymbolName,
                dependent,
                null_mut::<SymType>(),
            ));
    }

    if ghciLookupSymbolInfo(symhash, lbl, &raw mut pinfo) == 0 {
        if RtsFlags.DebugFlags.linker_verbose {
            debugBelch(
                c"lookupSymbol: symbol '%s' not found, trying dlsym\n".as_ptr(),
                lbl,
            );
        }

        if RtsFlags.DebugFlags.linker {
            debugBelch(c"lookupSymbol: looking up %s with dlsym\n".as_ptr(), lbl);
        }

        if (*lbl.offset(0) as i32 == '_' as i32) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Linker.c".as_ptr(), 866);
        }

        if !r#type.is_null() {
            *r#type = SYM_TYPE_CODE;
        }

        return internal_dlsym(lbl.offset(1)) as *mut c_void;
    } else {
        static mut RTS_NO_FINI: *mut c_void = NULL;

        if strcmp(lbl, c"__fini_array_end".as_ptr()) == 0 {
            return &raw mut RTS_NO_FINI as *mut c_void;
        }

        if strcmp(lbl, c"__fini_array_start".as_ptr()) == 0 {
            return &raw mut RTS_NO_FINI as *mut c_void;
        }

        if !r#type.is_null() {
            *r#type = (*pinfo).r#type;
        }

        if !dependent.is_null() {
            let mut owner = (*pinfo).owner;

            if !owner.is_null() {
                insertHashSet((*dependent).dependencies, owner as StgWord);
            }
        }

        return loadSymbol(lbl, pinfo);
    };
}

unsafe fn loadSymbol(mut lbl: *mut SymbolName, mut pinfo: *mut RtsSymbolInfo) -> *mut c_void {
    if RtsFlags.DebugFlags.linker_verbose {
        debugBelch(
            c"lookupSymbol: value of %s is %p, owned by %s\n".as_ptr(),
            lbl,
            (*pinfo).value,
            if !(*pinfo).owner.is_null() {
                (if !(*(*pinfo).owner).archiveMemberName.is_null() {
                    (*(*pinfo).owner).archiveMemberName
                } else {
                    (*(*pinfo).owner).fileName
                }) as *const pathchar
            } else {
                b"No owner, probably built-in.\0" as *const u8 as *const pathchar
            },
        );
    }

    let mut oc = (*pinfo).owner;

    if !oc.is_null() && !lbl.is_null() && (*oc).status as u32 == OBJECT_LOADED as i32 as u32 {
        (*oc).status = OBJECT_NEEDED;

        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"lookupSymbol: on-demand loading symbol '%s'\n".as_ptr(),
                lbl,
            );
        }

        let mut r = ocTryLoad(oc);

        if r == 0 {
            return NULL as *mut c_void;
        }
    }

    return (*pinfo).value;
}

unsafe fn printLoadedObjects() {
    let mut oc = null_mut::<ObjectCode>();
    oc = objects;

    while !oc.is_null() {
        if !(*oc).sections.is_null() {
            let mut i: i32 = 0;

            printf(
                c"%s\n".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            i = 0;

            while i < (*oc).n_sections {
                if !(*(*oc).sections.offset(i as isize)).mapped_start.is_null()
                    || !(*(*oc).sections.offset(i as isize)).start.is_null()
                {
                    printf(
                        c"\tsec %2d[alloc: %d; kind: %d]: %p - %p; mmaped: %p - %p\n".as_ptr(),
                        i,
                        (*(*oc).sections.offset(i as isize)).alloc as u32,
                        (*(*oc).sections.offset(i as isize)).kind as u32,
                        (*(*oc).sections.offset(i as isize)).start,
                        ((*(*oc).sections.offset(i as isize)).start as usize as StgWord)
                            .wrapping_add((*(*oc).sections.offset(i as isize)).size)
                            as *mut c_void,
                        (*(*oc).sections.offset(i as isize)).mapped_start,
                        ((*(*oc).sections.offset(i as isize)).mapped_start as usize as StgWord)
                            .wrapping_add((*(*oc).sections.offset(i as isize)).mapped_size)
                            as *mut c_void,
                    );
                }

                i += 1;
            }
        }

        oc = (*oc).next as *mut ObjectCode;
    }
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupSymbol(mut lbl: *mut SymbolName) -> *mut c_void {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            972,
            __r,
        );
    }

    let mut r = lookupDependentSymbol(lbl, null_mut::<ObjectCode>(), null_mut::<SymType>());

    if r.is_null() {
        if !RtsFlags.MiscFlags.linkerOptimistic {
            errorBelch(
                c"^^ Could not load '%s', dependency unresolved. See top entry above. You might consider using --optimistic-linking\n"
                    .as_ptr(),
                lbl,
            );

            if RtsFlags.DebugFlags.linker {
                printLoadedObjects();
            }

            fflush(__stderrp);
        } else {
            errorBelch(
                c"^^ Could not load '%s', dependency unresolved, optimistically continuing\n"
                    .as_ptr(),
                lbl,
            );

            r = 0xdeadbeef;
        }
    }

    if runPendingInitializers() == 0 {
        errorBelch(c"lookupSymbol: Failed to run initializers.".as_ptr());
    }

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            996,
        );
    }

    return r as *mut c_void;
}

unsafe fn ghci_enquire(mut addr: *mut c_void) {
    let mut i: i32 = 0;
    let mut sym = null_mut::<SymbolName>();
    let mut a = null_mut::<RtsSymbolInfo>();
    let DELTA = 64;
    let mut oc = null_mut::<ObjectCode>();
    oc = objects;

    while !oc.is_null() {
        i = 0;

        while i < (*oc).n_symbols {
            sym = (*(*oc).symbols.offset(i as isize)).name;

            if !sym.is_null() {
                a = null_mut::<RtsSymbolInfo>();

                if a.is_null() {
                    ghciLookupSymbolInfo(symhash, sym, &raw mut a);
                }

                if !a.is_null() {
                    if !(*a).value.is_null()
                        && (addr as *mut c_char).offset(-(DELTA as isize))
                            <= (*a).value as *mut c_char
                        && (*a).value as *mut c_char <= (addr as *mut c_char).offset(DELTA as isize)
                    {
                        debugBelch(
                            c"%p + %3d  ==  `%s'\n".as_ptr(),
                            addr,
                            ((*a).value as *mut c_char).offset_from(addr as *mut c_char) as i64
                                as i32,
                            sym,
                        );
                    }
                }
            }

            i += 1;
        }

        oc = (*oc).next as *mut ObjectCode;
    }
}

unsafe fn resolveSymbolAddr(
    mut buffer: *mut pathchar,
    mut size: i32,
    mut symbol: *mut c_void,
    mut top: *mut usize,
) -> *mut pathchar {
    return null_mut::<pathchar>();
}

unsafe fn removeOcSymbols(mut oc: *mut ObjectCode) {
    if (*oc).symbols.is_null() {
        return;
    }

    let mut i: i32 = 0;
    i = 0;

    while i < (*oc).n_symbols {
        if !(*(*oc).symbols.offset(i as isize)).name.is_null() {
            ghciRemoveSymbolTable(symhash, (*(*oc).symbols.offset(i as isize)).name, oc);
        }

        i += 1;
    }

    stgFree((*oc).symbols as *mut c_void);
    (*oc).symbols = null_mut::<Symbol_t>();
}

unsafe fn freeOcStablePtrs(mut oc: *mut ObjectCode) {
    let mut exports = null_mut::<ForeignExportsList>();
    let mut next = null_mut::<ForeignExportsList>();
    exports = (*oc).foreign_exports;

    while !exports.is_null() {
        next = (*exports).next;

        let mut i = 0;

        while i < (*exports).n_entries {
            freeStablePtr(*(*exports).stable_ptrs.offset(i as isize) as StgStablePtr);
            i += 1;
        }

        stgFree((*exports).stable_ptrs as *mut c_void);
        (*exports).stable_ptrs = null_mut::<*mut StgStablePtr>();
        (*exports).next = null_mut::<ForeignExportsList>();
        exports = next;
    }

    (*oc).foreign_exports = null_mut::<ForeignExportsList>();
}

unsafe fn freePreloadObjectFile(mut oc: *mut ObjectCode) {
    if RTS_LINKER_USE_MMAP != 0 && (*oc).imageMapped != 0 {
        munmapForLinker(
            (*oc).image as *mut c_void,
            (*oc).fileSize as usize,
            c"freePreloadObjectFile".as_ptr(),
        );
    } else {
        stgFree((*oc).image as *mut c_void);
    }

    (*oc).image = null_mut::<c_char>();
    (*oc).fileSize = 0;
}

unsafe fn freeObjectCode(mut oc: *mut ObjectCode) {
    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: freeObjectCode: start\n".as_ptr(),
            c"freeObjectCode".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    if (*oc).r#type as u32 == STATIC_OBJECT as i32 as u32
        && ((*oc).status as u32 == OBJECT_READY as i32 as u32
            || (*oc).status as u32 == OBJECT_UNLOADED as i32 as u32)
    {
        ocRunFini_MachO(oc);
    }

    if (*oc).cxa_finalize.is_some() {
        (*oc).cxa_finalize.expect("non-null function pointer")((*oc).image as *mut c_void);
    }

    if (*oc).r#type as u32 == DYNAMIC_OBJECT as i32 as u32 {
        let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Linker.c".as_ptr(),
                1144,
                __r,
            );
        }

        freeNativeCode_POSIX(oc);

        if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Linker.c".as_ptr(),
                1146,
            );
        }
    }

    freePreloadObjectFile(oc);

    if !(*oc).symbols.is_null() {
        stgFree((*oc).symbols as *mut c_void);
        (*oc).symbols = null_mut::<Symbol_t>();
    }

    if !(*oc).extraInfos.is_null() {
        freeStrHashTable((*oc).extraInfos, None);
        (*oc).extraInfos = null_mut::<StrHashTable>();
    }

    if !(*oc).sections.is_null() {
        let mut i: i32 = 0;
        i = 0;

        while i < (*oc).n_sections {
            if !(*(*oc).sections.offset(i as isize)).start.is_null() {
                match (*(*oc).sections.offset(i as isize)).alloc as u32 {
                    2 => {
                        munmapForLinker(
                            (*(*oc).sections.offset(i as isize)).mapped_start,
                            (*(*oc).sections.offset(i as isize)).mapped_size as usize,
                            c"freeObjectCode".as_ptr(),
                        );
                    }
                    3 => {
                        if RtsFlags.DebugFlags.zero_on_gc {
                            memset(
                                (*(*oc).sections.offset(i as isize)).start,
                                0,
                                (*(*oc).sections.offset(i as isize)).size as usize,
                            );
                        }

                        stgFree((*(*oc).sections.offset(i as isize)).start);
                    }
                    1 | _ => {}
                }
            }

            if !(*(*oc).sections.offset(i as isize)).info.is_null() {
                stgFree((*(*oc).sections.offset(i as isize)).info as *mut c_void);
            }

            i += 1;
        }

        stgFree((*oc).sections as *mut c_void);
    }

    freeProddableBlocks(&raw mut (*oc).proddables);
    freeSegments(oc);
    USE_CONTIGUOUS_MMAP == 0
        && !RtsFlags.MiscFlags.linkerAlwaysPic
        && !(*oc).symbol_extras.is_null();
    m32_allocator_free((*oc).rx_m32);
    m32_allocator_free((*oc).rw_m32);
    stgFree((*oc).fileName as *mut c_void);
    stgFree((*oc).archiveMemberName as *mut c_void);
    freeHashSet((*oc).dependencies);
    stgFree(oc as *mut c_void);
}

unsafe fn mkOc(
    mut r#type: ObjectType,
    mut path: *mut pathchar,
    mut image: *mut c_char,
    mut imageSize: i32,
    mut mapped: bool,
    mut archiveMemberName: *mut pathchar,
    mut misalignment: i32,
) -> *mut ObjectCode {
    let mut oc = null_mut::<ObjectCode>();

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"mkOc: %s\n".as_ptr(), path);
    }

    oc = stgMallocBytes(size_of::<ObjectCode>() as usize, c"mkOc(oc)".as_ptr()) as *mut ObjectCode;
    (*oc).info = null_mut::<ObjectCodeFormatInfo>();
    (*oc).r#type = r#type;
    (*oc).formatName = c"Mach-O".as_ptr();
    (*oc).image = image;
    (*oc).fileName = pathdup(path);

    if !archiveMemberName.is_null() {
        (*oc).archiveMemberName = stgMallocBytes(
            strlen(archiveMemberName)
                .wrapping_add(1 as usize)
                .wrapping_mul(pathsize),
            c"loadObj".as_ptr(),
        ) as *mut pathchar;

        strcpy((*oc).archiveMemberName as *mut c_char, archiveMemberName);
    } else {
        (*oc).archiveMemberName = null_mut::<pathchar>();
    }

    if (*oc).archiveMemberName.is_null() {
        (*oc).status = OBJECT_NEEDED;
    } else {
        (*oc).status = OBJECT_LOADED;
    }

    (*oc).fileSize = imageSize;
    (*oc).n_symbols = 0;
    (*oc).symbols = null_mut::<Symbol_t>();
    (*oc).n_sections = 0;
    (*oc).sections = null_mut::<Section>();
    (*oc).n_segments = 0;
    (*oc).segments = null_mut::<Segment>();
    initProddableBlockSet(&raw mut (*oc).proddables);
    (*oc).foreign_exports = null_mut::<ForeignExportsList>();
    (*oc).symbol_extras = null_mut::<SymbolExtra>();
    (*oc).bssBegin = null_mut::<c_char>();
    (*oc).bssEnd = null_mut::<c_char>();
    (*oc).imageMapped = mapped as i32;
    (*oc).misalignment = misalignment;
    (*oc).cxa_finalize = None;
    (*oc).extraInfos = null_mut::<StrHashTable>();
    (*oc).next = null_mut::<_ObjectCode>();
    (*oc).prev = null_mut::<_ObjectCode>();
    (*oc).next_loaded_object = null_mut::<_ObjectCode>();
    (*oc).mark = object_code_mark_bit as StgWord;
    (*oc).unloadable = true;
    (*oc).dependencies = allocHashSet();
    (*oc).rw_m32 = m32_allocator_new(false);
    (*oc).rx_m32 = m32_allocator_new(true);
    (*oc).nc_ranges = null_mut::<NativeCodeRange>();
    (*oc).dlopen_handle = NULL;

    return oc;
}

unsafe fn isAlreadyLoaded(mut path: *mut pathchar) -> HsInt {
    let mut o = objects;

    while !o.is_null() {
        if 0 == strcmp((*o).fileName, path) && (*o).status as u32 != OBJECT_UNLOADED as i32 as u32 {
            return 1;
        }

        o = (*o).next as *mut ObjectCode;
    }

    return 0;
}

unsafe fn preloadObjectFile(mut path: *mut pathchar) -> *mut ObjectCode {
    let mut fileSize: i32 = 0;

    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };

    let mut r: i32 = 0;
    let mut image = null_mut::<c_void>();
    let mut oc = null_mut::<ObjectCode>();
    let mut misalignment = 0;
    r = stat(path, &raw mut st);

    if r == -1 {
        errorBelch(c"loadObj: %s: file doesn't exist".as_ptr(), path);

        return null_mut::<ObjectCode>();
    }

    fileSize = st.st_size as i32;

    let mut fd: i32 = 0;
    fd = open(path, O_RDONLY);

    if fd == -1 {
        errorBelch(c"loadObj: can't open %s".as_ptr(), path);

        return null_mut::<ObjectCode>();
    }

    image = mmapForLinker(fileSize as usize, MEM_READ_WRITE, MAP_PRIVATE as u32, fd, 0);

    if image == MAP_FAILED {
        errorBelch(c"mmap: failed. errno = %d".as_ptr(), *__error());
    }

    close(fd);

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadObj: preloaded image at %p\n".as_ptr(), image);
    }

    oc = mkOc(
        STATIC_OBJECT,
        path,
        image as *mut c_char,
        fileSize,
        true,
        null_mut::<pathchar>(),
        misalignment,
    );

    if ocVerifyImage_MachO(oc) != 0 {
        ocInit_MachO(oc);
    }

    return oc;
}

unsafe fn loadObj_(mut path: *mut pathchar) -> HsInt {
    if isAlreadyLoaded(path) != 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(c"ignoring repeated load of %s\n".as_ptr(), path);
        }

        return 1;
    }

    if isArchive(path) {
        if loadArchive_(path) != 0 {
            return 1;
        } else if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"tried and failed to load %s as an archive\n".as_ptr(),
                path,
            );
        }
    }

    let mut oc = preloadObjectFile(path);

    if oc.is_null() {
        return 0;
    }

    if loadOc(oc) == 0 {
        removeOcSymbols(oc);
        freeObjectCode(oc);

        return 0;
    }

    insertOCSectionIndices(oc);
    (*oc).next_loaded_object = loaded_objects as *mut _ObjectCode;
    loaded_objects = oc;

    return 1;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadObj(mut path: *mut pathchar) -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1502,
            __r,
        );
    }

    let mut r = loadObj_(path);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1504,
        );
    }

    return r;
}

unsafe fn loadOc(mut oc: *mut ObjectCode) -> HsInt {
    let mut r: i32 = 0;

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: start\n".as_ptr(),
            c"loadOc".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    r = ocVerifyImage_MachO(oc);

    if r == 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s(%s: ocVerifyImage_* failed\n".as_ptr(),
                c"loadOc".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );
        }

        return r as HsInt;
    }

    r = ocAllocateExtras_MachO(oc);

    if r == 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s(%s: ocAllocateExtras_MachO failed\n".as_ptr(),
                c"loadOc".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );
        }

        return r as HsInt;
    }

    r = ocGetNames_MachO(oc);

    if r == 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s(%s: ocGetNames_* failed\n".as_ptr(),
                c"loadOc".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );
        }

        return r as HsInt;
    }

    if (*oc).status as u32 != OBJECT_DONT_RESOLVE as i32 as u32 {
        if (*oc).archiveMemberName.is_null() {
            (*oc).status = OBJECT_NEEDED;
        } else {
            (*oc).status = OBJECT_LOADED;
        }
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: done\n".as_ptr(),
            c"loadOc".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    return 1;
}

unsafe fn ocTryLoad(mut oc: *mut ObjectCode) -> i32 {
    let mut r: i32 = 0;

    if (*oc).status as u32 != OBJECT_NEEDED as i32 as u32 {
        return 1;
    }

    let mut x: i32 = 0;

    let mut symbol = _Symbol {
        name: null_mut::<SymbolName>(),
        addr: null_mut::<c_void>(),
        r#type: 0,
    };

    x = 0;

    while x < (*oc).n_symbols {
        symbol = *(*oc).symbols.offset(x as isize);

        if !symbol.name.is_null()
            && ghciInsertSymbolTable(
                (*oc).fileName,
                symhash,
                symbol.name,
                symbol.addr,
                isSymbolWeak(oc, symbol.name as *const c_void) as SymStrength,
                symbol.r#type,
                oc,
            ) == 0
        {
            return 0;
        }

        x += 1;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: resolving\n".as_ptr(),
            c"ocTryLoad".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    r = ocResolve_MachO(oc);

    if r == 0 {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s(%s: resolution failed\n".as_ptr(),
                c"ocTryLoad".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );
        }

        return r;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: protecting mappings\n".as_ptr(),
            c"ocTryLoad".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    ocProtectExtras(oc);
    m32_allocator_flush((*oc).rx_m32);
    m32_allocator_flush((*oc).rw_m32);

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: resolved\n".as_ptr(),
            c"ocTryLoad".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    (*oc).status = OBJECT_RESOLVED;

    return 1;
}

unsafe fn ocRunInit(mut oc: *mut ObjectCode) -> i32 {
    if (*oc).status as u32 != OBJECT_RESOLVED as i32 as u32 {
        return 1;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"%s(%s: running initializers\n".as_ptr(),
            c"ocRunInit".as_ptr(),
            if !(*oc).archiveMemberName.is_null() {
                (*oc).archiveMemberName
            } else {
                (*oc).fileName
            },
        );
    }

    foreignExportsLoadingObject(oc);

    let mut r: i32 = 0;
    r = ocRunInit_MachO(oc);
    foreignExportsFinishedLoadingObject();

    if r == 0 {
        return r;
    }

    (*oc).status = OBJECT_READY;

    return 1;
}

unsafe fn runPendingInitializers() -> i32 {
    let mut oc = objects;

    while !oc.is_null() {
        let mut r = ocRunInit(oc);

        if r == 0 {
            errorBelch(
                c"Could not run initializers of Object Code %s.\n".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            if RtsFlags.DebugFlags.linker {
                printLoadedObjects();
            }

            fflush(__stderrp);

            return r;
        }

        oc = (*oc).next as *mut ObjectCode;
    }

    refreshProfilingCCSs();

    return 1;
}

unsafe fn resolveObjs_() -> HsInt {
    if RtsFlags.DebugFlags.linker {
        debugBelch(c"resolveObjs: start\n".as_ptr());
    }

    let mut oc = objects;

    while !oc.is_null() {
        let mut r = ocTryLoad(oc);

        if r == 0 {
            errorBelch(
                c"Could not load Object Code %s.\n".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            if RtsFlags.DebugFlags.linker {
                printLoadedObjects();
            }

            fflush(__stderrp);

            return r as HsInt;
        }

        oc = (*oc).next as *mut ObjectCode;
    }

    if runPendingInitializers() == 0 {
        return 0;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"resolveObjs: done\n".as_ptr());
    }

    return 1;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1742,
            __r,
        );
    }

    let mut r = resolveObjs_();

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1744,
        );
    }

    return r;
}

unsafe fn unloadObj_(mut path: *mut pathchar, mut just_purge: bool) -> HsInt {
    if !symhash.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 1753);
    }

    if !objects.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Linker.c".as_ptr(), 1754);
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"unloadObj: %s\n".as_ptr(), path);
    }

    let mut unloadedAnyObj = false;
    let mut prev = null_mut::<ObjectCode>();
    let mut oc = loaded_objects;

    while !oc.is_null() {
        if strcmp((*oc).fileName, path) == 0 {
            (*oc).status = OBJECT_UNLOADED;
            removeOcSymbols(oc);
            freeOcStablePtrs(oc);
            unloadedAnyObj = true;

            if !just_purge {
                n_unloaded_objects += 1;

                if prev.is_null() {
                    loaded_objects = (*oc).next_loaded_object as *mut ObjectCode;
                } else {
                    (*prev).next_loaded_object = (*oc).next_loaded_object;
                }
            }
        } else {
            prev = oc;
        }

        oc = (*oc).next_loaded_object as *mut ObjectCode;
    }

    if unloadedAnyObj {
        return 1;
    } else {
        errorBelch(c"unloadObj: can't find `%s' to unload".as_ptr(), path);

        return 0;
    };
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadObj(mut path: *mut pathchar) -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1797,
            __r,
        );
    }

    let mut r = unloadObj_(path, false);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1799,
        );
    }

    return r;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn purgeObj(mut path: *mut pathchar) -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1805,
            __r,
        );
    }

    let mut r = unloadObj_(path, true);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1807,
        );
    }

    return r;
}

unsafe fn lookupObjectByPath(mut path: *mut pathchar) -> *mut ObjectCode {
    let mut o = objects;

    while !o.is_null() {
        if 0 == strcmp((*o).fileName, path) {
            return o;
        }

        o = (*o).next as *mut ObjectCode;
    }

    return null_mut::<ObjectCode>();
}

unsafe fn getObjectLoadStatus_(mut path: *mut pathchar) -> OStatus {
    let mut oc = lookupObjectByPath(path);

    if !oc.is_null() {
        return (*oc).status;
    }

    return OBJECT_NOT_LOADED;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getObjectLoadStatus(mut path: *mut pathchar) -> OStatus {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1831,
            __r,
        );
    }

    let mut r = getObjectLoadStatus_(path);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1833,
        );
    }

    return r;
}

unsafe fn addSection(
    mut s: *mut Section,
    mut kind: SectionKind,
    mut alloc: SectionAlloc,
    mut start: *mut c_void,
    mut size: StgWord,
    mut mapped_offset: StgWord,
    mut mapped_start: *mut c_void,
    mut mapped_size: StgWord,
) {
    (*s).start = start;
    (*s).size = size;
    (*s).kind = kind;
    (*s).alloc = alloc;
    (*s).mapped_offset = mapped_offset;
    (*s).mapped_start = mapped_start;
    (*s).mapped_size = mapped_size;

    if (*s).info.is_null() {
        (*s).info = stgCallocBytes(
            1,
            size_of::<SectionFormatInfo>() as usize,
            c"addSection(SectionFormatInfo)".as_ptr(),
        ) as *mut SectionFormatInfo;
    }

    if RtsFlags.DebugFlags.linker {
        debugBelch(
            c"addSection: %p-%p (size %llu), kind %d\n".as_ptr(),
            start,
            (start as StgWord).wrapping_add(size) as *mut c_void,
            size,
            kind as u32,
        );
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadNativeObj(
    mut path: *mut pathchar,
    mut errmsg: *mut *mut c_char,
) -> *mut c_void {
    if RtsFlags.DebugFlags.linker {
        debugBelch(c"loadNativeObj: path = '%s'\n".as_ptr(), path);
    }

    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1868,
            __r,
        );
    }

    let mut r = loadNativeObj_POSIX(path, errmsg);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1889,
        );
    }

    return r;
}

unsafe fn unloadNativeObj_(mut handle: *mut c_void) -> HsInt {
    let mut unloadedAnyObj = false;

    if RtsFlags.DebugFlags.linker {
        debugBelch(c"unloadNativeObj: %p\n".as_ptr(), handle);
    }

    let mut prev = null_mut::<ObjectCode>();
    let mut next = null_mut::<ObjectCode>();
    let mut nc = loaded_objects;

    while !nc.is_null() {
        next = (*nc).next_loaded_object as *mut ObjectCode;

        if (*nc).r#type as u32 == DYNAMIC_OBJECT as i32 as u32 && (*nc).dlopen_handle == handle {
            (*nc).status = OBJECT_UNLOADED;
            n_unloaded_objects += 1;

            if (*nc).symbols.is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Linker.c".as_ptr(), 1908);
            }

            freeOcStablePtrs(nc);

            if prev.is_null() {
                loaded_objects = (*nc).next_loaded_object as *mut ObjectCode;
            } else {
                (*prev).next_loaded_object = (*nc).next_loaded_object;
            }

            unloadedAnyObj = true;
        } else {
            prev = nc;
        }

        nc = next;
    }

    if unloadedAnyObj {
        return 1;
    } else {
        errorBelch(
            c"unloadObjNativeObj_: can't find `%p' to unload".as_ptr(),
            handle,
        );

        return 0;
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadNativeObj(mut handle: *mut c_void) -> HsInt {
    let mut __r = pthread_mutex_lock(&raw mut linker_mutex);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1932,
            __r,
        );
    }

    let mut r = unloadNativeObj_(handle);

    if pthread_mutex_unlock(&raw mut linker_mutex) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Linker.c".as_ptr(),
            1934,
        );
    }

    return r;
}

unsafe fn initSegment(
    mut s: *mut Segment,
    mut start: *mut c_void,
    mut size: usize,
    mut prot: SegmentProt,
    mut n_sections: i32,
) {
    (*s).start = start;
    (*s).size = size;
    (*s).prot = prot;

    (*s).sections_idx = stgCallocBytes(
        n_sections as usize,
        size_of::<i32>() as usize,
        c"initSegment(segment)".as_ptr(),
    ) as *mut i32;

    (*s).n_sections = n_sections;
}

unsafe fn freeSegments(mut oc: *mut ObjectCode) {
    if !(*oc).segments.is_null() {
        if RtsFlags.DebugFlags.linker {
            debugBelch(
                c"%s(%s: freeing %d segments\n".as_ptr(),
                c"freeSegments".as_ptr(),
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
                (*oc).n_segments,
            );
        }

        let mut i = 0;

        while i < (*oc).n_segments {
            let mut s: *mut Segment = (*oc).segments.offset(i as isize) as *mut Segment;

            if RtsFlags.DebugFlags.linker {
                debugBelch(
                    c"%s(%s: freeing segment %d at %p size %zu\n".as_ptr(),
                    c"freeSegments".as_ptr(),
                    if !(*oc).archiveMemberName.is_null() {
                        (*oc).archiveMemberName
                    } else {
                        (*oc).fileName
                    },
                    i,
                    (*s).start,
                    (*s).size,
                );
            }

            stgFree((*s).sections_idx as *mut c_void);
            (*s).sections_idx = null_mut::<i32>();

            if 0 == (*s).size {
                if RtsFlags.DebugFlags.linker {
                    debugBelch(
                        c"%s(%s: skipping segment of 0 size\n".as_ptr(),
                        c"freeSegments".as_ptr(),
                        if !(*oc).archiveMemberName.is_null() {
                            (*oc).archiveMemberName
                        } else {
                            (*oc).fileName
                        },
                    );
                }
            } else {
                munmapForLinker((*s).start, (*s).size, c"freeSegments".as_ptr());
                (*s).start = NULL;
            }

            i += 1;
        }

        stgFree((*oc).segments as *mut c_void);
        (*oc).segments = null_mut::<Segment>();
    }
}
