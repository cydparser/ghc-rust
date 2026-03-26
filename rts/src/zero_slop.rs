use crate::ffi::rts::storage::closure_macros::{
    overwritingClosure, overwritingClosureSize, overwritingMutableClosureOfs,
};
use crate::ffi::rts::types::StgClosure;
use crate::prelude::*;

unsafe fn stg_overwritingClosure(mut p: *mut StgClosure) {
    overwritingClosure(p);
}

unsafe fn stg_overwritingMutableClosureOfs(mut p: *mut StgClosure, mut offset: uint32_t) {
    overwritingMutableClosureOfs(p, offset);
}

unsafe fn stg_overwritingClosureSize(mut p: *mut StgClosure, mut size: uint32_t) {
    overwritingClosureSize(p, size);
}
