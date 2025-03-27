use crate::rts::storage::closures;
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

#[cfg(test)]
impl Arbitrary for StgTSOProfInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSOProfInfo {
            cccs: Arbitrary::arbitrary(g),
        }
    }
}

pub(crate) type StgThreadID = StgWord64;

pub(crate) type StgThreadReturnCode = ::core::ffi::c_uint;

#[repr(C)]
pub(crate) union StgTSOBlockInfo {
    pub closure: ::core::mem::ManuallyDrop<*mut StgClosure>,
    pub prev: ::core::mem::ManuallyDrop<*mut StgTSO>,
    pub bh: ::core::mem::ManuallyDrop<*mut MessageBlackHole_>,
    pub throwto: ::core::mem::ManuallyDrop<*mut MessageThrowTo_>,
    pub wakeup: ::core::mem::ManuallyDrop<*mut MessageWakeup_>,
    pub fd: ::core::mem::ManuallyDrop<StgInt>,
    pub target: ::core::mem::ManuallyDrop<StgWord>,
}

#[cfg(feature = "sys")]
impl From<StgTSOBlockInfo> for sys::StgTSOBlockInfo {
    fn from(x: StgTSOBlockInfo) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgTSOBlockInfo {
    fn arbitrary(g: &mut Gen) -> Self {
        match Arbitrary::arbitrary::<usize>(g) % 7usize {
            0 => StgTSOBlockInfo {
                closure: Arbitrary::arbitrary(g),
            },
            1 => StgTSOBlockInfo {
                prev: Arbitrary::arbitrary(g),
            },
            2 => StgTSOBlockInfo {
                bh: Arbitrary::arbitrary(g),
            },
            3 => StgTSOBlockInfo {
                throwto: Arbitrary::arbitrary(g),
            },
            4 => StgTSOBlockInfo {
                wakeup: Arbitrary::arbitrary(g),
            },
            5 => StgTSOBlockInfo {
                fd: Arbitrary::arbitrary(g),
            },
            6 => StgTSOBlockInfo {
                target: Arbitrary::arbitrary(g),
            },
        }
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

#[cfg(test)]
impl Arbitrary for StgTSO_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSO_ {
            header: Arbitrary::arbitrary(g),
            _link: Arbitrary::arbitrary(g),
            global_link: Arbitrary::arbitrary(g),
            stackobj: Arbitrary::arbitrary(g),
            what_next: Arbitrary::arbitrary(g),
            flags: Arbitrary::arbitrary(g),
            why_blocked: Arbitrary::arbitrary(g),
            block_info: Arbitrary::arbitrary(g),
            id: Arbitrary::arbitrary(g),
            saved_errno: Arbitrary::arbitrary(g),
            dirty: Arbitrary::arbitrary(g),
            bound: Arbitrary::arbitrary(g),
            cap: Arbitrary::arbitrary(g),
            trec: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            blocked_exceptions: Arbitrary::arbitrary(g),
            bq: Arbitrary::arbitrary(g),
            alloc_limit: Arbitrary::arbitrary(g),
            tot_stack_size: Arbitrary::arbitrary(g),
        }
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

#[cfg(test)]
impl Arbitrary for StgStack_ {
    fn arbitrary(g: &mut Gen) -> Self {
        StgStack_ {
            header: Arbitrary::arbitrary(g),
            stack_size: Arbitrary::arbitrary(g),
            dirty: Arbitrary::arbitrary(g),
            marking: Arbitrary::arbitrary(g),
            sp: Arbitrary::arbitrary(g),
            stack: Arbitrary::arbitrary(g),
        }
    }
}

pub type StgStack = StgStack_;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_TSO(cap: *mut Capability, tso: *mut StgTSO) {
    unsafe { transmute(sys::dirty_TSO(&mut cap.into(), &mut tso.into())) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOLink(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        transmute(sys::setTSOLink(
            &mut cap.into(),
            &mut tso.into(),
            &mut target.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOPrev(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        transmute(sys::setTSOPrev(
            &mut cap.into(),
            &mut tso.into(),
            &mut target.into(),
        ))
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_STACK(cap: *mut Capability, stack: *mut StgStack) {
    unsafe { transmute(sys::dirty_STACK(&mut cap.into(), &mut stack.into())) }
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
