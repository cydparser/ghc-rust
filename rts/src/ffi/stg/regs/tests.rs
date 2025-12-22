#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;
use crate::prelude::*;

#[cfg(feature = "sys")]
#[test]
fn sys_StgFunTable_layout() {
    assert_eq!(
        offset_of!(StgFunTable, stgEagerBlackholeInfo),
        offset_of!(sys::StgFunTable, stgEagerBlackholeInfo)
    );
    assert_eq!(
        offset_of!(StgFunTable, stgGCEnter1),
        offset_of!(sys::StgFunTable, stgGCEnter1)
    );
    assert_eq!(
        offset_of!(StgFunTable, stgGCFun),
        offset_of!(sys::StgFunTable, stgGCFun)
    );
    assert_eq!(size_of::<StgFunTable>(), size_of::<sys::StgFunTable>());
    assert_eq!(align_of::<StgFunTable>(), align_of::<sys::StgFunTable>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgUnion_layout() {
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(align_of::<StgUnion>(), align_of::<sys::StgUnion>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgRegTable_layout() {
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR1),
        offset_of!(sys::StgRegTable, rR1)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR2),
        offset_of!(sys::StgRegTable, rR2)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR3),
        offset_of!(sys::StgRegTable, rR3)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR4),
        offset_of!(sys::StgRegTable, rR4)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR5),
        offset_of!(sys::StgRegTable, rR5)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR6),
        offset_of!(sys::StgRegTable, rR6)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR7),
        offset_of!(sys::StgRegTable, rR7)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR8),
        offset_of!(sys::StgRegTable, rR8)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR9),
        offset_of!(sys::StgRegTable, rR9)
    );
    assert_eq!(size_of::<StgUnion>(), size_of::<sys::StgUnion>());
    assert_eq!(
        offset_of!(StgRegTable, rR10),
        offset_of!(sys::StgRegTable, rR10)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF1),
        offset_of!(sys::StgRegTable, rF1)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF2),
        offset_of!(sys::StgRegTable, rF2)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF3),
        offset_of!(sys::StgRegTable, rF3)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF4),
        offset_of!(sys::StgRegTable, rF4)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF5),
        offset_of!(sys::StgRegTable, rF5)
    );
    assert_eq!(
        offset_of!(StgRegTable, rF6),
        offset_of!(sys::StgRegTable, rF6)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD1),
        offset_of!(sys::StgRegTable, rD1)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD2),
        offset_of!(sys::StgRegTable, rD2)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD3),
        offset_of!(sys::StgRegTable, rD3)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD4),
        offset_of!(sys::StgRegTable, rD4)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD5),
        offset_of!(sys::StgRegTable, rD5)
    );
    assert_eq!(
        offset_of!(StgRegTable, rD6),
        offset_of!(sys::StgRegTable, rD6)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM1),
        offset_of!(sys::StgRegTable, rXMM1)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM2),
        offset_of!(sys::StgRegTable, rXMM2)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM3),
        offset_of!(sys::StgRegTable, rXMM3)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM4),
        offset_of!(sys::StgRegTable, rXMM4)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM5),
        offset_of!(sys::StgRegTable, rXMM5)
    );
    assert_eq!(size_of::<StgWord128>(), size_of::<sys::StgWord128>());
    assert_eq!(
        offset_of!(StgRegTable, rXMM6),
        offset_of!(sys::StgRegTable, rXMM6)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM1),
        offset_of!(sys::StgRegTable, rYMM1)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM2),
        offset_of!(sys::StgRegTable, rYMM2)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM3),
        offset_of!(sys::StgRegTable, rYMM3)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM4),
        offset_of!(sys::StgRegTable, rYMM4)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM5),
        offset_of!(sys::StgRegTable, rYMM5)
    );
    assert_eq!(size_of::<StgWord256>(), size_of::<sys::StgWord256>());
    assert_eq!(
        offset_of!(StgRegTable, rYMM6),
        offset_of!(sys::StgRegTable, rYMM6)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM1),
        offset_of!(sys::StgRegTable, rZMM1)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM2),
        offset_of!(sys::StgRegTable, rZMM2)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM3),
        offset_of!(sys::StgRegTable, rZMM3)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM4),
        offset_of!(sys::StgRegTable, rZMM4)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM5),
        offset_of!(sys::StgRegTable, rZMM5)
    );
    assert_eq!(size_of::<StgWord512>(), size_of::<sys::StgWord512>());
    assert_eq!(
        offset_of!(StgRegTable, rZMM6),
        offset_of!(sys::StgRegTable, rZMM6)
    );
    assert_eq!(
        offset_of!(StgRegTable, rL1),
        offset_of!(sys::StgRegTable, rL1)
    );
    assert_eq!(
        offset_of!(StgRegTable, rSp),
        offset_of!(sys::StgRegTable, rSp)
    );
    assert_eq!(
        offset_of!(StgRegTable, rSpLim),
        offset_of!(sys::StgRegTable, rSpLim)
    );
    assert_eq!(
        offset_of!(StgRegTable, rHp),
        offset_of!(sys::StgRegTable, rHp)
    );
    assert_eq!(
        offset_of!(StgRegTable, rHpLim),
        offset_of!(sys::StgRegTable, rHpLim)
    );
    assert_eq!(
        size_of::<*mut CostCentreStack_>(),
        size_of::<*mut sys::CostCentreStack_>()
    );
    assert_eq!(
        offset_of!(StgRegTable, rCCCS),
        offset_of!(sys::StgRegTable, rCCCS)
    );
    assert_eq!(size_of::<*mut StgTSO_>(), size_of::<*mut sys::StgTSO_>());
    assert_eq!(
        offset_of!(StgRegTable, rCurrentTSO),
        offset_of!(sys::StgRegTable, rCurrentTSO)
    );
    assert_eq!(size_of::<*mut nursery_>(), size_of::<*mut sys::nursery_>());
    assert_eq!(
        offset_of!(StgRegTable, rNursery),
        offset_of!(sys::StgRegTable, rNursery)
    );
    assert_eq!(size_of::<*mut bdescr_>(), size_of::<*mut sys::bdescr_>());
    assert_eq!(
        offset_of!(StgRegTable, rCurrentNursery),
        offset_of!(sys::StgRegTable, rCurrentNursery)
    );
    assert_eq!(size_of::<*mut bdescr_>(), size_of::<*mut sys::bdescr_>());
    assert_eq!(
        offset_of!(StgRegTable, rCurrentAlloc),
        offset_of!(sys::StgRegTable, rCurrentAlloc)
    );
    assert_eq!(
        offset_of!(StgRegTable, rHpAlloc),
        offset_of!(sys::StgRegTable, rHpAlloc)
    );
    assert_eq!(
        offset_of!(StgRegTable, rRet),
        offset_of!(sys::StgRegTable, rRet)
    );
    assert_eq!(size_of::<StgRegTable>(), size_of::<sys::StgRegTable>());
    assert_eq!(align_of::<StgRegTable>(), align_of::<sys::StgRegTable>());
}
