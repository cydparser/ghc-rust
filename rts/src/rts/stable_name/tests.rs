use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
#[cfg(feature = "sys")]
#[test]
fn sys_size_snEntry() {
    assert_eq!(size_of::<sys::snEntry>(), size_of::<snEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of snEntry"][size_of::<snEntry>() - 24usize];
    ["Alignment of snEntry"][align_of::<snEntry>() - 8usize];
    ["Offset of field: snEntry::addr"][offset_of!(snEntry, addr) - 0usize];
    ["Offset of field: snEntry::old"][offset_of!(snEntry, old) - 8usize];
    ["Offset of field: snEntry::sn_obj"][offset_of!(snEntry, sn_obj) - 16usize];
};
