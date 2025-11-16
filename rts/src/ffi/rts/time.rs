use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: u32 = 1000000000;

pub(crate) const TIME_MAX: u64 = 9223372036854775807;

/// - GHC_PLACES: {libraries}
pub type Time = i64;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProcessElapsedTime() -> Time {
    #[cfg(feature = "sys")]
    unsafe {
        sys::getProcessElapsedTime()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("getProcessElapsedTime")
}
