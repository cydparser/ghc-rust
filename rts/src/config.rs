use crate::ffi::rts::event_log_writer::EventLogWriter;
use crate::ffi::rts::time::Time;
use crate::ffi::stg::W_;
use crate::hs_ffi::HsBool;
use crate::prelude::*;
pub use crate::stats::GCDetails;

#[cfg(test)]
mod tests;

#[ffi(compiler, docs, driver, testsuite, utils)]
#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum RtsOptsEnabledEnum {
    RtsOptsNone = 0,
    RtsOptsIgnore = 1,
    RtsOptsIgnoreAll = 2,
    RtsOptsSafeOnly = 3,
    RtsOptsAll = 4,
}

#[cfg(feature = "sys")]
impl From<RtsOptsEnabledEnum> for sys::RtsOptsEnabledEnum {
    fn from(v: RtsOptsEnabledEnum) -> Self {
        use RtsOptsEnabledEnum::*;

        match v {
            RtsOptsNone => sys::RtsOptsEnabledEnum::RtsOptsNone,
            RtsOptsIgnore => sys::RtsOptsEnabledEnum::RtsOptsIgnore,
            RtsOptsIgnoreAll => sys::RtsOptsEnabledEnum::RtsOptsIgnoreAll,
            RtsOptsSafeOnly => sys::RtsOptsEnabledEnum::RtsOptsSafeOnly,
            RtsOptsAll => sys::RtsOptsEnabledEnum::RtsOptsAll,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::RtsOptsEnabledEnum> for RtsOptsEnabledEnum {
    fn from(v: sys::RtsOptsEnabledEnum) -> Self {
        use RtsOptsEnabledEnum::*;

        match v {
            sys::RtsOptsEnabledEnum::RtsOptsNone => RtsOptsNone,
            sys::RtsOptsEnabledEnum::RtsOptsIgnore => RtsOptsIgnore,
            sys::RtsOptsEnabledEnum::RtsOptsIgnoreAll => RtsOptsIgnoreAll,
            sys::RtsOptsEnabledEnum::RtsOptsSafeOnly => RtsOptsSafeOnly,
            sys::RtsOptsEnabledEnum::RtsOptsAll => RtsOptsAll,
        }
    }
}

impl TryFrom<u32> for RtsOptsEnabledEnum {
    type Error = ();

    fn try_from(d: u32) -> Result<RtsOptsEnabledEnum, ()> {
        use RtsOptsEnabledEnum::*;

        match d {
            0 => Ok(RtsOptsNone),
            1 => Ok(RtsOptsIgnore),
            2 => Ok(RtsOptsIgnoreAll),
            3 => Ok(RtsOptsSafeOnly),
            4 => Ok(RtsOptsAll),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for RtsOptsEnabledEnum {
    fn arbitrary(g: &mut Gen) -> Self {
        use RtsOptsEnabledEnum::*;

        match usize::arbitrary(g) % 5 {
            0 => RtsOptsNone,
            1 => RtsOptsIgnore,
            2 => RtsOptsIgnoreAll,
            3 => RtsOptsSafeOnly,
            4.. => RtsOptsAll,
        }
    }
}

#[ffi(compiler, docs, driver, testsuite, utils)]
#[repr(C)]
pub struct RtsConfig {
    pub rts_opts_enabled: RtsOptsEnabledEnum,
    pub rts_opts_suggestions: HsBool,
    pub rts_opts: *const c_char,
    pub rts_hs_main: HsBool,
    pub keep_cafs: HsBool,
    pub eventlog_writer: *const EventLogWriter,
    pub defaultsHook: Option<unsafe extern "C" fn()>,
    pub onExitHook: Option<unsafe extern "C" fn()>,
    pub stackOverflowHook: Option<unsafe extern "C" fn(stack_size: W_)>,
    pub outOfHeapHook: Option<unsafe extern "C" fn(request_size: W_, heap_size: W_)>,
    pub mallocFailHook: Option<unsafe extern "C" fn(request_size: W_, msg: *const c_char)>,
    pub gcDoneHook: Option<unsafe extern "C" fn(stats: *const GCDetails)>,
    pub longGCSync: Option<unsafe extern "C" fn(this_cap: u32, time_ns: Time)>,
    pub longGCSyncEnd: Option<unsafe extern "C" fn(time_ns: Time)>,
}
