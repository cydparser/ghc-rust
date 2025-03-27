use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventLogWriter {
    pub initEventLogWriter: ::core::option::Option<unsafe extern "C" fn()>,
    pub writeEventLog: ::core::option::Option<
        unsafe extern "C" fn(eventlog: *mut ::core::ffi::c_void, eventlog_size: usize) -> bool,
    >,
    pub flushEventLog: ::core::option::Option<unsafe extern "C" fn()>,
    pub stopEventLogWriter: ::core::option::Option<unsafe extern "C" fn()>,
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

static FileEventLogWriter: EventLogWriter = sys::FileEventLogWriter;

static NullEventLogWriter: EventLogWriter = sys::NullEventLogWriter;

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

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    unsafe { transmute(sys::startEventLogging(&writer.into())) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn endEventLogging() {
    unsafe { transmute(sys::endEventLogging()) }
}

#[unsafe(no_mangle)]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    unsafe { transmute(sys::flushEventLog(&mut &mut cap.into())) }
}
