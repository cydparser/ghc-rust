use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_EventLogWriter_layout() {
    assert_eq!(
        offset_of!(EventLogWriter, initEventLogWriter),
        offset_of!(sys::EventLogWriter, initEventLogWriter)
    );
    assert_eq!(
        offset_of!(EventLogWriter, writeEventLog),
        offset_of!(sys::EventLogWriter, writeEventLog)
    );
    assert_eq!(
        offset_of!(EventLogWriter, flushEventLog),
        offset_of!(sys::EventLogWriter, flushEventLog)
    );
    assert_eq!(
        offset_of!(EventLogWriter, stopEventLogWriter),
        offset_of!(sys::EventLogWriter, stopEventLogWriter)
    );
    assert_eq!(
        size_of::<EventLogWriter>(),
        size_of::<sys::EventLogWriter>()
    );
    assert_eq!(
        align_of::<EventLogWriter>(),
        align_of::<sys::EventLogWriter>()
    );
}
