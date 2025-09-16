use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_size_EventLogWriter() {
    assert_eq!(
        size_of::<sys::EventLogWriter>(),
        size_of::<EventLogWriter>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of EventLogWriter"][size_of::<EventLogWriter>() - 32usize];
    ["Alignment of EventLogWriter"][align_of::<EventLogWriter>() - 8usize];
    ["Offset of field: EventLogWriter::initEventLogWriter"]
        [offset_of!(EventLogWriter, initEventLogWriter) - 0usize];
    ["Offset of field: EventLogWriter::writeEventLog"]
        [offset_of!(EventLogWriter, writeEventLog) - 8usize];
    ["Offset of field: EventLogWriter::flushEventLog"]
        [offset_of!(EventLogWriter, flushEventLog) - 16usize];
    ["Offset of field: EventLogWriter::stopEventLogWriter"]
        [offset_of!(EventLogWriter, stopEventLogWriter) - 24usize];
};

#[test]
#[ignore]
fn test_startEventLogging() {
    let writer = null();
    unsafe { startEventLogging(writer) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_endEventLogging() {
    todo!()
}

#[test]
#[ignore]
fn test_endEventLogging() {
    unsafe { endEventLogging() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
fn equivalent_flushEventLog() {
    todo!()
}

#[test]
#[ignore]
fn test_flushEventLog() {
    let cap = null_mut();
    unsafe { flushEventLog(cap) };
    todo!("assert")
}
