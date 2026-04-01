use crate::capability::getCapability;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::{
    BITMAP_BITS_SHIFT, BITMAP_SIZE_MASK, ThreadComplete, ThreadKilled,
};
use crate::ffi::rts::flags::RtsFlags;
use crate::ffi::rts::messages::{barf, vdebugBelch};
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::block::bdescr;
use crate::ffi::rts::storage::block::{BLOCK_SIZE_W, allocGroup, bdescr, bdescr_, freeChain};
use crate::ffi::rts::storage::closure_macros::{
    STATIC_LINK, UNTAG_CLOSURE, UNTAG_CONST_CLOSURE, get_fun_itbl, get_itbl, get_ret_itbl,
    get_thunk_itbl,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgBCO, StgBlockingQueue, StgClosure_, StgContinuation, StgInd,
    StgIndStatic, StgMVar, StgMutArrPtrs, StgMutVar, StgPAP, StgRetFun, StgSelector,
    StgSmallMutArrPtrs, StgTRecChunk, StgThunk, StgUpdateFrame, StgWeak, TRecEntry,
};
use crate::ffi::rts::storage::info_tables::{
    StgFunInfoTable, StgLargeBitmap, StgSRTField, StgThunkInfoTable, stg_arg_bitmaps,
};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::threads::getNumCapabilities;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::types::{STG_WORD_MAX, StgHalfWord, StgPtr, StgWord};
use crate::ffi::stg::types::{StgHalfWord, StgPtr, StgWord};
use crate::ffi::stg::{BITS_PER_BYTE, P_, W_};
use crate::prelude::*;
use crate::retainer_set::retainer;
use crate::retainer_set::retainer;
use crate::sm::storage::{STATIC_BITS, static_flag};
use crate::traverse_heap::{
    C2RustUnnamed_11, C2RustUnnamed_12, nextPos, nextPosType, posTypeEmpty, posTypeFresh,
    posTypePtrs, posTypeSRT, posTypeStep, stackAccum, stackAccum_, stackData, stackData_,
    stackElement, stackElement_, stackPos, stackPos_, traverseState, traverseState_,
    visitClosure_cb,
};

pub(crate) type nextPosType = u32;

pub(crate) const posTypeEmpty: nextPosType = 4;

pub(crate) const posTypeFresh: nextPosType = 3;

pub(crate) const posTypeSRT: nextPosType = 2;

pub(crate) const posTypePtrs: nextPosType = 1;

pub(crate) const posTypeStep: nextPosType = 0;

pub(crate) union nextPos {
    pub(crate) step: StgWord,
    pub(crate) ptrs: C2RustUnnamed_12,
    pub(crate) srt: C2RustUnnamed_11,
    pub(crate) cp: *mut StgClosure,
}

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_11 {
    pub(crate) srt: *mut StgClosure,
}

/// cbindgen:no-export
pub(crate) struct C2RustUnnamed_12 {
    pub(crate) pos: StgHalfWord,
    pub(crate) ptrs: StgHalfWord,
    pub(crate) payload: StgPtr,
}

/// cbindgen:no-export
pub(crate) struct stackPos_ {
    pub(crate) r#type: nextPosType,
    pub(crate) next: nextPos,
}

pub(crate) type stackPos = stackPos_;

pub(crate) union stackData_ {
    pub(crate) c_child_r: retainer,
}

pub(crate) type stackData = stackData_;

pub(crate) union stackAccum_ {
    pub(crate) subtree_sizeW: StgWord,
}

pub(crate) type stackAccum = stackAccum_;

/// cbindgen:no-export
pub(crate) struct stackElement_ {
    pub(crate) info: stackPos,
    pub(crate) c: *mut StgClosure,
    pub(crate) sep: *mut stackElement_,
    pub(crate) data: stackData,
    pub(crate) accum: stackAccum,
}

pub(crate) type stackElement = stackElement_;

/// cbindgen:no-export
pub(crate) struct traverseState_ {
    pub(crate) flip: StgWord,
    pub(crate) firstStack: *mut bdescr,
    pub(crate) currentStack: *mut bdescr,
    pub(crate) stackBottom: *mut stackElement,
    pub(crate) stackTop: *mut stackElement,
    pub(crate) stackLimit: *mut stackElement,
    pub(crate) stackSize: i32,
    pub(crate) maxStackSize: i32,
    pub(crate) return_cb: Option<
        unsafe extern "C" fn(*mut StgClosure, stackAccum, *mut StgClosure, *mut stackAccum) -> (),
    >,
}

pub(crate) type traverseState = traverseState_;

pub(crate) type visitClosure_cb = Option<
    unsafe extern "C" fn(
        *mut StgClosure,
        *const StgClosure,
        stackData,
        bool,
        *mut stackAccum,
        *mut stackData,
    ) -> bool,
>;

static mut nullStackData: stackData = stackData_ {
    c_child_r: null_mut::<CostCentreStack>(),
};

unsafe fn getTravData(mut c: *const StgClosure) -> StgWord {
    let hp_hdr: StgWord = (*c).header.prof.hp.trav;

    return hp_hdr & (STG_WORD_MAX as StgWord ^ 1);
}

unsafe fn setTravData(mut ts: *const traverseState, mut c: *mut StgClosure, mut w: StgWord) {
    (*c).header.prof.hp.trav = w | (*ts).flip;
}

unsafe fn isTravDataValid(mut ts: *const traverseState, mut c: *const StgClosure) -> bool {
    return (*c).header.prof.hp.trav & 1 == (*ts).flip;
}

static mut g_traversalDebugLevel: u32 = 0;

unsafe fn debug(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;

    if g_traversalDebugLevel == 0 {
        return;
    }

    ap = args.clone();
    vdebugBelch(s, ap.as_va_list());
}

