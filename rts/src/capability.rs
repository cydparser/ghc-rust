use crate::capability::{
    _CapIOManager, CapIOManager, Capability_, PendingSync, SyncType, contextSwitchCapability,
    getCapability, interruptCapability, recordMutableCap, stopCapability,
};
use crate::ffi::rts::config::MAX_N_CAPABILITIES;
use crate::ffi::rts::constants::MAX_NUMA_NODES;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::CostCentreStack_;
use crate::ffi::rts::storage::block::{
    BLOCK_SIZE_W, Bdescr, allocBlockOnNode_lock, bdescr, bdescr_,
};
use crate::ffi::rts::storage::block::{
    BLOCK_SIZE_W, Bdescr, allocBlockOnNode_lock, bdescr, bdescr_,
};
use crate::ffi::rts::storage::closures::{StgTRecChunk, StgTRecHeader, StgTVarWatchQueue, StgWeak};
use crate::ffi::rts::storage::closures::{StgTRecChunk, StgTRecHeader, StgTVarWatchQueue, StgWeak};
use crate::ffi::rts::storage::gc::nursery_;
use crate::ffi::rts::storage::tso::StgTSO_;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, __stg_gc_enter_1, __stg_gc_fun, stg_END_STM_CHUNK_LIST_closure,
    stg_END_STM_WATCH_QUEUE_closure, stg_END_TSO_QUEUE_closure, stg_NO_TREC_closure,
};
use crate::ffi::stg::regs::{StgFunTable, StgRegTable};
use crate::ffi::stg::regs::{StgFunTable, StgRegTable, StgUnion};
use crate::ffi::stg::types::{StgFunPtr, StgPtr, StgWord, StgWord128, StgWord256, StgWord512};
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::io_manager::{initCapabilityIOManager, markCapabilityIOManager};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::sm::gc::evac_fn;
use crate::sm::non_moving::NonmovingSegment;
use crate::sm::non_moving::NonmovingSegment;
use crate::sm::non_moving_mark::UpdRemSet;
use crate::sm::non_moving_mark::{
    C2RustUnnamed_2, C2RustUnnamed_5, MarkQueue_, MarkQueueBlock, MarkQueueEnt, UpdRemSet,
};
use crate::sm::os_mem::{osNumaMask, osNumaNodes};
use crate::stm::stmPreGCHook;
use crate::task::{InCall, Task};
use crate::task::{InCall, Task};
use crate::trace::{
    CAPSET_CLOCKDOMAIN_DEFAULT, CAPSET_OSPROCESS_DEFAULT, CapsetType, CapsetTypeClockdomain,
    CapsetTypeOsProcess, traceCapCreate, traceCapDelete, traceCapsetAssignCap, traceCapsetCreate,
    traceCapsetDelete, traceCapsetRemoveCap,
};

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib, testsuite)]
pub type Capability = Capability_;

/// cbindgen:no-export
pub(crate) struct Capability_ {
    pub(crate) f: StgFunTable,
    pub(crate) r: StgRegTable,
    pub(crate) no: uint32_t,
    pub(crate) node: uint32_t,
    pub(crate) running_task: *mut Task,
    pub(crate) in_haskell: bool,
    pub(crate) idle: uint32_t,
    pub(crate) disabled: bool,
    pub(crate) run_queue_hd: *mut StgTSO,
    pub(crate) run_queue_tl: *mut StgTSO,
    pub(crate) n_run_queue: uint32_t,
    pub(crate) suspended_ccalls: *mut InCall,
    pub(crate) n_suspended_ccalls: uint32_t,
    pub(crate) mut_lists: *mut *mut bdescr,
    pub(crate) saved_mut_lists: *mut *mut bdescr,
    pub(crate) upd_rem_set: UpdRemSet,
    pub(crate) current_segments: *mut *mut NonmovingSegment,
    pub(crate) pinned_object_block: *mut bdescr,
    pub(crate) pinned_object_blocks: *mut bdescr,
    pub(crate) pinned_object_empty: *mut bdescr,
    pub(crate) weak_ptr_list_hd: *mut StgWeak,
    pub(crate) weak_ptr_list_tl: *mut StgWeak,
    pub(crate) context_switch: c_int,
    pub(crate) interrupt: c_int,
    pub(crate) total_allocated: uint64_t,
    pub(crate) iomgr: *mut CapIOManager,
    pub(crate) free_tvar_watch_queues: *mut StgTVarWatchQueue,
    pub(crate) free_trec_chunks: *mut StgTRecChunk,
    pub(crate) free_trec_headers: *mut StgTRecHeader,
    pub(crate) transaction_tokens: uint32_t,
}

