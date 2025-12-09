use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getMonotonicNSec() -> StgWord64 {
    sys! {
        getMonotonicNSec()
    }
}
