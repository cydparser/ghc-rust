use crate::prelude::*;
use crate::rts::prof::ccs::CostCentreStack;
use crate::rts::storage::closures::{
    MessageBlackHole_, MessageThrowTo_, MessageWakeup_, StgArrBytes, StgBlockingQueue_, StgClosure,
    StgHeader, StgTRecHeader_,
};
use crate::rts_api::Capability_;
use crate::stg::types::{
    StgInt, StgInt64, StgPtr, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};

#[cfg(test)]
mod tests;

pub(crate) const STACK_DIRTY: u32 = 1;

pub(crate) const STACK_SANE: u32 = 64;

/// - GHC_PLACES: {libraries}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgTSOProfInfo {
    pub cccs: *mut CostCentreStack,
}

#[cfg(feature = "sys")]
impl From<StgTSOProfInfo> for sys::StgTSOProfInfo {
    fn from(x: StgTSOProfInfo) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgThreadID = StgWord64;

pub(crate) type StgThreadReturnCode = c_uint;

#[repr(C)]
pub(crate) union StgTSOBlockInfo {
    closure: *mut StgClosure,
    prev: *mut StgTSO,
    bh: *mut MessageBlackHole_,
    throwto: *mut MessageThrowTo_,
    wakeup: *mut MessageWakeup_,
    fd: StgInt,
    target: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgTSOBlockInfo> for sys::StgTSOBlockInfo {
    fn from(x: StgTSOBlockInfo) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgTSO = StgTSO_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgTSO_ {
    header: StgHeader,
    _link: *mut StgTSO_,
    global_link: *mut StgTSO_,
    stackobj: *mut StgStack_,
    what_next: StgWord16,
    flags: StgWord32,
    why_blocked: StgWord32,
    block_info: StgTSOBlockInfo,
    id: StgThreadID,
    saved_errno: StgWord32,
    dirty: StgWord32,
    bound: *mut InCall_,
    cap: *mut Capability_,
    trec: *mut StgTRecHeader_,
    label: *mut StgArrBytes,
    blocked_exceptions: *mut MessageThrowTo_,
    bq: *mut StgBlockingQueue_,
    alloc_limit: StgInt64,
    tot_stack_size: StgWord32,
}

#[cfg(feature = "sys")]
impl From<StgTSO_> for sys::StgTSO_ {
    fn from(x: StgTSO_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgTSOPtr = *mut StgTSO_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgStack_ {
    header: StgHeader,
    stack_size: StgWord32,
    dirty: StgWord8,
    marking: StgWord8,
    sp: StgPtr,
    stack: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgStack_> for sys::StgStack_ {
    fn from(x: StgStack_) -> Self {
        unsafe { transmute(x) }
    }
}

/// - GHC_PLACES: {libraries, testsuite}
pub type StgStack = StgStack_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InCall_ {
    _address: u8,
}

#[cfg(feature = "sys")]
impl From<InCall_> for sys::InCall_ {
    fn from(x: InCall_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for InCall_ {
    fn arbitrary(g: &mut Gen) -> Self {
        InCall_ {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
