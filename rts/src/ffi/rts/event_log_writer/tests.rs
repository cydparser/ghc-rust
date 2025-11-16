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

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_startEventLogging() {
    let expected: bool = {
        let mut writer: sys::EventLogWriter = todo!();
        unsafe { sys::startEventLogging(&raw mut writer) }
    };
    let actual: bool = {
        let mut writer: EventLogWriter = todo!();
        unsafe { startEventLogging(&raw mut writer) }
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_startEventLogging() {
    let actual: bool = {
        let writer: EventLogWriter = todo!();
        unsafe { startEventLogging(&raw mut writer) }
    };
    let expected: bool = todo!();
    assert_eq!(expected, actual);
}

#[cfg(all(feature = "ghc_testsuite", feature = "sys"))]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_endEventLogging() {
    let expected = {
        unsafe { sys::endEventLogging() };
        todo!()
    };
    let actual = {
        unsafe { endEventLogging() };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[cfg(feature = "ghc_testsuite")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_endEventLogging() {
    let actual = {
        unsafe { endEventLogging() };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_flushEventLog() {
    let expected = {
        let mut cap: sys::Capability = todo!();
        let mut cap = &raw mut cap;
        unsafe { sys::flushEventLog(&raw mut cap) };
        todo!()
    };
    let actual = {
        let mut cap: Capability = todo!();
        let mut cap = &raw mut cap;
        unsafe { flushEventLog(&raw mut cap) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_flushEventLog() {
    let actual = {
        let mut cap: Capability = todo!();
        let mut cap = &raw mut cap;
        unsafe { flushEventLog(&raw mut cap) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}
