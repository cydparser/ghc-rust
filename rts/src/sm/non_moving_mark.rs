use crate::capability::regTableToCapability;
use crate::ffi::mach_deps::TAG_MASK;
use crate::ffi::mach_deps::TAG_MASK;
use crate::ffi::rts::constants::{BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::storage::block::{
    BF_COMPACT, BF_LARGE, BF_MARKED, BF_NONMOVING, BF_NONMOVING_SWEEPING, BF_PINNED, BLOCK_SIZE,
    BLOCK_SIZE_W, Bdescr, allocGroup, bdescr, bdescr_, block_get_flags, block_set_flag,
    dbl_link_onto, dbl_link_remove, freeChain_lock, freeGroup,
};
use crate::ffi::rts::storage::block::{BLOCK_SIZE, bdescr};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, STATIC_LINK, TAG_CLOSURE, THUNK_INFO_PTR_TO_STRUCT, UNTAG_CLOSURE,
    UNTAG_CONST_CLOSURE, get_fun_itbl, get_itbl, get_ret_itbl, itbl_to_fun_itbl,
    itbl_to_thunk_itbl,
};
use crate::ffi::rts::storage::closure_types::{CONSTR_0_1, CONSTR_0_2, CONSTR_NOCAF};
use crate::ffi::rts::storage::closures::StgMutArrPtrs;
use crate::ffi::rts::storage::closures::{
    _StgWeak, MessageThrowTo, StgAP, StgAP_STACK, StgBCO, StgBlockingQueue, StgClosure_,
    StgContinuation, StgInd, StgMVar, StgMutArrPtrs, StgMutVar, StgPAP, StgRetFun, StgSelector,
    StgSmallMutArrPtrs, StgTRecChunk, StgTRecHeader, StgTVar, StgThunk, StgUpdateFrame, StgWeak,
    TRecEntry,
};
use crate::ffi::rts::storage::gc::{memcount, oldest_gen};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::storage::info_tables::{
    StgFunInfoTable, StgLargeBitmap, StgSRTField, StgThunkInfoTable, stg_arg_bitmaps,
};
use crate::ffi::rts::storage::tso::{StgStack, StgTSO_};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgInfoTable, StgTSO};
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_DEAD_WEAK_info, stg_END_STM_CHUNK_LIST_closure, stg_END_TSO_QUEUE_closure,
    stg_NO_FINALIZER_closure, stg_NO_TREC_closure, stg_WHITEHOLE_info,
};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::smp::{cas, cas_word8};
use crate::ffi::stg::types::StgWord;
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgVolatilePtr, StgWord, StgWord8, StgWord32};
use crate::prelude::*;
use crate::sm::cnf::objectGetCompact;
use crate::sm::heap_utils::walk_large_bitmap;
use crate::sm::non_moving::{
    nonmoving_block_idx, nonmoving_segment_live_words, nonmovingClosureMarkedThisCycle,
    nonmovingGetBlockIdx, nonmovingGetClosureMark, nonmovingGetMark, nonmovingGetSegment,
    nonmovingMarkEpoch, nonmovingSegmentBlockSize, nonmovingSegmentGetBlock, nonmovingSegmentInfo,
    nonmovingSetMark,
};
use crate::sm::non_moving_mark::{
    C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5, EntryType, MARK_ARRAY,
    MARK_CLOSURE, MARK_PREFETCH_QUEUE_DEPTH, MARK_QUEUE_BLOCK_ENTRIES, MARK_QUEUE_BLOCKS,
    MarkBudget, MarkQueue, MarkQueue_, MarkQueueBlock, MarkQueueEnt, NULL_ENTRY,
    UNLIMITED_MARK_BUDGET, UpdRemSet, markQueueIsEmpty, nonmovingMarkQueueEntryType,
};
use crate::sm::non_moving_shortcut::nonmoving_eval_thunk_selector;
use crate::sm::storage::{STATIC_BITS, static_flag};
use crate::trace::{DEBUG_RTS, trace_, traceConcMarkBegin, traceConcMarkEnd};

#[cfg(test)]
mod tests;

/// cbindgen:no-export
pub(crate) struct UpdRemSet {
    pub(crate) queue: MarkQueue,
}

pub(crate) type MarkQueue = MarkQueue_;

/// cbindgen:no-export
pub(crate) struct MarkQueue_ {
    pub(crate) blocks: *mut bdescr,
    pub(crate) top: *mut MarkQueueBlock,
    pub(crate) is_upd_rem_set: bool,
    pub(crate) prefetch_queue: [MarkQueueEnt; 5],
    pub(crate) prefetch_head: uint8_t,
}

/// cbindgen:no-export
pub(crate) struct MarkQueueEnt {
    pub(crate) c2rust_unnamed: C2RustUnnamed_2,
}

pub(crate) union C2RustUnnamed_2 {
    pub(crate) null_entry: C2RustUnnamed_5,
    pub(crate) mark_closure: C2RustUnnamed_4,
    pub(crate) mark_array: C2RustUnnamed_3,
}

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_3 {
    pub(crate) array: *const StgMutArrPtrs,
    pub(crate) start_index: StgWord,
}

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_4 {
    pub(crate) p: *mut StgClosure,
    pub(crate) origin: *mut *mut StgClosure,
}

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_5 {
    pub(crate) p: *mut c_void,
}

/// cbindgen:no-export
pub(crate) struct MarkQueueBlock {
    pub(crate) head: uint32_t,
    pub(crate) entries: [MarkQueueEnt; 0],
}

pub(crate) const MARK_CLOSURE: EntryType = 1;

pub(crate) type EntryType = c_uint;

pub(crate) const MARK_ARRAY: EntryType = 2;

pub(crate) const NULL_ENTRY: EntryType = 0;

pub(crate) type MarkBudget = int64_t;

#[inline]
pub(crate) unsafe fn nonmovingMarkQueueEntryType(mut ent: *mut MarkQueueEnt) -> EntryType {
    let mut tag: uintptr_t =
        (*ent).c2rust_unnamed.null_entry.p as uintptr_t & TAG_MASK as uintptr_t;

    return tag as EntryType;
}

pub(crate) const MARK_PREFETCH_QUEUE_DEPTH: c_int = 5 as c_int;

pub(crate) const UNLIMITED_MARK_BUDGET: c_longlong = INT64_MIN;

pub(crate) const MARK_QUEUE_BLOCKS: c_int = 16 as c_int;

pub(crate) const MARK_QUEUE_BLOCK_ENTRIES: usize = (MARK_QUEUE_BLOCKS as usize)
    .wrapping_mul(BLOCK_SIZE as usize)
    .wrapping_sub(size_of::<MarkQueueBlock>() as usize)
    .wrapping_div(size_of::<MarkQueueEnt>() as usize);

#[inline]
pub(crate) unsafe fn nonmovingMarkUnlimitedBudget(mut queue: *mut MarkQueue_) {
    let mut budget: MarkBudget = UNLIMITED_MARK_BUDGET as MarkBudget;
    nonmovingMark(&raw mut budget, queue as *mut MarkQueue);
}

#[inline]
pub(crate) unsafe fn markQueueIsEmpty(mut q: *mut MarkQueue) -> bool {
    return (*q).blocks.is_null()
        || (*(*q).top).head == 0 as uint32_t && (*(*q).blocks).link.is_null();
}

const MARK_ARRAY_CHUNK_LENGTH: c_int = 128 as c_int;

static mut nonmoving_large_objects: *mut bdescr = null::<bdescr>() as *mut bdescr;

static mut nonmoving_marked_large_objects: *mut bdescr = null::<bdescr>() as *mut bdescr;

static mut n_nonmoving_large_blocks: memcount = 0 as memcount;

static mut n_nonmoving_marked_large_blocks: memcount = 0 as memcount;

static mut nonmoving_large_words: memcount = 0 as memcount;

static mut nonmoving_compact_words: memcount = 0 as memcount;

static mut nonmoving_compact_objects: *mut bdescr = null::<bdescr>() as *mut bdescr;

static mut nonmoving_marked_compact_objects: *mut bdescr = null::<bdescr>() as *mut bdescr;

