use crate::capability::Capability;
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// - GHC_PLACES: {testsuite}
#[repr(C)]
#[derive(Debug)]
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

#[repr(u32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum EventLogStatus {
    EVENTLOG_NOT_SUPPORTED = 0,
    EVENTLOG_NOT_CONFIGURED = 1,
    EVENTLOG_RUNNING = 2,
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

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    unsafe { sys::startEventLogging(writer as *const sys::EventLogWriter) }
}

#[cfg(feature = "ghc_testsuite")]
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_endEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn endEventLogging() {
    unsafe { sys::endEventLogging() }
}

/// - GHC_PLACES: {libraries, testsuite}
#[cfg_attr(feature = "sys", unsafe(export_name = "rust_flushEventLog"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    unsafe { sys::flushEventLog(cap as *mut *mut sys::Capability) }
}
