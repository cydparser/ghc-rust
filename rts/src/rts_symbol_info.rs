use crate::hash::{allocStrHashTable, insertStrHashTable, lookupStrHashTable};
use crate::linker_internals::ObjectCode;
use crate::prelude::*;
use crate::rts_symbol_info::{
    _SymbolInfo, _SymbolKind, KIND_IMPORT, KIND_WEAK, SymbolInfo, SymbolKind, symbolUpdater,
};
use crate::rts_utils::stgMallocBytes;

pub(crate) type _SymbolKind = c_uint;

pub(crate) const KIND_IMPORT: _SymbolKind = 4;

pub(crate) const KIND_WEAK: _SymbolKind = 2;

pub(crate) const KIND_NORMAL: _SymbolKind = 1;

pub(crate) type SymbolKind = _SymbolKind;

/// cbindgen:no-export
pub(crate) struct _SymbolInfo {
    pub(crate) kind: SymbolKind,
}

pub(crate) type SymbolInfo = _SymbolInfo;

pub(crate) type symbolUpdater = Option<unsafe extern "C" fn(*mut SymbolInfo) -> ()>;

unsafe fn setSymbolInfo(
    mut owner: *mut ObjectCode,
    mut label: *const c_void,
    mut updater: symbolUpdater,
) {
    let mut info = null_mut::<SymbolInfo>();

    if !owner.is_null() && !label.is_null() {
        info = null_mut::<SymbolInfo>();

        if (*owner).extraInfos.is_null() {
            (*owner).extraInfos = allocStrHashTable();
        } else {
            info =
                lookupStrHashTable((*owner).extraInfos, label as *const c_char) as *mut SymbolInfo;
        }

        if info.is_null() {
            info = stgMallocBytes(
                size_of::<SymbolInfo>() as size_t,
                b"setSymbolInfo\0" as *const u8 as *const c_char as *mut c_char,
            ) as *mut SymbolInfo;

            (*info).kind = 0 as SymbolKind;
        }

        updater.expect("non-null function pointer")(info);

        insertStrHashTable(
            (*owner).extraInfos,
            label as *const c_char,
            info as *const c_void,
        );
    }
}

unsafe fn isSymbolWeak(mut owner: *mut ObjectCode, mut label: *const c_void) -> bool {
    let mut info = null_mut::<SymbolInfo>();

    return !owner.is_null()
        && !label.is_null()
        && !(*owner).extraInfos.is_null()
        && {
            info =
                lookupStrHashTable((*owner).extraInfos, label as *const c_char) as *mut SymbolInfo;
            !info.is_null()
        }
        && (*info).kind as c_uint & KIND_WEAK as c_int as c_uint == KIND_WEAK as c_int as c_uint;
}

unsafe fn isSymbolImport(mut owner: *mut ObjectCode, mut label: *const c_void) -> bool {
    let mut info = null_mut::<SymbolInfo>();

    return !owner.is_null()
        && !label.is_null()
        && !(*owner).extraInfos.is_null()
        && {
            info =
                lookupStrHashTable((*owner).extraInfos, label as *const c_char) as *mut SymbolInfo;
            !info.is_null()
        }
        && (*info).kind as c_uint & KIND_IMPORT as c_int as c_uint
            == KIND_IMPORT as c_int as c_uint;
}

unsafe fn markWeak(mut info: *mut SymbolInfo) {
    if !info.is_null() {
        (*info).kind =
            transmute::<c_uint, SymbolKind>((*info).kind as c_uint | KIND_WEAK as c_int as c_uint);
    }
}

unsafe fn markImport(mut info: *mut SymbolInfo) {
    if !info.is_null() {
        (*info).kind = transmute::<c_uint, SymbolKind>(
            (*info).kind as c_uint | KIND_IMPORT as c_int as c_uint,
        );
    }
}

unsafe fn unmarkImport(mut info: *mut SymbolInfo) {
    if !info.is_null() {
        (*info).kind = transmute::<c_uint, SymbolKind>(
            (*info).kind as c_uint & !(KIND_IMPORT as c_int) as c_uint,
        );
    }
}

unsafe fn setWeakSymbol(mut owner: *mut ObjectCode, mut label: *const c_void) {
    setSymbolInfo(
        owner,
        label,
        Some(markWeak as unsafe extern "C" fn(*mut SymbolInfo) -> ()),
    );
}

unsafe fn setImportSymbol(mut owner: *mut ObjectCode, mut label: *const c_void) {
    setSymbolInfo(
        owner,
        label,
        Some(markImport as unsafe extern "C" fn(*mut SymbolInfo) -> ()),
    );
}

unsafe fn clearImportSymbol(mut owner: *mut ObjectCode, mut label: *const c_void) {
    setSymbolInfo(
        owner,
        label,
        Some(unmarkImport as unsafe extern "C" fn(*mut SymbolInfo) -> ()),
    );
}