const BLOCKS_IN_STACK: i32 = 1;

unsafe fn newStackBlock(mut ts: *mut traverseState, mut bd: *mut bdescr) {
    (*ts).currentStack = bd;
    (*ts).stackTop = (*bd)
        .start
        .offset(BLOCK_SIZE_W.wrapping_mul((*bd).blocks as usize) as isize)
        as *mut stackElement;
    (*ts).stackBottom = (*bd).start as *mut stackElement;
    (*ts).stackLimit = (*ts).stackTop;
    (*bd).c2rust_unnamed.free = (*ts).stackLimit as StgPtr;
}

unsafe fn returnToOldStack(mut ts: *mut traverseState, mut bd: *mut bdescr) {
    (*ts).currentStack = bd;
    (*ts).stackTop = (*bd).c2rust_unnamed.free as *mut stackElement;
    (*ts).stackBottom = (*bd).start as *mut stackElement;
    (*ts).stackLimit = (*bd)
        .start
        .offset(BLOCK_SIZE_W.wrapping_mul((*bd).blocks as usize) as isize)
        as *mut stackElement;
    (*bd).c2rust_unnamed.free = (*ts).stackLimit as StgPtr;
}

unsafe fn initializeTraverseStack(mut ts: *mut traverseState) {
    if !(*ts).firstStack.is_null() {
        freeChain((*ts).firstStack);
    }

    (*ts).firstStack = allocGroup(BLOCKS_IN_STACK as W_);
    (*(*ts).firstStack).link = null_mut::<bdescr_>();
    (*(*ts).firstStack).u.back = null_mut::<bdescr_>();
    (*ts).stackSize = 0;
    (*ts).maxStackSize = 0;
    newStackBlock(ts, (*ts).firstStack);
}

unsafe fn closeTraverseStack(mut ts: *mut traverseState) {
    freeChain((*ts).firstStack);
    (*ts).firstStack = null_mut::<bdescr>();
}

unsafe fn getTraverseStackMaxSize(mut ts: *mut traverseState) -> i32 {
    return (*ts).maxStackSize;
}

unsafe fn isEmptyWorkStack(mut ts: *mut traverseState) -> bool {
    return (*ts).firstStack == (*ts).currentStack && (*ts).stackTop == (*ts).stackLimit;
}

unsafe fn traverseWorkStackBlocks(mut ts: *mut traverseState) -> W_ {
    let mut bd = null_mut::<bdescr>();
    let mut res: W_ = 0;
    bd = (*ts).firstStack;

    while !bd.is_null() {
        res = res.wrapping_add((*bd).blocks as W_);
        bd = (*bd).link as *mut bdescr;
    }

    return res;
}

unsafe fn init_ptrs(mut info: *mut stackPos, mut ptrs: u32, mut payload: StgPtr) {
    (*info).r#type = posTypePtrs;
    (*info).next.ptrs.pos = 0;
    (*info).next.ptrs.ptrs = ptrs as StgHalfWord;
    (*info).next.ptrs.payload = payload;
}

unsafe fn find_ptrs(mut info: *mut stackPos) -> *mut StgClosure {
    if (*info).next.ptrs.pos < (*info).next.ptrs.ptrs {
        let fresh6 = (*info).next.ptrs.pos;
        (*info).next.ptrs.pos = (*info).next.ptrs.pos.wrapping_add(1);

        return *(*info).next.ptrs.payload.offset(fresh6 as isize) as *mut StgClosure;
    } else {
        return null_mut::<StgClosure>();
    };
}

unsafe fn init_srt_fun(mut info: *mut stackPos, mut infoTable: *const StgFunInfoTable) {
    (*info).r#type = posTypeSRT;

    if (*infoTable).i.srt != 0 {
        (*info).next.srt.srt = (infoTable.offset(1 as i32 as isize) as StgWord)
            .wrapping_add((*infoTable).i.srt as StgWord)
            as *mut StgClosure;
    } else {
        (*info).next.srt.srt = null_mut::<StgClosure>();
    };
}

unsafe fn init_srt_thunk(mut info: *mut stackPos, mut infoTable: *const StgThunkInfoTable) {
    (*info).r#type = posTypeSRT;

    if (*infoTable).i.srt != 0 {
        (*info).next.srt.srt = (infoTable.offset(1 as i32 as isize) as StgWord)
            .wrapping_add((*infoTable).i.srt as StgWord)
            as *mut StgClosure;
    } else {
        (*info).next.srt.srt = null_mut::<StgClosure>();
    };
}

unsafe fn find_srt(mut info: *mut stackPos) -> *mut StgClosure {
    let mut c = null_mut::<StgClosure>();

    if (*info).r#type as u32 == posTypeSRT as i32 as u32 {
        c = (*info).next.srt.srt;
        (*info).next.srt.srt = null_mut::<StgClosure>();

        return c;
    }

    return null_mut::<StgClosure>();
}

unsafe fn pushStackElement(mut ts: *mut traverseState, se: stackElement) -> *mut stackElement {
    let mut nbd = null_mut::<bdescr>();

    if (*ts).stackTop.offset(-1) < (*ts).stackBottom {
        debug(c"pushStackElement() to the next stack.\n".as_ptr());
        (*(*ts).currentStack).c2rust_unnamed.free = (*ts).stackTop as StgPtr;

        if (*(*ts).currentStack).link.is_null() {
            nbd = allocGroup(BLOCKS_IN_STACK as W_);
            (*nbd).link = null_mut::<bdescr_>();
            (*nbd).u.back = (*ts).currentStack as *mut bdescr_;
            (*(*ts).currentStack).link = nbd as *mut bdescr_;
        } else {
            nbd = (*(*ts).currentStack).link as *mut bdescr;
        }

        newStackBlock(ts, nbd);
    }

    (*ts).stackTop = (*ts).stackTop.offset(-1);
    *(*ts).stackTop = se;
    (*ts).stackSize += 1;

    if (*ts).stackSize > (*ts).maxStackSize {
        (*ts).maxStackSize = (*ts).stackSize;
    }

    if ((*ts).stackSize >= 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 259);
    }

    debug(c"stackSize = %d\n".as_ptr(), (*ts).stackSize);

    return (*ts).stackTop;
}

