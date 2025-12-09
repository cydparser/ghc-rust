use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lockFile(
    id: StgWord64,
    dev: StgWord64,
    ino: StgWord64,
    for_writing: c_int,
) -> c_int {
    sys! {
        lockFile(id, dev, ino, for_writing)
    }
}

#[ffi(ghc_lib, libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unlockFile(id: StgWord64) -> c_int {
    sys! {
        unlockFile(id)
    }
}
