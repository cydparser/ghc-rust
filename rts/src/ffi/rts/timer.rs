use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startTimer() {
    sys! {
        startTimer()
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn stopTimer() {
    sys! {
        stopTimer()
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rtsTimerSignal() -> c_int {
    sys! {
        rtsTimerSignal()
    }
}
