use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STACK_DIRTY() {
    assert_eq!(sys::STACK_DIRTY, STACK_DIRTY);
}

#[cfg(feature = "sys")]
#[test]
fn sys_eq_STACK_SANE() {
    assert_eq!(sys::STACK_SANE, STACK_SANE);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTSOProfInfo() {
    assert_eq!(
        size_of::<sys::StgTSOProfInfo>(),
        size_of::<StgTSOProfInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSOProfInfo"][size_of::<StgTSOProfInfo>() - 8usize];
    ["Alignment of StgTSOProfInfo"][align_of::<StgTSOProfInfo>() - 8usize];
    ["Offset of field: StgTSOProfInfo::cccs"][offset_of!(StgTSOProfInfo, cccs) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTSOBlockInfo() {
    assert_eq!(
        size_of::<sys::StgTSOBlockInfo>(),
        size_of::<StgTSOBlockInfo>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSOBlockInfo"][size_of::<StgTSOBlockInfo>() - 8usize];
    ["Alignment of StgTSOBlockInfo"][align_of::<StgTSOBlockInfo>() - 8usize];
    ["Offset of field: StgTSOBlockInfo::closure"][offset_of!(StgTSOBlockInfo, closure) - 0usize];
    ["Offset of field: StgTSOBlockInfo::prev"][offset_of!(StgTSOBlockInfo, prev) - 0usize];
    ["Offset of field: StgTSOBlockInfo::bh"][offset_of!(StgTSOBlockInfo, bh) - 0usize];
    ["Offset of field: StgTSOBlockInfo::throwto"][offset_of!(StgTSOBlockInfo, throwto) - 0usize];
    ["Offset of field: StgTSOBlockInfo::wakeup"][offset_of!(StgTSOBlockInfo, wakeup) - 0usize];
    ["Offset of field: StgTSOBlockInfo::fd"][offset_of!(StgTSOBlockInfo, fd) - 0usize];
    ["Offset of field: StgTSOBlockInfo::target"][offset_of!(StgTSOBlockInfo, target) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTSO_() {
    assert_eq!(size_of::<sys::StgTSO_>(), size_of::<StgTSO_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTSO_"][size_of::<StgTSO_>() - 136usize];
    ["Alignment of StgTSO_"][align_of::<StgTSO_>() - 8usize];
    ["Offset of field: StgTSO_::header"][offset_of!(StgTSO_, header) - 0usize];
    ["Offset of field: StgTSO_::_link"][offset_of!(StgTSO_, _link) - 8usize];
    ["Offset of field: StgTSO_::global_link"][offset_of!(StgTSO_, global_link) - 16usize];
    ["Offset of field: StgTSO_::stackobj"][offset_of!(StgTSO_, stackobj) - 24usize];
    ["Offset of field: StgTSO_::what_next"][offset_of!(StgTSO_, what_next) - 32usize];
    ["Offset of field: StgTSO_::flags"][offset_of!(StgTSO_, flags) - 36usize];
    ["Offset of field: StgTSO_::why_blocked"][offset_of!(StgTSO_, why_blocked) - 40usize];
    ["Offset of field: StgTSO_::block_info"][offset_of!(StgTSO_, block_info) - 48usize];
    ["Offset of field: StgTSO_::id"][offset_of!(StgTSO_, id) - 56usize];
    ["Offset of field: StgTSO_::saved_errno"][offset_of!(StgTSO_, saved_errno) - 64usize];
    ["Offset of field: StgTSO_::dirty"][offset_of!(StgTSO_, dirty) - 68usize];
    ["Offset of field: StgTSO_::bound"][offset_of!(StgTSO_, bound) - 72usize];
    ["Offset of field: StgTSO_::cap"][offset_of!(StgTSO_, cap) - 80usize];
    ["Offset of field: StgTSO_::trec"][offset_of!(StgTSO_, trec) - 88usize];
    ["Offset of field: StgTSO_::label"][offset_of!(StgTSO_, label) - 96usize];
    ["Offset of field: StgTSO_::blocked_exceptions"]
        [offset_of!(StgTSO_, blocked_exceptions) - 104usize];
    ["Offset of field: StgTSO_::bq"][offset_of!(StgTSO_, bq) - 112usize];
    ["Offset of field: StgTSO_::alloc_limit"][offset_of!(StgTSO_, alloc_limit) - 120usize];
    ["Offset of field: StgTSO_::tot_stack_size"][offset_of!(StgTSO_, tot_stack_size) - 128usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgStack_() {
    assert_eq!(size_of::<sys::StgStack_>(), size_of::<StgStack_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgStack_"][size_of::<StgStack_>() - 24usize];
    ["Alignment of StgStack_"][align_of::<StgStack_>() - 8usize];
    ["Offset of field: StgStack_::header"][offset_of!(StgStack_, header) - 0usize];
    ["Offset of field: StgStack_::stack_size"][offset_of!(StgStack_, stack_size) - 8usize];
    ["Offset of field: StgStack_::dirty"][offset_of!(StgStack_, dirty) - 12usize];
    ["Offset of field: StgStack_::marking"][offset_of!(StgStack_, marking) - 13usize];
    ["Offset of field: StgStack_::sp"][offset_of!(StgStack_, sp) - 16usize];
    ["Offset of field: StgStack_::stack"][offset_of!(StgStack_, stack) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_InCall_() {
    assert_eq!(size_of::<sys::InCall_>(), size_of::<InCall_>())
}
