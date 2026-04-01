use crate::check_unload::markObjectCode;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::storage::block::{BF_EVACUATED, BF_LARGE, BF_MARKED, BF_NONMOVING, Bdescr};
use crate::ffi::rts::storage::closure_macros::{
    GET_CLOSURE_TAG, INFO_PTR_TO_STRUCT, LOOKS_LIKE_CLOSURE_PTR, SET_INFO, TAG_CLOSURE,
    UNTAG_CLOSURE,
};
use crate::ffi::rts::storage::closures::{StgInd, StgIndStatic};
use crate::ffi::rts::storage::heap_alloc::mblock_address_space;
use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::W_;
use crate::ffi::stg::types::{StgPtr, StgWord};
use crate::prelude::*;
use crate::sm::compact::is_marked;
use crate::sm::gc::{evac_fn, unload_mark_needed};
use crate::sm::storage::{
    END_OF_CAF_LIST, STATIC_BITS, STATIC_FLAG_LIST, dyn_caf_list, revertible_caf_list,
};

#[cfg(test)]
mod tests;

unsafe fn isAlive(mut p: *mut StgClosure) -> *mut StgClosure {
    let mut info = null::<StgInfoTable>();
    let mut bd = null_mut::<bdescr>();
    let mut tag: StgWord = 0;
    let mut q = null_mut::<StgClosure>();

    loop {
        tag = GET_CLOSURE_TAG(p);
        q = UNTAG_CLOSURE(p);

        if LOOKS_LIKE_CLOSURE_PTR(q as *const c_void) as i32 as i64 != 0 {
        } else {
            _assertFail(c"rts/sm/GCAux.c".as_ptr(), 45);
        }

        if !(q as W_ >= mblock_address_space.0.begin && (q as W_) < mblock_address_space.0.end) {
            return p;
        }

        bd = Bdescr(q as StgPtr);

        if (*bd).flags as i32 & BF_NONMOVING != 0 {
            return p;
        }

        if (*bd).flags as i32 & BF_EVACUATED != 0 {
            return p;
        }

        if (*bd).flags as i32 & BF_LARGE != 0 {
            return null_mut::<StgClosure>();
        }

        if (*bd).flags as i32 & BF_MARKED != 0 && is_marked(q as StgPtr, bd) != 0 {
            return p;
        }

        info = (&raw mut (*q).header.info).load(Ordering::Relaxed);

        if info as StgWord & 1 != 0 {
            return TAG_CLOSURE(
                tag,
                (info as StgWord).wrapping_sub(1 as StgWord) as *mut StgClosure,
            );
        }

        ::std::sync::atomic::fence(::std::sync::atomic::Ordering::Acquire);
        info = INFO_PTR_TO_STRUCT(info);

        match (*info).r#type {
            27 | 28 => {
                p = (*(q as *mut StgInd)).indirectee;
            }
            38 => {
                p = (*(q as *mut StgInd)).indirectee;

                if GET_CLOSURE_TAG(p) != 0 {
                    continue;
                }

                return null_mut::<StgClosure>();
            }
            _ => return null_mut::<StgClosure>(),
        }
    }
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn revertCAFs() {
    let mut c = revertible_caf_list;

    while c != END_OF_CAF_LIST as *mut StgIndStatic {
        c = (c as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure as *mut StgIndStatic;

        let mut next = (*c).static_link as *mut StgIndStatic;
        SET_INFO(c as *mut StgClosure, (*c).saved_info);
        (*c).saved_info = null::<StgInfoTable>();
        (*c).static_link = null_mut::<StgClosure>();
        c = next;
    }

    revertible_caf_list = END_OF_CAF_LIST as *mut StgIndStatic;
}

unsafe fn markCAFs(mut evac: evac_fn, mut user: *mut c_void) {
    let mut c = dyn_caf_list;

    while c as StgWord | STATIC_FLAG_LIST as StgWord != END_OF_CAF_LIST as StgWord {
        c = (c as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure as *mut StgIndStatic;
        evac.expect("non-null function pointer")(user, &raw mut (*c).indirectee);

        if unload_mark_needed {
            markObjectCode(c as *const c_void);
        }

        c = (*c).static_link as *mut StgIndStatic;
    }

    let mut c_0 = revertible_caf_list;

    while c_0 as StgWord | STATIC_FLAG_LIST as StgWord != END_OF_CAF_LIST as StgWord {
        c_0 = (c_0 as StgWord & !STATIC_BITS as StgWord) as *mut StgClosure as *mut StgIndStatic;
        evac.expect("non-null function pointer")(user, &raw mut (*c_0).indirectee);

        if unload_mark_needed {
            markObjectCode(c_0 as *const c_void);
        }

        c_0 = (*c_0).static_link as *mut StgIndStatic;
    }
}