static mut n_nonmoving_compact_blocks: memcount = 0 as memcount;

static mut n_nonmoving_marked_compact_blocks: memcount = 0 as memcount;

static mut nonmoving_old_threads: *mut StgTSO = unsafe {
    &raw const stg_END_TSO_QUEUE_closure as *mut StgClosure as *mut c_void as *mut StgTSO
};

static mut nonmoving_old_weak_ptr_list: *mut StgWeak = null::<StgWeak>() as *mut StgWeak;

static mut nonmoving_threads: *mut StgTSO = unsafe {
    &raw const stg_END_TSO_QUEUE_closure as *mut StgClosure as *mut c_void as *mut StgTSO
};

static mut nonmoving_weak_ptr_list: *mut StgWeak = null::<StgWeak>() as *mut StgWeak;

static mut upd_rem_set_block_list: *mut bdescr = null::<bdescr>() as *mut bdescr;

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut nonmoving_write_barrier_enabled: StgWord = r#false as StgWord;

static mut current_mark_queue: *mut MarkQueue = null::<MarkQueue>() as *mut MarkQueue;

unsafe fn nonmovingMarkInit() {}

unsafe fn nonmovingAddUpdRemSetBlocks_(mut rset: *mut MarkQueue) {
    let mut start = (*rset).blocks;
    let mut end = start;

    while !(*end).link.is_null() {
        end = (*end).link as *mut bdescr;
    }

    (*rset).blocks = null_mut::<bdescr>();
    (*end).link = upd_rem_set_block_list as *mut bdescr_;
    upd_rem_set_block_list = start;
}

unsafe fn nonmovingAddUpdRemSetBlocks_lock(mut rset: *mut MarkQueue) {
    if markQueueIsEmpty(rset) {
        return;
    }

    nonmovingAddUpdRemSetBlocks_(rset);
    init_mark_queue_(rset);
    (*rset).is_upd_rem_set = r#true != 0;
}

unsafe fn nonmovingAddUpdRemSetBlocks(mut rset: *mut UpdRemSet) {
    if markQueueIsEmpty(&raw mut (*rset).queue) {
        return;
    }

    nonmovingAddUpdRemSetBlocks_(&raw mut (*rset).queue);
    init_mark_queue_(&raw mut (*rset).queue);
    (*rset).queue.is_upd_rem_set = r#true != 0;
}

#[inline]
unsafe fn push(mut q: *mut MarkQueue, mut ent: *const MarkQueueEnt) {
    if (*(*q).top).head as usize == MARK_QUEUE_BLOCK_ENTRIES {
        if (*q).is_upd_rem_set {
            nonmovingAddUpdRemSetBlocks_lock(q);
        } else {
            let mut bd = allocGroup(MARK_QUEUE_BLOCKS as W_);
            (*bd).link = (*q).blocks as *mut bdescr_;
            (*q).blocks = bd;
            (*q).top = (*bd).start as *mut MarkQueueBlock;
            (*(*q).top).head = 0 as uint32_t;
        }
    }

    *(&raw mut (*(*q).top).entries as *mut MarkQueueEnt).offset((*(*q).top).head as isize) = *ent;
    (*(*q).top).head = (*(*q).top).head.wrapping_add(1);
}

unsafe fn markQueuePushClosureGC(mut q: *mut MarkQueue, mut p: *mut StgClosure) {
    if !check_in_nonmoving_heap(p) {
        return;
    }

    if (*(*q).top).head as usize == MARK_QUEUE_BLOCK_ENTRIES {
        let mut bd = allocGroup(MARK_QUEUE_BLOCKS as W_);
        (*bd).link = (*q).blocks as *mut bdescr_;
        (*q).blocks = bd;
        (*q).top = (*bd).start as *mut MarkQueueBlock;
        (*(*q).top).head = 0 as uint32_t;
    }

    let mut ent = MarkQueueEnt {
        c2rust_unnamed: C2RustUnnamed_2 {
            mark_closure: C2RustUnnamed_4 {
                p: TAG_CLOSURE(MARK_CLOSURE as c_int as StgWord, UNTAG_CLOSURE(p)),
                origin: null_mut::<*mut StgClosure>(),
            },
        },
    };

    *(&raw mut (*(*q).top).entries as *mut MarkQueueEnt).offset((*(*q).top).head as isize) = ent;
    (*(*q).top).head = (*(*q).top).head.wrapping_add(1);
}

#[inline]
unsafe fn push_closure(
    mut q: *mut MarkQueue,
    mut p: *mut StgClosure,
    mut origin: *mut *mut StgClosure,
) {
    let mut ent = MarkQueueEnt {
        c2rust_unnamed: C2RustUnnamed_2 {
            mark_closure: C2RustUnnamed_4 {
                p: TAG_CLOSURE(MARK_CLOSURE as c_int as StgWord, UNTAG_CLOSURE(p)),
                origin: origin,
            },
        },
    };

    push(q, &raw mut ent);
}

unsafe fn push_array(
    mut q: *mut MarkQueue,
    mut array: *const StgMutArrPtrs,
    mut start_index: StgWord,
) {
    if array as W_ >= mblock_address_space.0.begin
        && (array as W_) < mblock_address_space.0.end
        && (*Bdescr(array as StgPtr)).r#gen != oldest_gen
    {
        return;
    }

    let mut ent = MarkQueueEnt {
        c2rust_unnamed: C2RustUnnamed_2 {
            mark_array: C2RustUnnamed_3 {
                array: TAG_CLOSURE(
                    MARK_ARRAY as c_int as StgWord,
                    UNTAG_CLOSURE(array as *mut StgClosure),
                ) as *const StgMutArrPtrs,
                start_index: start_index,
            },
        },
    };

    push(q, &raw mut ent);
}

unsafe fn push_thunk_srt(mut q: *mut MarkQueue, mut info: *const StgInfoTable) {
    let mut thunk_info: *const StgThunkInfoTable = itbl_to_thunk_itbl(info);

    if (*thunk_info).i.srt != 0 {
        push_closure(
            q,
            (thunk_info.offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*thunk_info).i.srt as StgWord) as *mut StgClosure,
            null_mut::<*mut StgClosure>(),
        );
    }
}

unsafe fn push_fun_srt(mut q: *mut MarkQueue, mut info: *const StgInfoTable) {
    let mut fun_info: *const StgFunInfoTable = itbl_to_fun_itbl(info);

    if (*fun_info).i.srt != 0 {
        push_closure(
            q,
            (fun_info.offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*fun_info).i.srt as StgWord) as *mut StgClosure,
            null_mut::<*mut StgClosure>(),
        );
    }
}

unsafe fn check_in_nonmoving_heap(mut p: *mut StgClosure) -> bool {
    if p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end {
        return block_get_flags(Bdescr(p as StgPtr)) as c_int & BF_NONMOVING != 0;
    } else {
        return r#true != 0;
    };
}

#[inline]
unsafe fn updateRemembSetPushThunk(mut cap: *mut Capability, mut thunk: *mut StgThunk) {
    let mut info = null::<StgInfoTable>();

    loop {
        info = (*thunk).header.info as *mut StgInfoTable;

        if !(info == &raw const stg_WHITEHOLE_info) {
            break;
        }
    }

    let mut thunk_info: *const StgThunkInfoTable = THUNK_INFO_PTR_TO_STRUCT(info);
    updateRemembSetPushThunkEager(cap, thunk_info, thunk);
}

