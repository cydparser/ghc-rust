use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn blockUserSignals() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::blockUserSignals()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("blockUserSignals")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::unblockUserSignals()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("unblockUserSignals")
}
