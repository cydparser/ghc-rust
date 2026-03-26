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

unsafe fn closure_sizeW_(mut p: *const StgClosure, mut info: *const StgInfoTable) -> uint32_t {
    match (*info).r#type {
        17 | 16 => {
            return (size_of::<StgThunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(1 as usize) as uint32_t;
        }
        10 | 3 | 9 | 2 => {
            return (size_of::<StgHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(1 as usize) as uint32_t;
        }
        20 | 19 | 18 => {
            return (size_of::<StgThunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(2 as usize) as uint32_t;
        }
        13 | 6 | 12 | 5 | 11 | 4 => {
            return (size_of::<StgHeader>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize)
                .wrapping_add(2 as usize) as uint32_t;
        }
        15 => return thunk_sizeW_fromITBL(info) as uint32_t,
        22 => return THUNK_SELECTOR_sizeW() as uint32_t,
        26 => return ap_stack_sizeW(p as *mut StgAP_STACK) as uint32_t,
        24 => return ap_sizeW(p as *mut StgAP) as uint32_t,
        25 => return pap_sizeW(p as *mut StgPAP) as uint32_t,
        27 => {
            return (size_of::<StgInd>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as uint32_t;
        }
        42 => return arr_words_sizeW(p as *mut StgArrBytes) as uint32_t,
        43 | 44 | 46 | 45 => {
            return mut_arr_ptrs_sizeW(p as *mut StgMutArrPtrs) as uint32_t;
        }
        59 | 60 | 62 | 61 => {
            return small_mut_arr_ptrs_sizeW(p as *mut StgSmallMutArrPtrs) as uint32_t;
        }
        52 => {
            return (size_of::<StgTSO>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as uint32_t;
        }
        53 => return stack_sizeW(p as *mut StgStack) as uint32_t,
        23 => return bco_sizeW(p as *mut StgBCO) as uint32_t,
        54 => {
            return (size_of::<StgTRecChunk>() as usize)
                .wrapping_add(size_of::<W_>() as usize)
                .wrapping_sub(1 as usize)
                .wrapping_div(size_of::<W_>() as usize) as uint32_t;
        }
        64 => return continuation_sizeW(p as *mut StgContinuation) as uint32_t,
        _ => return sizeW_fromITBL(info) as uint32_t,
    };
}
