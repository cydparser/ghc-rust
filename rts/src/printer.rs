use crate::ffi::rts::messages::debugBelch;
use crate::ffi::rts::storage::closure_macros::get_itbl;
use crate::ffi::rts::types::{StgClosure, StgInfoTable};
use crate::ffi::stg::types::StgPtr;
use crate::prelude::*;

unsafe fn printPtr(mut p: StgPtr) {
    debugBelch(
        b"ptr 0x%p (enable -DDEBUG for more info) \0" as *const u8 as *const c_char,
        p,
    );
}

unsafe fn printObj(mut obj: *mut StgClosure) {
    debugBelch(
        b"obj 0x%p (enable -DDEBUG for more info) \0" as *const u8 as *const c_char,
        obj,
    );
}

static mut closure_type_names: [*const c_char; 66] = [
    b"INVALID_OBJECT\0" as *const u8 as *const c_char,
    b"CONSTR\0" as *const u8 as *const c_char,
    b"CONSTR_1_0\0" as *const u8 as *const c_char,
    b"CONSTR_0_1\0" as *const u8 as *const c_char,
    b"CONSTR_2_0\0" as *const u8 as *const c_char,
    b"CONSTR_1_1\0" as *const u8 as *const c_char,
    b"CONSTR_0_2\0" as *const u8 as *const c_char,
    b"CONSTR_NOCAF\0" as *const u8 as *const c_char,
    b"FUN\0" as *const u8 as *const c_char,
    b"FUN_1_0\0" as *const u8 as *const c_char,
    b"FUN_0_1\0" as *const u8 as *const c_char,
    b"FUN_2_0\0" as *const u8 as *const c_char,
    b"FUN_1_1\0" as *const u8 as *const c_char,
    b"FUN_0_2\0" as *const u8 as *const c_char,
    b"FUN_STATIC\0" as *const u8 as *const c_char,
    b"THUNK\0" as *const u8 as *const c_char,
    b"THUNK_1_0\0" as *const u8 as *const c_char,
    b"THUNK_0_1\0" as *const u8 as *const c_char,
    b"THUNK_2_0\0" as *const u8 as *const c_char,
    b"THUNK_1_1\0" as *const u8 as *const c_char,
    b"THUNK_0_2\0" as *const u8 as *const c_char,
    b"THUNK_STATIC\0" as *const u8 as *const c_char,
    b"THUNK_SELECTOR\0" as *const u8 as *const c_char,
    b"BCO\0" as *const u8 as *const c_char,
    b"AP\0" as *const u8 as *const c_char,
    b"PAP\0" as *const u8 as *const c_char,
    b"AP_STACK\0" as *const u8 as *const c_char,
    b"IND\0" as *const u8 as *const c_char,
    b"IND_STATIC\0" as *const u8 as *const c_char,
    b"RET_BCO\0" as *const u8 as *const c_char,
    b"RET_SMALL\0" as *const u8 as *const c_char,
    b"RET_BIG\0" as *const u8 as *const c_char,
    b"RET_FUN\0" as *const u8 as *const c_char,
    b"UPDATE_FRAME\0" as *const u8 as *const c_char,
    b"CATCH_FRAME\0" as *const u8 as *const c_char,
    b"UNDERFLOW_FRAME\0" as *const u8 as *const c_char,
    b"STOP_FRAME\0" as *const u8 as *const c_char,
    b"BLOCKING_QUEUE\0" as *const u8 as *const c_char,
    b"BLACKHOLE\0" as *const u8 as *const c_char,
    b"MVAR_CLEAN\0" as *const u8 as *const c_char,
    b"MVAR_DIRTY\0" as *const u8 as *const c_char,
    b"TVAR\0" as *const u8 as *const c_char,
    b"ARR_WORDS\0" as *const u8 as *const c_char,
    b"MUT_ARR_PTRS_CLEAN\0" as *const u8 as *const c_char,
    b"MUT_ARR_PTRS_DIRTY\0" as *const u8 as *const c_char,
    b"MUT_ARR_PTRS_FROZEN_DIRTY\0" as *const u8 as *const c_char,
    b"MUT_ARR_PTRS_FROZEN_CLEAN\0" as *const u8 as *const c_char,
    b"MUT_VAR_CLEAN\0" as *const u8 as *const c_char,
    b"MUT_VAR_DIRTY\0" as *const u8 as *const c_char,
    b"WEAK\0" as *const u8 as *const c_char,
    b"PRIM\0" as *const u8 as *const c_char,
    b"MUT_PRIM\0" as *const u8 as *const c_char,
    b"TSO\0" as *const u8 as *const c_char,
    b"STACK\0" as *const u8 as *const c_char,
    b"TREC_CHUNK\0" as *const u8 as *const c_char,
    b"ATOMICALLY_FRAME\0" as *const u8 as *const c_char,
    b"CATCH_RETRY_FRAME\0" as *const u8 as *const c_char,
    b"CATCH_STM_FRAME\0" as *const u8 as *const c_char,
    b"WHITEHOLE\0" as *const u8 as *const c_char,
    b"SMALL_MUT_ARR_PTRS_CLEAN\0" as *const u8 as *const c_char,
    b"SMALL_MUT_ARR_PTRS_DIRTY\0" as *const u8 as *const c_char,
    b"SMALL_MUT_ARR_PTRS_FROZEN_DIRTY\0" as *const u8 as *const c_char,
    b"SMALL_MUT_ARR_PTRS_FROZEN_CLEAN\0" as *const u8 as *const c_char,
    b"COMPACT_NFDATA\0" as *const u8 as *const c_char,
    b"CONTINUATION\0" as *const u8 as *const c_char,
    b"ANN_FRAME\0" as *const u8 as *const c_char,
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
