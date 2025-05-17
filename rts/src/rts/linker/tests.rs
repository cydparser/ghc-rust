use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[test]
fn sys_eq_PATH_FMT() {
    assert_eq!(sys::PATH_FMT, PATH_FMT);
}

#[test]
#[ignore]
fn test_initLinker() {
    unsafe { initLinker() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_initLinker_() {
    let retain_cafs = Default::default();
    unsafe { initLinker_(retain_cafs) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_insertSymbol(obj_name: pathchar, key: c_char, data: c_void) -> bool {
    let expected = unsafe { sys::insertSymbol(&mut obj_name, &mut key, &mut data) };
    let actual = unsafe { insertSymbol(&mut obj_name, &mut key, &mut data) };
    actual == expected
}

#[test]
#[ignore]
fn test_insertSymbol() {
    let mut obj_name = null_mut();
    let mut key = null_mut();
    let mut data = null_mut();
    unsafe { insertSymbol(&mut obj_name, &mut key, &mut data) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupSymbol(lbl: c_char) -> bool {
    let expected = unsafe { sys::lookupSymbol(&mut lbl) };
    let actual = unsafe { lookupSymbol(&mut lbl) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupSymbol() {
    let mut lbl = null_mut();
    unsafe { lookupSymbol(&mut lbl) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getObjectLoadStatus(path: pathchar) -> bool {
    let expected = unsafe { transmute(sys::getObjectLoadStatus(&mut path)) };
    let actual = unsafe { getObjectLoadStatus(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_getObjectLoadStatus() {
    let mut path = null_mut();
    unsafe { getObjectLoadStatus(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unloadObj(path: pathchar) -> bool {
    let expected = unsafe { sys::unloadObj(&mut path) };
    let actual = unsafe { unloadObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_unloadObj() {
    let mut path = null_mut();
    unsafe { unloadObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_purgeObj(path: pathchar) -> bool {
    let expected = unsafe { sys::purgeObj(&mut path) };
    let actual = unsafe { purgeObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_purgeObj() {
    let mut path = null_mut();
    unsafe { purgeObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadObj(path: pathchar) -> bool {
    let expected = unsafe { sys::loadObj(&mut path) };
    let actual = unsafe { loadObj(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadObj() {
    let mut path = null_mut();
    unsafe { loadObj(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadArchive(path: pathchar) -> bool {
    let expected = unsafe { sys::loadArchive(&mut path) };
    let actual = unsafe { loadArchive(&mut path) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadArchive() {
    let mut path = null_mut();
    unsafe { loadArchive(&mut path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_resolveObjs() -> bool {
    let expected = unsafe { sys::resolveObjs() };
    let actual = unsafe { resolveObjs() };
    actual == expected
}

#[test]
#[ignore]
fn test_resolveObjs() {
    unsafe { resolveObjs() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_loadNativeObj(path: pathchar, errmsg: c_char) -> bool {
    let expected = unsafe { sys::loadNativeObj(&mut path, &mut &mut errmsg) };
    let actual = unsafe { loadNativeObj(&mut path, &mut &mut errmsg) };
    actual == expected
}

#[test]
#[ignore]
fn test_loadNativeObj() {
    let mut path = null_mut();
    let mut errmsg = null_mut();
    unsafe { loadNativeObj(&mut path, &mut &mut errmsg) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_unloadNativeObj(handle: c_void) -> bool {
    let expected = unsafe { sys::unloadNativeObj(&mut handle) };
    let actual = unsafe { unloadNativeObj(&mut handle) };
    actual == expected
}

#[test]
#[ignore]
fn test_unloadNativeObj() {
    let mut handle = null_mut();
    unsafe { unloadNativeObj(&mut handle) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_lookupSymbolInNativeObj(handle: c_void, symbol_name: c_char) -> bool {
    let expected = unsafe { sys::lookupSymbolInNativeObj(&mut handle, &symbol_name) };
    let actual = unsafe { lookupSymbolInNativeObj(&mut handle, &symbol_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_lookupSymbolInNativeObj() {
    let mut handle = null_mut();
    let symbol_name = null();
    unsafe { lookupSymbolInNativeObj(&mut handle, &symbol_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_addDLL(dll_name: pathchar) -> bool {
    let expected = unsafe { sys::addDLL(&mut dll_name) };
    let actual = unsafe { addDLL(&mut dll_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_addDLL() {
    let mut dll_name = null_mut();
    unsafe { addDLL(&mut dll_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_addLibrarySearchPath(dll_path: pathchar) -> bool {
    let expected = unsafe { sys::addLibrarySearchPath(&mut dll_path) };
    let actual = unsafe { addLibrarySearchPath(&mut dll_path) };
    actual == expected
}

#[test]
#[ignore]
fn test_addLibrarySearchPath() {
    let mut dll_path = null_mut();
    unsafe { addLibrarySearchPath(&mut dll_path) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_removeLibrarySearchPath(dll_path_index: HsPtr) -> bool {
    let expected = unsafe { sys::removeLibrarySearchPath(dll_path_index) };
    let actual = unsafe { removeLibrarySearchPath(dll_path_index) };
    actual == expected
}

#[test]
#[ignore]
fn test_removeLibrarySearchPath() {
    let dll_path_index = Default::default();
    unsafe { removeLibrarySearchPath(dll_path_index) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_warnMissingKBLibraryPaths() {
    unsafe { warnMissingKBLibraryPaths() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_findSystemLibrary(dll_name: pathchar) -> bool {
    let expected = unsafe { sys::findSystemLibrary(&mut dll_name) };
    let actual = unsafe { findSystemLibrary(&mut dll_name) };
    actual == expected
}

#[test]
#[ignore]
fn test_findSystemLibrary() {
    let mut dll_name = null_mut();
    unsafe { findSystemLibrary(&mut dll_name) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_foreignExportStablePtr(p: StgPtr) -> bool {
    let expected = unsafe { sys::foreignExportStablePtr(p) };
    let actual = unsafe { foreignExportStablePtr(p) };
    actual == expected
}

#[test]
#[ignore]
fn test_foreignExportStablePtr() {
    let p = Default::default();
    unsafe { foreignExportStablePtr(p) };
    todo!("assert")
}