#[inline]
unsafe fn traversePushClosure(
    mut ts: *mut traverseState,
    mut c: *mut StgClosure,
    mut cp: *mut StgClosure,
    mut sep: *mut stackElement,
    mut data: stackData,
) {
    let mut se = stackElement_ {
        info: stackPos_ {
            r#type: posTypeStep,
            next: nextPos { step: 0 },
        },
        c: null_mut::<StgClosure>(),
        sep: null_mut::<stackElement_>(),
        data: stackData_ {
            c_child_r: null_mut::<CostCentreStack>(),
        },
        accum: stackAccum_ { subtree_sizeW: 0 },
    };

    se.c = c;
    se.info.next.cp = cp;
    se.sep = sep as *mut stackElement_;
    se.data = data;
    se.accum = stackAccum_ { subtree_sizeW: 0 };
    se.info.r#type = posTypeFresh;
    pushStackElement(ts, se);
}

unsafe fn traversePushRoot(
    mut ts: *mut traverseState,
    mut c: *mut StgClosure,
    mut cp: *mut StgClosure,
    mut data: stackData,
) {
    traversePushClosure(ts, c, cp, null_mut::<stackElement>(), data);
}

unsafe fn traversePushReturn(
    mut ts: *mut traverseState,
    mut c: *mut StgClosure,
    mut acc: stackAccum,
    mut sep: *mut stackElement,
) -> *mut stackElement {
    if (*ts).return_cb.is_none() {
        return sep;
    }

    let mut se = stackElement_ {
        info: stackPos_ {
            r#type: posTypeStep,
            next: nextPos { step: 0 },
        },
        c: null_mut::<StgClosure>(),
        sep: null_mut::<stackElement_>(),
        data: stackData_ {
            c_child_r: null_mut::<CostCentreStack>(),
        },
        accum: stackAccum_ { subtree_sizeW: 0 },
    };

    se.c = c;
    se.info.next.cp = null_mut::<StgClosure>();
    se.accum = acc;
    se.sep = sep as *mut stackElement_;
    memset(
        &raw mut se.data as *mut c_void,
        0,
        size_of::<stackData>() as usize,
    );
    se.info.r#type = posTypeEmpty;

    return pushStackElement(ts, se);
}

