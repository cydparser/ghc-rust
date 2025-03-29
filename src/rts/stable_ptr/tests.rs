use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_getStablePtr(p: StgPtr) -> bool {
    let expected = unsafe { transmute(sys::getStablePtr(p.into())) };
    let actual = unsafe { super::getStablePtr(p) };
    actual == expected
}

#[test]
#[ignore]
fn test_getStablePtr() {
    let p = Default::default();
    unsafe { super::getStablePtr(p) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_spEntry() {
    assert_eq!(size_of::<sys::spEntry>(), size_of::<super::spEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of spEntry"][::core::mem::size_of::<spEntry>() - 8usize];
    ["Alignment of spEntry"][::core::mem::align_of::<spEntry>() - 8usize];
    ["Offset of field: spEntry::addr"][::core::mem::offset_of!(spEntry, addr) - 0usize];
};
