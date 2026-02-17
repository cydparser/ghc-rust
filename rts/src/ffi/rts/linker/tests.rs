use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_pathchar_layout() {
    assert_eq!(size_of::<pathchar>(), size_of::<pathchar>());
    assert_eq!(align_of::<pathchar>(), align_of::<pathchar>());
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_initLinker() {
    let expected = {
        unsafe { sys::initLinker() };
        todo!()
    };

    let actual = {
        unsafe { initLinker() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_initLinker() {
    let actual = {
        unsafe { initLinker() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_initLinker_(retain_cafs: c_int) -> bool {
    let expected = {
        unsafe { sys::initLinker_(retain_cafs) };
        todo!()
    };

    let actual = {
        unsafe { initLinker_(retain_cafs) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_initLinker_() {
    let g = &mut Gen::new(100);
    let actual = {
        let retain_cafs: c_int = Arbitrary::arbitrary(g);
        unsafe { initLinker_(retain_cafs) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_lookupSymbol(lbl: c_char) -> bool {
    let expected = {
        let mut lbl = lbl;
        let result: &c_void = unsafe { &*sys::lookupSymbol(&raw mut lbl) };
        todo!()
    };

    let actual = {
        let mut lbl = lbl;
        let result: &c_void = unsafe { &*lookupSymbol(&raw mut lbl) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_lookupSymbol() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut lbl: c_char = Arbitrary::arbitrary(g);
        let result: &c_void = unsafe { &*lookupSymbol(&raw mut lbl) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
fn sys_OStatus_layout() {
    assert_eq!(size_of::<OStatus>(), size_of::<sys::OStatus>());
    assert_eq!(align_of::<OStatus>(), align_of::<sys::OStatus>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_OStatus_discriminants() {
    assert_eq!(
        OStatus::OBJECT_LOADED as isize,
        sys::OStatus::OBJECT_LOADED as isize
    );
    assert_eq!(
        OStatus::OBJECT_NEEDED as isize,
        sys::OStatus::OBJECT_NEEDED as isize
    );
    assert_eq!(
        OStatus::OBJECT_RESOLVED as isize,
        sys::OStatus::OBJECT_RESOLVED as isize
    );
    assert_eq!(
        OStatus::OBJECT_READY as isize,
        sys::OStatus::OBJECT_READY as isize
    );
    assert_eq!(
        OStatus::OBJECT_UNLOADED as isize,
        sys::OStatus::OBJECT_UNLOADED as isize
    );
    assert_eq!(
        OStatus::OBJECT_DONT_RESOLVE as isize,
        sys::OStatus::OBJECT_DONT_RESOLVE as isize
    );
    assert_eq!(
        OStatus::OBJECT_NOT_LOADED as isize,
        sys::OStatus::OBJECT_NOT_LOADED as isize
    )
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_getObjectLoadStatus(path: pathchar) -> bool {
    let expected: OStatus = {
        let mut path = path;
        unsafe { transmute(sys::getObjectLoadStatus(&raw mut path)) }
    };

    let actual: OStatus = {
        let mut path = path;
        unsafe { getObjectLoadStatus(&raw mut path) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_getObjectLoadStatus() {
    let g = &mut Gen::new(100);
    let actual: OStatus = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { getObjectLoadStatus(&raw mut path) }
    };

    let expected: OStatus = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_unloadObj(path: pathchar) -> bool {
    let expected: HsInt = {
        let mut path = path;
        unsafe { sys::unloadObj(&raw mut path) }
    };

    let actual: HsInt = {
        let mut path = path;
        unsafe { unloadObj(&raw mut path) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unloadObj() {
    let g = &mut Gen::new(100);
    let actual: HsInt = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { unloadObj(&raw mut path) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_purgeObj(path: pathchar) -> bool {
    let expected: HsInt = {
        let mut path = path;
        unsafe { sys::purgeObj(&raw mut path) }
    };

    let actual: HsInt = {
        let mut path = path;
        unsafe { purgeObj(&raw mut path) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_purgeObj() {
    let g = &mut Gen::new(100);
    let actual: HsInt = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { purgeObj(&raw mut path) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_loadObj(path: pathchar) -> bool {
    let expected: HsInt = {
        let mut path = path;
        unsafe { sys::loadObj(&raw mut path) }
    };

    let actual: HsInt = {
        let mut path = path;
        unsafe { loadObj(&raw mut path) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_loadObj() {
    let g = &mut Gen::new(100);
    let actual: HsInt = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { loadObj(&raw mut path) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_loadArchive(path: pathchar) -> bool {
    let expected: HsInt = {
        let mut path = path;
        unsafe { sys::loadArchive(&raw mut path) }
    };

    let actual: HsInt = {
        let mut path = path;
        unsafe { loadArchive(&raw mut path) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_loadArchive() {
    let g = &mut Gen::new(100);
    let actual: HsInt = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        unsafe { loadArchive(&raw mut path) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_resolveObjs() {
    let expected: HsInt = { unsafe { sys::resolveObjs() } };
    let actual: HsInt = { unsafe { resolveObjs() } };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_resolveObjs() {
    let actual: HsInt = { unsafe { resolveObjs() } };
    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_loadNativeObj(path: pathchar, errmsg: c_char) -> bool {
    let expected = {
        let mut path = path;
        let mut errmsg = errmsg;
        let mut errmsg = &raw mut errmsg;
        let result: &c_void = unsafe { &*sys::loadNativeObj(&raw mut path, &raw mut errmsg) };
        todo!()
    };

    let actual = {
        let mut path = path;
        let mut errmsg = errmsg;
        let mut errmsg = &raw mut errmsg;
        let result: &c_void = unsafe { &*loadNativeObj(&raw mut path, &raw mut errmsg) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_loadNativeObj() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut path: pathchar = Arbitrary::arbitrary(g);
        let mut errmsg: c_char = Arbitrary::arbitrary(g);
        let mut errmsg = &raw mut errmsg;
        let result: &c_void = unsafe { &*loadNativeObj(&raw mut path, &raw mut errmsg) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_unloadNativeObj() {
    let expected: HsInt = {
        let mut handle: c_void = todo!();
        unsafe { sys::unloadNativeObj(&raw mut handle) }
    };

    let actual: HsInt = {
        let mut handle: c_void = todo!();
        unsafe { unloadNativeObj(&raw mut handle) }
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_unloadNativeObj() {
    let actual: HsInt = {
        let handle: c_void = todo!();
        unsafe { unloadNativeObj(&raw mut handle) }
    };

    let expected: HsInt = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_lookupSymbolInNativeObj(symbol_name: c_char) -> bool {
    let expected = {
        let mut handle: c_void = todo!();
        let mut symbol_name = symbol_name;
        let result: &c_void =
            unsafe { &*sys::lookupSymbolInNativeObj(&raw mut handle, &raw mut symbol_name) };
        todo!()
    };

    let actual = {
        let mut handle: c_void = todo!();
        let mut symbol_name = symbol_name;
        let result: &c_void =
            unsafe { &*lookupSymbolInNativeObj(&raw mut handle, &raw mut symbol_name) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_lookupSymbolInNativeObj() {
    let g = &mut Gen::new(100);
    let actual = {
        let handle: c_void = todo!();
        let mut symbol_name: c_char = Arbitrary::arbitrary(g);
        let result: &c_void =
            unsafe { &*lookupSymbolInNativeObj(&raw mut handle, &raw mut symbol_name) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_addDLL(dll_name: pathchar) -> bool {
    let expected: &c_char = {
        let mut dll_name = dll_name;
        unsafe { &*sys::addDLL(&raw mut dll_name) }
    };

    let actual: &c_char = {
        let mut dll_name = dll_name;
        unsafe { &*addDLL(&raw mut dll_name) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_addDLL() {
    let g = &mut Gen::new(100);
    let actual: &c_char = {
        let mut dll_name: pathchar = Arbitrary::arbitrary(g);
        unsafe { &*addDLL(&raw mut dll_name) }
    };

    let expected: &c_char = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_addLibrarySearchPath(dll_path: pathchar) -> bool {
    let expected = {
        let mut dll_path = dll_path;
        let result: HsPtr = unsafe { sys::addLibrarySearchPath(&raw mut dll_path) };
        todo!()
    };

    let actual = {
        let mut dll_path = dll_path;
        let result: HsPtr = unsafe { addLibrarySearchPath(&raw mut dll_path) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_addLibrarySearchPath() {
    let g = &mut Gen::new(100);
    let actual = {
        let mut dll_path: pathchar = Arbitrary::arbitrary(g);
        let result: HsPtr = unsafe { addLibrarySearchPath(&raw mut dll_path) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_removeLibrarySearchPath() {
    let expected: HsBool = {
        let dll_path_index: HsPtr = todo!();
        unsafe { sys::removeLibrarySearchPath(dll_path_index) }
    };

    let actual: HsBool = {
        let dll_path_index: HsPtr = todo!();
        unsafe { removeLibrarySearchPath(dll_path_index) }
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_removeLibrarySearchPath() {
    let actual: HsBool = {
        let dll_path_index: HsPtr = todo!();
        unsafe { removeLibrarySearchPath(dll_path_index) }
    };

    let expected: HsBool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_findSystemLibrary(dll_name: pathchar) -> bool {
    let expected: &pathchar = {
        let mut dll_name = dll_name;
        unsafe { &*sys::findSystemLibrary(&raw mut dll_name) }
    };

    let actual: &pathchar = {
        let mut dll_name = dll_name;
        unsafe { &*findSystemLibrary(&raw mut dll_name) }
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_findSystemLibrary() {
    let g = &mut Gen::new(100);
    let actual: &pathchar = {
        let mut dll_name: pathchar = Arbitrary::arbitrary(g);
        unsafe { &*findSystemLibrary(&raw mut dll_name) }
    };

    let expected: &pathchar = todo!();
    assert_eq!(expected, actual);
}
