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
fn sys_StgTSO__layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgTSO_, header),
        offset_of!(sys::StgTSO_, header)
    );
    assert_eq!(size_of::<*mut StgTSO_>(), size_of::<*mut sys::StgTSO_>());
    assert_eq!(offset_of!(StgTSO_, _link), offset_of!(sys::StgTSO_, _link));
    assert_eq!(size_of::<*mut StgTSO_>(), size_of::<*mut sys::StgTSO_>());
    assert_eq!(
        offset_of!(StgTSO_, global_link),
        offset_of!(sys::StgTSO_, global_link)
    );
    assert_eq!(
        size_of::<*mut StgStack_>(),
        size_of::<*mut sys::StgStack_>()
    );
    assert_eq!(
        offset_of!(StgTSO_, stackobj),
        offset_of!(sys::StgTSO_, stackobj)
    );
    assert_eq!(
        offset_of!(StgTSO_, what_next),
        offset_of!(sys::StgTSO_, what_next)
    );
    assert_eq!(offset_of!(StgTSO_, flags), offset_of!(sys::StgTSO_, flags));
    assert_eq!(
        offset_of!(StgTSO_, why_blocked),
        offset_of!(sys::StgTSO_, why_blocked)
    );
    assert_eq!(
        size_of::<StgTSOBlockInfo>(),
        size_of::<sys::StgTSOBlockInfo>()
    );
    assert_eq!(
        offset_of!(StgTSO_, block_info),
        offset_of!(sys::StgTSO_, block_info)
    );
    assert_eq!(offset_of!(StgTSO_, id), offset_of!(sys::StgTSO_, id));
    assert_eq!(
        offset_of!(StgTSO_, saved_errno),
        offset_of!(sys::StgTSO_, saved_errno)
    );
    assert_eq!(offset_of!(StgTSO_, dirty), offset_of!(sys::StgTSO_, dirty));
    assert_eq!(size_of::<*mut InCall_>(), size_of::<*mut sys::InCall_>());
    assert_eq!(offset_of!(StgTSO_, bound), offset_of!(sys::StgTSO_, bound));
    assert_eq!(
        size_of::<*mut Capability_>(),
        size_of::<*mut sys::Capability_>()
    );
    assert_eq!(offset_of!(StgTSO_, cap), offset_of!(sys::StgTSO_, cap));
    assert_eq!(
        size_of::<*mut StgTRecHeader_>(),
        size_of::<*mut sys::StgTRecHeader_>()
    );
    assert_eq!(offset_of!(StgTSO_, trec), offset_of!(sys::StgTSO_, trec));
    assert_eq!(
        size_of::<*mut StgArrBytes>(),
        size_of::<*mut sys::StgArrBytes>()
    );
    assert_eq!(offset_of!(StgTSO_, label), offset_of!(sys::StgTSO_, label));
    assert_eq!(
        size_of::<*mut MessageThrowTo_>(),
        size_of::<*mut sys::MessageThrowTo_>()
    );
    assert_eq!(
        offset_of!(StgTSO_, blocked_exceptions),
        offset_of!(sys::StgTSO_, blocked_exceptions)
    );
    assert_eq!(
        size_of::<*mut StgBlockingQueue_>(),
        size_of::<*mut sys::StgBlockingQueue_>()
    );
    assert_eq!(offset_of!(StgTSO_, bq), offset_of!(sys::StgTSO_, bq));
    assert_eq!(
        offset_of!(StgTSO_, alloc_limit),
        offset_of!(sys::StgTSO_, alloc_limit)
    );
    assert_eq!(
        offset_of!(StgTSO_, tot_stack_size),
        offset_of!(sys::StgTSO_, tot_stack_size)
    );
    assert_eq!(size_of::<StgTSO_>(), size_of::<sys::StgTSO_>());
    assert_eq!(align_of::<StgTSO_>(), align_of::<sys::StgTSO_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgStack__layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgStack_, header),
        offset_of!(sys::StgStack_, header)
    );
    assert_eq!(
        offset_of!(StgStack_, stack_size),
        offset_of!(sys::StgStack_, stack_size)
    );
    assert_eq!(
        offset_of!(StgStack_, dirty),
        offset_of!(sys::StgStack_, dirty)
    );
    assert_eq!(
        offset_of!(StgStack_, marking),
        offset_of!(sys::StgStack_, marking)
    );
    assert_eq!(offset_of!(StgStack_, sp), offset_of!(sys::StgStack_, sp));
    assert_eq!(
        offset_of!(StgStack_, stack),
        offset_of!(sys::StgStack_, stack)
    );
    assert_eq!(size_of::<StgStack_>(), size_of::<sys::StgStack_>());
    assert_eq!(align_of::<StgStack_>(), align_of::<sys::StgStack_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgStack_layout() {
    assert_eq!(size_of::<StgStack>(), size_of::<sys::StgStack>());
    assert_eq!(align_of::<StgStack>(), align_of::<sys::StgStack>());
}
