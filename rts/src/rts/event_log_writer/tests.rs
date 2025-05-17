use super::*;
use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
use crate::utils::test::*;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
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
fn equivalent_startEventLogging(writer: EventLogWriter) -> bool {
    let expected = unsafe { transmute(sys::startEventLogging(&writer.into())) };
    let actual = unsafe { startEventLogging(&writer) };
    actual == expected
}

#[test]
#[ignore]
fn test_startEventLogging() {
    let writer = null();
    unsafe { startEventLogging(&writer) };
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
    unsafe { flushEventLog(&mut &mut cap) };
    todo!("assert")
}
