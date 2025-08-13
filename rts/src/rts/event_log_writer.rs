use crate::prelude::*;
use crate::rts::capability::Capability;

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

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) enum EventLogStatus {
    EVENTLOG_NOT_SUPPORTED = 0,
    EVENTLOG_NOT_CONFIGURED = 1,
    EVENTLOG_RUNNING = 2,
}

#[instrument]
pub(crate) unsafe fn eventLogStatus() -> EventLogStatus {
    unsafe { transmute(sys::eventLogStatus()) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_startEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    unsafe { transmute(sys::startEventLogging(writer as *const sys::EventLogWriter)) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_endEventLogging"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn endEventLogging() {
    unsafe { sys::endEventLogging() }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_flushEventLog"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[instrument]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    unsafe { sys::flushEventLog(cap as *mut *mut sys::Capability) }
}
