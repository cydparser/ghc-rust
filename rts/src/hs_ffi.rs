use crate::ffi::hs_ffi::{HsFunPtr, HsStablePtr};
use crate::ffi::rts::adjustor::freeHaskellFunctionPtr;
use crate::ffi::rts::storage::gc::performBlockingMajorGC;
use crate::ffi::rts_api::setProgArgv;
use crate::prelude::*;
use crate::stable_ptr::{freeStablePtr, freeStablePtrUnsafe, stablePtrLock, stablePtrUnlock};
use crate::task::freeMyTask;

#[cfg(test)]
mod tests;

#[ffi(docs, testsuite)]
pub type HsFunPtr = Option<unsafe extern "C" fn() -> ()>;

#[ffi(docs, ghc_lib, testsuite)]
pub type HsStablePtr = *mut c_void;

unsafe fn hs_set_argv(mut argc: i32, mut argv: *mut *mut c_char) {
    setProgArgv(argc, argv);
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_perform_gc() {
    performBlockingMajorGC();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_ptr_table() {
    stablePtrLock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_lock_stable_tables() {
    stablePtrLock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_ptr_table() {
    stablePtrUnlock();
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_unlock_stable_tables() {
    stablePtrUnlock();
}

#[ffi(docs, ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr(mut sp: HsStablePtr) {
    freeStablePtr(sp);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_stable_ptr_unsafe(mut sp: HsStablePtr) {
    freeStablePtrUnsafe(sp);
}

#[ffi(docs)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_free_fun_ptr(mut fp: HsFunPtr) {
    freeHaskellFunctionPtr(transmute::<HsFunPtr, *mut c_void>(fp));
}

#[ffi(docs, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn hs_thread_done() {
    freeMyTask();
}
