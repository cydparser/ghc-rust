pub use crate::posix::get_time::getProcessElapsedTime;
use crate::prelude::*;
pub use crate::win32::get_time::getProcessElapsedTime;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: u32 = 1000000000;

pub(crate) const TIME_MAX: u64 = 9223372036854775807;

#[ffi(compiler, ghc_lib, libraries)]
pub type Time = i64;
