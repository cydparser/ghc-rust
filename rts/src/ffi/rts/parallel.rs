use crate::ffi::rts::storage::closures::StgClosure;
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgInt;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn newSpark(reg: *mut StgRegTable, p: *mut StgClosure) -> StgInt {
    sys! {
        newSpark(reg as * mut sys::StgRegTable, p as * mut sys::StgClosure)
    }
}