unsafe fn traverseGetChildren(
    mut c: *mut StgClosure,
    mut first_child: *mut *mut StgClosure,
    mut other_children: *mut bool,
    mut se: *mut stackElement,
) {
    if ((*get_itbl(c)).r#type != 52) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 347);
    }

    if ((*get_itbl(c)).r#type != 26) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 348);
    }

    (*se).c = c;
    *other_children = false;

    let mut current_block_68: u64;

    match (*get_itbl(c)).r#type {
        3 | 6 | 42 | 63 => {
            *first_child = null_mut::<StgClosure>();
            return;
        }
        47 | 48 => {
            *first_child = (*(c as *mut StgMutVar)).var;
            return;
        }
        22 => {
            *first_child = (*(c as *mut StgSelector)).selectee;
            return;
        }
        38 => {
            *first_child = (*(c as *mut StgInd)).indirectee;
            return;
        }
        2 | 5 => {
            *first_child =
                *(&raw mut (*c).payload as *mut *mut StgClosure_).offset(0) as *mut StgClosure;
            return;
        }
        4 => {
            *first_child =
                *(&raw mut (*c).payload as *mut *mut StgClosure_).offset(0) as *mut StgClosure;
            (*se).info.r#type = posTypeStep;
            (*se).info.next.step = 2;
            current_block_68 = 8716029205547827362;
        }
        39 | 40 => {
            *first_child = (*(c as *mut StgMVar)).head as *mut StgClosure;
            (*se).info.r#type = posTypeStep;
            (*se).info.next.step = 2;
            current_block_68 = 8716029205547827362;
        }
        49 => {
            *first_child = (*(c as *mut StgWeak)).key;
            (*se).info.r#type = posTypeStep;
            (*se).info.next.step = 2;
            current_block_68 = 8716029205547827362;
        }
        41 | 1 | 7 | 50 | 51 | 23 => {
            init_ptrs(
                &raw mut (*se).info,
                (*get_itbl(c)).layout.payload.ptrs as u32,
                &raw mut (*c).payload as *mut *mut StgClosure_ as StgPtr,
            );

            *first_child = find_ptrs(&raw mut (*se).info);

            if (*first_child).is_null() {
                return;
            }

            current_block_68 = 8716029205547827362;
        }
        43 | 44 | 46 | 45 => {
            init_ptrs(
                &raw mut (*se).info,
                (*(c as *mut StgMutArrPtrs)).ptrs as u32,
                &raw mut (*(c as *mut StgMutArrPtrs)).payload as *mut *mut StgClosure as StgPtr,
            );

            *first_child = find_ptrs(&raw mut (*se).info);

            if (*first_child).is_null() {
                return;
            }

            current_block_68 = 8716029205547827362;
        }
        59 | 60 | 62 | 61 => {
            init_ptrs(
                &raw mut (*se).info,
                (*(c as *mut StgSmallMutArrPtrs)).ptrs as u32,
                &raw mut (*(c as *mut StgSmallMutArrPtrs)).payload as *mut *mut StgClosure
                    as StgPtr,
            );

            *first_child = find_ptrs(&raw mut (*se).info);

            if (*first_child).is_null() {
                return;
            }

            current_block_68 = 8716029205547827362;
        }
        14 | 8 | 11 => {
            init_ptrs(
                &raw mut (*se).info,
                (*get_itbl(c)).layout.payload.ptrs as u32,
                &raw mut (*c).payload as *mut *mut StgClosure_ as StgPtr,
            );

            *first_child = find_ptrs(&raw mut (*se).info);

            if (*first_child).is_null() {
                current_block_68 = 2391886799049156403;
            } else {
                current_block_68 = 8716029205547827362;
            }
        }
        15 | 18 => {
            init_ptrs(
                &raw mut (*se).info,
                (*get_itbl(c)).layout.payload.ptrs as u32,
                &raw mut (*(c as *mut StgThunk)).payload as *mut *mut StgClosure_ as StgPtr,
            );

            *first_child = find_ptrs(&raw mut (*se).info);

            if (*first_child).is_null() {
                current_block_68 = 14093766898566112057;
            } else {
                current_block_68 = 8716029205547827362;
            }
        }
        9 | 12 => {
            *first_child =
                *(&raw mut (*c).payload as *mut *mut StgClosure_).offset(0) as *mut StgClosure;

            if !(*first_child).is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/TraverseHeap.c".as_ptr(), 477);
            }

            init_srt_fun(&raw mut (*se).info, get_fun_itbl(c));
            current_block_68 = 8716029205547827362;
        }
        16 | 19 => {
            *first_child = *(&raw mut (*(c as *mut StgThunk)).payload as *mut *mut StgClosure_)
                .offset(0) as *mut StgClosure;

            if !(*first_child).is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/TraverseHeap.c".as_ptr(), 484);
            }

            init_srt_thunk(&raw mut (*se).info, get_thunk_itbl(c));
            current_block_68 = 8716029205547827362;
        }
        10 | 13 => {
            current_block_68 = 2391886799049156403;
        }
        21 => {
            if ((*get_itbl(c)).srt != 0) as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/TraverseHeap.c".as_ptr(), 499);
            }

            current_block_68 = 14093766898566112057;
        }
        17 | 20 => {
            current_block_68 = 14093766898566112057;
        }
        54 => {
            *first_child = (*(c as *mut StgTRecChunk)).prev_chunk as *mut StgClosure;
            (*se).info.r#type = posTypeStep;
            (*se).info.next.step = 0;
            current_block_68 = 8716029205547827362;
        }

        25 | 24 | 26 | 64 | 52 | 53 | 28 | 33 | 34 | 35 | 36 | 29 | 30 | 31 | 65 | 27 | 0 | _ => {
            barf(
                c"Invalid object *c in push(): %d".as_ptr(),
                (*get_itbl(c)).r#type,
            );
        }
    }

    match current_block_68 {
        14093766898566112057 => {
            init_srt_thunk(&raw mut (*se).info, get_thunk_itbl(c));
            *first_child = find_srt(&raw mut (*se).info);

            if (*first_child).is_null() {
                return;
            }
        }
        2391886799049156403 => {
            init_srt_fun(&raw mut (*se).info, get_fun_itbl(c));
            *first_child = find_srt(&raw mut (*se).info);

            if (*first_child).is_null() {
                return;
            }
        }
        _ => {}
    }

    if ((*se).info.r#type as u32 != posTypeFresh as i32 as u32) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 543);
    }

    *other_children = true;
}

unsafe fn popStackElement(mut ts: *mut traverseState) {
    debug(
        c"popStackElement(): stackTop = 0x%x\n".as_ptr(),
        (*ts).stackTop,
    );

    if ((*ts).stackTop != (*ts).stackLimit) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 552);
    }

    if !isEmptyWorkStack(ts) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 553);
    }

    if (*ts).stackTop.offset(1) < (*ts).stackLimit {
        (*ts).stackTop = (*ts).stackTop.offset(1);
        (*ts).stackSize -= 1;

        if (*ts).stackSize > (*ts).maxStackSize {
            (*ts).maxStackSize = (*ts).stackSize;
        }

        if ((*ts).stackSize >= 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/TraverseHeap.c".as_ptr(), 561);
        }

        debug(c"stackSize = (--) %d\n".as_ptr(), (*ts).stackSize);
        return;
    }

    let mut pbd = null_mut::<bdescr>();
    debug(c"popStackElement() to the previous stack.\n".as_ptr());

    if ((*ts).stackTop.offset(1) == (*ts).stackLimit) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 571);
    }

    if ((*ts).stackBottom == (*(*ts).currentStack).start as *mut stackElement) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 572);
    }

    if (*ts).firstStack == (*ts).currentStack {
        (*ts).stackTop = (*ts).stackTop.offset(1);

        if ((*ts).stackTop == (*ts).stackLimit) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/TraverseHeap.c".as_ptr(), 577);
        }

        (*ts).stackSize -= 1;

        if (*ts).stackSize > (*ts).maxStackSize {
            (*ts).maxStackSize = (*ts).stackSize;
        }

        if ((*ts).stackSize >= 0) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/TraverseHeap.c".as_ptr(), 581);
        }

        debug(c"stackSize = %d\n".as_ptr(), (*ts).stackSize);
        return;
    }

    (*(*ts).currentStack).c2rust_unnamed.free = (*ts).stackLimit as StgPtr;
    pbd = (*(*ts).currentStack).u.back as *mut bdescr;

    if !pbd.is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 593);
    }

    returnToOldStack(ts, pbd);
    (*ts).stackSize -= 1;

    if (*ts).stackSize > (*ts).maxStackSize {
        (*ts).maxStackSize = (*ts).stackSize;
    }

    if ((*ts).stackSize >= 0) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 599);
    }

    debug(c"stackSize = %d\n".as_ptr(), (*ts).stackSize);
}

