use crate::capability::Capability_;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::closures::{
    MessageBlackHole_, MessageThrowTo_, MessageWakeup_, StgArrBytes, StgBlockingQueue, StgClosure,
    StgHeader, StgTRecHeader_,
};
use crate::ffi::stg::types::{StgInt, StgInt64, StgPtr, StgWord8, StgWord16, StgWord32, StgWord64};
use crate::prelude::*;
pub use crate::task::InCall_;

#[cfg(test)]
mod tests;

pub(crate) const STACK_DIRTY: u32 = 1;

pub(crate) const STACK_SANE: u32 = 64;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct StgTSOProfInfo {
    pub cccs: *mut CostCentreStack,
}

#[ffi(compiler, ghc_lib, libraries, testsuite)]
pub type StgThreadID = StgWord64;

pub(crate) type StgThreadReturnCode = u32;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
#[repr(C)]
pub union StgTSOBlockInfo {
    pub closure: *mut StgClosure,
    pub prev: *mut StgTSO,
    pub bh: *mut MessageBlackHole_,
    pub throwto: *mut MessageThrowTo_,
    pub wakeup: *mut MessageWakeup_,
    pub fd: StgInt,
}

#[ffi(ghc_lib, testsuite)]
pub type StgTSO = StgTSO_;

#[ffi(compiler, ghc_lib, libraries, testsuite)]
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
    pub bq: *mut StgBlockingQueue,
    pub alloc_limit: StgInt64,
    pub tot_stack_size: StgWord32,
    pub prof: StgTSOProfInfo,
}

pub(crate) type StgTSOPtr = *mut StgTSO_;

#[ffi(compiler, ghc_lib, testsuite)]
#[repr(C)]
pub struct StgStack_ {
    pub header: StgHeader,
    pub stack_size: StgWord32,
    pub dirty: StgWord8,
    pub marking: StgWord8,
    pub sp: StgPtr,
    pub stack: __IncompleteArrayField<StgWord>,
}

#[ffi(ghc_lib, testsuite)]
pub type StgStack = StgStack_;
