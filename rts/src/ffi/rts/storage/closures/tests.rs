use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_eq_TREC_CHUNK_NUM_ENTRIES() {
    assert_eq!(sys::TREC_CHUNK_NUM_ENTRIES, TREC_CHUNK_NUM_ENTRIES);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgProfHeader() {
    assert_eq!(size_of::<sys::StgProfHeader>(), size_of::<StgProfHeader>())
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgProfHeader__bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgProfHeader__bindgen_ty_1>(),
        size_of::<StgProfHeader__bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfHeader__bindgen_ty_1"][size_of::<StgProfHeader__bindgen_ty_1>() - 8usize];
    ["Alignment of StgProfHeader__bindgen_ty_1"]
        [align_of::<StgProfHeader__bindgen_ty_1>() - 8usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::trav"]
        [offset_of!(StgProfHeader__bindgen_ty_1, trav) - 0usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::ldvw"]
        [offset_of!(StgProfHeader__bindgen_ty_1, ldvw) - 0usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::era"]
        [offset_of!(StgProfHeader__bindgen_ty_1, era) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfHeader"][size_of::<StgProfHeader>() - 16usize];
    ["Alignment of StgProfHeader"][align_of::<StgProfHeader>() - 8usize];
    ["Offset of field: StgProfHeader::ccs"][offset_of!(StgProfHeader, ccs) - 0usize];
    ["Offset of field: StgProfHeader::hp"][offset_of!(StgProfHeader, hp) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgSMPThunkHeader() {
    assert_eq!(
        size_of::<sys::StgSMPThunkHeader>(),
        size_of::<StgSMPThunkHeader>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSMPThunkHeader"][size_of::<StgSMPThunkHeader>() - 8usize];
    ["Alignment of StgSMPThunkHeader"][align_of::<StgSMPThunkHeader>() - 8usize];
    ["Offset of field: StgSMPThunkHeader::pad"][offset_of!(StgSMPThunkHeader, pad) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgHeader() {
    assert_eq!(size_of::<sys::StgHeader>(), size_of::<StgHeader>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgHeader"][size_of::<StgHeader>() - 8usize];
    ["Alignment of StgHeader"][align_of::<StgHeader>() - 8usize];
    ["Offset of field: StgHeader::info"][offset_of!(StgHeader, info) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgThunkHeader() {
    assert_eq!(
        size_of::<sys::StgThunkHeader>(),
        size_of::<StgThunkHeader>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunkHeader"][size_of::<StgThunkHeader>() - 16usize];
    ["Alignment of StgThunkHeader"][align_of::<StgThunkHeader>() - 8usize];
    ["Offset of field: StgThunkHeader::info"][offset_of!(StgThunkHeader, info) - 0usize];
    ["Offset of field: StgThunkHeader::smp"][offset_of!(StgThunkHeader, smp) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgClosure_() {
    assert_eq!(size_of::<sys::StgClosure_>(), size_of::<StgClosure_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosure_"][size_of::<StgClosure_>() - 8usize];
    ["Alignment of StgClosure_"][align_of::<StgClosure_>() - 8usize];
    ["Offset of field: StgClosure_::header"][offset_of!(StgClosure_, header) - 0usize];
    ["Offset of field: StgClosure_::payload"][offset_of!(StgClosure_, payload) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgThunk_() {
    assert_eq!(size_of::<sys::StgThunk_>(), size_of::<StgThunk_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunk_"][size_of::<StgThunk_>() - 16usize];
    ["Alignment of StgThunk_"][align_of::<StgThunk_>() - 8usize];
    ["Offset of field: StgThunk_::header"][offset_of!(StgThunk_, header) - 0usize];
    ["Offset of field: StgThunk_::payload"][offset_of!(StgThunk_, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgSelector() {
    assert_eq!(size_of::<sys::StgSelector>(), size_of::<StgSelector>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSelector"][size_of::<StgSelector>() - 24usize];
    ["Alignment of StgSelector"][align_of::<StgSelector>() - 8usize];
    ["Offset of field: StgSelector::header"][offset_of!(StgSelector, header) - 0usize];
    ["Offset of field: StgSelector::selectee"][offset_of!(StgSelector, selectee) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgPAP() {
    assert_eq!(size_of::<sys::StgPAP>(), size_of::<StgPAP>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgPAP"][size_of::<StgPAP>() - 24usize];
    ["Alignment of StgPAP"][align_of::<StgPAP>() - 8usize];
    ["Offset of field: StgPAP::header"][offset_of!(StgPAP, header) - 0usize];
    ["Offset of field: StgPAP::arity"][offset_of!(StgPAP, arity) - 8usize];
    ["Offset of field: StgPAP::n_args"][offset_of!(StgPAP, n_args) - 12usize];
    ["Offset of field: StgPAP::fun"][offset_of!(StgPAP, fun) - 16usize];
    ["Offset of field: StgPAP::payload"][offset_of!(StgPAP, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgAP() {
    assert_eq!(size_of::<sys::StgAP>(), size_of::<StgAP>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAP"][size_of::<StgAP>() - 32usize];
    ["Alignment of StgAP"][align_of::<StgAP>() - 8usize];
    ["Offset of field: StgAP::header"][offset_of!(StgAP, header) - 0usize];
    ["Offset of field: StgAP::arity"][offset_of!(StgAP, arity) - 16usize];
    ["Offset of field: StgAP::n_args"][offset_of!(StgAP, n_args) - 20usize];
    ["Offset of field: StgAP::fun"][offset_of!(StgAP, fun) - 24usize];
    ["Offset of field: StgAP::payload"][offset_of!(StgAP, payload) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgAP_STACK() {
    assert_eq!(size_of::<sys::StgAP_STACK>(), size_of::<StgAP_STACK>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAP_STACK"][size_of::<StgAP_STACK>() - 32usize];
    ["Alignment of StgAP_STACK"][align_of::<StgAP_STACK>() - 8usize];
    ["Offset of field: StgAP_STACK::header"][offset_of!(StgAP_STACK, header) - 0usize];
    ["Offset of field: StgAP_STACK::size"][offset_of!(StgAP_STACK, size) - 16usize];
    ["Offset of field: StgAP_STACK::fun"][offset_of!(StgAP_STACK, fun) - 24usize];
    ["Offset of field: StgAP_STACK::payload"][offset_of!(StgAP_STACK, payload) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgInd() {
    assert_eq!(size_of::<sys::StgInd>(), size_of::<StgInd>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgInd"][size_of::<StgInd>() - 16usize];
    ["Alignment of StgInd"][align_of::<StgInd>() - 8usize];
    ["Offset of field: StgInd::header"][offset_of!(StgInd, header) - 0usize];
    ["Offset of field: StgInd::indirectee"][offset_of!(StgInd, indirectee) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgIndStatic() {
    assert_eq!(size_of::<sys::StgIndStatic>(), size_of::<StgIndStatic>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgIndStatic"][size_of::<StgIndStatic>() - 32usize];
    ["Alignment of StgIndStatic"][align_of::<StgIndStatic>() - 8usize];
    ["Offset of field: StgIndStatic::header"][offset_of!(StgIndStatic, header) - 0usize];
    ["Offset of field: StgIndStatic::indirectee"][offset_of!(StgIndStatic, indirectee) - 8usize];
    ["Offset of field: StgIndStatic::static_link"][offset_of!(StgIndStatic, static_link) - 16usize];
    ["Offset of field: StgIndStatic::saved_info"][offset_of!(StgIndStatic, saved_info) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgBlockingQueue_() {
    assert_eq!(
        size_of::<sys::StgBlockingQueue_>(),
        size_of::<StgBlockingQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgBlockingQueue_"][size_of::<StgBlockingQueue_>() - 40usize];
    ["Alignment of StgBlockingQueue_"][align_of::<StgBlockingQueue_>() - 8usize];
    ["Offset of field: StgBlockingQueue_::header"][offset_of!(StgBlockingQueue_, header) - 0usize];
    ["Offset of field: StgBlockingQueue_::link"][offset_of!(StgBlockingQueue_, link) - 8usize];
    ["Offset of field: StgBlockingQueue_::bh"][offset_of!(StgBlockingQueue_, bh) - 16usize];
    ["Offset of field: StgBlockingQueue_::owner"][offset_of!(StgBlockingQueue_, owner) - 24usize];
    ["Offset of field: StgBlockingQueue_::queue"][offset_of!(StgBlockingQueue_, queue) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgArrBytes() {
    assert_eq!(size_of::<sys::StgArrBytes>(), size_of::<StgArrBytes>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgArrBytes"][size_of::<StgArrBytes>() - 16usize];
    ["Alignment of StgArrBytes"][align_of::<StgArrBytes>() - 8usize];
    ["Offset of field: StgArrBytes::header"][offset_of!(StgArrBytes, header) - 0usize];
    ["Offset of field: StgArrBytes::bytes"][offset_of!(StgArrBytes, bytes) - 8usize];
    ["Offset of field: StgArrBytes::payload"][offset_of!(StgArrBytes, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgMutArrPtrs() {
    assert_eq!(
        size_of::<sys::_StgMutArrPtrs>(),
        size_of::<_StgMutArrPtrs>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgMutArrPtrs"][size_of::<_StgMutArrPtrs>() - 24usize];
    ["Alignment of _StgMutArrPtrs"][align_of::<_StgMutArrPtrs>() - 8usize];
    ["Offset of field: _StgMutArrPtrs::header"][offset_of!(_StgMutArrPtrs, header) - 0usize];
    ["Offset of field: _StgMutArrPtrs::ptrs"][offset_of!(_StgMutArrPtrs, ptrs) - 8usize];
    ["Offset of field: _StgMutArrPtrs::size"][offset_of!(_StgMutArrPtrs, size) - 16usize];
    ["Offset of field: _StgMutArrPtrs::payload"][offset_of!(_StgMutArrPtrs, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgSmallMutArrPtrs() {
    assert_eq!(
        size_of::<sys::StgSmallMutArrPtrs>(),
        size_of::<StgSmallMutArrPtrs>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSmallMutArrPtrs"][size_of::<StgSmallMutArrPtrs>() - 16usize];
    ["Alignment of StgSmallMutArrPtrs"][align_of::<StgSmallMutArrPtrs>() - 8usize];
    ["Offset of field: StgSmallMutArrPtrs::header"]
        [offset_of!(StgSmallMutArrPtrs, header) - 0usize];
    ["Offset of field: StgSmallMutArrPtrs::ptrs"][offset_of!(StgSmallMutArrPtrs, ptrs) - 8usize];
    ["Offset of field: StgSmallMutArrPtrs::payload"]
        [offset_of!(StgSmallMutArrPtrs, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgMutVar() {
    assert_eq!(size_of::<sys::StgMutVar>(), size_of::<StgMutVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMutVar"][size_of::<StgMutVar>() - 16usize];
    ["Alignment of StgMutVar"][align_of::<StgMutVar>() - 8usize];
    ["Offset of field: StgMutVar::header"][offset_of!(StgMutVar, header) - 0usize];
    ["Offset of field: StgMutVar::var"][offset_of!(StgMutVar, var) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgUpdateFrame() {
    assert_eq!(
        size_of::<sys::_StgUpdateFrame>(),
        size_of::<_StgUpdateFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgUpdateFrame"][size_of::<_StgUpdateFrame>() - 16usize];
    ["Alignment of _StgUpdateFrame"][align_of::<_StgUpdateFrame>() - 8usize];
    ["Offset of field: _StgUpdateFrame::header"][offset_of!(_StgUpdateFrame, header) - 0usize];
    ["Offset of field: _StgUpdateFrame::updatee"][offset_of!(_StgUpdateFrame, updatee) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgOrigThunkInfoFrame() {
    assert_eq!(
        size_of::<sys::_StgOrigThunkInfoFrame>(),
        size_of::<_StgOrigThunkInfoFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgOrigThunkInfoFrame"][size_of::<_StgOrigThunkInfoFrame>() - 16usize];
    ["Alignment of _StgOrigThunkInfoFrame"][align_of::<_StgOrigThunkInfoFrame>() - 8usize];
    ["Offset of field: _StgOrigThunkInfoFrame::header"]
        [offset_of!(_StgOrigThunkInfoFrame, header) - 0usize];
    ["Offset of field: _StgOrigThunkInfoFrame::info_ptr"]
        [offset_of!(_StgOrigThunkInfoFrame, info_ptr) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgKeepAliveFrame() {
    assert_eq!(
        size_of::<sys::StgKeepAliveFrame>(),
        size_of::<StgKeepAliveFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgKeepAliveFrame"][size_of::<StgKeepAliveFrame>() - 16usize];
    ["Alignment of StgKeepAliveFrame"][align_of::<StgKeepAliveFrame>() - 8usize];
    ["Offset of field: StgKeepAliveFrame::header"][offset_of!(StgKeepAliveFrame, header) - 0usize];
    ["Offset of field: StgKeepAliveFrame::c"][offset_of!(StgKeepAliveFrame, c) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgCatchFrame() {
    assert_eq!(size_of::<sys::StgCatchFrame>(), size_of::<StgCatchFrame>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchFrame"][size_of::<StgCatchFrame>() - 16usize];
    ["Alignment of StgCatchFrame"][align_of::<StgCatchFrame>() - 8usize];
    ["Offset of field: StgCatchFrame::header"][offset_of!(StgCatchFrame, header) - 0usize];
    ["Offset of field: StgCatchFrame::handler"][offset_of!(StgCatchFrame, handler) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgUnderflowFrame() {
    assert_eq!(
        size_of::<sys::StgUnderflowFrame>(),
        size_of::<StgUnderflowFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgUnderflowFrame"][size_of::<StgUnderflowFrame>() - 16usize];
    ["Alignment of StgUnderflowFrame"][align_of::<StgUnderflowFrame>() - 8usize];
    ["Offset of field: StgUnderflowFrame::info"][offset_of!(StgUnderflowFrame, info) - 0usize];
    ["Offset of field: StgUnderflowFrame::next_chunk"]
        [offset_of!(StgUnderflowFrame, next_chunk) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgStopFrame() {
    assert_eq!(size_of::<sys::StgStopFrame>(), size_of::<StgStopFrame>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgStopFrame"][size_of::<StgStopFrame>() - 8usize];
    ["Alignment of StgStopFrame"][align_of::<StgStopFrame>() - 8usize];
    ["Offset of field: StgStopFrame::header"][offset_of!(StgStopFrame, header) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgDeadThreadFrame() {
    assert_eq!(
        size_of::<sys::StgDeadThreadFrame>(),
        size_of::<StgDeadThreadFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgDeadThreadFrame"][size_of::<StgDeadThreadFrame>() - 16usize];
    ["Alignment of StgDeadThreadFrame"][align_of::<StgDeadThreadFrame>() - 8usize];
    ["Offset of field: StgDeadThreadFrame::header"]
        [offset_of!(StgDeadThreadFrame, header) - 0usize];
    ["Offset of field: StgDeadThreadFrame::result"]
        [offset_of!(StgDeadThreadFrame, result) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgRetFun() {
    assert_eq!(size_of::<sys::StgRetFun>(), size_of::<StgRetFun>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRetFun"][size_of::<StgRetFun>() - 24usize];
    ["Alignment of StgRetFun"][align_of::<StgRetFun>() - 8usize];
    ["Offset of field: StgRetFun::info"][offset_of!(StgRetFun, info) - 0usize];
    ["Offset of field: StgRetFun::size"][offset_of!(StgRetFun, size) - 8usize];
    ["Offset of field: StgRetFun::fun"][offset_of!(StgRetFun, fun) - 16usize];
    ["Offset of field: StgRetFun::payload"][offset_of!(StgRetFun, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgIntCharlikeClosure() {
    assert_eq!(
        size_of::<sys::StgIntCharlikeClosure>(),
        size_of::<StgIntCharlikeClosure>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgIntCharlikeClosure"][size_of::<StgIntCharlikeClosure>() - 16usize];
    ["Alignment of StgIntCharlikeClosure"][align_of::<StgIntCharlikeClosure>() - 8usize];
    ["Offset of field: StgIntCharlikeClosure::header"]
        [offset_of!(StgIntCharlikeClosure, header) - 0usize];
    ["Offset of field: StgIntCharlikeClosure::data"]
        [offset_of!(StgIntCharlikeClosure, data) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgStableName() {
    assert_eq!(
        size_of::<sys::_StgStableName>(),
        size_of::<_StgStableName>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgStableName"][size_of::<_StgStableName>() - 16usize];
    ["Alignment of _StgStableName"][align_of::<_StgStableName>() - 8usize];
    ["Offset of field: _StgStableName::header"][offset_of!(_StgStableName, header) - 0usize];
    ["Offset of field: _StgStableName::sn"][offset_of!(_StgStableName, sn) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgWeak() {
    assert_eq!(size_of::<sys::_StgWeak>(), size_of::<_StgWeak>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgWeak"][size_of::<_StgWeak>() - 48usize];
    ["Alignment of _StgWeak"][align_of::<_StgWeak>() - 8usize];
    ["Offset of field: _StgWeak::header"][offset_of!(_StgWeak, header) - 0usize];
    ["Offset of field: _StgWeak::cfinalizers"][offset_of!(_StgWeak, cfinalizers) - 8usize];
    ["Offset of field: _StgWeak::key"][offset_of!(_StgWeak, key) - 16usize];
    ["Offset of field: _StgWeak::value"][offset_of!(_StgWeak, value) - 24usize];
    ["Offset of field: _StgWeak::finalizer"][offset_of!(_StgWeak, finalizer) - 32usize];
    ["Offset of field: _StgWeak::link"][offset_of!(_StgWeak, link) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size__StgCFinalizerList() {
    assert_eq!(
        size_of::<sys::_StgCFinalizerList>(),
        size_of::<_StgCFinalizerList>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgCFinalizerList"][size_of::<_StgCFinalizerList>() - 48usize];
    ["Alignment of _StgCFinalizerList"][align_of::<_StgCFinalizerList>() - 8usize];
    ["Offset of field: _StgCFinalizerList::header"]
        [offset_of!(_StgCFinalizerList, header) - 0usize];
    ["Offset of field: _StgCFinalizerList::link"][offset_of!(_StgCFinalizerList, link) - 8usize];
    ["Offset of field: _StgCFinalizerList::fptr"][offset_of!(_StgCFinalizerList, fptr) - 16usize];
    ["Offset of field: _StgCFinalizerList::ptr"][offset_of!(_StgCFinalizerList, ptr) - 24usize];
    ["Offset of field: _StgCFinalizerList::eptr"][offset_of!(_StgCFinalizerList, eptr) - 32usize];
    ["Offset of field: _StgCFinalizerList::flag"][offset_of!(_StgCFinalizerList, flag) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgBCO() {
    assert_eq!(size_of::<sys::StgBCO>(), size_of::<StgBCO>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgBCO"][size_of::<StgBCO>() - 40usize];
    ["Alignment of StgBCO"][align_of::<StgBCO>() - 8usize];
    ["Offset of field: StgBCO::header"][offset_of!(StgBCO, header) - 0usize];
    ["Offset of field: StgBCO::instrs"][offset_of!(StgBCO, instrs) - 8usize];
    ["Offset of field: StgBCO::literals"][offset_of!(StgBCO, literals) - 16usize];
    ["Offset of field: StgBCO::ptrs"][offset_of!(StgBCO, ptrs) - 24usize];
    ["Offset of field: StgBCO::arity"][offset_of!(StgBCO, arity) - 32usize];
    ["Offset of field: StgBCO::size"][offset_of!(StgBCO, size) - 36usize];
    ["Offset of field: StgBCO::bitmap"][offset_of!(StgBCO, bitmap) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgMVarTSOQueue_() {
    assert_eq!(
        size_of::<sys::StgMVarTSOQueue_>(),
        size_of::<StgMVarTSOQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMVarTSOQueue_"][size_of::<StgMVarTSOQueue_>() - 24usize];
    ["Alignment of StgMVarTSOQueue_"][align_of::<StgMVarTSOQueue_>() - 8usize];
    ["Offset of field: StgMVarTSOQueue_::header"][offset_of!(StgMVarTSOQueue_, header) - 0usize];
    ["Offset of field: StgMVarTSOQueue_::link"][offset_of!(StgMVarTSOQueue_, link) - 8usize];
    ["Offset of field: StgMVarTSOQueue_::tso"][offset_of!(StgMVarTSOQueue_, tso) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgMVar() {
    assert_eq!(size_of::<sys::StgMVar>(), size_of::<StgMVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMVar"][size_of::<StgMVar>() - 32usize];
    ["Alignment of StgMVar"][align_of::<StgMVar>() - 8usize];
    ["Offset of field: StgMVar::header"][offset_of!(StgMVar, header) - 0usize];
    ["Offset of field: StgMVar::head"][offset_of!(StgMVar, head) - 8usize];
    ["Offset of field: StgMVar::tail"][offset_of!(StgMVar, tail) - 16usize];
    ["Offset of field: StgMVar::value"][offset_of!(StgMVar, value) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTVarWatchQueue_() {
    assert_eq!(
        size_of::<sys::StgTVarWatchQueue_>(),
        size_of::<StgTVarWatchQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTVarWatchQueue_"][size_of::<StgTVarWatchQueue_>() - 32usize];
    ["Alignment of StgTVarWatchQueue_"][align_of::<StgTVarWatchQueue_>() - 8usize];
    ["Offset of field: StgTVarWatchQueue_::header"]
        [offset_of!(StgTVarWatchQueue_, header) - 0usize];
    ["Offset of field: StgTVarWatchQueue_::closure"]
        [offset_of!(StgTVarWatchQueue_, closure) - 8usize];
    ["Offset of field: StgTVarWatchQueue_::next_queue_entry"]
        [offset_of!(StgTVarWatchQueue_, next_queue_entry) - 16usize];
    ["Offset of field: StgTVarWatchQueue_::prev_queue_entry"]
        [offset_of!(StgTVarWatchQueue_, prev_queue_entry) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTVar() {
    assert_eq!(size_of::<sys::StgTVar>(), size_of::<StgTVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTVar"][size_of::<StgTVar>() - 32usize];
    ["Alignment of StgTVar"][align_of::<StgTVar>() - 8usize];
    ["Offset of field: StgTVar::header"][offset_of!(StgTVar, header) - 0usize];
    ["Offset of field: StgTVar::current_value"][offset_of!(StgTVar, current_value) - 8usize];
    ["Offset of field: StgTVar::first_watch_queue_entry"]
        [offset_of!(StgTVar, first_watch_queue_entry) - 16usize];
    ["Offset of field: StgTVar::num_updates"][offset_of!(StgTVar, num_updates) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_TRecEntry() {
    assert_eq!(size_of::<sys::TRecEntry>(), size_of::<TRecEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TRecEntry"][size_of::<TRecEntry>() - 24usize];
    ["Alignment of TRecEntry"][align_of::<TRecEntry>() - 8usize];
    ["Offset of field: TRecEntry::tvar"][offset_of!(TRecEntry, tvar) - 0usize];
    ["Offset of field: TRecEntry::expected_value"][offset_of!(TRecEntry, expected_value) - 8usize];
    ["Offset of field: TRecEntry::new_value"][offset_of!(TRecEntry, new_value) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTRecChunk_() {
    assert_eq!(size_of::<sys::StgTRecChunk_>(), size_of::<StgTRecChunk_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTRecChunk_"][size_of::<StgTRecChunk_>() - 408usize];
    ["Alignment of StgTRecChunk_"][align_of::<StgTRecChunk_>() - 8usize];
    ["Offset of field: StgTRecChunk_::header"][offset_of!(StgTRecChunk_, header) - 0usize];
    ["Offset of field: StgTRecChunk_::prev_chunk"][offset_of!(StgTRecChunk_, prev_chunk) - 8usize];
    ["Offset of field: StgTRecChunk_::next_entry_idx"]
        [offset_of!(StgTRecChunk_, next_entry_idx) - 16usize];
    ["Offset of field: StgTRecChunk_::entries"][offset_of!(StgTRecChunk_, entries) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgTRecHeader_() {
    assert_eq!(
        size_of::<sys::StgTRecHeader_>(),
        size_of::<StgTRecHeader_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTRecHeader_"][size_of::<StgTRecHeader_>() - 32usize];
    ["Alignment of StgTRecHeader_"][align_of::<StgTRecHeader_>() - 8usize];
    ["Offset of field: StgTRecHeader_::header"][offset_of!(StgTRecHeader_, header) - 0usize];
    ["Offset of field: StgTRecHeader_::enclosing_trec"]
        [offset_of!(StgTRecHeader_, enclosing_trec) - 8usize];
    ["Offset of field: StgTRecHeader_::current_chunk"]
        [offset_of!(StgTRecHeader_, current_chunk) - 16usize];
    ["Offset of field: StgTRecHeader_::state"][offset_of!(StgTRecHeader_, state) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgAtomicallyFrame() {
    assert_eq!(
        size_of::<sys::StgAtomicallyFrame>(),
        size_of::<StgAtomicallyFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAtomicallyFrame"][size_of::<StgAtomicallyFrame>() - 24usize];
    ["Alignment of StgAtomicallyFrame"][align_of::<StgAtomicallyFrame>() - 8usize];
    ["Offset of field: StgAtomicallyFrame::header"]
        [offset_of!(StgAtomicallyFrame, header) - 0usize];
    ["Offset of field: StgAtomicallyFrame::code"][offset_of!(StgAtomicallyFrame, code) - 8usize];
    ["Offset of field: StgAtomicallyFrame::result"]
        [offset_of!(StgAtomicallyFrame, result) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgCatchSTMFrame() {
    assert_eq!(
        size_of::<sys::StgCatchSTMFrame>(),
        size_of::<StgCatchSTMFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchSTMFrame"][size_of::<StgCatchSTMFrame>() - 24usize];
    ["Alignment of StgCatchSTMFrame"][align_of::<StgCatchSTMFrame>() - 8usize];
    ["Offset of field: StgCatchSTMFrame::header"][offset_of!(StgCatchSTMFrame, header) - 0usize];
    ["Offset of field: StgCatchSTMFrame::code"][offset_of!(StgCatchSTMFrame, code) - 8usize];
    ["Offset of field: StgCatchSTMFrame::handler"][offset_of!(StgCatchSTMFrame, handler) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgCatchRetryFrame() {
    assert_eq!(
        size_of::<sys::StgCatchRetryFrame>(),
        size_of::<StgCatchRetryFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchRetryFrame"][size_of::<StgCatchRetryFrame>() - 32usize];
    ["Alignment of StgCatchRetryFrame"][align_of::<StgCatchRetryFrame>() - 8usize];
    ["Offset of field: StgCatchRetryFrame::header"]
        [offset_of!(StgCatchRetryFrame, header) - 0usize];
    ["Offset of field: StgCatchRetryFrame::running_alt_code"]
        [offset_of!(StgCatchRetryFrame, running_alt_code) - 8usize];
    ["Offset of field: StgCatchRetryFrame::first_code"]
        [offset_of!(StgCatchRetryFrame, first_code) - 16usize];
    ["Offset of field: StgCatchRetryFrame::alt_code"]
        [offset_of!(StgCatchRetryFrame, alt_code) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_Message_() {
    assert_eq!(size_of::<sys::Message_>(), size_of::<Message_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Message_"][size_of::<Message_>() - 16usize];
    ["Alignment of Message_"][align_of::<Message_>() - 8usize];
    ["Offset of field: Message_::header"][offset_of!(Message_, header) - 0usize];
    ["Offset of field: Message_::link"][offset_of!(Message_, link) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_MessageWakeup_() {
    assert_eq!(
        size_of::<sys::MessageWakeup_>(),
        size_of::<MessageWakeup_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageWakeup_"][size_of::<MessageWakeup_>() - 24usize];
    ["Alignment of MessageWakeup_"][align_of::<MessageWakeup_>() - 8usize];
    ["Offset of field: MessageWakeup_::header"][offset_of!(MessageWakeup_, header) - 0usize];
    ["Offset of field: MessageWakeup_::link"][offset_of!(MessageWakeup_, link) - 8usize];
    ["Offset of field: MessageWakeup_::tso"][offset_of!(MessageWakeup_, tso) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_MessageThrowTo_() {
    assert_eq!(
        size_of::<sys::MessageThrowTo_>(),
        size_of::<MessageThrowTo_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageThrowTo_"][size_of::<MessageThrowTo_>() - 40usize];
    ["Alignment of MessageThrowTo_"][align_of::<MessageThrowTo_>() - 8usize];
    ["Offset of field: MessageThrowTo_::header"][offset_of!(MessageThrowTo_, header) - 0usize];
    ["Offset of field: MessageThrowTo_::link"][offset_of!(MessageThrowTo_, link) - 8usize];
    ["Offset of field: MessageThrowTo_::source"][offset_of!(MessageThrowTo_, source) - 16usize];
    ["Offset of field: MessageThrowTo_::target"][offset_of!(MessageThrowTo_, target) - 24usize];
    ["Offset of field: MessageThrowTo_::exception"]
        [offset_of!(MessageThrowTo_, exception) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_MessageBlackHole_() {
    assert_eq!(
        size_of::<sys::MessageBlackHole_>(),
        size_of::<MessageBlackHole_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageBlackHole_"][size_of::<MessageBlackHole_>() - 32usize];
    ["Alignment of MessageBlackHole_"][align_of::<MessageBlackHole_>() - 8usize];
    ["Offset of field: MessageBlackHole_::header"][offset_of!(MessageBlackHole_, header) - 0usize];
    ["Offset of field: MessageBlackHole_::link"][offset_of!(MessageBlackHole_, link) - 8usize];
    ["Offset of field: MessageBlackHole_::tso"][offset_of!(MessageBlackHole_, tso) - 16usize];
    ["Offset of field: MessageBlackHole_::bh"][offset_of!(MessageBlackHole_, bh) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_MessageCloneStack_() {
    assert_eq!(
        size_of::<sys::MessageCloneStack_>(),
        size_of::<MessageCloneStack_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageCloneStack_"][size_of::<MessageCloneStack_>() - 32usize];
    ["Alignment of MessageCloneStack_"][align_of::<MessageCloneStack_>() - 8usize];
    ["Offset of field: MessageCloneStack_::header"]
        [offset_of!(MessageCloneStack_, header) - 0usize];
    ["Offset of field: MessageCloneStack_::link"][offset_of!(MessageCloneStack_, link) - 8usize];
    ["Offset of field: MessageCloneStack_::result"]
        [offset_of!(MessageCloneStack_, result) - 16usize];
    ["Offset of field: MessageCloneStack_::tso"][offset_of!(MessageCloneStack_, tso) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgCompactNFDataBlock_() {
    assert_eq!(
        size_of::<sys::StgCompactNFDataBlock_>(),
        size_of::<StgCompactNFDataBlock_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCompactNFDataBlock_"][size_of::<StgCompactNFDataBlock_>() - 24usize];
    ["Alignment of StgCompactNFDataBlock_"][align_of::<StgCompactNFDataBlock_>() - 8usize];
    ["Offset of field: StgCompactNFDataBlock_::self_"]
        [offset_of!(StgCompactNFDataBlock_, self_) - 0usize];
    ["Offset of field: StgCompactNFDataBlock_::owner"]
        [offset_of!(StgCompactNFDataBlock_, owner) - 8usize];
    ["Offset of field: StgCompactNFDataBlock_::next"]
        [offset_of!(StgCompactNFDataBlock_, next) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgCompactNFData_() {
    assert_eq!(
        size_of::<sys::StgCompactNFData_>(),
        size_of::<StgCompactNFData_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCompactNFData_"][size_of::<StgCompactNFData_>() - 80usize];
    ["Alignment of StgCompactNFData_"][align_of::<StgCompactNFData_>() - 8usize];
    ["Offset of field: StgCompactNFData_::header"][offset_of!(StgCompactNFData_, header) - 0usize];
    ["Offset of field: StgCompactNFData_::totalW"][offset_of!(StgCompactNFData_, totalW) - 8usize];
    ["Offset of field: StgCompactNFData_::autoBlockW"]
        [offset_of!(StgCompactNFData_, autoBlockW) - 16usize];
    ["Offset of field: StgCompactNFData_::hp"][offset_of!(StgCompactNFData_, hp) - 24usize];
    ["Offset of field: StgCompactNFData_::hpLim"][offset_of!(StgCompactNFData_, hpLim) - 32usize];
    ["Offset of field: StgCompactNFData_::nursery"]
        [offset_of!(StgCompactNFData_, nursery) - 40usize];
    ["Offset of field: StgCompactNFData_::last"][offset_of!(StgCompactNFData_, last) - 48usize];
    ["Offset of field: StgCompactNFData_::hash"][offset_of!(StgCompactNFData_, hash) - 56usize];
    ["Offset of field: StgCompactNFData_::result"][offset_of!(StgCompactNFData_, result) - 64usize];
    ["Offset of field: StgCompactNFData_::link"][offset_of!(StgCompactNFData_, link) - 72usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgPromptFrame() {
    assert_eq!(
        size_of::<sys::StgPromptFrame>(),
        size_of::<StgPromptFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgPromptFrame"][size_of::<StgPromptFrame>() - 16usize];
    ["Alignment of StgPromptFrame"][align_of::<StgPromptFrame>() - 8usize];
    ["Offset of field: StgPromptFrame::header"][offset_of!(StgPromptFrame, header) - 0usize];
    ["Offset of field: StgPromptFrame::tag"][offset_of!(StgPromptFrame, tag) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_StgContinuation() {
    assert_eq!(
        size_of::<sys::StgContinuation>(),
        size_of::<StgContinuation>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgContinuation"][size_of::<StgContinuation>() - 32usize];
    ["Alignment of StgContinuation"][align_of::<StgContinuation>() - 8usize];
    ["Offset of field: StgContinuation::header"][offset_of!(StgContinuation, header) - 0usize];
    ["Offset of field: StgContinuation::apply_mask_frame"]
        [offset_of!(StgContinuation, apply_mask_frame) - 8usize];
    ["Offset of field: StgContinuation::mask_frame_offset"]
        [offset_of!(StgContinuation, mask_frame_offset) - 16usize];
    ["Offset of field: StgContinuation::stack_size"]
        [offset_of!(StgContinuation, stack_size) - 24usize];
    ["Offset of field: StgContinuation::stack"][offset_of!(StgContinuation, stack) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_hashtable() {
    assert_eq!(size_of::<sys::hashtable>(), size_of::<hashtable>())
}
