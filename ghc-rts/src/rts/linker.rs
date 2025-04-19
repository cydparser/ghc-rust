use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub const PATH_FMT: &[u8; 2] = b"s\0";

pub type pathchar = ::core::ffi::c_char;

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initLinker() {
    unsafe { sys::initLinker() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initLinker_(retain_cafs: ::core::ffi::c_int) {
    unsafe { sys::initLinker_(retain_cafs) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn insertSymbol(
    obj_name: *mut pathchar,
    key: *mut ::core::ffi::c_char,
    data: *mut ::core::ffi::c_void,
) -> HsInt {
    unsafe { transmute(sys::insertSymbol(obj_name, key, data)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbol(lbl: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::lookupSymbol(lbl)) }
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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn getObjectLoadStatus(path: *mut pathchar) -> OStatus {
    unsafe { transmute(sys::getObjectLoadStatus(path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::unloadObj(path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn purgeObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::purgeObj(path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::loadObj(path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadArchive(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::loadArchive(path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    unsafe { transmute(sys::resolveObjs()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadNativeObj(
    path: *mut pathchar,
    errmsg: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::loadNativeObj(path, errmsg)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadNativeObj(handle: *mut ::core::ffi::c_void) -> HsInt {
    unsafe { transmute(sys::unloadNativeObj(handle)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    handle: *mut ::core::ffi::c_void,
    symbol_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::lookupSymbolInNativeObj(handle, symbol_name)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addDLL(dll_name: *mut pathchar) -> *const ::core::ffi::c_char {
    unsafe { transmute(sys::addDLL(dll_name)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addLibrarySearchPath(dll_path: *mut pathchar) -> HsPtr {
    unsafe { transmute(sys::addLibrarySearchPath(dll_path)) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn removeLibrarySearchPath(dll_path_index: HsPtr) -> HsBool {
    unsafe { transmute(sys::removeLibrarySearchPath(dll_path_index)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn warnMissingKBLibraryPaths() {
    unsafe { sys::warnMissingKBLibraryPaths() }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn findSystemLibrary(dll_name: *mut pathchar) -> *mut pathchar {
    unsafe { transmute(sys::findSystemLibrary(dll_name)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn foreignExportStablePtr(p: StgPtr) -> StgStablePtr {
    unsafe { transmute(sys::foreignExportStablePtr(p)) }
}
