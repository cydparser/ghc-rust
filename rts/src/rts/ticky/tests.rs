use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgEntCounter() {
    assert_eq!(
        size_of::<sys::_StgEntCounter>(),
        size_of::<_StgEntCounter>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgEntCounter"][size_of::<_StgEntCounter>() - 80usize];
    ["Alignment of _StgEntCounter"][align_of::<_StgEntCounter>() - 8usize];
    ["Offset of field: _StgEntCounter::registeredp"]
        [offset_of!(_StgEntCounter, registeredp) - 0usize];
    ["Offset of field: _StgEntCounter::arity"][offset_of!(_StgEntCounter, arity) - 8usize];
    ["Offset of field: _StgEntCounter::allocd"][offset_of!(_StgEntCounter, allocd) - 16usize];
    ["Offset of field: _StgEntCounter::str_"][offset_of!(_StgEntCounter, str_) - 24usize];
    ["Offset of field: _StgEntCounter::arg_kinds"][offset_of!(_StgEntCounter, arg_kinds) - 32usize];
    ["Offset of field: _StgEntCounter::ticky_json"]
        [offset_of!(_StgEntCounter, ticky_json) - 40usize];
    ["Offset of field: _StgEntCounter::info"][offset_of!(_StgEntCounter, info) - 48usize];
    ["Offset of field: _StgEntCounter::entry_count"]
        [offset_of!(_StgEntCounter, entry_count) - 56usize];
    ["Offset of field: _StgEntCounter::allocs"][offset_of!(_StgEntCounter, allocs) - 64usize];
    ["Offset of field: _StgEntCounter::link"][offset_of!(_StgEntCounter, link) - 72usize];
};

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_requestTickyCounterSamples() {
    let expected = {
        unsafe { sys::requestTickyCounterSamples() };
        todo!()
    };
    let actual = {
        unsafe { requestTickyCounterSamples() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_requestTickyCounterSamples() {
    let actual = {
        unsafe { requestTickyCounterSamples() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
