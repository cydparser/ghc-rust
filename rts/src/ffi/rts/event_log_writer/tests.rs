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

#[cfg(feature = "sys")]
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

    assert_eq!(actual, expected);
}

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

#[cfg(feature = "sys")]
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

    assert_eq!(actual, expected);
}

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

    assert_eq!(actual, expected);
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
