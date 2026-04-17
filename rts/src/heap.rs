use crate::alloc_array::allocateMutArrPtrs;
use crate::capability::Capability;
use crate::ffi::rts::_assertFail;
use crate::ffi::rts::constants::BITMAP_BITS_SHIFT;
use crate::ffi::rts::messages::barf;
use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::closure_macros::{
    LOOKS_LIKE_CLOSURE_PTR, SET_INFO, UNTAG_CLOSURE, closure_sizeW, get_fun_itbl, get_itbl,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgBCO, StgBlockingQueue, StgClosure_, StgInd, StgMVar, StgMutArrPtrs,
    StgMutVar, StgPAP, StgSelector, StgSmallMutArrPtrs, StgThunk, StgWeak,
};
use crate::ffi::rts::storage::info_tables::{StgLargeBitmap, stg_arg_bitmaps};
use crate::ffi::rts::types::{StgClosure, StgTSO};
use crate::ffi::stg::misc_closures::stg_MUT_ARR_PTRS_FROZEN_CLEAN_info;
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;
use crate::printer::closure_type_names;
use crate::rts_utils::{stgFree, stgMallocBytes};

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn heap_view_closureSize(mut closure: *mut StgClosure) -> StgWord {
    if LOOKS_LIKE_CLOSURE_PTR(closure as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Heap.c".as_ptr(), 19);
    }

    return closure_sizeW(closure) as StgWord;
}

unsafe fn heap_view_closure_ptrs_in_large_bitmap(
    mut ptrs: *mut *mut StgClosure,
    mut nptrs: *mut StgWord,
    mut p: *mut *mut StgClosure,
    mut large_bitmap: *mut StgLargeBitmap,
    mut size: u32,
) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut b: u32 = 0;
    let mut bitmap: StgWord = 0;
    b = 0;
    i = 0;

    while i < size {
        bitmap = *(&raw mut (*large_bitmap).bitmap as *mut StgWord).offset(b as isize);

        j = ({
            let mut _a: u32 = (size as u32).wrapping_sub(i as u32);
            let mut _b: u32 = (8 as usize).wrapping_mul(size_of::<W_>() as usize) as u32;

            if _a <= _b { _a } else { _b as u32 }
        });

        i = i.wrapping_add(j);

        while j > 0 {
            if bitmap & 1 == 0 {
                let fresh68 = *nptrs;
                *nptrs = (*nptrs).wrapping_add(1);

                let ref mut fresh69 = *ptrs.offset(fresh68 as isize);
                *fresh69 = *p;
            }

            bitmap = bitmap >> 1;
            j = j.wrapping_sub(1);
            p = p.offset(1);
        }

        b = b.wrapping_add(1);
    }
}

