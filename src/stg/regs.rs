use std::mem::transmute;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use crate::{
    rts::prof::ccs::CostCentreStack_,
    rts::storage::{block::bdescr_, gc::nursery_, tso::StgTSO_},
    stg::types::{
        StgAddr, StgChar, StgDouble, StgFloat, StgFunPtr, StgInt, StgPtr, StgWord, StgWord128,
        StgWord256, StgWord512, StgWord64,
    },
};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Clone)]
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
            stgGCEnter1: None,
            stgGCFun: None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union StgUnion {
    pub w: StgWord,
    pub a: StgAddr,
    pub c: StgChar,
    pub f: StgFloat,
    pub i: StgInt,
    pub p: StgPtr,
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
        match <usize as Arbitrary>::arbitrary(g) % 4_usize {
            0 => StgUnion {
                w: Arbitrary::arbitrary(g),
            },
            1 => StgUnion {
                c: Arbitrary::arbitrary(g),
            },
            2 => StgUnion {
                f: Arbitrary::arbitrary(g),
            },
            3.. => StgUnion {
                i: Arbitrary::arbitrary(g),
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
