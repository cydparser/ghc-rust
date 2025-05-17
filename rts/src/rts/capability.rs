use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
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
#[derive(Clone)]
struct Capability_Owned {
    pub f: StgFunTable,
    pub r: StgRegTable,
    pub no: u32,
    pub node: u32,
    pub in_haskell: bool,
    pub idle: u32,
    pub disabled: bool,
    pub n_run_queue: u32,
    pub n_suspended_ccalls: u32,
    pub upd_rem_set: UpdRemSet,
    pub context_switch: c_int,
    pub interrupt: c_int,
    pub total_allocated: u64,
    pub transaction_tokens: u32,
}

#[cfg(test)]
impl Arbitrary for Capability_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        Capability_Owned {
            f: Arbitrary::arbitrary(g),
            r: Arbitrary::arbitrary(g),
            no: Arbitrary::arbitrary(g),
            node: Arbitrary::arbitrary(g),
            in_haskell: Arbitrary::arbitrary(g),
            idle: Arbitrary::arbitrary(g),
            disabled: Arbitrary::arbitrary(g),
            n_run_queue: Arbitrary::arbitrary(g),
            n_suspended_ccalls: Arbitrary::arbitrary(g),
            upd_rem_set: Arbitrary::arbitrary(g),
            context_switch: Arbitrary::arbitrary(g),
            interrupt: Arbitrary::arbitrary(g),
            total_allocated: Arbitrary::arbitrary(g),
            transaction_tokens: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct Capability_Pointees {
    pub running_task: Task,
    pub run_queue_hd: StgTSO,
    pub run_queue_tl: StgTSO,
    pub suspended_ccalls: InCall,
    pub mut_lists: *mut bdescr,
    pub saved_mut_lists: *mut bdescr,
    pub current_segments: *mut NonmovingSegment,
    pub pinned_object_block: bdescr,
    pub pinned_object_blocks: bdescr,
    pub pinned_object_empty: bdescr,
    pub weak_ptr_list_hd: StgWeak,
    pub weak_ptr_list_tl: StgWeak,
    pub iomgr: CapIOManager,
    pub free_tvar_watch_queues: StgTVarWatchQueue,
    pub free_trec_chunks: StgTRecChunk,
    pub free_trec_headers: StgTRecHeader,
}

#[cfg(test)]
impl Arbitrary for Capability_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        Capability_Pointees {
            running_task: Arbitrary::arbitrary(g),
            run_queue_hd: Arbitrary::arbitrary(g),
            run_queue_tl: Arbitrary::arbitrary(g),
            suspended_ccalls: Arbitrary::arbitrary(g),
            mut_lists: Arbitrary::arbitrary(g),
            saved_mut_lists: Arbitrary::arbitrary(g),
            current_segments: Arbitrary::arbitrary(g),
            pinned_object_block: Arbitrary::arbitrary(g),
            pinned_object_blocks: Arbitrary::arbitrary(g),
            pinned_object_empty: Arbitrary::arbitrary(g),
            weak_ptr_list_hd: Arbitrary::arbitrary(g),
            weak_ptr_list_tl: Arbitrary::arbitrary(g),
            iomgr: Arbitrary::arbitrary(g),
            free_tvar_watch_queues: Arbitrary::arbitrary(g),
            free_trec_chunks: Arbitrary::arbitrary(g),
            free_trec_headers: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for Capability_ {
    type Owned = Capability_Owned;
    type Pointees = Capability_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            f: owned.f.clone(),
            r: owned.r.clone(),
            no: owned.no,
            node: owned.node,
            in_haskell: owned.in_haskell.clone(),
            idle: owned.idle,
            disabled: owned.disabled.clone(),
            n_run_queue: owned.n_run_queue,
            n_suspended_ccalls: owned.n_suspended_ccalls,
            upd_rem_set: owned.upd_rem_set.clone(),
            context_switch: owned.context_switch,
            interrupt: owned.interrupt,
            total_allocated: owned.total_allocated.clone(),
            transaction_tokens: owned.transaction_tokens,
            running_task: unsafe { &raw mut (*pointees).running_task },
            run_queue_hd: unsafe { &raw mut (*pointees).run_queue_hd },
            run_queue_tl: unsafe { &raw mut (*pointees).run_queue_tl },
            suspended_ccalls: unsafe { &raw mut (*pointees).suspended_ccalls },
            mut_lists: unsafe { &raw mut (*pointees).mut_lists },
            saved_mut_lists: unsafe { &raw mut (*pointees).saved_mut_lists },
            current_segments: unsafe { &raw mut (*pointees).current_segments },
            pinned_object_block: unsafe { &raw mut (*pointees).pinned_object_block },
            pinned_object_blocks: unsafe { &raw mut (*pointees).pinned_object_blocks },
            pinned_object_empty: unsafe { &raw mut (*pointees).pinned_object_empty },
            weak_ptr_list_hd: unsafe { &raw mut (*pointees).weak_ptr_list_hd },
            weak_ptr_list_tl: unsafe { &raw mut (*pointees).weak_ptr_list_tl },
            iomgr: unsafe { &raw mut (*pointees).iomgr },
            free_tvar_watch_queues: unsafe { &raw mut (*pointees).free_tvar_watch_queues },
            free_trec_chunks: unsafe { &raw mut (*pointees).free_trec_chunks },
            free_trec_headers: unsafe { &raw mut (*pointees).free_trec_headers },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            f: self.f.clone(),
            r: self.r.clone(),
            no: self.no,
            node: self.node,
            in_haskell: self.in_haskell.clone(),
            idle: self.idle,
            disabled: self.disabled.clone(),
            n_run_queue: self.n_run_queue,
            n_suspended_ccalls: self.n_suspended_ccalls,
            upd_rem_set: self.upd_rem_set.clone(),
            context_switch: self.context_switch,
            interrupt: self.interrupt,
            total_allocated: self.total_allocated.clone(),
            transaction_tokens: self.transaction_tokens,
        }
    }
}

static mut capabilities: [*mut Capability; 1usize] = [];

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
#[derive(Clone)]
struct PendingSyncOwned {
    pub type_: SyncType,
}

#[cfg(test)]
impl Arbitrary for PendingSyncOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        PendingSyncOwned {
            type_: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct PendingSyncPointees {
    pub idle: bool,
    pub task: Task,
}

#[cfg(test)]
impl Arbitrary for PendingSyncPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        PendingSyncPointees {
            idle: Arbitrary::arbitrary(g),
            task: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for PendingSync {
    type Owned = PendingSyncOwned;
    type Pointees = PendingSyncPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            type_: owned.type_.clone(),
            idle: unsafe { &raw mut (*pointees).idle },
            task: unsafe { &raw mut (*pointees).task },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            type_: self.type_.clone(),
        }
    }
}

static mut pending_sync: *mut PendingSync = null_mut();

static mut n_numa_nodes: u32 = 0;

static mut numa_map: [u32; 16usize] = [];

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
#[derive(Clone)]
struct PutMVar_Owned {}
#[cfg(test)]
impl Arbitrary for PutMVar_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        PutMVar_Owned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct PutMVar_Pointees {
    pub mvar: StgStablePtr,
    pub link: PutMVar_,
}

#[cfg(test)]
impl Arbitrary for PutMVar_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        PutMVar_Pointees {
            mvar: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for PutMVar_ {
    type Owned = PutMVar_Owned;
    type Pointees = PutMVar_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            mvar: unsafe { &raw mut (*pointees).mvar },
            link: unsafe { &raw mut (*pointees).link },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

pub(crate) type PutMVar = PutMVar_;
