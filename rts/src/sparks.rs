use crate::capability::regTableToCapability;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::storage::block::{BF_EVACUATED, BF_NONMOVING, Bdescr};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, INFO_PTR_TO_STRUCT, UNTAG_CONST_CLOSURE, get_itbl,
};
use crate::ffi::rts::storage::closure_macros::{GET_CLOSURE_TAG, UNTAG_CONST_CLOSURE, get_itbl};
use crate::ffi::rts::storage::closure_types::THUNK_STATIC;
use crate::ffi::rts::storage::closures::StgClosurePtr;
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::info_tables::{_NS, closure_flags};
use crate::ffi::rts::storage::info_tables::{_NS, closure_flags};
use crate::ffi::rts::threads::createIOThread;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::{StgHalfWord, StgInt, StgPtr, StgWord, StgWord16};
use crate::ffi::stg::types::{StgInt, StgWord, StgWord16};
use crate::prelude::*;
use crate::schedule::appendToRunQueue;
use crate::sm::gc::evac_fn;
use crate::sm::non_moving_mark::nonmovingIsAlive;
use crate::sparks::{SparkPool, fizzledSpark, sparkPoolSize};
use crate::thread_labels::setThreadLabel;
use crate::trace::{
    DEBUG_RTS, trace_, traceEventCreateSparkThread, traceEventSparkCreate, traceEventSparkDud,
    traceEventSparkFizzle, traceEventSparkGC, traceEventSparkOverflow,
};
use crate::ws_deque::{
    WSDeque, dequeElements, discardElements, freeWSDeque, looksEmptyWSDeque, newWSDeque,
    popWSDeque, pushWSDeque, stealWSDeque_,
};
use crate::ws_deque::{
    WSDeque, dequeElements, discardElements, looksEmptyWSDeque, popWSDeque, stealWSDeque_,
};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
pub(crate) struct SparkCounters {
    pub(crate) created: StgWord,
    pub(crate) dud: StgWord,
    pub(crate) overflowed: StgWord,
    pub(crate) converted: StgWord,
    pub(crate) gcd: StgWord,
    pub(crate) fizzled: StgWord,
}

pub(crate) type SparkPool = WSDeque;

#[inline]
pub(crate) unsafe fn reclaimSpark(mut pool: *mut SparkPool) -> *mut StgClosure {
    return popWSDeque(pool as *mut WSDeque) as *mut StgClosure;
}

#[inline]
pub(crate) unsafe fn looksEmpty(mut deque: *mut SparkPool) -> bool {
    return looksEmptyWSDeque(deque as *mut WSDeque);
}

#[inline]
pub(crate) unsafe fn sparkPoolSize(mut pool: *mut SparkPool) -> i64 {
    return dequeElements(pool as *mut WSDeque) as i64;
}

#[inline]
pub(crate) unsafe fn discardSparks(mut pool: *mut SparkPool) {
    discardElements(pool as *mut WSDeque);
}

#[inline]
pub(crate) unsafe fn tryStealSpark(mut pool: *mut SparkPool) -> *mut StgClosure {
    return stealWSDeque_(pool as *mut WSDeque) as *mut StgClosure;
}

