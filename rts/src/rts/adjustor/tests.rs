use super::*;

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_freeHaskellFunctionPtr() {
    todo!()
}

#[test]
#[ignore]
fn test_freeHaskellFunctionPtr() {
    let ptr = null_mut();
    unsafe { freeHaskellFunctionPtr(ptr) };
    todo!("assert")
}