pub(crate) type CapIOManager = _CapIOManager;

pub(crate) type SyncType = c_uint;

pub(crate) const SYNC_FLUSH_EVENT_LOG: SyncType = 4;

pub(crate) const SYNC_FLUSH_UPD_REM_SET: SyncType = 3;

pub(crate) const SYNC_GC_PAR: SyncType = 2;

pub(crate) const SYNC_GC_SEQ: SyncType = 1;

pub(crate) const SYNC_OTHER: SyncType = 0;

/// cbindgen:no-export
pub(crate) struct PendingSync {
    pub(crate) r#type: SyncType,
    pub(crate) idle: *mut bool,
    pub(crate) task: *mut Task,
}

#[inline]
pub(crate) unsafe fn regTableToCapability(mut reg: *mut StgRegTable) -> *mut Capability {
    return (reg as *mut c_uchar).offset(-(24 as c_ulong as isize)) as *mut c_void
        as *mut Capability;
}

#[inline]
pub(crate) unsafe fn releaseCapability(mut cap: *mut Capability) {}

#[inline]
pub(crate) unsafe fn releaseAndWakeupCapability(mut cap: *mut Capability) {}

#[inline]
pub(crate) unsafe fn releaseCapability_(mut cap: *mut Capability, mut always_wakeup: bool) {}

#[inline]
pub(crate) unsafe fn getCapability(mut i: uint32_t) -> *mut Capability {
    return *capabilities.offset(i as isize);
}

