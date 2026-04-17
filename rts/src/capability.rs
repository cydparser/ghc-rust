use crate::ffi::rts::_assertFail;
use crate::ffi::rts::config::{CACHELINE_SIZE, MAX_N_CAPABILITIES};
use crate::ffi::rts::constants::{MAX_NUMA_NODES, MAX_SPARE_WORKERS};
use crate::ffi::rts::os_threads::{
    Mutex, OS_TRY_ACQUIRE_LOCK, getNumberOfProcessors, initMutex, osThreadId, osThreadIsAlive,
    shutdownThread, signalCondition, waitCondition, yieldThread,
};
use crate::ffi::rts::prof::ccs::{CCS_IDLE, CCS_SYSTEM, CostCentreStack, CostCentreStack_};
use crate::ffi::rts::storage::block::{
    BLOCK_SIZE_W, Bdescr, allocBlockOnNode_lock, bdescr, bdescr_,
};
use crate::ffi::rts::storage::closures::{
    Message, StgClosurePtr, StgTRecChunk, StgTRecHeader, StgTVarWatchQueue, StgWeak,
};
use crate::ffi::rts::storage::gc::nursery_;
use crate::ffi::rts::storage::tso::StgTSO_;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::timer::{startTimer, stopTimer};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts_messages::barf;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, __stg_gc_enter_1, __stg_gc_fun, stg_END_STM_CHUNK_LIST_closure,
    stg_END_STM_WATCH_QUEUE_closure, stg_END_TSO_QUEUE_closure, stg_NO_TREC_closure,
};
use crate::ffi::stg::regs::{StgFunTable, StgRegTable, StgUnion};
use crate::ffi::stg::types::{
    StgFunPtr, StgPtr, StgStablePtr, StgWord, StgWord64, StgWord128, StgWord256, StgWord512,
};
use crate::io_manager::{initCapabilityIOManager, markCapabilityIOManager, stopIOManager};
use crate::prelude::*;
use crate::rts_flags::RtsFlags;
use crate::rts_utils::{stgFree, stgFreeAligned, stgMallocAlignedBytes, stgMallocBytes};
use crate::schedule::{
    ACTIVITY_INACTIVE, SCHED_INTERRUPTING, SCHED_SHUTTING_DOWN, emptyRunQueue, getRecentActivity,
    getSchedState, peekRunQueue,
};
use crate::sm::gc::{evac_fn, gcWorkerThread};
use crate::sm::non_moving::NonmovingSegment;
use crate::sm::non_moving_mark::{
    C2RustUnnamed_3, C2RustUnnamed_6, MarkQueue_, MarkQueueBlock, MarkQueueEnt, UpdRemSet,
};
use crate::sm::os_mem::{osNumaMask, osNumaNodes};
use crate::sparks::{
    SparkCounters, SparkPool, allocSparkPool, discardSparks, fizzledSpark, freeSparkPool,
    looksEmpty, sparkPoolSize, traverseSparkQueue, tryStealSpark,
};
use crate::stm::stmPreGCHook;
use crate::task::{
    InCall, Task, Task_, isBoundTask, isWorker, myTask, serialisableTaskId, startWorkerTask,
    workerTaskStop,
};
use crate::trace::{
    CAPSET_CLOCKDOMAIN_DEFAULT, CAPSET_OSPROCESS_DEFAULT, CapsetType, CapsetTypeClockdomain,
    CapsetTypeOsProcess, DEBUG_RTS, trace_, traceCapCreate, traceCapDelete, traceCapsetAssignCap,
    traceCapsetCreate, traceCapsetDelete, traceCapsetRemoveCap, traceEventGcEnd, traceEventGcStart,
    traceEventSparkFizzle, traceEventSparkRun, traceEventSparkSteal, traceSparkCounters,
};

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib, testsuite)]
pub type Capability = Capability_;

/// cbindgen:no-export
pub(crate) struct Capability_ {
    pub(crate) f: StgFunTable,
    pub(crate) r: StgRegTable,
    pub(crate) no: u32,
    pub(crate) node: u32,
    pub(crate) running_task: *mut Task,
    pub(crate) in_haskell: bool,
    pub(crate) idle: u32,
    pub(crate) disabled: bool,
    pub(crate) run_queue_hd: *mut StgTSO,
    pub(crate) run_queue_tl: *mut StgTSO,
    pub(crate) n_run_queue: u32,
    pub(crate) suspended_ccalls: *mut InCall,
    pub(crate) n_suspended_ccalls: u32,
    pub(crate) mut_lists: *mut *mut bdescr,
    pub(crate) saved_mut_lists: *mut *mut bdescr,
    pub(crate) upd_rem_set: UpdRemSet,
    pub(crate) current_segments: *mut *mut NonmovingSegment,
    pub(crate) pinned_object_block: *mut bdescr,
    pub(crate) pinned_object_blocks: *mut bdescr,
    pub(crate) pinned_object_empty: *mut bdescr,
    pub(crate) weak_ptr_list_hd: *mut StgWeak,
    pub(crate) weak_ptr_list_tl: *mut StgWeak,
    pub(crate) context_switch: i32,
    pub(crate) interrupt: i32,
    pub(crate) total_allocated: u64,
    pub(crate) spare_workers: *mut Task,
    pub(crate) n_spare_workers: u32,
    pub(crate) lock: Mutex,
    pub(crate) returning_tasks_hd: *mut Task,
    pub(crate) returning_tasks_tl: *mut Task,
    pub(crate) n_returning_tasks: u32,
    pub(crate) inbox: *mut Message,
    pub(crate) putMVars: *mut PutMVar_,
    pub(crate) sparks: *mut SparkPool,
    pub(crate) spark_stats: SparkCounters,
    pub(crate) iomgr: *mut CapIOManager,
    pub(crate) free_tvar_watch_queues: *mut StgTVarWatchQueue,
    pub(crate) free_trec_chunks: *mut StgTRecChunk,
    pub(crate) free_trec_headers: *mut StgTRecHeader,
    pub(crate) transaction_tokens: u32,
}

