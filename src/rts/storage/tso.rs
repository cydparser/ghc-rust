use std::{ffi::c_uint, mem::transmute};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::{
    rts::{
        capability::Capability_,
        prof::ccs::CostCentreStack,
        storage::closures::{
            MessageBlackHole_, MessageThrowTo_, MessageWakeup_, StgArrBytes, StgBlockingQueue_,
            StgHeader, StgTRecHeader_,
        },
        types::{StgClosure, StgTSO},
    },
    stg::types::{StgInt, StgInt64, StgPtr, StgWord, StgWord16, StgWord32, StgWord64, StgWord8},
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);

pub(crate) const FMT_StgThreadID: &[u8; 3] = b"lu\0";

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
#[derive(Copy, Clone)]
pub(crate) union StgTSOBlockInfo {
    pub closure: *mut StgClosure,
    pub prev: *mut StgTSO,
    pub bh: *mut MessageBlackHole_,
    pub throwto: *mut MessageThrowTo_,
    pub wakeup: *mut MessageWakeup_,
    pub fd: StgInt,
    pub target: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgTSOBlockInfo> for sys::StgTSOBlockInfo {
    fn from(x: StgTSOBlockInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
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

#[repr(C)]
pub struct StgStack_ {
    pub header: StgHeader,
    pub stack_size: StgWord32,
    pub dirty: StgWord8,
    pub marking: StgWord8,
    pub sp: StgPtr,
    pub stack: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgStack_> for sys::StgStack_ {
    fn from(x: StgStack_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgStack = StgStack_;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_TSO(cap: *mut Capability_, tso: *mut StgTSO) {
    unsafe { sys::dirty_TSO(transmute(cap), transmute(tso)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOLink(cap: *mut Capability_, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe { sys::setTSOLink(transmute(cap), transmute(tso), transmute(target)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOPrev(cap: *mut Capability_, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe { sys::setTSOPrev(transmute(cap), transmute(tso), transmute(target)) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_STACK(cap: *mut Capability_, stack: *mut StgStack) {
    unsafe { sys::dirty_STACK(transmute(cap), transmute(stack)) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct InCall_ {
    pub _address: u8,
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
