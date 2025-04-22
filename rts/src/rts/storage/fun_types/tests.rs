use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_GEN() {
    assert_eq!(sys::ARG_GEN, super::ARG_GEN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_GEN_BIG() {
    assert_eq!(sys::ARG_GEN_BIG, super::ARG_GEN_BIG);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_BCO() {
    assert_eq!(sys::ARG_BCO, super::ARG_BCO);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NONE() {
    assert_eq!(sys::ARG_NONE, super::ARG_NONE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_N() {
    assert_eq!(sys::ARG_N, super::ARG_N);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_P() {
    assert_eq!(sys::ARG_P, super::ARG_P);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_F() {
    assert_eq!(sys::ARG_F, super::ARG_F);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_D() {
    assert_eq!(sys::ARG_D, super::ARG_D);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_L() {
    assert_eq!(sys::ARG_L, super::ARG_L);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V16() {
    assert_eq!(sys::ARG_V16, super::ARG_V16);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V32() {
    assert_eq!(sys::ARG_V32, super::ARG_V32);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V64() {
    assert_eq!(sys::ARG_V64, super::ARG_V64);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NN() {
    assert_eq!(sys::ARG_NN, super::ARG_NN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NP() {
    assert_eq!(sys::ARG_NP, super::ARG_NP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PN() {
    assert_eq!(sys::ARG_PN, super::ARG_PN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PP() {
    assert_eq!(sys::ARG_PP, super::ARG_PP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NNN() {
    assert_eq!(sys::ARG_NNN, super::ARG_NNN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NNP() {
    assert_eq!(sys::ARG_NNP, super::ARG_NNP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NPN() {
    assert_eq!(sys::ARG_NPN, super::ARG_NPN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NPP() {
    assert_eq!(sys::ARG_NPP, super::ARG_NPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PNN() {
    assert_eq!(sys::ARG_PNN, super::ARG_PNN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PNP() {
    assert_eq!(sys::ARG_PNP, super::ARG_PNP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPN() {
    assert_eq!(sys::ARG_PPN, super::ARG_PPN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPP() {
    assert_eq!(sys::ARG_PPP, super::ARG_PPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPP() {
    assert_eq!(sys::ARG_PPPP, super::ARG_PPPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPP() {
    assert_eq!(sys::ARG_PPPPP, super::ARG_PPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPP() {
    assert_eq!(sys::ARG_PPPPPP, super::ARG_PPPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPPP() {
    assert_eq!(sys::ARG_PPPPPPP, super::ARG_PPPPPPP);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPPPP() {
    assert_eq!(sys::ARG_PPPPPPPP, super::ARG_PPPPPPPP);
}
