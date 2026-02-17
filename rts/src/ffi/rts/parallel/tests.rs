use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_newSpark() {
    let expected: StgInt = {
        let mut reg: sys::StgRegTable = todo!();
        let mut p: sys::StgClosure = todo!();
        unsafe { sys::newSpark(&raw mut reg, &raw mut p) }
    };

    let actual: StgInt = {
        let mut reg: StgRegTable = todo!();
        let mut p: StgClosure = todo!();
        unsafe { newSpark(&raw mut reg, &raw mut p) }
    };

    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_newSpark() {
    let actual: StgInt = {
        let reg: StgRegTable = todo!();
        let p: StgClosure = todo!();
        unsafe { newSpark(&raw mut reg, &raw mut p) }
    };

    let expected: StgInt = todo!();
    assert_eq!(expected, actual);
}
