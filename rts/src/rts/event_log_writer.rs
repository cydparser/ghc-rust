use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventLogWriter {
    pub initEventLogWriter: Option<unsafe extern "C" fn()>,
    pub writeEventLog:
        Option<unsafe extern "C" fn(eventlog: *mut c_void, eventlog_size: usize) -> bool>,
    pub flushEventLog: Option<unsafe extern "C" fn()>,
    pub stopEventLogWriter: Option<unsafe extern "C" fn()>,
}

#[cfg(feature = "sys")]
impl From<EventLogWriter> for sys::EventLogWriter {
    fn from(x: EventLogWriter) -> Self {
        unsafe { transmute(x) }
    }
}

#[cfg(test)]
impl Arbitrary for EventLogWriter {
    fn arbitrary(g: &mut Gen) -> Self {
        EventLogWriter {
            initEventLogWriter: Arbitrary::arbitrary(g),
            writeEventLog: Arbitrary::arbitrary(g),
            flushEventLog: Arbitrary::arbitrary(g),
            stopEventLogWriter: Arbitrary::arbitrary(g),
        }
    }
}

static FileEventLogWriter: EventLogWriter = 0;

static NullEventLogWriter: EventLogWriter = 0;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum EventLogStatus {
    EVENTLOG_NOT_SUPPORTED = 0,
    EVENTLOG_NOT_CONFIGURED = 1,
    EVENTLOG_RUNNING = 2,
}

#[cfg_attr(feature = "tracing", instrument)]
pub(crate) unsafe fn eventLogStatus() -> EventLogStatus {
    unsafe { transmute(sys::eventLogStatus()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    unsafe { transmute(sys::startEventLogging(writer as *const sys::EventLogWriter)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_endEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn endEventLogging() {
    unsafe { sys::endEventLogging() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_flushEventLog"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    unsafe { sys::flushEventLog(cap as *mut *mut sys::Capability) }
}