pub(crate) type CapIOManager = _CapIOManager;

/// cbindgen:no-export
pub(crate) struct PutMVar_ {
    pub(crate) mvar: StgStablePtr,
    pub(crate) link: *mut PutMVar_,
}

/// cbindgen:no-export
pub(crate) struct PendingSync {
    pub(crate) r#type: SyncType,
    pub(crate) idle: *mut bool,
    pub(crate) task: *mut Task,
}

pub(crate) type SyncType = u32;

pub(crate) const SYNC_FLUSH_EVENT_LOG: SyncType = 4;

pub(crate) const SYNC_FLUSH_UPD_REM_SET: SyncType = 3;

pub(crate) const SYNC_GC_PAR: SyncType = 2;

pub(crate) const SYNC_GC_SEQ: SyncType = 1;

pub(crate) const SYNC_OTHER: SyncType = 0;

pub(crate) const CAPABILITY_ALIGNMENT: i32 = CACHELINE_SIZE;

#[inline]
pub(crate) unsafe fn regTableToCapability(mut reg: *mut StgRegTable) -> *mut Capability {
    return (reg as *mut u8).offset(-24) as *mut c_void as *mut Capability;
}

#[inline]
pub(crate) unsafe fn getCapability(mut i: u32) -> *mut Capability {
    return (capabilities.offset(i as isize) as *mut *mut Capability).load(Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn recordMutableCap(
    mut p: *const StgClosure,
    mut cap: *mut Capability,
    mut r#gen: u32,
) {
    let mut bd = null_mut::<bdescr>();
    bd = *(*cap).mut_lists.offset(r#gen as isize);

    if (&raw mut (*bd).c2rust_unnamed.free).load(Ordering::Relaxed)
        >= (*bd).start.offset(BLOCK_SIZE_W as isize)
    {
        let mut new_bd = null_mut::<bdescr>();
        new_bd = allocBlockOnNode_lock((*cap).node);
        (*new_bd).link = bd as *mut bdescr_;
        (*new_bd).c2rust_unnamed.free = (*new_bd).start;
        bd = new_bd;

        let ref mut fresh20 = *(*cap).mut_lists.offset(r#gen as isize);
        *fresh20 = bd;
    }

    ((*bd).c2rust_unnamed.free).store(p as StgWord, Ordering::Relaxed);
    (&raw mut (*bd).c2rust_unnamed.free).store(
        (&raw mut (*bd).c2rust_unnamed.free)
            .load(Ordering::Relaxed)
            .offset(1),
        Ordering::Relaxed,
    );
}

#[inline]
pub(crate) unsafe fn recordClosureMutated(mut cap: *mut Capability, mut p: *mut StgClosure) {
    let mut bd = null_mut::<bdescr>();
    bd = Bdescr(p as StgPtr);

    if (*bd).gen_no as i32 != 0 {
        recordMutableCap(p, cap, (*bd).gen_no as u32);
    }
}

#[inline]
pub(crate) unsafe fn emptySparkPoolCap(mut cap: *mut Capability) -> bool {
    return looksEmpty((*cap).sparks);
}

#[inline]
pub(crate) unsafe fn sparkPoolSizeCap(mut cap: *mut Capability) -> u32 {
    return sparkPoolSize((*cap).sparks) as u32;
}

#[inline]
pub(crate) unsafe fn discardSparksCap(mut cap: *mut Capability) {
    discardSparks((*cap).sparks);
}

#[inline]
pub(crate) unsafe fn stopCapability(mut cap: *mut Capability) {
    (&raw mut (*cap).r.rHpLim).store(null_mut::<StgWord>(), Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn interruptCapability(mut cap: *mut Capability) {
    stopCapability(cap);
    (&raw mut (*cap).interrupt).store(1, Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn contextSwitchCapability(mut cap: *mut Capability, mut immediately: bool) {
    if immediately {
        stopCapability(cap);
    }

    (&raw mut (*cap).context_switch).store(1, Ordering::Relaxed);
}

#[inline]
pub(crate) unsafe fn emptyInbox(mut cap: *mut Capability) -> bool {
    return (&raw mut (*cap).inbox).load(Ordering::Relaxed)
        == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut Message
        && (&raw mut (*cap).putMVars).load(Ordering::Relaxed).is_null();
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
        rSp: null_mut::<StgWord>(),
        rSpLim: null_mut::<StgWord>(),
        rHp: null_mut::<StgWord>(),
        rHpLim: null_mut::<StgWord>(),
        rCCCS: null_mut::<CostCentreStack_>(),
        rCurrentTSO: null_mut::<StgTSO_>(),
        rNursery: null_mut::<nursery_>(),
        rCurrentNursery: null_mut::<bdescr_>(),
        rCurrentAlloc: null_mut::<bdescr_>(),
        rHpAlloc: 0,
        rRet: 0,
    },
    no: 0,
    node: 0,
    running_task: null_mut::<Task>(),
    in_haskell: false,
    idle: 0,
    disabled: false,
    run_queue_hd: null_mut::<StgTSO>(),
    run_queue_tl: null_mut::<StgTSO>(),
    n_run_queue: 0,
    suspended_ccalls: null_mut::<InCall>(),
    n_suspended_ccalls: 0,
    mut_lists: null_mut::<*mut bdescr>(),
    saved_mut_lists: null_mut::<*mut bdescr>(),
    upd_rem_set: UpdRemSet {
        queue: MarkQueue_ {
            blocks: null_mut::<bdescr>(),
            top: null_mut::<MarkQueueBlock>(),
            is_upd_rem_set: false,
            prefetch_queue: [MarkQueueEnt {
                c2rust_unnamed: C2RustUnnamed_3 {
                    null_entry: C2RustUnnamed_6 {
                        p: null_mut::<c_void>(),
                    },
                },
            }; 5],
            prefetch_head: 0,
        },
    },
    current_segments: null_mut::<*mut NonmovingSegment>(),
    pinned_object_block: null_mut::<bdescr>(),
    pinned_object_blocks: null_mut::<bdescr>(),
    pinned_object_empty: null_mut::<bdescr>(),
    weak_ptr_list_hd: null_mut::<StgWeak>(),
    weak_ptr_list_tl: null_mut::<StgWeak>(),
    context_switch: 0,
    interrupt: 0,
    total_allocated: 0,
    spare_workers: null_mut::<Task>(),
    n_spare_workers: 0,
    lock: _opaque_pthread_mutex_t {
        __sig: 0,
        __opaque: [0; 56],
    },
    returning_tasks_hd: null_mut::<Task>(),
    returning_tasks_tl: null_mut::<Task>(),
    n_returning_tasks: 0,
    inbox: null_mut::<Message>(),
    putMVars: null_mut::<PutMVar_>(),
    sparks: null_mut::<SparkPool>(),
    spark_stats: SparkCounters {
        created: 0,
        dud: 0,
        overflowed: 0,
        converted: 0,
        gcd: 0,
        fizzled: 0,
    },
    iomgr: null_mut::<CapIOManager>(),
    free_tvar_watch_queues: null_mut::<StgTVarWatchQueue>(),
    free_trec_chunks: null_mut::<StgTRecChunk>(),
    free_trec_headers: null_mut::<StgTRecHeader>(),
    transaction_tokens: 0,
};

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut n_capabilities: c_uint = 0;

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
pub static mut enabled_capabilities: c_uint = 0;

static mut max_n_capabilities: u32 = MAX_N_CAPABILITIES as u32;

static mut capabilities: *mut *mut Capability = null_mut::<*mut Capability>();

static mut last_free_capability: [*mut Capability; 16] = [null_mut::<Capability>(); 16];

static mut pending_sync: *mut PendingSync = null_mut::<PendingSync>();

static mut n_numa_nodes: u32 = 0;

static mut numa_map: [u32; 16] = [0; 16];

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unsafeGetMyCapability() -> *mut Capability {
    return (*myTask()).cap as *mut Capability;
}

unsafe fn globalWorkToDo() -> bool {
    return getSchedState() as u32 >= SCHED_INTERRUPTING as i32 as u32
        || getRecentActivity() as u32 == ACTIVITY_INACTIVE as i32 as u32;
}

unsafe fn findSpark(mut cap: *mut Capability) -> *mut StgClosure {
    let mut robbed = null_mut::<Capability>();
    let mut spark = null_mut::<StgClosure_>();
    let mut retry: bool = false;
    let mut i: u32 = 0;

    if !emptyRunQueue(cap) || (&raw mut (*cap).n_returning_tasks).load(Ordering::Relaxed) != 0 {
        return null_mut::<StgClosure>();
    }

    loop {
        retry = false;
        spark = tryStealSpark((*cap).sparks) as StgClosurePtr;

        while !spark.is_null() && fizzledSpark(spark as *mut StgClosure) as i32 != 0 {
            (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
            traceEventSparkFizzle(cap);
            spark = tryStealSpark((*cap).sparks) as StgClosurePtr;
        }

        if !spark.is_null() {
            (*cap).spark_stats.converted = (*cap).spark_stats.converted.wrapping_add(1);
            traceEventSparkRun(cap);

            return spark as *mut StgClosure;
        }

        if !emptySparkPoolCap(cap) {
            retry = true;
        }

        if getNumCapabilities() == 1 {
            return null_mut::<StgClosure>();
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"cap %d: Trying to steal work from other capabilities".as_ptr(),
                (*cap).no,
            );
        }

        i = 0;

        while i < getNumCapabilities() as u32 {
            robbed = getCapability(i);

            if !(cap == robbed) {
                if !emptySparkPoolCap(robbed) {
                    spark = tryStealSpark((*robbed).sparks) as StgClosurePtr;

                    while !spark.is_null() && fizzledSpark(spark as *mut StgClosure) as i32 != 0 {
                        (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
                        traceEventSparkFizzle(cap);
                        spark = tryStealSpark((*robbed).sparks) as StgClosurePtr;
                    }

                    if spark.is_null() && !emptySparkPoolCap(robbed) {
                        retry = true;
                    }

                    if !spark.is_null() {
                        (*cap).spark_stats.converted = (*cap).spark_stats.converted.wrapping_add(1);
                        traceEventSparkSteal(cap, (*robbed).no);

                        return spark as *mut StgClosure;
                    }
                }
            }

            i = i.wrapping_add(1);
        }

        if !retry {
            break;
        }
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"No sparks stolen".as_ptr());
    }

    return null_mut::<StgClosure>();
}

unsafe fn anySparks() -> bool {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        if !emptySparkPoolCap(getCapability(i)) {
            return true;
        }

        i = i.wrapping_add(1);
    }

    return false;
}

unsafe fn newReturningTask(mut cap: *mut Capability, mut task: *mut Task) {
    if (pthread_mutex_lock(&raw mut (*cap).lock) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 209);
    }

    if (*task).next.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 210);
    }

    if !(*cap).returning_tasks_hd.is_null() {
        if (*(*cap).returning_tasks_tl).next.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Capability.c".as_ptr(), 212);
        }

        (*(*cap).returning_tasks_tl).next = task as *mut Task_;
    } else {
        (*cap).returning_tasks_hd = task;
    }

    (*cap).returning_tasks_tl = task;

    let fresh18 = &raw mut (*cap).n_returning_tasks;
    let fresh19 = 1;
    (fresh18).xadd(fresh19, Ordering::Relaxed) + fresh19;

    if (if (*cap).returning_tasks_hd.is_null() {
        ((*cap).returning_tasks_tl.is_null() && (*cap).n_returning_tasks == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 222);
    };
}

unsafe fn popReturningTask(mut cap: *mut Capability) -> *mut Task {
    if (pthread_mutex_lock(&raw mut (*cap).lock) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 228);
    }

    let mut task = null_mut::<Task>();
    task = (*cap).returning_tasks_hd;

    if !task.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 231);
    }

    (*cap).returning_tasks_hd = (*task).next as *mut Task;

    if (*cap).returning_tasks_hd.is_null() {
        (*cap).returning_tasks_tl = null_mut::<Task>();
    }

    (*task).next = null_mut::<Task_>();

    let fresh16 = &raw mut (*cap).n_returning_tasks;
    let fresh17 = -1 as u32;
    (fresh16).xadd(fresh17, Ordering::Relaxed) + fresh17;

    if (if (*cap).returning_tasks_hd.is_null() {
        ((*cap).returning_tasks_tl.is_null() && (*cap).n_returning_tasks == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 241);
    }

    return task;
}

unsafe fn initCapability(mut cap: *mut Capability, mut i: u32) {
    let mut g: u32 = 0;
    (*cap).no = i;
    (*cap).node = i.wrapping_rem(n_numa_nodes);
    (*cap).in_haskell = false;
    (*cap).idle = 0;
    (*cap).disabled = false;
    (*cap).run_queue_hd = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).run_queue_tl = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO;
    (*cap).n_run_queue = 0;
    initMutex(&raw mut (*cap).lock);
    (*cap).running_task = null_mut::<Task>();
    (*cap).spare_workers = null_mut::<Task>();
    (*cap).n_spare_workers = 0;
    (*cap).suspended_ccalls = null_mut::<InCall>();
    (*cap).n_suspended_ccalls = 0;
    (*cap).returning_tasks_hd = null_mut::<Task>();
    (*cap).returning_tasks_tl = null_mut::<Task>();
    (*cap).n_returning_tasks = 0;
    (*cap).inbox = &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO as *mut Message;
    (*cap).putMVars = null_mut::<PutMVar_>();
    (*cap).sparks = allocSparkPool();
    (*cap).spark_stats.created = 0;
    (*cap).spark_stats.dud = 0;
    (*cap).spark_stats.overflowed = 0;
    (*cap).spark_stats.converted = 0;
    (*cap).spark_stats.gcd = 0;
    (*cap).spark_stats.fizzled = 0;
    (*cap).total_allocated = 0;
    initCapabilityIOManager(cap);
    (*cap).f.stgEagerBlackholeInfo = &raw const __stg_EAGER_BLACKHOLE_info as W_ as StgWord;

    (*cap).f.stgGCEnter1 = transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(
        Some(__stg_gc_enter_1 as unsafe extern "C" fn() -> StgFunPtr),
    );

    (*cap).f.stgGCFun = transmute::<Option<unsafe extern "C" fn() -> StgFunPtr>, StgFunPtr>(Some(
        __stg_gc_fun as unsafe extern "C" fn() -> StgFunPtr,
    ));

    (*cap).mut_lists = stgMallocBytes(
        (size_of::<*mut bdescr>() as usize).wrapping_mul(RtsFlags.GcFlags.generations as usize),
        c"initCapability".as_ptr(),
    ) as *mut *mut bdescr;

    (*cap).saved_mut_lists = stgMallocBytes(
        (size_of::<*mut bdescr>() as usize).wrapping_mul(RtsFlags.GcFlags.generations as usize),
        c"initCapability".as_ptr(),
    ) as *mut *mut bdescr;

    (*cap).current_segments = null_mut::<*mut NonmovingSegment>();
    (*cap).upd_rem_set.queue.blocks = null_mut::<bdescr>();
    g = 0;

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
    (*cap).transaction_tokens = 0;
    (*cap).context_switch = 0;
    (*cap).interrupt = 0;
    (*cap).pinned_object_block = null_mut::<bdescr>();
    (*cap).pinned_object_blocks = null_mut::<bdescr>();
    (*cap).pinned_object_empty = null_mut::<bdescr>();
    (*cap).r.rCCCS = &raw mut CCS_SYSTEM as *mut CostCentreStack as *mut CostCentreStack_;
    (*cap).r.rCurrentTSO = null_mut::<StgTSO_>();
    traceCapCreate(cap);
    traceCapsetAssignCap(CAPSET_OSPROCESS_DEFAULT, i);
    traceCapsetAssignCap(CAPSET_CLOCKDOMAIN_DEFAULT, i);
    traceSparkCounters(cap);
}

