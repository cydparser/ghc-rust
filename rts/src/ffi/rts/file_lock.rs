use crate::ffi::stg::types::StgWord64;
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn lockFile(
    id: StgWord64,
    dev: StgWord64,
    ino: StgWord64,
    for_writing: c_int,
) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::lockFile(id, dev, ino, for_writing)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("lockFile")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unlockFile(id: StgWord64) -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::unlockFile(id)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("unlockFile")
}
