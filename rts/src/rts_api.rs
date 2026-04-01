use crate::capability::Capability_;
use crate::capability::{
    Capability_, PutMVar, PutMVar_, getCapability, releaseCapability, releaseCapability_,
    waitForCapability,
};
use crate::ffi::hs_ffi::{
    HsBool, HsChar, HsDouble, HsFloat, HsFunPtr, HsInt, HsInt8, HsInt16, HsInt32, HsInt64, HsPtr,
    HsStablePtr, HsWord, HsWord8, HsWord16, HsWord32, HsWord64,
};
use crate::ffi::rts::constants::{
    LDV_SHIFT, LDV_STATE_CREATE, MAX_CHARLIKE, MAX_INTLIKE, MIN_INTLIKE, TSO_BLOCKEX,
    TSO_INTERRUPTIBLE, TSO_LOCKED,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, errorBelch};
use crate::ffi::rts::os_threads::{osThreadId, shutdownThread};
use crate::ffi::rts::prof::ccs::{CCS_MAIN, CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::{deRefStablePtr, getStablePtr};
use crate::ffi::rts::storage::closure_macros::{
    CHARLIKE_CLOSURE, CONSTR_sizeW, GET_CLOSURE_TAG, INTLIKE_CLOSURE, TAG_CLOSURE, UNTAG_CLOSURE,
    UNTAG_CONST_CLOSURE, doingErasProfiling, doingLDVProfiling, doingRetainerProfiling, get_itbl,
};
use crate::ffi::rts::storage::closures::StgClosure_;
use crate::ffi::rts::storage::closures::{StgClosure_, StgMVar, StgThunk};
use crate::ffi::rts::storage::gc::{allocate, generations};
use crate::ffi::rts::storage::info_tables::StgSRTField;
use crate::ffi::rts::threads::{
    createThread, enabled_capabilities, getNumCapabilities, scheduleWaitThread,
};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::rts::{_assertFail, stg_exit};
use crate::ffi::rts_api::{
    Capability, HaskellObj, HeapExhausted, Interrupted, Killed, ListRootsCb, ListThreadsCb,
    NoStatus, PauseToken, SchedulerStatus, SchedulerStatus_End, Success,
};
use crate::ffi::stg::misc_closures::{
    stg_END_TSO_QUEUE_closure, stg_ap_2_upd_info, stg_ap_v_info, stg_enter_info, stg_forceIO_info,
};
use crate::ffi::stg::types::{
    StgInt, StgInt8, StgInt16, StgInt32, StgInt64, StgPtr, StgStablePtr, StgWord, StgWord8,
    StgWord16, StgWord32, StgWord64,
};
use crate::ffi::stg::{
    ASSIGN_DBL, ASSIGN_FLT, ASSIGN_Int64, ASSIGN_Word64, PK_DBL, PK_FLT, PK_Int64, PK_Word64, W_,
};
use crate::prelude::*;
use crate::rts_utils::{stgFree, stgMallocBytes};
use crate::schedule::{releaseAllCapabilities, stopAllCapabilities};
use crate::sm::non_moving::{nonmovingBlockConcurrentMark, nonmovingUnblockConcurrentMark};
use crate::stable_name::threadStableNameTable;
use crate::stable_ptr::{freeStablePtr, threadStablePtrTable};
use crate::task::{Task, exitMyTask, freeMyTask, getMyTask, myTask, newBoundTask};
use crate::threads::performTryPutMVar;
use crate::trace::{traceTaskCreate, traceTaskDelete};

#[cfg(test)]
mod tests;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum SchedulerStatus {
    NoStatus = 0,
    Success = 1,
    Killed = 2,
    Interrupted = 3,
    HeapExhausted = 4,
    SchedulerStatus_End = 5,
}

#[cfg(feature = "sys")]
impl From<SchedulerStatus> for sys::SchedulerStatus {
    fn from(v: SchedulerStatus) -> Self {
        use SchedulerStatus::*;

        match v {
            NoStatus => sys::SchedulerStatus::NoStatus,
            Success => sys::SchedulerStatus::Success,
            Killed => sys::SchedulerStatus::Killed,
            Interrupted => sys::SchedulerStatus::Interrupted,
            HeapExhausted => sys::SchedulerStatus::HeapExhausted,
            SchedulerStatus_End => sys::SchedulerStatus::SchedulerStatus_End,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::SchedulerStatus> for SchedulerStatus {
    fn from(v: sys::SchedulerStatus) -> Self {
        use SchedulerStatus::*;

        match v {
            sys::SchedulerStatus::NoStatus => NoStatus,
            sys::SchedulerStatus::Success => Success,
            sys::SchedulerStatus::Killed => Killed,
            sys::SchedulerStatus::Interrupted => Interrupted,
            sys::SchedulerStatus::HeapExhausted => HeapExhausted,
            sys::SchedulerStatus::SchedulerStatus_End => SchedulerStatus_End,
        }
    }
}

impl TryFrom<u32> for SchedulerStatus {
    type Error = ();

    fn try_from(d: u32) -> Result<SchedulerStatus, ()> {
        use SchedulerStatus::*;

        match d {
            0 => Ok(NoStatus),
            1 => Ok(Success),
            2 => Ok(Killed),
            3 => Ok(Interrupted),
            4 => Ok(HeapExhausted),
            5 => Ok(SchedulerStatus_End),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for SchedulerStatus {
    fn arbitrary(g: &mut Gen) -> Self {
        use SchedulerStatus::*;

        match usize::arbitrary(g) % 6 {
            0 => NoStatus,
            1 => Success,
            2 => Killed,
            3 => Interrupted,
            4 => HeapExhausted,
            5.. => SchedulerStatus_End,
        }
    }
}

pub(crate) const SchedulerStatus_End: SchedulerStatus = 5;

pub(crate) const HeapExhausted: SchedulerStatus = 4;

pub(crate) const Interrupted: SchedulerStatus = 3;

pub(crate) const Killed: SchedulerStatus = 2;

pub(crate) const Success: SchedulerStatus = 1;

pub(crate) const NoStatus: SchedulerStatus = 0;

#[ffi(compiler, ghc_lib, testsuite)]
pub type Capability = Capability_;

#[ffi(compiler, ghc_lib, testsuite)]
pub type HaskellObj = *mut StgClosure_;

#[ffi(ghc_lib, testsuite)]
pub type PauseToken = PauseToken_;

#[ffi(testsuite)]
pub type ListThreadsCb = Option<unsafe extern "C" fn(*mut c_void, *mut StgTSO) -> ()>;

#[ffi(testsuite)]
pub type ListRootsCb = Option<unsafe extern "C" fn(*mut c_void, *mut StgClosure) -> ()>;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct PauseToken_ {
    capability: *mut Capability,
}

/// cbindgen:no-export
struct list_roots_ctx {
    cb: ListRootsCb,
    user: *mut c_void,
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkChar(mut cap: *mut Capability, mut c: HsChar) -> HaskellObj {
    let mut p = null_mut::<StgClosure>();

    if c <= MAX_CHARLIKE as HsChar {
        p = CHARLIKE_CLOSURE(c as i32) as *mut StgClosure;
    } else {
        p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
        (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*p).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*p).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*p).header.prof.hp.era = user_era;
        }

        (&raw mut (*p).header.info).store((*ghc_hs_iface).Czh_con_info, Ordering::Relaxed);

        let ref mut fresh0 = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(0);
        *fresh0 = c as StgWord as *mut StgClosure as *mut StgClosure_;
    }

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt(mut cap: *mut Capability, mut i: HsInt) -> HaskellObj {
    let mut p = null_mut::<StgClosure>();

    if i >= MIN_INTLIKE as HsInt && i <= MAX_INTLIKE as HsInt {
        p = INTLIKE_CLOSURE(i as i32) as *mut StgClosure;
    } else {
        p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
        (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

        if doingLDVProfiling() {
            if doingLDVProfiling() {
                (*p).header.prof.hp.ldvw =
                    (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
            }
        } else if doingRetainerProfiling() {
            (*p).header.prof.hp.trav = 0;
        } else if doingErasProfiling() {
            (*p).header.prof.hp.era = user_era;
        }

        (&raw mut (*p).header.info).store((*ghc_hs_iface).Izh_con_info, Ordering::Relaxed);
        *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgInt) = i as StgInt;
    }

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt8(mut cap: *mut Capability, mut i: HsInt8) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).I8zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgInt8) = i as StgInt8;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt16(mut cap: *mut Capability, mut i: HsInt16) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).I16zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgInt16) = i as StgInt16;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt32(mut cap: *mut Capability, mut i: HsInt32) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).I32zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgInt32) = i as StgInt32;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkInt64(mut cap: *mut Capability, mut i: HsInt64) -> HaskellObj {
    let mut p = allocate(
        cap,
        CONSTR_sizeW(
            0,
            (size_of::<StgInt64>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32,
        ) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).I64zh_con_info, Ordering::Relaxed);

    ASSIGN_Int64(
        (&raw mut (*p).payload as *mut *mut StgClosure_).offset(0) as *mut *mut StgClosure_
            as *mut W_,
        i as StgInt64,
    );

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord(mut cap: *mut Capability, mut i: HsWord) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).Wzh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgWord) = i as StgWord;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord8(mut cap: *mut Capability, mut w: HsWord8) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).W8zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgWord8) = w as StgWord8;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord16(mut cap: *mut Capability, mut w: HsWord16) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).W16zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgWord16) = w as StgWord16;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord32(mut cap: *mut Capability, mut w: HsWord32) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).W32zh_con_info, Ordering::Relaxed);
    *(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut StgWord32) = w as StgWord32;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkWord64(mut cap: *mut Capability, mut w: HsWord64) -> HaskellObj {
    let mut p = allocate(
        cap,
        CONSTR_sizeW(
            0,
            (size_of::<StgWord64>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32,
        ) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).W64zh_con_info, Ordering::Relaxed);

    ASSIGN_Word64(
        (&raw mut (*p).payload as *mut *mut StgClosure_).offset(0) as *mut *mut StgClosure_
            as *mut W_,
        w as StgWord64,
    );

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFloat(mut cap: *mut Capability, mut f: HsFloat) -> HaskellObj {
    let mut p = allocate(cap, CONSTR_sizeW(0, 1) as W_) as *mut StgClosure;
    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).Fzh_con_info, Ordering::Relaxed);
    ASSIGN_FLT(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut W_, f);

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkDouble(mut cap: *mut Capability, mut d: HsDouble) -> HaskellObj {
    let mut p = allocate(
        cap,
        CONSTR_sizeW(
            0,
            (size_of::<StgDouble>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32,
        ) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).Dzh_con_info, Ordering::Relaxed);
    ASSIGN_DBL(&raw mut (*p).payload as *mut *mut StgClosure_ as *mut W_, d);

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkStablePtr(
    mut cap: *mut Capability,
    mut s: HsStablePtr,
) -> HaskellObj {
    let mut p = allocate(
        cap,
        (size_of::<StgHeader>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize)
            .wrapping_add(1 as usize) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).StablePtr_con_info, Ordering::Relaxed);

    let ref mut fresh3 = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(0);
    *fresh3 = s as *mut StgClosure as *mut StgClosure_;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkPtr(mut cap: *mut Capability, mut a: HsPtr) -> HaskellObj {
    let mut p = allocate(
        cap,
        (size_of::<StgHeader>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize)
            .wrapping_add(1 as usize) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).Ptr_con_info, Ordering::Relaxed);

    let ref mut fresh1 = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(0);
    *fresh1 = a as *mut StgClosure as *mut StgClosure_;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkFunPtr(mut cap: *mut Capability, mut a: HsFunPtr) -> HaskellObj {
    let mut p = allocate(
        cap,
        (size_of::<StgHeader>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize)
            .wrapping_add(1 as usize) as W_,
    ) as *mut StgClosure;

    (*p).header.prof.ccs = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*p).header.prof.hp.ldvw = (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*p).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*p).header.prof.hp.era = user_era;
    }

    (&raw mut (*p).header.info).store((*ghc_hs_iface).FunPtr_con_info, Ordering::Relaxed);

    let ref mut fresh2 = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(0);
    *fresh2 = transmute::<HsFunPtr, *mut StgClosure>(a) as *mut StgClosure_;

    return TAG_CLOSURE(1, p) as HaskellObj;
}

