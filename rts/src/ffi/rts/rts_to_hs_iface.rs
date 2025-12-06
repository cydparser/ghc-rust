use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::rts::storage::info_tables::StgInfoTable;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc-lib)]
#[repr(C)]
#[derive(Debug)]
pub struct HsIface {
    pub processRemoteCompletion_closure: *mut StgClosure,
    pub runIO_closure: *mut StgClosure,
    pub runNonIO_closure: *mut StgClosure,
    pub Z0T_closure: *mut StgClosure,
    pub True_closure: *mut StgClosure,
    pub False_closure: *mut StgClosure,
    pub unpackCString_closure: *mut StgClosure,
    pub runFinalizzerBatch_closure: *mut StgClosure,
    pub stackOverflow_closure: *mut StgClosure,
    pub heapOverflow_closure: *mut StgClosure,
    pub allocationLimitExceeded_closure: *mut StgClosure,
    pub blockedIndefinitelyOnMVar_closure: *mut StgClosure,
    pub blockedIndefinitelyOnSTM_closure: *mut StgClosure,
    pub cannotCompactFunction_closure: *mut StgClosure,
    pub cannotCompactPinned_closure: *mut StgClosure,
    pub cannotCompactMutable_closure: *mut StgClosure,
    pub nonTermination_closure: *mut StgClosure,
    pub nestedAtomically_closure: *mut StgClosure,
    pub noMatchingContinuationPrompt_closure: *mut StgClosure,
    pub blockedOnBadFD_closure: *mut StgClosure,
    pub runSparks_closure: *mut StgClosure,
    pub ensureIOManagerIsRunning_closure: *mut StgClosure,
    pub interruptIOManager_closure: *mut StgClosure,
    pub ioManagerCapabilitiesChanged_closure: *mut StgClosure,
    pub runHandlersPtr_closure: *mut StgClosure,
    pub flushStdHandles_closure: *mut StgClosure,
    pub runMainIO_closure: *mut StgClosure,
    pub Czh_con_info: *const StgInfoTable,
    pub Izh_con_info: *const StgInfoTable,
    pub Fzh_con_info: *const StgInfoTable,
    pub Dzh_con_info: *const StgInfoTable,
    pub Wzh_con_info: *const StgInfoTable,
    pub absentSumFieldError_closure: *mut StgClosure,
    pub runAllocationLimitHandler_closure: *mut StgClosure,
    pub Ptr_con_info: *const StgInfoTable,
    pub FunPtr_con_info: *const StgInfoTable,
    pub I8zh_con_info: *const StgInfoTable,
    pub I16zh_con_info: *const StgInfoTable,
    pub I32zh_con_info: *const StgInfoTable,
    pub I64zh_con_info: *const StgInfoTable,
    pub W8zh_con_info: *const StgInfoTable,
    pub W16zh_con_info: *const StgInfoTable,
    pub W32zh_con_info: *const StgInfoTable,
    pub W64zh_con_info: *const StgInfoTable,
    pub StablePtr_con_info: *const StgInfoTable,
    pub StackSnapshot_closure: *mut StgClosure,
    pub divZZeroException_closure: *mut StgClosure,
    pub underflowException_closure: *mut StgClosure,
    pub overflowException_closure: *mut StgClosure,
    pub unpackCStringzh_info: *const StgInfoTable,
    pub unpackCStringUtf8zh_info: *const StgInfoTable,
}

#[ffi(compiler, ghc-lib)]
#[unsafe(no_mangle)]
pub static mut ghc_hs_iface: *mut HsIface = null_mut();