unsafe fn updateRemembSetPushThunkEager(
    mut cap: *mut Capability,
    mut info: *const StgThunkInfoTable,
    mut thunk: *mut StgThunk,
) {
    let mut queue: *mut MarkQueue = &raw mut (*cap).upd_rem_set.queue;

    match (*info).i.r#type {
        15 | 16 | 17 | 18 | 19 | 20 => {
            push_thunk_srt(queue, &raw const (*info).i);

            let mut i: StgWord = 0 as StgWord;

            while i < (*info).i.layout.payload.ptrs as StgWord {
                if check_in_nonmoving_heap(
                    *(&raw mut (*thunk).payload as *mut *mut StgClosure_).offset(i as isize)
                        as *mut StgClosure,
                ) {
                    push_closure(
                        queue,
                        *(&raw mut (*thunk).payload as *mut *mut StgClosure_).offset(i as isize)
                            as *mut StgClosure,
                        null_mut::<*mut StgClosure>(),
                    );
                }

                i = i.wrapping_add(1);
            }
        }
        22 => {
            let mut sel = thunk as *mut StgSelector;

            if check_in_nonmoving_heap((*sel).selectee) {
                push_closure(queue, (*sel).selectee, null_mut::<*mut StgClosure>());
            }
        }
        24 => {
            let mut ap = thunk as *mut StgAP;

            if check_in_nonmoving_heap((*ap).fun) {
                push_closure(queue, (*ap).fun, null_mut::<*mut StgClosure>());
            }

            trace_PAP_payload(
                queue,
                (*ap).fun,
                &raw mut (*ap).payload as *mut *mut StgClosure,
                (*ap).n_args as StgWord,
            );
        }
        38 => {}
        27 => {
            let mut ind = thunk as *mut StgInd;
            let mut indirectee = (*ind).indirectee;

            if check_in_nonmoving_heap(indirectee) {
                push_closure(queue, indirectee, null_mut::<*mut StgClosure>());
            }
        }
        _ => {
            barf(
                b"updateRemembSetPushThunk: invalid thunk pushed: p=%p, type=%d\0" as *const u8
                    as *const c_char,
                thunk,
                (*info).i.r#type,
            );
        }
    };
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn updateRemembSetPushThunk_(
    mut reg: *mut StgRegTable,
    mut p: *mut StgThunk,
) {
    updateRemembSetPushThunk(regTableToCapability(reg), p);
}

#[inline]
unsafe fn updateRemembSetPushClosure(mut cap: *mut Capability, mut p: *mut StgClosure) {
    if check_in_nonmoving_heap(p) {
        let mut queue: *mut MarkQueue = &raw mut (*cap).upd_rem_set.queue;
        push_closure(queue, p, null_mut::<*mut StgClosure>());
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn updateRemembSetPushClosure_(
    mut reg: *mut StgRegTable,
    mut p: *mut StgClosure_,
) {
    updateRemembSetPushClosure(regTableToCapability(reg), p as *mut StgClosure);
}

#[inline]
unsafe fn needs_upd_rem_set_mark(mut p: *mut StgClosure) -> bool {
    let mut bd = Bdescr(p as StgPtr);
    let mut flags = block_get_flags(bd);

    if (*bd).r#gen != oldest_gen {
        return r#false != 0;
    } else if flags as c_int & BF_LARGE != 0 {
        if flags as c_int & BF_NONMOVING_SWEEPING == 0 {
            return r#false != 0;
        } else {
            return flags as c_int & BF_MARKED == 0;
        }
    } else {
        let mut seg = nonmovingGetSegment(p as StgPtr);
        let mut block_idx = nonmovingGetBlockIdx(p as StgPtr);

        return nonmovingGetMark(seg, block_idx) as c_int != nonmovingMarkEpoch as c_int;
    };
}

unsafe fn finish_upd_rem_set_mark_large(mut bd: *mut bdescr) {
    if block_get_flags(bd) as c_int & BF_MARKED == 0 {
        block_set_flag(bd, BF_MARKED as uint16_t);
        dbl_link_remove(bd, &raw mut nonmoving_large_objects);
        dbl_link_onto(bd, &raw mut nonmoving_marked_large_objects);
        n_nonmoving_large_blocks = n_nonmoving_large_blocks.wrapping_sub((*bd).blocks as memcount);
        n_nonmoving_marked_large_blocks =
            n_nonmoving_marked_large_blocks.wrapping_add((*bd).blocks as memcount);
    }
}

#[inline]
unsafe fn finish_upd_rem_set_mark(mut p: *mut StgClosure) {
    let mut bd = Bdescr(p as StgPtr);

    if block_get_flags(bd) as c_int & BF_LARGE != 0 {
        finish_upd_rem_set_mark_large(bd);
    } else {
        let mut seg = nonmovingGetSegment(p as StgPtr);
        let mut block_idx = nonmovingGetBlockIdx(p as StgPtr);
        nonmovingSetMark(seg, block_idx);
    };
}

unsafe fn updateRemembSetPushTSO(mut cap: *mut Capability, mut tso: *mut StgTSO) {
    if needs_upd_rem_set_mark(tso as *mut StgClosure) {
        if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
            trace_(
                b"upd_rem_set: TSO %p\0" as *const u8 as *const c_char as *mut c_char,
                tso,
            );
        }

        trace_tso(&raw mut (*cap).upd_rem_set.queue, tso);
        finish_upd_rem_set_mark(tso as *mut StgClosure);
    }
}

unsafe fn updateRemembSetPushStack(mut cap: *mut Capability, mut stack: *mut StgStack) {
    if needs_upd_rem_set_mark(stack as *mut StgClosure) {
        let mut marking: StgWord8 = (*stack).marking;

        if cas_word8(
            &raw mut (*stack).marking,
            marking,
            nonmovingMarkEpoch as StgWord8,
        ) as c_int
            != nonmovingMarkEpoch as c_int
        {
            if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
                trace_(
                    b"upd_rem_set: STACK %p\0" as *const u8 as *const c_char as *mut c_char,
                    (*stack).sp,
                );
            }

            trace_stack(&raw mut (*cap).upd_rem_set.queue, stack);
            finish_upd_rem_set_mark(stack as *mut StgClosure);
            return;
        } else {
            while needs_upd_rem_set_mark(stack as *mut StgClosure) {}
            return;
        }
    }
}

unsafe fn updateRemembSetPushMessageThrowTo(mut cap: *mut Capability, mut m: *mut MessageThrowTo) {
    updateRemembSetPushClosure(cap, (*m).link as *mut StgClosure);
    updateRemembSetPushClosure(cap, (*m).source as *mut StgClosure);
    updateRemembSetPushClosure(cap, (*m).target as *mut StgClosure);
    updateRemembSetPushClosure(cap, (*m).exception);
}

unsafe fn markQueuePush(mut q: *mut MarkQueue, mut ent: *const MarkQueueEnt) {
    push(q, ent);
}

unsafe fn markQueuePushClosure(
    mut q: *mut MarkQueue,
    mut p: *mut StgClosure,
    mut origin: *mut *mut StgClosure,
) {
    if check_in_nonmoving_heap(p) {
        push_closure(q, p, origin);
    }
}

unsafe fn markQueueAddRoot(mut q: *mut MarkQueue, mut root: *mut *mut StgClosure) {
    markQueuePushClosureGC(q, *root);
}

unsafe fn markQueuePushClosure_(mut q: *mut MarkQueue, mut p: *mut StgClosure) {
    markQueuePushClosure(q, p, null_mut::<*mut StgClosure>());
}

unsafe fn markQueuePushFunSrt(mut q: *mut MarkQueue, mut info: *const StgInfoTable) {
    push_fun_srt(q, info);
}

unsafe fn markQueuePushThunkSrt(mut q: *mut MarkQueue, mut info: *const StgInfoTable) {
    push_thunk_srt(q, info);
}

unsafe fn markQueuePushArray(
    mut q: *mut MarkQueue,
    mut array: *const StgMutArrPtrs,
    mut start_index: StgWord,
) {
    push_array(q, array, start_index);
}

unsafe fn markQueuePop_(mut q: *mut MarkQueue) -> MarkQueueEnt {
    let mut top = null_mut::<MarkQueueBlock>();

    loop {
        top = (*q).top;

        if (*top).head == 0 as uint32_t {
            if (*(*q).blocks).link.is_null() {
                let mut none = MarkQueueEnt {
                    c2rust_unnamed: C2RustUnnamed_2 {
                        null_entry: C2RustUnnamed_5 { p: NULL },
                    },
                };

                return none;
            } else {
                let mut old_block = (*q).blocks;
                (*q).blocks = (*old_block).link as *mut bdescr;
                (*q).top = (*(*q).blocks).start as *mut MarkQueueBlock;
                freeGroup(old_block);
            }
        } else {
            (*top).head = (*top).head.wrapping_sub(1);

            let mut ent =
                *(&raw mut (*top).entries as *mut MarkQueueEnt).offset((*top).head as isize);

            return ent;
        }
    }
}

