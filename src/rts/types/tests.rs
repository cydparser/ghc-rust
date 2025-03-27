use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgInfoTable_() {
    assert_eq!(
        size_of::<sys::StgInfoTable_>(),
        size_of::<super::StgInfoTable_>()
    )
}
