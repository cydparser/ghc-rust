use crate::ffi::rts::prof::ccs::CostCentreStack;
use crate::ffi::rts::storage::closure_macros::{mutArrPtrsCardTableSize, mutArrPtrsCards};
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

    if (arr == null_mut::<c_void>() as *mut StgMutArrPtrs) as c_int as c_long != 0 {
        return null_mut::<StgMutArrPtrs>();
    }

    (*arr).header.info = &raw const stg_MUT_ARR_PTRS_DIRTY_info;
    (*arr).ptrs = nelements;
    (*arr).size = arrsize;

    memset(
        (&raw mut (*arr).payload as *mut *mut StgClosure).offset(nelements as isize)
            as *mut *mut StgClosure as *mut c_void,
        0 as c_int,
        mutArrPtrsCards(nelements as W_) as size_t,
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

    if (arr == null_mut::<c_void>() as *mut StgSmallMutArrPtrs) as c_int as c_long != 0 {
        return null_mut::<StgSmallMutArrPtrs>();
    }

    (*arr).header.info = &raw const stg_SMALL_MUT_ARR_PTRS_DIRTY_info;
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

    if (arr == null_mut::<c_void>() as *mut StgArrBytes) as c_int as c_long != 0 {
        return null_mut::<StgArrBytes>();
    }

    (*arr).header.info = &raw const stg_ARR_WORDS_info;
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

    if (arr == null_mut::<c_void>() as *mut StgArrBytes) as c_int as c_long != 0 {
        return null_mut::<StgArrBytes>();
    }

    (*arr).header.info = &raw const stg_ARR_WORDS_info;
    (*arr).bytes = arrbytes;

    return arr;
}
