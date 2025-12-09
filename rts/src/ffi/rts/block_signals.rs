use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn blockUserSignals() {
    sys! {
        blockUserSignals()
    }
}

#[ffi(libraries)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn unblockUserSignals() {
    sys! {
        unblockUserSignals()
    }
}
