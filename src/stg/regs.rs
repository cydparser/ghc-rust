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
impl Arbitrary for StgFunTable {
    fn arbitrary(g: &mut Gen) -> Self {
        StgFunTable {
            stgEagerBlackholeInfo: Arbitrary::arbitrary(g),
            stgGCEnter1: Arbitrary::arbitrary(g),
            stgGCFun: Arbitrary::arbitrary(g),
        }
    }
}

#[repr(C)]
pub(crate) union StgUnion {
    pub w: ::core::mem::ManuallyDrop<StgWord>,
    pub a: ::core::mem::ManuallyDrop<StgAddr>,
    pub c: ::core::mem::ManuallyDrop<StgChar>,
    pub f: ::core::mem::ManuallyDrop<StgFloat>,
    pub i: ::core::mem::ManuallyDrop<StgInt>,
    pub p: ::core::mem::ManuallyDrop<StgPtr>,
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
        match Arbitrary::arbitrary::<usize>(g) % 6usize {
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
            5 => StgUnion {
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
impl Arbitrary for StgRegTable {
    fn arbitrary(g: &mut Gen) -> Self {
        StgRegTable {
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
            rSp: Arbitrary::arbitrary(g),
            rSpLim: Arbitrary::arbitrary(g),
            rHp: Arbitrary::arbitrary(g),
            rHpLim: Arbitrary::arbitrary(g),
            rCCCS: Arbitrary::arbitrary(g),
            rCurrentTSO: Arbitrary::arbitrary(g),
            rNursery: Arbitrary::arbitrary(g),
            rCurrentNursery: Arbitrary::arbitrary(g),
            rCurrentAlloc: Arbitrary::arbitrary(g),
            rHpAlloc: Arbitrary::arbitrary(g),
            rRet: Arbitrary::arbitrary(g),
        }
    }
}