unsafe fn callReturnAndPopStackElement(mut ts: *mut traverseState) {
    let mut se = (*ts).stackTop;

    if (*ts).return_cb.is_some() {
        (*ts).return_cb.expect("non-null function pointer")(
            (*se).c,
            (*se).accum,
            (*(*se).sep).c,
            &raw mut (*(*se).sep).accum,
        );
    }

    popStackElement(ts);
}

unsafe fn traversePop(
    mut ts: *mut traverseState,
    mut c: *mut *mut StgClosure,
    mut cp: *mut *mut StgClosure,
    mut data: *mut stackData,
    mut sep: *mut *mut stackElement,
) {
    let mut current_block: u64;
    let mut se = null_mut::<stackElement>();
    debug(c"traversePop(): stackTop = 0x%x\n".as_ptr(), (*ts).stackTop);

    let mut last = false;
    *c = null_mut::<StgClosure>();

    loop {
        if isEmptyWorkStack(ts) {
            *c = null_mut::<StgClosure>();
            return;
        }

        se = (*ts).stackTop;
        *sep = (*se).sep as *mut stackElement;

        if (*se).info.r#type as u32 == posTypeFresh as i32 as u32 {
            *cp = (*se).info.next.cp;
            *c = (*se).c;
            *data = (*se).data;
            popStackElement(ts);
            return;
        } else {
            if (*se).info.r#type as u32 == posTypeEmpty as i32 as u32 {
                callReturnAndPopStackElement(ts);
            } else {
                match (*get_itbl((*se).c)).r#type {
                    4 => {
                        *c = *(&raw mut (*(*se).c).payload as *mut *mut StgClosure_).offset(1)
                            as *mut StgClosure;
                        last = true;
                        break;
                    }
                    39 | 40 => {
                        if (*se).info.next.step == 2 {
                            *c = (*((*se).c as *mut StgMVar)).tail as *mut StgClosure;
                            (*se).info.next.step = (*se).info.next.step.wrapping_add(1);
                        } else {
                            *c = (*((*se).c as *mut StgMVar)).value;
                            last = true;
                        }

                        break;
                    }
                    49 => {
                        if (*se).info.next.step == 2 {
                            *c = (*((*se).c as *mut StgWeak)).value;
                            (*se).info.next.step = (*se).info.next.step.wrapping_add(1);
                        } else {
                            *c = (*((*se).c as *mut StgWeak)).finalizer;
                            last = true;
                        }

                        break;
                    }
                    54 => {
                        let mut entry = null_mut::<TRecEntry>();
                        let mut step: StgWord = (*se).info.next.step;
                        let mut entry_no: u32 = (step >> 2) as u32;
                        let mut field_no: u32 = (step & 3) as u32;
                        entry = (&raw mut (*((*se).c as *mut StgTRecChunk)).entries
                            as *mut TRecEntry)
                            .offset(entry_no as isize)
                            as *mut TRecEntry;

                        if field_no == 0 {
                            *c = (*entry).tvar as *mut StgClosure;
                        } else if field_no == 1 {
                            *c = (*entry).expected_value;
                        } else {
                            *c = (*entry).new_value;
                        }

                        step = step.wrapping_add(1);
                        (*se).info.next.step = step;
                        entry_no = (step >> 2) as u32;

                        if !(entry_no as StgWord
                            == (*((*se).c as *mut StgTRecChunk)).next_entry_idx)
                        {
                            break;
                        }

                        (*se).info.r#type = posTypeEmpty;
                        current_block = 8258075665625361029;
                    }
                    41 | 1 | 50 | 51 | 23 | 43 | 44 | 46 | 45 | 59 | 60 | 62 | 61 => {
                        *c = find_ptrs(&raw mut (*se).info);

                        if !(*c).is_null() {
                            break;
                        }

                        (*se).info.r#type = posTypeEmpty;
                        current_block = 8258075665625361029;
                    }
                    8 | 14 | 11 => {
                        if (*se).info.r#type as u32 == posTypePtrs as i32 as u32 {
                            *c = find_ptrs(&raw mut (*se).info);

                            if !(*c).is_null() {
                                break;
                            }

                            init_srt_fun(&raw mut (*se).info, get_fun_itbl((*se).c));
                            current_block = 9963957861488720293;
                        } else {
                            current_block = 9963957861488720293;
                        }
                    }
                    15 | 18 => {
                        if (*se).info.r#type as u32 == posTypePtrs as i32 as u32 {
                            *c = find_ptrs(&raw mut (*se).info);

                            if !(*c).is_null() {
                                break;
                            }

                            init_srt_thunk(&raw mut (*se).info, get_thunk_itbl((*se).c));
                            current_block = 9963957861488720293;
                        } else {
                            current_block = 9963957861488720293;
                        }
                    }
                    21 | 10 | 13 | 17 | 20 | 9 | 12 | 16 | 19 => {
                        current_block = 9963957861488720293;
                    }

                    3 | 6 | 42 | 47 | 48 | 22 | 5 | 25 | 24 | 26 | 64 | 52 | 53 | 28 | 7 | 33
                    | 34 | 35 | 36 | 29 | 30 | 31 | 65 | 27 | 0 | _ => {
                        barf(
                            c"Invalid object *c in traversePop(): %d".as_ptr(),
                            (*get_itbl((*se).c)).r#type,
                        );
                    }
                }

                match current_block {
                    8258075665625361029 => {}
                    _ => {
                        *c = find_srt(&raw mut (*se).info);

                        if !(*c).is_null() {
                            break;
                        }

                        (*se).info.r#type = posTypeEmpty;
                    }
                }
            }

            if !(*c).is_null() {
                break;
            }
        }
    }

    if !(*c).is_null() as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 848);
    }

    *cp = (*se).c;
    *data = (*se).data;
    *sep = se;

    if last as i32 != 0 && (*ts).return_cb.is_some() {
        (*se).info.r#type = posTypeEmpty;
    } else if last {
        popStackElement(ts);
    }
}

