#![cfg_attr(not(feature = "sys"), expect(unused_imports))]
use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_HsIface_layout() {
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, processRemoteCompletion_closure),
        offset_of!(sys::HsIface, processRemoteCompletion_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runIO_closure),
        offset_of!(sys::HsIface, runIO_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runNonIO_closure),
        offset_of!(sys::HsIface, runNonIO_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, Z0T_closure),
        offset_of!(sys::HsIface, Z0T_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, True_closure),
        offset_of!(sys::HsIface, True_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, False_closure),
        offset_of!(sys::HsIface, False_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, unpackCString_closure),
        offset_of!(sys::HsIface, unpackCString_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runFinalizzerBatch_closure),
        offset_of!(sys::HsIface, runFinalizzerBatch_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, stackOverflow_closure),
        offset_of!(sys::HsIface, stackOverflow_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, heapOverflow_closure),
        offset_of!(sys::HsIface, heapOverflow_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, allocationLimitExceeded_closure),
        offset_of!(sys::HsIface, allocationLimitExceeded_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, blockedIndefinitelyOnMVar_closure),
        offset_of!(sys::HsIface, blockedIndefinitelyOnMVar_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, blockedIndefinitelyOnSTM_closure),
        offset_of!(sys::HsIface, blockedIndefinitelyOnSTM_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, cannotCompactFunction_closure),
        offset_of!(sys::HsIface, cannotCompactFunction_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, cannotCompactPinned_closure),
        offset_of!(sys::HsIface, cannotCompactPinned_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, cannotCompactMutable_closure),
        offset_of!(sys::HsIface, cannotCompactMutable_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, nonTermination_closure),
        offset_of!(sys::HsIface, nonTermination_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, nestedAtomically_closure),
        offset_of!(sys::HsIface, nestedAtomically_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, noMatchingContinuationPrompt_closure),
        offset_of!(sys::HsIface, noMatchingContinuationPrompt_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, blockedOnBadFD_closure),
        offset_of!(sys::HsIface, blockedOnBadFD_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runSparks_closure),
        offset_of!(sys::HsIface, runSparks_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, ensureIOManagerIsRunning_closure),
        offset_of!(sys::HsIface, ensureIOManagerIsRunning_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, interruptIOManager_closure),
        offset_of!(sys::HsIface, interruptIOManager_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, ioManagerCapabilitiesChanged_closure),
        offset_of!(sys::HsIface, ioManagerCapabilitiesChanged_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runHandlersPtr_closure),
        offset_of!(sys::HsIface, runHandlersPtr_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, flushStdHandles_closure),
        offset_of!(sys::HsIface, flushStdHandles_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runMainIO_closure),
        offset_of!(sys::HsIface, runMainIO_closure)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Czh_con_info),
        offset_of!(sys::HsIface, Czh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Izh_con_info),
        offset_of!(sys::HsIface, Izh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Fzh_con_info),
        offset_of!(sys::HsIface, Fzh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Dzh_con_info),
        offset_of!(sys::HsIface, Dzh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Wzh_con_info),
        offset_of!(sys::HsIface, Wzh_con_info)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, absentSumFieldError_closure),
        offset_of!(sys::HsIface, absentSumFieldError_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, runAllocationLimitHandler_closure),
        offset_of!(sys::HsIface, runAllocationLimitHandler_closure)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, Ptr_con_info),
        offset_of!(sys::HsIface, Ptr_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, FunPtr_con_info),
        offset_of!(sys::HsIface, FunPtr_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, I8zh_con_info),
        offset_of!(sys::HsIface, I8zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, I16zh_con_info),
        offset_of!(sys::HsIface, I16zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, I32zh_con_info),
        offset_of!(sys::HsIface, I32zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, I64zh_con_info),
        offset_of!(sys::HsIface, I64zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, W8zh_con_info),
        offset_of!(sys::HsIface, W8zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, W16zh_con_info),
        offset_of!(sys::HsIface, W16zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, W32zh_con_info),
        offset_of!(sys::HsIface, W32zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, W64zh_con_info),
        offset_of!(sys::HsIface, W64zh_con_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, StablePtr_con_info),
        offset_of!(sys::HsIface, StablePtr_con_info)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, StackSnapshot_closure),
        offset_of!(sys::HsIface, StackSnapshot_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, divZZeroException_closure),
        offset_of!(sys::HsIface, divZZeroException_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, underflowException_closure),
        offset_of!(sys::HsIface, underflowException_closure)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(HsIface, overflowException_closure),
        offset_of!(sys::HsIface, overflowException_closure)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, unpackCStringzh_info),
        offset_of!(sys::HsIface, unpackCStringzh_info)
    );
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(HsIface, unpackCStringUtf8zh_info),
        offset_of!(sys::HsIface, unpackCStringUtf8zh_info)
    );
    assert_eq!(size_of::<HsIface>(), size_of::<sys::HsIface>());
    assert_eq!(align_of::<HsIface>(), align_of::<sys::HsIface>());
}

#[cfg(feature = "sys")]
#[test]
#[expect(static_mut_refs)]
fn sys_ghc_hs_iface_layout() {
    assert_eq!(
        size_of_val(unsafe { &ghc_hs_iface }),
        size_of_val(unsafe { &sys::ghc_hs_iface })
    );
    assert_eq!(
        align_of_val(unsafe { &ghc_hs_iface }),
        align_of_val(unsafe { &sys::ghc_hs_iface })
    );
}
