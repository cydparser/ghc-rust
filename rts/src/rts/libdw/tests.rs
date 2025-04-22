use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_BACKTRACE_CHUNK_SZ() {
    assert_eq!(sys::BACKTRACE_CHUNK_SZ, super::BACKTRACE_CHUNK_SZ);
}

#[cfg(feature = "sys")]
#[test]
fn test_size_of_BacktraceChunk_() {
    assert_eq!(
        size_of::<sys::BacktraceChunk_>(),
        size_of::<super::BacktraceChunk_>()
    )
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BacktraceChunk_"][::core::mem::size_of::<BacktraceChunk_>() - 2064usize];
    ["Alignment of BacktraceChunk_"][::core::mem::align_of::<BacktraceChunk_>() - 1usize];
    ["Offset of field: BacktraceChunk_::n_frames"]
        [::core::mem::offset_of!(BacktraceChunk_, n_frames) - 0usize];
    ["Offset of field: BacktraceChunk_::next"]
        [::core::mem::offset_of!(BacktraceChunk_, next) - 8usize];
    ["Offset of field: BacktraceChunk_::frames"]
        [::core::mem::offset_of!(BacktraceChunk_, frames) - 16usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_Backtrace_() {
    assert_eq!(size_of::<sys::Backtrace_>(), size_of::<super::Backtrace_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Backtrace_"][::core::mem::size_of::<Backtrace_>() - 16usize];
    ["Alignment of Backtrace_"][::core::mem::align_of::<Backtrace_>() - 8usize];
    ["Offset of field: Backtrace_::n_frames"]
        [::core::mem::offset_of!(Backtrace_, n_frames) - 0usize];
    ["Offset of field: Backtrace_::last"][::core::mem::offset_of!(Backtrace_, last) - 8usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_Location_() {
    assert_eq!(size_of::<sys::Location_>(), size_of::<super::Location_>())
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Location_"][::core::mem::size_of::<Location_>() - 32usize];
    ["Alignment of Location_"][::core::mem::align_of::<Location_>() - 1usize];
    ["Offset of field: Location_::object_file"]
        [::core::mem::offset_of!(Location_, object_file) - 0usize];
    ["Offset of field: Location_::function"][::core::mem::offset_of!(Location_, function) - 8usize];
    ["Offset of field: Location_::source_file"]
        [::core::mem::offset_of!(Location_, source_file) - 16usize];
    ["Offset of field: Location_::lineno"][::core::mem::offset_of!(Location_, lineno) - 24usize];
    ["Offset of field: Location_::colno"][::core::mem::offset_of!(Location_, colno) - 28usize];
};

#[cfg(feature = "sys")]
#[test]
fn test_size_of_LibdwSession_() {
    assert_eq!(
        size_of::<sys::LibdwSession_>(),
        size_of::<super::LibdwSession_>()
    )
}

#[test]
#[ignore]
fn test_backtraceFree() {
    let mut bt = Default::default();
    unsafe { super::backtraceFree(&mut bt) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_libdwGetBacktrace(session: LibdwSession) -> bool {
    let expected = unsafe { transmute(sys::libdwGetBacktrace(&mut session.into())) };
    let actual = unsafe { super::libdwGetBacktrace(&mut session) };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwGetBacktrace() {
    let mut session = Default::default();
    unsafe { super::libdwGetBacktrace(&mut session) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
fn equivalent_libdwLookupLocation(session: LibdwSession, loc: Location, pc: StgPtr) -> bool {
    let expected = unsafe {
        transmute(sys::libdwLookupLocation(
            &mut session.into(),
            &mut loc.into(),
            pc.into(),
        ))
    };
    let actual = unsafe { super::libdwLookupLocation(&mut session, &mut loc, pc) };
    actual == expected
}

#[test]
#[ignore]
fn test_libdwLookupLocation() {
    let mut session = Default::default();
    let mut loc = Default::default();
    let pc = Default::default();
    unsafe { super::libdwLookupLocation(&mut session, &mut loc, pc) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_libdwPrintBacktrace() {
    let mut session = Default::default();
    let mut file = Default::default();
    let mut bt = Default::default();
    unsafe { super::libdwPrintBacktrace(&mut session, &mut file, &mut bt) };
    todo!("assert")
}
