cfg_select! {
    unix => {
        pub use crate::posix::get_time::getProcessElapsedTime;
    }
    windows => {
        pub use crate::win32::get_time::getProcessElapsedTime;
    }
    target_family = "wasm" => {
        pub use crate::wasm::get_time::getProcessElapsedTime;
    }
}
use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: Time = 1_000_000_000;

#[ffi(compiler, ghc_lib, libraries)]
pub type Time = i64;

/// TODO(rust): Use newtype wrappers for time types.
pub(crate) type s = i64;
pub(crate) type ms = i64;
pub(crate) type us = i64;
pub(crate) type ns = i64;

#[doc(alias = "USToTime")]
macro_rules! us_to_time {
    ($expr:expr) => {
        (t as Time) * 1000 as Time
    };
}

pub(crate) use us_to_time;

#[inline]
pub(crate) const fn TimeToMS(t: Time) -> ms {
    t / 1_000_000
}

#[inline]
pub(crate) const fn TimeToUS(t: Time) -> us {
    t / 1_000
}

#[inline]
pub(crate) const fn TimeToNS(t: Time) -> ns {
    t
}

#[inline]
pub(crate) const fn MSToTime(ms: ms) -> Time {
    ms * 1_000_000
}

#[inline]
pub(crate) const fn USToTime(us: us) -> Time {
    us * 1_000
}

#[inline]
pub(crate) const fn NSToTime(ns: ns) -> Time {
    ns
}

#[inline]
pub(crate) const fn SecondsToTime(s: s) -> Time {
    s * TIME_RESOLUTION
}

#[inline]
pub(crate) const fn TimeToSeconds(time: Time) -> s {
    time / TIME_RESOLUTION
}

#[inline]
pub(crate) const fn TimeToSecondsDbl(time: Time) -> f64 {
    time as f64 / TIME_RESOLUTION as f64
}

/// Use instead of SecondsToTime() when we have a floating-point
/// seconds value, to avoid truncating it.
pub(crate) fn fsecondsToTime(t: f64) -> Time {
    (t * TIME_RESOLUTION as f64) as Time
}