unsafe fn initCapabilities() {
    traceCapsetCreate(
        CAPSET_OSPROCESS_DEFAULT,
        CapsetTypeOsProcess as i32 as CapsetType,
    );

    traceCapsetCreate(
        CAPSET_CLOCKDOMAIN_DEFAULT,
        CapsetTypeClockdomain as i32 as CapsetType,
    );

    if !RtsFlags.GcFlags.numa {
        n_numa_nodes = 1;

        let mut i: u32 = 0;

        while i < MAX_NUMA_NODES as u32 {
            numa_map[i as usize] = 0;
            i = i.wrapping_add(1);
        }
    } else if !RtsFlags.DebugFlags.numa {
        let mut nNodes = osNumaNodes();

        if nNodes > MAX_NUMA_NODES as u32 {
            barf(c"Too many NUMA nodes (max %d)".as_ptr(), MAX_NUMA_NODES);
        }

        let mut mask: StgWord = RtsFlags.GcFlags.numaMask & osNumaMask() as StgWord;
        let mut logical: u32 = 0;
        let mut physical: u32 = 0;

        while physical < MAX_NUMA_NODES as u32 {
            if mask & 1 != 0 {
                let fresh5 = logical;
                logical = logical.wrapping_add(1);
                numa_map[fresh5 as usize] = physical;
            }

            mask = mask >> 1;
            physical = physical.wrapping_add(1);
        }

        n_numa_nodes = logical;

        if logical == 0 {
            barf(c"available NUMA node set is empty".as_ptr());
        }
    }

    let mut core_count = getNumberOfProcessors();

    if core_count > max_n_capabilities {
        max_n_capabilities = core_count;
    }

    if RtsFlags.ParFlags.nCapabilities > max_n_capabilities {
        max_n_capabilities = RtsFlags.ParFlags.nCapabilities;
    }

    capabilities = stgMallocBytes(
        (size_of::<Capability>() as usize).wrapping_mul(max_n_capabilities as usize),
        c"initCapabilities".as_ptr(),
    ) as *mut *mut Capability;

    n_capabilities = 0;
    moreCapabilities(0, RtsFlags.ParFlags.nCapabilities);
    n_capabilities = RtsFlags.ParFlags.nCapabilities;
    enabled_capabilities = n_capabilities;

    let mut i_0: u32 = 0;

    while i_0 < n_numa_nodes {
        last_free_capability[i_0 as usize] = getCapability(0);
        i_0 = i_0.wrapping_add(1);
    }
}

