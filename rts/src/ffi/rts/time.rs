use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: u32 = 1000000000;

pub(crate) const TIME_MAX: u64 = 9223372036854775807;

#[ffi(compiler, ghc_lib, libraries)]
pub type Time = i64;

#[ffi(ghc_lib)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn getProcessElapsedTime() -> Time {
    sys! {
        getProcessElapsedTime()
    }
}
