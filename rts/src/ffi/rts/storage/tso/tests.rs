#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_StgTSOProfInfo_layout() {
    assert_eq!(
        size_of::<*mut CostCentreStack>(),
        size_of::<*mut sys::CostCentreStack>()
    );
    assert_eq!(
        offset_of!(StgTSOProfInfo, cccs),
        offset_of!(sys::StgTSOProfInfo, cccs)
    );
    assert_eq!(
        size_of::<StgTSOProfInfo>(),
        size_of::<sys::StgTSOProfInfo>()
    );
    assert_eq!(
        align_of::<StgTSOProfInfo>(),
        align_of::<sys::StgTSOProfInfo>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgThreadID_layout() {
    assert_eq!(size_of::<StgThreadID>(), size_of::<StgThreadID>());
    assert_eq!(align_of::<StgThreadID>(), align_of::<StgThreadID>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgTSOBlockInfo_layout() {
    assert_eq!(
        size_of::<StgTSOBlockInfo>(),
        size_of::<sys::StgTSOBlockInfo>()
    );
    assert_eq!(
        align_of::<StgTSOBlockInfo>(),
        align_of::<sys::StgTSOBlockInfo>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgStack_layout() {
    assert_eq!(size_of::<StgStack>(), size_of::<sys::StgStack>());
    assert_eq!(align_of::<StgStack>(), align_of::<sys::StgStack>());
}
