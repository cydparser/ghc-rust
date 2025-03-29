use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_TREC_CHUNK_NUM_ENTRIES() {
    assert_eq!(sys::TREC_CHUNK_NUM_ENTRIES, super::TREC_CHUNK_NUM_ENTRIES);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgProfHeader() {
    assert_eq!(
        size_of::<sys::StgProfHeader>(),
        size_of::<super::StgProfHeader>()
    )
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgProfHeader__bindgen_ty_1() {
    assert_eq!(
        size_of::<sys::StgProfHeader__bindgen_ty_1>(),
        size_of::<super::StgProfHeader__bindgen_ty_1>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfHeader__bindgen_ty_1"]
        [::core::mem::size_of::<StgProfHeader__bindgen_ty_1>() - 8usize];
    ["Alignment of StgProfHeader__bindgen_ty_1"]
        [::core::mem::align_of::<StgProfHeader__bindgen_ty_1>() - 8usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::trav"]
        [::core::mem::offset_of!(StgProfHeader__bindgen_ty_1, trav) - 0usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::ldvw"]
        [::core::mem::offset_of!(StgProfHeader__bindgen_ty_1, ldvw) - 0usize];
    ["Offset of field: StgProfHeader__bindgen_ty_1::era"]
        [::core::mem::offset_of!(StgProfHeader__bindgen_ty_1, era) - 0usize];
};

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgProfHeader"][::core::mem::size_of::<StgProfHeader>() - 16usize];
    ["Alignment of StgProfHeader"][::core::mem::align_of::<StgProfHeader>() - 8usize];
    ["Offset of field: StgProfHeader::ccs"][::core::mem::offset_of!(StgProfHeader, ccs) - 0usize];
    ["Offset of field: StgProfHeader::hp"][::core::mem::offset_of!(StgProfHeader, hp) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgSMPThunkHeader() {
    assert_eq!(
        size_of::<sys::StgSMPThunkHeader>(),
        size_of::<super::StgSMPThunkHeader>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSMPThunkHeader"][::core::mem::size_of::<StgSMPThunkHeader>() - 8usize];
    ["Alignment of StgSMPThunkHeader"][::core::mem::align_of::<StgSMPThunkHeader>() - 8usize];
    ["Offset of field: StgSMPThunkHeader::pad"]
        [::core::mem::offset_of!(StgSMPThunkHeader, pad) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgHeader() {
    assert_eq!(size_of::<sys::StgHeader>(), size_of::<super::StgHeader>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgHeader"][::core::mem::size_of::<StgHeader>() - 8usize];
    ["Alignment of StgHeader"][::core::mem::align_of::<StgHeader>() - 8usize];
    ["Offset of field: StgHeader::info"][::core::mem::offset_of!(StgHeader, info) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgThunkHeader() {
    assert_eq!(
        size_of::<sys::StgThunkHeader>(),
        size_of::<super::StgThunkHeader>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunkHeader"][::core::mem::size_of::<StgThunkHeader>() - 16usize];
    ["Alignment of StgThunkHeader"][::core::mem::align_of::<StgThunkHeader>() - 8usize];
    ["Offset of field: StgThunkHeader::info"]
        [::core::mem::offset_of!(StgThunkHeader, info) - 0usize];
    ["Offset of field: StgThunkHeader::smp"][::core::mem::offset_of!(StgThunkHeader, smp) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgClosure_() {
    assert_eq!(
        size_of::<sys::StgClosure_>(),
        size_of::<super::StgClosure_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgClosure_"][::core::mem::size_of::<StgClosure_>() - 8usize];
    ["Alignment of StgClosure_"][::core::mem::align_of::<StgClosure_>() - 8usize];
    ["Offset of field: StgClosure_::header"][::core::mem::offset_of!(StgClosure_, header) - 0usize];
    ["Offset of field: StgClosure_::payload"]
        [::core::mem::offset_of!(StgClosure_, payload) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgThunk_() {
    assert_eq!(size_of::<sys::StgThunk_>(), size_of::<super::StgThunk_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgThunk_"][::core::mem::size_of::<StgThunk_>() - 16usize];
    ["Alignment of StgThunk_"][::core::mem::align_of::<StgThunk_>() - 8usize];
    ["Offset of field: StgThunk_::header"][::core::mem::offset_of!(StgThunk_, header) - 0usize];
    ["Offset of field: StgThunk_::payload"][::core::mem::offset_of!(StgThunk_, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgSelector() {
    assert_eq!(
        size_of::<sys::StgSelector>(),
        size_of::<super::StgSelector>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSelector"][::core::mem::size_of::<StgSelector>() - 24usize];
    ["Alignment of StgSelector"][::core::mem::align_of::<StgSelector>() - 8usize];
    ["Offset of field: StgSelector::header"][::core::mem::offset_of!(StgSelector, header) - 0usize];
    ["Offset of field: StgSelector::selectee"]
        [::core::mem::offset_of!(StgSelector, selectee) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgPAP() {
    assert_eq!(size_of::<sys::StgPAP>(), size_of::<super::StgPAP>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgPAP"][::core::mem::size_of::<StgPAP>() - 24usize];
    ["Alignment of StgPAP"][::core::mem::align_of::<StgPAP>() - 8usize];
    ["Offset of field: StgPAP::header"][::core::mem::offset_of!(StgPAP, header) - 0usize];
    ["Offset of field: StgPAP::arity"][::core::mem::offset_of!(StgPAP, arity) - 8usize];
    ["Offset of field: StgPAP::n_args"][::core::mem::offset_of!(StgPAP, n_args) - 12usize];
    ["Offset of field: StgPAP::fun"][::core::mem::offset_of!(StgPAP, fun) - 16usize];
    ["Offset of field: StgPAP::payload"][::core::mem::offset_of!(StgPAP, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgAP() {
    assert_eq!(size_of::<sys::StgAP>(), size_of::<super::StgAP>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAP"][::core::mem::size_of::<StgAP>() - 32usize];
    ["Alignment of StgAP"][::core::mem::align_of::<StgAP>() - 8usize];
    ["Offset of field: StgAP::header"][::core::mem::offset_of!(StgAP, header) - 0usize];
    ["Offset of field: StgAP::arity"][::core::mem::offset_of!(StgAP, arity) - 16usize];
    ["Offset of field: StgAP::n_args"][::core::mem::offset_of!(StgAP, n_args) - 20usize];
    ["Offset of field: StgAP::fun"][::core::mem::offset_of!(StgAP, fun) - 24usize];
    ["Offset of field: StgAP::payload"][::core::mem::offset_of!(StgAP, payload) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgAP_STACK() {
    assert_eq!(
        size_of::<sys::StgAP_STACK>(),
        size_of::<super::StgAP_STACK>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAP_STACK"][::core::mem::size_of::<StgAP_STACK>() - 32usize];
    ["Alignment of StgAP_STACK"][::core::mem::align_of::<StgAP_STACK>() - 8usize];
    ["Offset of field: StgAP_STACK::header"][::core::mem::offset_of!(StgAP_STACK, header) - 0usize];
    ["Offset of field: StgAP_STACK::size"][::core::mem::offset_of!(StgAP_STACK, size) - 16usize];
    ["Offset of field: StgAP_STACK::fun"][::core::mem::offset_of!(StgAP_STACK, fun) - 24usize];
    ["Offset of field: StgAP_STACK::payload"]
        [::core::mem::offset_of!(StgAP_STACK, payload) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgInd() {
    assert_eq!(size_of::<sys::StgInd>(), size_of::<super::StgInd>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgInd"][::core::mem::size_of::<StgInd>() - 16usize];
    ["Alignment of StgInd"][::core::mem::align_of::<StgInd>() - 8usize];
    ["Offset of field: StgInd::header"][::core::mem::offset_of!(StgInd, header) - 0usize];
    ["Offset of field: StgInd::indirectee"][::core::mem::offset_of!(StgInd, indirectee) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgIndStatic() {
    assert_eq!(
        size_of::<sys::StgIndStatic>(),
        size_of::<super::StgIndStatic>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgIndStatic"][::core::mem::size_of::<StgIndStatic>() - 32usize];
    ["Alignment of StgIndStatic"][::core::mem::align_of::<StgIndStatic>() - 8usize];
    ["Offset of field: StgIndStatic::header"]
        [::core::mem::offset_of!(StgIndStatic, header) - 0usize];
    ["Offset of field: StgIndStatic::indirectee"]
        [::core::mem::offset_of!(StgIndStatic, indirectee) - 8usize];
    ["Offset of field: StgIndStatic::static_link"]
        [::core::mem::offset_of!(StgIndStatic, static_link) - 16usize];
    ["Offset of field: StgIndStatic::saved_info"]
        [::core::mem::offset_of!(StgIndStatic, saved_info) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgBlockingQueue_() {
    assert_eq!(
        size_of::<sys::StgBlockingQueue_>(),
        size_of::<super::StgBlockingQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgBlockingQueue_"][::core::mem::size_of::<StgBlockingQueue_>() - 40usize];
    ["Alignment of StgBlockingQueue_"][::core::mem::align_of::<StgBlockingQueue_>() - 8usize];
    ["Offset of field: StgBlockingQueue_::header"]
        [::core::mem::offset_of!(StgBlockingQueue_, header) - 0usize];
    ["Offset of field: StgBlockingQueue_::link"]
        [::core::mem::offset_of!(StgBlockingQueue_, link) - 8usize];
    ["Offset of field: StgBlockingQueue_::bh"]
        [::core::mem::offset_of!(StgBlockingQueue_, bh) - 16usize];
    ["Offset of field: StgBlockingQueue_::owner"]
        [::core::mem::offset_of!(StgBlockingQueue_, owner) - 24usize];
    ["Offset of field: StgBlockingQueue_::queue"]
        [::core::mem::offset_of!(StgBlockingQueue_, queue) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgArrBytes() {
    assert_eq!(
        size_of::<sys::StgArrBytes>(),
        size_of::<super::StgArrBytes>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgArrBytes"][::core::mem::size_of::<StgArrBytes>() - 16usize];
    ["Alignment of StgArrBytes"][::core::mem::align_of::<StgArrBytes>() - 8usize];
    ["Offset of field: StgArrBytes::header"][::core::mem::offset_of!(StgArrBytes, header) - 0usize];
    ["Offset of field: StgArrBytes::bytes"][::core::mem::offset_of!(StgArrBytes, bytes) - 8usize];
    ["Offset of field: StgArrBytes::payload"]
        [::core::mem::offset_of!(StgArrBytes, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgMutArrPtrs() {
    assert_eq!(
        size_of::<sys::_StgMutArrPtrs>(),
        size_of::<super::_StgMutArrPtrs>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgMutArrPtrs"][::core::mem::size_of::<_StgMutArrPtrs>() - 24usize];
    ["Alignment of _StgMutArrPtrs"][::core::mem::align_of::<_StgMutArrPtrs>() - 8usize];
    ["Offset of field: _StgMutArrPtrs::header"]
        [::core::mem::offset_of!(_StgMutArrPtrs, header) - 0usize];
    ["Offset of field: _StgMutArrPtrs::ptrs"]
        [::core::mem::offset_of!(_StgMutArrPtrs, ptrs) - 8usize];
    ["Offset of field: _StgMutArrPtrs::size"]
        [::core::mem::offset_of!(_StgMutArrPtrs, size) - 16usize];
    ["Offset of field: _StgMutArrPtrs::payload"]
        [::core::mem::offset_of!(_StgMutArrPtrs, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgSmallMutArrPtrs() {
    assert_eq!(
        size_of::<sys::StgSmallMutArrPtrs>(),
        size_of::<super::StgSmallMutArrPtrs>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgSmallMutArrPtrs"][::core::mem::size_of::<StgSmallMutArrPtrs>() - 16usize];
    ["Alignment of StgSmallMutArrPtrs"][::core::mem::align_of::<StgSmallMutArrPtrs>() - 8usize];
    ["Offset of field: StgSmallMutArrPtrs::header"]
        [::core::mem::offset_of!(StgSmallMutArrPtrs, header) - 0usize];
    ["Offset of field: StgSmallMutArrPtrs::ptrs"]
        [::core::mem::offset_of!(StgSmallMutArrPtrs, ptrs) - 8usize];
    ["Offset of field: StgSmallMutArrPtrs::payload"]
        [::core::mem::offset_of!(StgSmallMutArrPtrs, payload) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgMutVar() {
    assert_eq!(size_of::<sys::StgMutVar>(), size_of::<super::StgMutVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMutVar"][::core::mem::size_of::<StgMutVar>() - 16usize];
    ["Alignment of StgMutVar"][::core::mem::align_of::<StgMutVar>() - 8usize];
    ["Offset of field: StgMutVar::header"][::core::mem::offset_of!(StgMutVar, header) - 0usize];
    ["Offset of field: StgMutVar::var"][::core::mem::offset_of!(StgMutVar, var) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgUpdateFrame() {
    assert_eq!(
        size_of::<sys::_StgUpdateFrame>(),
        size_of::<super::_StgUpdateFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgUpdateFrame"][::core::mem::size_of::<_StgUpdateFrame>() - 16usize];
    ["Alignment of _StgUpdateFrame"][::core::mem::align_of::<_StgUpdateFrame>() - 8usize];
    ["Offset of field: _StgUpdateFrame::header"]
        [::core::mem::offset_of!(_StgUpdateFrame, header) - 0usize];
    ["Offset of field: _StgUpdateFrame::updatee"]
        [::core::mem::offset_of!(_StgUpdateFrame, updatee) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgOrigThunkInfoFrame() {
    assert_eq!(
        size_of::<sys::_StgOrigThunkInfoFrame>(),
        size_of::<super::_StgOrigThunkInfoFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgOrigThunkInfoFrame"][::core::mem::size_of::<_StgOrigThunkInfoFrame>() - 16usize];
    ["Alignment of _StgOrigThunkInfoFrame"]
        [::core::mem::align_of::<_StgOrigThunkInfoFrame>() - 8usize];
    ["Offset of field: _StgOrigThunkInfoFrame::header"]
        [::core::mem::offset_of!(_StgOrigThunkInfoFrame, header) - 0usize];
    ["Offset of field: _StgOrigThunkInfoFrame::info_ptr"]
        [::core::mem::offset_of!(_StgOrigThunkInfoFrame, info_ptr) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgKeepAliveFrame() {
    assert_eq!(
        size_of::<sys::StgKeepAliveFrame>(),
        size_of::<super::StgKeepAliveFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgKeepAliveFrame"][::core::mem::size_of::<StgKeepAliveFrame>() - 16usize];
    ["Alignment of StgKeepAliveFrame"][::core::mem::align_of::<StgKeepAliveFrame>() - 8usize];
    ["Offset of field: StgKeepAliveFrame::header"]
        [::core::mem::offset_of!(StgKeepAliveFrame, header) - 0usize];
    ["Offset of field: StgKeepAliveFrame::c"]
        [::core::mem::offset_of!(StgKeepAliveFrame, c) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgCatchFrame() {
    assert_eq!(
        size_of::<sys::StgCatchFrame>(),
        size_of::<super::StgCatchFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchFrame"][::core::mem::size_of::<StgCatchFrame>() - 16usize];
    ["Alignment of StgCatchFrame"][::core::mem::align_of::<StgCatchFrame>() - 8usize];
    ["Offset of field: StgCatchFrame::header"]
        [::core::mem::offset_of!(StgCatchFrame, header) - 0usize];
    ["Offset of field: StgCatchFrame::handler"]
        [::core::mem::offset_of!(StgCatchFrame, handler) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgUnderflowFrame() {
    assert_eq!(
        size_of::<sys::StgUnderflowFrame>(),
        size_of::<super::StgUnderflowFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgUnderflowFrame"][::core::mem::size_of::<StgUnderflowFrame>() - 16usize];
    ["Alignment of StgUnderflowFrame"][::core::mem::align_of::<StgUnderflowFrame>() - 8usize];
    ["Offset of field: StgUnderflowFrame::info"]
        [::core::mem::offset_of!(StgUnderflowFrame, info) - 0usize];
    ["Offset of field: StgUnderflowFrame::next_chunk"]
        [::core::mem::offset_of!(StgUnderflowFrame, next_chunk) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgStopFrame() {
    assert_eq!(
        size_of::<sys::StgStopFrame>(),
        size_of::<super::StgStopFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgStopFrame"][::core::mem::size_of::<StgStopFrame>() - 8usize];
    ["Alignment of StgStopFrame"][::core::mem::align_of::<StgStopFrame>() - 8usize];
    ["Offset of field: StgStopFrame::header"]
        [::core::mem::offset_of!(StgStopFrame, header) - 0usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgDeadThreadFrame() {
    assert_eq!(
        size_of::<sys::StgDeadThreadFrame>(),
        size_of::<super::StgDeadThreadFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgDeadThreadFrame"][::core::mem::size_of::<StgDeadThreadFrame>() - 16usize];
    ["Alignment of StgDeadThreadFrame"][::core::mem::align_of::<StgDeadThreadFrame>() - 8usize];
    ["Offset of field: StgDeadThreadFrame::header"]
        [::core::mem::offset_of!(StgDeadThreadFrame, header) - 0usize];
    ["Offset of field: StgDeadThreadFrame::result"]
        [::core::mem::offset_of!(StgDeadThreadFrame, result) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgRetFun() {
    assert_eq!(size_of::<sys::StgRetFun>(), size_of::<super::StgRetFun>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgRetFun"][::core::mem::size_of::<StgRetFun>() - 24usize];
    ["Alignment of StgRetFun"][::core::mem::align_of::<StgRetFun>() - 8usize];
    ["Offset of field: StgRetFun::info"][::core::mem::offset_of!(StgRetFun, info) - 0usize];
    ["Offset of field: StgRetFun::size"][::core::mem::offset_of!(StgRetFun, size) - 8usize];
    ["Offset of field: StgRetFun::fun"][::core::mem::offset_of!(StgRetFun, fun) - 16usize];
    ["Offset of field: StgRetFun::payload"][::core::mem::offset_of!(StgRetFun, payload) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgIntCharlikeClosure() {
    assert_eq!(
        size_of::<sys::StgIntCharlikeClosure>(),
        size_of::<super::StgIntCharlikeClosure>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgIntCharlikeClosure"][::core::mem::size_of::<StgIntCharlikeClosure>() - 16usize];
    ["Alignment of StgIntCharlikeClosure"]
        [::core::mem::align_of::<StgIntCharlikeClosure>() - 8usize];
    ["Offset of field: StgIntCharlikeClosure::header"]
        [::core::mem::offset_of!(StgIntCharlikeClosure, header) - 0usize];
    ["Offset of field: StgIntCharlikeClosure::data"]
        [::core::mem::offset_of!(StgIntCharlikeClosure, data) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgStableName() {
    assert_eq!(
        size_of::<sys::_StgStableName>(),
        size_of::<super::_StgStableName>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgStableName"][::core::mem::size_of::<_StgStableName>() - 16usize];
    ["Alignment of _StgStableName"][::core::mem::align_of::<_StgStableName>() - 8usize];
    ["Offset of field: _StgStableName::header"]
        [::core::mem::offset_of!(_StgStableName, header) - 0usize];
    ["Offset of field: _StgStableName::sn"][::core::mem::offset_of!(_StgStableName, sn) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgWeak() {
    assert_eq!(size_of::<sys::_StgWeak>(), size_of::<super::_StgWeak>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgWeak"][::core::mem::size_of::<_StgWeak>() - 48usize];
    ["Alignment of _StgWeak"][::core::mem::align_of::<_StgWeak>() - 8usize];
    ["Offset of field: _StgWeak::header"][::core::mem::offset_of!(_StgWeak, header) - 0usize];
    ["Offset of field: _StgWeak::cfinalizers"]
        [::core::mem::offset_of!(_StgWeak, cfinalizers) - 8usize];
    ["Offset of field: _StgWeak::key"][::core::mem::offset_of!(_StgWeak, key) - 16usize];
    ["Offset of field: _StgWeak::value"][::core::mem::offset_of!(_StgWeak, value) - 24usize];
    ["Offset of field: _StgWeak::finalizer"]
        [::core::mem::offset_of!(_StgWeak, finalizer) - 32usize];
    ["Offset of field: _StgWeak::link"][::core::mem::offset_of!(_StgWeak, link) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of__StgCFinalizerList() {
    assert_eq!(
        size_of::<sys::_StgCFinalizerList>(),
        size_of::<super::_StgCFinalizerList>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _StgCFinalizerList"][::core::mem::size_of::<_StgCFinalizerList>() - 48usize];
    ["Alignment of _StgCFinalizerList"][::core::mem::align_of::<_StgCFinalizerList>() - 8usize];
    ["Offset of field: _StgCFinalizerList::header"]
        [::core::mem::offset_of!(_StgCFinalizerList, header) - 0usize];
    ["Offset of field: _StgCFinalizerList::link"]
        [::core::mem::offset_of!(_StgCFinalizerList, link) - 8usize];
    ["Offset of field: _StgCFinalizerList::fptr"]
        [::core::mem::offset_of!(_StgCFinalizerList, fptr) - 16usize];
    ["Offset of field: _StgCFinalizerList::ptr"]
        [::core::mem::offset_of!(_StgCFinalizerList, ptr) - 24usize];
    ["Offset of field: _StgCFinalizerList::eptr"]
        [::core::mem::offset_of!(_StgCFinalizerList, eptr) - 32usize];
    ["Offset of field: _StgCFinalizerList::flag"]
        [::core::mem::offset_of!(_StgCFinalizerList, flag) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgBCO() {
    assert_eq!(size_of::<sys::StgBCO>(), size_of::<super::StgBCO>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgBCO"][::core::mem::size_of::<StgBCO>() - 40usize];
    ["Alignment of StgBCO"][::core::mem::align_of::<StgBCO>() - 8usize];
    ["Offset of field: StgBCO::header"][::core::mem::offset_of!(StgBCO, header) - 0usize];
    ["Offset of field: StgBCO::instrs"][::core::mem::offset_of!(StgBCO, instrs) - 8usize];
    ["Offset of field: StgBCO::literals"][::core::mem::offset_of!(StgBCO, literals) - 16usize];
    ["Offset of field: StgBCO::ptrs"][::core::mem::offset_of!(StgBCO, ptrs) - 24usize];
    ["Offset of field: StgBCO::arity"][::core::mem::offset_of!(StgBCO, arity) - 32usize];
    ["Offset of field: StgBCO::size"][::core::mem::offset_of!(StgBCO, size) - 36usize];
    ["Offset of field: StgBCO::bitmap"][::core::mem::offset_of!(StgBCO, bitmap) - 40usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgMVarTSOQueue_() {
    assert_eq!(
        size_of::<sys::StgMVarTSOQueue_>(),
        size_of::<super::StgMVarTSOQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMVarTSOQueue_"][::core::mem::size_of::<StgMVarTSOQueue_>() - 24usize];
    ["Alignment of StgMVarTSOQueue_"][::core::mem::align_of::<StgMVarTSOQueue_>() - 8usize];
    ["Offset of field: StgMVarTSOQueue_::header"]
        [::core::mem::offset_of!(StgMVarTSOQueue_, header) - 0usize];
    ["Offset of field: StgMVarTSOQueue_::link"]
        [::core::mem::offset_of!(StgMVarTSOQueue_, link) - 8usize];
    ["Offset of field: StgMVarTSOQueue_::tso"]
        [::core::mem::offset_of!(StgMVarTSOQueue_, tso) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgMVar() {
    assert_eq!(size_of::<sys::StgMVar>(), size_of::<super::StgMVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgMVar"][::core::mem::size_of::<StgMVar>() - 32usize];
    ["Alignment of StgMVar"][::core::mem::align_of::<StgMVar>() - 8usize];
    ["Offset of field: StgMVar::header"][::core::mem::offset_of!(StgMVar, header) - 0usize];
    ["Offset of field: StgMVar::head"][::core::mem::offset_of!(StgMVar, head) - 8usize];
    ["Offset of field: StgMVar::tail"][::core::mem::offset_of!(StgMVar, tail) - 16usize];
    ["Offset of field: StgMVar::value"][::core::mem::offset_of!(StgMVar, value) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTVarWatchQueue_() {
    assert_eq!(
        size_of::<sys::StgTVarWatchQueue_>(),
        size_of::<super::StgTVarWatchQueue_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTVarWatchQueue_"][::core::mem::size_of::<StgTVarWatchQueue_>() - 32usize];
    ["Alignment of StgTVarWatchQueue_"][::core::mem::align_of::<StgTVarWatchQueue_>() - 8usize];
    ["Offset of field: StgTVarWatchQueue_::header"]
        [::core::mem::offset_of!(StgTVarWatchQueue_, header) - 0usize];
    ["Offset of field: StgTVarWatchQueue_::closure"]
        [::core::mem::offset_of!(StgTVarWatchQueue_, closure) - 8usize];
    ["Offset of field: StgTVarWatchQueue_::next_queue_entry"]
        [::core::mem::offset_of!(StgTVarWatchQueue_, next_queue_entry) - 16usize];
    ["Offset of field: StgTVarWatchQueue_::prev_queue_entry"]
        [::core::mem::offset_of!(StgTVarWatchQueue_, prev_queue_entry) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTVar() {
    assert_eq!(size_of::<sys::StgTVar>(), size_of::<super::StgTVar>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTVar"][::core::mem::size_of::<StgTVar>() - 32usize];
    ["Alignment of StgTVar"][::core::mem::align_of::<StgTVar>() - 8usize];
    ["Offset of field: StgTVar::header"][::core::mem::offset_of!(StgTVar, header) - 0usize];
    ["Offset of field: StgTVar::current_value"]
        [::core::mem::offset_of!(StgTVar, current_value) - 8usize];
    ["Offset of field: StgTVar::first_watch_queue_entry"]
        [::core::mem::offset_of!(StgTVar, first_watch_queue_entry) - 16usize];
    ["Offset of field: StgTVar::num_updates"]
        [::core::mem::offset_of!(StgTVar, num_updates) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_TRecEntry() {
    assert_eq!(size_of::<sys::TRecEntry>(), size_of::<super::TRecEntry>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TRecEntry"][::core::mem::size_of::<TRecEntry>() - 24usize];
    ["Alignment of TRecEntry"][::core::mem::align_of::<TRecEntry>() - 8usize];
    ["Offset of field: TRecEntry::tvar"][::core::mem::offset_of!(TRecEntry, tvar) - 0usize];
    ["Offset of field: TRecEntry::expected_value"]
        [::core::mem::offset_of!(TRecEntry, expected_value) - 8usize];
    ["Offset of field: TRecEntry::new_value"]
        [::core::mem::offset_of!(TRecEntry, new_value) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTRecChunk_() {
    assert_eq!(
        size_of::<sys::StgTRecChunk_>(),
        size_of::<super::StgTRecChunk_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTRecChunk_"][::core::mem::size_of::<StgTRecChunk_>() - 408usize];
    ["Alignment of StgTRecChunk_"][::core::mem::align_of::<StgTRecChunk_>() - 8usize];
    ["Offset of field: StgTRecChunk_::header"]
        [::core::mem::offset_of!(StgTRecChunk_, header) - 0usize];
    ["Offset of field: StgTRecChunk_::prev_chunk"]
        [::core::mem::offset_of!(StgTRecChunk_, prev_chunk) - 8usize];
    ["Offset of field: StgTRecChunk_::next_entry_idx"]
        [::core::mem::offset_of!(StgTRecChunk_, next_entry_idx) - 16usize];
    ["Offset of field: StgTRecChunk_::entries"]
        [::core::mem::offset_of!(StgTRecChunk_, entries) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgTRecHeader_() {
    assert_eq!(
        size_of::<sys::StgTRecHeader_>(),
        size_of::<super::StgTRecHeader_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgTRecHeader_"][::core::mem::size_of::<StgTRecHeader_>() - 32usize];
    ["Alignment of StgTRecHeader_"][::core::mem::align_of::<StgTRecHeader_>() - 8usize];
    ["Offset of field: StgTRecHeader_::header"]
        [::core::mem::offset_of!(StgTRecHeader_, header) - 0usize];
    ["Offset of field: StgTRecHeader_::enclosing_trec"]
        [::core::mem::offset_of!(StgTRecHeader_, enclosing_trec) - 8usize];
    ["Offset of field: StgTRecHeader_::current_chunk"]
        [::core::mem::offset_of!(StgTRecHeader_, current_chunk) - 16usize];
    ["Offset of field: StgTRecHeader_::state"]
        [::core::mem::offset_of!(StgTRecHeader_, state) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgAtomicallyFrame() {
    assert_eq!(
        size_of::<sys::StgAtomicallyFrame>(),
        size_of::<super::StgAtomicallyFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgAtomicallyFrame"][::core::mem::size_of::<StgAtomicallyFrame>() - 24usize];
    ["Alignment of StgAtomicallyFrame"][::core::mem::align_of::<StgAtomicallyFrame>() - 8usize];
    ["Offset of field: StgAtomicallyFrame::header"]
        [::core::mem::offset_of!(StgAtomicallyFrame, header) - 0usize];
    ["Offset of field: StgAtomicallyFrame::code"]
        [::core::mem::offset_of!(StgAtomicallyFrame, code) - 8usize];
    ["Offset of field: StgAtomicallyFrame::result"]
        [::core::mem::offset_of!(StgAtomicallyFrame, result) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgCatchSTMFrame() {
    assert_eq!(
        size_of::<sys::StgCatchSTMFrame>(),
        size_of::<super::StgCatchSTMFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchSTMFrame"][::core::mem::size_of::<StgCatchSTMFrame>() - 24usize];
    ["Alignment of StgCatchSTMFrame"][::core::mem::align_of::<StgCatchSTMFrame>() - 8usize];
    ["Offset of field: StgCatchSTMFrame::header"]
        [::core::mem::offset_of!(StgCatchSTMFrame, header) - 0usize];
    ["Offset of field: StgCatchSTMFrame::code"]
        [::core::mem::offset_of!(StgCatchSTMFrame, code) - 8usize];
    ["Offset of field: StgCatchSTMFrame::handler"]
        [::core::mem::offset_of!(StgCatchSTMFrame, handler) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgCatchRetryFrame() {
    assert_eq!(
        size_of::<sys::StgCatchRetryFrame>(),
        size_of::<super::StgCatchRetryFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCatchRetryFrame"][::core::mem::size_of::<StgCatchRetryFrame>() - 32usize];
    ["Alignment of StgCatchRetryFrame"][::core::mem::align_of::<StgCatchRetryFrame>() - 8usize];
    ["Offset of field: StgCatchRetryFrame::header"]
        [::core::mem::offset_of!(StgCatchRetryFrame, header) - 0usize];
    ["Offset of field: StgCatchRetryFrame::running_alt_code"]
        [::core::mem::offset_of!(StgCatchRetryFrame, running_alt_code) - 8usize];
    ["Offset of field: StgCatchRetryFrame::first_code"]
        [::core::mem::offset_of!(StgCatchRetryFrame, first_code) - 16usize];
    ["Offset of field: StgCatchRetryFrame::alt_code"]
        [::core::mem::offset_of!(StgCatchRetryFrame, alt_code) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_Message_() {
    assert_eq!(size_of::<sys::Message_>(), size_of::<super::Message_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Message_"][::core::mem::size_of::<Message_>() - 16usize];
    ["Alignment of Message_"][::core::mem::align_of::<Message_>() - 8usize];
    ["Offset of field: Message_::header"][::core::mem::offset_of!(Message_, header) - 0usize];
    ["Offset of field: Message_::link"][::core::mem::offset_of!(Message_, link) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_MessageWakeup_() {
    assert_eq!(
        size_of::<sys::MessageWakeup_>(),
        size_of::<super::MessageWakeup_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageWakeup_"][::core::mem::size_of::<MessageWakeup_>() - 24usize];
    ["Alignment of MessageWakeup_"][::core::mem::align_of::<MessageWakeup_>() - 8usize];
    ["Offset of field: MessageWakeup_::header"]
        [::core::mem::offset_of!(MessageWakeup_, header) - 0usize];
    ["Offset of field: MessageWakeup_::link"]
        [::core::mem::offset_of!(MessageWakeup_, link) - 8usize];
    ["Offset of field: MessageWakeup_::tso"]
        [::core::mem::offset_of!(MessageWakeup_, tso) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_MessageThrowTo_() {
    assert_eq!(
        size_of::<sys::MessageThrowTo_>(),
        size_of::<super::MessageThrowTo_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageThrowTo_"][::core::mem::size_of::<MessageThrowTo_>() - 40usize];
    ["Alignment of MessageThrowTo_"][::core::mem::align_of::<MessageThrowTo_>() - 8usize];
    ["Offset of field: MessageThrowTo_::header"]
        [::core::mem::offset_of!(MessageThrowTo_, header) - 0usize];
    ["Offset of field: MessageThrowTo_::link"]
        [::core::mem::offset_of!(MessageThrowTo_, link) - 8usize];
    ["Offset of field: MessageThrowTo_::source"]
        [::core::mem::offset_of!(MessageThrowTo_, source) - 16usize];
    ["Offset of field: MessageThrowTo_::target"]
        [::core::mem::offset_of!(MessageThrowTo_, target) - 24usize];
    ["Offset of field: MessageThrowTo_::exception"]
        [::core::mem::offset_of!(MessageThrowTo_, exception) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_MessageBlackHole_() {
    assert_eq!(
        size_of::<sys::MessageBlackHole_>(),
        size_of::<super::MessageBlackHole_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageBlackHole_"][::core::mem::size_of::<MessageBlackHole_>() - 32usize];
    ["Alignment of MessageBlackHole_"][::core::mem::align_of::<MessageBlackHole_>() - 8usize];
    ["Offset of field: MessageBlackHole_::header"]
        [::core::mem::offset_of!(MessageBlackHole_, header) - 0usize];
    ["Offset of field: MessageBlackHole_::link"]
        [::core::mem::offset_of!(MessageBlackHole_, link) - 8usize];
    ["Offset of field: MessageBlackHole_::tso"]
        [::core::mem::offset_of!(MessageBlackHole_, tso) - 16usize];
    ["Offset of field: MessageBlackHole_::bh"]
        [::core::mem::offset_of!(MessageBlackHole_, bh) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_MessageCloneStack_() {
    assert_eq!(
        size_of::<sys::MessageCloneStack_>(),
        size_of::<super::MessageCloneStack_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MessageCloneStack_"][::core::mem::size_of::<MessageCloneStack_>() - 32usize];
    ["Alignment of MessageCloneStack_"][::core::mem::align_of::<MessageCloneStack_>() - 8usize];
    ["Offset of field: MessageCloneStack_::header"]
        [::core::mem::offset_of!(MessageCloneStack_, header) - 0usize];
    ["Offset of field: MessageCloneStack_::link"]
        [::core::mem::offset_of!(MessageCloneStack_, link) - 8usize];
    ["Offset of field: MessageCloneStack_::result"]
        [::core::mem::offset_of!(MessageCloneStack_, result) - 16usize];
    ["Offset of field: MessageCloneStack_::tso"]
        [::core::mem::offset_of!(MessageCloneStack_, tso) - 24usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgCompactNFDataBlock_() {
    assert_eq!(
        size_of::<sys::StgCompactNFDataBlock_>(),
        size_of::<super::StgCompactNFDataBlock_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCompactNFDataBlock_"][::core::mem::size_of::<StgCompactNFDataBlock_>() - 24usize];
    ["Alignment of StgCompactNFDataBlock_"]
        [::core::mem::align_of::<StgCompactNFDataBlock_>() - 8usize];
    ["Offset of field: StgCompactNFDataBlock_::self_"]
        [::core::mem::offset_of!(StgCompactNFDataBlock_, self_) - 0usize];
    ["Offset of field: StgCompactNFDataBlock_::owner"]
        [::core::mem::offset_of!(StgCompactNFDataBlock_, owner) - 8usize];
    ["Offset of field: StgCompactNFDataBlock_::next"]
        [::core::mem::offset_of!(StgCompactNFDataBlock_, next) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgCompactNFData_() {
    assert_eq!(
        size_of::<sys::StgCompactNFData_>(),
        size_of::<super::StgCompactNFData_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgCompactNFData_"][::core::mem::size_of::<StgCompactNFData_>() - 80usize];
    ["Alignment of StgCompactNFData_"][::core::mem::align_of::<StgCompactNFData_>() - 8usize];
    ["Offset of field: StgCompactNFData_::header"]
        [::core::mem::offset_of!(StgCompactNFData_, header) - 0usize];
    ["Offset of field: StgCompactNFData_::totalW"]
        [::core::mem::offset_of!(StgCompactNFData_, totalW) - 8usize];
    ["Offset of field: StgCompactNFData_::autoBlockW"]
        [::core::mem::offset_of!(StgCompactNFData_, autoBlockW) - 16usize];
    ["Offset of field: StgCompactNFData_::hp"]
        [::core::mem::offset_of!(StgCompactNFData_, hp) - 24usize];
    ["Offset of field: StgCompactNFData_::hpLim"]
        [::core::mem::offset_of!(StgCompactNFData_, hpLim) - 32usize];
    ["Offset of field: StgCompactNFData_::nursery"]
        [::core::mem::offset_of!(StgCompactNFData_, nursery) - 40usize];
    ["Offset of field: StgCompactNFData_::last"]
        [::core::mem::offset_of!(StgCompactNFData_, last) - 48usize];
    ["Offset of field: StgCompactNFData_::hash"]
        [::core::mem::offset_of!(StgCompactNFData_, hash) - 56usize];
    ["Offset of field: StgCompactNFData_::result"]
        [::core::mem::offset_of!(StgCompactNFData_, result) - 64usize];
    ["Offset of field: StgCompactNFData_::link"]
        [::core::mem::offset_of!(StgCompactNFData_, link) - 72usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgPromptFrame() {
    assert_eq!(
        size_of::<sys::StgPromptFrame>(),
        size_of::<super::StgPromptFrame>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgPromptFrame"][::core::mem::size_of::<StgPromptFrame>() - 16usize];
    ["Alignment of StgPromptFrame"][::core::mem::align_of::<StgPromptFrame>() - 8usize];
    ["Offset of field: StgPromptFrame::header"]
        [::core::mem::offset_of!(StgPromptFrame, header) - 0usize];
    ["Offset of field: StgPromptFrame::tag"][::core::mem::offset_of!(StgPromptFrame, tag) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_StgContinuation() {
    assert_eq!(
        size_of::<sys::StgContinuation>(),
        size_of::<super::StgContinuation>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of StgContinuation"][::core::mem::size_of::<StgContinuation>() - 32usize];
    ["Alignment of StgContinuation"][::core::mem::align_of::<StgContinuation>() - 8usize];
    ["Offset of field: StgContinuation::header"]
        [::core::mem::offset_of!(StgContinuation, header) - 0usize];
    ["Offset of field: StgContinuation::apply_mask_frame"]
        [::core::mem::offset_of!(StgContinuation, apply_mask_frame) - 8usize];
    ["Offset of field: StgContinuation::mask_frame_offset"]
        [::core::mem::offset_of!(StgContinuation, mask_frame_offset) - 16usize];
    ["Offset of field: StgContinuation::stack_size"]
        [::core::mem::offset_of!(StgContinuation, stack_size) - 24usize];
    ["Offset of field: StgContinuation::stack"]
        [::core::mem::offset_of!(StgContinuation, stack) - 32usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_hashtable() {
    assert_eq!(size_of::<sys::hashtable>(), size_of::<super::hashtable>())
}
