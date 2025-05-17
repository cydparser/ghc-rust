use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

use crate::utils::bindgen;
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
        slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), len)
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
#[derive(Clone)]
struct StgProfHeaderOwned {
    pub hp: StgProfHeader__bindgen_ty_1,
}

#[cfg(test)]
impl Arbitrary for StgProfHeaderOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgProfHeaderOwned {
            hp: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgProfHeaderPointees {
    pub ccs: CostCentreStack,
}

#[cfg(test)]
impl Arbitrary for StgProfHeaderPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgProfHeaderPointees {
            ccs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgProfHeader {
    type Owned = StgProfHeaderOwned;
    type Pointees = StgProfHeaderPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            hp: owned.hp.clone(),
            ccs: unsafe { &raw mut (*pointees).ccs },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            hp: self.hp.clone(),
        }
    }
}

#[repr(C)]
pub(crate) union StgProfHeader__bindgen_ty_1 {
    pub trav: ManuallyDrop<StgWord>,
    pub ldvw: ManuallyDrop<StgWord>,
    pub era: ManuallyDrop<StgWord>,
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
#[derive(Clone)]
struct StgHeaderOwned {}
#[cfg(test)]
impl Arbitrary for StgHeaderOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgHeaderOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgHeaderPointees {
    pub info: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for StgHeaderPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgHeaderPointees {
            info: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgHeader {
    type Owned = StgHeaderOwned;
    type Pointees = StgHeaderPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            info: unsafe { &raw mut (*pointees).info },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
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
#[derive(Clone)]
struct StgThunkHeaderOwned {
    pub smp: StgSMPThunkHeader,
}

#[cfg(test)]
impl Arbitrary for StgThunkHeaderOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunkHeaderOwned {
            smp: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgThunkHeaderPointees {
    pub info: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for StgThunkHeaderPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgThunkHeaderPointees {
            info: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgThunkHeader {
    type Owned = StgThunkHeaderOwned;
    type Pointees = StgThunkHeaderPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            smp: owned.smp.clone(),
            info: unsafe { &raw mut (*pointees).info },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            smp: self.smp.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
///cbindgen:no-export
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
///cbindgen:no-export
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
#[derive(Clone)]
struct StgSelectorOwned {
    pub header: StgThunkHeader,
}

#[cfg(test)]
impl Arbitrary for StgSelectorOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSelectorOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgSelectorPointees {
    pub selectee: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgSelectorPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgSelectorPointees {
            selectee: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgSelector {
    type Owned = StgSelectorOwned;
    type Pointees = StgSelectorPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            selectee: unsafe { &raw mut (*pointees).selectee },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgPAPOwned {
    pub header: StgHeader,
    pub arity: StgHalfWord,
    pub n_args: StgHalfWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(test)]
impl Arbitrary for StgPAPOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPAPOwned {
            header: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            n_args: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgPAPPointees {
    pub fun: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgPAPPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPAPPointees {
            fun: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgPAP {
    type Owned = StgPAPOwned;
    type Pointees = StgPAPPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            arity: owned.arity,
            n_args: owned.n_args,
            payload: owned.payload.clone(),
            fun: unsafe { &raw mut (*pointees).fun },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            arity: self.arity,
            n_args: self.n_args,
            payload: self.payload.clone(),
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
#[derive(Clone)]
struct StgAPOwned {
    pub header: StgThunkHeader,
    pub arity: StgHalfWord,
    pub n_args: StgHalfWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(test)]
impl Arbitrary for StgAPOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAPOwned {
            header: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            n_args: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgAPPointees {
    pub fun: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgAPPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAPPointees {
            fun: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgAP {
    type Owned = StgAPOwned;
    type Pointees = StgAPPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            arity: owned.arity,
            n_args: owned.n_args,
            payload: owned.payload.clone(),
            fun: unsafe { &raw mut (*pointees).fun },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            arity: self.arity,
            n_args: self.n_args,
            payload: self.payload.clone(),
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
#[derive(Clone)]
struct StgAP_STACKOwned {
    pub header: StgThunkHeader,
    pub size: StgWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(test)]
impl Arbitrary for StgAP_STACKOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAP_STACKOwned {
            header: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgAP_STACKPointees {
    pub fun: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgAP_STACKPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAP_STACKPointees {
            fun: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgAP_STACK {
    type Owned = StgAP_STACKOwned;
    type Pointees = StgAP_STACKPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            size: owned.size,
            payload: owned.payload.clone(),
            fun: unsafe { &raw mut (*pointees).fun },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            size: self.size,
            payload: self.payload.clone(),
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
#[derive(Clone)]
struct StgIndOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgIndOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIndOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgIndPointees {
    pub indirectee: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgIndPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIndPointees {
            indirectee: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgInd {
    type Owned = StgIndOwned;
    type Pointees = StgIndPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            indirectee: unsafe { &raw mut (*pointees).indirectee },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgIndStaticOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgIndStaticOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIndStaticOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgIndStaticPointees {
    pub indirectee: StgClosure,
    pub static_link: StgClosure,
    pub saved_info: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for StgIndStaticPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgIndStaticPointees {
            indirectee: Arbitrary::arbitrary(g),
            static_link: Arbitrary::arbitrary(g),
            saved_info: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgIndStatic {
    type Owned = StgIndStaticOwned;
    type Pointees = StgIndStaticPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            indirectee: unsafe { &raw mut (*pointees).indirectee },
            static_link: unsafe { &raw mut (*pointees).static_link },
            saved_info: unsafe { &raw mut (*pointees).saved_info },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgBlockingQueue_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgBlockingQueue_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBlockingQueue_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgBlockingQueue_Pointees {
    pub link: StgBlockingQueue_,
    pub bh: StgClosure,
    pub owner: StgTSO,
    pub queue: MessageBlackHole_,
}

#[cfg(test)]
impl Arbitrary for StgBlockingQueue_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBlockingQueue_Pointees {
            link: Arbitrary::arbitrary(g),
            bh: Arbitrary::arbitrary(g),
            owner: Arbitrary::arbitrary(g),
            queue: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgBlockingQueue_ {
    type Owned = StgBlockingQueue_Owned;
    type Pointees = StgBlockingQueue_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            bh: unsafe { &raw mut (*pointees).bh },
            owner: unsafe { &raw mut (*pointees).owner },
            queue: unsafe { &raw mut (*pointees).queue },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
///cbindgen:no-export
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
#[derive(Clone)]
struct StgMutVarOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgMutVarOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMutVarOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgMutVarPointees {
    pub var: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgMutVarPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMutVarPointees {
            var: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgMutVar {
    type Owned = StgMutVarOwned;
    type Pointees = StgMutVarPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            var: unsafe { &raw mut (*pointees).var },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct _StgUpdateFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for _StgUpdateFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgUpdateFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgUpdateFramePointees {
    pub updatee: StgClosure,
}

#[cfg(test)]
impl Arbitrary for _StgUpdateFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgUpdateFramePointees {
            updatee: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _StgUpdateFrame {
    type Owned = _StgUpdateFrameOwned;
    type Pointees = _StgUpdateFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            updatee: unsafe { &raw mut (*pointees).updatee },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type StgUpdateFrame = _StgUpdateFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct _StgOrigThunkInfoFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for _StgOrigThunkInfoFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgOrigThunkInfoFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgOrigThunkInfoFramePointees {
    pub info_ptr: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for _StgOrigThunkInfoFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgOrigThunkInfoFramePointees {
            info_ptr: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _StgOrigThunkInfoFrame {
    type Owned = _StgOrigThunkInfoFrameOwned;
    type Pointees = _StgOrigThunkInfoFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            info_ptr: unsafe { &raw mut (*pointees).info_ptr },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type StgOrigThunkInfoFrame = _StgOrigThunkInfoFrame;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgKeepAliveFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgKeepAliveFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgKeepAliveFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgKeepAliveFramePointees {
    pub c: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgKeepAliveFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgKeepAliveFramePointees {
            c: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgKeepAliveFrame {
    type Owned = StgKeepAliveFrameOwned;
    type Pointees = StgKeepAliveFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            c: unsafe { &raw mut (*pointees).c },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgCatchFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgCatchFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgCatchFramePointees {
    pub handler: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgCatchFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchFramePointees {
            handler: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgCatchFrame {
    type Owned = StgCatchFrameOwned;
    type Pointees = StgCatchFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            handler: unsafe { &raw mut (*pointees).handler },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgUnderflowFrameOwned {}
#[cfg(test)]
impl Arbitrary for StgUnderflowFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgUnderflowFrameOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgUnderflowFramePointees {
    pub info: StgInfoTable,
    pub next_chunk: StgStack_,
}

#[cfg(test)]
impl Arbitrary for StgUnderflowFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgUnderflowFramePointees {
            info: Arbitrary::arbitrary(g),
            next_chunk: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgUnderflowFrame {
    type Owned = StgUnderflowFrameOwned;
    type Pointees = StgUnderflowFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            info: unsafe { &raw mut (*pointees).info },
            next_chunk: unsafe { &raw mut (*pointees).next_chunk },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
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
#[derive(Clone)]
struct StgDeadThreadFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgDeadThreadFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgDeadThreadFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgDeadThreadFramePointees {
    pub result: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgDeadThreadFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgDeadThreadFramePointees {
            result: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgDeadThreadFrame {
    type Owned = StgDeadThreadFrameOwned;
    type Pointees = StgDeadThreadFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            result: unsafe { &raw mut (*pointees).result },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgRetFunOwned {
    pub size: StgWord,
    pub payload: __IncompleteArrayField<*mut StgClosure>,
}

#[cfg(test)]
impl Arbitrary for StgRetFunOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRetFunOwned {
            size: Arbitrary::arbitrary(g),
            payload: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgRetFunPointees {
    pub info: StgInfoTable,
    pub fun: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgRetFunPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRetFunPointees {
            info: Arbitrary::arbitrary(g),
            fun: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgRetFun {
    type Owned = StgRetFunOwned;
    type Pointees = StgRetFunPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            size: owned.size,
            payload: owned.payload.clone(),
            info: unsafe { &raw mut (*pointees).info },
            fun: unsafe { &raw mut (*pointees).fun },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            size: self.size,
            payload: self.payload.clone(),
        }
    }
}

#[repr(C)]
///cbindgen:no-export
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
///cbindgen:no-export
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
///cbindgen:no-export
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
#[derive(Clone)]
struct _StgWeakOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for _StgWeakOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgWeakOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgWeakPointees {
    pub cfinalizers: StgClosure,
    pub key: StgClosure,
    pub value: StgClosure,
    pub finalizer: StgClosure,
    pub link: _StgWeak,
}

#[cfg(test)]
impl Arbitrary for _StgWeakPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgWeakPointees {
            cfinalizers: Arbitrary::arbitrary(g),
            key: Arbitrary::arbitrary(g),
            value: Arbitrary::arbitrary(g),
            finalizer: Arbitrary::arbitrary(g),
            link: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _StgWeak {
    type Owned = _StgWeakOwned;
    type Pointees = _StgWeakPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            cfinalizers: unsafe { &raw mut (*pointees).cfinalizers },
            key: unsafe { &raw mut (*pointees).key },
            value: unsafe { &raw mut (*pointees).value },
            finalizer: unsafe { &raw mut (*pointees).finalizer },
            link: unsafe { &raw mut (*pointees).link },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type StgWeak = _StgWeak;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct _StgCFinalizerList {
    pub header: StgHeader,
    pub link: *mut StgClosure,
    pub fptr: Option<unsafe extern "C" fn()>,
    pub ptr: *mut c_void,
    pub eptr: *mut c_void,
    pub flag: StgWord,
}

#[cfg(feature = "sys")]
impl From<_StgCFinalizerList> for sys::_StgCFinalizerList {
    fn from(x: _StgCFinalizerList) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgCFinalizerListOwned {
    pub header: StgHeader,
    pub fptr: Option<unsafe extern "C" fn()>,
    pub flag: StgWord,
}

#[cfg(test)]
impl Arbitrary for _StgCFinalizerListOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgCFinalizerListOwned {
            header: Arbitrary::arbitrary(g),
            fptr: Arbitrary::arbitrary(g),
            flag: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct _StgCFinalizerListPointees {
    pub link: StgClosure,
    pub ptr: c_void,
    pub eptr: c_void,
}

#[cfg(test)]
impl Arbitrary for _StgCFinalizerListPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        _StgCFinalizerListPointees {
            link: Arbitrary::arbitrary(g),
            ptr: Arbitrary::arbitrary(g),
            eptr: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for _StgCFinalizerList {
    type Owned = _StgCFinalizerListOwned;
    type Pointees = _StgCFinalizerListPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            fptr: owned.fptr,
            flag: owned.flag,
            link: unsafe { &raw mut (*pointees).link },
            ptr: unsafe { &raw mut (*pointees).ptr },
            eptr: unsafe { &raw mut (*pointees).eptr },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            fptr: self.fptr,
            flag: self.flag,
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
#[derive(Clone)]
struct StgBCOOwned {
    pub header: StgHeader,
    pub arity: StgHalfWord,
    pub size: StgHalfWord,
    pub bitmap: __IncompleteArrayField<StgWord>,
}

#[cfg(test)]
impl Arbitrary for StgBCOOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBCOOwned {
            header: Arbitrary::arbitrary(g),
            arity: Arbitrary::arbitrary(g),
            size: Arbitrary::arbitrary(g),
            bitmap: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgBCOPointees {
    pub instrs: StgArrBytes,
    pub literals: StgArrBytes,
    pub ptrs: StgMutArrPtrs,
}

#[cfg(test)]
impl Arbitrary for StgBCOPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgBCOPointees {
            instrs: Arbitrary::arbitrary(g),
            literals: Arbitrary::arbitrary(g),
            ptrs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgBCO {
    type Owned = StgBCOOwned;
    type Pointees = StgBCOPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            arity: owned.arity,
            size: owned.size,
            bitmap: owned.bitmap.clone(),
            instrs: unsafe { &raw mut (*pointees).instrs },
            literals: unsafe { &raw mut (*pointees).literals },
            ptrs: unsafe { &raw mut (*pointees).ptrs },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            arity: self.arity,
            size: self.size,
            bitmap: self.bitmap.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgMVarTSOQueue_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgMVarTSOQueue_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVarTSOQueue_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgMVarTSOQueue_Pointees {
    pub link: StgMVarTSOQueue_,
    pub tso: StgTSO_,
}

#[cfg(test)]
impl Arbitrary for StgMVarTSOQueue_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVarTSOQueue_Pointees {
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgMVarTSOQueue_ {
    type Owned = StgMVarTSOQueue_Owned;
    type Pointees = StgMVarTSOQueue_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            tso: unsafe { &raw mut (*pointees).tso },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgMVarOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgMVarOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVarOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgMVarPointees {
    pub head: StgMVarTSOQueue_,
    pub tail: StgMVarTSOQueue_,
    pub value: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgMVarPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgMVarPointees {
            head: Arbitrary::arbitrary(g),
            tail: Arbitrary::arbitrary(g),
            value: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgMVar {
    type Owned = StgMVarOwned;
    type Pointees = StgMVarPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            head: unsafe { &raw mut (*pointees).head },
            tail: unsafe { &raw mut (*pointees).tail },
            value: unsafe { &raw mut (*pointees).value },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type StgTRecHeader = StgTRecHeader_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgTVarWatchQueue_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgTVarWatchQueue_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVarWatchQueue_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTVarWatchQueue_Pointees {
    pub closure: StgClosure,
    pub next_queue_entry: StgTVarWatchQueue_,
    pub prev_queue_entry: StgTVarWatchQueue_,
}

#[cfg(test)]
impl Arbitrary for StgTVarWatchQueue_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVarWatchQueue_Pointees {
            closure: Arbitrary::arbitrary(g),
            next_queue_entry: Arbitrary::arbitrary(g),
            prev_queue_entry: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTVarWatchQueue_ {
    type Owned = StgTVarWatchQueue_Owned;
    type Pointees = StgTVarWatchQueue_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            closure: unsafe { &raw mut (*pointees).closure },
            next_queue_entry: unsafe { &raw mut (*pointees).next_queue_entry },
            prev_queue_entry: unsafe { &raw mut (*pointees).prev_queue_entry },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgTVarOwned {
    pub header: StgHeader,
    pub num_updates: StgInt,
}

#[cfg(test)]
impl Arbitrary for StgTVarOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVarOwned {
            header: Arbitrary::arbitrary(g),
            num_updates: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTVarPointees {
    pub current_value: StgClosure,
    pub first_watch_queue_entry: StgTVarWatchQueue,
}

#[cfg(test)]
impl Arbitrary for StgTVarPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTVarPointees {
            current_value: Arbitrary::arbitrary(g),
            first_watch_queue_entry: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTVar {
    type Owned = StgTVarOwned;
    type Pointees = StgTVarPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            num_updates: owned.num_updates,
            current_value: unsafe { &raw mut (*pointees).current_value },
            first_watch_queue_entry: unsafe { &raw mut (*pointees).first_watch_queue_entry },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            num_updates: self.num_updates,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct TRecEntryOwned {}
#[cfg(test)]
impl Arbitrary for TRecEntryOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        TRecEntryOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct TRecEntryPointees {
    pub tvar: StgTVar,
    pub expected_value: StgClosure,
    pub new_value: StgClosure,
}

#[cfg(test)]
impl Arbitrary for TRecEntryPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        TRecEntryPointees {
            tvar: Arbitrary::arbitrary(g),
            expected_value: Arbitrary::arbitrary(g),
            new_value: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for TRecEntry {
    type Owned = TRecEntryOwned;
    type Pointees = TRecEntryPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            tvar: unsafe { &raw mut (*pointees).tvar },
            expected_value: unsafe { &raw mut (*pointees).expected_value },
            new_value: unsafe { &raw mut (*pointees).new_value },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

#[repr(C)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgTRecChunk_Owned {
    pub header: StgHeader,
    pub next_entry_idx: StgWord,
    pub entries: [TRecEntry; 16usize],
}

#[cfg(test)]
impl Arbitrary for StgTRecChunk_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecChunk_Owned {
            header: Arbitrary::arbitrary(g),
            next_entry_idx: Arbitrary::arbitrary(g),
            entries: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTRecChunk_Pointees {
    pub prev_chunk: StgTRecChunk_,
}

#[cfg(test)]
impl Arbitrary for StgTRecChunk_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecChunk_Pointees {
            prev_chunk: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTRecChunk_ {
    type Owned = StgTRecChunk_Owned;
    type Pointees = StgTRecChunk_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            next_entry_idx: owned.next_entry_idx,
            entries: owned.entries.clone(),
            prev_chunk: unsafe { &raw mut (*pointees).prev_chunk },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            next_entry_idx: self.next_entry_idx,
            entries: self.entries.clone(),
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
///cbindgen:no-export
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
#[derive(Clone)]
struct StgTRecHeader_Owned {
    pub header: StgHeader,
    pub state: TRecState,
}

#[cfg(test)]
impl Arbitrary for StgTRecHeader_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecHeader_Owned {
            header: Arbitrary::arbitrary(g),
            state: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTRecHeader_Pointees {
    pub enclosing_trec: StgTRecHeader_,
    pub current_chunk: StgTRecChunk,
}

#[cfg(test)]
impl Arbitrary for StgTRecHeader_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTRecHeader_Pointees {
            enclosing_trec: Arbitrary::arbitrary(g),
            current_chunk: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTRecHeader_ {
    type Owned = StgTRecHeader_Owned;
    type Pointees = StgTRecHeader_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            state: owned.state.clone(),
            enclosing_trec: unsafe { &raw mut (*pointees).enclosing_trec },
            current_chunk: unsafe { &raw mut (*pointees).current_chunk },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            state: self.state.clone(),
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
#[derive(Clone)]
struct StgAtomicallyFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgAtomicallyFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAtomicallyFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgAtomicallyFramePointees {
    pub code: StgClosure,
    pub result: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgAtomicallyFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgAtomicallyFramePointees {
            code: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgAtomicallyFrame {
    type Owned = StgAtomicallyFrameOwned;
    type Pointees = StgAtomicallyFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            code: unsafe { &raw mut (*pointees).code },
            result: unsafe { &raw mut (*pointees).result },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgCatchSTMFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgCatchSTMFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchSTMFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgCatchSTMFramePointees {
    pub code: StgClosure,
    pub handler: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgCatchSTMFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchSTMFramePointees {
            code: Arbitrary::arbitrary(g),
            handler: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgCatchSTMFrame {
    type Owned = StgCatchSTMFrameOwned;
    type Pointees = StgCatchSTMFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            code: unsafe { &raw mut (*pointees).code },
            handler: unsafe { &raw mut (*pointees).handler },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgCatchRetryFrameOwned {
    pub header: StgHeader,
    pub running_alt_code: StgWord,
}

#[cfg(test)]
impl Arbitrary for StgCatchRetryFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchRetryFrameOwned {
            header: Arbitrary::arbitrary(g),
            running_alt_code: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgCatchRetryFramePointees {
    pub first_code: StgClosure,
    pub alt_code: StgClosure,
}

#[cfg(test)]
impl Arbitrary for StgCatchRetryFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCatchRetryFramePointees {
            first_code: Arbitrary::arbitrary(g),
            alt_code: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgCatchRetryFrame {
    type Owned = StgCatchRetryFrameOwned;
    type Pointees = StgCatchRetryFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            running_alt_code: owned.running_alt_code,
            first_code: unsafe { &raw mut (*pointees).first_code },
            alt_code: unsafe { &raw mut (*pointees).alt_code },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            running_alt_code: self.running_alt_code,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct Message_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for Message_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        Message_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct Message_Pointees {
    pub link: Message_,
}

#[cfg(test)]
impl Arbitrary for Message_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        Message_Pointees {
            link: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for Message_ {
    type Owned = Message_Owned;
    type Pointees = Message_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type Message = Message_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct MessageWakeup_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for MessageWakeup_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageWakeup_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct MessageWakeup_Pointees {
    pub link: Message,
    pub tso: StgTSO,
}

#[cfg(test)]
impl Arbitrary for MessageWakeup_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageWakeup_Pointees {
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for MessageWakeup_ {
    type Owned = MessageWakeup_Owned;
    type Pointees = MessageWakeup_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            tso: unsafe { &raw mut (*pointees).tso },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub(crate) type MessageWakeup = MessageWakeup_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct MessageThrowTo_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for MessageThrowTo_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageThrowTo_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct MessageThrowTo_Pointees {
    pub link: MessageThrowTo_,
    pub source: StgTSO,
    pub target: StgTSO,
    pub exception: StgClosure,
}

#[cfg(test)]
impl Arbitrary for MessageThrowTo_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageThrowTo_Pointees {
            link: Arbitrary::arbitrary(g),
            source: Arbitrary::arbitrary(g),
            target: Arbitrary::arbitrary(g),
            exception: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for MessageThrowTo_ {
    type Owned = MessageThrowTo_Owned;
    type Pointees = MessageThrowTo_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            source: unsafe { &raw mut (*pointees).source },
            target: unsafe { &raw mut (*pointees).target },
            exception: unsafe { &raw mut (*pointees).exception },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub(crate) type MessageThrowTo = MessageThrowTo_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct MessageBlackHole_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for MessageBlackHole_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageBlackHole_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct MessageBlackHole_Pointees {
    pub link: MessageBlackHole_,
    pub tso: StgTSO,
    pub bh: StgClosure,
}

#[cfg(test)]
impl Arbitrary for MessageBlackHole_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageBlackHole_Pointees {
            link: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
            bh: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for MessageBlackHole_ {
    type Owned = MessageBlackHole_Owned;
    type Pointees = MessageBlackHole_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            tso: unsafe { &raw mut (*pointees).tso },
            bh: unsafe { &raw mut (*pointees).bh },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type MessageBlackHole = MessageBlackHole_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
pub(crate) struct MessageCloneStack_ {
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
#[derive(Clone)]
struct MessageCloneStack_Owned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for MessageCloneStack_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageCloneStack_Owned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct MessageCloneStack_Pointees {
    pub link: Message,
    pub result: StgMVar,
    pub tso: StgTSO,
}

#[cfg(test)]
impl Arbitrary for MessageCloneStack_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        MessageCloneStack_Pointees {
            link: Arbitrary::arbitrary(g),
            result: Arbitrary::arbitrary(g),
            tso: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for MessageCloneStack_ {
    type Owned = MessageCloneStack_Owned;
    type Pointees = MessageCloneStack_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            link: unsafe { &raw mut (*pointees).link },
            result: unsafe { &raw mut (*pointees).result },
            tso: unsafe { &raw mut (*pointees).tso },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
        }
    }
}

pub type MessageCloneStack = MessageCloneStack_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgCompactNFDataBlock_Owned {}
#[cfg(test)]
impl Arbitrary for StgCompactNFDataBlock_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFDataBlock_Owned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgCompactNFDataBlock_Pointees {
    pub self_: StgCompactNFDataBlock_,
    pub owner: StgCompactNFData_,
    pub next: StgCompactNFDataBlock_,
}

#[cfg(test)]
impl Arbitrary for StgCompactNFDataBlock_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFDataBlock_Pointees {
            self_: Arbitrary::arbitrary(g),
            owner: Arbitrary::arbitrary(g),
            next: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgCompactNFDataBlock_ {
    type Owned = StgCompactNFDataBlock_Owned;
    type Pointees = StgCompactNFDataBlock_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            self_: unsafe { &raw mut (*pointees).self_ },
            owner: unsafe { &raw mut (*pointees).owner },
            next: unsafe { &raw mut (*pointees).next },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

pub type StgCompactNFDataBlock = StgCompactNFDataBlock_;

#[repr(C)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgCompactNFData_Owned {
    pub header: StgHeader,
    pub totalW: StgWord,
    pub autoBlockW: StgWord,
}

#[cfg(test)]
impl Arbitrary for StgCompactNFData_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFData_Owned {
            header: Arbitrary::arbitrary(g),
            totalW: Arbitrary::arbitrary(g),
            autoBlockW: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgCompactNFData_Pointees {
    pub hp: StgPtr,
    pub hpLim: StgPtr,
    pub nursery: StgCompactNFDataBlock,
    pub last: StgCompactNFDataBlock,
    pub hash: hashtable,
    pub result: StgClosure,
    pub link: StgCompactNFData_,
}

#[cfg(test)]
impl Arbitrary for StgCompactNFData_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgCompactNFData_Pointees {
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

#[cfg(test)]
impl HasReferences for StgCompactNFData_ {
    type Owned = StgCompactNFData_Owned;
    type Pointees = StgCompactNFData_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            totalW: owned.totalW,
            autoBlockW: owned.autoBlockW,
            hp: unsafe { &raw mut (*pointees).hp },
            hpLim: unsafe { &raw mut (*pointees).hpLim },
            nursery: unsafe { &raw mut (*pointees).nursery },
            last: unsafe { &raw mut (*pointees).last },
            hash: unsafe { &raw mut (*pointees).hash },
            result: unsafe { &raw mut (*pointees).result },
            link: unsafe { &raw mut (*pointees).link },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            totalW: self.totalW,
            autoBlockW: self.autoBlockW,
        }
    }
}

pub type StgCompactNFData = StgCompactNFData_;

pub(crate) type StgPromptTag = *mut StgClosure;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
#[derive(Clone)]
struct StgPromptFrameOwned {
    pub header: StgHeader,
}

#[cfg(test)]
impl Arbitrary for StgPromptFrameOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPromptFrameOwned {
            header: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgPromptFramePointees {
    pub tag: StgPromptTag,
}

#[cfg(test)]
impl Arbitrary for StgPromptFramePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgPromptFramePointees {
            tag: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgPromptFrame {
    type Owned = StgPromptFrameOwned;
    type Pointees = StgPromptFramePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            tag: unsafe { &raw mut (*pointees).tag },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
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
#[derive(Clone)]
struct StgContinuationOwned {
    pub header: StgHeader,
    pub mask_frame_offset: StgWord,
    pub stack_size: StgWord,
    pub stack: __IncompleteArrayField<StgWord>,
}

#[cfg(test)]
impl Arbitrary for StgContinuationOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgContinuationOwned {
            header: Arbitrary::arbitrary(g),
            mask_frame_offset: Arbitrary::arbitrary(g),
            stack_size: Arbitrary::arbitrary(g),
            stack: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgContinuationPointees {
    pub apply_mask_frame: StgInfoTable,
}

#[cfg(test)]
impl Arbitrary for StgContinuationPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgContinuationPointees {
            apply_mask_frame: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgContinuation {
    type Owned = StgContinuationOwned;
    type Pointees = StgContinuationPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            mask_frame_offset: owned.mask_frame_offset,
            stack_size: owned.stack_size,
            stack: owned.stack.clone(),
            apply_mask_frame: unsafe { &raw mut (*pointees).apply_mask_frame },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            mask_frame_offset: self.mask_frame_offset,
            stack_size: self.stack_size,
            stack: self.stack.clone(),
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
