use crate::capability::Capability;
use crate::capability::recordClosureMutated;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE, TSO_SQUEEZED, ThreadKilled};
use crate::ffi::rts::non_moving::nonmoving_write_barrier_enabled;
use crate::ffi::rts::prof::ccs::era;
use crate::ffi::rts::storage::closure_macros::{
    INFO_PTR_TO_STRUCT, SET_INFO, SET_INFO_RELEASE, THUNK_INFO_PTR_TO_STRUCT, UNTAG_CONST_CLOSURE,
    closure_sizeW_, doingLDVProfiling, get_itbl, get_ret_itbl, overwritingClosureSize,
    stack_frame_sizeW,
};
use crate::ffi::rts::storage::closures::{StgInd, StgThunk, StgUpdateFrame};
use crate::ffi::rts::storage::info_tables::{_IND, _THU, closure_flags};
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::stg::misc_closures::{
    __stg_EAGER_BLACKHOLE_info, stg_BLACKHOLE_info, stg_CAF_BLACKHOLE_info, stg_TSO_info,
    stg_WHITEHOLE_info, stg_bh_upd_frame_info, stg_enter_info, stg_marked_upd_frame_info,
};
use crate::ffi::stg::smp::{busy_wait_nop, cas};
use crate::ffi::stg::types::StgWord64;
use crate::ffi::stg::types::{
    StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord16, StgWord32, StgWord64,
};
use crate::ffi::stg::{P_, W_};
use crate::prelude::*;
use crate::raise_async::{maybePerformBlockedException, suspendComputation};
use crate::rts_flags::RtsFlags;
use crate::sm::non_moving_mark::{updateRemembSetPushClosure, updateRemembSetPushThunkEager};
use crate::thread_paused::whitehole_threadPaused_spin;
use crate::threads::updateThunk;
use crate::trace::{DEBUG_RTS, trace_};

extern "C" {
    pub(crate) static mut whitehole_threadPaused_spin: StgWord64;
}

/// cbindgen:no-export
struct stack_gap {
    gap_size: StgWord,
    next_gap: *mut stack_gap,
}