unsafe fn markQueuePop(mut q: *mut MarkQueue) -> MarkQueueEnt {
    let mut i = (*q).prefetch_head as c_uint;

    's_3: while nonmovingMarkQueueEntryType(
        (&raw mut (*q).prefetch_queue as *mut MarkQueueEnt).offset(i as isize) as *mut MarkQueueEnt,
    ) as c_uint
        == NULL_ENTRY as c_int as c_uint
    {
        let mut new = markQueuePop_(q);

        if nonmovingMarkQueueEntryType(&raw mut new) as c_uint == NULL_ENTRY as c_int as c_uint {
            let mut j = i
                .wrapping_add(1 as c_uint)
                .wrapping_rem(MARK_PREFETCH_QUEUE_DEPTH as c_uint);

            while j != i {
                if nonmovingMarkQueueEntryType(
                    (&raw mut (*q).prefetch_queue as *mut MarkQueueEnt).offset(j as isize)
                        as *mut MarkQueueEnt,
                ) as c_uint
                    != NULL_ENTRY as c_int as c_uint
                {
                    i = j;
                    break 's_3;
                } else {
                    j = j
                        .wrapping_add(1 as c_uint)
                        .wrapping_rem(MARK_PREFETCH_QUEUE_DEPTH as c_uint);
                }
            }

            return new;
        } else {
            &raw mut (*(UNTAG_CLOSURE as unsafe extern "C" fn(*mut StgClosure) -> *mut StgClosure)(
                new.c2rust_unnamed.mark_closure.p,
            ))
            .header
            .info;
            Bdescr(new.c2rust_unnamed.mark_closure.p as StgPtr);
            (*q).prefetch_queue[i as usize] = new;
            i = i
                .wrapping_add(1 as c_uint)
                .wrapping_rem(MARK_PREFETCH_QUEUE_DEPTH as c_uint);
        }
    }

    let mut ret = (*q).prefetch_queue[i as usize];
    (*q).prefetch_queue[i as usize].c2rust_unnamed.null_entry.p = NULL;
    (*q).prefetch_head = i as uint8_t;

    return ret;
}

unsafe fn init_mark_queue_(mut queue: *mut MarkQueue) {
    let mut bd = allocGroup(MARK_QUEUE_BLOCKS as W_);
    (*queue).blocks = bd;
    (*queue).top = (*bd).start as *mut MarkQueueBlock;
    (*(*queue).top).head = 0 as uint32_t;

    memset(
        &raw mut (*queue).prefetch_queue as *mut c_void,
        0 as c_int,
        size_of::<[MarkQueueEnt; 5]>() as size_t,
    );

    (*queue).prefetch_head = 0 as uint8_t;
}

unsafe fn initMarkQueue(mut queue: *mut MarkQueue) {
    init_mark_queue_(queue);
    (*queue).is_upd_rem_set = r#false != 0;
}

unsafe fn nonmovingInitUpdRemSet(mut rset: *mut UpdRemSet) {
    init_mark_queue_(&raw mut (*rset).queue);
    (*rset).queue.is_upd_rem_set = r#true != 0;
}

unsafe fn freeMarkQueue(mut queue: *mut MarkQueue) {
    freeChain_lock((*queue).blocks);
}

unsafe fn trace_trec_chunk(mut queue: *mut MarkQueue, mut chunk: *mut StgTRecChunk) {
    markQueuePushClosure_(queue, chunk as *mut StgClosure);

    let mut i: StgWord = 0 as StgWord;

    while i < (*chunk).next_entry_idx {
        let mut ent: *mut TRecEntry =
            (&raw mut (*chunk).entries as *mut TRecEntry).offset(i as isize) as *mut TRecEntry;
        markQueuePushClosure_(queue, (*ent).tvar as *mut StgClosure);
        markQueuePushClosure_(queue, (*ent).expected_value);
        markQueuePushClosure_(queue, (*ent).new_value);
        i = i.wrapping_add(1);
    }
}

unsafe fn trace_trec_header(mut queue: *mut MarkQueue, mut trec: *mut StgTRecHeader) {
    while trec != &raw mut stg_NO_TREC_closure as *mut c_void as *mut StgTRecHeader {
        let mut chunk = (*trec).current_chunk;
        markQueuePushClosure_(queue, trec as *mut StgClosure);

        while chunk != &raw mut stg_END_STM_CHUNK_LIST_closure as *mut c_void as *mut StgTRecChunk {
            trace_trec_chunk(queue, chunk);
            chunk = (*chunk).prev_chunk as *mut StgTRecChunk;
        }

        trec = (*trec).enclosing_trec as *mut StgTRecHeader;
    }
}

unsafe fn trace_tso(mut queue: *mut MarkQueue, mut tso: *mut StgTSO) {
    if !(*tso).bound.is_null() {
        markQueuePushClosure_(queue, (*(*tso).bound).tso as *mut StgClosure);
    }

    markQueuePushClosure_(queue, (*tso).blocked_exceptions as *mut StgClosure);
    markQueuePushClosure_(queue, (*tso).bq as *mut StgClosure);
    trace_trec_header(queue, (*tso).trec as *mut StgTRecHeader);
    markQueuePushClosure_(queue, (*tso).stackobj as *mut StgClosure);
    markQueuePushClosure_(queue, (*tso)._link as *mut StgClosure);

    if !(*tso).label.is_null() {
        markQueuePushClosure_(queue, (*tso).label as *mut StgClosure);
    }

    match (*tso).why_blocked {
        1 | 14 | 2 | 12 | 0 => {
            markQueuePushClosure_(queue, (*tso).block_info.closure);
        }
        _ => {}
    };
}

unsafe fn do_push_closure(mut p: *mut *mut StgClosure, mut user: *mut c_void) {
    let mut queue = user as *mut MarkQueue;
    markQueuePushClosure_(queue, *p);
}

unsafe fn trace_large_bitmap(
    mut queue: *mut MarkQueue,
    mut p: *mut *mut StgClosure,
    mut large_bitmap: *mut StgLargeBitmap,
    mut size: StgWord,
) {
    walk_large_bitmap(
        Some(do_push_closure as unsafe extern "C" fn(*mut *mut StgClosure, *mut c_void) -> ()),
        p,
        large_bitmap,
        size,
        queue as *mut c_void,
    );
}

unsafe fn trace_small_bitmap(
    mut queue: *mut MarkQueue,
    mut p: *mut *mut StgClosure,
    mut size: StgWord,
    mut bitmap: StgWord,
) {
    while size > 0 as StgWord {
        if bitmap & 1 as StgWord == 0 as StgWord {
            markQueuePushClosure(queue, *p, null_mut::<*mut StgClosure>());
        }

        p = p.offset(1);
        bitmap = bitmap >> 1 as c_int;
        size = size.wrapping_sub(1);
    }
}

unsafe fn trace_PAP_payload(
    mut queue: *mut MarkQueue,
    mut fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut size: StgWord,
) {
    let mut fun_info = get_fun_itbl(UNTAG_CONST_CLOSURE(fun));
    let mut p = payload as StgPtr;
    let mut bitmap: StgWord = 0;
    let mut current_block_7: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            current_block_7 = 4348410881353642614;
        }
        1 => {
            trace_large_bitmap(
                queue,
                payload,
                (fun_info.offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            current_block_7 = 13536709405535804910;
        }
        2 => {
            trace_large_bitmap(
                queue,
                payload,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size,
            );

            current_block_7 = 13536709405535804910;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;

            current_block_7 = 4348410881353642614;
        }
    }

    match current_block_7 {
        4348410881353642614 => {
            trace_small_bitmap(queue, p as *mut *mut StgClosure, size, bitmap);
        }
        _ => {}
    };
}

