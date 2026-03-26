use crate::ffi::rts::types::StgClosure;
use crate::ffi::stg::regs::StgRegTable;
use crate::ffi::stg::types::StgInt;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(compiler)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn newSpark(mut reg: *mut StgRegTable, mut p: *mut StgClosure) -> StgInt {
    return 1 as StgInt;
}
