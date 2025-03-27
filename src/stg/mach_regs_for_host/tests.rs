use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_NO_REGS() {
    assert_eq!(sys::MACHREGS_NO_REGS, super::MACHREGS_NO_REGS.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_MACHREGS_x86_64() {
    assert_eq!(sys::MACHREGS_x86_64, super::MACHREGS_x86_64.into());
}