#[inline]
pub(crate) unsafe fn recordMutableCap(
    mut p: *const StgClosure,
    mut cap: *mut Capability,
    mut r#gen: uint32_t,
) {
    let mut bd = null_mut::<bdescr>();
    bd = *(*cap).mut_lists.offset(r#gen as isize);

    if (*bd).c2rust_unnamed.free >= (*bd).start.offset(BLOCK_SIZE_W as isize) {
        let mut new_bd = null_mut::<bdescr>();
        new_bd = allocBlockOnNode_lock((*cap).node);
        (*new_bd).link = bd as *mut bdescr_;
        (*new_bd).c2rust_unnamed.free = (*new_bd).start;
        bd = new_bd;

        let ref mut fresh8 = *(*cap).mut_lists.offset(r#gen as isize);
        *fresh8 = bd;
    }

    *(*bd).c2rust_unnamed.free = p as StgWord;
    (*bd).c2rust_unnamed.free = (*bd).c2rust_unnamed.free.offset(1 as c_int as isize);
}

#[inline]
pub(crate) unsafe fn recordClosureMutated(mut cap: *mut Capability, mut p: *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    bd = Bdescr(p as StgPtr);

    if (*bd).gen_no as c_int != 0 as c_int {
        recordMutableCap(p, cap, (*bd).gen_no as uint32_t);
    }
}

#[inline]
pub(crate) unsafe fn stopCapability(mut cap: *mut Capability) {
    (&raw mut (*cap).r.rHpLim).store(null_mut::<StgWord>(), Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn interruptCapability(mut cap: *mut Capability) {
    stopCapability(cap);
    (&raw mut (*cap).interrupt).store(1 as c_int, Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn contextSwitchCapability(mut cap: *mut Capability, mut immediately: bool) {
    if immediately {
        stopCapability(cap);
    }

    (&raw mut (*cap).context_switch).store(1 as c_int, Ordering::Relaxed);
}

extern "C" {
    pub(crate) type _CapIOManager;
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut MainCapability: Capability = Capability_ {
    f: StgFunTable {
        stgEagerBlackholeInfo: 0,
        stgGCEnter1: None,
        stgGCFun: None,
    },
    r: StgRegTable {
        rR1: StgUnion { w: 0 },
        rR2: StgUnion { w: 0 },
        rR3: StgUnion { w: 0 },
        rR4: StgUnion { w: 0 },
        rR5: StgUnion { w: 0 },
        rR6: StgUnion { w: 0 },
        rR7: StgUnion { w: 0 },
        rR8: StgUnion { w: 0 },
        rR9: StgUnion { w: 0 },
        rR10: StgUnion { w: 0 },
        rF1: 0.,
        rF2: 0.,
        rF3: 0.,
        rF4: 0.,
        rF5: 0.,
        rF6: 0.,
        rD1: 0.,
        rD2: 0.,
        rD3: 0.,
        rD4: 0.,
        rD5: 0.,
        rD6: 0.,
        rXMM1: StgWord128 { h: 0, l: 0 },
        rXMM2: StgWord128 { h: 0, l: 0 },
        rXMM3: StgWord128 { h: 0, l: 0 },
        rXMM4: StgWord128 { h: 0, l: 0 },
        rXMM5: StgWord128 { h: 0, l: 0 },
        rXMM6: StgWord128 { h: 0, l: 0 },
        rYMM1: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rYMM2: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rYMM3: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rYMM4: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rYMM5: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rYMM6: StgWord256 {
            h: StgWord128 { h: 0, l: 0 },
            l: StgWord128 { h: 0, l: 0 },
        },
        rZMM1: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rZMM2: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rZMM3: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rZMM4: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rZMM5: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rZMM6: StgWord512 {
            h: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
            l: StgWord256 {
                h: StgWord128 { h: 0, l: 0 },
                l: StgWord128 { h: 0, l: 0 },
            },
        },
        rL1: 0,
        rSp: null::<StgWord>() as *mut StgWord,
        rSpLim: null::<StgWord>() as *mut StgWord,
        rHp: null::<StgWord>() as *mut StgWord,
        rHpLim: null::<StgWord>() as *mut StgWord,
        rCCCS: null::<CostCentreStack_>() as *mut CostCentreStack_,
        rCurrentTSO: null::<StgTSO_>() as *mut StgTSO_,
        rNursery: null::<nursery_>() as *mut nursery_,
        rCurrentNursery: null::<bdescr_>() as *mut bdescr_,
        rCurrentAlloc: null::<bdescr_>() as *mut bdescr_,
        rHpAlloc: 0,
        rRet: 0,
    },
    no: 0,
    node: 0,
    running_task: null::<Task>() as *mut Task,
    in_haskell: false,
    idle: 0,
    disabled: false,
    run_queue_hd: null::<StgTSO>() as *mut StgTSO,
    run_queue_tl: null::<StgTSO>() as *mut StgTSO,
    n_run_queue: 0,
    suspended_ccalls: null::<InCall>() as *mut InCall,
    n_suspended_ccalls: 0,
    mut_lists: null::<*mut bdescr>() as *mut *mut bdescr,
    saved_mut_lists: null::<*mut bdescr>() as *mut *mut bdescr,
    upd_rem_set: UpdRemSet {
        queue: MarkQueue_ {
            blocks: null::<bdescr>() as *mut bdescr,
            top: null::<MarkQueueBlock>() as *mut MarkQueueBlock,
            is_upd_rem_set: false,
            prefetch_queue: [MarkQueueEnt {
                c2rust_unnamed: C2RustUnnamed_2 {
                    null_entry: C2RustUnnamed_5 {
                        p: null::<c_void>() as *mut c_void,
                    },
                },
            }; 5],
            prefetch_head: 0,
        },
    },
    current_segments: null::<*mut NonmovingSegment>() as *mut *mut NonmovingSegment,
    pinned_object_block: null::<bdescr>() as *mut bdescr,
    pinned_object_blocks: null::<bdescr>() as *mut bdescr,
    pinned_object_empty: null::<bdescr>() as *mut bdescr,
    weak_ptr_list_hd: null::<StgWeak>() as *mut StgWeak,
    weak_ptr_list_tl: null::<StgWeak>() as *mut StgWeak,
    context_switch: 0,
    interrupt: 0,
    total_allocated: 0,
    iomgr: null::<CapIOManager>() as *mut CapIOManager,
    free_tvar_watch_queues: null::<StgTVarWatchQueue>() as *mut StgTVarWatchQueue,
    free_trec_chunks: null::<StgTRecChunk>() as *mut StgTRecChunk,
    free_trec_headers: null::<StgTRecHeader>() as *mut StgTRecHeader,
    transaction_tokens: 0,
};

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut n_capabilities: uint32_t = 0 as uint32_t;

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub static mut enabled_capabilities: uint32_t = 0 as uint32_t;

static mut max_n_capabilities: uint32_t = MAX_N_CAPABILITIES as uint32_t;

static mut capabilities: *mut *mut Capability = null::<*mut Capability>() as *mut *mut Capability;

static mut last_free_capability: [*mut Capability; 16] =
    [null::<Capability>() as *mut Capability; 16];

static mut pending_sync: *mut PendingSync = null::<PendingSync>() as *mut PendingSync;

static mut n_numa_nodes: uint32_t = 0;

static mut numa_map: [uint32_t; 16] = [0; 16];

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    return &raw mut MainCapability;
}

unsafe fn initCapability(mut cap: *mut Capability, mut i: uint32_t) {
    let mut g: uint32_t = 0;
    (*cap).no = i;
    (*cap).node = i.wrapping_rem(n_numa_nodes);
    (*cap).in_haskell = r#false != 0;
    (*cap).idle = 0 as uint32_t;
    (*cap).disabled = r#false != 0;
    (*cap).run_queue_hd = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).run_queue_tl = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).n_run_queue = 0 as uint32_t;
    (*cap).total_allocated = 0 as uint64_t;
    initCapabilityIOManager(cap);
    (*cap).f.stgEagerBlackholeInfo = &raw const __stg_EAGER_BLACKHOLE_info as W_ as StgWord;

    (*cap).f.stgGCEnter1 = transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(
        Some(__stg_gc_enter_1 as unsafe extern "C" fn() -> StgFunPtr),
    );

    (*cap).f.stgGCFun = transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(Some(
        __stg_gc_fun as unsafe extern "C" fn() -> StgFunPtr,
    ));

    (*cap).mut_lists = stgMallocBytes(
        (size_of::<*mut bdescr>() as size_t).wrapping_mul(RtsFlags.GcFlags.generations as size_t),
        b"initCapability\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut bdescr;

    (*cap).saved_mut_lists = stgMallocBytes(
        (size_of::<*mut bdescr>() as size_t).wrapping_mul(RtsFlags.GcFlags.generations as size_t),
        b"initCapability\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut bdescr;

    (*cap).current_segments = null_mut::<*mut NonmovingSegment>();
    (*cap).upd_rem_set.queue.blocks = null_mut::<bdescr>();
    g = 0 as uint32_t;

    while g < RtsFlags.GcFlags.generations {
        let ref mut fresh7 = *(*cap).mut_lists.offset(g as isize);
        *fresh7 = null_mut::<bdescr>();
        g = g.wrapping_add(1);
    }

    (*cap).weak_ptr_list_hd = null_mut::<StgWeak>();
    (*cap).weak_ptr_list_tl = null_mut::<StgWeak>();
    (*cap).free_tvar_watch_queues =
        &raw mut stg_END_STM_WATCH_QUEUE_closure as *mut c_void as *mut StgTVarWatchQueue;
    (*cap).free_trec_chunks =
        &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk;
    (*cap).free_trec_headers = &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader;
    (*cap).transaction_tokens = 0 as uint32_t;
    (*cap).context_switch = 0 as c_int;
    (*cap).interrupt = 0 as c_int;
    (*cap).pinned_object_block = null_mut::<bdescr>();
    (*cap).pinned_object_blocks = null_mut::<bdescr>();
    (*cap).pinned_object_empty = null_mut::<bdescr>();
    (*cap).r.rCCCS = null_mut::<CostCentreStack_>();
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
    traceCapCreate(cap);
    traceCapsetAssignCap(CAPSET_OSPROCESS_DEFAULT, i);
    traceCapsetAssignCap(CAPSET_CLOCKDOMAIN_DEFAULT, i);
}

unsafe fn initCapabilities() {
    traceCapsetCreate(
        CAPSET_OSPROCESS_DEFAULT,
        CapsetTypeOsProcess as c_int as CapsetType,
    );

    traceCapsetCreate(
        CAPSET_CLOCKDOMAIN_DEFAULT,
        CapsetTypeClockdomain as c_int as CapsetType,
    );

    if !RtsFlags.GcFlags.numa {
        n_numa_nodes = 1 as uint32_t;

        let mut i: uint32_t = 0 as uint32_t;

        while i < MAX_NUMA_NODES as uint32_t {
            numa_map[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
    } else if !RtsFlags.DebugFlags.numa {
        let mut nNodes = osNumaNodes();

        if nNodes > MAX_NUMA_NODES as uint32_t {
            barf(
                b"Too many NUMA nodes (max %d)\0" as *const u8 as *const c_char,
                MAX_NUMA_NODES,
            );
        }

        let mut mask: StgWord = RtsFlags.GcFlags.numaMask & osNumaMask() as StgWord;
        let mut logical: uint32_t = 0 as uint32_t;
        let mut physical: uint32_t = 0 as uint32_t;

        while physical < MAX_NUMA_NODES as uint32_t {
            if mask & 1 as StgWord != 0 {
                let fresh5 = logical;
                logical = logical.wrapping_add(1);
                numa_map[fresh5 as usize] = physical;
            }

            mask = mask >> 1 as c_int;
            physical = physical.wrapping_add(1);
        }

        n_numa_nodes = logical;

        if logical == 0 as uint32_t {
            barf(b"available NUMA node set is empty\0" as *const u8 as *const c_char);
        }
    }

    n_capabilities = 1 as uint32_t;

    capabilities = stgMallocBytes(
        size_of::<Capability>() as size_t,
        b"initCapabilities\0" as *const u8 as *const c_char as *mut c_char,
    ) as *mut *mut Capability;

    let ref mut fresh6 = *capabilities.offset(0 as c_int as isize);
    *fresh6 = &raw mut MainCapability;
    initCapability(&raw mut MainCapability, 0 as uint32_t);
    enabled_capabilities = n_capabilities;

    let mut i_0: uint32_t = 0 as uint32_t;

    while i_0 < n_numa_nodes {
        last_free_capability[i_0 as usize] = getCapability(0 as uint32_t);
        i_0 = i_0.wrapping_add(1);
    }
}

unsafe fn moreCapabilities(mut from: uint32_t, mut to: uint32_t) {}

unsafe fn contextSwitchAllCapabilities() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
        contextSwitchCapability(getCapability(i), r#true != 0);
        i = i.wrapping_add(1);
    }
}

unsafe fn interruptAllCapabilities() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
        interruptCapability(getCapability(i));
        i = i.wrapping_add(1);
    }
}

unsafe fn waitForCapability(mut pCap: *mut *mut Capability, mut task: *mut Task) {
    MainCapability.running_task = task;
    (*task).cap = &raw mut MainCapability as *mut Capability_;
    *pCap = &raw mut MainCapability;
}

unsafe fn shutdownCapability(mut cap: *mut Capability, mut task: *mut Task, mut safe: bool) {}

unsafe fn shutdownCapabilities(mut task: *mut Task, mut safe: bool) {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;

    while i < getNumCapabilities() as uint32_t {
        shutdownCapability(getCapability(i), task, safe);
        i = i.wrapping_add(1);
    }
}

unsafe fn freeCapability(mut cap: *mut Capability) {
    stgFree((*cap).mut_lists as *mut c_void);
    stgFree((*cap).saved_mut_lists as *mut c_void);

    if !(*cap).current_segments.is_null() {
        stgFree((*cap).current_segments as *mut c_void);
    }

    traceCapsetRemoveCap(CAPSET_OSPROCESS_DEFAULT, (*cap).no);
    traceCapsetRemoveCap(CAPSET_CLOCKDOMAIN_DEFAULT, (*cap).no);
    traceCapDelete(cap);
}

unsafe fn freeCapabilities() {
    freeCapability(&raw mut MainCapability);
    traceCapsetDelete(CAPSET_OSPROCESS_DEFAULT);
    traceCapsetDelete(CAPSET_CLOCKDOMAIN_DEFAULT);
}

unsafe fn markCapability(
    mut evac: evac_fn,
    mut user: *mut c_void,
    mut cap: *mut Capability,
    mut no_mark_sparks: bool,
) {
    let mut incall = null_mut::<InCall>();

    evac.expect("non-null function pointer")(
        user,
        &raw mut (*cap).run_queue_hd as *mut c_void as *mut *mut StgClosure,
    );

    evac.expect("non-null function pointer")(
        user,
        &raw mut (*cap).run_queue_tl as *mut c_void as *mut *mut StgClosure,
    );

    incall = (*cap).suspended_ccalls;

    while !incall.is_null() {
        evac.expect("non-null function pointer")(
            user,
            &raw mut (*incall).suspended_tso as *mut c_void as *mut *mut StgClosure,
        );

        incall = (*incall).next as *mut InCall;
    }

    markCapabilityIOManager(evac, user, cap);
    stmPreGCHook(cap);
}

unsafe fn markCapabilities(mut evac: evac_fn, mut user: *mut c_void) {
    let mut n: uint32_t = 0;
    n = 0 as uint32_t;

    while n < getNumCapabilities() as uint32_t {
        markCapability(evac, user, getCapability(n), r#false != 0);
        n = n.wrapping_add(1);
    }
}
