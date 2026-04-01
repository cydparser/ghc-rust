use crate::capability::{getCapability, markCapabilities};
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack};
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::closure_macros::{UNTAG_CLOSURE, get_itbl};
use crate::ffi::rts::storage::closures::StgWeak;
use crate::ffi::rts::storage::gc::generations;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::stg_END_TSO_QUEUE_closure;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::profiling::prof_file;
use crate::retainer_set::{
    RetainerSet, addElement, initializeAllRetainerSet, isMember, outputAllRetainerSet, retainer,
    singleton,
};
use crate::stable_name::rememberOldStableNameAddresses;
use crate::stable_ptr::markStablePtrTable;
use crate::stats::{stat_endRP, stat_startRP};
use crate::traverse_heap::{
    closeTraverseStack, getTravData, getTraverseStackMaxSize, initializeTraverseStack,
    isTravDataValid, setTravData, stackAccum, stackData, stackData_, stackElement,
    traverseInvalidateClosureData, traverseMaybeInitClosureData, traversePushRoot, traverseState,
    traverseState_, traverseWorkStack, traverseWorkStackBlocks,
};

static mut retainerGeneration: u32 = 0;

static mut numObjectVisited: u32 = 0;

static mut timesAnyObjectVisited: u32 = 0;

static mut g_retainerTraverseState: traverseState = traverseState_ {
    flip: 0,
    firstStack: null_mut::<bdescr>(),
    currentStack: null_mut::<bdescr>(),
    stackBottom: null_mut::<stackElement>(),
    stackTop: null_mut::<stackElement>(),
    stackLimit: null_mut::<stackElement>(),
    stackSize: 0,
    maxStackSize: 0,
    return_cb: None,
};

unsafe fn retainerStackBlocks() -> W_ {
    return traverseWorkStackBlocks(&raw mut g_retainerTraverseState);
}

unsafe fn initRetainerProfiling() {
    initializeAllRetainerSet();
    retainerGeneration = 0;
}

unsafe fn endRetainerProfiling() {
    outputAllRetainerSet(prof_file);
}

unsafe fn isRetainer(mut c: *const StgClosure) -> bool {
    match (*get_itbl(c)).r#type {
        52 | 53 | 51 | 39 | 40 | 41 | 47 | 48 | 43 | 44 | 59 | 60 | 37 | 15 | 16 | 17 | 18 | 19
        | 20 | 22 | 24 | 26 | 21 | 49 => return true,
        1 | 7 | 2 | 3 | 4 | 5 | 6 | 8 | 9 | 10 | 11 | 12 | 13 | 25 | 64 | 28 | 38 | 58 | 14
        | 50 | 23 | 42 | 63 | 54 | 46 | 45 | 62 | 61 => return false,
        33 | 34 | 56 | 57 | 35 | 55 | 36 | 29 | 30 | 31 | 32 | 65 | 27 | 0 | _ => {
            barf(
                c"Invalid object in isRetainer(): %d".as_ptr(),
                (*get_itbl(c)).r#type,
            );
        }
    };
}

unsafe fn getRetainerFrom(mut c: *mut StgClosure) -> retainer {
    if isRetainer(c) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RetainerProfile.c".as_ptr(), 239);
    }

    return (*c).header.prof.ccs as retainer;
}

unsafe fn associate(mut c: *mut StgClosure, mut s: *mut RetainerSet) {
    setTravData(&raw mut g_retainerTraverseState, c, s as StgWord);
}

unsafe fn isRetainerSetValid(mut c: *const StgClosure) -> bool {
    return isTravDataValid(&raw mut g_retainerTraverseState, c);
}

#[inline]
unsafe fn retainerSetOf(mut c: *const StgClosure) -> *mut RetainerSet {
    if isRetainerSetValid(c) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RetainerProfile.c".as_ptr(), 267);
    }

    return getTravData(c) as *mut RetainerSet;
}

