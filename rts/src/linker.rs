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
    pub(crate) fileSize: c_int,
    pub(crate) formatName: *mut c_char,
    pub(crate) r#type: ObjectType,
    pub(crate) archiveMemberName: *mut pathchar,
    pub(crate) symbols: *mut Symbol_t,
    pub(crate) n_symbols: c_int,
    pub(crate) image: *mut c_char,
    pub(crate) info: *mut ObjectCodeFormatInfo,
    pub(crate) imageMapped: c_int,
    pub(crate) misalignment: c_int,
    pub(crate) cxa_finalize: cxa_finalize_fn,
    pub(crate) n_sections: c_int,
    pub(crate) sections: *mut Section,
    pub(crate) n_segments: c_int,
    pub(crate) segments: *mut Segment,
    pub(crate) next: *mut _ObjectCode,
    pub(crate) prev: *mut _ObjectCode,
    pub(crate) next_loaded_object: *mut _ObjectCode,
    pub(crate) mark: StgWord,
    pub(crate) unloadable: bool,
    pub(crate) dependencies: *mut HashSet,
    pub(crate) proddables: ProddableBlockSet,
    pub(crate) symbol_extras: *mut SymbolExtra,
    pub(crate) first_symbol_extra: c_ulong,
    pub(crate) n_symbol_extras: c_ulong,
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
    pub(crate) size: size_t,
    pub(crate) prot: SegmentProt,
    pub(crate) sections_idx: *mut c_int,
    pub(crate) n_sections: c_int,
}

pub(crate) type SegmentProt = c_uint;

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

pub(crate) type SectionAlloc = c_uint;

pub(crate) const SECTION_MALLOC: SectionAlloc = 3;

pub(crate) const SECTION_MMAP: SectionAlloc = 2;

pub(crate) const SECTION_M32: SectionAlloc = 1;

pub(crate) const SECTION_NOMEM: SectionAlloc = 0;

pub(crate) type SectionKind = c_uint;

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

pub(crate) type ObjectType = c_uint;

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

pub(crate) const USE_CONTIGUOUS_MMAP: c_int = 0 as c_int;

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

