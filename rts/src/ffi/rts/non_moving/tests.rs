use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_updateRemembSetPushClosure_() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut p: sys::StgClosure_ = todo!();
        unsafe { sys::updateRemembSetPushClosure_(&raw mut reg, &raw mut p) };
        todo!()
    };

    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut p: StgClosure_ = todo!();
        unsafe { updateRemembSetPushClosure_(&raw mut reg, &raw mut p) };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_updateRemembSetPushClosure_() {
    let actual = {
        let reg: StgRegTable = todo!();
        let p: StgClosure_ = todo!();
        unsafe { updateRemembSetPushClosure_(&raw mut reg, &raw mut p) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_updateRemembSetPushThunk_() {
    let expected = {
        let mut reg: sys::StgRegTable = todo!();
        let mut p: sys::StgThunk_ = todo!();
        unsafe { sys::updateRemembSetPushThunk_(&raw mut reg, &raw mut p) };
        todo!()
    };

    let actual = {
        let mut reg: StgRegTable = todo!();
        let mut p: StgThunk = todo!();
        unsafe { updateRemembSetPushThunk_(&raw mut reg, &raw mut p) };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_updateRemembSetPushThunk_() {
    let actual = {
        let reg: StgRegTable = todo!();
        let p: StgThunk = todo!();
        unsafe { updateRemembSetPushThunk_(&raw mut reg, &raw mut p) };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_copyArray_barrier() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_copyArray_barrier() };
        todo!()
    };

    let actual = {
        let result: StgFunPtr = unsafe { stg_copyArray_barrier() };
        todo!()
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_copyArray_barrier() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_copyArray_barrier() };
        todo!()
    };

    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_nonmoving_write_barrier_enabled_layout() {
    assert_eq!(
        size_of_val(unsafe { &nonmoving_write_barrier_enabled }),
        size_of_val(unsafe { &sys::nonmoving_write_barrier_enabled })
    );
    assert_eq!(
        align_of_val(unsafe { &nonmoving_write_barrier_enabled }),
        align_of_val(unsafe { &sys::nonmoving_write_barrier_enabled })
    );
}
