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
    unsafe { transmute(sys::initLinker()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn initLinker_(retain_cafs: ::core::ffi::c_int) {
    unsafe { transmute(sys::initLinker_(retain_cafs.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn insertSymbol(
    obj_name: *mut pathchar,
    key: *mut ::core::ffi::c_char,
    data: *mut ::core::ffi::c_void,
) -> HsInt {
    unsafe {
        transmute(sys::insertSymbol(
            &mut obj_name.into(),
            &mut key.into(),
            &mut data.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbol(lbl: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_void {
    unsafe { transmute(sys::lookupSymbol(&mut lbl.into())) }
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
    unsafe { transmute(sys::getObjectLoadStatus(&mut path.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::unloadObj(&mut path.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn purgeObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::purgeObj(&mut path.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadObj(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::loadObj(&mut path.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn loadArchive(path: *mut pathchar) -> HsInt {
    unsafe { transmute(sys::loadArchive(&mut path.into())) }
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
    unsafe {
        transmute(sys::loadNativeObj(
            &mut path.into(),
            &mut &mut errmsg.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn unloadNativeObj(handle: *mut ::core::ffi::c_void) -> HsInt {
    unsafe { transmute(sys::unloadNativeObj(&mut handle.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    handle: *mut ::core::ffi::c_void,
    symbol_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        transmute(sys::lookupSymbolInNativeObj(
            &mut handle.into(),
            &symbol_name.into(),
        ))
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addDLL(dll_name: *mut pathchar) -> *const ::core::ffi::c_char {
    unsafe { transmute(sys::addDLL(&mut dll_name.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn addLibrarySearchPath(dll_path: *mut pathchar) -> HsPtr {
    unsafe { transmute(sys::addLibrarySearchPath(&mut dll_path.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn removeLibrarySearchPath(dll_path_index: HsPtr) -> HsBool {
    unsafe { transmute(sys::removeLibrarySearchPath(dll_path_index.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn warnMissingKBLibraryPaths() {
    unsafe { transmute(sys::warnMissingKBLibraryPaths()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn findSystemLibrary(dll_name: *mut pathchar) -> *mut pathchar {
    unsafe { transmute(sys::findSystemLibrary(&mut dll_name.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn foreignExportStablePtr(p: StgPtr) -> StgStablePtr {
    unsafe { transmute(sys::foreignExportStablePtr(p.into())) }
}
