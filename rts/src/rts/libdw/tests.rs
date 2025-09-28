use super::*;

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

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_backtraceFree() {
    let expected = {
        let mut bt: sys::Backtrace = todo!();
        unsafe { sys::backtraceFree(&raw mut bt) };
        todo!()
    };
    let actual = {
        let mut bt: Backtrace = todo!();
        unsafe { backtraceFree(&raw mut bt) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_backtraceFree() {
    let actual = {
        let bt: Backtrace = todo!();
        unsafe { backtraceFree(&raw mut bt) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_libdwGetBacktrace() {
    let expected = {
        let mut session: sys::LibdwSession = todo!();
        let result: &Backtrace = unsafe { transmute(&*sys::libdwGetBacktrace(&raw mut session)) };
        todo!()
    };
    let actual = {
        let mut session: LibdwSession = todo!();
        let result: &Backtrace = unsafe { &*libdwGetBacktrace(&raw mut session) };
        todo!()
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_libdwGetBacktrace() {
    let actual = {
        let session: LibdwSession = todo!();
        let result: &Backtrace = unsafe { &*libdwGetBacktrace(&raw mut session) };
        todo!()
    };
    let expected = todo!();
    assert_eq!(expected, actual);
}

#[cfg(feature = "sys")]
#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn equivalent_libdwLookupLocation() {
    let expected: c_int = {
        let mut session: sys::LibdwSession = todo!();
        let mut loc: sys::Location = todo!();
        let pc: StgPtr = todo!();
        unsafe { sys::libdwLookupLocation(&raw mut session, &raw mut loc, pc) }
    };
    let actual: c_int = {
        let mut session: LibdwSession = todo!();
        let mut loc: Location = todo!();
        let pc: StgPtr = todo!();
        unsafe { libdwLookupLocation(&raw mut session, &raw mut loc, pc) }
    };
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
#[expect(unreachable_code, unused_variables)]
fn test_libdwLookupLocation() {
    let actual: c_int = {
        let session: LibdwSession = todo!();
        let loc: Location = todo!();
        let pc: StgPtr = todo!();
        unsafe { libdwLookupLocation(&raw mut session, &raw mut loc, pc) }
    };
    let expected: c_int = todo!();
    assert_eq!(expected, actual);
}
