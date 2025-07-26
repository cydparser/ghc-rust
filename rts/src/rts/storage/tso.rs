use crate::prelude::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};

#[cfg(test)]
mod tests;

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
#[derive(Clone)]
struct StgTSOProfInfoOwned {}
#[cfg(test)]
impl Arbitrary for StgTSOProfInfoOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSOProfInfoOwned {}
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTSOProfInfoPointees {
    pub cccs: CostCentreStack,
}

#[cfg(test)]
impl Arbitrary for StgTSOProfInfoPointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSOProfInfoPointees {
            cccs: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTSOProfInfo {
    type Owned = StgTSOProfInfoOwned;
    type Pointees = StgTSOProfInfoPointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            cccs: unsafe { &raw mut (*pointees).cccs },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {}
    }
}

pub(crate) type StgThreadID = StgWord64;

pub(crate) type StgThreadReturnCode = c_uint;

#[repr(C)]
pub(crate) union StgTSOBlockInfo {
    pub closure: ManuallyDrop<*mut StgClosure>,
    pub prev: ManuallyDrop<*mut StgTSO>,
    pub bh: ManuallyDrop<*mut MessageBlackHole_>,
    pub throwto: ManuallyDrop<*mut MessageThrowTo_>,
    pub wakeup: ManuallyDrop<*mut MessageWakeup_>,
    pub fd: ManuallyDrop<StgInt>,
    pub target: ManuallyDrop<StgWord>,
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
        match <usize as Arbitrary>::arbitrary(g) % 7usize {
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
            6.. => StgTSOBlockInfo {
                target: Arbitrary::arbitrary(g),
            },
        }
    }
}

pub type StgTSO = StgTSO_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct StgTSO_ {
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
#[derive(Clone)]
struct StgTSO_Owned {
    pub header: StgHeader,
    pub what_next: StgWord16,
    pub flags: StgWord32,
    pub why_blocked: StgWord32,
    pub block_info: StgTSOBlockInfo,
    pub id: StgThreadID,
    pub saved_errno: StgWord32,
    pub dirty: StgWord32,
    pub alloc_limit: StgInt64,
    pub tot_stack_size: StgWord32,
}

#[cfg(test)]
impl Arbitrary for StgTSO_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSO_Owned {
            header: Arbitrary::arbitrary(g),
            what_next: Arbitrary::arbitrary(g),
            flags: Arbitrary::arbitrary(g),
            why_blocked: Arbitrary::arbitrary(g),
            block_info: Arbitrary::arbitrary(g),
            id: Arbitrary::arbitrary(g),
            saved_errno: Arbitrary::arbitrary(g),
            dirty: Arbitrary::arbitrary(g),
            alloc_limit: Arbitrary::arbitrary(g),
            tot_stack_size: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgTSO_Pointees {
    pub _link: StgTSO_,
    pub global_link: StgTSO_,
    pub stackobj: StgStack_,
    pub bound: InCall_,
    pub cap: Capability_,
    pub trec: StgTRecHeader_,
    pub label: StgArrBytes,
    pub blocked_exceptions: MessageThrowTo_,
    pub bq: StgBlockingQueue_,
}

#[cfg(test)]
impl Arbitrary for StgTSO_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgTSO_Pointees {
            _link: Arbitrary::arbitrary(g),
            global_link: Arbitrary::arbitrary(g),
            stackobj: Arbitrary::arbitrary(g),
            bound: Arbitrary::arbitrary(g),
            cap: Arbitrary::arbitrary(g),
            trec: Arbitrary::arbitrary(g),
            label: Arbitrary::arbitrary(g),
            blocked_exceptions: Arbitrary::arbitrary(g),
            bq: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgTSO_ {
    type Owned = StgTSO_Owned;
    type Pointees = StgTSO_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            what_next: owned.what_next,
            flags: owned.flags,
            why_blocked: owned.why_blocked,
            block_info: owned.block_info.clone(),
            id: owned.id,
            saved_errno: owned.saved_errno,
            dirty: owned.dirty,
            alloc_limit: owned.alloc_limit,
            tot_stack_size: owned.tot_stack_size,
            _link: unsafe { &raw mut (*pointees)._link },
            global_link: unsafe { &raw mut (*pointees).global_link },
            stackobj: unsafe { &raw mut (*pointees).stackobj },
            bound: unsafe { &raw mut (*pointees).bound },
            cap: unsafe { &raw mut (*pointees).cap },
            trec: unsafe { &raw mut (*pointees).trec },
            label: unsafe { &raw mut (*pointees).label },
            blocked_exceptions: unsafe { &raw mut (*pointees).blocked_exceptions },
            bq: unsafe { &raw mut (*pointees).bq },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            what_next: self.what_next,
            flags: self.flags,
            why_blocked: self.why_blocked,
            block_info: self.block_info.clone(),
            id: self.id,
            saved_errno: self.saved_errno,
            dirty: self.dirty,
            alloc_limit: self.alloc_limit,
            tot_stack_size: self.tot_stack_size,
        }
    }
}

pub(crate) type StgTSOPtr = *mut StgTSO_;

#[repr(C)]
///cbindgen:no-export
pub(crate) struct StgStack_ {
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
#[derive(Clone)]
struct StgStack_Owned {
    pub header: StgHeader,
    pub stack_size: StgWord32,
    pub dirty: StgWord8,
    pub marking: StgWord8,
    pub stack: __IncompleteArrayField<StgWord>,
}

#[cfg(test)]
impl Arbitrary for StgStack_Owned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgStack_Owned {
            header: Arbitrary::arbitrary(g),
            stack_size: Arbitrary::arbitrary(g),
            dirty: Arbitrary::arbitrary(g),
            marking: Arbitrary::arbitrary(g),
            stack: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgStack_Pointees {
    pub sp: StgPtr,
}

#[cfg(test)]
impl Arbitrary for StgStack_Pointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgStack_Pointees {
            sp: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgStack_ {
    type Owned = StgStack_Owned;
    type Pointees = StgStack_Pointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            header: owned.header.clone(),
            stack_size: owned.stack_size,
            dirty: owned.dirty,
            marking: owned.marking,
            stack: owned.stack.clone(),
            sp: unsafe { &raw mut (*pointees).sp },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            header: self.header.clone(),
            stack_size: self.stack_size,
            dirty: self.dirty,
            marking: self.marking,
            stack: self.stack.clone(),
        }
    }
}

pub type StgStack = StgStack_;

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_TSO(cap: *mut Capability, tso: *mut StgTSO) {
    unsafe { sys::dirty_TSO(cap as *mut sys::Capability, tso as *mut sys::StgTSO) }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOLink(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        sys::setTSOLink(
            cap as *mut sys::Capability,
            tso as *mut sys::StgTSO,
            target as *mut sys::StgTSO,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn setTSOPrev(cap: *mut Capability, tso: *mut StgTSO, target: *mut StgTSO) {
    unsafe {
        sys::setTSOPrev(
            cap as *mut sys::Capability,
            tso as *mut sys::StgTSO,
            target as *mut sys::StgTSO,
        )
    }
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn dirty_STACK(cap: *mut Capability, stack: *mut StgStack) {
    unsafe { sys::dirty_STACK(cap as *mut sys::Capability, stack as *mut sys::StgStack) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
///cbindgen:no-export
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
