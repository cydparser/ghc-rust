use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgEntCounter() {
    assert_eq!(
        size_of::<sys::_StgEntCounter>(),
        size_of::<super::_StgEntCounter>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgEntCounter"][::core::mem::size_of::<_StgEntCounter>() - 80usize];
    ["Alignment of _StgEntCounter"][::core::mem::align_of::<_StgEntCounter>() - 8usize];
    ["Offset of field: _StgEntCounter::registeredp"]
        [::core::mem::offset_of!(_StgEntCounter, registeredp) - 0usize];
    ["Offset of field: _StgEntCounter::arity"]
        [::core::mem::offset_of!(_StgEntCounter, arity) - 8usize];
    ["Offset of field: _StgEntCounter::allocd"]
        [::core::mem::offset_of!(_StgEntCounter, allocd) - 16usize];
    ["Offset of field: _StgEntCounter::str_"]
        [::core::mem::offset_of!(_StgEntCounter, str_) - 24usize];
    ["Offset of field: _StgEntCounter::arg_kinds"]
        [::core::mem::offset_of!(_StgEntCounter, arg_kinds) - 32usize];
    ["Offset of field: _StgEntCounter::ticky_json"]
        [::core::mem::offset_of!(_StgEntCounter, ticky_json) - 40usize];
    ["Offset of field: _StgEntCounter::info"]
        [::core::mem::offset_of!(_StgEntCounter, info) - 48usize];
    ["Offset of field: _StgEntCounter::entry_count"]
        [::core::mem::offset_of!(_StgEntCounter, entry_count) - 56usize];
    ["Offset of field: _StgEntCounter::allocs"]
        [::core::mem::offset_of!(_StgEntCounter, allocs) - 64usize];
    ["Offset of field: _StgEntCounter::link"]
        [::core::mem::offset_of!(_StgEntCounter, link) - 72usize];
};