unsafe fn traverseMaybeInitClosureData(
    mut ts: *const traverseState,
    mut c: *mut StgClosure,
) -> bool {
    if !isTravDataValid(ts, c) {
        setTravData(ts, c, 0);

        return true;
    }

    return false;
}

unsafe fn traverseLargeBitmap(
    mut ts: *mut traverseState,
    mut p: StgPtr,
    mut large_bitmap: *mut StgLargeBitmap,
    mut size: u32,
    mut c: *mut StgClosure,
    mut sep: *mut stackElement,
    mut data: stackData,
) {
    let mut i: u32 = 0;
    let mut b: u32 = 0;
    let mut bitmap: StgWord = 0;
    b = 0;
    bitmap = *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(b as isize);
    i = 0;

    while i < size {
        if bitmap & 1 == 0 {
            traversePushClosure(ts, *p as *mut StgClosure, c, sep, data);
        }

        i = i.wrapping_add(1);
        p = p.offset(1);

        if (i as usize)
            .wrapping_rem((BITS_PER_BYTE as usize).wrapping_mul(size_of::<W_>() as usize))
            == 0
        {
            b = b.wrapping_add(1);
            bitmap = *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(b as isize);
        } else {
            bitmap = bitmap >> 1;
        }
    }
}

unsafe fn traverseSmallBitmap(
    mut ts: *mut traverseState,
    mut p: StgPtr,
    mut size: u32,
    mut bitmap: StgWord,
    mut c: *mut StgClosure,
    mut sep: *mut stackElement,
    mut data: stackData,
) -> StgPtr {
    while size > 0 {
        if bitmap & 1 == 0 {
            traversePushClosure(ts, *p as *mut StgClosure, c, sep, data);
        }

        p = p.offset(1);
        bitmap = bitmap >> 1;
        size = size.wrapping_sub(1);
    }

    return p;
}

unsafe fn traversePushStack(
    mut ts: *mut traverseState,
    mut cp: *mut StgClosure,
    mut sep: *mut stackElement,
    mut data: stackData,
    mut stackStart: StgPtr,
    mut stackEnd: StgPtr,
) {
    let mut p = null_mut::<StgWord>();
    let mut info = null::<StgRetInfoTable>();
    let mut bitmap: StgWord = 0;
    let mut size: u32 = 0;

    if ((*get_itbl(cp)).r#type == 53) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 950);
    }

    p = stackStart;

    while p < stackEnd {
        info = get_ret_itbl(p as *mut StgClosure);

        match (*info).i.r#type {
            33 => {
                traversePushClosure(ts, (*(p as *mut StgUpdateFrame)).updatee, cp, sep, data);

                p = p.offset(
                    (size_of::<StgUpdateFrame>() as usize)
                        .wrapping_add(size_of::<W_>() as usize)
                        .wrapping_sub(1 as usize)
                        .wrapping_div(size_of::<W_>() as usize) as isize,
                );

                continue;
            }
            35 | 36 | 34 | 57 | 56 | 55 | 30 | 65 => {
                bitmap = (*info).i.layout.bitmap >> BITMAP_BITS_SHIFT;
                size = ((*info).i.layout.bitmap & BITMAP_SIZE_MASK as StgWord) as u32;
                p = p.offset(1);
                p = traverseSmallBitmap(ts, p, size, bitmap, cp, sep, data);
            }
            29 => {
                let mut bco = null_mut::<StgBCO>();
                p = p.offset(1);
                traversePushClosure(ts, *p as *mut StgClosure, cp, sep, data);
                bco = *p as *mut StgBCO;
                p = p.offset(1);
                size =
                    (*(&raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap)).size as u32;

                traverseLargeBitmap(
                    ts,
                    p,
                    &raw mut (*bco).bitmap as *mut StgWord as *mut StgLargeBitmap,
                    size,
                    cp,
                    sep,
                    data,
                );

                p = p.offset(size as isize);
                continue;
            }
            31 => {
                size = (*(((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                    as *mut StgLargeBitmap))
                    .size as u32;
                p = p.offset(1);

                traverseLargeBitmap(
                    ts,
                    p,
                    ((&raw const (*info).i).offset(1 as i32 as isize) as StgWord)
                        .wrapping_add((*info).i.layout.large_bitmap_offset as StgWord)
                        as *mut StgLargeBitmap,
                    size,
                    cp,
                    sep,
                    data,
                );

                p = p.offset(size as isize);
            }
            32 => {
                let mut ret_fun = p as *mut StgRetFun;
                let mut fun_info = null::<StgFunInfoTable>();
                traversePushClosure(ts, (*ret_fun).fun, cp, sep, data);
                fun_info = get_fun_itbl(UNTAG_CONST_CLOSURE((*ret_fun).fun));
                p = &raw mut (*ret_fun).payload as P_ as StgPtr;

                match (*fun_info).f.fun_type {
                    0 => {
                        bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
                        size = ((*fun_info).f.b.bitmap & BITMAP_SIZE_MASK as StgWord) as u32;
                        p = traverseSmallBitmap(ts, p, size, bitmap, cp, sep, data);
                    }
                    1 => {
                        size = (*((fun_info.offset(1 as i32 as isize) as StgWord)
                            .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                            as *mut StgLargeBitmap))
                            .size as u32;

                        traverseLargeBitmap(
                            ts,
                            p,
                            (fun_info.offset(1 as i32 as isize) as StgWord)
                                .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                                as *mut StgLargeBitmap,
                            size,
                            cp,
                            sep,
                            data,
                        );

                        p = p.offset(size as isize);
                    }
                    _ => {
                        bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                            .offset((*fun_info).f.fun_type as isize)
                            >> BITMAP_BITS_SHIFT;

                        size = (*(&raw const stg_arg_bitmaps as *const StgWord)
                            .offset((*fun_info).f.fun_type as isize)
                            & BITMAP_SIZE_MASK as StgWord) as u32;
                        p = traverseSmallBitmap(ts, p, size, bitmap, cp, sep, data);
                    }
                }
            }
            _ => {
                barf(
                    c"Invalid object found in traversePushStack(): %d".as_ptr(),
                    (*info).i.r#type as i32,
                );
            }
        }

        if (*info).i.srt != 0 {
            traversePushClosure(
                ts,
                (info.offset(1 as i32 as isize) as StgWord).wrapping_add((*info).i.srt as StgWord)
                    as *mut StgClosure,
                cp,
                sep,
                data,
            );
        }
    }
}

unsafe fn traversePAP(
    mut ts: *mut traverseState,
    mut pap: *mut StgClosure,
    mut sep: *mut stackElement,
    mut data: stackData,
    mut fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut n_args: StgWord,
) -> StgPtr {
    let mut p = null_mut::<StgWord>();
    let mut bitmap: StgWord = 0;
    let mut fun_info = null::<StgFunInfoTable>();
    traversePushClosure(ts, fun, pap, sep, data);
    fun = UNTAG_CLOSURE(fun);
    fun_info = get_fun_itbl(fun);

    if ((*fun_info).i.r#type != 25) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/TraverseHeap.c".as_ptr(), 1059);
    }

    p = payload as StgPtr;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            p = traverseSmallBitmap(ts, p, n_args as u32, bitmap, pap, sep, data);
        }
        1 => {
            traverseLargeBitmap(
                ts,
                p,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                n_args as u32,
                pap,
                sep,
                data,
            );

            p = p.offset(n_args as isize);
        }
        2 => {
            traverseLargeBitmap(
                ts,
                payload as StgPtr,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                n_args as u32,
                pap,
                sep,
                data,
            );

            p = p.offset(n_args as isize);
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;
            p = traverseSmallBitmap(ts, p, n_args as u32, bitmap, pap, sep, data);
        }
    }

    return p;
}

