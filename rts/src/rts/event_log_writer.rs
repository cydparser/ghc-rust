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
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn startEventLogging(writer: *const EventLogWriter) -> bool {
    #[cfg(feature = "sys")]
    unsafe {
        sys::startEventLogging(writer as *const sys::EventLogWriter)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("startEventLogging")
}

#[cfg(feature = "ghc_testsuite")]
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn endEventLogging() {
    #[cfg(feature = "sys")]
    unsafe {
        sys::endEventLogging()
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("endEventLogging")
}

/// - GHC_PLACES: {libraries, testsuite}
#[ffi]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn flushEventLog(cap: *mut *mut Capability) {
    #[cfg(feature = "sys")]
    unsafe {
        sys::flushEventLog(cap as *mut *mut sys::Capability)
    }
    #[cfg(not(feature = "sys"))]
    unimplemented!("flushEventLog")
}
