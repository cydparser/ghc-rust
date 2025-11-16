use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_paniczh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_paniczh() };
        todo!()
    };
    let actual = {
        let result: StgFunPtr = unsafe { stg_paniczh() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_paniczh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_paniczh() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_absentErrorzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_absentErrorzh() };
        todo!()
    };
    let actual = {
        let result: StgFunPtr = unsafe { stg_absentErrorzh() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_absentErrorzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_absentErrorzh() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_getThreadAllocationCounterzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_getThreadAllocationCounterzh() };
        todo!()
    };
    let actual = {
        let result: StgFunPtr = unsafe { stg_getThreadAllocationCounterzh() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_getThreadAllocationCounterzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_getThreadAllocationCounterzh() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_stg_getOtherThreadAllocationCounterzh() {
    let expected = {
        let result: StgFunPtr = unsafe { sys::stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };
    let actual = {
        let result: StgFunPtr = unsafe { stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_stg_getOtherThreadAllocationCounterzh() {
    let actual = {
        let result: StgFunPtr = unsafe { stg_getOtherThreadAllocationCounterzh() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
