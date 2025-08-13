use crate::prelude::*;
use crate::rts::capability::{Capability, Capability_};
use crate::rts::prof::ccs::CostCentreStack;
use crate::rts::storage::closures::{
    MessageBlackHole_, MessageThrowTo_, MessageWakeup_, StgArrBytes, StgBlockingQueue_, StgClosure,
    StgHeader, StgTRecHeader_,
};
use crate::stg::types::{
    StgInt, StgInt64, StgPtr, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};

#[cfg(test)]
mod tests;

pub const STACK_DIRTY: u32 = 1;

pub(crate) const STACK_SANE: u32 = 64;

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
///cbindgen:no-export
pub union StgTSOBlockInfo {
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

#[repr(C)]
///cbindgen:no-export
pub struct StgTSO_ {
    pub header: StgHeader,
    pub _link: *mut StgTSO_,
    pub global_link: *mut StgTSO_,
    pub stackobj: *mut StgStack_,
    pub what_next: StgWord16,
    pub flags: StgWord32,
    pub why_blocked: StgWord32,
    pub block_info: StgTSOBlockInfo,
    pub id: StgThreadID,
    pub saved_errno: StgWord32,
    pub dirty: StgWord32,
    pub bound: *mut InCall_,
    pub cap: *mut Capability_,
    pub trec: *mut StgTRecHeader_,
    pub label: *mut StgArrBytes,
    pub blocked_exceptions: *mut MessageThrowTo_,
    pub bq: *mut StgBlockingQueue_,
    pub alloc_limit: StgInt64,
    pub tot_stack_size: StgWord32,
}

#[cfg(feature = "sys")]
impl From<StgTSO_> for sys::StgTSO_ {
    fn from(x: StgTSO_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgTSOPtr = *mut StgTSO_;

pub type StgStack = StgStack_;

#[repr(C)]
///cbindgen:no-export
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

#[instrument]
pub(crate) unsafe fn dirty_TSO(cap: *mut Capability, tso: *mut StgTSO) {
    unsafe { sys::dirty_TSO(cap as *mut sys::Capability, tso as *mut sys::StgTSO) }
}

#[instrument]
pub(crate) unsafe fn setTSOLink(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        sys::setTSOLink(
            cap as *mut sys::Capability,
            tso as *mut sys::StgTSO,
            target as *mut sys::StgTSO,
        )
    }
}

#[instrument]
pub(crate) unsafe fn setTSOPrev(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        sys::setTSOPrev(
            cap as *mut sys::Capability,
            tso as *mut sys::StgTSO,
            target as *mut sys::StgTSO,
        )
    }
}

#[instrument]
pub(crate) unsafe fn dirty_STACK(cap: *mut Capability, stack: *mut StgStack) {
    unsafe { sys::dirty_STACK(cap as *mut sys::Capability, stack as *mut sys::StgStack) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