unsafe fn moreCapabilities(mut from: u32, mut to: u32) {
    stopTimer();

    if to == 1 {
        let ref mut fresh13 = *capabilities.offset(0);
        *fresh13 = &raw mut MainCapability;
        initCapability(&raw mut MainCapability, 0);
    } else {
        let mut i: u32 = 0;

        while i < to {
            if i >= from {
                let ref mut fresh14 = *capabilities.offset(i as isize);

                *fresh14 = stgMallocAlignedBytes(
                    size_of::<Capability>() as usize,
                    CAPABILITY_ALIGNMENT as usize,
                    c"moreCapabilities".as_ptr(),
                ) as *mut Capability;

                initCapability(*capabilities.offset(i as isize), i);
            }

            i = i.wrapping_add(1);
        }
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"allocated %d more capabilities".as_ptr(),
            to.wrapping_sub(from),
        );
    }

    startTimer();
}

unsafe fn contextSwitchAllCapabilities() {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        contextSwitchCapability(getCapability(i), true);
        i = i.wrapping_add(1);
    }
}

unsafe fn interruptAllCapabilities() {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        interruptCapability(getCapability(i));
        i = i.wrapping_add(1);
    }
}

unsafe fn giveCapabilityToTask(mut cap: *mut Capability, mut task: *mut Task) {
    if (pthread_mutex_lock(&raw mut (*cap).lock) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 517);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 518);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(
            c"passing capability %d to %s %#llx".as_ptr(),
            (*cap).no,
            if !(*task).incall.is_null() && !(*(*task).incall).tso.is_null() {
                c"bound task".as_ptr()
            } else {
                c"worker".as_ptr()
            },
            serialisableTaskId(task),
        );
    }

    let mut __r = pthread_mutex_lock(&raw mut (*task).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            522,
            __r,
        );
    }

    if (*task).wakeup as i32 == false {
        (*task).wakeup = true;
        signalCondition(&raw mut (*task).cond);
    }

    if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            530,
        );
    }
}

