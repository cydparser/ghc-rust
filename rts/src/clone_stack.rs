use crate::ffi::hs_ffi::HsStablePtr;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::closures::StgUnderflowFrame;
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::{StgStack, StgStack_};
use crate::ffi::rts::types::StgTSO;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_stack_underflow_frame_d_info, stg_stack_underflow_frame_v16_info,
    stg_stack_underflow_frame_v32_info, stg_stack_underflow_frame_v64_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8};
use crate::prelude::*;

unsafe fn cloneStackChunk(
    mut capability: *mut Capability,
    mut stack: *const StgStack,
) -> *mut StgStack {
    let mut spOffset: StgWord = (*stack)
        .sp
        .offset_from(&raw const (*stack).stack as *const StgWord)
        as c_long as StgWord;

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
        closureSizeBytes as size_t,
    );

    (*newStackClosure).sp =
        (&raw mut (*newStackClosure).stack as *mut StgWord).offset(spOffset as isize) as StgPtr;
    (*newStackClosure).dirty = 0 as StgWord8;

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

unsafe fn sendCloneStackMessage(mut tso: *mut StgTSO, mut mvar: HsStablePtr) -> ! {
    barf(
        b"Sending CloneStackMessages is only available in threaded RTS!\0" as *const u8
            as *const c_char,
    );
}
