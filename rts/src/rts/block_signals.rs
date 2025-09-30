use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_blockUserSignals"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
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
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_unblockUserSignals"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::unblockUserSignals()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("unblockUserSignals")
}