#[ffi(compiler, ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkBool(mut cap: *mut Capability, mut b: HsBool) -> HaskellObj {
    if b != 0 {
        return TAG_CLOSURE(2, (*ghc_hs_iface).True_closure) as HaskellObj;
    } else {
        return TAG_CLOSURE(1, (*ghc_hs_iface).False_closure) as HaskellObj;
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_mkString(mut cap: *mut Capability, mut s: *mut c_char) -> HaskellObj {
    return rts_apply(
        cap,
        (*ghc_hs_iface).unpackCString_closure as HaskellObj,
        rts_mkPtr(cap, s as HsPtr),
    );
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_apply(
    mut cap: *mut Capability,
    mut f: HaskellObj,
    mut arg: HaskellObj,
) -> HaskellObj {
    let mut ap = null_mut::<StgThunk>();

    ap = allocate(
        cap,
        (size_of::<StgThunk>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize)
            .wrapping_add(2 as usize) as W_,
    ) as *mut StgThunk;

    let ref mut fresh11 = (*(ap as *mut StgClosure)).header.prof.ccs;
    *fresh11 = &raw mut CCS_MAIN as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(ap as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(ap as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(ap as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*ap).header.info).store(
        &raw const stg_ap_2_upd_info as *mut StgInfoTable,
        Ordering::Relaxed,
    );

    let ref mut fresh4 = *(&raw mut (*ap).payload as *mut *mut StgClosure_).offset(0);
    *fresh4 = f as *mut StgClosure_;

    let ref mut fresh5 = *(&raw mut (*ap).payload as *mut *mut StgClosure_).offset(1);
    *fresh5 = arg as *mut StgClosure_;

    return ap as HaskellObj;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getChar(mut p: HaskellObj) -> HsChar {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_)
        .offset(0) as StgWord as HsChar;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt(mut p: HaskellObj) -> HsInt {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsInt);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt8(mut p: HaskellObj) -> HsInt8 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsInt8);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt16(mut p: HaskellObj) -> HsInt16 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsInt16);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt32(mut p: HaskellObj) -> HsInt32 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsInt32);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getInt64(mut p: HaskellObj) -> HsInt64 {
    return PK_Int64(
        (&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
            p as *mut StgClosure,
        ))
        .payload as *mut *mut StgClosure_)
            .offset(0) as *mut *mut StgClosure_ as *mut W_,
    ) as HsInt64;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord(mut p: HaskellObj) -> HsWord {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsWord);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord8(mut p: HaskellObj) -> HsWord8 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsWord8);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord16(mut p: HaskellObj) -> HsWord16 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsWord16);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord32(mut p: HaskellObj) -> HsWord32 {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_ as *mut HsWord32);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getWord64(mut p: HaskellObj) -> HsWord64 {
    return PK_Word64(
        (&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
            p as *mut StgClosure,
        ))
        .payload as *mut *mut StgClosure_)
            .offset(0) as *mut *mut StgClosure_ as *mut W_,
    ) as HsWord64;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFloat(mut p: HaskellObj) -> HsFloat {
    return PK_FLT(
        &raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
            p as *mut StgClosure,
        ))
        .payload as *mut *mut StgClosure_ as *mut W_,
    );
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getDouble(mut p: HaskellObj) -> HsDouble {
    return PK_DBL(
        &raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
            p as *mut StgClosure,
        ))
        .payload as *mut *mut StgClosure_ as *mut W_,
    );
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getStablePtr(mut p: HaskellObj) -> HsStablePtr {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_)
        .offset(0) as HsStablePtr;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getPtr(mut p: HaskellObj) -> HsPtr {
    return *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
        p as *mut StgClosure,
    ))
    .payload as *mut *mut StgClosure_)
        .offset(0) as *mut Capability as HsPtr;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getFunPtr(mut p: HaskellObj) -> HsFunPtr {
    return transmute::<*mut c_void, HsFunPtr>(
        *(&raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
            p as *mut StgClosure,
        ))
        .payload as *mut *mut StgClosure_)
            .offset(0) as *mut c_void,
    );
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getBool(mut p: HaskellObj) -> HsBool {
    let tag = GET_CLOSURE_TAG(p as *const StgClosure) as StgWord;

    if tag > 0 {
        return tag.wrapping_sub(1 as StgWord) as HsBool;
    }

    let mut info = null::<StgInfoTable>();
    info = get_itbl(UNTAG_CONST_CLOSURE(p as *const StgClosure));

    if (*info).srt == 0 {
        return 0;
    } else {
        return 1;
    };
}