#[inline]
pub(crate) unsafe fn fizzledSpark(mut spark: *mut StgClosure) -> bool {
    return GET_CLOSURE_TAG(spark) != 0
        || *(&raw const closure_flags as *const StgWord16)
            .offset((*get_itbl(UNTAG_CONST_CLOSURE(spark))).r#type as isize) as i32
            & _NS
            != 0;
}

unsafe fn allocSparkPool() -> *mut SparkPool {
    return newWSDeque(RtsFlags.ParFlags.maxLocalSparks) as *mut SparkPool;
}

unsafe fn freeSparkPool(mut pool: *mut SparkPool) {
    freeWSDeque(pool as *mut WSDeque);
}

unsafe fn createSparkThread(mut cap: *mut Capability) {
    let mut tso = null_mut::<StgTSO>();

    tso = createIOThread(
        cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        (*ghc_hs_iface).runSparks_closure,
    );

    setThreadLabel(cap, tso, c"spark evaluator".as_ptr());
    traceEventCreateSparkThread(cap, (*tso).id);
    appendToRunQueue(cap, tso);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn newSpark(mut reg: *mut StgRegTable, mut p: *mut StgClosure) -> StgInt {
    let mut cap = regTableToCapability(reg);
    let mut pool = (*cap).sparks;

    if !fizzledSpark(p) {
        if pushWSDeque(pool as *mut WSDeque, p as *mut c_void) {
            (*cap).spark_stats.created = (*cap).spark_stats.created.wrapping_add(1);
            traceEventSparkCreate(cap);
        } else {
            (*cap).spark_stats.overflowed = (*cap).spark_stats.overflowed.wrapping_add(1);
            traceEventSparkOverflow(cap);
        }
    } else {
        (*cap).spark_stats.dud = (*cap).spark_stats.dud.wrapping_add(1);
        traceEventSparkDud(cap);
    }

    return 1;
}

unsafe fn pruneSparkQueue(mut nonmovingMarkFinished: bool, mut cap: *mut Capability) {
    let mut pool = null_mut::<SparkPool>();
    let mut spark = null_mut::<StgClosure_>();
    let mut tmp = null_mut::<StgClosure_>();
    let mut elements = null_mut::<StgClosurePtr>();
    let mut pruned_sparks: u32 = 0;
    let mut botInd: StgInt = 0;
    let mut oldBotInd: StgInt = 0;
    let mut currInd: StgInt = 0;
    let mut info = null::<StgInfoTable>();
    pruned_sparks = 0;
    pool = (*cap).sparks;

    if (*pool).top > (*pool).bottom {
        (*pool).top = (*pool).bottom;
    }

    (*pool).bottom = ((*pool).bottom as StgWord)
        .wrapping_sub((*pool).top as StgWord & !(*pool).moduloSize) as StgInt
        as StgInt;
    (*pool).top = ((*pool).top as StgWord & (*pool).moduloSize) as StgInt;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.sparks as i64 != 0 {
        trace_(
            c"markSparkQueue: current spark queue len=%ld; (hd=%ld; tl=%ld)".as_ptr(),
            sparkPoolSize(pool),
            (*pool).bottom,
            (*pool).top,
        );
    }

    if ((*pool).size > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 146);
    }

    if !(&raw mut (*pool).elements)
        .load(Ordering::Relaxed)
        .is_null() as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 146);
    }

    if (!((*pool).elements.offset(0) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 146);
    }

    if (!((*pool).elements.offset(((*pool).size - 1) as isize) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 146);
    }

    elements = (*pool).elements as *mut StgClosurePtr;
    currInd = ((*pool).top as StgWord & (*pool).moduloSize) as StgInt;
    botInd = ((*pool).bottom as StgWord & (*pool).moduloSize) as StgInt;
    oldBotInd = botInd;

    if (currInd < (*pool).size && botInd < (*pool).size) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 186);
    }

    while currInd != oldBotInd {
        spark = *elements.offset(currInd as isize);

        if GET_CLOSURE_TAG(spark as *const StgClosure) != 0 {
            pruned_sparks = pruned_sparks.wrapping_add(1);
            (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
            traceEventSparkFizzle(cap);
        } else {
            info = (&raw mut (*spark).header.info).load(Ordering::Acquire);

            if info as StgWord & 1 != 0 {
                tmp = (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure
                    as StgClosurePtr;

                if *(&raw const closure_flags as *const StgWord16).offset(
                    (*get_itbl(UNTAG_CONST_CLOSURE(tmp as *const StgClosure))).r#type as isize,
                ) as i32
                    & _NS
                    == 0
                {
                    let ref mut fresh12 = *elements.offset(botInd as isize);
                    *fresh12 = tmp;
                    botInd += 1;
                } else {
                    pruned_sparks = pruned_sparks.wrapping_add(1);
                    (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
                    traceEventSparkFizzle(cap);
                }
            } else if spark as W_ >= mblock_address_space.0.begin
                && (spark as W_) < mblock_address_space.0.end
            {
                let mut spark_bd = Bdescr(spark as StgPtr);
                let mut is_alive = false;

                if nonmovingMarkFinished {
                    if (*spark_bd).flags as i32 & BF_NONMOVING != 0 {
                        is_alive = nonmovingIsAlive(spark as *mut StgClosure);
                    } else {
                        is_alive = true;
                    }
                } else if (*spark_bd).flags as i32 & (BF_EVACUATED | BF_NONMOVING) != 0 {
                    is_alive = true;
                }

                if is_alive {
                    if *(&raw const closure_flags as *const StgWord16).offset(
                        (*get_itbl(UNTAG_CONST_CLOSURE(spark as *const StgClosure))).r#type
                            as isize,
                    ) as i32
                        & _NS
                        == 0
                    {
                        let ref mut fresh13 = *elements.offset(botInd as isize);
                        *fresh13 = spark;
                        botInd += 1;
                    } else {
                        pruned_sparks = pruned_sparks.wrapping_add(1);
                        (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
                        traceEventSparkFizzle(cap);
                    }
                } else {
                    pruned_sparks = pruned_sparks.wrapping_add(1);
                    (*cap).spark_stats.gcd = (*cap).spark_stats.gcd.wrapping_add(1);
                    traceEventSparkGC(cap);
                }
            } else if (*INFO_PTR_TO_STRUCT(info)).r#type == THUNK_STATIC as StgHalfWord {
                let ref mut fresh14 = *elements.offset(botInd as isize);
                *fresh14 = spark;
                botInd += 1;
            } else {
                pruned_sparks = pruned_sparks.wrapping_add(1);
                (*cap).spark_stats.fizzled = (*cap).spark_stats.fizzled.wrapping_add(1);
                traceEventSparkFizzle(cap);
            }
        }

        currInd += 1;

        if (currInd <= (*pool).size && botInd <= (*pool).size) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/Sparks.c".as_ptr(), 275);
        }

        if currInd == (*pool).size {
            currInd = 0;
        }

        if botInd == (*pool).size {
            botInd = 0;
        }
    }

    if (currInd == oldBotInd) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 281);
    }

    (*pool).top = oldBotInd;
    (*pool).bottom = if oldBotInd <= botInd {
        botInd
    } else {
        botInd + (*pool).size
    };

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.sparks as i64 != 0 {
        trace_(c"pruned %d sparks".as_ptr(), pruned_sparks);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.sparks as i64 != 0 {
        trace_(
            c"new spark queue len=%ld; (hd=%ld; tl=%ld)".as_ptr(),
            sparkPoolSize(pool),
            (*pool).bottom,
            (*pool).top,
        );
    }

    if ((*pool).size > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 294);
    }

    if !(&raw mut (*pool).elements)
        .load(Ordering::Relaxed)
        .is_null() as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 294);
    }

    if (!((*pool).elements.offset(0) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 294);
    }

    if (!((*pool).elements.offset(((*pool).size - 1) as isize) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 294);
    };
}

