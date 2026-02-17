use crate::capability::Capability;
use crate::prelude::*;

#[cfg(test)]
mod tests;

#[ffi(testsuite)]
#[repr(C)]
#[derive(Debug)]
pub struct EventLogWriter {
    pub(crate) initEventLogWriter: Option<unsafe extern "C" fn()>,

    pub(crate) writeEventLog:
        Option<unsafe extern "C" fn(eventlog: *mut c_void, eventlog_size: usize) -> bool>,
    pub(crate) flushEventLog: Option<unsafe extern "C" fn()>,
    pub(crate) stopEventLogWriter: Option<unsafe extern "C" fn()>,
}

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum EventLogStatus {
    EVENTLOG_NOT_SUPPORTED = 0,
    EVENTLOG_NOT_CONFIGURED = 1,
    EVENTLOG_RUNNING = 2,
}

#[cfg(feature = "sys")]
impl From<EventLogStatus> for sys::EventLogStatus {
    fn from(v: EventLogStatus) -> Self {
        use EventLogStatus::*;

        match v {
            EVENTLOG_NOT_SUPPORTED => sys::EventLogStatus::EVENTLOG_NOT_SUPPORTED,
            EVENTLOG_NOT_CONFIGURED => sys::EventLogStatus::EVENTLOG_NOT_CONFIGURED,
            EVENTLOG_RUNNING => sys::EventLogStatus::EVENTLOG_RUNNING,
        }
    }
}

#[cfg(feature = "sys")]
impl From<sys::EventLogStatus> for EventLogStatus {
    fn from(v: sys::EventLogStatus) -> Self {
        use EventLogStatus::*;

        match v {
            sys::EventLogStatus::EVENTLOG_NOT_SUPPORTED => EVENTLOG_NOT_SUPPORTED,
            sys::EventLogStatus::EVENTLOG_NOT_CONFIGURED => EVENTLOG_NOT_CONFIGURED,
            sys::EventLogStatus::EVENTLOG_RUNNING => EVENTLOG_RUNNING,
        }
    }
}

impl TryFrom<u32> for EventLogStatus {
    type Error = ();
    fn try_from(d: u32) -> Result<EventLogStatus, ()> {
        use EventLogStatus::*;

        match d {
            0 => Ok(EVENTLOG_NOT_SUPPORTED),
            1 => Ok(EVENTLOG_NOT_CONFIGURED),
            2 => Ok(EVENTLOG_RUNNING),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
impl Arbitrary for EventLogStatus {
    fn arbitrary(g: &mut Gen) -> Self {
        use EventLogStatus::*;

        match usize::arbitrary(g) % 3 {
            0 => EVENTLOG_NOT_SUPPORTED,
            1 => EVENTLOG_NOT_CONFIGURED,
            2.. => EVENTLOG_RUNNING,
        }
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    sys! {
        startEventLogging(writer as * const sys::EventLogWriter)
    }
}

#[ffi(testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn endEventLogging() {
    sys! {
        endEventLogging()
    }
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    sys! {
        flushEventLog(cap as * mut * mut sys::Capability)
    }
}