static mut symhash: *mut StrHashTable = null::<StrHashTable>() as *mut StrHashTable;

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
    match r#type as c_uint & !(SYM_TYPE_DUP_DISCARD as c_int | SYM_TYPE_HIDDEN as c_int) as c_uint {
        1 => return b"code\0" as *const u8 as *const c_char,
        2 => return b"data\0" as *const u8 as *const c_char,
        4 => return b"indirect-data\0" as *const u8 as *const c_char,
        _ => {
            barf(
                b"symbolTypeString: unknown symbol type (%d)\0" as *const u8 as *const c_char,
                r#type as c_uint,
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
) -> c_int {
    let mut pinfo = lookupStrHashTable(table, key as *const c_char) as *mut RtsSymbolInfo;

    if pinfo.is_null() {
        pinfo = stgMallocBytes(
            size_of::<RtsSymbolInfo>() as size_t,
            b"ghciInsertToSymbolTable\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut RtsSymbolInfo;

        (*pinfo).value = data;
        (*pinfo).owner = owner;
        (*pinfo).strength = strength;
        (*pinfo).r#type = r#type;
        insertStrHashTable(table, key as *const c_char, pinfo as *const c_void);

        return 1 as c_int;
    } else if (*pinfo).r#type as c_uint ^ r#type as c_uint != 0 {
        if (*pinfo).r#type as c_uint & SYM_TYPE_HIDDEN as c_int as c_uint != 0 {
            (*pinfo).value = data;
            (*pinfo).owner = owner;
            (*pinfo).strength = strength;
            (*pinfo).r#type = r#type;

            return 1 as c_int;
        }

        if r#type as c_uint & (SYM_TYPE_DUP_DISCARD as c_int | SYM_TYPE_HIDDEN as c_int) as c_uint
            == 0
        {
            debugBelch(
                b"Symbol type mismatch (existing %d, new %d).\n\0" as *const u8 as *const c_char,
                (*pinfo).r#type as c_uint,
                r#type as c_uint,
            );

            debugBelch(
                b"Symbol %s was defined by %s to be a %s symbol.\n\0" as *const u8 as *const c_char,
                key,
                obj_name,
                symbolTypeString(r#type),
            );

            debugBelch(
                b"      yet was defined by %s to be a %s symbol.\n\0" as *const u8 as *const c_char,
                if !(*pinfo).owner.is_null() {
                    (*(*pinfo).owner).fileName as *const pathchar
                } else {
                    b"<builtin>\0" as *const u8 as *const pathchar
                },
                symbolTypeString((*pinfo).r#type),
            );
        }

        return 1 as c_int;
    } else if (*pinfo).strength as c_uint == STRENGTH_STRONG as c_int as c_uint {
        return 1 as c_int;
    } else if strength as c_uint == STRENGTH_WEAK as c_int as c_uint
        && !data.is_null()
        && (*pinfo).strength as c_uint == STRENGTH_WEAK as c_int as c_uint
        && (*pinfo).value.is_null()
    {
        (*pinfo).value = data;
        (*pinfo).owner = owner;

        return 1 as c_int;
    } else if strength as c_uint == STRENGTH_WEAK as c_int as c_uint {
        return 1 as c_int;
    } else if (*pinfo).strength as c_uint == STRENGTH_WEAK as c_int as c_uint
        && strength as c_uint != STRENGTH_WEAK as c_int as c_uint
    {
        (*pinfo).value = data;
        (*pinfo).owner = owner;
        (*pinfo).strength = strength;

        return 1 as c_int;
    } else if !(*pinfo).owner.is_null()
        && (*(*pinfo).owner).status as c_uint != OBJECT_READY as c_int as c_uint
        && (*(*pinfo).owner).status as c_uint != OBJECT_RESOLVED as c_int as c_uint
        && (*(*pinfo).owner).status as c_uint != OBJECT_NEEDED as c_int as c_uint
    {
        if !owner.is_null()
            && ((*owner).status as c_uint == OBJECT_NEEDED as c_int as c_uint
                || (*owner).status as c_uint == OBJECT_RESOLVED as c_int as c_uint
                || (*owner).status as c_uint == OBJECT_READY as c_int as c_uint)
        {
            (*pinfo).value = data;
            (*pinfo).owner = owner;
            (*pinfo).strength = strength;
        }

        return 1 as c_int;
    } else if (*pinfo).owner == owner {
        return 1 as c_int;
    } else if !owner.is_null() && (*owner).status as c_uint == OBJECT_LOADED as c_int as c_uint {
        return 1 as c_int;
    }

    debugBelch(
        b"GHC runtime linker: fatal error: I found a duplicate definition for symbol\n   %s\nwhilst processing object file\n   %s\nThe symbol was previously defined in\n   %s\nThis could be caused by:\n   * Loading two different object files which export the same symbol\n   * Specifying the same object file twice on the GHCi command line\n   * An incorrect `package.conf' entry, causing some object to be\n     loaded twice.\n\0"
            as *const u8 as *const c_char,
        key as *mut c_char,
        obj_name,
        if (*pinfo).owner.is_null() {
            b"(GHCi built-in symbols)\0" as *const u8 as *const c_char
        } else {
            (if !(*(*pinfo).owner).archiveMemberName.is_null() {
                (*(*pinfo).owner).archiveMemberName
            } else {
                (*(*pinfo).owner).fileName
            }) as *const c_char
        },
    );

    return 0 as c_int;
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

    if (*pinfo).strength as c_uint == STRENGTH_WEAK as c_int as c_uint {
        (*pinfo).strength = STRENGTH_NORMAL;
    }

    *result = pinfo;

    return HS_BOOL_TRUE as HsBool;
}

static mut linker_init_done: c_int = 0 as c_int;

static mut dl_prog_handle: *mut c_void = null::<c_void>() as *mut c_void;

static mut re_invalid: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: null::<c_char>(),
    re_g: null::<re_guts>() as *mut re_guts,
};

static mut re_realso: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: null::<c_char>(),
    re_g: null::<re_guts>() as *mut re_guts,
};

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker() {
    initLinker_(1 as c_int);
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker_(mut retain_cafs: c_int) {
    let mut compileResult: c_int = 0;

    if linker_init_done == 1 as c_int {
        return;
    } else {
        linker_init_done = 1 as c_int;
    }

    initUnloadCheck();
    symhash = allocStrHashTable();

    let mut sym: *const RtsSymbolVal = &raw mut rtsSyms as *mut RtsSymbolVal;

    while !(*sym).lbl.is_null() {
        if ghciInsertSymbolTable(
            b"(GHCi built-in symbols)\0" as *const u8 as *const c_char as *mut pathchar,
            symhash,
            (*sym).lbl,
            (*sym).addr,
            (*sym).strength,
            (*sym).r#type,
            null_mut::<ObjectCode>(),
        ) == 0
        {
            barf(b"ghciInsertSymbolTable failed\0" as *const u8 as *const c_char);
        }

        sym = sym.offset(1);
    }

    if Some(rtsExtraSyms as unsafe extern "C" fn() -> *mut RtsSymbolVal).is_some()
        && !rtsExtraSyms().is_null()
    {
        let mut sym_0 = rtsExtraSyms();

        while !(*sym_0).lbl.is_null() {
            if ghciInsertSymbolTable(
                b"(GHCi built-in symbols)\0" as *const u8 as *const c_char as *mut pathchar,
                symhash,
                (*sym_0).lbl,
                (*sym_0).addr,
                (*sym_0).strength,
                (*sym_0).r#type,
                null_mut::<ObjectCode>(),
            ) == 0
            {
                barf(b"ghciInsertSymbolTable failed\0" as *const u8 as *const c_char);
            }

            sym_0 = sym_0.offset(1);
        }
    }

    if ghciInsertSymbolTable(
        b"(GHCi built-in symbols)\0" as *const u8 as *const c_char as *mut pathchar,
        symhash,
        b"_newCAF\0" as *const u8 as *const SymbolName,
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
        barf(b"ghciInsertSymbolTable failed\0" as *const u8 as *const c_char);
    }

    dl_prog_handle = RTLD_DEFAULT;

    compileResult = regcomp(
        &raw mut re_invalid,
        b"(([^ \t()])+\\.so([^ \t:()])*):([ \t])*(invalid ELF header|file too short|invalid file format|Exec format error)\0"
            as *const u8 as *const c_char,
        REG_EXTENDED,
    );

    if compileResult != 0 as c_int {
        barf(b"Compiling re_invalid failed\0" as *const u8 as *const c_char);
    }

    compileResult = regcomp(
        &raw mut re_realso,
        b"(GROUP|INPUT) *\\( *([^ )]+)\0" as *const u8 as *const c_char,
        REG_EXTENDED,
    );

    if compileResult != 0 as c_int {
        barf(b"Compiling re_realso failed\0" as *const u8 as *const c_char);
    }
}

unsafe fn exitLinker() {
    if linker_init_done == 1 as c_int {
        regfree(&raw mut re_invalid);
        regfree(&raw mut re_realso);
    }

    if linker_init_done == 1 as c_int {
        freeStrHashTable(
            symhash,
            Some(free as unsafe extern "C" fn(*mut c_void) -> ()),
        );

        exitUnloadCheck();
    }
}

unsafe fn internal_dlsym(mut symbol: *const c_char) -> *mut c_void {
    let mut v = null_mut::<c_void>();
    dlerror();
    v = dlsym(dl_prog_handle, symbol);

    if dlerror().is_null() {
        return v;
    }

    let mut nc = loaded_objects;

    while !nc.is_null() {
        if (*nc).r#type as c_uint == DYNAMIC_OBJECT as c_int as c_uint {
            v = dlsym((*nc).dlopen_handle, symbol);

            if dlerror().is_null() {
                return v;
            }
        }

        nc = (*nc).next_loaded_object as *mut ObjectCode;
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
    if (*symbol_name.offset(0 as c_int as isize) as c_int == '_' as i32) as c_int as c_long != 0 {
    } else {
        _assertFail(
            b"rts/Linker.c\0" as *const u8 as *const c_char,
            673 as c_uint,
        );
    }

    symbol_name = symbol_name.offset(1 as c_int as isize);

    let mut result = dlsym(handle, symbol_name);

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
        return errmsg;
    };
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn findSystemLibrary(mut dll_name: *mut pathchar) -> *mut pathchar {
    return null_mut::<pathchar>();
}

unsafe fn warnMissingKBLibraryPaths() {
    static mut missing_update_warn: HsBool = HS_BOOL_FALSE as HsBool;

    if missing_update_warn == 0 {
        debugBelch(
            b"Warning: If linking fails, consider installing KB2533623.\n\0" as *const u8
                as *const c_char,
        );

        missing_update_warn = HS_BOOL_TRUE as HsBool;
    }
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn addLibrarySearchPath(mut dll_path: *mut pathchar) -> HsPtr {
    return NULL;
}

#[ffi(compiler, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn removeLibrarySearchPath(mut dll_path_index: HsPtr) -> HsBool {
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
    let mut pinfo = null_mut::<RtsSymbolInfo>();

    if strcmp(lbl, b"___dso_handle\0" as *const u8 as *const c_char) == 0 as c_int {
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

    if strcmp(lbl, b"___cxa_atexit\0" as *const u8 as *const c_char) == 0 as c_int
        && !dependent.is_null()
    {
        (*dependent).cxa_finalize =
            transmute::<*mut c_void, cxa_finalize_fn>(lookupDependentSymbol(
                b"___cxa_finalize\0" as *const u8 as *const c_char as *mut SymbolName,
                dependent,
                null_mut::<SymType>(),
            ));
    }

    if ghciLookupSymbolInfo(symhash, lbl, &raw mut pinfo) == 0 {
        if (*lbl.offset(0 as c_int as isize) as c_int == '_' as i32) as c_int as c_long != 0 {
        } else {
            _assertFail(
                b"rts/Linker.c\0" as *const u8 as *const c_char,
                866 as c_uint,
            );
        }

        if !r#type.is_null() {
            *r#type = SYM_TYPE_CODE;
        }

        return internal_dlsym(lbl.offset(1 as c_int as isize)) as *mut c_void;
    } else {
        static mut RTS_NO_FINI: *mut c_void = NULL;

        if strcmp(lbl, b"__fini_array_end\0" as *const u8 as *const c_char) == 0 as c_int {
            return &raw mut RTS_NO_FINI as *mut c_void;
        }

        if strcmp(lbl, b"__fini_array_start\0" as *const u8 as *const c_char) == 0 as c_int {
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
    let mut oc = (*pinfo).owner;

    if !oc.is_null() && !lbl.is_null() && (*oc).status as c_uint == OBJECT_LOADED as c_int as c_uint
    {
        (*oc).status = OBJECT_NEEDED;

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
            let mut i: c_int = 0;

            printf(
                b"%s\n\0" as *const u8 as *const c_char,
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            i = 0 as c_int;

            while i < (*oc).n_sections {
                if !(*(*oc).sections.offset(i as isize)).mapped_start.is_null()
                    || !(*(*oc).sections.offset(i as isize)).start.is_null()
                {
                    printf(
                        b"\tsec %2d[alloc: %d; kind: %d]: %p - %p; mmaped: %p - %p\n\0" as *const u8
                            as *const c_char,
                        i,
                        (*(*oc).sections.offset(i as isize)).alloc as c_uint,
                        (*(*oc).sections.offset(i as isize)).kind as c_uint,
                        (*(*oc).sections.offset(i as isize)).start,
                        ((*(*oc).sections.offset(i as isize)).start as uintptr_t as StgWord)
                            .wrapping_add((*(*oc).sections.offset(i as isize)).size)
                            as *mut c_void,
                        (*(*oc).sections.offset(i as isize)).mapped_start,
                        ((*(*oc).sections.offset(i as isize)).mapped_start as uintptr_t as StgWord)
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
    let mut r = lookupDependentSymbol(lbl, null_mut::<ObjectCode>(), null_mut::<SymType>());

    if r.is_null() {
        if !RtsFlags.MiscFlags.linkerOptimistic {
            errorBelch(
                b"^^ Could not load '%s', dependency unresolved. See top entry above. You might consider using --optimistic-linking\n\0"
                    as *const u8 as *const c_char,
                lbl,
            );

            fflush(__stderrp);
        } else {
            errorBelch(
                b"^^ Could not load '%s', dependency unresolved, optimistically continuing\n\0"
                    as *const u8 as *const c_char,
                lbl,
            );

            r = 0xdeadbeef as c_uint as *mut c_void as *mut c_void;
        }
    }

    if runPendingInitializers() == 0 {
        errorBelch(b"lookupSymbol: Failed to run initializers.\0" as *const u8 as *const c_char);
    }

    return r as *mut c_void;
}

unsafe fn resolveSymbolAddr(
    mut buffer: *mut pathchar,
    mut size: c_int,
    mut symbol: *mut c_void,
    mut top: *mut uintptr_t,
) -> *mut pathchar {
    return null_mut::<pathchar>();
}

unsafe fn removeOcSymbols(mut oc: *mut ObjectCode) {
    if (*oc).symbols.is_null() {
        return;
    }

    let mut i: c_int = 0;
    i = 0 as c_int;

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

        let mut i = 0 as c_int;

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
            (*oc).fileSize as size_t,
            b"freePreloadObjectFile\0" as *const u8 as *const c_char,
        );
    } else {
        stgFree((*oc).image as *mut c_void);
    }

    (*oc).image = null_mut::<c_char>();
    (*oc).fileSize = 0 as c_int;
}

unsafe fn freeObjectCode(mut oc: *mut ObjectCode) {
    if (*oc).r#type as c_uint == STATIC_OBJECT as c_int as c_uint
        && ((*oc).status as c_uint == OBJECT_READY as c_int as c_uint
            || (*oc).status as c_uint == OBJECT_UNLOADED as c_int as c_uint)
    {
        ocRunFini_MachO(oc);
    }

    if (*oc).cxa_finalize.is_some() {
        (*oc).cxa_finalize.expect("non-null function pointer")((*oc).image as *mut c_void);
    }

    if (*oc).r#type as c_uint == DYNAMIC_OBJECT as c_int as c_uint {
        freeNativeCode_POSIX(oc);
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
        let mut i: c_int = 0;
        i = 0 as c_int;

        while i < (*oc).n_sections {
            if !(*(*oc).sections.offset(i as isize)).start.is_null() {
                match (*(*oc).sections.offset(i as isize)).alloc as c_uint {
                    2 => {
                        munmapForLinker(
                            (*(*oc).sections.offset(i as isize)).mapped_start,
                            (*(*oc).sections.offset(i as isize)).mapped_size as size_t,
                            b"freeObjectCode\0" as *const u8 as *const c_char,
                        );
                    }
                    3 => {
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
    mut imageSize: c_int,
    mut mapped: bool,
    mut archiveMemberName: *mut pathchar,
    mut misalignment: c_int,
) -> *mut ObjectCode {
    let mut oc = null_mut::<ObjectCode>();

    oc = stgMallocBytes(
        size_of::<ObjectCode>() as size_t,
        b"mkOc(oc)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut ObjectCode;

    (*oc).info = null_mut::<ObjectCodeFormatInfo>();
    (*oc).r#type = r#type;
    (*oc).formatName = b"Mach-O\0" as *const u8 as *const c_char as *mut c_char;
    (*oc).image = image;
    (*oc).fileName = pathdup(path);

    if !archiveMemberName.is_null() {
        (*oc).archiveMemberName = stgMallocBytes(
            strlen(archiveMemberName)
                .wrapping_add(1 as size_t)
                .wrapping_mul(pathsize),
            b"loadObj\0" as *const u8 as *const c_char as *mut c_char,
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
    (*oc).n_symbols = 0 as c_int;
    (*oc).symbols = null_mut::<Symbol_t>();
    (*oc).n_sections = 0 as c_int;
    (*oc).sections = null_mut::<Section>();
    (*oc).n_segments = 0 as c_int;
    (*oc).segments = null_mut::<Segment>();
    initProddableBlockSet(&raw mut (*oc).proddables);
    (*oc).foreign_exports = null_mut::<ForeignExportsList>();
    (*oc).symbol_extras = null_mut::<SymbolExtra>();
    (*oc).bssBegin = null_mut::<c_char>();
    (*oc).bssEnd = null_mut::<c_char>();
    (*oc).imageMapped = mapped as c_int;
    (*oc).misalignment = misalignment;
    (*oc).cxa_finalize = None;
    (*oc).extraInfos = null_mut::<StrHashTable>();
    (*oc).next = null_mut::<_ObjectCode>();
    (*oc).prev = null_mut::<_ObjectCode>();
    (*oc).next_loaded_object = null_mut::<_ObjectCode>();
    (*oc).mark = object_code_mark_bit as StgWord;
    (*oc).unloadable = r#true != 0;
    (*oc).dependencies = allocHashSet();
    (*oc).rw_m32 = m32_allocator_new(r#false != 0);
    (*oc).rx_m32 = m32_allocator_new(r#true != 0);
    (*oc).nc_ranges = null_mut::<NativeCodeRange>();
    (*oc).dlopen_handle = NULL;

    return oc;
}

unsafe fn isAlreadyLoaded(mut path: *mut pathchar) -> HsInt {
    let mut o = objects;

    while !o.is_null() {
        if 0 as c_int == strcmp((*o).fileName, path)
            && (*o).status as c_uint != OBJECT_UNLOADED as c_int as c_uint
        {
            return 1 as HsInt;
        }

        o = (*o).next as *mut ObjectCode;
    }

    return 0 as HsInt;
}

unsafe fn preloadObjectFile(mut path: *mut pathchar) -> *mut ObjectCode {
    let mut fileSize: c_int = 0;

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

    let mut r: c_int = 0;
    let mut image = null_mut::<c_void>();
    let mut oc = null_mut::<ObjectCode>();
    let mut misalignment = 0 as c_int;
    r = stat(path, &raw mut st);

    if r == -(1 as c_int) {
        errorBelch(
            b"loadObj: %s: file doesn't exist\0" as *const u8 as *const c_char,
            path,
        );

        return null_mut::<ObjectCode>();
    }

    fileSize = st.st_size as c_int;

    let mut fd: c_int = 0;
    fd = open(path, O_RDONLY);

    if fd == -(1 as c_int) {
        errorBelch(
            b"loadObj: can't open %s\0" as *const u8 as *const c_char,
            path,
        );

        return null_mut::<ObjectCode>();
    }

    image = mmapForLinker(
        fileSize as size_t,
        MEM_READ_WRITE,
        MAP_PRIVATE as uint32_t,
        fd,
        0 as c_int,
    );

    if image == MAP_FAILED {
        errorBelch(
            b"mmap: failed. errno = %d\0" as *const u8 as *const c_char,
            *__error(),
        );
    }

    close(fd);

    oc = mkOc(
        STATIC_OBJECT,
        path,
        image as *mut c_char,
        fileSize,
        r#true != 0,
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
        return 1 as HsInt;
    }

    if isArchive(path) {
        if loadArchive_(path) != 0 {
            return 1 as HsInt;
        }
    }

    let mut oc = preloadObjectFile(path);

    if oc.is_null() {
        return 0 as HsInt;
    }

    if loadOc(oc) == 0 {
        removeOcSymbols(oc);
        freeObjectCode(oc);

        return 0 as HsInt;
    }

    insertOCSectionIndices(oc);
    (*oc).next_loaded_object = loaded_objects as *mut _ObjectCode;
    loaded_objects = oc;

    return 1 as HsInt;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadObj(mut path: *mut pathchar) -> HsInt {
    let mut r = loadObj_(path);

    return r;
}

unsafe fn loadOc(mut oc: *mut ObjectCode) -> HsInt {
    let mut r: c_int = 0;
    r = ocVerifyImage_MachO(oc);

    if r == 0 {
        return r as HsInt;
    }

    r = ocAllocateExtras_MachO(oc);

    if r == 0 {
        return r as HsInt;
    }

    r = ocGetNames_MachO(oc);

    if r == 0 {
        return r as HsInt;
    }

    if (*oc).status as c_uint != OBJECT_DONT_RESOLVE as c_int as c_uint {
        if (*oc).archiveMemberName.is_null() {
            (*oc).status = OBJECT_NEEDED;
        } else {
            (*oc).status = OBJECT_LOADED;
        }
    }

    return 1 as HsInt;
}

unsafe fn ocTryLoad(mut oc: *mut ObjectCode) -> c_int {
    let mut r: c_int = 0;

    if (*oc).status as c_uint != OBJECT_NEEDED as c_int as c_uint {
        return 1 as c_int;
    }

    let mut x: c_int = 0;

    let mut symbol = _Symbol {
        name: null_mut::<SymbolName>(),
        addr: null_mut::<c_void>(),
        r#type: 0 as SymType,
    };

    x = 0 as c_int;

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
            return 0 as c_int;
        }

        x += 1;
    }

    r = ocResolve_MachO(oc);

    if r == 0 {
        return r;
    }

    ocProtectExtras(oc);
    m32_allocator_flush((*oc).rx_m32);
    m32_allocator_flush((*oc).rw_m32);
    (*oc).status = OBJECT_RESOLVED;

    return 1 as c_int;
}

unsafe fn ocRunInit(mut oc: *mut ObjectCode) -> c_int {
    if (*oc).status as c_uint != OBJECT_RESOLVED as c_int as c_uint {
        return 1 as c_int;
    }

    foreignExportsLoadingObject(oc);

    let mut r: c_int = 0;
    r = ocRunInit_MachO(oc);
    foreignExportsFinishedLoadingObject();

    if r == 0 {
        return r;
    }

    (*oc).status = OBJECT_READY;

    return 1 as c_int;
}

unsafe fn runPendingInitializers() -> c_int {
    let mut oc = objects;

    while !oc.is_null() {
        let mut r = ocRunInit(oc);

        if r == 0 {
            errorBelch(
                b"Could not run initializers of Object Code %s.\n\0" as *const u8 as *const c_char,
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            fflush(__stderrp);

            return r;
        }

        oc = (*oc).next as *mut ObjectCode;
    }

    return 1 as c_int;
}

unsafe fn resolveObjs_() -> HsInt {
    let mut oc = objects;

    while !oc.is_null() {
        let mut r = ocTryLoad(oc);

        if r == 0 {
            errorBelch(
                b"Could not load Object Code %s.\n\0" as *const u8 as *const c_char,
                if !(*oc).archiveMemberName.is_null() {
                    (*oc).archiveMemberName
                } else {
                    (*oc).fileName
                },
            );

            fflush(__stderrp);

            return r as HsInt;
        }

        oc = (*oc).next as *mut ObjectCode;
    }

    if runPendingInitializers() == 0 {
        return 0 as HsInt;
    }

    return 1 as HsInt;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    let mut r = resolveObjs_();

    return r;
}

unsafe fn unloadObj_(mut path: *mut pathchar, mut just_purge: bool) -> HsInt {
    let mut unloadedAnyObj = r#false != 0;
    let mut prev = null_mut::<ObjectCode>();
    let mut oc = loaded_objects;

    while !oc.is_null() {
        if strcmp((*oc).fileName, path) == 0 as c_int {
            (*oc).status = OBJECT_UNLOADED;
            removeOcSymbols(oc);
            freeOcStablePtrs(oc);
            unloadedAnyObj = r#true != 0;

            if !just_purge {
                n_unloaded_objects += 1 as c_int;

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
        return 1 as HsInt;
    } else {
        errorBelch(
            b"unloadObj: can't find `%s' to unload\0" as *const u8 as *const c_char,
            path,
        );

        return 0 as HsInt;
    };
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadObj(mut path: *mut pathchar) -> HsInt {
    let mut r = unloadObj_(path, r#false != 0);

    return r;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn purgeObj(mut path: *mut pathchar) -> HsInt {
    let mut r = unloadObj_(path, r#true != 0);

    return r;
}

unsafe fn lookupObjectByPath(mut path: *mut pathchar) -> *mut ObjectCode {
    let mut o = objects;

    while !o.is_null() {
        if 0 as c_int == strcmp((*o).fileName, path) {
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
    let mut r = getObjectLoadStatus_(path);

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
            1 as size_t,
            size_of::<SectionFormatInfo>() as size_t,
            b"addSection(SectionFormatInfo)\0" as *const u8 as *const c_char as *mut c_char,
        ) as *mut SectionFormatInfo;
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadNativeObj(
    mut path: *mut pathchar,
    mut errmsg: *mut *mut c_char,
) -> *mut c_void {
    let mut r = loadNativeObj_POSIX(path, errmsg);

    return r;
}

unsafe fn unloadNativeObj_(mut handle: *mut c_void) -> HsInt {
    let mut unloadedAnyObj = r#false != 0;
    let mut prev = null_mut::<ObjectCode>();
    let mut next = null_mut::<ObjectCode>();
    let mut nc = loaded_objects;

    while !nc.is_null() {
        next = (*nc).next_loaded_object as *mut ObjectCode;

        if (*nc).r#type as c_uint == DYNAMIC_OBJECT as c_int as c_uint
            && (*nc).dlopen_handle == handle
        {
            (*nc).status = OBJECT_UNLOADED;
            n_unloaded_objects += 1 as c_int;

            if (*nc).symbols.is_null() as c_int as c_long != 0 {
            } else {
                _assertFail(
                    b"rts/Linker.c\0" as *const u8 as *const c_char,
                    1908 as c_uint,
                );
            }

            freeOcStablePtrs(nc);

            if prev.is_null() {
                loaded_objects = (*nc).next_loaded_object as *mut ObjectCode;
            } else {
                (*prev).next_loaded_object = (*nc).next_loaded_object;
            }

            unloadedAnyObj = r#true != 0;
        } else {
            prev = nc;
        }

        nc = next;
    }

    if unloadedAnyObj {
        return 1 as HsInt;
    } else {
        errorBelch(
            b"unloadObjNativeObj_: can't find `%p' to unload\0" as *const u8 as *const c_char,
            handle,
        );

        return 0 as HsInt;
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadNativeObj(mut handle: *mut c_void) -> HsInt {
    let mut r = unloadNativeObj_(handle);

    return r;
}

unsafe fn initSegment(
    mut s: *mut Segment,
    mut start: *mut c_void,
    mut size: size_t,
    mut prot: SegmentProt,
    mut n_sections: c_int,
) {
    (*s).start = start;
    (*s).size = size;
    (*s).prot = prot;

    (*s).sections_idx = stgCallocBytes(
        n_sections as size_t,
        size_of::<c_int>() as size_t,
        b"initSegment(segment)\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut c_int;

    (*s).n_sections = n_sections;
}

unsafe fn freeSegments(mut oc: *mut ObjectCode) {
    if !(*oc).segments.is_null() {
        let mut i = 0 as c_int;

        while i < (*oc).n_segments {
            let mut s: *mut Segment = (*oc).segments.offset(i as isize) as *mut Segment;
            stgFree((*s).sections_idx as *mut c_void);
            (*s).sections_idx = null_mut::<c_int>();

            if !(0 as size_t == (*s).size) {
                munmapForLinker(
                    (*s).start,
                    (*s).size,
                    b"freeSegments\0" as *const u8 as *const c_char,
                );

                (*s).start = NULL;
            }

            i += 1;
        }

        stgFree((*oc).segments as *mut c_void);
        (*oc).segments = null_mut::<Segment>();
    }
}
