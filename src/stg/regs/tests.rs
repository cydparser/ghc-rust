use std::mem::size_of;

use super::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgFunTable() {
    assert_eq!(
        size_of::<sys::StgFunTable>(),
        size_of::<super::StgFunTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunTable"][::core::mem::size_of::<StgFunTable>() - 24usize];
    ["Alignment of StgFunTable"][::core::mem::align_of::<StgFunTable>() - 8usize];
    ["Offset of field: StgFunTable::stgEagerBlackholeInfo"]
        [::core::mem::offset_of!(StgFunTable, stgEagerBlackholeInfo) - 0usize];
    ["Offset of field: StgFunTable::stgGCEnter1"]
        [::core::mem::offset_of!(StgFunTable, stgGCEnter1) - 8usize];
    ["Offset of field: StgFunTable::stgGCFun"]
        [::core::mem::offset_of!(StgFunTable, stgGCFun) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgUnion() {
    assert_eq!(size_of::<sys::StgUnion>(), size_of::<super::StgUnion>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgUnion"][::core::mem::size_of::<StgUnion>() - 8usize];
    ["Alignment of StgUnion"][::core::mem::align_of::<StgUnion>() - 8usize];
    ["Offset of field: StgUnion::w"][::core::mem::offset_of!(StgUnion, w) - 0usize];
    ["Offset of field: StgUnion::a"][::core::mem::offset_of!(StgUnion, a) - 0usize];
    ["Offset of field: StgUnion::c"][::core::mem::offset_of!(StgUnion, c) - 0usize];
    ["Offset of field: StgUnion::f"][::core::mem::offset_of!(StgUnion, f) - 0usize];
    ["Offset of field: StgUnion::i"][::core::mem::offset_of!(StgUnion, i) - 0usize];
    ["Offset of field: StgUnion::p"][::core::mem::offset_of!(StgUnion, p) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgRegTable() {
    assert_eq!(
        size_of::<sys::StgRegTable>(),
        size_of::<super::StgRegTable>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRegTable"][::core::mem::size_of::<StgRegTable>() - 920usize];
    ["Alignment of StgRegTable"][::core::mem::align_of::<StgRegTable>() - 8usize];
    ["Offset of field: StgRegTable::rR1"][::core::mem::offset_of!(StgRegTable, rR1) - 0usize];
    ["Offset of field: StgRegTable::rR2"][::core::mem::offset_of!(StgRegTable, rR2) - 8usize];
    ["Offset of field: StgRegTable::rR3"][::core::mem::offset_of!(StgRegTable, rR3) - 16usize];
    ["Offset of field: StgRegTable::rR4"][::core::mem::offset_of!(StgRegTable, rR4) - 24usize];
    ["Offset of field: StgRegTable::rR5"][::core::mem::offset_of!(StgRegTable, rR5) - 32usize];
    ["Offset of field: StgRegTable::rR6"][::core::mem::offset_of!(StgRegTable, rR6) - 40usize];
    ["Offset of field: StgRegTable::rR7"][::core::mem::offset_of!(StgRegTable, rR7) - 48usize];
    ["Offset of field: StgRegTable::rR8"][::core::mem::offset_of!(StgRegTable, rR8) - 56usize];
    ["Offset of field: StgRegTable::rR9"][::core::mem::offset_of!(StgRegTable, rR9) - 64usize];
    ["Offset of field: StgRegTable::rR10"][::core::mem::offset_of!(StgRegTable, rR10) - 72usize];
    ["Offset of field: StgRegTable::rF1"][::core::mem::offset_of!(StgRegTable, rF1) - 80usize];
    ["Offset of field: StgRegTable::rF2"][::core::mem::offset_of!(StgRegTable, rF2) - 84usize];
    ["Offset of field: StgRegTable::rF3"][::core::mem::offset_of!(StgRegTable, rF3) - 88usize];
    ["Offset of field: StgRegTable::rF4"][::core::mem::offset_of!(StgRegTable, rF4) - 92usize];
    ["Offset of field: StgRegTable::rF5"][::core::mem::offset_of!(StgRegTable, rF5) - 96usize];
    ["Offset of field: StgRegTable::rF6"][::core::mem::offset_of!(StgRegTable, rF6) - 100usize];
    ["Offset of field: StgRegTable::rD1"][::core::mem::offset_of!(StgRegTable, rD1) - 104usize];
    ["Offset of field: StgRegTable::rD2"][::core::mem::offset_of!(StgRegTable, rD2) - 112usize];
    ["Offset of field: StgRegTable::rD3"][::core::mem::offset_of!(StgRegTable, rD3) - 120usize];
    ["Offset of field: StgRegTable::rD4"][::core::mem::offset_of!(StgRegTable, rD4) - 128usize];
    ["Offset of field: StgRegTable::rD5"][::core::mem::offset_of!(StgRegTable, rD5) - 136usize];
    ["Offset of field: StgRegTable::rD6"][::core::mem::offset_of!(StgRegTable, rD6) - 144usize];
    ["Offset of field: StgRegTable::rXMM1"][::core::mem::offset_of!(StgRegTable, rXMM1) - 152usize];
    ["Offset of field: StgRegTable::rXMM2"][::core::mem::offset_of!(StgRegTable, rXMM2) - 168usize];
    ["Offset of field: StgRegTable::rXMM3"][::core::mem::offset_of!(StgRegTable, rXMM3) - 184usize];
    ["Offset of field: StgRegTable::rXMM4"][::core::mem::offset_of!(StgRegTable, rXMM4) - 200usize];
    ["Offset of field: StgRegTable::rXMM5"][::core::mem::offset_of!(StgRegTable, rXMM5) - 216usize];
    ["Offset of field: StgRegTable::rXMM6"][::core::mem::offset_of!(StgRegTable, rXMM6) - 232usize];
    ["Offset of field: StgRegTable::rYMM1"][::core::mem::offset_of!(StgRegTable, rYMM1) - 248usize];
    ["Offset of field: StgRegTable::rYMM2"][::core::mem::offset_of!(StgRegTable, rYMM2) - 280usize];
    ["Offset of field: StgRegTable::rYMM3"][::core::mem::offset_of!(StgRegTable, rYMM3) - 312usize];
    ["Offset of field: StgRegTable::rYMM4"][::core::mem::offset_of!(StgRegTable, rYMM4) - 344usize];
    ["Offset of field: StgRegTable::rYMM5"][::core::mem::offset_of!(StgRegTable, rYMM5) - 376usize];
    ["Offset of field: StgRegTable::rYMM6"][::core::mem::offset_of!(StgRegTable, rYMM6) - 408usize];
    ["Offset of field: StgRegTable::rZMM1"][::core::mem::offset_of!(StgRegTable, rZMM1) - 440usize];
    ["Offset of field: StgRegTable::rZMM2"][::core::mem::offset_of!(StgRegTable, rZMM2) - 504usize];
    ["Offset of field: StgRegTable::rZMM3"][::core::mem::offset_of!(StgRegTable, rZMM3) - 568usize];
    ["Offset of field: StgRegTable::rZMM4"][::core::mem::offset_of!(StgRegTable, rZMM4) - 632usize];
    ["Offset of field: StgRegTable::rZMM5"][::core::mem::offset_of!(StgRegTable, rZMM5) - 696usize];
    ["Offset of field: StgRegTable::rZMM6"][::core::mem::offset_of!(StgRegTable, rZMM6) - 760usize];
    ["Offset of field: StgRegTable::rL1"][::core::mem::offset_of!(StgRegTable, rL1) - 824usize];
    ["Offset of field: StgRegTable::rSp"][::core::mem::offset_of!(StgRegTable, rSp) - 832usize];
    ["Offset of field: StgRegTable::rSpLim"]
        [::core::mem::offset_of!(StgRegTable, rSpLim) - 840usize];
    ["Offset of field: StgRegTable::rHp"][::core::mem::offset_of!(StgRegTable, rHp) - 848usize];
    ["Offset of field: StgRegTable::rHpLim"]
        [::core::mem::offset_of!(StgRegTable, rHpLim) - 856usize];
    ["Offset of field: StgRegTable::rCCCS"][::core::mem::offset_of!(StgRegTable, rCCCS) - 864usize];
    ["Offset of field: StgRegTable::rCurrentTSO"]
        [::core::mem::offset_of!(StgRegTable, rCurrentTSO) - 872usize];
    ["Offset of field: StgRegTable::rNursery"]
        [::core::mem::offset_of!(StgRegTable, rNursery) - 880usize];
    ["Offset of field: StgRegTable::rCurrentNursery"]
        [::core::mem::offset_of!(StgRegTable, rCurrentNursery) - 888usize];
    ["Offset of field: StgRegTable::rCurrentAlloc"]
        [::core::mem::offset_of!(StgRegTable, rCurrentAlloc) - 896usize];
    ["Offset of field: StgRegTable::rHpAlloc"]
        [::core::mem::offset_of!(StgRegTable, rHpAlloc) - 904usize];
    ["Offset of field: StgRegTable::rRet"][::core::mem::offset_of!(StgRegTable, rRet) - 912usize];
};
