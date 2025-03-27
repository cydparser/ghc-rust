use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_GEN() {
    assert_eq!(sys::ARG_GEN, super::ARG_GEN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_GEN_BIG() {
    assert_eq!(sys::ARG_GEN_BIG, super::ARG_GEN_BIG.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_BCO() {
    assert_eq!(sys::ARG_BCO, super::ARG_BCO.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NONE() {
    assert_eq!(sys::ARG_NONE, super::ARG_NONE.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_N() {
    assert_eq!(sys::ARG_N, super::ARG_N.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_P() {
    assert_eq!(sys::ARG_P, super::ARG_P.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_F() {
    assert_eq!(sys::ARG_F, super::ARG_F.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_D() {
    assert_eq!(sys::ARG_D, super::ARG_D.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_L() {
    assert_eq!(sys::ARG_L, super::ARG_L.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V16() {
    assert_eq!(sys::ARG_V16, super::ARG_V16.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V32() {
    assert_eq!(sys::ARG_V32, super::ARG_V32.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_V64() {
    assert_eq!(sys::ARG_V64, super::ARG_V64.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NN() {
    assert_eq!(sys::ARG_NN, super::ARG_NN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NP() {
    assert_eq!(sys::ARG_NP, super::ARG_NP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PN() {
    assert_eq!(sys::ARG_PN, super::ARG_PN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PP() {
    assert_eq!(sys::ARG_PP, super::ARG_PP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NNN() {
    assert_eq!(sys::ARG_NNN, super::ARG_NNN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NNP() {
    assert_eq!(sys::ARG_NNP, super::ARG_NNP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NPN() {
    assert_eq!(sys::ARG_NPN, super::ARG_NPN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_NPP() {
    assert_eq!(sys::ARG_NPP, super::ARG_NPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PNN() {
    assert_eq!(sys::ARG_PNN, super::ARG_PNN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PNP() {
    assert_eq!(sys::ARG_PNP, super::ARG_PNP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPN() {
    assert_eq!(sys::ARG_PPN, super::ARG_PPN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPP() {
    assert_eq!(sys::ARG_PPP, super::ARG_PPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPP() {
    assert_eq!(sys::ARG_PPPP, super::ARG_PPPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPP() {
    assert_eq!(sys::ARG_PPPPP, super::ARG_PPPPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPP() {
    assert_eq!(sys::ARG_PPPPPP, super::ARG_PPPPPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPPP() {
    assert_eq!(sys::ARG_PPPPPPP, super::ARG_PPPPPPP.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_ARG_PPPPPPPP() {
    assert_eq!(sys::ARG_PPPPPPPP, super::ARG_PPPPPPPP.into());
}
