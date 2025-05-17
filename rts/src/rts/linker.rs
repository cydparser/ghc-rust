use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub const PATH_FMT: &[u8; 2] = b"s\0";

pub type pathchar = c_char;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initLinker"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initLinker() {
    unsafe { sys::initLinker() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initLinker_"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initLinker_(retain_cafs: c_int) {
    unsafe { sys::initLinker_(retain_cafs) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn insertSymbol(
    obj_name: *mut pathchar,
    key: *mut c_char,
    data: *mut c_void,
) -> HsInt {
    unsafe { sys::insertSymbol(obj_name, key, data) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupSymbol"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbol(lbl: *mut c_char) -> *mut c_void {
    unsafe { sys::lookupSymbol(lbl) }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum OStatus {
    OBJECT_LOADED = 0,
    OBJECT_NEEDED = 1,
    OBJECT_RESOLVED = 2,
    OBJECT_READY = 3,
    OBJECT_UNLOADED = 4,
    OBJECT_DONT_RESOLVE = 5,
    OBJECT_NOT_LOADED = 6,
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getObjectLoadStatus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getObjectLoadStatus(path: *mut pathchar) -> OStatus {
    unsafe { transmute(sys::getObjectLoadStatus(path)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_unloadObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadObj(path: *mut pathchar) -> HsInt {
    unsafe { sys::unloadObj(path) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_purgeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn purgeObj(path: *mut pathchar) -> HsInt {
    unsafe { sys::purgeObj(path) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadObj(path: *mut pathchar) -> HsInt {
    unsafe { sys::loadObj(path) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadArchive"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadArchive(path: *mut pathchar) -> HsInt {
    unsafe { sys::loadArchive(path) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_resolveObjs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    unsafe { sys::resolveObjs() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadNativeObj(
    path: *mut pathchar,
    errmsg: *mut *mut c_char,
) -> *mut c_void {
    unsafe { sys::loadNativeObj(path, errmsg) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_unloadNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadNativeObj(handle: *mut c_void) -> HsInt {
    unsafe { sys::unloadNativeObj(handle) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupSymbolInNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    handle: *mut c_void,
    symbol_name: *const c_char,
) -> *mut c_void {
    unsafe { sys::lookupSymbolInNativeObj(handle, symbol_name) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_addDLL"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addDLL(dll_name: *mut pathchar) -> *const c_char {
    unsafe { sys::addDLL(dll_name) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_addLibrarySearchPath"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addLibrarySearchPath(dll_path: *mut pathchar) -> HsPtr {
    unsafe { sys::addLibrarySearchPath(dll_path) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_removeLibrarySearchPath"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn removeLibrarySearchPath(dll_path_index: HsPtr) -> HsBool {
    unsafe { sys::removeLibrarySearchPath(dll_path_index) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn warnMissingKBLibraryPaths() {
    unsafe { sys::warnMissingKBLibraryPaths() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_findSystemLibrary"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn findSystemLibrary(dll_name: *mut pathchar) -> *mut pathchar {
    unsafe { sys::findSystemLibrary(dll_name) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn foreignExportStablePtr(p: StgPtr) -> StgStablePtr {
    unsafe { sys::foreignExportStablePtr(p) }
}
