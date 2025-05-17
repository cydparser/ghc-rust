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

#[repr(C)]
///cbindgen:no-export
pub(crate) struct StgFunTable {
    pub stgEagerBlackholeInfo: StgWord,
    pub stgGCEnter1: StgFunPtr,
    pub stgGCFun: StgFunPtr,
}

#[cfg(feature = "sys")]
impl From<StgFunTable> for sys::StgFunTable {
    fn from(x: StgFunTable) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgFunTableOwned {
    pub stgEagerBlackholeInfo: StgWord,
}

#[cfg(test)]
impl Arbitrary for StgFunTableOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunTableOwned {
            stgEagerBlackholeInfo: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgFunTablePointees {
    pub stgGCEnter1: StgFunPtr,
    pub stgGCFun: StgFunPtr,
}

#[cfg(test)]
impl Arbitrary for StgFunTablePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunTablePointees {
            stgGCEnter1: Arbitrary::arbitrary(g),
            stgGCFun: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgFunTable {
    type Owned = StgFunTableOwned;
    type Pointees = StgFunTablePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            stgEagerBlackholeInfo: owned.stgEagerBlackholeInfo,
            stgGCEnter1: unsafe { &raw mut (*pointees).stgGCEnter1 },
            stgGCFun: unsafe { &raw mut (*pointees).stgGCFun },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            stgEagerBlackholeInfo: self.stgEagerBlackholeInfo,
        }
    }
}

#[repr(C)]
pub(crate) union StgUnion {
    pub w: ManuallyDrop<StgWord>,
    pub a: ManuallyDrop<StgAddr>,
    pub c: ManuallyDrop<StgChar>,
    pub f: ManuallyDrop<StgFloat>,
    pub i: ManuallyDrop<StgInt>,
    pub p: ManuallyDrop<StgPtr>,
}

#[cfg(feature = "sys")]
impl From<StgUnion> for sys::StgUnion {
    fn from(x: StgUnion) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for StgUnion {
    fn arbitrary(g: &mut Gen) -> Self {
        match <usize as Arbitrary>::arbitrary(g) % 6usize {
            0 => StgUnion {
                w: Arbitrary::arbitrary(g),
            },
            1 => StgUnion {
                a: Arbitrary::arbitrary(g),
            },
            2 => StgUnion {
                c: Arbitrary::arbitrary(g),
            },
            3 => StgUnion {
                f: Arbitrary::arbitrary(g),
            },
            4 => StgUnion {
                i: Arbitrary::arbitrary(g),
            },
            5.. => StgUnion {
                p: Arbitrary::arbitrary(g),
            },
        }
    }
}

#[repr(C)]
pub struct StgRegTable {
    pub rR1: StgUnion,
    pub rR2: StgUnion,
    pub rR3: StgUnion,
    pub rR4: StgUnion,
    pub rR5: StgUnion,
    pub rR6: StgUnion,
    pub rR7: StgUnion,
    pub rR8: StgUnion,
    pub rR9: StgUnion,
    pub rR10: StgUnion,
    pub rF1: StgFloat,
    pub rF2: StgFloat,
    pub rF3: StgFloat,
    pub rF4: StgFloat,
    pub rF5: StgFloat,
    pub rF6: StgFloat,
    pub rD1: StgDouble,
    pub rD2: StgDouble,
    pub rD3: StgDouble,
    pub rD4: StgDouble,
    pub rD5: StgDouble,
    pub rD6: StgDouble,
    pub rXMM1: StgWord128,
    pub rXMM2: StgWord128,
    pub rXMM3: StgWord128,
    pub rXMM4: StgWord128,
    pub rXMM5: StgWord128,
    pub rXMM6: StgWord128,
    pub rYMM1: StgWord256,
    pub rYMM2: StgWord256,
    pub rYMM3: StgWord256,
    pub rYMM4: StgWord256,
    pub rYMM5: StgWord256,
    pub rYMM6: StgWord256,
    pub rZMM1: StgWord512,
    pub rZMM2: StgWord512,
    pub rZMM3: StgWord512,
    pub rZMM4: StgWord512,
    pub rZMM5: StgWord512,
    pub rZMM6: StgWord512,
    pub rL1: StgWord64,
    pub rSp: StgPtr,
    pub rSpLim: StgPtr,
    pub rHp: StgPtr,
    pub rHpLim: StgPtr,
    pub rCCCS: *mut CostCentreStack_,
    pub rCurrentTSO: *mut StgTSO_,
    pub rNursery: *mut nursery_,
    pub rCurrentNursery: *mut bdescr_,
    pub rCurrentAlloc: *mut bdescr_,
    pub rHpAlloc: StgWord,
    pub rRet: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgRegTable> for sys::StgRegTable {
    fn from(x: StgRegTable) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgRegTableOwned {
    pub rR1: StgUnion,
    pub rR2: StgUnion,
    pub rR3: StgUnion,
    pub rR4: StgUnion,
    pub rR5: StgUnion,
    pub rR6: StgUnion,
    pub rR7: StgUnion,
    pub rR8: StgUnion,
    pub rR9: StgUnion,
    pub rR10: StgUnion,
    pub rF1: StgFloat,
    pub rF2: StgFloat,
    pub rF3: StgFloat,
    pub rF4: StgFloat,
    pub rF5: StgFloat,
    pub rF6: StgFloat,
    pub rD1: StgDouble,
    pub rD2: StgDouble,
    pub rD3: StgDouble,
    pub rD4: StgDouble,
    pub rD5: StgDouble,
    pub rD6: StgDouble,
    pub rXMM1: StgWord128,
    pub rXMM2: StgWord128,
    pub rXMM3: StgWord128,
    pub rXMM4: StgWord128,
    pub rXMM5: StgWord128,
    pub rXMM6: StgWord128,
    pub rYMM1: StgWord256,
    pub rYMM2: StgWord256,
    pub rYMM3: StgWord256,
    pub rYMM4: StgWord256,
    pub rYMM5: StgWord256,
    pub rYMM6: StgWord256,
    pub rZMM1: StgWord512,
    pub rZMM2: StgWord512,
    pub rZMM3: StgWord512,
    pub rZMM4: StgWord512,
    pub rZMM5: StgWord512,
    pub rZMM6: StgWord512,
    pub rL1: StgWord64,
    pub rHpAlloc: StgWord,
    pub rRet: StgWord,
}

#[cfg(test)]
impl Arbitrary for StgRegTableOwned {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRegTableOwned {
            rR1: Arbitrary::arbitrary(g),
            rR2: Arbitrary::arbitrary(g),
            rR3: Arbitrary::arbitrary(g),
            rR4: Arbitrary::arbitrary(g),
            rR5: Arbitrary::arbitrary(g),
            rR6: Arbitrary::arbitrary(g),
            rR7: Arbitrary::arbitrary(g),
            rR8: Arbitrary::arbitrary(g),
            rR9: Arbitrary::arbitrary(g),
            rR10: Arbitrary::arbitrary(g),
            rF1: Arbitrary::arbitrary(g),
            rF2: Arbitrary::arbitrary(g),
            rF3: Arbitrary::arbitrary(g),
            rF4: Arbitrary::arbitrary(g),
            rF5: Arbitrary::arbitrary(g),
            rF6: Arbitrary::arbitrary(g),
            rD1: Arbitrary::arbitrary(g),
            rD2: Arbitrary::arbitrary(g),
            rD3: Arbitrary::arbitrary(g),
            rD4: Arbitrary::arbitrary(g),
            rD5: Arbitrary::arbitrary(g),
            rD6: Arbitrary::arbitrary(g),
            rXMM1: Arbitrary::arbitrary(g),
            rXMM2: Arbitrary::arbitrary(g),
            rXMM3: Arbitrary::arbitrary(g),
            rXMM4: Arbitrary::arbitrary(g),
            rXMM5: Arbitrary::arbitrary(g),
            rXMM6: Arbitrary::arbitrary(g),
            rYMM1: Arbitrary::arbitrary(g),
            rYMM2: Arbitrary::arbitrary(g),
            rYMM3: Arbitrary::arbitrary(g),
            rYMM4: Arbitrary::arbitrary(g),
            rYMM5: Arbitrary::arbitrary(g),
            rYMM6: Arbitrary::arbitrary(g),
            rZMM1: Arbitrary::arbitrary(g),
            rZMM2: Arbitrary::arbitrary(g),
            rZMM3: Arbitrary::arbitrary(g),
            rZMM4: Arbitrary::arbitrary(g),
            rZMM5: Arbitrary::arbitrary(g),
            rZMM6: Arbitrary::arbitrary(g),
            rL1: Arbitrary::arbitrary(g),
            rHpAlloc: Arbitrary::arbitrary(g),
            rRet: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
#[derive(Clone)]
struct StgRegTablePointees {
    pub rSp: StgPtr,
    pub rSpLim: StgPtr,
    pub rHp: StgPtr,
    pub rHpLim: StgPtr,
    pub rCCCS: CostCentreStack_,
    pub rCurrentTSO: StgTSO_,
    pub rNursery: nursery_,
    pub rCurrentNursery: bdescr_,
    pub rCurrentAlloc: bdescr_,
}

#[cfg(test)]
impl Arbitrary for StgRegTablePointees {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRegTablePointees {
            rSp: Arbitrary::arbitrary(g),
            rSpLim: Arbitrary::arbitrary(g),
            rHp: Arbitrary::arbitrary(g),
            rHpLim: Arbitrary::arbitrary(g),
            rCCCS: Arbitrary::arbitrary(g),
            rCurrentTSO: Arbitrary::arbitrary(g),
            rNursery: Arbitrary::arbitrary(g),
            rCurrentNursery: Arbitrary::arbitrary(g),
            rCurrentAlloc: Arbitrary::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl HasReferences for StgRegTable {
    type Owned = StgRegTableOwned;
    type Pointees = StgRegTablePointees;
    fn from_parts(owned: Self::Owned, pointees: *mut Self::Pointees) -> Self {
        Self {
            rR1: owned.rR1.clone(),
            rR2: owned.rR2.clone(),
            rR3: owned.rR3.clone(),
            rR4: owned.rR4.clone(),
            rR5: owned.rR5.clone(),
            rR6: owned.rR6.clone(),
            rR7: owned.rR7.clone(),
            rR8: owned.rR8.clone(),
            rR9: owned.rR9.clone(),
            rR10: owned.rR10.clone(),
            rF1: owned.rF1,
            rF2: owned.rF2,
            rF3: owned.rF3,
            rF4: owned.rF4,
            rF5: owned.rF5,
            rF6: owned.rF6,
            rD1: owned.rD1,
            rD2: owned.rD2,
            rD3: owned.rD3,
            rD4: owned.rD4,
            rD5: owned.rD5,
            rD6: owned.rD6,
            rXMM1: owned.rXMM1.clone(),
            rXMM2: owned.rXMM2.clone(),
            rXMM3: owned.rXMM3.clone(),
            rXMM4: owned.rXMM4.clone(),
            rXMM5: owned.rXMM5.clone(),
            rXMM6: owned.rXMM6.clone(),
            rYMM1: owned.rYMM1.clone(),
            rYMM2: owned.rYMM2.clone(),
            rYMM3: owned.rYMM3.clone(),
            rYMM4: owned.rYMM4.clone(),
            rYMM5: owned.rYMM5.clone(),
            rYMM6: owned.rYMM6.clone(),
            rZMM1: owned.rZMM1.clone(),
            rZMM2: owned.rZMM2.clone(),
            rZMM3: owned.rZMM3.clone(),
            rZMM4: owned.rZMM4.clone(),
            rZMM5: owned.rZMM5.clone(),
            rZMM6: owned.rZMM6.clone(),
            rL1: owned.rL1,
            rHpAlloc: owned.rHpAlloc,
            rRet: owned.rRet,
            rSp: unsafe { &raw mut (*pointees).rSp },
            rSpLim: unsafe { &raw mut (*pointees).rSpLim },
            rHp: unsafe { &raw mut (*pointees).rHp },
            rHpLim: unsafe { &raw mut (*pointees).rHpLim },
            rCCCS: unsafe { &raw mut (*pointees).rCCCS },
            rCurrentTSO: unsafe { &raw mut (*pointees).rCurrentTSO },
            rNursery: unsafe { &raw mut (*pointees).rNursery },
            rCurrentNursery: unsafe { &raw mut (*pointees).rCurrentNursery },
            rCurrentAlloc: unsafe { &raw mut (*pointees).rCurrentAlloc },
        }
    }
    fn owned(&self) -> Self::Owned {
        Self::Owned {
            rR1: self.rR1.clone(),
            rR2: self.rR2.clone(),
            rR3: self.rR3.clone(),
            rR4: self.rR4.clone(),
            rR5: self.rR5.clone(),
            rR6: self.rR6.clone(),
            rR7: self.rR7.clone(),
            rR8: self.rR8.clone(),
            rR9: self.rR9.clone(),
            rR10: self.rR10.clone(),
            rF1: self.rF1,
            rF2: self.rF2,
            rF3: self.rF3,
            rF4: self.rF4,
            rF5: self.rF5,
            rF6: self.rF6,
            rD1: self.rD1,
            rD2: self.rD2,
            rD3: self.rD3,
            rD4: self.rD4,
            rD5: self.rD5,
            rD6: self.rD6,
            rXMM1: self.rXMM1.clone(),
            rXMM2: self.rXMM2.clone(),
            rXMM3: self.rXMM3.clone(),
            rXMM4: self.rXMM4.clone(),
            rXMM5: self.rXMM5.clone(),
            rXMM6: self.rXMM6.clone(),
            rYMM1: self.rYMM1.clone(),
            rYMM2: self.rYMM2.clone(),
            rYMM3: self.rYMM3.clone(),
            rYMM4: self.rYMM4.clone(),
            rYMM5: self.rYMM5.clone(),
            rYMM6: self.rYMM6.clone(),
            rZMM1: self.rZMM1.clone(),
            rZMM2: self.rZMM2.clone(),
            rZMM3: self.rZMM3.clone(),
            rZMM4: self.rZMM4.clone(),
            rZMM5: self.rZMM5.clone(),
            rZMM6: self.rZMM6.clone(),
            rL1: self.rL1,
            rHpAlloc: self.rHpAlloc,
            rRet: self.rRet,
        }
    }
}
