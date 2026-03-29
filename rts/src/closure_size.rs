use crate::ffi::rts::storage::closure_macros::{
    THUNK_SELECTOR_sizeW, ap_sizeW, ap_stack_sizeW, arr_words_sizeW, bco_sizeW, continuation_sizeW,
    mut_arr_ptrs_sizeW, pap_sizeW, sizeW_fromITBL, small_mut_arr_ptrs_sizeW, stack_sizeW,
    thunk_sizeW_fromITBL,
};
use crate::ffi::rts::storage::closures::{
    StgAP, StgAP_STACK, StgArrBytes, StgBCO, StgContinuation, StgMutArrPtrs, StgPAP,
    StgSmallMutArrPtrs,
};
use crate::ffi::rts::storage::tso::StgStack;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::prelude::*;

unsafe fn closure_sizeW_(mut p: *const StgClosure, mut info: *const StgInfoTable) -> u32 {
    match (*info).r#type {
        17 | 16 => {
            return (size_of::<StgThunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(1 as usize) as u32;
        }
        10 | 3 | 9 | 2 => {
            return (size_of::<StgHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(1 as usize) as u32;
        }
        20 | 19 | 18 => {
            return (size_of::<StgThunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(2 as usize) as u32;
        }
        13 | 6 | 12 | 5 | 11 | 4 => {
            return (size_of::<StgHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(2 as usize) as u32;
        }
        15 => return thunk_sizeW_fromITBL(info) as u32,
        22 => return THUNK_SELECTOR_sizeW() as u32,
        26 => return ap_stack_sizeW(p as *mut StgAP_STACK) as u32,
        24 => return ap_sizeW(p as *mut StgAP) as u32,
        25 => return pap_sizeW(p as *mut StgPAP) as u32,
        27 => {
            return (size_of::<StgInd>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32;
        }
        42 => return arr_words_sizeW(p as *mut StgArrBytes) as u32,
        43 | 44 | 46 | 45 => {
            return mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as u32;
        }
        59 | 60 | 62 | 61 => {
            return small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as u32;
        }
        52 => {
            return (size_of::<StgTSO>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32;
        }
        53 => return stack_sizeW(p as *mut StgStack) as u32,
        23 => return bco_sizeW(p as *mut StgBCO) as u32,
        54 => {
            return (size_of::<StgTRecChunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as u32;
        }
        64 => return continuation_sizeW(p as *mut StgContinuation) as u32,
        _ => return sizeW_fromITBL(info) as u32,
    };
}
