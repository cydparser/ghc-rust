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
fn sys_eq_BACKTRACE_CHUNK_SZ() {
    assert_eq!(sys::BACKTRACE_CHUNK_SZ, BACKTRACE_CHUNK_SZ);
}

#[cfg(feature = "sys")]
#[test]
fn sys_size_BacktraceChunk_() {
    assert_eq!(
        size_of::<sys::BacktraceChunk_>(),
        size_of::<BacktraceChunk_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BacktraceChunk_"][size_of::<BacktraceChunk_>() - 2064usize];
    ["Alignment of BacktraceChunk_"][align_of::<BacktraceChunk_>() - 1usize];
    ["Offset of field: BacktraceChunk_::n_frames"][offset_of!(BacktraceChunk_, n_frames) - 0usize];
    ["Offset of field: BacktraceChunk_::next"][offset_of!(BacktraceChunk_, next) - 8usize];
    ["Offset of field: BacktraceChunk_::frames"][offset_of!(BacktraceChunk_, frames) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_Backtrace_() {
    assert_eq!(size_of::<sys::Backtrace_>(), size_of::<Backtrace_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Backtrace_"][size_of::<Backtrace_>() - 16usize];
    ["Alignment of Backtrace_"][align_of::<Backtrace_>() - 8usize];
    ["Offset of field: Backtrace_::n_frames"][offset_of!(Backtrace_, n_frames) - 0usize];
    ["Offset of field: Backtrace_::last"][offset_of!(Backtrace_, last) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_Location_() {
    assert_eq!(size_of::<sys::Location_>(), size_of::<Location_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Location_"][size_of::<Location_>() - 32usize];
    ["Alignment of Location_"][align_of::<Location_>() - 1usize];
    ["Offset of field: Location_::object_file"][offset_of!(Location_, object_file) - 0usize];
    ["Offset of field: Location_::function"][offset_of!(Location_, function) - 8usize];
    ["Offset of field: Location_::source_file"][offset_of!(Location_, source_file) - 16usize];
    ["Offset of field: Location_::lineno"][offset_of!(Location_, lineno) - 24usize];
    ["Offset of field: Location_::colno"][offset_of!(Location_, colno) - 28usize];
};

#[cfg(feature = "sys")]
#[test]
fn sys_size_LibdwSession_() {
    assert_eq!(size_of::<sys::LibdwSession_>(), size_of::<LibdwSession_>())
}

#[test]
#[ignore]
fn test_backtraceFree() {
    let mut bt = null_mut();
    unsafe { backtraceFree(&mut bt) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_libdwGetBacktrace(session: LibdwSession) -> bool {
    let expected = unsafe { transmute(sys::libdwGetBacktrace(&mut session.into())) };
    let actual = unsafe { libdwGetBacktrace(&mut session) };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwGetBacktrace() {
    let mut session = null_mut();
    unsafe { libdwGetBacktrace(&mut session) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_libdwLookupLocation(session: LibdwSession, loc: Location, pc: StgPtr) -> bool {
    let expected = unsafe { sys::libdwLookupLocation(&mut session.into(), &mut loc.into(), pc) };
    let actual = unsafe { libdwLookupLocation(&mut session, &mut loc, pc) };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwLookupLocation() {
    let mut session = null_mut();
    let mut loc = null_mut();
    let pc = Default::default();
    unsafe { libdwLookupLocation(&mut session, &mut loc, pc) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPrintBacktrace() {
    let mut session = null_mut();
    let mut file = null_mut();
    let mut bt = null_mut();
    unsafe { libdwPrintBacktrace(&mut session, &mut file, &mut bt) };
    todo!("assert")
}