#[inline]
unsafe fn pushClosure(mut tso: *mut StgTSO, mut c: StgWord) {
    (*(*tso).stackobj).sp = (*(*tso).stackobj).sp.offset(-1);
    *(*(*tso).stackobj).sp.offset(0) = c as StgWord;
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn createGenThread(
    mut cap: *mut Capability,
    mut stack_size: W_,
    mut closure: *mut StgClosure,
) -> *mut StgTSO {
    let mut t = null_mut::<StgTSO>();
    t = createThread(cap, stack_size);
    pushClosure(t, closure as StgWord);
    pushClosure(t, &raw const stg_enter_info as StgWord);

    return t;
}

unsafe fn createIOThread(
    mut cap: *mut Capability,
    mut stack_size: W_,
    mut closure: *mut StgClosure,
) -> *mut StgTSO {
    let mut t = null_mut::<StgTSO>();
    t = createThread(cap, stack_size);
    pushClosure(t, &raw const stg_ap_v_info as StgWord);
    pushClosure(t, closure as StgWord);
    pushClosure(t, &raw const stg_enter_info as StgWord);

    return t;
}

unsafe fn createStrictIOThread(
    mut cap: *mut Capability,
    mut stack_size: W_,
    mut closure: *mut StgClosure,
) -> *mut StgTSO {
    let mut t = null_mut::<StgTSO>();
    t = createThread(cap, stack_size);
    pushClosure(t, &raw const stg_forceIO_info as StgWord);
    pushClosure(t, &raw const stg_ap_v_info as StgWord);
    pushClosure(t, closure as StgWord);
    pushClosure(t, &raw const stg_enter_info as StgWord);

    return t;
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();

    tso = createGenThread(
        *cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        p as *mut StgClosure,
    );

    scheduleWaitThread(tso, ret, cap);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_eval_(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut stack_size: c_uint,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();
    tso = createGenThread(*cap, stack_size as W_, p as *mut StgClosure);
    scheduleWaitThread(tso, ret, cap);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalIO(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();

    tso = createStrictIOThread(
        *cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        p as *mut StgClosure,
    );

    scheduleWaitThread(tso, ret, cap);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_inCall(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();

    tso = createStrictIOThread(
        *cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        p as *mut StgClosure,
    );

    if (*(**cap).running_task).preferred_capability != -1 {
        if ((**cap).no
            == ((*(**cap).running_task).preferred_capability as u32)
                .wrapping_rem(enabled_capabilities)) as i32 as i64
            != 0
        {
        } else {
            _assertFail(c"rts/RtsAPI.c".as_ptr(), 490);
        }

        (*tso).flags |= TSO_LOCKED as StgWord32;
    }

    scheduleWaitThread(tso, ret, cap);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIOMain(
    mut cap: *mut *mut Capability,
    mut s: HsStablePtr,
    mut ret: *mut HsStablePtr,
) {
    let mut tso = null_mut::<StgTSO>();
    let mut p = null_mut::<StgClosure>();
    let mut r = null_mut::<StgClosure>();
    let mut w = null_mut::<StgClosure>();
    let mut stat = NoStatus;
    p = deRefStablePtr(s as StgStablePtr) as *mut StgClosure;
    w = rts_apply(
        *cap,
        (*ghc_hs_iface).runMainIO_closure as HaskellObj,
        p as HaskellObj,
    ) as *mut StgClosure;
    tso = createStrictIOThread(*cap, RtsFlags.GcFlags.initialStkSize as W_, w);
    (*tso).flags |= (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
    scheduleWaitThread(tso, &raw mut r, cap);
    stat = rts_getSchedStatus(*cap);

    if stat as u32 == Success as i32 as u32 && !ret.is_null() {
        if !r.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/RtsAPI.c".as_ptr(), 521);
        }

        *ret = getStablePtr(r as StgPtr) as HsStablePtr;
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalStableIO(
    mut cap: *mut *mut Capability,
    mut s: HsStablePtr,
    mut ret: *mut HsStablePtr,
) {
    let mut tso = null_mut::<StgTSO>();
    let mut p = null_mut::<StgClosure>();
    let mut r = null_mut::<StgClosure>();
    let mut stat = NoStatus;
    p = deRefStablePtr(s as StgStablePtr) as *mut StgClosure;
    tso = createStrictIOThread(*cap, RtsFlags.GcFlags.initialStkSize as W_, p);
    (*tso).flags |= (TSO_BLOCKEX | TSO_INTERRUPTIBLE) as StgWord32;
    scheduleWaitThread(tso, &raw mut r, cap);
    stat = rts_getSchedStatus(*cap);

    if stat as u32 == Success as i32 as u32 && !ret.is_null() {
        if !r.is_null() as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/RtsAPI.c".as_ptr(), 549);
        }

        *ret = getStablePtr(r as StgPtr) as HsStablePtr;
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();

    tso = createIOThread(
        *cap,
        RtsFlags.GcFlags.initialStkSize as W_,
        p as *mut StgClosure,
    );

    scheduleWaitThread(tso, ret, cap);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_evalLazyIO_(
    mut cap: *mut *mut Capability,
    mut p: HaskellObj,
    mut stack_size: c_uint,
    mut ret: *mut HaskellObj,
) {
    let mut tso = null_mut::<StgTSO>();
    tso = createIOThread(*cap, stack_size as W_, p as *mut StgClosure);
    scheduleWaitThread(tso, ret, cap);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_checkSchedStatus(mut site: *mut c_char, mut cap: *mut Capability) {
    let mut rc = (*(*(*cap).running_task).incall).rstat;

    match rc as u32 {
        0 => {
            errorBelch(c"%s: no status, not ok".as_ptr(), site);
            stg_exit(EXIT_FAILURE);
        }
        1 => return,
        2 => {
            errorBelch(c"%s: uncaught exception".as_ptr(), site);
            stg_exit(EXIT_FAILURE);
        }
        3 => {
            errorBelch(c"%s: interrupted".as_ptr(), site);
            rts_unlock(cap);
            shutdownThread();
        }
        4 => {
            errorBelch(c"%s: out of memory".as_ptr(), site);
            stg_exit(EXIT_FAILURE);
        }
        _ => {
            errorBelch(
                c"%s: SchedulerStatus code (%d) unknown".as_ptr(),
                site,
                rc as u32,
            );

            stg_exit(EXIT_FAILURE);
        }
    };
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_getSchedStatus(mut cap: *mut Capability) -> SchedulerStatus {
    return (*(*(*cap).running_task).incall).rstat;
}

static mut rts_pausing_task: *mut Task = null_mut::<Task>();

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_lock() -> *mut Capability {
    let mut cap = null_mut::<Capability>();
    let mut task = null_mut::<Task>();
    task = newBoundTask();

    if (*task).running_finalizers {
        errorBelch(
            c"error: a C finalizer called back into Haskell.\n   This was previously allowed, but is disallowed in GHC 6.10.2 and later.\n   To create finalizers that may call back into Haskell, use\n   Foreign.Concurrent.newForeignPtr instead of Foreign.newForeignPtr."
                .as_ptr(),
        );

        stg_exit(EXIT_FAILURE);
    }

    if rts_pausing_task == task {
        errorBelch(
            c"error: rts_lock: The RTS is already paused by this thread.\n   There is no need to call rts_lock if you have already called rts_pause."
                .as_ptr(),
        );

        stg_exit(EXIT_FAILURE);
    }

    cap = null_mut::<Capability>();
    waitForCapability(&raw mut cap, task);

    if (*(*task).incall).prev_stack.is_null() {
        traceTaskCreate(task, cap);
    }

    return cap;
}

#[ffi(compiler, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_unlock(mut cap: *mut Capability) {
    let mut task = null_mut::<Task>();
    task = (*cap).running_task;

    if (!(*cap).running_task.is_null() && (*cap).running_task == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
    }

    if ((*task).cap == cap) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
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
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
    }

    if (if (*cap).suspended_ccalls.is_null() {
        ((*cap).n_suspended_ccalls == 0) as i32
    } else {
        1
    } != 0) as i32 as i64
        != 0
    {
    } else {
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
    }

    if (myTask() == task) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
    }

    if ((*task).id == osThreadId()) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/RtsAPI.c".as_ptr(), 682);
    }

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/RtsAPI.c".as_ptr(),
            694,
            __r,
        );
    }

    releaseCapability_(cap, false);
    exitMyTask();

    if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
        barf(
            c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
            c"rts/RtsAPI.c".as_ptr(),
            699,
        );
    }

    if (*task).incall.is_null() {
        traceTaskDelete(task);
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn pauseTokenCapability(mut pauseToken: *mut PauseToken) -> *mut Capability {
    return (*pauseToken).capability;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_pause() -> *mut PauseToken {
    if RtsFlags.GcFlags.useNonmoving {
        nonmovingBlockConcurrentMark(true);
    }

    let mut task = getMyTask();

    if rts_pausing_task == task {
        errorBelch(c"error: rts_pause: This thread has already paused the RTS.".as_ptr());

        stg_exit(EXIT_FAILURE);
    }

    if !(*task).cap.is_null()
        && (&raw mut (*(*task).cap).running_task).load(Ordering::Relaxed) == task
    {
        errorBelch(if (*(*task).cap).in_haskell as i32 != 0 {
            c"error: rts_pause: attempting to pause via an unsafe FFI call.\n   Perhaps a 'foreign import unsafe' should be 'safe'?"
                    .as_ptr()
        } else {
            c"error: rts_pause: attempting to pause from a Task that owns a capability.\n   Have you already acquired a capability e.g. with rts_lock?"
                    .as_ptr()
        });

        stg_exit(EXIT_FAILURE);
    }

    task = newBoundTask();
    stopAllCapabilities(null_mut::<*mut Capability>(), task);
    rts_pausing_task = task;

    let mut token =
        stgMallocBytes(size_of::<PauseToken>() as usize, c"rts_pause".as_ptr()) as *mut PauseToken;

    (*token).capability = (*task).cap as *mut Capability;

    return token;
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_resume(mut pauseToken: *mut PauseToken) {
    assert_isPausedOnMyTask(c"rts_resume".as_ptr());

    let mut task = getMyTask();
    rts_pausing_task = null_mut::<Task>();
    releaseAllCapabilities(getNumCapabilities() as u32, null_mut::<Capability>(), task);
    exitMyTask();
    stgFree(pauseToken as *mut c_void);

    if RtsFlags.GcFlags.useNonmoving {
        nonmovingUnblockConcurrentMark();
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_isPaused() -> bool {
    return !rts_pausing_task.is_null();
}

unsafe fn assert_isPausedOnMyTask(mut functionName: *const c_char) {
    let mut task = getMyTask();

    if rts_pausing_task.is_null() {
        errorBelch(
            c"error: %s: the rts is not paused. Did you forget to call rts_pause?".as_ptr(),
            functionName,
        );

        stg_exit(EXIT_FAILURE);
    }

    if task != rts_pausing_task {
        errorBelch(
            c"error: %s: called from a different OS thread than rts_pause.".as_ptr(),
            functionName,
        );

        stg_exit(EXIT_FAILURE);
    }

    let mut i = 0;

    while i < getNumCapabilities() {
        let mut cap = getCapability(i as u32);

        if (*cap).running_task != task {
            errorBelch(
                c"error: %s: the pausing thread does not own all capabilities.\n   Have you manually released a capability after calling rts_pause?"
                    .as_ptr(),
                functionName,
            );

            stg_exit(EXIT_FAILURE);
        }

        i = i.wrapping_add(1);
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listThreads(mut cb: ListThreadsCb, mut user: *mut c_void) {
    assert_isPausedOnMyTask(c"rts_listThreads".as_ptr());

    let mut g: u32 = 0;

    while g < RtsFlags.GcFlags.generations {
        let mut tso = (*generations.offset(g as isize)).threads;

        while tso != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
            cb.expect("non-null function pointer")(user, tso);
            tso = (*tso).global_link as *mut StgTSO;
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn list_roots_helper(mut user: *mut c_void, mut p: *mut *mut StgClosure) {
    let mut ctx = user as *mut list_roots_ctx;
    (*ctx).cb.expect("non-null function pointer")((*ctx).user, *p);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rts_listMiscRoots(mut cb: ListRootsCb, mut user: *mut c_void) {
    assert_isPausedOnMyTask(c"rts_listMiscRoots".as_ptr());

    let mut ctx = list_roots_ctx {
        cb: None,
        user: null_mut::<c_void>(),
    };

    ctx.cb = cb;
    ctx.user = user;

    threadStableNameTable(
        Some(list_roots_helper as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        &raw mut ctx as *mut c_void,
    );

    threadStablePtrTable(
        Some(list_roots_helper as unsafe extern "C" fn(*mut c_void, *mut *mut StgClosure) -> ()),
        &raw mut ctx as *mut c_void,
    );
}

unsafe fn rts_done() {
    freeMyTask();
}

#[ffi(docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_try_putmvar(mut capability: c_int, mut mvar: HsStablePtr) {
    hs_try_putmvar_with_value(
        capability,
        mvar,
        TAG_CLOSURE(1, (*ghc_hs_iface).Z0T_closure),
    );
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_try_putmvar_with_value(
    mut capability: c_int,
    mut mvar: HsStablePtr,
    mut value: *mut StgClosure,
) {
    let mut task = getMyTask();
    let mut cap = null_mut::<Capability>();
    let mut task_old_cap = null_mut::<Capability>();

    if capability < 0 {
        capability = (*task).preferred_capability;

        if capability < 0 {
            capability = 0;
        }
    }

    cap = getCapability((capability as u32).wrapping_rem(enabled_capabilities));

    let mut __r = pthread_mutex_lock(&raw mut (*cap).lock);

    if __r != 0 {
        barf(
            c"ACQUIRE_LOCK failed (%s:%d): %d".as_ptr(),
            c"rts/RtsAPI.c".as_ptr(),
            990,
            __r,
        );
    }

    if (*cap).running_task.is_null() {
        (*cap).running_task = task;
        task_old_cap = (*task).cap as *mut Capability;
        (*task).cap = cap as *mut Capability_;

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/RtsAPI.c".as_ptr(),
                996,
            );
        }

        performTryPutMVar(
            cap,
            deRefStablePtr(mvar as StgStablePtr) as *mut StgMVar,
            value,
        );

        freeStablePtr(mvar as StgStablePtr);
        releaseCapability(cap);
        (*task).cap = task_old_cap as *mut Capability_;
    } else {
        let mut p = stgMallocBytes(size_of::<PutMVar>() as usize, c"hs_try_putmvar".as_ptr())
            as *mut PutMVar;

        (*p).mvar = mvar as StgStablePtr;
        (*p).link = (*cap).putMVars as *mut PutMVar_;
        (*cap).putMVars = p as *mut PutMVar_;

        if pthread_mutex_unlock(&raw mut (*cap).lock) != 0 {
            barf(
                c"RELEASE_LOCK: I do not own this lock: %s %d".as_ptr(),
                c"rts/RtsAPI.c".as_ptr(),
                1013,
            );
        }
    };
}