unsafe fn resetMutableObjects(mut ts: *mut traverseState) {
    let mut g: u32 = 0;
    let mut n: u32 = 0;
    let mut bd = null_mut::<bdescr>();
    let mut ml = null_mut::<StgWord>();
    g = 0;

    while g < RtsFlags.GcFlags.generations {
        n = 0;

        while n < getNumCapabilities() as u32 {
            bd = *(*getCapability(n)).mut_lists.offset(g as isize);

            while !bd.is_null() {
                ml = (*bd).start;

                while ml < (*bd).c2rust_unnamed.free {
                    traverseMaybeInitClosureData(ts, *ml as *mut StgClosure);
                    ml = ml.offset(1);
                }

                bd = (*bd).link as *mut bdescr;
            }

            n = n.wrapping_add(1);
        }

        g = g.wrapping_add(1);
    }
}

unsafe fn traverseWorkStack(mut ts: *mut traverseState, mut visit_cb: visitClosure_cb) {
    let mut current_block: u64;
    let mut c = null_mut::<StgClosure>();
    let mut cp = null_mut::<StgClosure>();
    let mut first_child = null_mut::<StgClosure>();

    let mut data = stackData_ {
        c_child_r: null_mut::<CostCentreStack>(),
    };

    let mut child_data = stackData_ {
        c_child_r: null_mut::<CostCentreStack>(),
    };

    let mut typeOfc: StgWord = 0;
    let mut sep = null_mut::<stackElement>();
    let mut other_children: bool = false;

    loop {
        traversePop(ts, &raw mut c, &raw mut cp, &raw mut data, &raw mut sep);

        if c.is_null() {
            debug(c"maxStackSize= %d\n".as_ptr(), (*ts).maxStackSize);
            return;
        }

        loop {
            c = UNTAG_CLOSURE(c);
            typeOfc = (*get_itbl(c)).r#type as StgWord;

            match typeOfc {
                52 => {
                    if (*(c as *mut StgTSO)).what_next as i32 == ThreadComplete
                        || (*(c as *mut StgTSO)).what_next as i32 == ThreadKilled
                    {
                        debug(
                            c"ThreadComplete or ThreadKilled encountered in traverseWorkStack()\n"
                                .as_ptr(),
                        );

                        break;
                    } else {
                        current_block = 6057473163062296781;
                    }
                }
                28 => {
                    c = (*(c as *mut StgIndStatic)).indirectee;
                    continue;
                }
                7 => {
                    break;
                }
                21 => {
                    if (*get_itbl(c)).srt == 0 {
                        break;
                    } else {
                        current_block = 12349973810996921269;
                    }
                }
                14 => {
                    current_block = 12349973810996921269;
                }
                _ => {
                    current_block = 6057473163062296781;
                }
            }

            match current_block {
                12349973810996921269 => {
                    let mut info = get_itbl(c);

                    if (*info).srt == 0 && (*info).layout.payload.ptrs == 0 {
                        break;
                    }
                }
                _ => {}
            }

            let mut accum = stackAccum_ { subtree_sizeW: 0 };
            let mut first_visit = traverseMaybeInitClosureData(ts, c);
            let mut traverse_children = first_visit;

            if visit_cb.is_some() {
                traverse_children = visit_cb.expect("non-null function pointer")(
                    c,
                    cp,
                    data,
                    first_visit,
                    &raw mut accum,
                    &raw mut child_data,
                );
            }

            if !traverse_children {
                break;
            }

            match typeOfc {
                53 => {
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePushStack(
                        ts,
                        c,
                        sep,
                        child_data,
                        (*(c as *mut StgStack)).sp,
                        (&raw mut (*(c as *mut StgStack)).stack as *mut StgWord)
                            .offset((*(c as *mut StgStack)).stack_size as isize),
                    );

                    break;
                }
                52 => {
                    let mut tso = c as *mut StgTSO;
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePushClosure(ts, (*tso).stackobj as *mut StgClosure, c, sep, child_data);

                    traversePushClosure(
                        ts,
                        (*tso).blocked_exceptions as *mut StgClosure,
                        c,
                        sep,
                        child_data,
                    );

                    traversePushClosure(ts, (*tso).bq as *mut StgClosure, c, sep, child_data);

                    traversePushClosure(ts, (*tso).trec as *mut StgClosure, c, sep, child_data);

                    match (&raw mut (*tso).why_blocked).load(Ordering::Acquire) {
                        1 | 14 | 2 | 12 => {
                            traversePushClosure(ts, (*tso).block_info.closure, c, sep, child_data);
                        }
                        _ => {}
                    }

                    break;
                }
                37 => {
                    let mut bq = c as *mut StgBlockingQueue;
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePushClosure(ts, (*bq).link as *mut StgClosure, c, sep, child_data);

                    traversePushClosure(ts, (*bq).bh, c, sep, child_data);

                    traversePushClosure(ts, (*bq).owner as *mut StgClosure, c, sep, child_data);

                    break;
                }
                25 => {
                    let mut pap = c as *mut StgPAP;
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePAP(
                        ts,
                        c,
                        sep,
                        child_data,
                        (*pap).fun,
                        &raw mut (*pap).payload as *mut *mut StgClosure,
                        (*pap).n_args as StgWord,
                    );

                    break;
                }
                24 => {
                    let mut ap = c as *mut StgAP;
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePAP(
                        ts,
                        c,
                        sep,
                        child_data,
                        (*ap).fun,
                        &raw mut (*ap).payload as *mut *mut StgClosure,
                        (*ap).n_args as StgWord,
                    );

                    break;
                }
                26 => {
                    sep = traversePushReturn(ts, c, accum, sep);

                    traversePushClosure(ts, (*(c as *mut StgAP_STACK)).fun, c, sep, child_data);

                    traversePushStack(
                        ts,
                        c,
                        sep,
                        child_data,
                        &raw mut (*(c as *mut StgAP_STACK)).payload as *mut *mut StgClosure
                            as StgPtr,
                        (&raw mut (*(c as *mut StgAP_STACK)).payload as *mut *mut StgClosure
                            as StgPtr)
                            .offset((*(c as *mut StgAP_STACK)).size as isize),
                    );

                    break;
                }
                64 => {
                    let mut cont = c as *mut StgContinuation;

                    traversePushStack(
                        ts,
                        c,
                        sep,
                        child_data,
                        &raw mut (*cont).stack as StgPtr,
                        (&raw mut (*cont).stack as *mut StgWord)
                            .offset((*cont).stack_size as isize),
                    );

                    break;
                }
                _ => {
                    let mut se = stackElement_ {
                        info: stackPos_ {
                            r#type: posTypeStep,
                            next: nextPos { step: 0 },
                        },
                        c: null_mut::<StgClosure>(),
                        sep: null_mut::<stackElement_>(),
                        data: stackData_ {
                            c_child_r: null_mut::<CostCentreStack>(),
                        },
                        accum: stackAccum_ { subtree_sizeW: 0 },
                    };

                    traverseGetChildren(
                        c,
                        &raw mut first_child,
                        &raw mut other_children,
                        &raw mut se,
                    );

                    if first_child.is_null() && (*ts).return_cb.is_some() {
                        if ((*sep).c == cp) as i32 as i64 != 0 {
                        } else {
                            _assertFail(c"rts/TraverseHeap.c".as_ptr(), 1319);
                        }

                        (*ts).return_cb.expect("non-null function pointer")(
                            c,
                            accum,
                            cp,
                            &raw mut (*sep).accum,
                        );

                        break;
                    } else {
                        if first_child.is_null() {
                            break;
                        }

                        if !other_children {
                            sep = traversePushReturn(ts, c, accum, sep);
                        } else {
                            se.sep = sep as *mut stackElement_;
                            se.data = child_data;
                            se.accum = accum;
                            sep = pushStackElement(ts, se);
                        }

                        data = child_data;
                        cp = c;
                        c = first_child;
                    }
                }
            }
        }
    }
}

