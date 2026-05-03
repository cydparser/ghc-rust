pub use crate::posix::get_time::getProcessElapsedTime;
use crate::prelude::*;
pub use crate::win32::get_time::getProcessElapsedTime;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: Time = 1000000000;

pub(crate) const TIME_MAX: u64 = 9223372036854775807;

#[ffi(compiler, ghc_lib, libraries)]
pub type Time = i64;

#[doc(alias = "USToTime")]
macro_rules! us_to_time {
    ($expr:expr) => {
        (t as Time) * 1000 as Time
    };
}

pub(crate) use us_to_time;

/// Use instead of SecondsToTime() when we have a floating-point
/// seconds value, to avoid truncating it.
pub(crate) fn fsecondsToTime(t: f64) -> Time {
    (t * TIME_RESOLUTION as f64) as Time
}