unsafe fn releaseCapability_(mut cap: *mut Capability, mut always_wakeup: bool) {
    let mut task = null_mut::<Task>();
    task = (*cap).running_task;

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 556);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 556);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 556);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 556);
    }

    if (if (*cap).returning_tasks_hd.is_null() {
        ((*cap).returning_tasks_tl.is_null() && (*cap).n_returning_tasks == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 557);
    }

    if (pthread_mutex_lock(&raw mut (*cap).lock) == 11) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 558);
    }

    (&raw mut (*cap).running_task).store(null_mut::<Task>(), Ordering::Relaxed);

    if (*cap).n_returning_tasks != 0 {
        giveCapabilityToTask(cap, (*cap).returning_tasks_hd);
        return;
    }

    let mut sync = (&raw mut pending_sync).load(Ordering::SeqCst);

    if !sync.is_null()
        && ((*sync).r#type as u32 != SYNC_GC_PAR as i32 as u32
            || *(*sync).idle.offset((*cap).no as isize) as i32 != 0)
    {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"sync pending, freeing capability %d".as_ptr(), (*cap).no);
        }

        return;
    }

    if !emptyRunQueue(cap) && !(*peekRunQueue(cap)).bound.is_null() {
        task = (*(*peekRunQueue(cap)).bound).task as *mut Task;
        giveCapabilityToTask(cap, task);
        return;
    }

    if (*cap).spare_workers.is_null() {
        if (getSchedState() as u32) < SCHED_SHUTTING_DOWN as i32 as u32 || !emptyRunQueue(cap) {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(c"starting new worker on capability %d".as_ptr(), (*cap).no);
            }

            startWorkerTask(cap);
            return;
        }
    }

    if always_wakeup as i32 != 0
        || !emptyRunQueue(cap)
        || !emptyInbox(cap)
        || !(*cap).disabled && !emptySparkPoolCap(cap)
        || globalWorkToDo() as i32 != 0
    {
        if !(*cap).spare_workers.is_null() {
            giveCapabilityToTask(cap, (*cap).spare_workers);
            return;
        }
    }

    (*cap).r.rCCCS = &raw mut CCS_IDLE as *mut CostCentreStack as *mut CostCentreStack_;
    ((&raw mut last_free_capability as *mut *mut Capability).offset((*cap).node as isize)
        as *mut *mut Capability)
        .store(cap, Ordering::Relaxed);

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"freeing capability %d".as_ptr(), (*cap).no);
    }
}

