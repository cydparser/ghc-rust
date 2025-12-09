use crate::ffi::hs_ffi::{HsBool, HsInt, HsPtr};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
pub type pathchar = c_char;

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker() {
    sys! {
        initLinker()
    }
}

#[ffi(libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn initLinker_(retain_cafs: c_int) {
    sys! {
        initLinker_(retain_cafs)
    }
}

#[ffi(compiler, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupSymbol(lbl: *mut c_char) -> *mut c_void {
    sys! {
        lookupSymbol(lbl)
    }
}

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

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getObjectLoadStatus(path: *mut pathchar) -> OStatus {
    sys! {
        transmute(getObjectLoadStatus(path))
    }
}

#[ffi(compiler, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadObj(path: *mut pathchar) -> HsInt {
    sys! {
        unloadObj(path)
    }
}

#[ffi(libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn purgeObj(path: *mut pathchar) -> HsInt {
    sys! {
        purgeObj(path)
    }
}

#[ffi(compiler, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadObj(path: *mut pathchar) -> HsInt {
    sys! {
        loadObj(path)
    }
}

#[ffi(compiler, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadArchive(path: *mut pathchar) -> HsInt {
    sys! {
        loadArchive(path)
    }
}

#[ffi(compiler, libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn resolveObjs() -> HsInt {
    sys! {
        resolveObjs()
    }
}

#[ffi(libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn loadNativeObj(
    path: *mut pathchar,
    errmsg: *mut *mut c_char,
) -> *mut c_void {
    sys! {
        loadNativeObj(path, errmsg)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unloadNativeObj(handle: *mut c_void) -> HsInt {
    sys! {
        unloadNativeObj(handle)
    }
}

#[ffi(libraries, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lookupSymbolInNativeObj(
    handle: *mut c_void,
    symbol_name: *const c_char,
) -> *mut c_void {
    sys! {
        lookupSymbolInNativeObj(handle, symbol_name)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn addDLL(dll_name: *mut pathchar) -> *const c_char {
    sys! {
        addDLL(dll_name)
    }
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn addLibrarySearchPath(dll_path: *mut pathchar) -> HsPtr {
    sys! {
        addLibrarySearchPath(dll_path)
    }
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn removeLibrarySearchPath(dll_path_index: HsPtr) -> HsBool {
    sys! {
        removeLibrarySearchPath(dll_path_index)
    }
}

#[ffi(compiler, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn findSystemLibrary(dll_name: *mut pathchar) -> *mut pathchar {
    sys! {
        findSystemLibrary(dll_name)
    }
}
