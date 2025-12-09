use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_StgProfHeader__bindgen_ty_1_layout() {
    assert_eq!(
        size_of::<StgProfHeader__bindgen_ty_1>(),
        size_of::<sys::StgProfHeader__bindgen_ty_1>()
    );
    assert_eq!(
        align_of::<StgProfHeader__bindgen_ty_1>(),
        align_of::<sys::StgProfHeader__bindgen_ty_1>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgHeader_layout() {
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(StgHeader, info),
        offset_of!(sys::StgHeader, info)
    );
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(align_of::<StgHeader>(), align_of::<sys::StgHeader>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgClosurePtr_layout() {
    assert_eq!(size_of::<StgClosurePtr>(), size_of::<sys::StgClosurePtr>());
    assert_eq!(
        align_of::<StgClosurePtr>(),
        align_of::<sys::StgClosurePtr>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgAP_layout() {
    assert_eq!(
        size_of::<StgThunkHeader>(),
        size_of::<sys::StgThunkHeader>()
    );
    assert_eq!(offset_of!(StgAP, header), offset_of!(sys::StgAP, header));
    assert_eq!(offset_of!(StgAP, arity), offset_of!(sys::StgAP, arity));
    assert_eq!(offset_of!(StgAP, n_args), offset_of!(sys::StgAP, n_args));
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(offset_of!(StgAP, fun), offset_of!(sys::StgAP, fun));
    assert_eq!(
        size_of::<__IncompleteArrayField<*mut StgClosure>>(),
        size_of::<__IncompleteArrayField<*mut sys::StgClosure>>()
    );
    assert_eq!(offset_of!(StgAP, payload), offset_of!(sys::StgAP, payload));
    assert_eq!(size_of::<StgAP>(), size_of::<sys::StgAP>());
    assert_eq!(align_of::<StgAP>(), align_of::<sys::StgAP>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgInd_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(offset_of!(StgInd, header), offset_of!(sys::StgInd, header));
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgInd, indirectee),
        offset_of!(sys::StgInd, indirectee)
    );
    assert_eq!(size_of::<StgInd>(), size_of::<sys::StgInd>());
    assert_eq!(align_of::<StgInd>(), align_of::<sys::StgInd>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgArrBytes_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgArrBytes, header),
        offset_of!(sys::StgArrBytes, header)
    );
    assert_eq!(
        offset_of!(StgArrBytes, bytes),
        offset_of!(sys::StgArrBytes, bytes)
    );
    assert_eq!(
        offset_of!(StgArrBytes, payload),
        offset_of!(sys::StgArrBytes, payload)
    );
    assert_eq!(size_of::<StgArrBytes>(), size_of::<sys::StgArrBytes>());
    assert_eq!(align_of::<StgArrBytes>(), align_of::<sys::StgArrBytes>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgMutArrPtrs_layout() {
    assert_eq!(size_of::<StgMutArrPtrs>(), size_of::<sys::StgMutArrPtrs>());
    assert_eq!(
        align_of::<StgMutArrPtrs>(),
        align_of::<sys::StgMutArrPtrs>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgUpdateFrame_layout() {
    assert_eq!(
        size_of::<StgUpdateFrame>(),
        size_of::<sys::StgUpdateFrame>()
    );
    assert_eq!(
        align_of::<StgUpdateFrame>(),
        align_of::<sys::StgUpdateFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgCatchFrame_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgCatchFrame, header),
        offset_of!(sys::StgCatchFrame, header)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgCatchFrame, handler),
        offset_of!(sys::StgCatchFrame, handler)
    );
    assert_eq!(size_of::<StgCatchFrame>(), size_of::<sys::StgCatchFrame>());
    assert_eq!(
        align_of::<StgCatchFrame>(),
        align_of::<sys::StgCatchFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgUnderflowFrame_layout() {
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(StgUnderflowFrame, info),
        offset_of!(sys::StgUnderflowFrame, info)
    );
    assert_eq!(
        size_of::<*mut StgStack_>(),
        size_of::<*mut sys::StgStack_>()
    );
    assert_eq!(
        offset_of!(StgUnderflowFrame, next_chunk),
        offset_of!(sys::StgUnderflowFrame, next_chunk)
    );
    assert_eq!(
        size_of::<StgUnderflowFrame>(),
        size_of::<sys::StgUnderflowFrame>()
    );
    assert_eq!(
        align_of::<StgUnderflowFrame>(),
        align_of::<sys::StgUnderflowFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgStopFrame_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgStopFrame, header),
        offset_of!(sys::StgStopFrame, header)
    );
    assert_eq!(size_of::<StgStopFrame>(), size_of::<sys::StgStopFrame>());
    assert_eq!(align_of::<StgStopFrame>(), align_of::<sys::StgStopFrame>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgRetFun_layout() {
    assert_eq!(
        size_of::<*const StgInfoTable>(),
        size_of::<*const sys::StgInfoTable>()
    );
    assert_eq!(
        offset_of!(StgRetFun, info),
        offset_of!(sys::StgRetFun, info)
    );
    assert_eq!(
        offset_of!(StgRetFun, size),
        offset_of!(sys::StgRetFun, size)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(offset_of!(StgRetFun, fun), offset_of!(sys::StgRetFun, fun));
    assert_eq!(
        size_of::<__IncompleteArrayField<*mut StgClosure>>(),
        size_of::<__IncompleteArrayField<*mut sys::StgClosure>>()
    );
    assert_eq!(
        offset_of!(StgRetFun, payload),
        offset_of!(sys::StgRetFun, payload)
    );
    assert_eq!(size_of::<StgRetFun>(), size_of::<sys::StgRetFun>());
    assert_eq!(align_of::<StgRetFun>(), align_of::<sys::StgRetFun>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgBCO_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(offset_of!(StgBCO, header), offset_of!(sys::StgBCO, header));
    assert_eq!(
        size_of::<*mut StgArrBytes>(),
        size_of::<*mut sys::StgArrBytes>()
    );
    assert_eq!(offset_of!(StgBCO, instrs), offset_of!(sys::StgBCO, instrs));
    assert_eq!(
        size_of::<*mut StgArrBytes>(),
        size_of::<*mut sys::StgArrBytes>()
    );
    assert_eq!(
        offset_of!(StgBCO, literals),
        offset_of!(sys::StgBCO, literals)
    );
    assert_eq!(
        size_of::<*mut StgMutArrPtrs>(),
        size_of::<*mut sys::StgMutArrPtrs>()
    );
    assert_eq!(offset_of!(StgBCO, ptrs), offset_of!(sys::StgBCO, ptrs));
    assert_eq!(offset_of!(StgBCO, arity), offset_of!(sys::StgBCO, arity));
    assert_eq!(offset_of!(StgBCO, size), offset_of!(sys::StgBCO, size));
    assert_eq!(offset_of!(StgBCO, bitmap), offset_of!(sys::StgBCO, bitmap));
    assert_eq!(size_of::<StgBCO>(), size_of::<sys::StgBCO>());
    assert_eq!(align_of::<StgBCO>(), align_of::<sys::StgBCO>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgMVar_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgMVar, header),
        offset_of!(sys::StgMVar, header)
    );
    assert_eq!(
        size_of::<*mut StgMVarTSOQueue_>(),
        size_of::<*mut sys::StgMVarTSOQueue_>()
    );
    assert_eq!(offset_of!(StgMVar, head), offset_of!(sys::StgMVar, head));
    assert_eq!(
        size_of::<*mut StgMVarTSOQueue_>(),
        size_of::<*mut sys::StgMVarTSOQueue_>()
    );
    assert_eq!(offset_of!(StgMVar, tail), offset_of!(sys::StgMVar, tail));
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(offset_of!(StgMVar, value), offset_of!(sys::StgMVar, value));
    assert_eq!(size_of::<StgMVar>(), size_of::<sys::StgMVar>());
    assert_eq!(align_of::<StgMVar>(), align_of::<sys::StgMVar>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgAtomicallyFrame_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgAtomicallyFrame, header),
        offset_of!(sys::StgAtomicallyFrame, header)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgAtomicallyFrame, code),
        offset_of!(sys::StgAtomicallyFrame, code)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgAtomicallyFrame, result),
        offset_of!(sys::StgAtomicallyFrame, result)
    );
    assert_eq!(
        size_of::<StgAtomicallyFrame>(),
        size_of::<sys::StgAtomicallyFrame>()
    );
    assert_eq!(
        align_of::<StgAtomicallyFrame>(),
        align_of::<sys::StgAtomicallyFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgCatchSTMFrame_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgCatchSTMFrame, header),
        offset_of!(sys::StgCatchSTMFrame, header)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgCatchSTMFrame, code),
        offset_of!(sys::StgCatchSTMFrame, code)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgCatchSTMFrame, handler),
        offset_of!(sys::StgCatchSTMFrame, handler)
    );
    assert_eq!(
        size_of::<StgCatchSTMFrame>(),
        size_of::<sys::StgCatchSTMFrame>()
    );
    assert_eq!(
        align_of::<StgCatchSTMFrame>(),
        align_of::<sys::StgCatchSTMFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_StgCatchRetryFrame_layout() {
    assert_eq!(size_of::<StgHeader>(), size_of::<sys::StgHeader>());
    assert_eq!(
        offset_of!(StgCatchRetryFrame, header),
        offset_of!(sys::StgCatchRetryFrame, header)
    );
    assert_eq!(
        offset_of!(StgCatchRetryFrame, running_alt_code),
        offset_of!(sys::StgCatchRetryFrame, running_alt_code)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgCatchRetryFrame, first_code),
        offset_of!(sys::StgCatchRetryFrame, first_code)
    );
    assert_eq!(
        size_of::<*mut StgClosure>(),
        size_of::<*mut sys::StgClosure>()
    );
    assert_eq!(
        offset_of!(StgCatchRetryFrame, alt_code),
        offset_of!(sys::StgCatchRetryFrame, alt_code)
    );
    assert_eq!(
        size_of::<StgCatchRetryFrame>(),
        size_of::<sys::StgCatchRetryFrame>()
    );
    assert_eq!(
        align_of::<StgCatchRetryFrame>(),
        align_of::<sys::StgCatchRetryFrame>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_Message_layout() {
    assert_eq!(size_of::<Message>(), size_of::<sys::Message>());
    assert_eq!(align_of::<Message>(), align_of::<sys::Message>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_MessageCloneStack_layout() {
    assert_eq!(
        size_of::<MessageCloneStack>(),
        size_of::<sys::MessageCloneStack>()
    );
    assert_eq!(
        align_of::<MessageCloneStack>(),
        align_of::<sys::MessageCloneStack>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_hashtable_layout() {
    assert_eq!(
        offset_of!(hashtable, _address),
        offset_of!(sys::hashtable, _address)
    );
    assert_eq!(size_of::<hashtable>(), size_of::<sys::hashtable>());
    assert_eq!(align_of::<hashtable>(), align_of::<sys::hashtable>());
}
