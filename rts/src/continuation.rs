use crate::ffi::rts::constants::{TSO_BLOCKEX, TSO_INTERRUPTIBLE};
use crate::ffi::rts::storage::closure_macros::{
    CONTINUATION_sizeW, TAG_CLOSURE, get_ret_itbl, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closure_types::UNDERFLOW_FRAME;
use crate::ffi::rts::storage::closures::{
    StgContinuation, StgPromptFrame, StgPromptTag, StgUnderflowFrame,
};
use crate::ffi::rts::storage::gc::allocate;
use crate::ffi::rts::storage::tso::{StgStack, dirty_STACK, dirty_TSO};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_CONTINUATION_info, stg_maskAsyncExceptionszh_ret_info, stg_maskUninterruptiblezh_ret_info,
    stg_prompt_frame_info, stg_unmaskAsyncExceptionszh_ret_info,
};
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgWord, StgWord32};
use crate::prelude::*;
use crate::threads::threadStackUnderflow;

unsafe fn is_mask_frame_info(mut info: *const StgInfoTable) -> bool {
    return info == &raw const stg_unmaskAsyncExceptionszh_ret_info
        || info == &raw const stg_maskAsyncExceptionszh_ret_info
        || info == &raw const stg_maskUninterruptiblezh_ret_info;
}

unsafe fn pop_stack_chunk(mut cap: *mut Capability, mut tso: *mut StgTSO) -> *mut StgStack {
    let mut stack = (*tso).stackobj as *mut StgStack;
    (*stack).sp = (&raw mut (*stack).stack as *mut StgWord)
        .offset((*stack).stack_size as isize)
        .offset(
            -((size_of::<StgUnderflowFrame>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as isize),
        ) as StgPtr;

    threadStackUnderflow(cap, tso);

    return (*tso).stackobj as *mut StgStack;
}

unsafe fn captureContinuationAndAbort(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut prompt_tag: StgPromptTag,
) -> *mut StgClosure {
    let mut stack = (*tso).stackobj as *mut StgStack;
    let mut frame = (*stack).sp;
    let mut total_words: StgWord = 0;
    let mut in_first_chunk = true;
    let mut first_chunk_words: StgWord = 0;
    let mut last_chunk_words: StgWord = 0;
    let mut full_chunks: StgWord = 0;
    let mut apply_mask_frame = null::<StgInfoTable>();
    let mut mask_frame_offset: StgWord = 0;

    loop {
        let mut info_ptr = (*(frame as *mut StgClosure)).header.info;
        let mut info = get_ret_itbl(frame as *mut StgClosure);
        let mut chunk_words: StgWord = frame.offset_from((*stack).sp) as i64 as StgWord;

        if info_ptr == &raw const stg_prompt_frame_info
            && (*(frame as *mut StgPromptFrame)).tag == prompt_tag
        {
            total_words = total_words.wrapping_add(chunk_words);

            if in_first_chunk {
                first_chunk_words = chunk_words;
            } else {
                last_chunk_words = chunk_words;
            }

            break;
        } else if (*info).i.r#type == UNDERFLOW_FRAME as StgHalfWord {
            total_words = total_words.wrapping_add(chunk_words);

            if in_first_chunk {
                first_chunk_words = chunk_words;
            } else {
                full_chunks = full_chunks.wrapping_add(1);
            }

            stack = (*(frame as *mut StgUnderflowFrame)).next_chunk as *mut StgStack;
            frame = (*stack).sp;
            in_first_chunk = false;
        } else {
            if ((*info).i.r#type == 36
                || (*info).i.r#type == 33
                || (*info).i.r#type == 55
                || (*info).i.r#type == 56
                || (*info).i.r#type == 57) as i32 as i64
                != 0
            {
                return null_mut::<StgClosure>();
            }

            if is_mask_frame_info(info_ptr) {
                mask_frame_offset = total_words.wrapping_add(chunk_words);

                if apply_mask_frame.is_null() {
                    if (*tso).flags & TSO_BLOCKEX as StgWord32 == 0 {
                        apply_mask_frame = &raw const stg_unmaskAsyncExceptionszh_ret_info;
                    } else if (*tso).flags & TSO_INTERRUPTIBLE as StgWord32 == 0 {
                        apply_mask_frame = &raw const stg_maskUninterruptiblezh_ret_info;
                    } else {
                        apply_mask_frame = &raw const stg_maskAsyncExceptionszh_ret_info;
                    }
                }
            }

            frame = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);
        }
    }

    dirty_TSO(cap, tso);
    dirty_STACK(cap, stack);

    let mut cont = allocate(cap, CONTINUATION_sizeW(total_words) as W_) as *mut StgContinuation;
    (*cont).header.info = &raw const stg_CONTINUATION_info;
    (*cont).apply_mask_frame = apply_mask_frame;
    (*cont).mask_frame_offset = mask_frame_offset;
    (*cont).stack_size = total_words;
    stack = (*tso).stackobj as *mut StgStack;

    let mut cont_stack = &raw mut (*cont).stack as StgPtr;

    memcpy(
        cont_stack as *mut c_void,
        (*stack).sp as *const c_void,
        first_chunk_words.wrapping_mul(size_of::<StgWord>() as StgWord) as usize,
    );

    cont_stack = cont_stack.offset(first_chunk_words as isize);

    if in_first_chunk {
        (*stack).sp = (*stack).sp.offset(first_chunk_words as isize);
    } else {
        stack = pop_stack_chunk(cap, tso);

        let mut i: StgWord = 0;

        while i < full_chunks {
            let chunk_words_0: usize = ((&raw mut (*stack).stack as *mut StgWord)
                .offset((*stack).stack_size as isize)
                .offset_from((*stack).sp) as i64 as usize)
                .wrapping_sub(
                    (size_of::<StgUnderflowFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize),
                );

            memcpy(
                cont_stack as *mut c_void,
                (*stack).sp as *const c_void,
                chunk_words_0.wrapping_mul(size_of::<StgWord>() as usize),
            );

            cont_stack = cont_stack.offset(chunk_words_0 as isize);
            stack = pop_stack_chunk(cap, tso);
            i = i.wrapping_add(1);
        }

        memcpy(
            cont_stack as *mut c_void,
            (*stack).sp as *const c_void,
            last_chunk_words.wrapping_mul(size_of::<StgWord>() as StgWord) as usize,
        );

        cont_stack = cont_stack.offset(last_chunk_words as isize);
        (*stack).sp = (*stack).sp.offset(last_chunk_words as isize);
    }

    (*stack).sp = (*stack)
        .sp
        .offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);

    return TAG_CLOSURE(2, cont as *mut StgClosure);
}
