use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_createAdjustor(
    hptr: StgStablePtr,
    wptr: StgFunPtr,
    typeString: ::core::ffi::c_char,
) -> bool {
    let expected = unsafe {
        transmute(sys::createAdjustor(
            hptr.into(),
            wptr.into(),
            &mut typeString.into(),
        ))
    };
    let actual = unsafe { super::createAdjustor(hptr, wptr, &mut typeString) };
    actual == expected
}

#[test]
#[ignore]
fn test_createAdjustor() {
    let hptr = Default::default();
    let wptr = Default::default();
    let mut typeString = Default::default();
    unsafe { super::createAdjustor(hptr, wptr, &mut typeString) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_freeHaskellFunctionPtr() {
    let mut ptr = Default::default();
    unsafe { super::freeHaskellFunctionPtr(&mut ptr) };
    todo!("assert")
}