unsafe fn retainVisitClosure(
    mut c: *mut StgClosure,
    mut cp: *const StgClosure,
    data: stackData,
    first_visit: bool,
    mut acc: *mut stackAccum,
    mut out_data: *mut stackData,
) -> bool {
    let mut r = data.c_child_r;
    let mut s = null_mut::<RetainerSet>();
    let mut retainerSetOfc = null_mut::<RetainerSet>();
    retainerSetOfc = retainerSetOf(c);
    timesAnyObjectVisited = timesAnyObjectVisited.wrapping_add(1);

    if isRetainer(cp) {
        s = null_mut::<RetainerSet>();
    } else {
        s = retainerSetOf(cp);
    }

    if retainerSetOfc.is_null() {
        numObjectVisited = numObjectVisited.wrapping_add(1);

        if s.is_null() {
            associate(c, singleton(r));
        } else {
            associate(c, s);
        }

        (*out_data).c_child_r = if isRetainer(c) as i32 != 0 {
            getRetainerFrom(c)
        } else {
            r
        };
    } else {
        if isMember(r, retainerSetOfc) {
            return 0 != 0;
        }

        if s.is_null() {
            associate(c, addElement(r, retainerSetOfc));
        } else if (*s).num == (*retainerSetOfc).num.wrapping_add(1 as u32) {
            associate(c, s);
        } else {
            associate(c, addElement(r, retainerSetOfc));
        }

        if isRetainer(c) {
            return 0 != 0;
        }

        (*out_data).c_child_r = r;
    }

    return 1 != 0;
}

unsafe fn retainRoot(mut user: *mut c_void, mut tl: *mut *mut StgClosure) {
    let mut ts = user as *mut traverseState;
    let mut c = null_mut::<StgClosure>();
    c = UNTAG_CLOSURE(*tl);
    traverseMaybeInitClosureData(&raw mut g_retainerTraverseState, c);

    if c != &raw mut stg_END_TSO_QUEUE_closure && isRetainer(c) as i32 != 0 {
        traversePushRoot(
            ts,
            c,
            c,
            stackData_ {
                c_child_r: getRetainerFrom(c),
            },
        );
    } else {
        traversePushRoot(
            ts,
            c,
            c,
            stackData_ {
                c_child_r: &raw mut CCS_SYSTEM as *mut CostCentreStack,
            },
        );
    };
}

unsafe fn computeRetainerSet(mut ts: *mut traverseState) {
    let mut weak = null_mut::<StgWeak>();
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    traverseInvalidateClosureData(ts);

    markCapabilities(
        Some(retainRoot as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ts as *mut c_void,
    );

    n = 0;

    while n < getNumCapabilities() as u32 {
        if (*getCapability(n)).weak_ptr_list_hd.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/RetainerProfile.c".as_ptr(), 398);
        }

        if (*getCapability(n)).weak_ptr_list_tl.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/RetainerProfile.c".as_ptr(), 399);
        }

        n = n.wrapping_add(1);
    }

    g = 0;

    while g < RtsFlags.GcFlags.generations {
        weak = (*generations.offset(g as isize)).weak_ptr_list;

        while !weak.is_null() {
            retainRoot(ts as *mut c_void, &raw mut weak as *mut *mut StgClosure);
            weak = (*weak).link as *mut StgWeak;
        }

        g = g.wrapping_add(1);
    }

    markStablePtrTable(
        Some(retainRoot as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        ts as *mut c_void,
    );

    rememberOldStableNameAddresses();

    traverseWorkStack(
        ts,
        Some(
            retainVisitClosure
                as unsafe extern "C" fn(
                    *mut StgClosure,
                    *const StgClosure,
                    stackData,
                    bool,
                    *mut stackAccum,
                    *mut stackData,
                ) -> bool,
        ),
    );
}

unsafe fn retainerProfile() {
    stat_startRP();
    numObjectVisited = 0;
    timesAnyObjectVisited = 0;
    initializeTraverseStack(&raw mut g_retainerTraverseState);
    initializeAllRetainerSet();
    computeRetainerSet(&raw mut g_retainerTraverseState);
    closeTraverseStack(&raw mut g_retainerTraverseState);
    retainerGeneration = retainerGeneration.wrapping_add(1);

    stat_endRP(
        retainerGeneration.wrapping_sub(1 as u32),
        getTraverseStackMaxSize(&raw mut g_retainerTraverseState),
        timesAnyObjectVisited as f64 / numObjectVisited as f64,
    );
}
