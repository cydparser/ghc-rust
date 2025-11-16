use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startTimer() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::startTimer()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("startTimer")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopTimer() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::stopTimer()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("stopTimer")
}

/// - GHC_PLACES: {libraries}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rtsTimerSignal() -> c_int {
    #[cfg(feature = "sys")]
    unsafe {
        sys::rtsTimerSignal()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("rtsTimerSignal")
}
