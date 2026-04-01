use crate::ffi::rts::prof::ccs::CostCentreStack_;
use crate::ffi::rts::storage::block::bdescr_;
use crate::ffi::rts::storage::gc::nursery_;
use crate::ffi::rts::storage::tso::StgTSO_;
use crate::ffi::stg::types::{
    StgAddr, StgChar, StgDouble, StgFloat, StgFunPtr, StgInt, StgPtr, StgWord, StgWord64,
    StgWord128, StgWord256, StgWord512,
};
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

#[ffi(compiler, ghc_lib, testsuite, utils)]
#[repr(C)]
pub union StgUnion {
    pub w: StgWord,
    pub a: StgAddr,
    pub c: StgChar,
    pub f: StgFloat,
    pub i: StgInt,
    pub p: StgPtr,
}

#[ffi(compiler)]
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
