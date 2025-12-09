use super::*;

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_createAdjustor(typeString: c_char) -> bool {
    let expected = {
        let hptr: StgStablePtr = todo!();
        let wptr: StgFunPtr = todo!();
        let mut typeString = typeString;
        let result: &c_void = unsafe { &*sys::createAdjustor(hptr, wptr, &raw mut typeString) };
        todo!()
    };
    let actual = {
        let hptr: StgStablePtr = todo!();
        let wptr: StgFunPtr = todo!();
        let mut typeString = typeString;
        let result: &c_void = unsafe { &*createAdjustor(hptr, wptr, &raw mut typeString) };
        todo!()
    };
    actual == expected
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_createAdjustor() {
    let g = &mut Gen::new(100);
    let actual = {
        let hptr: StgStablePtr = todo!();
        let wptr: StgFunPtr = todo!();
        let mut typeString: c_char = Arbitrary::arbitrary(g);
        let result: &c_void = unsafe { &*createAdjustor(hptr, wptr, &raw mut typeString) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_freeHaskellFunctionPtr() {
    let expected = {
        let mut ptr: c_void = todo!();
        unsafe { sys::freeHaskellFunctionPtr(&raw mut ptr) };
        todo!()
    };
    let actual = {
        let mut ptr: c_void = todo!();
        unsafe { freeHaskellFunctionPtr(&raw mut ptr) };
        todo!()
    };
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_freeHaskellFunctionPtr() {
    let actual = {
        let ptr: c_void = todo!();
        unsafe { freeHaskellFunctionPtr(&raw mut ptr) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
