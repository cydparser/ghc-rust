use std::{
    ffi::{c_char, c_int, c_uint, c_void},
    mem::{size_of, transmute},
    ptr::{null, null_mut},
};

use super::*;
use crate::{
    rts,
    stg::types::{StgInt, StgPtr, StgWord, StgWord64},
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
#[cfg(feature = "sys")]
#[test]
fn test_eq_CAPABILITY_ALIGNMENT() {
    assert_eq!(sys::CAPABILITY_ALIGNMENT, CAPABILITY_ALIGNMENT);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of__CapIOManager() {
    assert_eq!(size_of::<sys::_CapIOManager>(), size_of::<_CapIOManager>())
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_Capability_() {
    assert_eq!(size_of::<sys::Capability_>(), size_of::<Capability_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
pub(crate) const _: () = {
    ["Size of Capability_"][::core::mem::size_of::<Capability_>() - 1280usize];
    ["Alignment of Capability_"][::core::mem::align_of::<Capability_>() - 64usize];
    ["Offset of field: Capability_::f"][::core::mem::offset_of!(Capability_, f) - 0usize];
    ["Offset of field: Capability_::r"][::core::mem::offset_of!(Capability_, r) - 24usize];
    ["Offset of field: Capability_::no"][::core::mem::offset_of!(Capability_, no) - 944usize];
    ["Offset of field: Capability_::node"][::core::mem::offset_of!(Capability_, node) - 948usize];
    ["Offset of field: Capability_::running_task"]
        [::core::mem::offset_of!(Capability_, running_task) - 952usize];
    ["Offset of field: Capability_::in_haskell"]
        [::core::mem::offset_of!(Capability_, in_haskell) - 960usize];
    ["Offset of field: Capability_::idle"][::core::mem::offset_of!(Capability_, idle) - 964usize];
    ["Offset of field: Capability_::disabled"]
        [::core::mem::offset_of!(Capability_, disabled) - 968usize];
    ["Offset of field: Capability_::run_queue_hd"]
        [::core::mem::offset_of!(Capability_, run_queue_hd) - 976usize];
    ["Offset of field: Capability_::run_queue_tl"]
        [::core::mem::offset_of!(Capability_, run_queue_tl) - 984usize];
    ["Offset of field: Capability_::n_run_queue"]
        [::core::mem::offset_of!(Capability_, n_run_queue) - 992usize];
    ["Offset of field: Capability_::suspended_ccalls"]
        [::core::mem::offset_of!(Capability_, suspended_ccalls) - 1000usize];
    ["Offset of field: Capability_::n_suspended_ccalls"]
        [::core::mem::offset_of!(Capability_, n_suspended_ccalls) - 1008usize];
    ["Offset of field: Capability_::mut_lists"]
        [::core::mem::offset_of!(Capability_, mut_lists) - 1016usize];
    ["Offset of field: Capability_::saved_mut_lists"]
        [::core::mem::offset_of!(Capability_, saved_mut_lists) - 1024usize];
    ["Offset of field: Capability_::upd_rem_set"]
        [::core::mem::offset_of!(Capability_, upd_rem_set) - 1032usize];
    ["Offset of field: Capability_::current_segments"]
        [::core::mem::offset_of!(Capability_, current_segments) - 1144usize];
    ["Offset of field: Capability_::pinned_object_block"]
        [::core::mem::offset_of!(Capability_, pinned_object_block) - 1152usize];
    ["Offset of field: Capability_::pinned_object_blocks"]
        [::core::mem::offset_of!(Capability_, pinned_object_blocks) - 1160usize];
    ["Offset of field: Capability_::pinned_object_empty"]
        [::core::mem::offset_of!(Capability_, pinned_object_empty) - 1168usize];
    ["Offset of field: Capability_::weak_ptr_list_hd"]
        [::core::mem::offset_of!(Capability_, weak_ptr_list_hd) - 1176usize];
    ["Offset of field: Capability_::weak_ptr_list_tl"]
        [::core::mem::offset_of!(Capability_, weak_ptr_list_tl) - 1184usize];
    ["Offset of field: Capability_::context_switch"]
        [::core::mem::offset_of!(Capability_, context_switch) - 1192usize];
    ["Offset of field: Capability_::interrupt"]
        [::core::mem::offset_of!(Capability_, interrupt) - 1196usize];
    ["Offset of field: Capability_::total_allocated"]
        [::core::mem::offset_of!(Capability_, total_allocated) - 1200usize];
    ["Offset of field: Capability_::iomgr"]
        [::core::mem::offset_of!(Capability_, iomgr) - 1208usize];
    ["Offset of field: Capability_::free_tvar_watch_queues"]
        [::core::mem::offset_of!(Capability_, free_tvar_watch_queues) - 1216usize];
    ["Offset of field: Capability_::free_trec_chunks"]
        [::core::mem::offset_of!(Capability_, free_trec_chunks) - 1224usize];
    ["Offset of field: Capability_::free_trec_headers"]
        [::core::mem::offset_of!(Capability_, free_trec_headers) - 1232usize];
    ["Offset of field: Capability_::transaction_tokens"]
        [::core::mem::offset_of!(Capability_, transaction_tokens) - 1240usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_PendingSync() {
    assert_eq!(size_of::<sys::PendingSync>(), size_of::<PendingSync>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
pub(crate) const _: () = {
    ["Size of PendingSync"][::core::mem::size_of::<PendingSync>() - 24usize];
    ["Alignment of PendingSync"][::core::mem::align_of::<PendingSync>() - 8usize];
    ["Offset of field: PendingSync::type_"][::core::mem::offset_of!(PendingSync, type_) - 0usize];
    ["Offset of field: PendingSync::idle"][::core::mem::offset_of!(PendingSync, idle) - 8usize];
    ["Offset of field: PendingSync::task"][::core::mem::offset_of!(PendingSync, task) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_PutMVar_() {
    assert_eq!(size_of::<sys::PutMVar_>(), size_of::<PutMVar_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
pub(crate) const _: () = {
    ["Size of PutMVar_"][::core::mem::size_of::<PutMVar_>() - 16usize];
    ["Alignment of PutMVar_"][::core::mem::align_of::<PutMVar_>() - 8usize];
    ["Offset of field: PutMVar_::mvar"][::core::mem::offset_of!(PutMVar_, mvar) - 0usize];
    ["Offset of field: PutMVar_::link"][::core::mem::offset_of!(PutMVar_, link) - 8usize];
};