unsafe fn releaseCapability(mut cap: *mut Capability) {
    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            633,
            __r,
        );
    }

    releaseCapability_(cap, false);

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            635,
        );
    }
}

unsafe fn releaseAndWakeupCapability(mut cap: *mut Capability) {
    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            641,
            __r,
        );
    }

    releaseCapability_(cap, true);

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            643,
        );
    }
}

unsafe fn enqueueWorker(mut cap: *mut Capability) {
    let mut task = null_mut::<Task>();
    task = (*cap).running_task;

    if !(*task).stopped as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 655);
    }

    if (*task).worker as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 656);
    }

    if (*cap).n_spare_workers < MAX_SPARE_WORKERS as u32 {
        (*task).next = (*cap).spare_workers as *mut Task_;
        (*cap).spare_workers = task;
        (*cap).n_spare_workers = (*cap).n_spare_workers.wrapping_add(1);
    } else {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"%d spare workers already, exiting".as_ptr(),
                (*cap).n_spare_workers,
            );
        }

        releaseCapability_(cap, false);
        workerTaskStop(task);

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                671,
            );
        }

        shutdownThread();
    };
}

unsafe fn waitForWorkerCapability(mut task: *mut Task) -> *mut Capability {
    let mut cap = null_mut::<Capability>();

    loop {
        let mut __r = pthread_mutex_lock(&raw mut (*task).lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                721,
                __r,
            );
        }

        if !(*task).wakeup {
            waitCondition(&raw mut (*task).cond, &raw mut (*task).lock);
        }

        cap = (*task).cap as *mut Capability;
        (*task).wakeup = false;

        if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                732,
            );
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(c"woken up on capability %d".as_ptr(), (*cap).no);
        }

        let mut __r_0 = pthread_mutex_lock(&raw mut (*cap).lock);

        if __r_0 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                736,
                __r_0,
            );
        }

        if !(*cap).running_task.is_null() {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(
                    c"capability %d is owned by another task".as_ptr(),
                    (*cap).no,
                );
            }

            if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Capability.c".as_ptr(),
                    740,
                );
            }
        } else if (*task).cap != cap {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(
                    c"task has been migrated to cap %d".as_ptr(),
                    (*(*task).cap).no,
                );
            }

            if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Capability.c".as_ptr(),
                    748,
                );
            }
        } else {
            if (*(*task).incall).tso.is_null() {
                if !(*cap).spare_workers.is_null() as i32 as i64 != 0 {
                } else {
                    _assertFail(c"rts/Capability.c".as_ptr(), 753);
                }

                if (*cap).spare_workers != task {
                    giveCapabilityToTask(cap, (*cap).spare_workers);

                    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                        barf(
                            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                            c"rts/Capability.c".as_ptr(),
                            758,
                        );
                    }

                    continue;
                } else {
                    (*cap).spare_workers = (*task).next as *mut Task;
                    (*task).next = null_mut::<Task_>();
                    (*cap).n_spare_workers = (*cap).n_spare_workers.wrapping_sub(1);
                }
            }

            (&raw mut (*cap).running_task).store(task, Ordering::Relaxed);

            if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Capability.c".as_ptr(),
                    767,
                );
            }

            break;
        }
    }

    return cap;
}

unsafe fn waitForReturnCapability(mut task: *mut Task) -> *mut Capability {
    let mut cap = null_mut::<Capability>();

    loop {
        let mut __r = pthread_mutex_lock(&raw mut (*task).lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                792,
                __r,
            );
        }

        if !(*task).wakeup {
            waitCondition(&raw mut (*task).cond, &raw mut (*task).lock);
        }

        cap = (*task).cap as *mut Capability;
        (*task).wakeup = false;

        if pthread_mutex_unlock(&raw mut (*task).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                797,
            );
        }

        let mut __r_0 = pthread_mutex_lock(&raw mut (*cap).lock);

        if __r_0 != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                800,
                __r_0,
            );
        }

        if (*cap).running_task.is_null() {
            if (*cap).returning_tasks_hd != task {
                giveCapabilityToTask(cap, (*cap).returning_tasks_hd);

                if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                    barf(
                        c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                        c"rts/Capability.c".as_ptr(),
                        804,
                    );
                }
            } else {
                (&raw mut (*cap).running_task).store(task, Ordering::Relaxed);
                popReturningTask(cap);

                if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                    barf(
                        c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                        c"rts/Capability.c".as_ptr(),
                        809,
                    );
                }

                break;
            }
        } else if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                812,
            );
        }
    }

    return cap;
}

unsafe fn capability_is_busy(mut cap: *const Capability) -> bool {
    return !(&raw const (*cap).running_task)
        .load(Ordering::Relaxed)
        .is_null();
}

