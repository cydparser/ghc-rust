use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);

impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

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

#[cfg(test)]
impl Arbitrary for StgProfHeader {
    fn arbitrary(g: &mut Gen) -> Self {
        StgProfHeader {
            ccs: Arbitrary::arbitrary(g),
            hp: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) union StgProfHeader__bindgen_ty_1 {
    pub trav: ::core::mem::ManuallyDrop<StgWord>,
    pub ldvw: ::core::mem::ManuallyDrop<StgWord>,
    pub era: ::core::mem::ManuallyDrop<StgWord>,
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
        match Arbitrary::arbitrary::<usize>(g) % 3usize {
            0 => StgProfHeader__bindgen_ty_1 {
                trav: Arbitrary::arbitrary(g),
            },
            1 => StgProfHeader__bindgen_ty_1 {
                ldvw: Arbitrary::arbitrary(g),
            },
            2 => StgProfHeader__bindgen_ty_1 {
                era: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
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

#[cfg(test)]
impl Arbitrary for StgHeader {
    fn arbitrary(g: &mut Gen) -> Self {
        StgHeader {
            info: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgThunkHeader {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunkHeader {
            info: Arbitrary::arbitrary(g),
            smp: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct StgClosure_ {
    pub header: StgHeader,
    pub payload: __IncompleteArrayField<*mut StgClosure_>,
}

#[cfg(feature = "sys")]
impl From<StgClosure_> for sys::StgClosure_ {
    fn from(x: StgClosure_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgClosure_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgClosure_ {
            header: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgClosurePtr = *mut StgClosure_;

#[repr(C)]
pub(crate) struct StgThunk_ {
    pub header: StgThunkHeader,
    pub payload: __IncompleteArrayField<*mut StgClosure_>,
}

#[cfg(feature = "sys")]
impl From<StgThunk_> for sys::StgThunk_ {
    fn from(x: StgThunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgThunk_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunk_ {
            header: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgThunk = StgThunk_;

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

#[cfg(test)]
impl Arbitrary for StgSelector {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSelector {
            header: Arbitrary::arbitrary(g),
            selectee: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgPAP {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPAP {
            header: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            n_args: Arbitrary::arbitrary(g),
            fun: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgAP {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAP {
            header: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            n_args: Arbitrary::arbitrary(g),
            fun: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgAP_STACK {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAP_STACK {
            header: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            fun: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgInd {
    fn arbitrary(g: &mut Gen) -> Self {
        StgInd {
            header: Arbitrary::arbitrary(g),
            indirectee: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgIndStatic {
    pub header: StgHeader,
    pub indirectee: *mut StgClosure,
    pub static_link: *mut StgClosure,
    pub saved_info: *const StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<StgIndStatic> for sys::StgIndStatic {
    fn from(x: StgIndStatic) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgIndStatic {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIndStatic {
            header: Arbitrary::arbitrary(g),
            indirectee: Arbitrary::arbitrary(g),
            static_link: Arbitrary::arbitrary(g),
            saved_info: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgBlockingQueue_ {
    pub header: StgHeader,
    pub link: *mut StgBlockingQueue_,
    pub bh: *mut StgClosure,
    pub owner: *mut StgTSO,
    pub queue: *mut MessageBlackHole_,
}

#[cfg(feature = "sys")]
impl From<StgBlockingQueue_> for sys::StgBlockingQueue_ {
    fn from(x: StgBlockingQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgBlockingQueue_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBlockingQueue_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            bh: Arbitrary::arbitrary(g),
            owner: Arbitrary::arbitrary(g),
            queue: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgBlockingQueue = StgBlockingQueue_;

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

#[cfg(test)]
impl Arbitrary for StgArrBytes {
    fn arbitrary(g: &mut Gen) -> Self {
        StgArrBytes {
            header: Arbitrary::arbitrary(g),
            bytes: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) struct _StgMutArrPtrs {
    pub header: StgHeader,
    pub ptrs: StgWord,
    pub size: StgWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(feature = "sys")]
impl From<_StgMutArrPtrs> for sys::_StgMutArrPtrs {
    fn from(x: _StgMutArrPtrs) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgMutArrPtrs {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgMutArrPtrs {
            header: Arbitrary::arbitrary(g),
            ptrs: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgMutArrPtrs = _StgMutArrPtrs;

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

#[cfg(test)]
impl Arbitrary for StgSmallMutArrPtrs {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSmallMutArrPtrs {
            header: Arbitrary::arbitrary(g),
            ptrs: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgMutVar {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMutVar {
            header: Arbitrary::arbitrary(g),
            var: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _StgUpdateFrame {
    pub header: StgHeader,
    pub updatee: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<_StgUpdateFrame> for sys::_StgUpdateFrame {
    fn from(x: _StgUpdateFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgUpdateFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgUpdateFrame {
            header: Arbitrary::arbitrary(g),
            updatee: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgUpdateFrame = _StgUpdateFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _StgOrigThunkInfoFrame {
    pub header: StgHeader,
    pub info_ptr: *mut StgInfoTable,
}

#[cfg(feature = "sys")]
impl From<_StgOrigThunkInfoFrame> for sys::_StgOrigThunkInfoFrame {
    fn from(x: _StgOrigThunkInfoFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgOrigThunkInfoFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgOrigThunkInfoFrame {
            header: Arbitrary::arbitrary(g),
            info_ptr: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgOrigThunkInfoFrame = _StgOrigThunkInfoFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgKeepAliveFrame {
    pub header: StgHeader,
    pub c: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<StgKeepAliveFrame> for sys::StgKeepAliveFrame {
    fn from(x: StgKeepAliveFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgKeepAliveFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgKeepAliveFrame {
            header: Arbitrary::arbitrary(g),
            c: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgCatchFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchFrame {
            header: Arbitrary::arbitrary(g),
            handler: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgUnderflowFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgUnderflowFrame {
            info: Arbitrary::arbitrary(g),
            next_chunk: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgStopFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgStopFrame {
            header: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgDeadThreadFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgDeadThreadFrame {
            header: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgRetFun {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRetFun {
            info: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            fun: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) struct StgIntCharlikeClosure {
    pub header: StgHeader,
    pub data: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgIntCharlikeClosure> for sys::StgIntCharlikeClosure {
    fn from(x: StgIntCharlikeClosure) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgIntCharlikeClosure {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIntCharlikeClosure {
            header: Arbitrary::arbitrary(g),
            data: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) struct _StgStableName {
    pub header: StgHeader,
    pub sn: StgWord,
}

#[cfg(feature = "sys")]
impl From<_StgStableName> for sys::_StgStableName {
    fn from(x: _StgStableName) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgStableName {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgStableName {
            header: Arbitrary::arbitrary(g),
            sn: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgStableName = _StgStableName;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct _StgWeak {
    pub header: StgHeader,
    pub cfinalizers: *mut StgClosure,
    pub key: *mut StgClosure,
    pub value: *mut StgClosure,
    pub finalizer: *mut StgClosure,
    pub link: *mut _StgWeak,
}

#[cfg(feature = "sys")]
impl From<_StgWeak> for sys::_StgWeak {
    fn from(x: _StgWeak) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgWeak {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgWeak {
            header: Arbitrary::arbitrary(g),
            cfinalizers: Arbitrary::arbitrary(g),
            key: Arbitrary::arbitrary(g),
            value: Arbitrary::arbitrary(g),
            finalizer: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgWeak = _StgWeak;

#[repr(C)]
pub(crate) struct _StgCFinalizerList {
    pub header: StgHeader,
    pub link: *mut StgClosure,
    pub fptr: ::core::option::Option<unsafe extern "C" fn()>,
    pub ptr: *mut ::core::ffi::c_void,
    pub eptr: *mut ::core::ffi::c_void,
    pub flag: StgWord,
}

#[cfg(feature = "sys")]
impl From<_StgCFinalizerList> for sys::_StgCFinalizerList {
    fn from(x: _StgCFinalizerList) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for _StgCFinalizerList {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgCFinalizerList {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            fptr: Arbitrary::arbitrary(g),
            ptr: Arbitrary::arbitrary(g),
            eptr: Arbitrary::arbitrary(g),
            flag: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgCFinalizerList = _StgCFinalizerList;

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

#[cfg(test)]
impl Arbitrary for StgBCO {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBCO {
            header: Arbitrary::arbitrary(g),
            instrs: Arbitrary::arbitrary(g),
            literals: Arbitrary::arbitrary(g),
            ptrs: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            bitmap: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgMVarTSOQueue_ {
    pub header: StgHeader,
    pub link: *mut StgMVarTSOQueue_,
    pub tso: *mut StgTSO_,
}

#[cfg(feature = "sys")]
impl From<StgMVarTSOQueue_> for sys::StgMVarTSOQueue_ {
    fn from(x: StgMVarTSOQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgMVarTSOQueue_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVarTSOQueue_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgMVarTSOQueue = StgMVarTSOQueue_;

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

#[cfg(test)]
impl Arbitrary for StgMVar {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVar {
            header: Arbitrary::arbitrary(g),
            head: Arbitrary::arbitrary(g),
            tail: Arbitrary::arbitrary(g),
            value: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgTRecHeader = StgTRecHeader_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgTVarWatchQueue_ {
    pub header: StgHeader,
    pub closure: *mut StgClosure,
    pub next_queue_entry: *mut StgTVarWatchQueue_,
    pub prev_queue_entry: *mut StgTVarWatchQueue_,
}

#[cfg(feature = "sys")]
impl From<StgTVarWatchQueue_> for sys::StgTVarWatchQueue_ {
    fn from(x: StgTVarWatchQueue_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgTVarWatchQueue_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVarWatchQueue_ {
            header: Arbitrary::arbitrary(g),
            closure: Arbitrary::arbitrary(g),
            next_queue_entry: Arbitrary::arbitrary(g),
            prev_queue_entry: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgTVarWatchQueue = StgTVarWatchQueue_;

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

#[cfg(test)]
impl Arbitrary for StgTVar {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVar {
            header: Arbitrary::arbitrary(g),
            current_value: Arbitrary::arbitrary(g),
            first_watch_queue_entry: Arbitrary::arbitrary(g),
            num_updates: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct TRecEntry {
    pub tvar: *mut StgTVar,
    pub expected_value: *mut StgClosure,
    pub new_value: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<TRecEntry> for sys::TRecEntry {
    fn from(x: TRecEntry) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for TRecEntry {
    fn arbitrary(g: &mut Gen) -> Self {
        TRecEntry {
            tvar: Arbitrary::arbitrary(g),
            expected_value: Arbitrary::arbitrary(g),
            new_value: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) struct StgTRecChunk_ {
    pub header: StgHeader,
    pub prev_chunk: *mut StgTRecChunk_,
    pub next_entry_idx: StgWord,
    pub entries: [TRecEntry; 16usize],
}

#[cfg(feature = "sys")]
impl From<StgTRecChunk_> for sys::StgTRecChunk_ {
    fn from(x: StgTRecChunk_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgTRecChunk_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecChunk_ {
            header: Arbitrary::arbitrary(g),
            prev_chunk: Arbitrary::arbitrary(g),
            next_entry_idx: Arbitrary::arbitrary(g),
            entries: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgTRecChunk = StgTRecChunk_;

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
pub(crate) struct StgTRecHeader_ {
    pub header: StgHeader,
    pub enclosing_trec: *mut StgTRecHeader_,
    pub current_chunk: *mut StgTRecChunk,
    pub state: TRecState,
}

#[cfg(feature = "sys")]
impl From<StgTRecHeader_> for sys::StgTRecHeader_ {
    fn from(x: StgTRecHeader_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgTRecHeader_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecHeader_ {
            header: Arbitrary::arbitrary(g),
            enclosing_trec: Arbitrary::arbitrary(g),
            current_chunk: Arbitrary::arbitrary(g),
            state: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgAtomicallyFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAtomicallyFrame {
            header: Arbitrary::arbitrary(g),
            code: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgCatchSTMFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchSTMFrame {
            header: Arbitrary::arbitrary(g),
            code: Arbitrary::arbitrary(g),
            handler: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgCatchRetryFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchRetryFrame {
            header: Arbitrary::arbitrary(g),
            running_alt_code: Arbitrary::arbitrary(g),
            first_code: Arbitrary::arbitrary(g),
            alt_code: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct Message_ {
    pub header: StgHeader,
    pub link: *mut Message_,
}

#[cfg(feature = "sys")]
impl From<Message_> for sys::Message_ {
    fn from(x: Message_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for Message_ {
    fn arbitrary(g: &mut Gen) -> Self {
        Message_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

pub type Message = Message_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct MessageWakeup_ {
    pub header: StgHeader,
    pub link: *mut Message,
    pub tso: *mut StgTSO,
}

#[cfg(feature = "sys")]
impl From<MessageWakeup_> for sys::MessageWakeup_ {
    fn from(x: MessageWakeup_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for MessageWakeup_ {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageWakeup_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type MessageWakeup = MessageWakeup_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct MessageThrowTo_ {
    pub header: StgHeader,
    pub link: *mut MessageThrowTo_,
    pub source: *mut StgTSO,
    pub target: *mut StgTSO,
    pub exception: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<MessageThrowTo_> for sys::MessageThrowTo_ {
    fn from(x: MessageThrowTo_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for MessageThrowTo_ {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageThrowTo_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            source: Arbitrary::arbitrary(g),
            target: Arbitrary::arbitrary(g),
            exception: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type MessageThrowTo = MessageThrowTo_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct MessageBlackHole_ {
    pub header: StgHeader,
    pub link: *mut MessageBlackHole_,
    pub tso: *mut StgTSO,
    pub bh: *mut StgClosure,
}

#[cfg(feature = "sys")]
impl From<MessageBlackHole_> for sys::MessageBlackHole_ {
    fn from(x: MessageBlackHole_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for MessageBlackHole_ {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageBlackHole_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
            bh: Arbitrary::arbitrary(g),
        }
    }
}

pub type MessageBlackHole = MessageBlackHole_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MessageCloneStack_ {
    pub header: StgHeader,
    pub link: *mut Message,
    pub result: *mut StgMVar,
    pub tso: *mut StgTSO,
}

#[cfg(feature = "sys")]
impl From<MessageCloneStack_> for sys::MessageCloneStack_ {
    fn from(x: MessageCloneStack_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for MessageCloneStack_ {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageCloneStack_ {
            header: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

pub type MessageCloneStack = MessageCloneStack_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgCompactNFDataBlock_ {
    pub self_: *mut StgCompactNFDataBlock_,
    pub owner: *mut StgCompactNFData_,
    pub next: *mut StgCompactNFDataBlock_,
}

#[cfg(feature = "sys")]
impl From<StgCompactNFDataBlock_> for sys::StgCompactNFDataBlock_ {
    fn from(x: StgCompactNFDataBlock_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgCompactNFDataBlock_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFDataBlock_ {
            self_: Arbitrary::arbitrary(g),
            owner: Arbitrary::arbitrary(g),
            next: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgCompactNFDataBlock = StgCompactNFDataBlock_;

#[repr(C)]
pub(crate) struct StgCompactNFData_ {
    pub header: StgHeader,
    pub totalW: StgWord,
    pub autoBlockW: StgWord,
    pub hp: StgPtr,
    pub hpLim: StgPtr,
    pub nursery: *mut StgCompactNFDataBlock,
    pub last: *mut StgCompactNFDataBlock,
    pub hash: *mut hashtable,
    pub result: *mut StgClosure,
    pub link: *mut StgCompactNFData_,
}

#[cfg(feature = "sys")]
impl From<StgCompactNFData_> for sys::StgCompactNFData_ {
    fn from(x: StgCompactNFData_) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgCompactNFData_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFData_ {
            header: Arbitrary::arbitrary(g),
            totalW: Arbitrary::arbitrary(g),
            autoBlockW: Arbitrary::arbitrary(g),
            hp: Arbitrary::arbitrary(g),
            hpLim: Arbitrary::arbitrary(g),
            nursery: Arbitrary::arbitrary(g),
            last: Arbitrary::arbitrary(g),
            hash: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgCompactNFData = StgCompactNFData_;

pub(crate) type StgPromptTag = *mut StgClosure;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct StgPromptFrame {
    pub header: StgHeader,
    pub tag: StgPromptTag,
}

#[cfg(feature = "sys")]
impl From<StgPromptFrame> for sys::StgPromptFrame {
    fn from(x: StgPromptFrame) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgPromptFrame {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPromptFrame {
            header: Arbitrary::arbitrary(g),
            tag: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgContinuation {
    fn arbitrary(g: &mut Gen) -> Self {
        StgContinuation {
            header: Arbitrary::arbitrary(g),
            apply_mask_frame: Arbitrary::arbitrary(g),
            mask_frame_offset: Arbitrary::arbitrary(g),
            stack_size: Arbitrary::arbitrary(g),
            stack: Arbitrary::arbitrary(g),
        }
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
