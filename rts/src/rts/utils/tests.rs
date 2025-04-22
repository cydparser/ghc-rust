use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_genericRaise(sig: ::core::ffi::c_int) -> bool {
    let expected = unsafe { transmute(sys::genericRaise(sig.into())) };
    let actual = unsafe { super::genericRaise(sig) };
    actual == expected
}

#[test]
#[ignore]
fn test_genericRaise() {
    let sig = Default::default();
    unsafe { super::genericRaise(sig) };
    todo!("assert")
}