unsafe fn traverseSparkQueue(mut evac: evac_fn, mut user: *mut c_void, mut cap: *mut Capability) {
    let mut sparkp = null_mut::<*mut StgClosure>();
    let mut pool = null_mut::<SparkPool>();
    let mut top: StgWord = 0;
    let mut bottom: StgWord = 0;
    let mut modMask: StgWord = 0;
    pool = (*cap).sparks;

    if ((*pool).size > 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 308);
    }

    if !(&raw mut (*pool).elements)
        .load(Ordering::Relaxed)
        .is_null() as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 308);
    }

    if (!((*pool).elements.offset(0) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 308);
    }

    if (!((*pool).elements.offset(((*pool).size - 1) as isize) as *mut *mut c_void)
        .load(Ordering::Relaxed)
        .is_null()
        || 1 != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/Sparks.c".as_ptr(), 308);
    }

    top = (*pool).top as StgWord;
    bottom = (*pool).bottom as StgWord;
    sparkp = (*pool).elements as *mut StgClosurePtr as *mut *mut StgClosure;
    modMask = (*pool).moduloSize;

    while top < bottom {
        evac.expect("non-null function pointer")(user, sparkp.offset((top & modMask) as isize));

        top = top.wrapping_add(1);
    }

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.sparks as i64 != 0 {
        trace_(
            c"traversed spark queue, len=%ld; (hd=%ld; tl=%ld)".as_ptr(),
            sparkPoolSize(pool),
            (*pool).bottom,
            (*pool).top,
        );
    }
}
