use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_snEntry() {
    assert_eq!(size_of::<sys::snEntry>(), size_of::<super::snEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of snEntry"][::core::mem::size_of::<snEntry>() - 24usize];
    ["Alignment of snEntry"][::core::mem::align_of::<snEntry>() - 8usize];
    ["Offset of field: snEntry::addr"][::core::mem::offset_of!(snEntry, addr) - 0usize];
    ["Offset of field: snEntry::old"][::core::mem::offset_of!(snEntry, old) - 8usize];
    ["Offset of field: snEntry::sn_obj"][::core::mem::offset_of!(snEntry, sn_obj) - 16usize];
};
