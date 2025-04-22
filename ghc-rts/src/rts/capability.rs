use std::{
    ffi::{c_char, c_int, c_uint, c_void},
    mem::transmute,
    ptr::{null, null_mut},
    slice,
};

use crate::{
    rts,
    stg::types::{StgInt, StgPtr, StgWord, StgWord64},
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub(crate) const CAPABILITY_ALIGNMENT: u32 = 64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct _CapIOManager {
    _unused: [u8; 0],
}

#[cfg(feature = "sys")]
impl From<_CapIOManager> for sys::_CapIOManager {
    fn from(x: _CapIOManager) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _CapIOManager {
    fn arbitrary(g: &mut Gen) -> Self {
        _CapIOManager {
            _unused: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type CapIOManager = _CapIOManager;

#[repr(C)]
#[repr(align(64))]
///cbindgen:no-export
pub(crate) struct Capability_ {
    pub f: StgFunTable,
    pub r: StgRegTable,
    pub no: u32,
    pub node: u32,
    pub running_task: *mut Task,
    pub in_haskell: bool,
    pub idle: u32,
    pub disabled: bool,
    pub run_queue_hd: *mut StgTSO,
    pub run_queue_tl: *mut StgTSO,
    pub n_run_queue: u32,
    pub suspended_ccalls: *mut InCall,
    pub n_suspended_ccalls: u32,
    pub mut_lists: *mut *mut bdescr,
    pub saved_mut_lists: *mut *mut bdescr,
    pub upd_rem_set: UpdRemSet,
    pub current_segments: *mut *mut NonmovingSegment,
    pub pinned_object_block: *mut bdescr,
    pub pinned_object_blocks: *mut bdescr,
    pub pinned_object_empty: *mut bdescr,
    pub weak_ptr_list_hd: *mut StgWeak,
    pub weak_ptr_list_tl: *mut StgWeak,
    pub context_switch: c_int,
    pub interrupt: c_int,
    pub total_allocated: u64,
    pub iomgr: *mut CapIOManager,
    pub free_tvar_watch_queues: *mut StgTVarWatchQueue,
    pub free_trec_chunks: *mut StgTRecChunk,
    pub free_trec_headers: *mut StgTRecHeader,
    pub transaction_tokens: u32,
}

#[cfg(feature = "sys")]
impl From<Capability_> for sys::Capability_ {
    fn from(x: Capability_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Capability_ {
    fn arbitrary(g: &mut Gen) -> Self {
        Capability_ {
            f: Arbitrary::arbitrary(g),
            r: Arbitrary::arbitrary(g),
            no: Arbitrary::arbitrary(g),
            node: Arbitrary::arbitrary(g),
            running_task: Arbitrary::arbitrary(g),
            in_haskell: Arbitrary::arbitrary(g),
            idle: Arbitrary::arbitrary(g),
            disabled: Arbitrary::arbitrary(g),
            run_queue_hd: Arbitrary::arbitrary(g),
            run_queue_tl: Arbitrary::arbitrary(g),
            n_run_queue: Arbitrary::arbitrary(g),
            suspended_ccalls: Arbitrary::arbitrary(g),
            n_suspended_ccalls: Arbitrary::arbitrary(g),
            mut_lists: Arbitrary::arbitrary(g),
            saved_mut_lists: Arbitrary::arbitrary(g),
            upd_rem_set: Arbitrary::arbitrary(g),
            current_segments: Arbitrary::arbitrary(g),
            pinned_object_block: Arbitrary::arbitrary(g),
            pinned_object_blocks: Arbitrary::arbitrary(g),
            pinned_object_empty: Arbitrary::arbitrary(g),
            weak_ptr_list_hd: Arbitrary::arbitrary(g),
            weak_ptr_list_tl: Arbitrary::arbitrary(g),
            context_switch: Arbitrary::arbitrary(g),
            interrupt: Arbitrary::arbitrary(g),
            total_allocated: Arbitrary::arbitrary(g),
            iomgr: Arbitrary::arbitrary(g),
            free_tvar_watch_queues: Arbitrary::arbitrary(g),
            free_trec_chunks: Arbitrary::arbitrary(g),
            free_trec_headers: Arbitrary::arbitrary(g),
            transaction_tokens: Arbitrary::arbitrary(g),
        }
    }
}

static mut _TODO_capabilities: [*mut Capability; 1usize] = todo!();

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum SyncType {
    SYNC_OTHER = 0,
    SYNC_GC_SEQ = 1,
    SYNC_GC_PAR = 2,
    SYNC_FLUSH_UPD_REM_SET = 3,
    SYNC_FLUSH_EVENT_LOG = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct PendingSync {
    pub type_: SyncType,
    pub idle: *mut bool,
    pub task: *mut Task,
}

#[cfg(feature = "sys")]
impl From<PendingSync> for sys::PendingSync {
    fn from(x: PendingSync) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for PendingSync {
    fn arbitrary(g: &mut Gen) -> Self {
        PendingSync {
            type_: Arbitrary::arbitrary(g),
            idle: Arbitrary::arbitrary(g),
            task: Arbitrary::arbitrary(g),
        }
    }
}

static mut _TODO_pending_sync: *mut PendingSync = null_mut();

static mut _TODO_n_numa_nodes: u32 = todo!();

static mut _TODO_numa_map: [u32; 16usize] = todo!();

#[repr(C)]
///cbindgen:no-export
pub(crate) struct PutMVar_ {
    pub mvar: StgStablePtr,
    pub link: *mut PutMVar_,
}

#[cfg(feature = "sys")]
impl From<PutMVar_> for sys::PutMVar_ {
    fn from(x: PutMVar_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for PutMVar_ {
    fn arbitrary(g: &mut Gen) -> Self {
        PutMVar_ {
            mvar: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type PutMVar = PutMVar_;

pub use ghc_rts_sys::{capabilities, n_numa_nodes, numa_map, pending_sync};