unsafe fn heap_view_closure_ptrs_in_pap_payload(
    mut ptrs: *mut *mut StgClosure,
    mut nptrs: *mut StgWord,
    mut fun: *mut StgClosure,
    mut payload: *mut *mut StgClosure,
    mut size: StgWord,
) {
    let mut bitmap: StgWord = 0;
    let mut fun_info = null::<StgFunInfoTable>();
    fun_info = get_fun_itbl(UNTAG_CLOSURE(fun));

    let mut p = payload;
    let mut current_block_12: u64;

    match (*fun_info).f.fun_type {
        0 => {
            bitmap = (*fun_info).f.b.bitmap >> BITMAP_BITS_SHIFT;
            current_block_12 = 17322574907599274005;
        }
        1 => {
            heap_view_closure_ptrs_in_large_bitmap(
                ptrs,
                nptrs,
                payload,
                (fun_info.offset(1 as i32 as isize) as StgWord)
                    .wrapping_add((*fun_info).f.b.bitmap_offset as StgWord)
                    as *mut StgLargeBitmap,
                size as u32,
            );

            current_block_12 = 10048703153582371463;
        }
        2 => {
            heap_view_closure_ptrs_in_large_bitmap(
                ptrs,
                nptrs,
                payload,
                &raw mut (*(fun as *mut StgBCO)).bitmap as *mut StgWord as *mut StgLargeBitmap,
                size as u32,
            );

            current_block_12 = 10048703153582371463;
        }
        _ => {
            bitmap = *(&raw const stg_arg_bitmaps as *const StgWord)
                .offset((*fun_info).f.fun_type as isize)
                >> BITMAP_BITS_SHIFT;
            current_block_12 = 17322574907599274005;
        }
    }

    match current_block_12 {
        17322574907599274005 => {
            while size > 0 {
                if bitmap & 1 == 0 {
                    let fresh66 = *nptrs;
                    *nptrs = (*nptrs).wrapping_add(1);

                    let ref mut fresh67 = *ptrs.offset(fresh66 as isize);
                    *fresh67 = *p;
                }

                bitmap = bitmap >> 1;
                p = p.offset(1);
                size = size.wrapping_sub(1);
            }
        }
        _ => {}
    };
}

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn collect_pointers(
    mut closure: *mut StgClosure,
    mut ptrs: *mut *mut StgClosure,
) -> StgWord {
    let mut end = null_mut::<*mut StgClosure>();
    let mut info = get_itbl(closure);
    let mut nptrs: StgWord = 0;
    let mut i: StgWord = 0;

    match (*info).r#type {
        0 => {
            barf(c"Invalid Object".as_ptr());
        }
        2 | 3 | 4 | 5 | 6 | 1 | 7 | 50 | 8 | 9 | 10 | 12 | 11 | 13 | 14 => {
            end = (&raw mut (*closure).payload as *mut *mut StgClosure_)
                .offset((*info).layout.payload.ptrs as isize)
                as *mut *mut StgClosure;

            let mut ptr = &raw mut (*closure).payload as *mut *mut StgClosure;

            while ptr < end {
                let fresh2 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh3 = *ptrs.offset(fresh2 as isize);
                *fresh3 = *ptr;
                ptr = ptr.offset(1);
            }
        }
        15 | 16 | 17 | 19 | 18 | 20 | 21 => {
            end = (&raw mut (*(closure as *mut StgThunk)).payload as *mut *mut StgClosure_)
                .offset((*info).layout.payload.ptrs as isize)
                as *mut *mut StgClosure;

            let mut ptr_0 = &raw mut (*(closure as *mut StgThunk)).payload as *mut *mut StgClosure;

            while ptr_0 < end {
                let fresh4 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh5 = *ptrs.offset(fresh4 as isize);
                *fresh5 = *ptr_0;
                ptr_0 = ptr_0.offset(1);
            }
        }
        22 => {
            let fresh6 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh7 = *ptrs.offset(fresh6 as isize);
            *fresh7 = (*(closure as *mut StgSelector)).selectee;
        }
        24 => {
            let fresh8 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh9 = *ptrs.offset(fresh8 as isize);
            *fresh9 = (*(closure as *mut StgAP)).fun;

            heap_view_closure_ptrs_in_pap_payload(
                ptrs,
                &raw mut nptrs,
                (*(closure as *mut StgAP)).fun,
                &raw mut (*(closure as *mut StgAP)).payload as *mut *mut StgClosure,
                (*(closure as *mut StgAP)).n_args as StgWord,
            );
        }
        25 => {
            let fresh10 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh11 = *ptrs.offset(fresh10 as isize);
            *fresh11 = (*(closure as *mut StgPAP)).fun;

            heap_view_closure_ptrs_in_pap_payload(
                ptrs,
                &raw mut nptrs,
                (*(closure as *mut StgPAP)).fun,
                &raw mut (*(closure as *mut StgPAP)).payload as *mut *mut StgClosure,
                (*(closure as *mut StgPAP)).n_args as StgWord,
            );
        }
        26 => {
            let fresh12 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh13 = *ptrs.offset(fresh12 as isize);
            *fresh13 = (*(closure as *mut StgAP_STACK)).fun;
        }
        23 => {
            let fresh14 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh15 = *ptrs.offset(fresh14 as isize);
            *fresh15 = (*(closure as *mut StgBCO)).instrs as *mut StgClosure;

            let fresh16 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh17 = *ptrs.offset(fresh16 as isize);
            *fresh17 = (*(closure as *mut StgBCO)).literals as *mut StgClosure;

            let fresh18 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh19 = *ptrs.offset(fresh18 as isize);
            *fresh19 = (*(closure as *mut StgBCO)).ptrs as *mut StgClosure;
        }
        27 | 28 | 38 => {
            let fresh27 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh28 = *ptrs.offset(fresh27 as isize);
            *fresh28 = (&raw mut (*(closure as *mut StgInd)).indirectee).load(Ordering::Acquire);
        }
        43 | 44 | 46 | 45 => {
            i = 0;

            while i < (*(closure as *mut StgMutArrPtrs)).ptrs {
                let fresh22 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh23 = *ptrs.offset(fresh22 as isize);
                *fresh23 = *(&raw mut (*(closure as *mut StgMutArrPtrs)).payload
                    as *mut *mut StgClosure)
                    .offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        59 | 60 | 62 | 61 => {
            i = 0;

            while i < (*(closure as *mut StgSmallMutArrPtrs)).ptrs {
                let fresh24 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh25 = *ptrs.offset(fresh24 as isize);
                *fresh25 = *(&raw mut (*(closure as *mut StgSmallMutArrPtrs)).payload
                    as *mut *mut StgClosure)
                    .offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        47 | 48 => {
            let fresh26 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh27 = *ptrs.offset(fresh26 as isize);
            *fresh27 = (*(closure as *mut StgMutVar)).var;
        }
        40 | 39 => {
            let fresh28 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh29 = *ptrs.offset(fresh28 as isize);
            *fresh29 = (*(closure as *mut StgMVar)).head as *mut StgClosure;

            let fresh30 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh31 = *ptrs.offset(fresh30 as isize);
            *fresh31 = (*(closure as *mut StgMVar)).tail as *mut StgClosure;

            let fresh32 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh33 = *ptrs.offset(fresh32 as isize);
            *fresh33 = (*(closure as *mut StgMVar)).value;
        }
        52 => {
            if !((*(closure as *mut StgTSO))._link as *mut StgClosure).is_null() as i32 as i64 != 0
            {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 210);
            }

            let fresh41 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh42 = *ptrs.offset(fresh41 as isize);
            *fresh42 = (*(closure as *mut StgTSO))._link as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).global_link as *mut StgClosure).is_null() as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 213);
            }

            let fresh43 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh44 = *ptrs.offset(fresh43 as isize);
            *fresh44 = (*(closure as *mut StgTSO)).global_link as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).stackobj as *mut StgClosure).is_null() as i32 as i64
                != 0
            {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 216);
            }

            let fresh45 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh46 = *ptrs.offset(fresh45 as isize);
            *fresh46 = (*(closure as *mut StgTSO)).stackobj as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).trec as *mut StgClosure).is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 219);
            }

            let fresh47 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh48 = *ptrs.offset(fresh47 as isize);
            *fresh48 = (*(closure as *mut StgTSO)).trec as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).blocked_exceptions as *mut StgClosure).is_null() as i32
                as i64
                != 0
            {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 222);
            }

            let fresh49 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh50 = *ptrs.offset(fresh49 as isize);
            *fresh50 = (*(closure as *mut StgTSO)).blocked_exceptions as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).bq as *mut StgClosure).is_null() as i32 as i64 != 0 {
            } else {
                _assertFail(c"rts/Heap.c".as_ptr(), 225);
            }

            let fresh51 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh52 = *ptrs.offset(fresh51 as isize);
            *fresh52 = (*(closure as *mut StgTSO)).bq as *mut StgClosure;

            if !((*(closure as *mut StgTSO)).label as *mut StgClosure).is_null() {
                let fresh46 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh47 = *ptrs.offset(fresh46 as isize);
                *fresh47 = (*(closure as *mut StgTSO)).label as *mut StgClosure;
            }
        }
        49 => {
            let mut w = closure as *mut StgWeak;
            let fresh48 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh49 = *ptrs.offset(fresh48 as isize);
            *fresh49 = (*w).cfinalizers;

            let fresh50 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh51 = *ptrs.offset(fresh50 as isize);
            *fresh51 = (*w).key;

            let fresh52 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh53 = *ptrs.offset(fresh52 as isize);
            *fresh53 = (*w).value;

            let fresh54 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh55 = *ptrs.offset(fresh54 as isize);
            *fresh55 = (*w).finalizer;

            if !(*w).link.is_null() {
                let fresh56 = nptrs;
                nptrs = nptrs.wrapping_add(1);

                let ref mut fresh57 = *ptrs.offset(fresh56 as isize);
                *fresh57 = (*w).link as *mut StgClosure;
            }
        }
        42 | 53 | 64 => {}
        37 => {
            let mut bq = closure as *mut StgBlockingQueue;
            let fresh58 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh59 = *ptrs.offset(fresh58 as isize);
            *fresh59 = (*bq).link as *mut StgClosure;

            let fresh60 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh61 = *ptrs.offset(fresh60 as isize);
            *fresh61 = (*bq).bh;

            let fresh62 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh63 = *ptrs.offset(fresh62 as isize);
            *fresh63 = (*bq).owner as *mut StgClosure;

            let fresh64 = nptrs;
            nptrs = nptrs.wrapping_add(1);

            let ref mut fresh65 = *ptrs.offset(fresh64 as isize);
            *fresh65 = (*bq).queue as *mut StgClosure;
        }
        _ => {
            fprintf(
                __stderrp,
                c"closurePtrs: Cannot handle type %s yet\n".as_ptr(),
                *(&raw mut closure_type_names as *mut *const c_char)
                    .offset((*info).r#type as isize),
            );
        }
    }

    return nptrs;
}