unsafe fn find_capability_for_task(mut task: *const Task) -> *mut Capability {
    if (*task).preferred_capability != -1 {
        return getCapability(
            ((*task).preferred_capability as u32).wrapping_rem(enabled_capabilities),
        );
    } else {
        let mut cap = ((&raw mut last_free_capability as *mut *mut Capability)
            .offset((*task).node as isize) as *mut *mut Capability)
            .load(Ordering::Relaxed);

        if !capability_is_busy(cap) {
            return cap;
        } else {
            let mut i: u32 = (*task).node;

            while i < enabled_capabilities {
                if (&raw mut (*(getCapability as unsafe extern "C" fn(c_uint) -> *mut Capability)(
                    i,
                ))
                .running_task)
                    .load(Ordering::Relaxed)
                    .is_null()
                {
                    return getCapability(i);
                }

                i = i.wrapping_add(n_numa_nodes);
            }

            return ((&raw mut last_free_capability as *mut *mut Capability)
                .offset((*task).node as isize) as *mut *mut Capability)
                .load(Ordering::Relaxed);
        }
    };
}

unsafe fn waitForCapability(mut pCap: *mut *mut Capability, mut task: *mut Task) {
    let mut cap = *pCap;

    if cap.is_null() {
        cap = find_capability_for_task(task);
        (*task).cap = cap as *mut Capability_;
    } else if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 908);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"returning; I want capability %d".as_ptr(), (*cap).no);
    }

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            913,
            __r,
        );
    }

    if (*cap).running_task.is_null() {
        (&raw mut (*cap).running_task).store(task, Ordering::Relaxed);

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                917,
            );
        }
    } else {
        newReturningTask(cap, task);

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                920,
            );
        }

        cap = waitForReturnCapability(task);
    }

    (*cap).r.rCCCS = &raw mut CCS_SYSTEM as *mut CostCentreStack as *mut CostCentreStack_;

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 928);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"resuming capability %d".as_ptr(), (*cap).no);
    }

    *pCap = cap;
}

unsafe fn yieldCapability(
    mut pCap: *mut *mut Capability,
    mut task: *mut Task,
    mut gcAllowed: bool,
) -> bool {
    let mut cap = *pCap;

    if gcAllowed {
        let mut sync = (&raw mut pending_sync).load(Ordering::SeqCst);

        if !sync.is_null() {
            match (*sync).r#type as u32 {
                2 => {
                    if !*(*sync).idle.offset((*cap).no as isize) {
                        traceEventGcStart(cap);
                        gcWorkerThread(cap);
                        traceEventGcEnd(cap);
                        traceSparkCounters(cap);

                        if (*task).cap == cap {
                            return true;
                        }
                    }
                }
                3 => {
                    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as i64 != 0 {
                        trace_(c"Flushing update remembered set blocks...".as_ptr());
                    }
                }
                4 | _ => {}
            }
        }
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"giving up capability %d".as_ptr(), (*cap).no);
    }

    (*task).wakeup = false;

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            1016,
            __r,
        );
    }

    if isWorker(task) {
        enqueueWorker(cap);
    }

    releaseCapability_(cap, false);

    if isWorker(task) as i32 != 0 || isBoundTask(task) as i32 != 0 {
        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                1026,
            );
        }

        cap = waitForWorkerCapability(task);
    } else {
        newReturningTask(cap, task);

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                1035,
            );
        }

        cap = waitForReturnCapability(task);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"resuming capability %d".as_ptr(), (*cap).no);
    }

    if ((*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1040);
    }

    (*cap).r.rCCCS = &raw mut CCS_SYSTEM as *mut CostCentreStack as *mut CostCentreStack_;
    *pCap = cap;

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    if (if (*cap).run_queue_hd == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        ((*cap).run_queue_tl == &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO
            && (*cap).n_run_queue == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1048);
    }

    return false;
}

unsafe fn prodCapability(mut cap: *mut Capability, mut task: *mut Task) {
    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            1101,
            __r,
        );
    }

    if (*cap).running_task.is_null() {
        (*cap).running_task = task;
        releaseCapability_(cap, true);
    }

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            1106,
        );
    }
}

unsafe fn tryGrabCapability(mut cap: *mut Capability, mut task: *mut Task) -> bool {
    let mut r: i32 = 0;

    if !(&raw mut (*cap).running_task)
        .load(Ordering::Relaxed)
        .is_null()
    {
        return false;
    }

    r = OS_TRY_ACQUIRE_LOCK(&raw mut (*cap).lock);

    if r != 0 {
        return false;
    }

    if !(*cap).running_task.is_null() {
        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                1131,
            );
        }

        return false;
    }

    (*task).cap = cap as *mut Capability_;
    (&raw mut (*cap).running_task).store(task, Ordering::Relaxed);

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/Capability.c".as_ptr(),
            1136,
        );
    }

    return true;
}

