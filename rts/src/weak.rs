use crate::alloc_array::allocateMutArrPtrs;
use crate::capability::Capability;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::closure_macros::{
    SET_INFO, doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
    mutArrPtrsCardTableSize,
};
use crate::ffi::rts::storage::closures::{StgCFinalizerList, StgMutArrPtrs, StgWeak};
use crate::ffi::rts::threads::createIOThread;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::{_assertFail, exitHeapOverflow};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_DEAD_WEAK_info, stg_MUT_ARR_PTRS_FROZEN_CLEAN_info, stg_NO_FINALIZER_closure,
};
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::StgWord;
use crate::hs_ffi::HsInt;
use crate::prelude::*;
use crate::rts_api::{HaskellObj, rts_apply, rts_mkInt};
use crate::rts_flags::RtsFlags;
use crate::schedule::scheduleThread;
use crate::task::myTask;
use crate::trace::{DEBUG_RTS, trace_};

static mut finalizer_list: *mut StgWeak = null_mut::<StgWeak>();

static mut n_finalizers: u32 = 0;

unsafe fn runCFinalizers(mut list: *mut StgCFinalizerList) {
    let mut head = null_mut::<StgCFinalizerList>();
    head = list;

    while head as *mut StgClosure != &raw mut stg_NO_FINALIZER_closure {
        if (*head).flag != 0 {
            transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
            >((*head).fptr)
            .expect("non-null function pointer")((*head).eptr, (*head).ptr);
        } else {
            transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn(*mut c_void) -> ()>,
            >((*head).fptr)
            .expect("non-null function pointer")((*head).ptr);
        }

        head = (*head).link as *mut StgCFinalizerList;
    }
}

unsafe fn runAllCFinalizers(mut list: *mut StgWeak) {
    let mut w = null_mut::<StgWeak>();
    let mut task = null_mut::<Task>();
    task = myTask();

    if !task.is_null() {
        (*task).running_finalizers = true;
    }

    w = list;

    while !w.is_null() {
        let mut winfo = (&raw mut (*w).header.info).load(Ordering::Acquire);

        if winfo != &raw const stg_DEAD_WEAK_info {
            runCFinalizers((*w).cfinalizers as *mut StgCFinalizerList);
        }

        w = (*w).link as *mut StgWeak;
    }

    if !task.is_null() {
        (*task).running_finalizers = false;
    }
}

unsafe fn scheduleFinalizers(mut cap: *mut Capability, mut list: *mut StgWeak) {
    let mut w = null_mut::<StgWeak>();
    let mut t = null_mut::<StgTSO>();
    let mut n: u32 = 0;
    let mut i: u32 = 0;

    if (RtsFlags.GcFlags.useNonmoving as i32 != 0
        || (&raw mut n_finalizers).load(Ordering::SeqCst) == 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Weak.c".as_ptr(), 98);
    }

    let mut tl: *mut *mut StgWeak = &raw mut finalizer_list;

    while !(*tl).is_null() {
        tl = &raw mut (**tl).link as *mut *mut StgWeak;
    }

    (tl).store(list, Ordering::SeqCst);
    n = 0;
    i = 0;
    w = list;

    while !w.is_null() {
        if ((*w).header.info != &raw const stg_DEAD_WEAK_info) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Weak.c".as_ptr(), 117);
        }

        if (*w).finalizer != &raw mut stg_NO_FINALIZER_closure {
            n = n.wrapping_add(1);
        }

        i = i.wrapping_add(1);

        let ref mut fresh13 = (*(w as *mut StgClosure)).header.prof.ccs;
        *fresh13 = (*w).header.prof.ccs;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*(w as *mut StgClosure)).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*(w as *mut StgClosure)).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*(w as *mut StgClosure)).header.prof.hp.era = user_era;
        }

        (&raw mut (*w).header.info).store(&raw const stg_DEAD_WEAK_info, Ordering::Relaxed);
        w = (*w).link as *mut StgWeak;
    }

    let fresh14 = &raw mut n_finalizers;
    let fresh15 = i;
    (fresh14).xadd(fresh15, Ordering::SeqCst) + fresh15;

    if n == 0 {
        return;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.weak as i64 != 0 {
        trace_(c"weak: batching %d finalizers".as_ptr(), n);
    }

    let mut arr = allocateMutArrPtrs(
        cap,
        n as StgWord,
        &raw mut CCS_SYSTEM as *mut CostCentreStack,
    );

    if (arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as i32 as i64 != 0 {
        exitHeapOverflow();
    }

    SET_INFO(
        arr as *mut StgClosure,
        &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    );
    n = 0;
    w = list;

    while !w.is_null() {
        if (*w).finalizer != &raw mut stg_NO_FINALIZER_closure {
            let ref mut fresh16 =
                *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(n as isize);
            *fresh16 = (*w).finalizer;
            n = n.wrapping_add(1);
        }

        w = (*w).link as *mut StgWeak;
    }

    let mut size: StgWord =
        (n as StgWord).wrapping_add(mutArrPtrsCardTableSize(n as W_) as StgWord);
    i = n;

    while (i as StgWord) < size {
        let ref mut fresh7 = *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(i as isize);
        *fresh7 = -1 as W_ as *mut StgClosure;
        i = i.wrapping_add(1);
    }

    t = createIOThread(
        cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        rts_apply(
            cap,
            rts_apply(
                cap,
                (*ghc_hs_iface).runFinalizzerBatch_closure as HaskellObj,
                rts_mkInt(cap, n as HsInt),
            ),
            arr as HaskellObj,
        ) as *mut StgClosure,
    );

    scheduleThread(cap, t);
}

static mut finalizer_chunk: i32 = 100;

static mut finalizer_lock: StgWord = 0;

unsafe fn runSomeFinalizers(mut all: bool) -> bool {
    if (&raw mut n_finalizers).load(Ordering::Relaxed) == 0 {
        return false;
    }

    if cas(&raw mut finalizer_lock, 0, 1) != 0 {
        return false;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"running C finalizers, %d remaining".as_ptr(), n_finalizers);
    }

    let mut task = myTask();

    if !task.is_null() {
        (*task).running_finalizers = true;
    }

    let mut w = finalizer_list;
    let mut count: i32 = 0;

    while !w.is_null() {
        runCFinalizers((*w).cfinalizers as *mut StgCFinalizerList);
        w = (*w).link as *mut StgWeak;
        count += 1;

        if !all && count >= finalizer_chunk {
            break;
        }
    }

    (&raw mut finalizer_list).store(w, Ordering::Relaxed);

    let fresh18 = &raw mut n_finalizers;
    let fresh19 = -count as u32;
    (fresh18).xadd(fresh19, Ordering::SeqCst) + fresh19;

    if !task.is_null() {
        (*task).running_finalizers = false;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as i64 != 0 {
        trace_(c"ran %d C finalizers".as_ptr(), count);
    }

    let mut ret = n_finalizers != 0;
    (&raw mut finalizer_lock).store(0, Ordering::Release);

    return ret;
}
