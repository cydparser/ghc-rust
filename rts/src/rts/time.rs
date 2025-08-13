use crate::prelude::*;

#[cfg(test)]
mod tests;

pub(crate) const TIME_RESOLUTION: u32 = 1000000000;

pub const TIME_MAX: u64 = 9223372036854775807;

pub type Time = i64;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_getProcessElapsedTime"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn getProcessElapsedTime() -> Time {
    unsafe { sys::getProcessElapsedTime() }
}