unsafe fn updateAdjacentFrames(
    mut cap: *mut Capability,
    mut tso: *mut StgTSO,
    mut upd: *mut StgUpdateFrame,
    mut count: u32,
    mut next: *mut stack_gap,
) -> *mut stack_gap {
    let mut updatee = null_mut::<StgClosure>();
    let mut gap = null_mut::<stack_gap>();
    let mut i: u32 = 0;
    updatee = (*upd).updatee;
    count = count.wrapping_sub(1);
    upd = upd.offset(-1);
    gap = upd as *mut stack_gap;
    i = count;

    while i > 0 {
        if (*upd).updatee != updatee
            && *(&raw const closure_flags as *const StgWord16)
                .offset((*get_itbl(UNTAG_CONST_CLOSURE((*upd).updatee))).r#type as isize)
                as i32
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
    let mut adjacent_update_frames: u32 = 0;
    let mut gap = null_mut::<stack_gap>();
    frame = (*(*tso).stackobj).sp;

    if (frame < bottom) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/ThreadPaused.c".as_ptr(), 96);
    }

    adjacent_update_frames = 0;
    gap = frame.offset(
        -((size_of::<StgUpdateFrame>() as usize)
            .wrapping_add(size_of::<W_>() as usize)
            .wrapping_sub(1 as usize)
            .wrapping_div(size_of::<W_>() as usize) as isize),
    ) as *mut stack_gap;

    while frame <= bottom {
        match (*get_ret_itbl(frame as *mut StgClosure)).i.r#type {
            33 => {
                adjacent_update_frames > 0;
                adjacent_update_frames = adjacent_update_frames.wrapping_add(1);
                frame = frame.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );
            }
            _ => {
                if adjacent_update_frames > 1 {
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

                adjacent_update_frames = 0;
                frame = frame.offset(stack_frame_sizeW(frame as *mut StgClosure) as isize);
            }
        }
    }

    if adjacent_update_frames > 1 {
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
    let mut chunk_size: u32 = 0;
    next_gap_start = (gap as *mut StgWord8).offset(size_of::<StgUpdateFrame>() as usize as isize);
    sp = next_gap_start;

    while gap as StgPtr > (*(*tso).stackobj).sp {
        gap_start = next_gap_start;
        gap_end =
            gap_start.offset(-((*gap).gap_size.wrapping_mul(size_of::<W_>() as StgWord) as isize));

        gap = (*gap).next_gap;
        next_gap_start =
            (gap as *mut StgWord8).offset(size_of::<StgUpdateFrame>() as usize as isize);
        chunk_size = gap_end.offset_from(next_gap_start) as i64 as u32;
        sp = sp.offset(-(chunk_size as isize));
        memmove(
            sp as *mut c_void,
            next_gap_start as *const c_void,
            chunk_size as usize,
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
    let mut words_to_squeeze: u32 = 0;
    let mut weight: u32 = 0;
    let mut weight_pending: u32 = 0;
    let mut prev_was_update_frame = false;
    let mut heuristic_says_squeeze: StgWord = 0;
    maybePerformBlockedException(cap, tso);

    if (*tso).what_next as i32 == ThreadKilled {
        return;
    }

    stack_end = (&raw mut (*(*tso).stackobj).stack as *mut StgWord)
        .offset((*(*tso).stackobj).stack_size as isize) as StgPtr;
    frame = (*(*tso).stackobj).sp as *mut StgClosure;

    's_44: while (frame as P_) < stack_end {
        info = get_ret_itbl(frame);

        match (*info).i.r#type {
            33 => {
                frame_info = (&raw mut (*frame).header.info).load(Ordering::Acquire);

                if frame_info
                    == &raw const stg_marked_upd_frame_info as *mut StgInfoTable
                        as *const StgInfoTable
                {
                    if prev_was_update_frame {
                        words_to_squeeze = (words_to_squeeze as u64).wrapping_add(
                            (size_of::<StgUpdateFrame>() as usize)
                                .wrapping_add(size_of::<W_>() as usize)
                                .wrapping_sub(1 as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as u64,
                        ) as u32 as u32;

                        weight = weight.wrapping_add(weight_pending);
                        weight_pending = 0;
                    }

                    break;
                } else {
                    SET_INFO(
                        frame,
                        &raw const stg_marked_upd_frame_info as *mut StgInfoTable,
                    );

                    bh = (*(frame as *mut StgUpdateFrame)).updatee;
                    bh_info = (&raw mut (*bh).header.info).load(Ordering::Acquire);

                    if nonmoving_write_barrier_enabled as i64 != 0 {
                        updateRemembSetPushClosure(cap, bh);
                    }

                    loop {
                        if bh_info == &raw const stg_BLACKHOLE_info
                            && (&raw mut (*(bh as *mut StgInd)).indirectee).load(Ordering::Relaxed)
                                != tso as *mut StgClosure
                            || bh_info == &raw const stg_WHITEHOLE_info
                        {
                            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.squeeze as i64 != 0 {
                                trace_(
                                    c"suspending duplicate work: %ld words of stack".as_ptr(),
                                    (frame as StgPtr).offset_from((*(*tso).stackobj).sp) as i64,
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
                                .offset(-2);
                            *(*(*tso).stackobj).sp.offset(1) = bh as StgWord;

                            if ((&raw mut (*bh).header.info).load(Ordering::Relaxed)
                                != &raw const stg_TSO_info) as i32
                                as i64
                                != 0
                            {
                            } else {
                                _assertFail(c"rts/ThreadPaused.c".as_ptr(), 308);
                            }

                            *(*(*tso).stackobj).sp.offset(0) =
                                &raw const stg_enter_info as W_ as StgWord;
                            frame = (*(*tso).stackobj).sp.offset(2) as *mut StgClosure;
                            prev_was_update_frame = false;
                            continue 's_44;
                        } else if frame_info == &raw const stg_bh_upd_frame_info {
                            if (bh_info == &raw const stg_BLACKHOLE_info
                                || bh_info == &raw const __stg_EAGER_BLACKHOLE_info
                                || bh_info == &raw const stg_CAF_BLACKHOLE_info)
                                as i32 as i64
                                != 0
                            {
                            } else {
                                _assertFail(c"rts/ThreadPaused.c".as_ptr(), 328);
                            }

                            break;
                        } else {
                            cur_bh_info = cas(
                                &raw mut (*bh).header.info as StgVolatilePtr,
                                bh_info as StgWord,
                                &raw const stg_WHITEHOLE_info as StgWord,
                            ) as *const StgInfoTable;

                            if cur_bh_info != bh_info {
                                bh_info = cur_bh_info;
                                (&raw mut whitehole_threadPaused_spin).store(
                                    (&raw mut whitehole_threadPaused_spin)
                                        .load(Ordering::Relaxed)
                                        .wrapping_add(1 as StgWord64),
                                    Ordering::Relaxed,
                                );

                                busy_wait_nop();
                            } else {
                                if (bh_info != &raw const stg_WHITEHOLE_info) as i32 as i64 != 0 {
                                } else {
                                    _assertFail(c"rts/ThreadPaused.c".as_ptr(), 350);
                                }

                                if nonmoving_write_barrier_enabled as i64 != 0 {
                                    if *(&raw const closure_flags as *const StgWord16)
                                        .offset((*INFO_PTR_TO_STRUCT(bh_info)).r#type as isize)
                                        as i32
                                        & _THU
                                        != 0
                                    {
                                        updateRemembSetPushThunkEager(
                                            cap,
                                            THUNK_INFO_PTR_TO_STRUCT(bh_info),
                                            bh as *mut StgThunk,
                                        );
                                    }
                                }

                                overwritingClosureSize(
                                    bh,
                                    closure_sizeW_(bh, INFO_PTR_TO_STRUCT(bh_info)),
                                );

                                (&raw mut (*(bh as *mut StgInd)).indirectee)
                                    .store(tso as *mut StgClosure, Ordering::Release);
                                SET_INFO_RELEASE(bh, &raw const stg_BLACKHOLE_info);
                                break;
                            }
                        }
                    }

                    recordClosureMutated(cap, bh);

                    if doingLDVProfiling() {
                        (*bh).header.prof.hp.ldvw =
                            (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
                    }

                    frame = (frame as *mut StgUpdateFrame).offset(1) as *mut StgClosure;

                    if prev_was_update_frame {
                        words_to_squeeze = (words_to_squeeze as u64).wrapping_add(
                            (size_of::<StgUpdateFrame>() as usize)
                                .wrapping_add(size_of::<W_>() as usize)
                                .wrapping_sub(1 as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as u64,
                        ) as u32 as u32;

                        weight = weight.wrapping_add(weight_pending);
                        weight_pending = 0;
                    }

                    prev_was_update_frame = true;
                }
            }
            35 | 36 => {
                break;
            }
            _ => {
                let mut frame_size: u32 = stack_frame_sizeW(frame) as u32;
                weight_pending = weight_pending.wrapping_add(frame_size);
                frame = (frame as StgPtr).offset(frame_size as isize) as *mut StgClosure;
                prev_was_update_frame = false;
            }
        }
    }

    heuristic_says_squeeze =
        (weight <= 8 && words_to_squeeze > 0 || weight < words_to_squeeze) as i32 as StgWord;

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.squeeze as i64 != 0 {
        trace_(
            c"words_to_squeeze: %d, weight: %d, squeeze: %s".as_ptr(),
            words_to_squeeze,
            weight,
            if heuristic_says_squeeze != 0 {
                c"YES".as_ptr()
            } else {
                c"NO".as_ptr()
            },
        );
    }

    if RtsFlags.GcFlags.squeezeUpdFrames as i32 == true && heuristic_says_squeeze != 0 {
        stackSqueeze(cap, tso, frame as StgPtr);
        (*tso).flags |= TSO_SQUEEZED as StgWord32;
    } else {
        (*tso).flags &= !TSO_SQUEEZED as StgWord32;
    };
}
