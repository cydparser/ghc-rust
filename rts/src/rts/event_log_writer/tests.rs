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

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_eventLogStatus() -> bool {
    let expected = unsafe { transmute(sys::eventLogStatus()) };
    let actual = unsafe { eventLogStatus() };
    actual == expected
}

#[test]
#[ignore]
fn test_eventLogStatus() {
    unsafe { eventLogStatus() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_startEventLogging(_TODO: ()) -> bool {
    let writer = null_mut();
    let expected = unsafe { transmute(sys::startEventLogging(writer as *mut sys::EventLogWriter)) };
    let actual = unsafe { startEventLogging(writer) };
    actual == expected
}

#[test]
#[ignore]
fn test_startEventLogging() {
    let writer = null();
    unsafe { startEventLogging(writer) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_endEventLogging() {
    unsafe { endEventLogging() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_flushEventLog() {
    let mut cap = null_mut();
    unsafe { flushEventLog(&mut cap) };
    todo!("assert")
}