unsafe fn traverseInvalidateClosureData(mut ts: *mut traverseState) {
    resetMutableObjects(ts);
    (*ts).flip = (*ts).flip ^ 1;
}

unsafe fn resetStaticObjectForProfiling(
    mut ts: *const traverseState,
    mut static_objects: *mut StgClosure,
) {
    let mut count: u32 = 0;
    let mut p = null_mut::<StgClosure>();
    p = static_objects;

    while p != static_flag as StgWord as *mut StgClosure {
        p = (p as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure;
        count = count.wrapping_add(1);

        match (*get_itbl(p)).r#type {
            28 => {
                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            21 => {
                traverseMaybeInitClosureData(ts, p);
                p = *(&raw mut (*p).payload as *mut *mut StgClosure_).offset(1) as *mut StgClosure;
            }
            14 | 1 | 2 | 4 | 5 | 7 => {
                traverseMaybeInitClosureData(ts, p);
                p = *STATIC_LINK(get_itbl(p), p);
            }
            _ => {
                barf(
                    c"resetStaticObjectForProfiling: %p (%lu)".as_ptr(),
                    p,
                    (*get_itbl(p)).r#type as u64,
                );
            }
        }
    }

    debug(c"count in scavenged_static_objects = %d\n".as_ptr(), count);
}