unsafe fn heap_view_closurePtrs(
    mut cap: *mut Capability,
    mut closure: *mut StgClosure,
) -> *mut StgMutArrPtrs {
    if LOOKS_LIKE_CLOSURE_PTR(closure as *const c_void) as i32 as i64 != 0 {
    } else {
        _assertFail(c"rts/Heap.c".as_ptr(), 270);
    }

    let mut size = heap_view_closureSize(closure);

    let mut ptrs = stgMallocBytes(
        (size_of::<*mut StgClosure>() as StgWord).wrapping_mul(size) as usize,
        c"heap_view_closurePtrs".as_ptr(),
    ) as *mut *mut StgClosure;

    let mut nptrs = collect_pointers(closure, ptrs as *mut *mut StgClosure);
    let mut arr = allocateMutArrPtrs(cap, nptrs, (*cap).r.rCCCS as *mut CostCentreStack);

    if !((arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as i32 as i64 != 0) {
        SET_INFO(
            arr as *mut StgClosure,
            &raw const stg_MUT_ARR_PTRS_FROZEN_CLEAN_info,
        );

        let mut i: StgWord = 0;

        while i < nptrs {
            let ref mut fresh1 =
                *(&raw mut (*arr).payload as *mut *mut StgClosure).offset(i as isize);
            *fresh1 = *ptrs.offset(i as isize);
            i = i.wrapping_add(1);
        }
    }

    stgFree(ptrs as *mut c_void);

    return arr;
}
