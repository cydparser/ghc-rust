use crate::capability::Capability;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::{CCS_SYSTEM, CostCentreStack, era, user_era};
use crate::ffi::rts::rts_to_hs_iface::ghc_hs_iface;
use crate::ffi::rts::stable_ptr::deRefStablePtr;
use crate::ffi::rts::storage::closure_macros::{
    doingErasProfiling, doingLDVProfiling, doingRetainerProfiling,
};
use crate::ffi::rts::storage::closures::{Message, MessageCloneStack, StgMVar, StgUnderflowFrame};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::{StgStack, StgStack_};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_MSG_CLONE_STACK_info, stg_stack_underflow_frame_d_info, stg_stack_underflow_frame_v16_info,
    stg_stack_underflow_frame_v32_info, stg_stack_underflow_frame_v64_info,
};
use crate::ffi::stg::types::{StgPtr, StgStablePtr, StgWord, StgWord8};
use crate::hs_ffi::HsStablePtr;
use crate::messages::sendMessage;
use crate::prelude::*;
use crate::rts_api::{HaskellObj, rts_apply, rts_unsafeGetMyCapability};
use crate::sm::sanity::checkClosure;
use crate::threads::performTryPutMVar;

unsafe fn cloneStackChunk(
    mut capability: *mut Capability,
    mut stack: *const StgStack,
) -> *mut StgStack {
    let mut spOffset: StgWord = (*stack)
        .sp
        .offset_from(&raw const (*stack).stack as *const StgWord)
        as i64 as StgWord;

    let mut closureSizeBytes: StgWord = (size_of::<StgStack>() as usize)
        .wrapping_add(((*stack).stack_size as usize).wrapping_mul(size_of::<StgWord>() as usize))
        as StgWord;

    let mut newStackClosure = allocate(
        capability,
        (closureSizeBytes as W_)
            .wrapping_add(size_of::<W_>() as W_)
            .wrapping_sub(1 as W_)
            .wrapping_div(size_of::<W_>() as W_),
    ) as *mut StgStack;

    memcpy(
        newStackClosure as *mut c_void,
        stack as *const c_void,
        closureSizeBytes as usize,
    );

    (*newStackClosure).sp =
        (&raw mut (*newStackClosure).stack as *mut StgWord).offset(spOffset as isize) as StgPtr;
    (*newStackClosure).dirty = 0;
    checkClosure(newStackClosure as *mut StgClosure);

    return newStackClosure;
}

unsafe fn cloneStack(mut capability: *mut Capability, mut stack: *const StgStack) -> *mut StgStack {
    let mut top_stack = cloneStackChunk(capability, stack);
    let mut last_stack = top_stack;

    loop {
        let mut frame = (&raw mut (*last_stack).stack as *mut StgWord)
            .offset((*last_stack).stack_size as isize)
            .offset(
                -((size_of::<StgUnderflowFrame>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize),
            ) as *mut StgUnderflowFrame;

        if !((*frame).info == &raw const stg_stack_underflow_frame_d_info
            || (*frame).info == &raw const stg_stack_underflow_frame_v16_info
            || (*frame).info == &raw const stg_stack_underflow_frame_v32_info
            || (*frame).info == &raw const stg_stack_underflow_frame_v64_info)
        {
            break;
        }

        let mut s = cloneStackChunk(capability, (*frame).next_chunk);
        (*frame).next_chunk = s as *mut StgStack_;
        last_stack = s;
    }

    return top_stack;
}

unsafe fn sendCloneStackMessage(mut tso: *mut StgTSO, mut mvar: HsStablePtr) {
    let mut srcCapability = rts_unsafeGetMyCapability();
    let mut msg = null_mut::<MessageCloneStack>();

    msg = allocate(
        srcCapability,
        (size_of::<MessageCloneStack>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as W_,
    ) as *mut MessageCloneStack;

    (*msg).tso = tso;
    (*msg).result = deRefStablePtr(mvar as StgStablePtr) as *mut StgMVar;

    let ref mut fresh13 = (*(msg as *mut StgClosure)).header.prof.ccs;
    *fresh13 = &raw mut CCS_SYSTEM as *mut CostCentreStack;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(msg as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(msg as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(msg as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*msg).header.info).store(&raw const stg_MSG_CLONE_STACK_info, Ordering::Release);
    sendMessage(
        srcCapability,
        (*tso).cap as *mut Capability,
        msg as *mut Message,
    );
}

unsafe fn handleCloneStackMessage(mut msg: *mut MessageCloneStack) {
    let mut newStackClosure =
        cloneStack((*(*msg).tso).cap as *mut Capability, (*(*msg).tso).stackobj);

    let mut result = rts_apply(
        (*(*msg).tso).cap as *mut Capability,
        (*ghc_hs_iface).StackSnapshot_closure as HaskellObj,
        newStackClosure as HaskellObj,
    );

    let mut putMVarWasSuccessful = performTryPutMVar(
        (*(*msg).tso).cap as *mut Capability,
        (*msg).result,
        result as *mut StgClosure,
    );

    if !putMVarWasSuccessful {
        barf(c"Can't put stack cloning result into MVar.".as_ptr());
    }
}
