use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::storage::closure_macros::get_itbl;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::types::StgPtr;
use crate::prelude::*;

unsafe fn printPtr(mut p: StgPtr) {
    debugBelch(c"ptr 0x%p (enable -DDEBUG for more info) ".as_ptr(), p);
}

unsafe fn printObj(mut obj: *mut StgClosure) {
    debugBelch(c"obj 0x%p (enable -DDEBUG for more info) ".as_ptr(), obj);
}

static mut closure_type_names: [*const c_char; 66] = [
    c"INVALID_OBJECT".as_ptr(),
    c"CONSTR".as_ptr(),
    c"CONSTR_1_0".as_ptr(),
    c"CONSTR_0_1".as_ptr(),
    c"CONSTR_2_0".as_ptr(),
    c"CONSTR_1_1".as_ptr(),
    c"CONSTR_0_2".as_ptr(),
    c"CONSTR_NOCAF".as_ptr(),
    c"FUN".as_ptr(),
    c"FUN_1_0".as_ptr(),
    c"FUN_0_1".as_ptr(),
    c"FUN_2_0".as_ptr(),
    c"FUN_1_1".as_ptr(),
    c"FUN_0_2".as_ptr(),
    c"FUN_STATIC".as_ptr(),
    c"THUNK".as_ptr(),
    c"THUNK_1_0".as_ptr(),
    c"THUNK_0_1".as_ptr(),
    c"THUNK_2_0".as_ptr(),
    c"THUNK_1_1".as_ptr(),
    c"THUNK_0_2".as_ptr(),
    c"THUNK_STATIC".as_ptr(),
    c"THUNK_SELECTOR".as_ptr(),
    c"BCO".as_ptr(),
    c"AP".as_ptr(),
    c"PAP".as_ptr(),
    c"AP_STACK".as_ptr(),
    c"IND".as_ptr(),
    c"IND_STATIC".as_ptr(),
    c"RET_BCO".as_ptr(),
    c"RET_SMALL".as_ptr(),
    c"RET_BIG".as_ptr(),
    c"RET_FUN".as_ptr(),
    c"UPDATE_FRAME".as_ptr(),
    c"CATCH_FRAME".as_ptr(),
    c"UNDERFLOW_FRAME".as_ptr(),
    c"STOP_FRAME".as_ptr(),
    c"BLOCKING_QUEUE".as_ptr(),
    c"BLACKHOLE".as_ptr(),
    c"MVAR_CLEAN".as_ptr(),
    c"MVAR_DIRTY".as_ptr(),
    c"TVAR".as_ptr(),
    c"ARR_WORDS".as_ptr(),
    c"MUT_ARR_PTRS_CLEAN".as_ptr(),
    c"MUT_ARR_PTRS_DIRTY".as_ptr(),
    c"MUT_ARR_PTRS_FROZEN_DIRTY".as_ptr(),
    c"MUT_ARR_PTRS_FROZEN_CLEAN".as_ptr(),
    c"MUT_VAR_CLEAN".as_ptr(),
    c"MUT_VAR_DIRTY".as_ptr(),
    c"WEAK".as_ptr(),
    c"PRIM".as_ptr(),
    c"MUT_PRIM".as_ptr(),
    c"TSO".as_ptr(),
    c"STACK".as_ptr(),
    c"TREC_CHUNK".as_ptr(),
    c"ATOMICALLY_FRAME".as_ptr(),
    c"CATCH_RETRY_FRAME".as_ptr(),
    c"CATCH_STM_FRAME".as_ptr(),
    c"WHITEHOLE".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_CLEAN".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_DIRTY".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_FROZEN_DIRTY".as_ptr(),
    c"SMALL_MUT_ARR_PTRS_FROZEN_CLEAN".as_ptr(),
    c"COMPACT_NFDATA".as_ptr(),
    c"CONTINUATION".as_ptr(),
    c"ANN_FRAME".as_ptr(),
];

unsafe fn info_type(mut closure: *const StgClosure) -> *const c_char {
    return closure_type_names[(*get_itbl(closure)).r#type as usize];
}

unsafe fn info_type_by_ip(mut ip: *const StgInfoTable) -> *const c_char {
    return closure_type_names[(*ip).r#type as usize];
}

unsafe fn info_hdr_type(mut closure: *const StgClosure, mut res: *mut c_char) {
    strcpy(
        res,
        closure_type_names[(*get_itbl(closure)).r#type as usize],
    );
}
