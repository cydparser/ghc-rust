use crate::ffi::rts::prof::ccs::CostCentreStack_;
use crate::ffi::rts::storage::block::bdescr_;
use crate::ffi::rts::storage::gc::nursery_;
use crate::ffi::rts::storage::tso::StgTSO_;
use crate::ffi::stg::types::{
    StgAddr, StgChar, StgDouble, StgFloat, StgFunPtr, StgInt, StgPtr, StgWord, StgWord64,
    StgWord128, StgWord256, StgWord512,
};
#[cfg(feature = "sys")]
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// cbindgen:no-export
#[repr(C)]
pub struct StgFunTable {
    stgEagerBlackholeInfo: StgWord,
    stgGCEnter1: StgFunPtr,
    stgGCFun: StgFunPtr,
}

#[cfg(feature = "sys")]
impl From<StgFunTable> for sys::StgFunTable {
    fn from(x: StgFunTable) -> Self {
        unsafe { transmute(x) }
    }
}

#[repr(C)]
pub(crate) union StgUnion {
    w: StgWord,
    a: StgAddr,
    c: StgChar,
    f: StgFloat,
    i: StgInt,
    p: StgPtr,
}

#[cfg(feature = "sys")]
impl From<StgUnion> for sys::StgUnion {
    fn from(x: StgUnion) -> Self {
        unsafe { transmute(x) }
    }
}

/// cbindgen:no-export
#[repr(C)]
pub struct StgRegTable {
    rR1: StgUnion,
    rR2: StgUnion,
    rR3: StgUnion,
    rR4: StgUnion,
    rR5: StgUnion,
    rR6: StgUnion,
    rR7: StgUnion,
    rR8: StgUnion,
    rR9: StgUnion,
    rR10: StgUnion,
    rF1: StgFloat,
    rF2: StgFloat,
    rF3: StgFloat,
    rF4: StgFloat,
    rF5: StgFloat,
    rF6: StgFloat,
    rD1: StgDouble,
    rD2: StgDouble,
    rD3: StgDouble,
    rD4: StgDouble,
    rD5: StgDouble,
    rD6: StgDouble,
    rXMM1: StgWord128,
    rXMM2: StgWord128,
    rXMM3: StgWord128,
    rXMM4: StgWord128,
    rXMM5: StgWord128,
    rXMM6: StgWord128,
    rYMM1: StgWord256,
    rYMM2: StgWord256,
    rYMM3: StgWord256,
    rYMM4: StgWord256,
    rYMM5: StgWord256,
    rYMM6: StgWord256,
    rZMM1: StgWord512,
    rZMM2: StgWord512,
    rZMM3: StgWord512,
    rZMM4: StgWord512,
    rZMM5: StgWord512,
    rZMM6: StgWord512,
    rL1: StgWord64,
    rSp: StgPtr,
    rSpLim: StgPtr,
    rHp: StgPtr,
    rHpLim: StgPtr,
    rCCCS: *mut CostCentreStack_,
    rCurrentTSO: *mut StgTSO_,
    rNursery: *mut nursery_,
    rCurrentNursery: *mut bdescr_,
    rCurrentAlloc: *mut bdescr_,
    rHpAlloc: StgWord,
    rRet: StgWord,
}

#[cfg(feature = "sys")]
impl From<StgRegTable> for sys::StgRegTable {
    fn from(x: StgRegTable) -> Self {
        unsafe { transmute(x) }
    }
}