unsafe fn shutdownCapability(mut cap: *mut Capability, mut task: *mut Task, mut safe: bool) {
    let mut i: u32 = 0;
    (*task).cap = cap as *mut Capability_;
    i = 0;

    loop {
        if (getSchedState() as u32 == SCHED_SHUTTING_DOWN as i32 as u32) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Capability.c".as_ptr(), 1175);
        }

        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
            trace_(
                c"shutting down capability %d, attempt %d".as_ptr(),
                (*cap).no,
                i,
            );
        }

        let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

        if __r != 0 {
            barf(
                c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
                c"rts/Capability.c".as_ptr(),
                1179,
                __r,
            );
        }

        if !(*cap).running_task.is_null() {
            if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                barf(
                    c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                    c"rts/Capability.c".as_ptr(),
                    1181,
                );
            }

            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                trace_(c"not owner, yielding".as_ptr());
            }

            yieldThread();
        } else {
            (*cap).running_task = task;

            if !(*cap).spare_workers.is_null() {
                let mut t = null_mut::<Task>();
                let mut prev = null_mut::<Task>();
                prev = null_mut::<Task>();
                t = (*cap).spare_workers;

                while !t.is_null() {
                    if !osThreadIsAlive((*t).id) {
                        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                            trace_(
                                c"worker thread %p has died unexpectedly".as_ptr(),
                                (*t).id as usize as *mut c_void,
                            );
                        }

                        (*cap).n_spare_workers = (*cap).n_spare_workers.wrapping_sub(1);

                        if prev.is_null() {
                            (*cap).spare_workers = (*t).next as *mut Task;
                        } else {
                            (*prev).next = (*t).next;
                        }

                        prev = t;
                    }

                    t = (*t).next as *mut Task;
                }
            }

            if !emptyRunQueue(cap) || !(*cap).spare_workers.is_null() {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(c"runnable threads or workers still alive, yielding".as_ptr());
                }

                releaseCapability_(cap, false);

                if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                    barf(
                        c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                        c"rts/Capability.c".as_ptr(),
                        1216,
                    );
                }

                yieldThread();
            } else if !(*cap).suspended_ccalls.is_null() && safe as i32 != 0 {
                if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
                    trace_(c"thread(s) are involved in foreign calls, yielding".as_ptr());
                }

                (*cap).running_task = null_mut::<Task>();

                if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                    barf(
                        c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                        c"rts/Capability.c".as_ptr(),
                        1231,
                    );
                }

                stopIOManager();
                yieldThread();
            } else {
                traceSparkCounters(cap);

                if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
                    barf(
                        c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                        c"rts/Capability.c".as_ptr(),
                        1253,
                    );
                }

                break;
            }
        }

        i = i.wrapping_add(1);
    }
}

unsafe fn shutdownCapabilities(mut task: *mut Task, mut safe: bool) {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        if (*(*task).incall).tso.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Capability.c".as_ptr(), 1271);
        }

        shutdownCapability(getCapability(i), task, safe);
        i = i.wrapping_add(1);
    }

    if checkSparkCountInvariant() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Capability.c".as_ptr(), 1275);
    };
}

unsafe fn freeCapability(mut cap: *mut Capability) {
    stgFree((*cap).mut_lists as *mut c_void);
    stgFree((*cap).saved_mut_lists as *mut c_void);

    if !(*cap).current_segments.is_null() {
        stgFree((*cap).current_segments as *mut c_void);
    }

    freeSparkPool((*cap).sparks);
    traceCapsetRemoveCap(CAPSET_OSPROCESS_DEFAULT, (*cap).no);
    traceCapsetRemoveCap(CAPSET_CLOCKDOMAIN_DEFAULT, (*cap).no);
    traceCapDelete(cap);
}

unsafe fn freeCapabilities() {
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        let mut cap = getCapability(i);
        freeCapability(cap);

        if cap != &raw mut MainCapability {
            stgFreeAligned(cap as *mut c_void);
        }

        i = i.wrapping_add(1);
    }

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

    evac.expect("non-null function pointer")(
        user,
        &raw mut (*cap).inbox as *mut c_void as *mut *mut StgClosure,
    );

    incall = (*cap).suspended_ccalls;

    while !incall.is_null() {
        evac.expect("non-null function pointer")(
            user,
            &raw mut (*incall).suspended_tso as *mut c_void as *mut *mut StgClosure,
        );

        incall = (*incall).next as *mut InCall;
    }

    if !no_mark_sparks {
        traverseSparkQueue(evac, user, cap);
    }

    markCapabilityIOManager(evac, user, cap);
    stmPreGCHook(cap);
}

unsafe fn markCapabilities(mut evac: evac_fn, mut user: *mut c_void) {
    let mut n: u32 = 0;
    n = 0;

    while n < getNumCapabilities() as u32 {
        markCapability(evac, user, getCapability(n), false);
        n = n.wrapping_add(1);
    }
}

unsafe fn checkSparkCountInvariant() -> bool {
    let mut sparks = SparkCounters {
        created: 0,
        dud: 0,
        overflowed: 0,
        converted: 0,
        gcd: 0,
        fizzled: 0,
    };

    let mut remaining: StgWord64 = 0;
    let mut i: u32 = 0;
    i = 0;

    while i < getNumCapabilities() as u32 {
        let mut cap = getCapability(i);
        sparks.created = sparks.created.wrapping_add((*cap).spark_stats.created);
        sparks.dud = sparks.dud.wrapping_add((*cap).spark_stats.dud);
        sparks.overflowed = sparks
            .overflowed
            .wrapping_add((*cap).spark_stats.overflowed);
        sparks.converted = sparks.converted.wrapping_add((*cap).spark_stats.converted);
        sparks.gcd = sparks.gcd.wrapping_add((*cap).spark_stats.gcd);
        sparks.fizzled = sparks.fizzled.wrapping_add((*cap).spark_stats.fizzled);
        remaining = remaining.wrapping_add(sparkPoolSize((*cap).sparks) as StgWord64);
        i = i.wrapping_add(1);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.sparks as i64 != 0 {
        trace_(
            c"spark invariant: %ld == %ld + %ld + %ld + %ld (created == converted + remaining + gcd + fizzled)"
                .as_ptr(),
            sparks.created,
            sparks.converted,
            remaining,
            sparks.gcd,
            sparks.fizzled,
        );
    }

    return sparks.created
        == sparks
            .converted
            .wrapping_add(remaining as StgWord)
            .wrapping_add(sparks.gcd)
            .wrapping_add(sparks.fizzled);
}
