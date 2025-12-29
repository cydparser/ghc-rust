use super::*;

#[cfg(feature = "sys")]
#[test]
fn sys_BacktraceChunk_layout() {
    assert_eq!(
        size_of::<BacktraceChunk>(),
        size_of::<sys::BacktraceChunk>()
    );
    assert_eq!(
        align_of::<BacktraceChunk>(),
        align_of::<sys::BacktraceChunk>()
    );
}

#[cfg(feature = "sys")]
#[test]
fn sys_Backtrace__layout() {
    assert_eq!(
        offset_of!(Backtrace_, n_frames),
        offset_of!(sys::Backtrace_, n_frames)
    );
    assert_eq!(
        size_of::<*mut BacktraceChunk>(),
        size_of::<*mut sys::BacktraceChunk>()
    );
    assert_eq!(
        offset_of!(Backtrace_, last),
        offset_of!(sys::Backtrace_, last)
    );
    assert_eq!(size_of::<Backtrace_>(), size_of::<sys::Backtrace_>());
    assert_eq!(align_of::<Backtrace_>(), align_of::<sys::Backtrace_>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_Backtrace_layout() {
    assert_eq!(size_of::<Backtrace>(), size_of::<sys::Backtrace>());
    assert_eq!(align_of::<Backtrace>(), align_of::<sys::Backtrace>());
}

#[cfg(feature = "sys")]
#[test]
fn sys_Location_layout() {
    assert_eq!(size_of::<Location>(), size_of::<sys::Location>());
    assert_eq!(align_of::<Location>(), align_of::<sys::Location>());
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
    assert_eq!(actual, expected);
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
