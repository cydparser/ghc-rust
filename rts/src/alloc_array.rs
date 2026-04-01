use crate::ffi::rts::constants::{LDV_SHIFT, LDV_STATE_CREATE};
use crate::ffi::rts::prof::ccs::{CostCentreStack, era, user_era};
use crate::ffi::rts::storage::closure_macros::{
    doingErasProfiling, doingLDVProfiling, doingRetainerProfiling, mutArrPtrsCardTableSize,
    mutArrPtrsCards,
};
use crate::ffi::rts::storage::closures::{StgArrBytes, StgMutArrPtrs, StgSmallMutArrPtrs};
use crate::ffi::rts::storage::gc::{allocateMightFail, allocatePinned};
use crate::ffi::rts::types::StgClosure;
use crate::ffi::rts_api::Capability;
use crate::ffi::stg::W_;
use crate::ffi::stg::misc_closures::{
    stg_ARR_WORDS_info, stg_MUT_ARR_PTRS_DIRTY_info, stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
};
use crate::ffi::stg::types::StgWord;
use crate::prelude::*;

unsafe fn allocateMutArrPtrs(
    mut cap: *mut Capability,
    mut nelements: StgWord,
    mut ccs: *mut CostCentreStack,
) -> *mut StgMutArrPtrs {
    let mut cardsize = mutArrPtrsCardTableSize(nelements as W_) as StgWord;
    let mut arrsize: StgWord = nelements.wrapping_add(cardsize);
    let mut objsize: StgWord = ((size_of::<StgMutArrPtrs>() as usize)
        .wrapping_add(size_of::<W_>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_div(size_of::<W_>() as usize) as StgWord)
        .wrapping_add(arrsize);

    let mut arr = null_mut::<StgMutArrPtrs>();
    arr = allocateMightFail(cap, objsize as W_) as *mut StgMutArrPtrs;

    if (arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as i32 as i64 != 0 {
        return null_mut::<StgMutArrPtrs>();
    }

    let ref mut fresh13 = (*(arr as *mut StgClosure)).header.prof.ccs;
    *fresh13 = ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(arr as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*arr).header.info).store(&raw const stg_MUT_ARR_PTRS_DIRTY_info, Ordering::Relaxed);
    (*arr).ptrs = nelements;
    (*arr).size = arrsize;

    memset(
        (&raw mut (*arr).payload as *mut *mut StgClosure).offset(nelements as isize)
            as *mut *mut StgClosure as *mut c_void,
        0,
        mutArrPtrsCards(nelements as W_) as usize,
    );

    return arr;
}

unsafe fn allocateSmallMutArrPtrs(
    mut cap: *mut Capability,
    mut nelements: StgWord,
    mut ccs: *mut CostCentreStack,
) -> *mut StgSmallMutArrPtrs {
    let mut arrsize: StgWord = nelements;
    let mut objsize: StgWord = ((size_of::<StgSmallMutArrPtrs>() as usize)
        .wrapping_add(size_of::<W_>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_div(size_of::<W_>() as usize) as StgWord)
        .wrapping_add(arrsize);

    let mut arr = null_mut::<StgSmallMutArrPtrs>();
    arr = allocateMightFail(cap, objsize as W_) as *mut StgSmallMutArrPtrs;

    if (arr == null_mut::<c_void>() as *mut StgSmallMutArrPtrs) as i32 as i64 != 0 {
        return null_mut::<StgSmallMutArrPtrs>();
    }

    let ref mut fresh14 = (*(arr as *mut StgClosure)).header.prof.ccs;
    *fresh14 = ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(arr as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*arr).header.info).store(
        &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info,
        Ordering::Relaxed,
    );
    (*arr).ptrs = nelements;

    return arr;
}

unsafe fn allocateArrBytes(
    mut cap: *mut Capability,
    mut arrbytes: StgWord,
    mut ccs: *mut CostCentreStack,
) -> *mut StgArrBytes {
    let mut arrwords: StgWord = arrbytes
        .wrapping_add(size_of::<W_>() as StgWord)
        .wrapping_sub(1 as StgWord)
        .wrapping_div(size_of::<W_>() as StgWord);

    let mut objsize: StgWord = ((size_of::<StgArrBytes>() as usize)
        .wrapping_add(size_of::<W_>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_div(size_of::<W_>() as usize) as StgWord)
        .wrapping_add(arrwords);

    let mut arr = null_mut::<StgArrBytes>();
    arr = allocateMightFail(cap, objsize as W_) as *mut StgArrBytes;

    if (arr == null_mut::<c_void>() as *mut StgArrBytes) as i32 as i64 != 0 {
        return null_mut::<StgArrBytes>();
    }

    let ref mut fresh15 = (*(arr as *mut StgClosure)).header.prof.ccs;
    *fresh15 = ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(arr as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*arr).header.info).store(&raw const stg_ARR_WORDS_info, Ordering::Relaxed);
    (*arr).bytes = arrbytes;

    return arr;
}

unsafe fn allocateArrBytesPinned(
    mut cap: *mut Capability,
    mut arrbytes: StgWord,
    mut alignment: StgWord,
    mut ccs: *mut CostCentreStack,
) -> *mut StgArrBytes {
    if alignment <= size_of::<StgWord>() as StgWord {
        alignment = size_of::<StgWord>() as StgWord;
    }

    let mut arrwords: StgWord = arrbytes
        .wrapping_add(size_of::<W_>() as StgWord)
        .wrapping_sub(1 as StgWord)
        .wrapping_div(size_of::<W_>() as StgWord);

    let mut objsize: StgWord = ((size_of::<StgArrBytes>() as usize)
        .wrapping_add(size_of::<W_>() as usize)
        .wrapping_sub(1 as usize)
        .wrapping_div(size_of::<W_>() as usize) as StgWord)
        .wrapping_add(arrwords);

    let mut alignoff: StgWord = size_of::<StgArrBytes>() as StgWord;
    let mut arr = null_mut::<StgArrBytes>();
    arr = allocatePinned(cap, objsize as W_, alignment as W_, alignoff as W_) as *mut StgArrBytes;

    if (arr == null_mut::<c_void>() as *mut StgArrBytes) as i32 as i64 != 0 {
        return null_mut::<StgArrBytes>();
    }

    let ref mut fresh16 = (*(arr as *mut StgClosure)).header.prof.ccs;
    *fresh16 = ccs;

    if doingLDVProfiling() {
        if doingLDVProfiling() {
            (*(arr as *mut StgClosure)).header.prof.hp.ldvw =
                (era as StgWord) << LDV_SHIFT | LDV_STATE_CREATE as StgWord;
        }
    } else if doingRetainerProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.trav = 0;
    } else if doingErasProfiling() {
        (*(arr as *mut StgClosure)).header.prof.hp.era = user_era;
    }

    (&raw mut (*arr).header.info).store(&raw const stg_ARR_WORDS_info, Ordering::Relaxed);
    (*arr).bytes = arrbytes;

    return arr;
}
