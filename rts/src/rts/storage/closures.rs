use crate::prelude::*;
use crate::rts::prof::ccs::CostCentreStack;
use crate::rts::storage::info_tables::StgInfoTable;
use crate::rts::storage::tso::{StgStack_, StgTSO, StgTSO_};
use crate::stg::types::{StgHalfWord, StgInt, StgPtr, StgWord};

#[cfg(test)]
mod tests;

pub(crate) const TREC_CHUNK_NUM_ENTRIES: u32 = 16;

#[repr(C)]
pub struct StgProfHeader {
    pub ccs: *mut CostCentreStack,
    pub hp: StgProfHeader__bindgen_ty_1,
}

#[cfg(feature = "sys")]
impl From<StgProfHeader> for sys::StgProfHeader {
    fn from(x: StgProfHeader) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union StgProfHeader__bindgen_ty_1 {
    trav: StgWord,
    ldvw: StgWord,
    era: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgProfHeader__bindgen_ty_1> for sys::StgProfHeader__bindgen_ty_1 {
    fn from(x: StgProfHeader__bindgen_ty_1) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgProfHeader__bindgen_ty_1 {
    fn arbitrary(g: &mut Gen) -> Self {
        match <usize as Arbitrary>::arbitrary(g) % 3usize {
            0 => StgProfHeader__bindgen_ty_1 {
                trav: Arbitrary::arbitrary(g),
            },
            1 => StgProfHeader__bindgen_ty_1 {
                ldvw: Arbitrary::arbitrary(g),
            },
            2.. => StgProfHeader__bindgen_ty_1 {
                era: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct StgSMPThunkHeader {
    pub pad: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgSMPThunkHeader> for sys::StgSMPThunkHeader {
    fn from(x: StgSMPThunkHeader) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgSMPThunkHeader {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSMPThunkHeader {
            pad: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgHeader {
    pub info: *const StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgHeader> for sys::StgHeader {
    fn from(x: StgHeader) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgThunkHeader {
    pub info: *const StgInfoTable,
    pub smp: StgSMPThunkHeader,
}

#[cfg(feature = "sys")]
impl From<StgThunkHeader> for sys::StgThunkHeader {
    fn from(x: StgThunkHeader) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgClosure = StgClosure_;

#[repr(C)]
#[derive(Debug)]
///cbindgen:no-export
pub struct StgClosure_ {
    header: StgHeader,
    payload: __IncompleteArrayField<*mut StgClosure_>,
}

#[cfg(feature = "sys")]
impl From<StgClosure_> for sys::StgClosure_ {
    fn from(x: StgClosure_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgClosurePtr = *mut StgClosure_;

pub(crate) type StgThunk = StgThunk_;

#[repr(C)]
///cbindgen:no-export
pub struct StgThunk_ {
    header: StgThunkHeader,
    payload: __IncompleteArrayField<*mut StgClosure_>,
}

#[cfg(feature = "sys")]
impl From<StgThunk_> for sys::StgThunk_ {
    fn from(x: StgThunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgSelector {
    pub header: StgThunkHeader,
    pub selectee: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgSelector> for sys::StgSelector {
    fn from(x: StgSelector) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgPAP {
    pub header: StgHeader,
    pub arity: StgHalfWord,
    pub n_args: StgHalfWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<StgPAP> for sys::StgPAP {
    fn from(x: StgPAP) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgAP {
    pub header: StgThunkHeader,
    pub arity: StgHalfWord,
    pub n_args: StgHalfWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<StgAP> for sys::StgAP {
    fn from(x: StgAP) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgAP_STACK {
    pub header: StgThunkHeader,
    pub size: StgWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<StgAP_STACK> for sys::StgAP_STACK {
    fn from(x: StgAP_STACK) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgInd {
    pub header: StgHeader,
    pub indirectee: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgInd> for sys::StgInd {
    fn from(x: StgInd) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgIndStatic {
    header: StgHeader,
    indirectee: *mut StgClosure,
    static_link: *mut StgClosure,
    saved_info: *const StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgIndStatic> for sys::StgIndStatic {
    fn from(x: StgIndStatic) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgBlockingQueue = StgBlockingQueue_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgBlockingQueue_ {
    header: StgHeader,
    link: *mut StgBlockingQueue_,
    bh: *mut StgClosure,
    owner: *mut StgTSO,
    queue: *mut MessageBlackHole_,
}

#[cfg(feature = "sys")]
impl From<StgBlockingQueue_> for sys::StgBlockingQueue_ {
    fn from(x: StgBlockingQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgArrBytes {
    pub header: StgHeader,
    pub bytes: StgWord,
    pub payload: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgArrBytes> for sys::StgArrBytes {
    fn from(x: StgArrBytes) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgMutArrPtrs = _StgMutArrPtrs;

#[repr(C)]
///cbindgen:no-export
pub struct _StgMutArrPtrs {
    header: StgHeader,
    ptrs: StgWord,
    size: StgWord,
    payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<_StgMutArrPtrs> for sys::_StgMutArrPtrs {
    fn from(x: _StgMutArrPtrs) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgSmallMutArrPtrs {
    pub header: StgHeader,
    pub ptrs: StgWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<StgSmallMutArrPtrs> for sys::StgSmallMutArrPtrs {
    fn from(x: StgSmallMutArrPtrs) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgMutVar {
    pub header: StgHeader,
    pub var: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgMutVar> for sys::StgMutVar {
    fn from(x: StgMutVar) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgUpdateFrame = _StgUpdateFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _StgUpdateFrame {
    header: StgHeader,
    updatee: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<_StgUpdateFrame> for sys::_StgUpdateFrame {
    fn from(x: _StgUpdateFrame) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgOrigThunkInfoFrame = _StgOrigThunkInfoFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _StgOrigThunkInfoFrame {
    header: StgHeader,
    info_ptr: *mut StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<_StgOrigThunkInfoFrame> for sys::_StgOrigThunkInfoFrame {
    fn from(x: _StgOrigThunkInfoFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgKeepAliveFrame {
    header: StgHeader,
    c: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgKeepAliveFrame> for sys::StgKeepAliveFrame {
    fn from(x: StgKeepAliveFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgCatchFrame {
    pub header: StgHeader,
    pub handler: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgCatchFrame> for sys::StgCatchFrame {
    fn from(x: StgCatchFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgUnderflowFrame {
    pub info: *const StgInfoTable,
    pub next_chunk: *mut StgStack_,
}

#[cfg(feature = "sys")]
impl From<StgUnderflowFrame> for sys::StgUnderflowFrame {
    fn from(x: StgUnderflowFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgStopFrame {
    pub header: StgHeader,
}

#[cfg(feature = "sys")]
impl From<StgStopFrame> for sys::StgStopFrame {
    fn from(x: StgStopFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgDeadThreadFrame {
    pub header: StgHeader,
    pub result: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgDeadThreadFrame> for sys::StgDeadThreadFrame {
    fn from(x: StgDeadThreadFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgRetFun {
    pub info: *const StgInfoTable,
    pub size: StgWord,
    pub fun: *mut StgClosure,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<StgRetFun> for sys::StgRetFun {
    fn from(x: StgRetFun) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
///cbindgen:no-export
pub struct StgIntCharlikeClosure {
    header: StgHeader,
    data: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgIntCharlikeClosure> for sys::StgIntCharlikeClosure {
    fn from(x: StgIntCharlikeClosure) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgStableName = _StgStableName;

#[repr(C)]
///cbindgen:no-export
pub struct _StgStableName {
    header: StgHeader,
    sn: StgWord,
}

#[cfg(feature = "sys")]
impl From<_StgStableName> for sys::_StgStableName {
    fn from(x: _StgStableName) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgWeak = _StgWeak;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct _StgWeak {
    header: StgHeader,
    cfinalizers: *mut StgClosure,
    key: *mut StgClosure,
    value: *mut StgClosure,
    finalizer: *mut StgClosure,
    link: *mut _StgWeak,
}

#[cfg(feature = "sys")]
impl From<_StgWeak> for sys::_StgWeak {
    fn from(x: _StgWeak) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgCFinalizerList = _StgCFinalizerList;

#[repr(C)]
///cbindgen:no-export
pub struct _StgCFinalizerList {
    header: StgHeader,
    link: *mut StgClosure,
    fptr: Option<unsafe extern "C" fn()>,
    ptr: *mut c_void,
    eptr: *mut c_void,
    flag: StgWord,
}

#[cfg(feature = "sys")]
impl From<_StgCFinalizerList> for sys::_StgCFinalizerList {
    fn from(x: _StgCFinalizerList) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgMVarTSOQueue = StgMVarTSOQueue_;

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

#[cfg(feature = "sys")]
impl From<StgBCO> for sys::StgBCO {
    fn from(x: StgBCO) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgMVarTSOQueue_ {
    header: StgHeader,
    link: *mut StgMVarTSOQueue_,
    tso: *mut StgTSO_,
}

#[cfg(feature = "sys")]
impl From<StgMVarTSOQueue_> for sys::StgMVarTSOQueue_ {
    fn from(x: StgMVarTSOQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgTRecHeader = StgTRecHeader_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgMVar {
    pub header: StgHeader,
    pub head: *mut StgMVarTSOQueue_,
    pub tail: *mut StgMVarTSOQueue_,
    pub value: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgMVar> for sys::StgMVar {
    fn from(x: StgMVar) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgTVarWatchQueue = StgTVarWatchQueue_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgTVarWatchQueue_ {
    header: StgHeader,
    closure: *mut StgClosure,
    next_queue_entry: *mut StgTVarWatchQueue_,
    prev_queue_entry: *mut StgTVarWatchQueue_,
}

#[cfg(feature = "sys")]
impl From<StgTVarWatchQueue_> for sys::StgTVarWatchQueue_ {
    fn from(x: StgTVarWatchQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgTVar {
    pub header: StgHeader,
    pub current_value: *mut StgClosure,
    pub first_watch_queue_entry: *mut StgTVarWatchQueue,
    pub num_updates: StgInt,
}

#[cfg(feature = "sys")]
impl From<StgTVar> for sys::StgTVar {
    fn from(x: StgTVar) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgTRecChunk = StgTRecChunk_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct TRecEntry {
    tvar: *mut StgTVar,
    expected_value: *mut StgClosure,
    new_value: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<TRecEntry> for sys::TRecEntry {
    fn from(x: TRecEntry) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
///cbindgen:no-export
pub struct StgTRecChunk_ {
    header: StgHeader,
    prev_chunk: *mut StgTRecChunk_,
    next_entry_idx: StgWord,
    entries: [TRecEntry; 16usize],
}

#[cfg(feature = "sys")]
impl From<StgTRecChunk_> for sys::StgTRecChunk_ {
    fn from(x: StgTRecChunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum TRecState {
    TREC_ACTIVE = 0,
    TREC_CONDEMNED = 1,
    TREC_ABORTED = 2,
    TREC_WAITING = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgTRecHeader_ {
    header: StgHeader,
    enclosing_trec: *mut StgTRecHeader_,
    current_chunk: *mut StgTRecChunk,
    state: TRecState,
}

#[cfg(feature = "sys")]
impl From<StgTRecHeader_> for sys::StgTRecHeader_ {
    fn from(x: StgTRecHeader_) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgAtomicallyFrame {
    pub header: StgHeader,
    pub code: *mut StgClosure,
    pub result: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgAtomicallyFrame> for sys::StgAtomicallyFrame {
    fn from(x: StgAtomicallyFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StgCatchSTMFrame {
    pub header: StgHeader,
    pub code: *mut StgClosure,
    pub handler: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgCatchSTMFrame> for sys::StgCatchSTMFrame {
    fn from(x: StgCatchSTMFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgCatchRetryFrame {
    pub header: StgHeader,
    pub running_alt_code: StgWord,
    pub first_code: *mut StgClosure,
    pub alt_code: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgCatchRetryFrame> for sys::StgCatchRetryFrame {
    fn from(x: StgCatchRetryFrame) -> Self {
        unsafe { transmute(x) }
    }
}

pub type Message = Message_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct Message_ {
    header: StgHeader,
    link: *mut Message_,
}

#[cfg(feature = "sys")]
impl From<Message_> for sys::Message_ {
    fn from(x: Message_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type MessageWakeup = MessageWakeup_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct MessageWakeup_ {
    header: StgHeader,
    link: *mut Message,
    tso: *mut StgTSO,
}

#[cfg(feature = "sys")]
impl From<MessageWakeup_> for sys::MessageWakeup_ {
    fn from(x: MessageWakeup_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type MessageThrowTo = MessageThrowTo_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct MessageThrowTo_ {
    header: StgHeader,
    link: *mut MessageThrowTo_,
    source: *mut StgTSO,
    target: *mut StgTSO,
    exception: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<MessageThrowTo_> for sys::MessageThrowTo_ {
    fn from(x: MessageThrowTo_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type MessageBlackHole = MessageBlackHole_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct MessageBlackHole_ {
    header: StgHeader,
    link: *mut MessageBlackHole_,
    tso: *mut StgTSO,
    bh: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<MessageBlackHole_> for sys::MessageBlackHole_ {
    fn from(x: MessageBlackHole_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct MessageBlackHole_Owned {
    header: StgHeader,
}

pub type MessageCloneStack = MessageCloneStack_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct MessageCloneStack_ {
    header: StgHeader,
    link: *mut Message,
    result: *mut StgMVar,
    tso: *mut StgTSO,
}

#[cfg(feature = "sys")]
impl From<MessageCloneStack_> for sys::MessageCloneStack_ {
    fn from(x: MessageCloneStack_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgCompactNFDataBlock = StgCompactNFDataBlock_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgCompactNFDataBlock_ {
    self_: *mut StgCompactNFDataBlock_,
    owner: *mut StgCompactNFData_,
    next: *mut StgCompactNFDataBlock_,
}

#[cfg(feature = "sys")]
impl From<StgCompactNFDataBlock_> for sys::StgCompactNFDataBlock_ {
    fn from(x: StgCompactNFDataBlock_) -> Self {
        unsafe { transmute(x) }
    }
}

pub type StgCompactNFData = StgCompactNFData_;

#[repr(C)]
///cbindgen:no-export
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

#[cfg(feature = "sys")]
impl From<StgCompactNFData_> for sys::StgCompactNFData_ {
    fn from(x: StgCompactNFData_) -> Self {
        unsafe { transmute(x) }
    }
}

pub(crate) type StgPromptTag = *mut StgClosure;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub struct StgPromptFrame {
    header: StgHeader,
    tag: StgPromptTag,
}

#[cfg(feature = "sys")]
impl From<StgPromptFrame> for sys::StgPromptFrame {
    fn from(x: StgPromptFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub struct StgContinuation {
    pub header: StgHeader,
    pub apply_mask_frame: *const StgInfoTable,
    pub mask_frame_offset: StgWord,
    pub stack_size: StgWord,
    pub stack: __IncompleteArrayField<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgContinuation> for sys::StgContinuation {
    fn from(x: StgContinuation) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hashtable {
    pub _address: u8,
}

#[cfg(feature = "sys")]
impl From<hashtable> for sys::hashtable {
    fn from(x: hashtable) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for hashtable {
    fn arbitrary(g: &mut Gen) -> Self {
        hashtable {
            _address: Arbitrary::arbitrary(g),
        }
    }
}
