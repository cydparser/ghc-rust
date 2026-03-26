use super::*;

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_keepCAFs_layout() {
    assert_eq!(
        size_of_val(unsafe { &keepCAFs }),
        size_of_val(unsafe { &sys::keepCAFs })
    );
    assert_eq!(
        align_of_val(unsafe { &keepCAFs }),
        align_of_val(unsafe { &sys::keepCAFs })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_generations_layout() {
    assert_eq!(
        size_of_val(unsafe { &generations }),
        size_of_val(unsafe { &sys::generations })
    );
    assert_eq!(
        align_of_val(unsafe { &generations }),
        align_of_val(unsafe { &sys::generations })
    );
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_g0_layout() {
    assert_eq!(
        size_of_val(unsafe { &g0 }),
        size_of_val(unsafe { &sys::g0 })
    );
    assert_eq!(
        align_of_val(unsafe { &g0 }),
        align_of_val(unsafe { &sys::g0 })
    );
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_newCAF() {
    let actual = {
        let reg: StgRegTable = todo!();
        let caf: StgIndStatic = todo!();
        let result: &StgInd = unsafe { &*newCAF(&raw mut reg, &raw mut caf) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_newCAF() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut caf: sys::StgIndStatic = todo!();
        let result: &StgInd = unsafe { transmute(&*sys::newCAF(&raw mut reg, &raw mut caf)) };
        todo!()
    };

    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut caf: StgIndStatic = todo!();
        let result: &StgInd = unsafe { &*newCAF(&raw mut reg, &raw mut caf) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setKeepCAFs() {
    let actual = {
        unsafe { setKeepCAFs() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setKeepCAFs() {
    let expected = {
        unsafe { sys::setKeepCAFs() };
        todo!()
    };

    let actual = {
        unsafe { setKeepCAFs() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_setHighMemDynamic() {
    let actual = {
        unsafe { setHighMemDynamic() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_setHighMemDynamic() {
    let expected = {
        unsafe { sys::setHighMemDynamic() };
        todo!()
    };

    let actual = {
        unsafe { setHighMemDynamic() };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_allocate() {
    let g = &mut Gen::new(100);

    let actual = {
        let cap: Capability = todo!();
        let n: W_ = Arbitrary::arbitrary(g);
        let result: StgPtr = unsafe { allocate(&raw mut cap, n) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_allocate(n: W_) -> bool {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let result: StgPtr = unsafe { sys::allocate(&raw mut cap, n) };
        todo!()
    };

    let actual = {
        let mut cap: Capability = todo!();
        let result: StgPtr = unsafe { allocate(&raw mut cap, n) };
        todo!()
    };

    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_dirty_MUT_VAR() {
    let actual = {
        let reg: StgRegTable = todo!();
        let mv: StgMutVar = todo!();
        let old: StgClosure = todo!();
        unsafe { dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_dirty_MUT_VAR() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut mv: sys::StgMutVar = todo!();
        let mut old: sys::StgClosure = todo!();
        unsafe { sys::dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };

    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut mv: StgMutVar = todo!();
        let mut old: StgClosure = todo!();
        unsafe { dirty_MUT_VAR(&raw mut reg, &raw mut mv, &raw mut old) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_rts_clearMemory() {
    let actual = {
        unsafe { rts_clearMemory() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_rts_clearMemory() {
    let expected = {
        unsafe { sys::rts_clearMemory() };
        todo!()
    };

    let actual = {
        unsafe { rts_clearMemory() };
        todo!()
    };
    assert_eq!(actual, expected);
}