unsafe fn mark_arg_block(
    mut queue: *mut MarkQueue,
    mut fun_info: *const StgFunInfoTable,
    mut args: *mut *mut StgClosure,
) -> StgPtr {
    let mut bitmap: StgWord = 0;
    let mut size: StgWord = 0;
    let mut p = args as StgPtr;
    let mut current_block_8: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            size = (*fun_info).f.b.bitmap & BITMAP_SIZE_MASK as StgWord;
            current_block_8 = 9372144099661896856;
        }
        1 => {
            size = (*((fun_info.offset(1 as c_int as isize) as StgWord)
                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                as *mut StgLargeBitmap))
                .size;

            trace_large_bitmap(
                queue,
                p as *mut *mut StgClosure,
                (fun_info.offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size,
            );

            p = p.offset(size as isize);
            current_block_8 = 1394248824506584008;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;

            size = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                & BITMAP_SIZE_MASK as StgWord;
            current_block_8 = 9372144099661896856;
        }
    }

    match current_block_8 {
        9372144099661896856 => {
            trace_small_bitmap(queue, p as *mut *mut StgClosure, size, bitmap);
            p = p.offset(size as isize);
        }
        _ => {}
    }

    return p;
}

unsafe fn trace_stack_(mut queue: *mut MarkQueue, mut sp: StgPtr, mut spBottom: StgPtr) {
    while sp < spBottom {
        let mut info = get_ret_itbl(sp as *mut StgClosure);

        match (*info).i.r#type {
            33 => {
                let mut frame = sp as *mut StgUpdateFrame;
                markQueuePushClosure_(queue, (*frame).updatee);

                sp = sp.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                continue;
            }
            57 | 56 | 55 | 35 | 36 | 34 | 30 | 65 => {
                let mut bitmap: StgWord = (*info).i.layout.bitmap >> BITMAP_BITS_SHIFT;
                let mut size: StgWord = (*info).i.layout.bitmap & BITMAP_SIZE_MASK as StgWord;
                sp = sp.offset(1);
                trace_small_bitmap(queue, sp as *mut *mut StgClosure, size, bitmap);
                sp = sp.offset(size as isize);
            }
            29 => {
                sp = sp.offset(1);
                markQueuePushClosure_(queue, *(sp as *mut *mut StgClosure));

                let mut bco = *sp as *mut StgBCO;
                sp = sp.offset(1);

                let mut size_0: StgWord =
                    (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size;

                trace_large_bitmap(
                    queue,
                    sp as *mut *mut StgClosure,
                    &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                    size_0,
                );

                sp = sp.offset(size_0 as isize);
                continue;
            }
            31 => {
                let mut size_1: StgWord = 0;
                size_1 = (*(((&raw const (*info).i).offset(1 as c_int as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap))
                    .size;
                sp = sp.offset(1);

                trace_large_bitmap(
                    queue,
                    sp as *mut *mut StgClosure,
                    ((&raw const (*info).i).offset(1 as c_int as isize) as StgWord)
                        .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                        as *mut StgLargeBitmap,
                    size_1,
                );

                sp = sp.offset(size_1 as isize);
            }
            32 => {
                let mut ret_fun = sp as *mut StgRetFun;
                let mut fun_info = null::<StgFunInfoTable>();
                markQueuePushClosure_(queue, (*ret_fun).fun);
                fun_info = get_fun_itbl(UNTAG_CLOSURE((*ret_fun).fun));

                sp = mark_arg_block(
                    queue,
                    fun_info,
                    &raw mut (*ret_fun).payload as *mut *mut StgClosure,
                );
            }
            _ => {
                barf(
                    b"trace_stack: weird activation record found on stack: %d\0" as *const u8
                        as *const c_char,
                    (*info).i.r#type as c_int,
                );
            }
        }

        if (*info).i.srt != 0 {
            markQueuePushClosure_(
                queue,
                (info.offset(1 as c_int as isize) as StgWord).wrapping_add((*info).i.srt as StgWord)
                    as *mut StgClosure,
            );
        }
    }
}

unsafe fn trace_stack(mut queue: *mut MarkQueue, mut stack: *mut StgStack) {
    trace_stack_(
        queue,
        (*stack).sp,
        (&raw mut (*stack).stack as *mut StgWord).offset((*stack).stack_size as isize),
    );
}

unsafe fn bump_static_flag(mut link_field: *mut *mut StgClosure, mut q: *mut StgClosure) -> bool {
    let mut needs_marking: bool = false;
    let mut link: StgWord = *link_field as StgWord;

    if link & STATIC_BITS as StgWord == static_flag as StgWord {
        needs_marking = r#false != 0;
    } else {
        *link_field = (link & !STATIC_BITS as StgWord | static_flag as StgWord) as *mut StgClosure;
        needs_marking = r#true != 0;
    }

    return needs_marking;
}

unsafe fn mark_closure(
    mut queue: *mut MarkQueue,
    mut p0: *const StgClosure,
    mut origin: *mut *mut StgClosure,
) {
    let mut bd: *mut bdescr = null_mut::<bdescr>();
    let mut p_next: *mut StgClosure = null_mut::<StgClosure>();
    let mut tag: StgWord = 0;
    let mut flags: uint16_t = 0;
    let mut info_0: *const StgInfoTable = null::<StgInfoTable>();
    let mut bd_flags: uint16_t = 0;
    let mut current_block: u64;
    let mut p = p0 as *mut StgClosure;

    loop {
        bd = null_mut::<bdescr>();
        p_next = null_mut::<StgClosure>();
        tag = GET_CLOSURE_TAG(p);
        p = UNTAG_CLOSURE(p);

        if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
            let mut info = get_itbl(p);
            let mut r#type: StgHalfWord = (*info).r#type;

            if r#type == CONSTR_0_1 as StgHalfWord
                || r#type == CONSTR_0_2 as StgHalfWord
                || r#type == CONSTR_NOCAF as StgHalfWord
            {
                return;
            }

            match r#type {
                21 => {
                    if (*info).srt != 0 as StgSRTField {
                        if bump_static_flag(
                            (&raw mut (*p).payload as *mut *mut StgClosure_)
                                .offset(1 as c_int as isize)
                                as *mut *mut StgClosure,
                            p,
                        ) {
                            markQueuePushThunkSrt(queue, info);
                        }
                    }

                    break;
                }
                14 => {
                    if (*info).srt != 0 as StgSRTField
                        || (*info).layout.payload.ptrs != 0 as StgHalfWord
                    {
                        if bump_static_flag(STATIC_LINK(info, p), p) {
                            markQueuePushFunSrt(queue, info);

                            let mut i: StgHalfWord = 0 as StgHalfWord;

                            while i < (*info).layout.payload.ptrs {
                                markQueuePushClosure(
                                    queue,
                                    *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                        .offset(i as isize)
                                        as *mut StgClosure,
                                    (&raw mut (*p).payload as *mut *mut StgClosure_)
                                        .offset(i as isize)
                                        as *mut *mut StgClosure_
                                        as *mut *mut StgClosure,
                                );

                                i = i.wrapping_add(1);
                            }
                        }
                    }

                    break;
                }
                28 => {
                    if bump_static_flag(
                        (&raw mut (*p).payload as *mut *mut StgClosure_).offset(1 as c_int as isize)
                            as *mut *mut StgClosure,
                        p,
                    ) {
                        markQueuePushClosure(
                            queue,
                            (*(p as *mut StgInd)).indirectee,
                            &raw mut (*(p as *mut StgInd)).indirectee,
                        );
                    }

                    break;
                }
                1 | 2 | 4 | 5 => {
                    if bump_static_flag(STATIC_LINK(info, p), p) {
                        let mut i_0: StgHalfWord = 0 as StgHalfWord;

                        while i_0 < (*info).layout.payload.ptrs {
                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(i_0 as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(i_0 as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            i_0 = i_0.wrapping_add(1);
                        }
                    }

                    break;
                }
                58 => while (*p).header.info == &raw const stg_WHITEHOLE_info {},
                _ => {
                    barf(
                        b"mark_closure(static): strange closure type %d\0" as *const u8
                            as *const c_char,
                        (*info).r#type as c_int,
                    );
                }
            }
        } else {
            bd = Bdescr(p as StgPtr);

            if (*bd).r#gen != oldest_gen {
                break;
            }

            flags = block_get_flags(bd) as uint16_t;

            if flags as c_int & (BF_COMPACT | BF_NONMOVING) != 0 {
                if flags as c_int & BF_COMPACT != 0 {
                    let mut str = objectGetCompact(p);
                    bd = Bdescr(str as StgPtr);

                    if flags as c_int & BF_NONMOVING_SWEEPING == 0 {
                        return;
                    }

                    if flags as c_int & BF_MARKED == 0 {
                        dbl_link_remove(bd, &raw mut nonmoving_compact_objects);
                        dbl_link_onto(bd, &raw mut nonmoving_marked_compact_objects);

                        let mut blocks: StgWord =
                            (*str).totalW.wrapping_div(BLOCK_SIZE_W as StgWord);
                        n_nonmoving_compact_blocks = (n_nonmoving_compact_blocks as StgWord)
                            .wrapping_sub(blocks)
                            as memcount
                            as memcount;
                        n_nonmoving_marked_compact_blocks =
                            (n_nonmoving_marked_compact_blocks as StgWord).wrapping_add(blocks)
                                as memcount as memcount;
                        block_set_flag(bd, BF_MARKED as uint16_t);
                    }

                    break;
                } else {
                    if flags as c_int & BF_LARGE != 0 {
                        if flags as c_int & BF_NONMOVING_SWEEPING == 0 {
                            break;
                        }

                        if flags as c_int & BF_MARKED != 0 {
                            break;
                        }
                    } else {
                        let mut seg = nonmovingGetSegment(p as StgPtr);
                        let mut block_idx = nonmovingGetBlockIdx(p as StgPtr);
                        let mut mark = nonmovingGetMark(seg, block_idx);

                        if mark as c_int == nonmovingMarkEpoch as c_int {
                            break;
                        }

                        let mut snapshot_loc = nonmovingSegmentGetBlock(
                            seg,
                            (*nonmovingSegmentInfo(seg)).next_free_snap as nonmoving_block_idx,
                        ) as *mut StgClosure;

                        if p >= snapshot_loc && mark as c_int == 0 as c_int {
                            break;
                        }
                    }

                    info_0 = get_itbl(p);

                    match (*info_0).r#type {
                        39 | 40 => {
                            let mut mvar = p as *mut StgMVar;

                            markQueuePushClosure(
                                queue,
                                (*mvar).head as *mut StgClosure,
                                &raw mut (*mvar).head as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                (*mvar).tail as *mut StgClosure,
                                &raw mut (*mvar).tail as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(queue, (*mvar).value, &raw mut (*mvar).value);
                            current_block = 2413388577390654262;
                        }
                        41 => {
                            let mut tvar = p as *mut StgTVar;

                            markQueuePushClosure(
                                queue,
                                (*tvar).current_value,
                                &raw mut (*tvar).current_value,
                            );

                            markQueuePushClosure(
                                queue,
                                (*tvar).first_watch_queue_entry as *mut StgClosure,
                                &raw mut (*tvar).first_watch_queue_entry as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        11 => {
                            markQueuePushFunSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        18 => {
                            let mut thunk = p as *mut StgThunk;
                            markQueuePushThunkSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*thunk).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*thunk).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*thunk).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*thunk).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        4 => {
                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(1 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        16 => {
                            markQueuePushThunkSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        9 => {
                            markQueuePushFunSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        2 => {
                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        17 => {
                            markQueuePushThunkSrt(queue, info_0);
                            current_block = 2413388577390654262;
                        }
                        10 => {
                            markQueuePushFunSrt(queue, info_0);
                            current_block = 2413388577390654262;
                        }
                        20 => {
                            markQueuePushThunkSrt(queue, info_0);
                            current_block = 2413388577390654262;
                        }
                        13 => {
                            markQueuePushFunSrt(queue, info_0);
                            current_block = 2413388577390654262;
                        }
                        19 => {
                            markQueuePushThunkSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*(p as *mut StgThunk)).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        12 => {
                            markQueuePushFunSrt(queue, info_0);

                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        5 => {
                            markQueuePushClosure(
                                queue,
                                *(&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut StgClosure,
                                (&raw mut (*p).payload as *mut *mut StgClosure_)
                                    .offset(0 as c_int as isize)
                                    as *mut *mut StgClosure_
                                    as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        8 => {
                            markQueuePushFunSrt(queue, info_0);
                            current_block = 4751196792806374320;
                        }
                        15 => {
                            markQueuePushThunkSrt(queue, info_0);

                            let mut i_1: StgWord = 0 as StgWord;

                            while i_1 < (*info_0).layout.payload.ptrs as StgWord {
                                let mut field: *mut *mut StgClosure =
                                    (&raw mut (*(p as *mut StgThunk)).payload
                                        as *mut *mut StgClosure_)
                                        .offset(i_1 as isize)
                                        as *mut *mut StgClosure;
                                markQueuePushClosure(queue, *field, field);
                                i_1 = i_1.wrapping_add(1);
                            }

                            current_block = 2413388577390654262;
                        }
                        49 | 1 | 7 | 50 => {
                            current_block = 4751196792806374320;
                        }
                        23 => {
                            let mut bco = p as *mut StgBCO;

                            markQueuePushClosure(
                                queue,
                                (*bco).instrs as *mut StgClosure,
                                &raw mut (*bco).instrs as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                (*bco).literals as *mut StgClosure,
                                &raw mut (*bco).literals as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                (*bco).ptrs as *mut StgClosure,
                                &raw mut (*bco).ptrs as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        27 => {
                            markQueuePushClosure(
                                queue,
                                (*(p as *mut StgInd)).indirectee,
                                &raw mut (*(p as *mut StgInd)).indirectee,
                            );

                            if !origin.is_null() {
                                p_next = (*(p as *mut StgInd)).indirectee;
                            }

                            current_block = 2413388577390654262;
                        }
                        38 => {
                            let mut ind = p as *mut StgInd;
                            let mut indirectee = (*ind).indirectee;
                            markQueuePushClosure(queue, indirectee, &raw mut (*ind).indirectee);

                            if !(GET_CLOSURE_TAG(indirectee) == 0 as StgWord || origin.is_null()) {
                                p_next = indirectee;
                            }

                            current_block = 2413388577390654262;
                        }
                        47 | 48 => {
                            markQueuePushClosure(
                                queue,
                                (*(p as *mut StgMutVar)).var,
                                &raw mut (*(p as *mut StgMutVar)).var,
                            );

                            current_block = 2413388577390654262;
                        }
                        37 => {
                            let mut bq = p as *mut StgBlockingQueue;
                            markQueuePushClosure(queue, (*bq).bh, &raw mut (*bq).bh);

                            markQueuePushClosure(
                                queue,
                                (*bq).owner as *mut StgClosure,
                                &raw mut (*bq).owner as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                (*bq).queue as *mut StgClosure,
                                &raw mut (*bq).queue as *mut *mut StgClosure,
                            );

                            markQueuePushClosure(
                                queue,
                                (*bq).link as *mut StgClosure,
                                &raw mut (*bq).link as *mut *mut StgClosure,
                            );

                            current_block = 2413388577390654262;
                        }
                        22 => {
                            let mut sel = p as *mut StgSelector;
                            markQueuePushClosure(queue, (*sel).selectee, &raw mut (*sel).selectee);
                            nonmoving_eval_thunk_selector(queue, sel, origin);
                            current_block = 2413388577390654262;
                        }
                        26 => {
                            let mut ap = p as *mut StgAP_STACK;
                            markQueuePushClosure(queue, (*ap).fun, &raw mut (*ap).fun);

                            trace_stack_(
                                queue,
                                &raw mut (*ap).payload as *mut *mut StgClosure as StgPtr,
                                (&raw mut (*ap).payload as *mut *mut StgClosure as StgPtr)
                                    .offset((*ap).size as isize),
                            );

                            current_block = 2413388577390654262;
                        }
                        25 => {
                            let mut pap = p as *mut StgPAP;
                            markQueuePushClosure(queue, (*pap).fun, &raw mut (*pap).fun);

                            trace_PAP_payload(
                                queue,
                                (*pap).fun,
                                &raw mut (*pap).payload as *mut *mut StgClosure,
                                (*pap).n_args as StgWord,
                            );

                            current_block = 2413388577390654262;
                        }
                        24 => {
                            let mut ap_0 = p as *mut StgAP;
                            markQueuePushClosure(queue, (*ap_0).fun, &raw mut (*ap_0).fun);

                            trace_PAP_payload(
                                queue,
                                (*ap_0).fun,
                                &raw mut (*ap_0).payload as *mut *mut StgClosure,
                                (*ap_0).n_args as StgWord,
                            );

                            current_block = 2413388577390654262;
                        }
                        43 | 44 | 46 | 45 => {
                            markQueuePushArray(queue, p as *mut StgMutArrPtrs, 0 as StgWord);
                            current_block = 2413388577390654262;
                        }
                        59 | 60 | 62 | 61 => {
                            let mut arr = p as *mut StgSmallMutArrPtrs;
                            let mut i_3: StgWord = 0 as StgWord;

                            while i_3 < (*arr).ptrs {
                                let mut field_1: *mut *mut StgClosure = (&raw mut (*arr).payload
                                    as *mut *mut StgClosure)
                                    .offset(i_3 as isize)
                                    as *mut *mut StgClosure;
                                markQueuePushClosure(queue, *field_1, field_1);
                                i_3 = i_3.wrapping_add(1);
                            }

                            current_block = 2413388577390654262;
                        }
                        52 => {
                            trace_tso(queue, p as *mut StgTSO);
                            current_block = 2413388577390654262;
                        }
                        53 => {
                            let mut stack = p as *mut StgStack;
                            let mut marking: StgWord8 = (*stack).marking;

                            if !(cas_word8(
                                &raw mut (*stack).marking,
                                marking,
                                nonmovingMarkEpoch as StgWord8,
                            ) as c_int
                                != nonmovingMarkEpoch as c_int)
                            {
                                break;
                            }

                            trace_stack(queue, stack);
                            current_block = 2413388577390654262;
                        }
                        51 => {
                            let mut p_idx: StgHalfWord = 0 as StgHalfWord;

                            while p_idx < (*info_0).layout.payload.ptrs {
                                let mut field_2: *mut *mut StgClosure = (&raw mut (*p).payload
                                    as *mut *mut StgClosure_)
                                    .offset(p_idx as isize)
                                    as *mut *mut StgClosure;
                                markQueuePushClosure(queue, *field_2, field_2);
                                p_idx = p_idx.wrapping_add(1);
                            }

                            current_block = 2413388577390654262;
                        }
                        58 => {
                            while (*p).header.info as *mut StgInfoTable
                                == &raw const stg_WHITEHOLE_info as *mut StgInfoTable
                            {
                            }

                            continue;
                        }
                        3 | 6 | 42 | 54 | 63 => {
                            current_block = 2413388577390654262;
                        }
                        64 => {
                            let mut cont = p as *mut StgContinuation;

                            trace_stack_(
                                queue,
                                &raw mut (*cont).stack as StgPtr,
                                (&raw mut (*cont).stack as *mut StgWord)
                                    .offset((*cont).stack_size as isize),
                            );

                            current_block = 2413388577390654262;
                        }
                        _ => {
                            barf(
                                b"mark_closure: unimplemented/strange closure type %d @ %p\0"
                                    as *const u8 as *const c_char,
                                (*info_0).r#type,
                                p,
                            );
                        }
                    }

                    match current_block {
                        4751196792806374320 => {
                            let mut i_2: StgWord = 0 as StgWord;

                            while i_2 < (*info_0).layout.payload.ptrs as StgWord {
                                let mut field_0: *mut *mut StgClosure = (&raw mut (*p).payload
                                    as *mut *mut StgClosure_)
                                    .offset(i_2 as isize)
                                    as *mut *mut StgClosure;
                                markQueuePushClosure(queue, *field_0, field_0);
                                i_2 = i_2.wrapping_add(1);
                            }
                        }
                        _ => {}
                    }

                    bd_flags = block_get_flags(bd);

                    if bd_flags as c_int & BF_LARGE != 0 {
                        if bd_flags as c_int & BF_MARKED == 0 {
                            dbl_link_remove(bd, &raw mut nonmoving_large_objects);
                            dbl_link_onto(bd, &raw mut nonmoving_marked_large_objects);
                            n_nonmoving_large_blocks =
                                n_nonmoving_large_blocks.wrapping_sub((*bd).blocks as memcount);
                            n_nonmoving_marked_large_blocks = n_nonmoving_marked_large_blocks
                                .wrapping_add((*bd).blocks as memcount);
                            block_set_flag(bd, BF_MARKED as uint16_t);
                        }
                    } else if bd_flags as c_int & BF_NONMOVING != 0 {
                        let mut seg_0 = nonmovingGetSegment(p as StgPtr);
                        let mut block_idx_0 = nonmovingGetBlockIdx(p as StgPtr);
                        nonmovingSetMark(seg_0, block_idx_0);

                        nonmoving_segment_live_words = nonmoving_segment_live_words.wrapping_add(
                            (nonmovingSegmentBlockSize(seg_0) as usize)
                                .wrapping_div(size_of::<W_>() as usize)
                                as memcount,
                        );
                    }

                    if p_next.is_null() {
                        break;
                    }

                    p = p_next;
                }
            } else if (*bd).flags as c_int & BF_PINNED != 0 {
                return;
            } else {
                barf(
                    b"Strange closure in nonmoving mark: %p\0" as *const u8 as *const c_char,
                    p,
                );
            }
        }
    }

    if !origin.is_null()
        && (!(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end)
            || block_get_flags(bd) as c_int & BF_NONMOVING != 0)
    {
        if UNTAG_CLOSURE(p0 as *mut StgClosure) != p && *origin == p0 as *mut StgClosure {
            cas(
                origin as StgVolatilePtr,
                p0 as StgWord,
                TAG_CLOSURE(tag, p) as StgWord,
            ) == p0 as StgWord;
        }
    }
}

unsafe fn nonmovingMark(mut budget: *mut MarkBudget, mut queue: *mut MarkQueue) {
    traceConcMarkBegin();

    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
        trace_(b"Starting mark pass\0" as *const u8 as *const c_char as *mut c_char);
    }

    let mut count: uint64_t = 0 as uint64_t;

    loop {
        count = count.wrapping_add(1);

        if *budget == 0 as MarkBudget {
            return;
        } else if *budget != UNLIMITED_MARK_BUDGET as MarkBudget {
            *budget -= 1 as MarkBudget;
        }

        let mut ent = markQueuePop(queue);

        match nonmovingMarkQueueEntryType(&raw mut ent) as c_uint {
            1 => {
                mark_closure(
                    queue,
                    ent.c2rust_unnamed.mark_closure.p,
                    ent.c2rust_unnamed.mark_closure.origin,
                );
            }
            2 => {
                let mut arr = UNTAG_CLOSURE(ent.c2rust_unnamed.mark_array.array as *mut StgClosure)
                    as *const StgMutArrPtrs;

                let mut start: StgWord = ent.c2rust_unnamed.mark_array.start_index;
                let mut end: StgWord = start.wrapping_add(MARK_ARRAY_CHUNK_LENGTH as StgWord);

                if end < (*arr).ptrs {
                    markQueuePushArray(queue, arr, end);
                } else {
                    end = (*arr).ptrs;
                }

                let mut i: StgWord = start;

                while i < end {
                    let mut c =
                        *(&raw const (*arr).payload as *const *mut StgClosure).offset(i as isize);
                    markQueuePushClosure_(queue, c);
                    i = i.wrapping_add(1);
                }
            }
            0 => {
                if !upd_rem_set_block_list.is_null() {
                    let mut old = (*queue).blocks;
                    (*queue).blocks = upd_rem_set_block_list;
                    (*queue).top = (*(*queue).blocks).start as *mut MarkQueueBlock;
                    upd_rem_set_block_list = null_mut::<bdescr>();
                    freeGroup(old);
                } else {
                    if DEBUG_RTS != 0 && RtsFlags.DebugFlags.nonmoving_gc as c_long != 0 {
                        trace_(
                            b"Finished mark pass: %d\0" as *const u8 as *const c_char
                                as *mut c_char,
                            count,
                        );
                    }

                    traceConcMarkEnd(count as StgWord32);
                    return;
                }
            }
            _ => {}
        }
    }
}

unsafe fn nonmovingIsAlive(mut p: *mut StgClosure) -> bool {
    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return r#true != 0;
    }

    let mut bd = Bdescr(p as StgPtr);
    let mut bd_flags = block_get_flags(bd);

    if bd_flags as c_int & (BF_COMPACT | BF_LARGE) != 0 {
        if bd_flags as c_int & BF_COMPACT != 0 {
            let mut str = objectGetCompact(p);
            bd = Bdescr(str as StgPtr);
        }

        return bd_flags as c_int & BF_NONMOVING_SWEEPING == 0 as c_int
            || bd_flags as c_int & BF_MARKED != 0 as c_int;
    } else {
        let mut seg = nonmovingGetSegment(p as StgPtr);
        let mut i = nonmovingGetBlockIdx(p as StgPtr);
        let mut mark = nonmovingGetMark(seg, i);

        if i as c_int >= (*nonmovingSegmentInfo(seg)).next_free_snap as c_int {
            return mark as c_int == nonmovingMarkEpoch as c_int || mark as c_int == 0 as c_int;
        } else {
            return mark as c_int == nonmovingMarkEpoch as c_int;
        }
    };
}

unsafe fn nonmovingIsNowAlive(mut p: *mut StgClosure) -> bool {
    if !(p as W_ >= mblock_address_space.0.begin && (p as W_) < mblock_address_space.0.end) {
        return r#true != 0;
    }

    let mut bd = Bdescr(p as StgPtr);
    let flags = block_get_flags(bd) as uint16_t;

    if flags as c_int & BF_LARGE != 0 {
        if flags as c_int & BF_PINNED != 0 && flags as c_int & BF_NONMOVING == 0 {
            return r#true != 0;
        }

        return flags as c_int & BF_NONMOVING_SWEEPING == 0 as c_int
            || flags as c_int & BF_MARKED != 0 as c_int;
    } else {
        let mut seg = nonmovingGetSegment(p as StgPtr);

        let mut snapshot_loc = nonmovingSegmentGetBlock(
            seg,
            (*nonmovingSegmentInfo(seg)).next_free_snap as nonmoving_block_idx,
        ) as *mut StgClosure;

        if p >= snapshot_loc && nonmovingGetClosureMark(p as StgPtr) as c_int == 0 as c_int {
            return r#true != 0;
        } else {
            return nonmovingClosureMarkedThisCycle(p as StgPtr);
        }
    };
}

unsafe fn nonmovingMarkWeakPtrList(mut queue: *mut MarkQueue_) {
    let mut w = nonmoving_old_weak_ptr_list;

    while !w.is_null() {
        mark_closure(
            queue as *mut MarkQueue,
            w as *mut StgClosure,
            null_mut::<*mut StgClosure>(),
        );

        w = (*w).link as *mut StgWeak;
    }
}

unsafe fn is_nonmoving_weak(mut weak: *mut StgWeak) -> bool {
    let mut w = nonmoving_old_weak_ptr_list;

    while !w.is_null() {
        if w == weak {
            return r#true != 0;
        }

        w = (*w).link as *mut StgWeak;
    }

    let mut w_0 = nonmoving_weak_ptr_list;

    while !w_0.is_null() {
        if w_0 == weak {
            return r#true != 0;
        }

        w_0 = (*w_0).link as *mut StgWeak;
    }

    return r#false != 0;
}

unsafe fn nonmovingTidyWeaks(mut queue: *mut MarkQueue_) -> bool {
    let mut did_work = r#false != 0;
    let mut last_w: *mut *mut StgWeak = &raw mut nonmoving_old_weak_ptr_list;
    let mut next_w = null_mut::<StgWeak>();
    let mut w = nonmoving_old_weak_ptr_list;

    while !w.is_null() {
        if (*w).header.info == &raw const stg_DEAD_WEAK_info {
            next_w = (*w).link as *mut StgWeak;
            *last_w = next_w;
        } else {
            let mut key_bd = Bdescr((*w).key as StgPtr);
            let mut key_in_nonmoving = (*w).key as W_ >= mblock_address_space.0.begin
                && ((*w).key as W_) < mblock_address_space.0.end
                && block_get_flags(key_bd) as c_int & BF_NONMOVING != 0;

            if !key_in_nonmoving || nonmovingIsNowAlive((*w).key) as c_int != 0 {
                nonmovingMarkLiveWeak(queue, w);
                did_work = r#true != 0;
                *last_w = (*w).link as *mut StgWeak;
                next_w = (*w).link as *mut StgWeak;
                (*w).link = nonmoving_weak_ptr_list as *mut _StgWeak;
                nonmoving_weak_ptr_list = w;
            } else {
                last_w = &raw mut (*w).link as *mut *mut StgWeak;
                next_w = (*w).link as *mut StgWeak;
            }
        }

        w = next_w;
    }

    return did_work;
}

unsafe fn nonmovingMarkDeadWeak(mut queue: *mut MarkQueue_, mut w: *mut StgWeak) {
    if (*w).cfinalizers != &raw mut stg_NO_FINALIZER_closure {
        markQueuePushClosure_(queue as *mut MarkQueue, (*w).value);
    }

    markQueuePushClosure_(queue as *mut MarkQueue, (*w).finalizer);
}

unsafe fn nonmovingMarkLiveWeak(mut queue: *mut MarkQueue_, mut w: *mut StgWeak) {
    markQueuePushClosure_(queue as *mut MarkQueue, (*w).value);
    markQueuePushClosure_(queue as *mut MarkQueue, (*w).finalizer);
    markQueuePushClosure_(queue as *mut MarkQueue, (*w).cfinalizers);
}

unsafe fn nonmovingMarkDeadWeaks(mut queue: *mut MarkQueue_, mut dead_weaks: *mut *mut StgWeak) {
    let mut next_w = null_mut::<StgWeak>();
    let mut w = nonmoving_old_weak_ptr_list;

    while !w.is_null() {
        nonmovingMarkDeadWeak(queue, w);
        next_w = (*w).link as *mut StgWeak;
        (*w).link = *dead_weaks as *mut _StgWeak;
        *dead_weaks = w;
        w = next_w;
    }
}

unsafe fn nonmovingTidyThreads() {
    let mut next = null_mut::<StgTSO>();
    let mut prev: *mut *mut StgTSO = &raw mut nonmoving_old_threads;
    let mut t = nonmoving_old_threads;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        next = (*t).global_link as *mut StgTSO;

        if nonmovingIsNowAlive(t as *mut StgClosure) {
            *prev = next;
            (*t).global_link = nonmoving_threads as *mut StgTSO_;
            nonmoving_threads = t;
        } else {
            prev = &raw mut (*t).global_link as *mut *mut StgTSO;
        }

        t = next;
    }
}

unsafe fn nonmovingResurrectThreads(
    mut queue: *mut MarkQueue_,
    mut resurrected_threads: *mut *mut StgTSO,
) {
    let mut next = null_mut::<StgTSO>();
    let mut t = nonmoving_old_threads;

    while t != &raw mut stg_END_TSO_QUEUE_closure as *mut c_void as *mut StgTSO {
        next = (*t).global_link as *mut StgTSO;

        match (*t).what_next as c_int {
            ThreadKilled | ThreadComplete => {}
            _ => {
                markQueuePushClosure_(queue as *mut MarkQueue, t as *mut StgClosure);
                (*t).global_link = *resurrected_threads as *mut StgTSO_;
                *resurrected_threads = t;
            }
        }

        t = next;
    }
}
