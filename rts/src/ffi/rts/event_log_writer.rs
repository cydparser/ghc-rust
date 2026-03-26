pub use crate::eventlog::event_log::{endEventLogging, flushEventLog, startEventLogging};
pub use crate::eventlog::event_log_writer::EventLogWriter;
use crate::prelude::*;

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
