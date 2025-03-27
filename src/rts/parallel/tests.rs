use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_newSpark(reg: StgRegTable, p: StgClosure) -> bool {
    let expected = unsafe { transmute(sys::newSpark(&mut reg.into(), &mut p.into())) };
    let actual = unsafe { super::newSpark(&mut reg, &mut p) };
    actual == expected
}

#[test]
#[ignore]
fn test_newSpark() {
    let reg = Default::default();
    let p = Default::default();
    unsafe { super::newSpark(&mut reg, &mut p) };
    todo!("assert")
}
