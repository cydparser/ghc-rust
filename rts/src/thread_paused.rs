use crate::capability::recordClosureMutated;
use crate::ffi::rts::constants::{TSO_SQUEEZED, ThreadKilled};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::storage::closure_macros::{
    SET_INFO, SET_INFO_RELEASE, UNTAG_CONST_CLOSURE, get_itbl, get_ret_itbl, stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{StgInd, StgUpdateFrame};
use crate::ffi::rts::storage::info_tables::{_IND, closure_flags};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::misc_closures::{
    stg_BLACKHOLE_info, stg_WHITEHOLE_info, stg_bh_upd_frame_info, stg_enter_info,
    stg_marked_upd_frame_info,
};
use crate::ffi::stg::types::{StgPtr, StgWord, StgWord8, StgWord16, StgWord32};
use crate::ffi::stg::{P_, W_};
use crate::prelude::*;
use crate::raise_async::{maybePerformBlockedException, suspendComputation};
use crate::threads::updateThunk;
use crate::trace::{DEBUG_RTS, trace_};

/// cbindgen:no-export
struct stack_gap {
    gap_size: StgWord,
    next_gap: *mut stack_gap,
}

unsafe fn updateAdjacentFrames(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut upd: *mut StgUpdateFrame,
    mut count: uint32_t,
    mut next: *mut stack_gap,
) -> *mut stack_gap {
    let mut updatee = null_mut::<StgClosure>();
    let mut gap = null_mut::<stack_gap>();
    let mut i: uint32_t = 0;
    updatee = (*upd).updatee;
    count = count.wrapping_sub(1);
    upd = upd.offset(-1);
    gap = upd as *mut stack_gap;
    i = count;

    while i > 0 as uint32_t {
        if (*upd).updatee != updatee
            && *(&raw const closure_flags as *const StgWord16)
                .offset((*get_itbl(UNTAG_CONST_CLOSURE((*upd).updatee))).r#type as isize)
                as c_int
                & _IND
                == 0
        {
            updateThunk(cap, tso, (*upd).updatee, updatee);
        }

        i = i.wrapping_sub(1);
        upd = upd.offset(-1);
    }

    (*gap).gap_size = (count as usize).wrapping_mul(
        (size_of::<StgUpdateFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize),
    ) as StgWord;

    (*gap).next_gap = next;

    return gap;
}

unsafe fn stackSqueeze(mut cap: *mut Capability, mut tso: *mut StgTSO, mut bottom: StgPtr) {
    let mut frame = null_mut::<StgWord>();
    let mut adjacent_update_frames: uint32_t = 0;
    let mut gap = null_mut::<stack_gap>();
    frame = (*(*tso).stackobj).sp;
    adjacent_update_frames = 0 as uint32_t;

    gap = frame.offset(
        -((size_of::<StgUpdateFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize),
    ) as *mut stack_gap;

    while frame <= bottom {
        match (*get_ret_itbl(frame as *mut StgClosure)).i.r#type {
            33 => {
                adjacent_update_frames > 0 as uint32_t;
                adjacent_update_frames = adjacent_update_frames.wrapping_add(1);

                frame = frame.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );
            }
            _ => {
                if adjacent_update_frames > 1 as uint32_t {
                    gap = updateAdjacentFrames(
                        cap,
                        tso,
                        frame.offset(
                            -((size_of::<StgUpdateFrame>() as usize)
                                .wrapping_add(size_of::<W_>() as usize)
                                .wrapping_sub(1 as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as isize),
                        ) as *mut StgUpdateFrame,
                        adjacent_update_frames,
                        gap,
                    );
                }

                adjacent_update_frames = 0 as uint32_t;
                frame = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);
            }
        }
    }

    if adjacent_update_frames > 1 as uint32_t {
        gap = updateAdjacentFrames(
            cap,
            tso,
            frame.offset(
                -((size_of::<StgUpdateFrame>() as usize)
                    .wrapping_add(size_of::<W_>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(size_of::<W_>() as usize) as isize),
            ) as *mut StgUpdateFrame,
            adjacent_update_frames,
            gap,
        );
    }

    let mut sp = null_mut::<StgWord8>();
    let mut gap_start = null_mut::<StgWord8>();
    let mut next_gap_start = null_mut::<StgWord8>();
    let mut gap_end = null_mut::<StgWord8>();
    let mut chunk_size: uint32_t = 0;
    next_gap_start = (gap as *mut StgWord8).offset(size_of::<StgUpdateFrame>() as usize as isize);
    sp = next_gap_start;

    while gap as StgPtr > (*(*tso).stackobj).sp {
        gap_start = next_gap_start;
        gap_end =
            gap_start.offset(-((*gap).gap_size.wrapping_mul(size_of::<W_>() as StgWord) as isize));
        gap = (*gap).next_gap;
        next_gap_start =
            (gap as *mut StgWord8).offset(size_of::<StgUpdateFrame>() as usize as isize);
        chunk_size = gap_end.offset_from(next_gap_start) as c_long as uint32_t;
        sp = sp.offset(-(chunk_size as isize));

        memmove(
            sp as *mut c_void,
            next_gap_start as *const c_void,
            chunk_size as size_t,
        );
    }

    (*(*tso).stackobj).sp = sp as StgPtr;
}

unsafe fn threadPaused(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    let mut frame = null_mut::<StgClosure>();
    let mut info = null::<StgRetInfoTable>();
    let mut bh_info = null::<StgInfoTable>();
    let mut cur_bh_info = null::<StgInfoTable>();
    let mut frame_info = null::<StgInfoTable>();
    let mut bh = null_mut::<StgClosure>();
    let mut stack_end = null_mut::<StgWord>();
    let mut words_to_squeeze: uint32_t = 0 as uint32_t;
    let mut weight: uint32_t = 0 as uint32_t;
    let mut weight_pending: uint32_t = 0 as uint32_t;
    let mut prev_was_update_frame = r#false != 0;
    let mut heuristic_says_squeeze: StgWord = 0;
    maybePerformBlockedException(cap, tso);

    if (*tso).what_next as c_int == ThreadKilled {
        return;
    }

    stack_end = (&raw mut (*(*tso).stackobj).stack as *mut StgWord)
        .offset((*(*tso).stackobj).stack_size as isize) as StgPtr;
    frame = (*(*tso).stackobj).sp as *mut StgClosure;

    while (frame as P_) < stack_end {
        info = get_ret_itbl(frame);

        match (*info).i.r#type {
            33 => {
                frame_info = (*frame).header.info;

                if frame_info
                    == &raw const stg_marked_upd_frame_info as *mut StgInfoTable
                        as *const StgInfoTable
                {
                    if prev_was_update_frame {
                        words_to_squeeze = (words_to_squeeze as c_ulong).wrapping_add(
                            (size_of::<StgUpdateFrame>() as usize)
                                .wrapping_add(size_of::<W_>() as usize)
                                .wrapping_sub(1 as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as c_ulong,
                        ) as uint32_t as uint32_t;

                        weight = weight.wrapping_add(weight_pending);
                        weight_pending = 0 as uint32_t;
                    }

                    break;
                } else {
                    SET_INFO(
                        frame,
                        &raw const stg_marked_upd_frame_info as *mut StgInfoTable,
                    );

                    bh = (*(frame as *mut StgUpdateFrame)).updatee;
                    bh_info = (*bh).header.info;

                    if bh_info == &raw const stg_BLACKHOLE_info
                        && (*(bh as *mut StgInd)).indirectee != tso as *mut StgClosure
                        || bh_info == &raw const stg_WHITEHOLE_info
                    {
                        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.squeeze as c_long != 0 {
                            trace_(
                                b"suspending duplicate work: %ld words of stack\0" as *const u8
                                    as *const c_char as *mut c_char,
                                (frame as StgPtr).offset_from((*(*tso).stackobj).sp) as c_long,
                            );
                        }

                        suspendComputation(cap, tso, frame as *mut StgUpdateFrame);
                        (*(*tso).stackobj).sp = (frame as StgPtr)
                            .offset(
                                (size_of::<StgUpdateFrame>() as usize)
                                    .wrapping_add(size_of::<W_>() as usize)
                                    .wrapping_sub(1 as usize)
                                    .wrapping_div(size_of::<W_>() as usize)
                                    as isize,
                            )
                            .offset(-(2 as c_int as isize));
                        *(*(*tso).stackobj).sp.offset(1 as c_int as isize) = bh as StgWord;
                        *(*(*tso).stackobj).sp.offset(0 as c_int as isize) =
                            &raw const stg_enter_info as W_ as StgWord;
                        frame =
                            (*(*tso).stackobj).sp.offset(2 as c_int as isize) as *mut StgClosure;
                        prev_was_update_frame = r#false != 0;
                    } else {
                        if !(frame_info == &raw const stg_bh_upd_frame_info) {
                            let ref mut fresh5 = (*(bh as *mut StgInd)).indirectee;
                            *fresh5 = tso as *mut StgClosure;
                            SET_INFO_RELEASE(bh, &raw const stg_BLACKHOLE_info);
                        }

                        recordClosureMutated(cap, bh);
                        frame = (frame as *mut StgUpdateFrame).offset(1 as c_int as isize)
                            as *mut StgClosure;

                        if prev_was_update_frame {
                            words_to_squeeze = (words_to_squeeze as c_ulong).wrapping_add(
                                (size_of::<StgUpdateFrame>() as usize)
                                    .wrapping_add(size_of::<W_>() as usize)
                                    .wrapping_sub(1 as usize)
                                    .wrapping_div(size_of::<W_>() as usize)
                                    as c_ulong,
                            ) as uint32_t
                                as uint32_t;
                            weight = weight.wrapping_add(weight_pending);
                            weight_pending = 0 as uint32_t;
                        }

                        prev_was_update_frame = r#true != 0;
                    }
                }
            }
            35 | 36 => {
                break;
            }
            _ => {
                let mut frame_size: uint32_t = stack_frame_sizeW(frame) as uint32_t;
                weight_pending = weight_pending.wrapping_add(frame_size);
                frame = (frame as StgPtr).offset(frame_size as isize) as *mut StgClosure;
                prev_was_update_frame = r#false != 0;
            }
        }
    }

    heuristic_says_squeeze = (weight <= 8 as uint32_t && words_to_squeeze > 0 as uint32_t
        || weight < words_to_squeeze) as c_int as StgWord;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.squeeze as c_long != 0 {
        trace_(
            b"words_to_squeeze: %d, weight: %d, squeeze: %s\0" as *const u8 as *const c_char
                as *mut c_char,
            words_to_squeeze,
            weight,
            if heuristic_says_squeeze != 0 {
                b"YES\0" as *const u8 as *const c_char
            } else {
                b"NO\0" as *const u8 as *const c_char
            },
        );
    }

    if RtsFlags.GcFlags.squeezeUpdFrames as c_int == r#true && heuristic_says_squeeze != 0 {
        stackSqueeze(cap, tso, frame as StgPtr);
        (*tso).flags |= TSO_SQUEEZED as StgWord32;
    } else {
        (*tso).flags &= !TSO_SQUEEZED as StgWord32;
    };
}
