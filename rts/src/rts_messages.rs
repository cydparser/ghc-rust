use crate::eventlog::event_log::flushAllCapsEventsBufs;
use crate::ffi::ghcversion::__GLASGOW_HASKELL_FULL_VERSION__;
use crate::ffi::rts::flags::{RtsFlags, TRACE_EVENTLOG};
use crate::ffi::rts::{EXIT_INTERNAL_ERROR, prog_argv, prog_name, stg_exit};
use crate::prelude::*;

#[cfg(test)]
mod tests;

static mut fatalInternalErrorFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsFatalInternalErrorFn as unsafe extern "C" fn(*const c_char, VaList) -> !) };

static mut debugMsgFn: Option<RtsMsgFunctionRetLen> =
    unsafe { Some(rtsDebugMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> c_int) };

static mut errorMsgFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsErrorMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> ()) };

static mut sysErrorMsgFn: Option<RtsMsgFunction> =
    unsafe { Some(rtsSysErrorMsgFn as unsafe extern "C" fn(*const c_char, VaList) -> ()) };

#[ffi(compiler, ghc_lib, libraries, testsuite, utils)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn barf(mut s: *const c_char, mut args: ...) -> ! {
    let mut ap: VaListImpl;
    ap = args.clone();
    Some(fatalInternalErrorFn.expect("non-null function pointer"))
        .expect("non-null function pointer")(s, ap.as_va_list());
    stg_exit(EXIT_INTERNAL_ERROR);
}

#[ffi(utils)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn vbarf(mut s: *const c_char, mut ap: VaList) -> ! {
    Some(fatalInternalErrorFn.expect("non-null function pointer"))
        .expect("non-null function pointer")(s, ap.as_va_list());
    stg_exit(EXIT_INTERNAL_ERROR);
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _assertFail(mut filename: *const c_char, mut linenum: c_uint) -> ! {
    barf(
        b"ASSERTION FAILED: file %s, line %u\n\0" as *const u8 as *const c_char,
        filename,
        linenum,
    );
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn errorBelch(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    Some(errorMsgFn.expect("non-null function pointer")).expect("non-null function pointer")(
        s,
        ap.as_va_list(),
    );
}

unsafe fn _warnFail(mut filename: *const c_char, mut linenum: c_uint) {
    errorBelch(
        b"ASSERTION FAILED: file %s, line %u\n\0" as *const u8 as *const c_char,
        filename,
        linenum,
    );
}

unsafe fn verrorBelch(mut s: *const c_char, mut ap: VaList) {
    Some(errorMsgFn.expect("non-null function pointer")).expect("non-null function pointer")(
        s,
        ap.as_va_list(),
    );
}

unsafe fn sysErrorBelch(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    Some(sysErrorMsgFn.expect("non-null function pointer")).expect("non-null function pointer")(
        s,
        ap.as_va_list(),
    );
}

unsafe fn vsysErrorBelch(mut s: *const c_char, mut ap: VaList) {
    Some(sysErrorMsgFn.expect("non-null function pointer")).expect("non-null function pointer")(
        s,
        ap.as_va_list(),
    );
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn debugBelch(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    Some(debugMsgFn.expect("non-null function pointer")).expect("non-null function pointer")(
        s,
        ap.as_va_list(),
    );
}

unsafe fn vdebugBelch(mut s: *const c_char, mut ap: VaList) -> c_int {
    return Some(debugMsgFn.expect("non-null function pointer"))
        .expect("non-null function pointer")(s, ap.as_va_list());
}

unsafe fn rtsFatalInternalErrorFn(mut s: *const c_char, mut ap: VaList) -> ! {
    if !prog_argv.is_null() && !prog_name.is_null() {
        fprintf(
            __stderrp,
            b"%s: internal error: \0" as *const u8 as *const c_char,
            prog_name,
        );
    } else {
        fprintf(
            __stderrp,
            b"internal error: \0" as *const u8 as *const c_char,
        );
    }

    vfprintf(__stderrp, s, ap.as_va_list());
    fprintf(__stderrp, b"\n\0" as *const u8 as *const c_char);

    fprintf(
        __stderrp,
        b"    (GHC version %s for %s)\n\0" as *const u8 as *const c_char,
        __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
        b"aarch64_apple_darwin\0" as *const u8 as *const c_char,
    );

    fprintf(
        __stderrp,
        b"    Please report this as a GHC bug:  https://www.haskell.org/ghc/reportabug\n\0"
            as *const u8 as *const c_char,
    );

    fflush(__stderrp);

    if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG {
        flushAllCapsEventsBufs();
    }

    abort();
}

unsafe fn rtsErrorMsgFn(mut s: *const c_char, mut ap: VaList) {
    if !prog_name.is_null() {
        fprintf(
            __stderrp,
            b"%s: \0" as *const u8 as *const c_char,
            prog_name,
        );
    }

    vfprintf(__stderrp, s, ap.as_va_list());
    fprintf(__stderrp, b"\n\0" as *const u8 as *const c_char);
}

unsafe fn rtsSysErrorMsgFn(mut s: *const c_char, mut ap: VaList) {
    let mut syserr = null_mut::<c_char>();
    syserr = strerror(*__error());

    if !prog_argv.is_null() && !prog_name.is_null() {
        fprintf(
            __stderrp,
            b"%s: \0" as *const u8 as *const c_char,
            prog_name,
        );
    }

    vfprintf(__stderrp, s, ap.as_va_list());

    if !syserr.is_null() {
        fprintf(__stderrp, b": %s\n\0" as *const u8 as *const c_char, syserr);
    } else {
        fprintf(__stderrp, b"\n\0" as *const u8 as *const c_char);
    };
}

unsafe fn rtsDebugMsgFn(mut s: *const c_char, mut ap: VaList) -> c_int {
    let mut r: c_int = 0;
    r = vfprintf(__stderrp, s, ap.as_va_list());
    fflush(__stderrp);

    return r;
}

unsafe fn rtsBadAlignmentBarf() -> ! {
    barf(
        b"Encountered incorrectly aligned pointer. This can't be good.\0" as *const u8
            as *const c_char,
    );
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    barf(b"Encountered out of bounds array access.\0" as *const u8 as *const c_char);
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    barf(
        b"Encountered overlapping source/destination ranges in a memcpy-using op.\0" as *const u8
            as *const c_char,
    );
}
