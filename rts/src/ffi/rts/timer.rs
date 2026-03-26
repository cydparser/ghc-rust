use crate::prelude::*;
pub use crate::timer::{startTimer, stopTimer};

#[cfg(test)]
mod tests;

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn rtsTimerSignal() -> c_int {
    sys! {
        rtsTimerSignal()
    }
}
