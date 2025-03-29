use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_PATH_FMT() {
    assert_eq!(sys::PATH_FMT, super::PATH_FMT);
}

#[test]
#[ignore]
fn test_initLinker() {
    unsafe { super::initLinker() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initLinker_() {
    let retain_cafs = Default::default();
    unsafe { super::initLinker_(retain_cafs) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_insertSymbol(
    obj_name: pathchar,
    key: ::core::ffi::c_char,
    data: ::core::ffi::c_void,
) -> bool {
    let expected = unsafe {
        transmute(sys::insertSymbol(
            &mut obj_name.into(),
            &mut key.into(),
            &mut data.into(),
        ))
    };
    let actual = unsafe { super::insertSymbol(&mut obj_name, &mut key, &mut data) };
    actual == expected
}

#[test]
#[ignore]
fn test_insertSymbol() {
    let mut obj_name = Default::default();
    let mut key = Default::default();
    let mut data = Default::default();
    unsafe { super::insertSymbol(&mut obj_name, &mut key, &mut data) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupSymbol(lbl: ::core::ffi::c_char) -> bool {
    let expected = unsafe { transmute(sys::lookupSymbol(&mut lbl.into())) };
    let actual = unsafe { super::lookupSymbol(&mut lbl) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupSymbol() {
    let mut lbl = Default::default();
    unsafe { super::lookupSymbol(&mut lbl) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getObjectLoadStatus(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::getObjectLoadStatus(&mut path.into())) };
    let actual = unsafe { super::getObjectLoadStatus(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_getObjectLoadStatus() {
    let mut path = Default::default();
    unsafe { super::getObjectLoadStatus(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unloadObj(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::unloadObj(&mut path.into())) };
    let actual = unsafe { super::unloadObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_unloadObj() {
    let mut path = Default::default();
    unsafe { super::unloadObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_purgeObj(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::purgeObj(&mut path.into())) };
    let actual = unsafe { super::purgeObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_purgeObj() {
    let mut path = Default::default();
    unsafe { super::purgeObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadObj(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::loadObj(&mut path.into())) };
    let actual = unsafe { super::loadObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadObj() {
    let mut path = Default::default();
    unsafe { super::loadObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadArchive(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::loadArchive(&mut path.into())) };
    let actual = unsafe { super::loadArchive(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadArchive() {
    let mut path = Default::default();
    unsafe { super::loadArchive(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_resolveObjs() -> bool {
    let expected = unsafe { transmute(sys::resolveObjs()) };
    let actual = unsafe { super::resolveObjs() };
    actual == expected
}

#[test]
#[ignore]
fn test_resolveObjs() {
    unsafe { super::resolveObjs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadNativeObj(path: pathchar, errmsg: ::core::ffi::c_char) -> bool {
    let expected = unsafe {
        transmute(sys::loadNativeObj(
            &mut path.into(),
            &mut &mut errmsg.into(),
        ))
    };
    let actual = unsafe { super::loadNativeObj(&mut path, &mut &mut errmsg) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadNativeObj() {
    let mut path = Default::default();
    let mut errmsg = Default::default();
    unsafe { super::loadNativeObj(&mut path, &mut &mut errmsg) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unloadNativeObj(handle: ::core::ffi::c_void) -> bool {
    let expected = unsafe { transmute(sys::unloadNativeObj(&mut handle.into())) };
    let actual = unsafe { super::unloadNativeObj(&mut handle) };
    actual == expected
}

#[test]
#[ignore]
fn test_unloadNativeObj() {
    let mut handle = Default::default();
    unsafe { super::unloadNativeObj(&mut handle) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupSymbolInNativeObj(
    handle: ::core::ffi::c_void,
    symbol_name: ::core::ffi::c_char,
) -> bool {
    let expected = unsafe {
        transmute(sys::lookupSymbolInNativeObj(
            &mut handle.into(),
            &symbol_name.into(),
        ))
    };
    let actual = unsafe { super::lookupSymbolInNativeObj(&mut handle, &symbol_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupSymbolInNativeObj() {
    let mut handle = Default::default();
    let symbol_name = Default::default();
    unsafe { super::lookupSymbolInNativeObj(&mut handle, &symbol_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_addDLL(dll_name: pathchar) -> bool {
    let expected = unsafe { transmute(sys::addDLL(&mut dll_name.into())) };
    let actual = unsafe { super::addDLL(&mut dll_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_addDLL() {
    let mut dll_name = Default::default();
    unsafe { super::addDLL(&mut dll_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_addLibrarySearchPath(dll_path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::addLibrarySearchPath(&mut dll_path.into())) };
    let actual = unsafe { super::addLibrarySearchPath(&mut dll_path) };
    actual == expected
}

#[test]
#[ignore]
fn test_addLibrarySearchPath() {
    let mut dll_path = Default::default();
    unsafe { super::addLibrarySearchPath(&mut dll_path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_removeLibrarySearchPath(dll_path_index: HsPtr) -> bool {
    let expected = unsafe { transmute(sys::removeLibrarySearchPath(dll_path_index.into())) };
    let actual = unsafe { super::removeLibrarySearchPath(dll_path_index) };
    actual == expected
}

#[test]
#[ignore]
fn test_removeLibrarySearchPath() {
    let dll_path_index = Default::default();
    unsafe { super::removeLibrarySearchPath(dll_path_index) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_warnMissingKBLibraryPaths() {
    unsafe { super::warnMissingKBLibraryPaths() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_findSystemLibrary(dll_name: pathchar) -> bool {
    let expected = unsafe { transmute(sys::findSystemLibrary(&mut dll_name.into())) };
    let actual = unsafe { super::findSystemLibrary(&mut dll_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_findSystemLibrary() {
    let mut dll_name = Default::default();
    unsafe { super::findSystemLibrary(&mut dll_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_foreignExportStablePtr(p: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::foreignExportStablePtr(p.into())) };
    let actual = unsafe { super::foreignExportStablePtr(p) };
    actual == expected
}

#[test]
#[ignore]
fn test_foreignExportStablePtr() {
    let p = Default::default();
    unsafe { super::foreignExportStablePtr(p) };
    todo!("assert")
}
