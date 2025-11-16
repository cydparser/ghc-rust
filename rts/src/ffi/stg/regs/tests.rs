use super::*;
#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgFunTable() {
    assert_eq!(size_of::<sys::StgFunTable>(), size_of::<StgFunTable>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgFunTable"][size_of::<StgFunTable>() - 24usize];
    ["Alignment of StgFunTable"][align_of::<StgFunTable>() - 8usize];
    ["Offset of field: StgFunTable::stgEagerBlackholeInfo"]
        [offset_of!(StgFunTable, stgEagerBlackholeInfo) - 0usize];
    ["Offset of field: StgFunTable::stgGCEnter1"][offset_of!(StgFunTable, stgGCEnter1) - 8usize];
    ["Offset of field: StgFunTable::stgGCFun"][offset_of!(StgFunTable, stgGCFun) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgUnion() {
    assert_eq!(size_of::<sys::StgUnion>(), size_of::<StgUnion>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgUnion"][size_of::<StgUnion>() - 8usize];
    ["Alignment of StgUnion"][align_of::<StgUnion>() - 8usize];
    ["Offset of field: StgUnion::w"][offset_of!(StgUnion, w) - 0usize];
    ["Offset of field: StgUnion::a"][offset_of!(StgUnion, a) - 0usize];
    ["Offset of field: StgUnion::c"][offset_of!(StgUnion, c) - 0usize];
    ["Offset of field: StgUnion::f"][offset_of!(StgUnion, f) - 0usize];
    ["Offset of field: StgUnion::i"][offset_of!(StgUnion, i) - 0usize];
    ["Offset of field: StgUnion::p"][offset_of!(StgUnion, p) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgRegTable() {
    assert_eq!(size_of::<sys::StgRegTable>(), size_of::<StgRegTable>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRegTable"][size_of::<StgRegTable>() - 920usize];
    ["Alignment of StgRegTable"][align_of::<StgRegTable>() - 8usize];
    ["Offset of field: StgRegTable::rR1"][offset_of!(StgRegTable, rR1) - 0usize];
    ["Offset of field: StgRegTable::rR2"][offset_of!(StgRegTable, rR2) - 8usize];
    ["Offset of field: StgRegTable::rR3"][offset_of!(StgRegTable, rR3) - 16usize];
    ["Offset of field: StgRegTable::rR4"][offset_of!(StgRegTable, rR4) - 24usize];
    ["Offset of field: StgRegTable::rR5"][offset_of!(StgRegTable, rR5) - 32usize];
    ["Offset of field: StgRegTable::rR6"][offset_of!(StgRegTable, rR6) - 40usize];
    ["Offset of field: StgRegTable::rR7"][offset_of!(StgRegTable, rR7) - 48usize];
    ["Offset of field: StgRegTable::rR8"][offset_of!(StgRegTable, rR8) - 56usize];
    ["Offset of field: StgRegTable::rR9"][offset_of!(StgRegTable, rR9) - 64usize];
    ["Offset of field: StgRegTable::rR10"][offset_of!(StgRegTable, rR10) - 72usize];
    ["Offset of field: StgRegTable::rF1"][offset_of!(StgRegTable, rF1) - 80usize];
    ["Offset of field: StgRegTable::rF2"][offset_of!(StgRegTable, rF2) - 84usize];
    ["Offset of field: StgRegTable::rF3"][offset_of!(StgRegTable, rF3) - 88usize];
    ["Offset of field: StgRegTable::rF4"][offset_of!(StgRegTable, rF4) - 92usize];
    ["Offset of field: StgRegTable::rF5"][offset_of!(StgRegTable, rF5) - 96usize];
    ["Offset of field: StgRegTable::rF6"][offset_of!(StgRegTable, rF6) - 100usize];
    ["Offset of field: StgRegTable::rD1"][offset_of!(StgRegTable, rD1) - 104usize];
    ["Offset of field: StgRegTable::rD2"][offset_of!(StgRegTable, rD2) - 112usize];
    ["Offset of field: StgRegTable::rD3"][offset_of!(StgRegTable, rD3) - 120usize];
    ["Offset of field: StgRegTable::rD4"][offset_of!(StgRegTable, rD4) - 128usize];
    ["Offset of field: StgRegTable::rD5"][offset_of!(StgRegTable, rD5) - 136usize];
    ["Offset of field: StgRegTable::rD6"][offset_of!(StgRegTable, rD6) - 144usize];
    ["Offset of field: StgRegTable::rXMM1"][offset_of!(StgRegTable, rXMM1) - 152usize];
    ["Offset of field: StgRegTable::rXMM2"][offset_of!(StgRegTable, rXMM2) - 168usize];
    ["Offset of field: StgRegTable::rXMM3"][offset_of!(StgRegTable, rXMM3) - 184usize];
    ["Offset of field: StgRegTable::rXMM4"][offset_of!(StgRegTable, rXMM4) - 200usize];
    ["Offset of field: StgRegTable::rXMM5"][offset_of!(StgRegTable, rXMM5) - 216usize];
    ["Offset of field: StgRegTable::rXMM6"][offset_of!(StgRegTable, rXMM6) - 232usize];
    ["Offset of field: StgRegTable::rYMM1"][offset_of!(StgRegTable, rYMM1) - 248usize];
    ["Offset of field: StgRegTable::rYMM2"][offset_of!(StgRegTable, rYMM2) - 280usize];
    ["Offset of field: StgRegTable::rYMM3"][offset_of!(StgRegTable, rYMM3) - 312usize];
    ["Offset of field: StgRegTable::rYMM4"][offset_of!(StgRegTable, rYMM4) - 344usize];
    ["Offset of field: StgRegTable::rYMM5"][offset_of!(StgRegTable, rYMM5) - 376usize];
    ["Offset of field: StgRegTable::rYMM6"][offset_of!(StgRegTable, rYMM6) - 408usize];
    ["Offset of field: StgRegTable::rZMM1"][offset_of!(StgRegTable, rZMM1) - 440usize];
    ["Offset of field: StgRegTable::rZMM2"][offset_of!(StgRegTable, rZMM2) - 504usize];
    ["Offset of field: StgRegTable::rZMM3"][offset_of!(StgRegTable, rZMM3) - 568usize];
    ["Offset of field: StgRegTable::rZMM4"][offset_of!(StgRegTable, rZMM4) - 632usize];
    ["Offset of field: StgRegTable::rZMM5"][offset_of!(StgRegTable, rZMM5) - 696usize];
    ["Offset of field: StgRegTable::rZMM6"][offset_of!(StgRegTable, rZMM6) - 760usize];
    ["Offset of field: StgRegTable::rL1"][offset_of!(StgRegTable, rL1) - 824usize];
    ["Offset of field: StgRegTable::rSp"][offset_of!(StgRegTable, rSp) - 832usize];
    ["Offset of field: StgRegTable::rSpLim"][offset_of!(StgRegTable, rSpLim) - 840usize];
    ["Offset of field: StgRegTable::rHp"][offset_of!(StgRegTable, rHp) - 848usize];
    ["Offset of field: StgRegTable::rHpLim"][offset_of!(StgRegTable, rHpLim) - 856usize];
    ["Offset of field: StgRegTable::rCCCS"][offset_of!(StgRegTable, rCCCS) - 864usize];
    ["Offset of field: StgRegTable::rCurrentTSO"][offset_of!(StgRegTable, rCurrentTSO) - 872usize];
    ["Offset of field: StgRegTable::rNursery"][offset_of!(StgRegTable, rNursery) - 880usize];
    ["Offset of field: StgRegTable::rCurrentNursery"]
        [offset_of!(StgRegTable, rCurrentNursery) - 888usize];
    ["Offset of field: StgRegTable::rCurrentAlloc"]
        [offset_of!(StgRegTable, rCurrentAlloc) - 896usize];
    ["Offset of field: StgRegTable::rHpAlloc"][offset_of!(StgRegTable, rHpAlloc) - 904usize];
    ["Offset of field: StgRegTable::rRet"][offset_of!(StgRegTable, rRet) - 912usize];
};
