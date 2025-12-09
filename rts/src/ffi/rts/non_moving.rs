use crate::ffi::rts::storage::closures::{StgClosure_, StgThunk_};
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::{StgFunPtr, StgWord};
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn updateRemembSetPushClosure_(reg: *mut StgRegTable, p: *mut StgClosure_) {
    sys! {
        updateRemembSetPushClosure_(reg as * mut sys::StgRegTable, p as * mut
        sys::StgClosure_)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn updateRemembSetPushThunk_(reg: *mut StgRegTable, p: *mut StgThunk_) {
    sys! {
        updateRemembSetPushThunk_(reg as * mut sys::StgRegTable, p as * mut
        sys::StgThunk_)
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stg_copyArray_barrier() -> StgFunPtr {
    sys! {
        stg_copyArray_barrier()
    }
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub static mut nonmoving_write_barrier_enabled: StgWord = 0;
