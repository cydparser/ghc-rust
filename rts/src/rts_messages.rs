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
    fatalInternalErrorFn.expect("non-null fatalInternalErrorFn")(s, ap.as_va_list());
    stg_exit(EXIT_INTERNAL_ERROR);
}

#[ffi(utils)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn vbarf(mut s: *const c_char, mut ap: VaList) -> ! {
    fatalInternalErrorFn.expect("non-null fatalInternalErrorFn")(s, ap.as_va_list());
    stg_exit(EXIT_INTERNAL_ERROR);
}

#[ffi(utils)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _assertFail(mut filename: *const c_char, mut linenum: u32) -> ! {
    barf(
        c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
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
    errorMsgFn.expect("non-null errorMsgFn")(s, ap.as_va_list());
}

unsafe fn _warnFail(mut filename: *const c_char, mut linenum: u32) {
    errorBelch(
        c"ASSERTION FAILED: file %s, line %u\n".as_ptr(),
        filename,
        linenum,
    );
}

unsafe fn verrorBelch(mut s: *const c_char, mut ap: VaList) {
    errorMsgFn.expect("non-null errorMsgFn")(s, ap.as_va_list());
}

unsafe fn sysErrorBelch(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, ap.as_va_list());
}

unsafe fn vsysErrorBelch(mut s: *const c_char, mut ap: VaList) {
    sysErrorMsgFn.expect("non-null sysErrorMsgFn")(s, ap.as_va_list());
}

#[ffi(ghc_lib, testsuite)]
#[unsafe(no_mangle)]
#[instrument]
pub unsafe extern "C" fn debugBelch(mut s: *const c_char, mut args: ...) {
    let mut ap: VaListImpl;
    ap = args.clone();

    debugMsgFn.expect("non-null debugMsgFn")(s, ap.as_va_list());
}

unsafe fn vdebugBelch(mut s: *const c_char, mut ap: VaList) -> i32 {
    return debugMsgFn.expect("non-null debugMsgFn")(s, ap.as_va_list());
}

unsafe fn rtsFatalInternalErrorFn(mut s: *const c_char, mut ap: VaList) -> ! {
    if !prog_argv.is_null() && !prog_name.is_null() {
        fprintf(__stderrp, c"%s: internal error: ".as_ptr(), prog_name);
    } else {
        fprintf(__stderrp, c"internal error: ".as_ptr());
    }

    vfprintf(__stderrp, s, ap.as_va_list());
    fprintf(__stderrp, c"\n".as_ptr());

    fprintf(
        __stderrp,
        c"    (GHC version %s for %s)\n".as_ptr(),
        __GLASGOW_HASKELL_FULL_VERSION__.as_ptr(),
        c"aarch64_apple_darwin".as_ptr(),
    );

    fprintf(
        __stderrp,
        c"    Please report this as a GHC bug:  https://www.haskell.org/ghc/reportabug\n".as_ptr(),
    );

    fflush(__stderrp);

    if RtsFlags.TraceFlags.tracing == TRACE_EVENTLOG {
        flushAllCapsEventsBufs();
    }

    abort();
}

unsafe fn rtsErrorMsgFn(mut s: *const c_char, mut ap: VaList) {
    if !prog_name.is_null() {
        fprintf(__stderrp, c"%s: ".as_ptr(), prog_name);
    }

    vfprintf(__stderrp, s, ap.as_va_list());
    fprintf(__stderrp, c"\n".as_ptr());
}

unsafe fn rtsSysErrorMsgFn(mut s: *const c_char, mut ap: VaList) {
    let mut syserr = null_mut::<c_char>();
    syserr = strerror(*__error());

    if !prog_argv.is_null() && !prog_name.is_null() {
        fprintf(__stderrp, c"%s: ".as_ptr(), prog_name);
    }

    vfprintf(__stderrp, s, ap.as_va_list());

    if !syserr.is_null() {
        fprintf(__stderrp, c": %s\n".as_ptr(), syserr);
    } else {
        fprintf(__stderrp, c"\n".as_ptr());
    };
}

unsafe fn rtsDebugMsgFn(mut s: *const c_char, mut ap: VaList) -> i32 {
    let mut r: i32 = 0;
    r = vfprintf(__stderrp, s, ap.as_va_list());
    fflush(__stderrp);

    return r;
}

unsafe fn rtsBadAlignmentBarf() -> ! {
    barf(c"Encountered incorrectly aligned pointer. This can't be good.".as_ptr());
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsOutOfBoundsAccess() -> ! {
    barf(c"Encountered out of bounds array access.".as_ptr());
}

#[ffi(compiler)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsMemcpyRangeOverlap() -> ! {
    barf(c"Encountered overlapping source/destination ranges in a memcpy-using op.".as_ptr());
}
