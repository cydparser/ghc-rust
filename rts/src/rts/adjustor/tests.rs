use super::*;

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
    assert_eq!(expected, actual);
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
