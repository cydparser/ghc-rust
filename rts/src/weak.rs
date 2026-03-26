use crate::alloc_array::allocateMutArrPtrs;
use crate::ffi::hs_ffi::HsInt;
use crate::ffi::rts::exitHeapOverflow;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::closure_macros::{SET_INFO, mutArrPtrsCardTableSize};
use crate::ffi::rts::storage::closures::{StgCFinalizerList, StgMutArrPtrs, StgWeak};
use crate::ffi::rts::threads::createIOThread;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts_api::{Capability, HaskellObj, rts_apply, rts_mkInt};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_DEAD_WEAK_info, stg_MUT_ARR_PTRS_FROZEN_CLEAN_info, stg_NO_FINALIZER_closure,
};
use crate::ffi::stg::smp::cas;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::schedule::scheduleThread;
use crate::task::myTask;
use crate::trace::{DEBUG_RTS, trace_};

static mut finalizer_list: *mut StgWeak = null::<StgWeak>() as *mut StgWeak;

static mut n_finalizers: uint32_t = 0 as uint32_t;

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
        (*task).running_finalizers = r#true != 0;
    }

    w = list;

    while !w.is_null() {
        let mut winfo = (*w).header.info;

        if winfo != &raw const stg_DEAD_WEAK_info {
            runCFinalizers((*w).cfinalizers as *mut StgCFinalizerList);
        }

        w = (*w).link as *mut StgWeak;
    }

    if !task.is_null() {
        (*task).running_finalizers = r#false != 0;
    }
}

unsafe fn scheduleFinalizers(mut cap: *mut Capability, mut list: *mut StgWeak) {
    let mut w = null_mut::<StgWeak>();
    let mut t = null_mut::<StgTSO>();
    let mut n: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut tl: *mut *mut StgWeak = &raw mut finalizer_list;

    while !(*tl).is_null() {
        tl = &raw mut (**tl).link as *mut *mut StgWeak;
    }

    *tl = list;
    n = 0 as uint32_t;
    i = 0 as uint32_t;
    w = list;

    while !w.is_null() {
        if (*w).finalizer != &raw mut stg_NO_FINALIZER_closure {
            n = n.wrapping_add(1);
        }

        i = i.wrapping_add(1);
        (*w).header.info = &raw const stg_DEAD_WEAK_info;
        w = (*w).link as *mut StgWeak;
    }

    n_finalizers = n_finalizers.wrapping_add(i);

    if n == 0 as uint32_t {
        return;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.weak as c_long != 0 {
        trace_(
            b"weak: batching %d finalizers\0" as *const u8 as *const c_char as *mut c_char,
            n,
        );
    }

    let mut arr = allocateMutArrPtrs(cap, n as StgWord, null_mut::<CostCentreStack>());

    if (arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as c_int as c_long != 0 {
        exitHeapOverflow();
    }

    SET_INFO(
        arr as *mut StgClosure,
        &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
    );

    n = 0 as uint32_t;
    w = list;

    while !w.is_null() {
        if (*w).finalizer != &raw mut stg_NO_FINALIZER_closure {
            let ref mut fresh6 =
                *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(n as isize);
            *fresh6 = (*w).finalizer;
            n = n.wrapping_add(1);
        }

        w = (*w).link as *mut StgWeak;
    }

    let mut size: StgWord =
        (n as StgWord).wrapping_add(mutArrPtrsCardTableSize(n as W_) as StgWord);
    i = n;

    while (i as StgWord) < size {
        let ref mut fresh7 = *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(i as isize);
        *fresh7 = -(1 as c_int) as W_ as *mut StgClosure;
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

static mut finalizer_chunk: int32_t = 100 as int32_t;

static mut finalizer_lock: StgWord = 0 as StgWord;

unsafe fn runSomeFinalizers(mut all: bool) -> bool {
    if n_finalizers == 0 as uint32_t {
        return r#false != 0;
    }

    if cas(&raw mut finalizer_lock, 0 as StgWord, 1 as StgWord) != 0 as StgWord {
        return r#false != 0;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"running C finalizers, %d remaining\0" as *const u8 as *const c_char as *mut c_char,
            n_finalizers,
        );
    }

    let mut task = myTask();

    if !task.is_null() {
        (*task).running_finalizers = r#true != 0;
    }

    let mut w = finalizer_list;
    let mut count: int32_t = 0 as int32_t;

    while !w.is_null() {
        runCFinalizers((*w).cfinalizers as *mut StgCFinalizerList);
        w = (*w).link as *mut StgWeak;
        count += 1;

        if !all && count >= finalizer_chunk {
            break;
        }
    }

    finalizer_list = w;
    n_finalizers = n_finalizers.wrapping_add(-count as uint32_t);

    if !task.is_null() {
        (*task).running_finalizers = r#false != 0;
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.scheduler as c_long != 0 {
        trace_(
            b"ran %d C finalizers\0" as *const u8 as *const c_char as *mut c_char,
            count,
        );
    }

    let mut ret = n_finalizers != 0 as uint32_t;
    write_volatile(&mut finalizer_lock as *mut StgWord, 0 as StgWord);

    return ret;
}
