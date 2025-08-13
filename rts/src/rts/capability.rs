use crate::prelude::*;
use crate::rts::storage::block::bdescr;
use crate::rts::storage::closures::{StgTRecChunk, StgTRecHeader, StgTVarWatchQueue, StgWeak};
use crate::rts::storage::tso::StgTSO;
use crate::rts::task::{InCall, Task};
use crate::stg::regs::{StgFunTable, StgRegTable};
use crate::stg::types::StgStablePtr;

#[cfg(test)]
mod tests;

// TODO: Replace these with implementation.
struct _CapIOManager;
struct NonmovingSegment;
struct UpdRemSet;

pub(crate) const CAPABILITY_ALIGNMENT: u32 = 64;

type CapIOManager = _CapIOManager;

pub type Capability = Capability_;

#[repr(C)]
#[repr(align(64))]
///cbindgen:no-export
pub struct Capability_ {
    f: StgFunTable,
    r: StgRegTable,
    no: u32,
    node: u32,
    running_task: *mut Task,
    in_haskell: bool,
    idle: u32,
    disabled: bool,
    run_queue_hd: *mut StgTSO,
    run_queue_tl: *mut StgTSO,
    n_run_queue: u32,
    suspended_ccalls: *mut InCall,
    n_suspended_ccalls: u32,
    mut_lists: *mut *mut bdescr,
    saved_mut_lists: *mut *mut bdescr,
    upd_rem_set: UpdRemSet,
    current_segments: *mut *mut NonmovingSegment,
    pinned_object_block: *mut bdescr,
    pinned_object_blocks: *mut bdescr,
    pinned_object_empty: *mut bdescr,
    weak_ptr_list_hd: *mut StgWeak,
    weak_ptr_list_tl: *mut StgWeak,
    context_switch: c_int,
    interrupt: c_int,
    total_allocated: u64,
    iomgr: *mut CapIOManager,
    free_tvar_watch_queues: *mut StgTVarWatchQueue,
    free_trec_chunks: *mut StgTRecChunk,
    free_trec_headers: *mut StgTRecHeader,
    transaction_tokens: u32,
}

static mut capabilities: [*mut Capability; 1] = [null_mut()];

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum SyncType {
    SYNC_OTHER = 0,
    SYNC_GC_SEQ = 1,
    SYNC_GC_PAR = 2,
    SYNC_FLUSH_UPD_REM_SET = 3,
    SYNC_FLUSH_EVENT_LOG = 4,
}

#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
struct PendingSync {
    type_: SyncType,
    idle: *mut bool,
    task: *mut Task,
}

static mut pending_sync: *mut PendingSync = null_mut();

static mut n_numa_nodes: u32 = 0;

static mut numa_map: [u32; 16] = [0; _];

pub(crate) type PutMVar = PutMVar_;

///cbindgen:no-export
struct PutMVar_ {
    mvar: StgStablePtr,
    link: *mut PutMVar_,
}
