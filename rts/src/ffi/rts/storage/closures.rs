use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::ffi::rts::storage::tso::{StgStack_, StgTSO, StgTSO_};
use crate::ffi::stg::types::{StgHalfWord, StgInt, StgPtr, StgWord};
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TREC_CHUNK_NUM_ENTRIES: u32 = 16;

/// cbindgen:no-export
#[repr(C)]
pub struct StgProfHeader {
    ccs: *mut CostCentreStack,
    hp: StgProfHeader__bindgen_ty_1,
}

#[repr(C)]
pub(crate) union StgProfHeader__bindgen_ty_1 {
    trav: StgWord,
    ldvw: StgWord,
    era: StgWord,
}

/// cbindgen:no-export
#[repr(C)]
#[cfg_attr(test, derive(Clone))]
pub struct StgSMPThunkHeader {
    pad: StgWord,
}

#[cfg(test)]
impl Arbitrary for StgSMPThunkHeader {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSMPThunkHeader {
            pad: Arbitrary::arbitrary(g),
        }
    }
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgHeader {
    pub info: *const StgInfoTable,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgThunkHeader {
    info: *const StgInfoTable,
    smp: StgSMPThunkHeader,
}

#[ffi(compiler, driver, ghc_lib, testsuite, utils)]
pub type StgClosure = StgClosure_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgClosure_ {
    header: StgHeader,
    payload: __IncompleteArrayField<*mut StgClosure_>,
}

#[ffi(ghc_lib)]
pub type StgClosurePtr = *mut StgClosure_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgThunk_ {
    header: StgThunkHeader,
    payload: __IncompleteArrayField<*mut StgClosure_>,
}

pub(crate) type StgThunk = StgThunk_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgSelector {
    header: StgThunkHeader,
    selectee: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgPAP {
    header: StgHeader,
    arity: StgHalfWord,
    n_args: StgHalfWord,
    fun: *mut StgClosure,
    payload: __IncompleteArrayField<*mut StgClosure>,
}

#[ffi(compiler)]
#[repr(C)]
pub struct StgAP {
    pub header: StgThunkHeader,
    pub arity: StgHalfWord,
    pub n_args: StgHalfWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgAP_STACK {
    header: StgThunkHeader,
    size: StgWord,
    fun: *mut StgClosure,
    payload: __IncompleteArrayField<*mut StgClosure>,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgInd {
    pub header: StgHeader,
    pub indirectee: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgIndStatic {
    header: StgHeader,
    indirectee: *mut StgClosure,
    static_link: *mut StgClosure,
    saved_info: *const StgInfoTable,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgBlockingQueue_ {
    header: StgHeader,
    link: *mut StgBlockingQueue_,
    bh: *mut StgClosure,
    owner: *mut StgTSO,
    queue: *mut MessageBlackHole_,
}

pub(crate) type StgBlockingQueue = StgBlockingQueue_;

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub struct StgArrBytes {
    pub header: StgHeader,
    pub bytes: StgWord,
    pub payload: __IncompleteArrayField<StgWord>,
}

/// cbindgen:no-export
#[repr(C)]
pub struct _StgMutArrPtrs {
    header: StgHeader,
    ptrs: StgWord,
    size: StgWord,
    payload: __IncompleteArrayField<*mut StgClosure>,
}

#[ffi(compiler, ghc_lib)]
pub type StgMutArrPtrs = _StgMutArrPtrs;

/// cbindgen:no-export
#[repr(C)]
pub struct StgSmallMutArrPtrs {
    header: StgHeader,
    ptrs: StgWord,
    payload: __IncompleteArrayField<*mut StgClosure>,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgMutVar {
    header: StgHeader,
    var: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _StgUpdateFrame {
    header: StgHeader,
    updatee: *mut StgClosure,
}

#[ffi(ghc_lib)]
pub type StgUpdateFrame = _StgUpdateFrame;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _StgOrigThunkInfoFrame {
    header: StgHeader,
    info_ptr: *mut StgInfoTable,
}

pub(crate) type StgOrigThunkInfoFrame = _StgOrigThunkInfoFrame;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgKeepAliveFrame {
    header: StgHeader,
    c: *mut StgClosure,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgCatchFrame {
    pub header: StgHeader,
    pub handler: *mut StgClosure,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgUnderflowFrame {
    pub info: *const StgInfoTable,
    pub next_chunk: *mut StgStack_,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgStopFrame {
    pub header: StgHeader,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgDeadThreadFrame {
    header: StgHeader,
    result: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgAnnFrame {
    header: StgHeader,
    ann: *mut StgClosure,
}

#[ffi(ghc_lib)]
#[repr(C)]
pub struct StgRetFun {
    pub info: *const StgInfoTable,
    pub size: StgWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgIntCharlikeClosure {
    pub(crate) header: StgHeader,
    pub(crate) data: StgWord,
}

/// cbindgen:no-export
#[repr(C)]
pub struct _StgStableName {
    header: StgHeader,
    sn: StgWord,
}

pub(crate) type StgStableName = _StgStableName;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct _StgWeak {
    header: StgHeader,
    cfinalizers: *mut StgClosure,
    key: *mut StgClosure,
    value: *mut StgClosure,
    finalizer: *mut StgClosure,
    link: *mut _StgWeak,
}

pub(crate) type StgWeak = _StgWeak;

/// cbindgen:no-export
#[repr(C)]
pub struct _StgCFinalizerList {
    header: StgHeader,
    link: *mut StgClosure,
    fptr: Option<unsafe extern "C" fn()>,
    ptr: *mut c_void,
    eptr: *mut c_void,
    flag: StgWord,
}

pub(crate) type StgCFinalizerList = _StgCFinalizerList;

#[ffi(compiler, ghc_lib)]
#[repr(C)]
pub struct StgBCO {
    pub header: StgHeader,
    pub instrs: *mut StgArrBytes,
    pub literals: *mut StgArrBytes,
    pub ptrs: *mut StgMutArrPtrs,
    pub arity: StgHalfWord,
    pub size: StgHalfWord,
    pub bitmap: __IncompleteArrayField<StgWord>,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgMVarTSOQueue_ {
    header: StgHeader,
    link: *mut StgMVarTSOQueue_,
    tso: *mut StgTSO_,
}

pub(crate) type StgMVarTSOQueue = StgMVarTSOQueue_;

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgMVar {
    pub header: StgHeader,
    pub head: *mut StgMVarTSOQueue_,
    pub tail: *mut StgMVarTSOQueue_,
    pub value: *mut StgClosure,
}

pub(crate) type StgTRecHeader = StgTRecHeader_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgTVarWatchQueue_ {
    header: StgHeader,
    closure: *mut StgClosure,
    next_queue_entry: *mut StgTVarWatchQueue_,
    prev_queue_entry: *mut StgTVarWatchQueue_,
}

pub(crate) type StgTVarWatchQueue = StgTVarWatchQueue_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgTVar {
    header: StgHeader,
    current_value: *mut StgClosure,
    first_watch_queue_entry: *mut StgTVarWatchQueue,
    num_updates: StgInt,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct TRecEntry {
    tvar: *mut StgTVar,
    expected_value: *mut StgClosure,
    new_value: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgTRecChunk_ {
    header: StgHeader,
    prev_chunk: *mut StgTRecChunk_,
    next_entry_idx: StgWord,
    entries: [TRecEntry; 16usize],
}

pub(crate) type StgTRecChunk = StgTRecChunk_;

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum TRecState {
    TREC_ACTIVE = 0,
    TREC_CONDEMNED = 1,
    TREC_ABORTED = 2,
    TREC_WAITING = 3,
}

#[cfg(feature = "sys")]
impl From<TRecState> for sys::TRecState {
    fn from(v: TRecState) -> Self {
        use TRecState::*;
        match v {
            TREC_ACTIVE => sys::TRecState::TREC_ACTIVE,
            TREC_CONDEMNED => sys::TRecState::TREC_CONDEMNED,
            TREC_ABORTED => sys::TRecState::TREC_ABORTED,
            TREC_WAITING => sys::TRecState::TREC_WAITING,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::TRecState> for TRecState {
    fn from(v: sys::TRecState) -> Self {
        use TRecState::*;
        match v {
            sys::TRecState::TREC_ACTIVE => TREC_ACTIVE,
            sys::TRecState::TREC_CONDEMNED => TREC_CONDEMNED,
            sys::TRecState::TREC_ABORTED => TREC_ABORTED,
            sys::TRecState::TREC_WAITING => TREC_WAITING,
        }
    }
}

impl TryFrom<u32> for TRecState {
    type Error = ();
    fn try_from(d: u32) -> Result<TRecState, ()> {
        use TRecState::*;
        match d {
            0 => Ok(TREC_ACTIVE),
            1 => Ok(TREC_CONDEMNED),
            2 => Ok(TREC_ABORTED),
            3 => Ok(TREC_WAITING),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for TRecState {
    fn arbitrary(g: &mut Gen) -> Self {
        use TRecState::*;
        match usize::arbitrary(g) % 4 {
            0 => TREC_ACTIVE,
            1 => TREC_CONDEMNED,
            2 => TREC_ABORTED,
            3.. => TREC_WAITING,
        }
    }
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgTRecHeader_ {
    header: StgHeader,
    enclosing_trec: *mut StgTRecHeader_,
    current_chunk: *mut StgTRecChunk,
    state: TRecState,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgAtomicallyFrame {
    pub header: StgHeader,
    pub code: *mut StgClosure,
    pub result: *mut StgClosure,
}

#[ffi(ghc_lib)]
#[repr(C)]
#[derive(Debug)]
pub struct StgCatchSTMFrame {
    pub header: StgHeader,
    pub code: *mut StgClosure,
    pub handler: *mut StgClosure,
}

#[ffi(ghc_lib)]
#[repr(C)]
pub struct StgCatchRetryFrame {
    pub header: StgHeader,
    pub running_alt_code: StgWord,
    pub first_code: *mut StgClosure,
    pub alt_code: *mut StgClosure,
}

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct Message_ {
    header: StgHeader,
    link: *mut Message_,
}

#[ffi(compiler, ghc_lib, libraries)]
pub type Message = Message_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct MessageWakeup_ {
    header: StgHeader,
    link: *mut Message,
    tso: *mut StgTSO,
}

pub(crate) type MessageWakeup = MessageWakeup_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct MessageThrowTo_ {
    header: StgHeader,
    link: *mut MessageThrowTo_,
    source: *mut StgTSO,
    target: *mut StgTSO,
    exception: *mut StgClosure,
}

pub(crate) type MessageThrowTo = MessageThrowTo_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct MessageBlackHole_ {
    header: StgHeader,
    link: *mut MessageBlackHole_,
    tso: *mut StgTSO,
    bh: *mut StgClosure,
}

pub(crate) type MessageBlackHole = MessageBlackHole_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct MessageCloneStack_ {
    header: StgHeader,
    link: *mut Message,
    result: *mut StgMVar,
    tso: *mut StgTSO,
}

#[ffi(ghc_lib)]
pub type MessageCloneStack = MessageCloneStack_;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgCompactNFDataBlock_ {
    self_: *mut StgCompactNFDataBlock_,
    owner: *mut StgCompactNFData_,
    next: *mut StgCompactNFDataBlock_,
}

pub(crate) type StgCompactNFDataBlock = StgCompactNFDataBlock_;

/// cbindgen:no-export
#[repr(C)]
pub struct StgCompactNFData_ {
    header: StgHeader,
    totalW: StgWord,
    autoBlockW: StgWord,
    hp: StgPtr,
    hpLim: StgPtr,
    nursery: *mut StgCompactNFDataBlock,
    last: *mut StgCompactNFDataBlock,
    hash: *mut hashtable,
    result: *mut StgClosure,
    link: *mut StgCompactNFData_,
}

pub(crate) type StgCompactNFData = StgCompactNFData_;

pub(crate) type StgPromptTag = *mut StgClosure;

/// cbindgen:no-export
#[repr(C)]
#[derive(Debug)]
pub struct StgPromptFrame {
    header: StgHeader,
    tag: StgPromptTag,
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgContinuation {
    header: StgHeader,
    apply_mask_frame: *const StgInfoTable,
    mask_frame_offset: StgWord,
    stack_size: StgWord,
    stack: __IncompleteArrayField<StgWord>,
}

#[ffi(compiler, utils)]
#[repr(C)]
#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct hashtable {
    pub _address: u8,
}

#[cfg(test)]
impl Arbitrary for hashtable {
    fn arbitrary(g: &mut Gen) -> Self {
        hashtable {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
