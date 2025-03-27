use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_size_of_EventLogWriter() {
    assert_eq!(
        size_of::<sys::EventLogWriter>(),
        size_of::<super::EventLogWriter>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of EventLogWriter"][::core::mem::size_of::<EventLogWriter>() - 32usize];
    ["Alignment of EventLogWriter"][::core::mem::align_of::<EventLogWriter>() - 8usize];
    ["Offset of field: EventLogWriter::initEventLogWriter"]
        [::core::mem::offset_of!(EventLogWriter, initEventLogWriter) - 0usize];
    ["Offset of field: EventLogWriter::writeEventLog"]
        [::core::mem::offset_of!(EventLogWriter, writeEventLog) - 8usize];
    ["Offset of field: EventLogWriter::flushEventLog"]
        [::core::mem::offset_of!(EventLogWriter, flushEventLog) - 16usize];
    ["Offset of field: EventLogWriter::stopEventLogWriter"]
        [::core::mem::offset_of!(EventLogWriter, stopEventLogWriter) - 24usize];
};

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_eventLogStatus() -> bool {
    let expected = unsafe { transmute(sys::eventLogStatus()) };
    let actual = unsafe { super::eventLogStatus() };
    actual == expected
}

#[test]
#[ignore]
fn test_eventLogStatus() {
    unsafe { super::eventLogStatus() };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_startEventLogging(writer: EventLogWriter) -> bool {
    let expected = unsafe { transmute(sys::startEventLogging(&writer.into())) };
    let actual = unsafe { super::startEventLogging(&writer) };
    actual == expected
}

#[test]
#[ignore]
fn test_startEventLogging() {
    let writer = Default::default();
    unsafe { super::startEventLogging(&writer) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_endEventLogging() {
    unsafe { super::endEventLogging() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_flushEventLog() {
    let cap = Default::default();
    unsafe { super::flushEventLog(&mut &mut cap) };
    todo!("assert")
}
