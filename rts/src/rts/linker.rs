use crate::hs_ffi::{HsBool, HsInt, HsPtr};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {testsuite}
pub type pathchar = c_char;

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initLinker"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn initLinker() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::initLinker()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("initLinker")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_initLinker_"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn initLinker_(retain_cafs: c_int) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::initLinker_(retain_cafs)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("initLinker_")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupSymbol"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn lookupSymbol(lbl: *mut c_char) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::lookupSymbol(lbl)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("lookupSymbol")
}

/// - GHC_PLACES: {testsuite}
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

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getObjectLoadStatus"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getObjectLoadStatus(path: *mut pathchar) -> OStatus {
    #[cfg(feature = "sys")]
    unsafe {
        transmute(sys::getObjectLoadStatus(path))
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getObjectLoadStatus")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_unloadObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn unloadObj(path: *mut pathchar) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::unloadObj(path)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("unloadObj")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_purgeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn purgeObj(path: *mut pathchar) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::purgeObj(path)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("purgeObj")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn loadObj(path: *mut pathchar) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::loadObj(path)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("loadObj")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadArchive"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn loadArchive(path: *mut pathchar) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::loadArchive(path)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("loadArchive")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_resolveObjs"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::resolveObjs()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("resolveObjs")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_loadNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn loadNativeObj(
    path: *mut pathchar,
    errmsg: *mut *mut c_char,
) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::loadNativeObj(path, errmsg)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("loadNativeObj")
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_unloadNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn unloadNativeObj(handle: *mut c_void) -> HsInt {
    #[cfg(feature = "sys")]
    unsafe {
        sys::unloadNativeObj(handle)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("unloadNativeObj")
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_lookupSymbolInNativeObj"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    handle: *mut c_void,
    symbol_name: *const c_char,
) -> *mut c_void {
    #[cfg(feature = "sys")]
    unsafe {
        sys::lookupSymbolInNativeObj(handle, symbol_name)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("lookupSymbolInNativeObj")
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_addDLL"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn addDLL(dll_name: *mut pathchar) -> *const c_char {
    #[cfg(feature = "sys")]
    unsafe {
        sys::addDLL(dll_name)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("addDLL")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_addLibrarySearchPath"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn addLibrarySearchPath(dll_path: *mut pathchar) -> HsPtr {
    #[cfg(feature = "sys")]
    unsafe {
        sys::addLibrarySearchPath(dll_path)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("addLibrarySearchPath")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_removeLibrarySearchPath"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn removeLibrarySearchPath(dll_path_index: HsPtr) -> HsBool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::removeLibrarySearchPath(dll_path_index)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("removeLibrarySearchPath")
}

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_findSystemLibrary"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn findSystemLibrary(dll_name: *mut pathchar) -> *mut pathchar {
    #[cfg(feature = "sys")]
    unsafe {
        sys::findSystemLibrary(dll_name)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("findSystemLibrary")
}
